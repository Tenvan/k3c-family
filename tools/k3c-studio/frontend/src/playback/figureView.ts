import Phaser from 'phaser';
import { GAME_HEIGHT, UNIT_PX } from '@game/core/constants';
import { EFFECT_CONFIG, type EffectKind } from '@game/scenes/effects/effects';
import { objectImage } from '@game/scenes/render/objectView';
import { createSheetAnims, face, makeSprite, playAnim, preloadSheet } from '@game/scenes/render/sprites';
import { MOUNTS, SHEETS, animInfo, inAtlas, mountPoseOf, mountsWithSheet, resolveAnim, unitTicks, type Anim, type Control, type FigureSpec, type Gap } from './figure';

// Phaser-Anteil der Abspiel-Bausteine (B-463, WZ10.1): zeichnet Figuren mit den Animationen und Definitionen des Spiels
// (`makeSprite`, `playAnim`, `face` aus `src/scenes/render/sprites.ts`) auf der Bühne des Studios. Die Entscheidungen
// (Animation, Pose, Maßstab) trifft `figure.ts`; hier wird nur gezeichnet. Nicht headless prüfbar: die Logik dahinter
// hat Tests, das Zeichnen prüft 🧑 im Fenster.

/** Bodenlinie der Figuren in der Bühne (Pixel von oben); die Kamera zeigt die Spielhöhe */
export const GROUND_Y = Math.round(GAME_HEIGHT * 0.78);

/** Eine Figur auf der Bühne: Lage in Units, Animation, optional auf einem Reittier. Alles andere sind Parameter des Spiels. */
export interface FigureItem {
  id: string;
  /** Lage auf der Bodenlinie in Units (0 = Mitte der Bühne) */
  x: number;
  spec: FigureSpec;
  anim: Anim;
  /** Der Reiter sitzt auf dem Reittier `key`; `speed` ist das Tempo (0 = steht), `refSpeed` das Lauftempo ohne Sprint */
  mount?: { key: string; refSpeed: number; speed: number; sheet?: string };
  /** Effekt des Spiels (Ring nach `EFFECT_CONFIG`, bei `noTarget` mit Text), der sich wiederholt */
  effect?: { kind: EffectKind; text?: string };
  /** Deckkraft: fest, oder `fade` wechselt zwischen den Werten (Respawn) */
  alpha?: number;
  fade?: { from: number; to: number };
  /** Objekt des Spiels (`objectImage`, z. B. `coin`), das neben der Figur steht */
  prop?: string;
}

/** Textur-Schlüssel eines Einzel-PNG-Sheets wie im Spiel */
const textureKey = (sheet: string, anim: Anim) => `${sheet}-${anim}`;

/** Lädt die Einzel-PNGs der Reittiere und Sheets außerhalb des Atlas, die noch fehlen, und legt ihre Animationen an. */
export function loadFigureSheets(scene: Phaser.Scene, items: readonly FigureItem[]): Promise<void> {
  const sheets = new Set<string>();
  for (const i of items) {
    if (i.mount) sheets.add(MOUNT_SHEET(i.mount));
    if (!inAtlas(i.spec.sheet)) sheets.add(i.spec.sheet);
  }
  let queued = 0;
  for (const sheet of sheets) queued += preloadSheet(scene, sheet); // Laden und Animationen kommen aus dem Spiel (B-476)
  const finish = () => { for (const sheet of sheets) createSheetAnims(scene, sheet); };
  if (queued === 0) { finish(); return Promise.resolve(); }
  return new Promise((resolve) => {
    scene.load.once('complete', () => { finish(); resolve(); });
    scene.load.start();
  });
}

const MOUNT_SHEET = (m: NonNullable<FigureItem['mount']>): string => m.sheet ?? MOUNTS[m.key]?.sheet ?? m.key;
const mountsOf = (m: NonNullable<FigureItem['mount']>) => (m.sheet ? mountsWithSheet(m.key, m.sheet) : MOUNTS);

/** Eine gezeichnete Figur. `gap` ist die Lücke der gewählten Animation oder des Reittiers (null = vollständig). */
export class FigureView {
  readonly gap: Gap | null;
  private readonly root: Phaser.GameObjects.Container;
  private readonly rider: Phaser.GameObjects.Sprite;
  private readonly mount: Phaser.GameObjects.Sprite | null;
  private readonly anim: Anim;
  private readonly extras: { remove?: () => void; destroy?: () => void; stop?: () => void }[] = [];
  private readonly props: Phaser.GameObjects.GameObject[] = [];
  constructor(private readonly scene: Phaser.Scene, private readonly item: FigureItem) {
    const resolved = resolveAnim(item.spec.sheet, item.anim);
    this.anim = resolved.anim;
    this.rider = makeSprite(scene, item.spec);
    let mountGap: Gap | null = null;
    if (item.mount) {
      const m = mountPoseOf(item.mount.key, item.spec, item.mount.refSpeed, item.mount.speed, 0, mountsOf(item.mount));
      mountGap = m.gap;
      const sheet = m.pose?.sheet;
      this.mount = m.pose && sheet && scene.textures.exists(textureKey(sheet, 'idle')) ? scene.add.sprite(0, 0, textureKey(sheet, 'idle'), 0).setData('sheet', sheet) : null;
      if (m.pose && !this.mount) mountGap = { kind: 'image', reason: `Lücke: Bild für Reittier ${item.mount.key} fehlt` };
    } else {
      this.mount = null;
    }
    this.gap = resolved.gap ?? mountGap;
    this.root = scene.add.container(item.x * UNIT_PX, GROUND_Y, this.mount ? [this.mount, this.rider] : [this.rider]);
    this.start(null);
    this.follow();
    this.decorate();
  }

  /** Wiedergabe übernehmen: Schleife, Tempo und Einzelbild */
  apply(control: Control): void {
    this.start(control);
    this.follow();
  }

  /** Animationen starten bzw. anhalten (Einzelbild) und Tempo und Schleife setzen */
  private start(c: Control | null): void {
    this.play(this.rider, this.anim, c);
    const mount = this.mount;
    if (!mount || !this.item.mount) return;
    const { pose } = mountPoseOf(this.item.mount.key, this.item.spec, this.item.mount.refSpeed, this.item.mount.speed, 0, mountsOf(this.item.mount));
    if (!pose) return;
    this.play(mount, pose.anim, c);
    mount.anims.timeScale = pose.timeScale * (c?.tempo ?? 1);
  }

  private play(sprite: Phaser.GameObjects.Sprite, anim: Anim, c: Control | null): void {
    if (!c) { // Standard wie im Spiel: playAnim wählt die Animation (Rückfall idle), ein Angriff läuft einmal
      playAnim(sprite, anim, true);
      sprite.anims.timeScale = 1;
      return;
    }
    const key = textureKey(sprite.getData('sheet') as string, anim);
    if (c.frame != null) {
      const sheet = sprite.getData('sheet') as string;
      const index = Math.min(c.frame, Math.max(0, (animInfo(sheet, anim)?.frames ?? 1) - 1)); // die Bildzahl kann je Sheet abweichen
      sprite.anims.pause();
      if (sprite.texture.key === 'atlas') sprite.setFrame(`${key}/${index}`);
      else sprite.setTexture(key, index); // Einzel-PNG: ein Bild je Animation, die Textur wechselt mit der Animation
      return;
    }
    sprite.play({ key, repeat: c.loop ? -1 : 0 });
    sprite.anims.timeScale = c.tempo;
  }

  /** Pro Bild: Sattelpunkt und Reiter-Pose nachführen (die Pose hängt am Bild des Reittiers) */
  follow(): void {
    if (!this.mount || !this.item.mount) return;
    const frame = Number(this.mount.frame.name);
    const { pose } = mountPoseOf(this.item.mount.key, this.item.spec, this.item.mount.refSpeed, this.item.mount.speed, Number.isFinite(frame) ? frame : 0, mountsOf(this.item.mount));
    if (!pose) return;
    const mount = this.mount;
    face(mount, 1);
    mount.setScale(pose.scale);
    if (pose.tint) mount.setTint(Phaser.Display.Color.HexStringToColor(pose.tint).color);
    const r = pose.rider;
    this.rider.setFlipX(pose.flip).setOrigin(r.originX, r.originY).setScale(r.scale).setPosition(r.x, r.y);
    this.rider.setCrop(0, 0, this.rider.frame.width, r.cropHeight);
  }

  /** Deckkraft, Objekt und Effekt der Aktion (WZ10.3); alles aus den Daten des Spiels */
  private decorate(): void {
    const { alpha, fade, prop, effect } = this.item;
    if (alpha !== undefined) this.root.setAlpha(alpha);
    if (fade) {
      this.root.setAlpha(fade.from);
      this.extras.push(this.scene.tweens.add({ targets: this.root, alpha: fade.to, duration: 1200, hold: 600, repeat: -1, onRepeat: () => this.root.setAlpha(fade.from) }));
    }
    if (prop) {
      const image = objectImage(this.scene, prop, this.root.x + 60, GROUND_Y);
      if (image) this.props.push(image);
    }
    if (effect) {
      const ring = () => this.ring(effect);
      ring();
      this.extras.push(this.scene.time.addEvent({ delay: EFFECT_CONFIG[effect.kind].durationMs + 700, loop: true, callback: ring }));
    }
  }

  /** Ein Effekt: Ring in Farbe und Größe aus `EFFECT_CONFIG`, wächst und verblasst in der Dauer des Spiels */
  private ring(effect: { kind: EffectKind; text?: string }): void {
    const cfg = EFFECT_CONFIG[effect.kind];
    const x = this.root.x;
    const y = GROUND_Y - 40;
    const made: Phaser.GameObjects.GameObject[] = [];
    if (cfg.radius > 0) made.push(this.scene.add.circle(x, y, cfg.radius).setStrokeStyle(4, cfg.color));
    if (effect.text) made.push(this.scene.add.text(x, y - 60, effect.text, { fontSize: '28px', color: '#ffffff' }).setOrigin(0.5));
    if (made.length === 0) return;
    this.scene.tweens.add({ targets: made, alpha: 0, scale: cfg.radius > 0 ? 2 : 1, duration: cfg.durationMs, onComplete: () => made.forEach((o) => o.destroy()) });
  }

  /** Bildzahl der gespielten Animation (für die Einzelbild-Schaltung) */
  frames(): number {
    return animInfo(this.item.spec.sheet, this.anim)?.frames ?? 1;
  }

  /** Bild, bei dem die Wiedergabe gerade steht */
  currentFrame(): number {
    const n = Number(this.rider.frame.name.split('/').pop());
    return Number.isFinite(n) ? n : 0;
  }

  destroy(): void {
    for (const e of this.extras) { e.stop?.(); e.remove?.(); e.destroy?.(); }
    for (const p of this.props) p.destroy();
    this.root.destroy();
  }
}

/** Unit-Skala an der Bodenlinie von `from` bis `to` (Strich je Unit, längere je 5 Units mit Zahl); liefert die Objekte zum Abbauen. */
export function drawScale(scene: Phaser.Scene, from: number, to: number): Phaser.GameObjects.GameObject[] {
  const g = scene.add.graphics();
  g.lineStyle(2, 0x6b6b80, 1).lineBetween(from * UNIT_PX, GROUND_Y, to * UNIT_PX, GROUND_Y);
  const out: Phaser.GameObjects.GameObject[] = [g];
  for (const t of unitTicks(from, to)) {
    const long = t.unit % 5 === 0;
    g.lineBetween(t.px, GROUND_Y, t.px, GROUND_Y + (long ? 16 : 8));
    if (long) out.push(scene.add.text(t.px, GROUND_Y + 18, String(t.unit), { fontSize: '14px', color: '#9a9ab0' }).setOrigin(0.5, 0));
  }
  return out;
}

/** Größte Höhe der Sheets in Units, damit die Bühne die Figuren ganz zeigt */
export const sheetHeightUnits = (sheet: string): number => ((SHEETS[sheet]?.height ?? 0) * (SHEETS[sheet]?.scale ?? 1)) / UNIT_PX;
