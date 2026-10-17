import Phaser from 'phaser';
import { GAME_HEIGHT, UNIT_PX } from '@game/core/constants';
import { face, makeSprite, playAnim } from '@game/scenes/sprites';
import { ANIMS, MOUNTS, SHEETS, animInfo, inAtlas, mountPoseOf, resolveAnim, sheetFiles, unitTicks, type Anim, type Control, type FigureSpec, type Gap } from './figure';

// Phaser-Anteil der Abspiel-Bausteine (B-463, WZ10.1): zeichnet Figuren mit den Animationen und Definitionen des Spiels
// (`makeSprite`, `playAnim`, `face` aus `src/scenes/sprites.ts`) auf der Bühne des Studios. Die Entscheidungen
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
  mount?: { key: string; refSpeed: number; speed: number };
}

/** Textur-Schlüssel eines Einzel-PNG-Sheets wie im Spiel */
const textureKey = (sheet: string, anim: Anim) => `${sheet}-${anim}`;

/** Lädt die Einzel-PNGs der Reittiere und Sheets außerhalb des Atlas, die noch fehlen, und legt ihre Animationen an. */
export function loadFigureSheets(scene: Phaser.Scene, items: readonly FigureItem[]): Promise<void> {
  const sheets = new Set<string>();
  for (const i of items) {
    if (i.mount) sheets.add(MOUNT_SHEET(i.mount.key));
    if (!inAtlas(i.spec.sheet)) sheets.add(i.spec.sheet);
  }
  let queued = 0;
  for (const sheet of sheets) {
    for (const f of sheetFiles(sheet)) {
      if (scene.textures.exists(f.key)) continue;
      scene.load.spritesheet(f.key, f.url, { frameWidth: f.frameWidth, frameHeight: f.frameHeight });
      queued++;
    }
  }
  const finish = () => { for (const sheet of sheets) createSheetAnims(scene, sheet); };
  if (queued === 0) { finish(); return Promise.resolve(); }
  return new Promise((resolve) => {
    scene.load.once('complete', () => { finish(); resolve(); });
    scene.load.start();
  });
}

const MOUNT_SHEET = (key: string): string => MOUNTS[key]?.sheet ?? key;

function createSheetAnims(scene: Phaser.Scene, sheet: string): void {
  for (const anim of ANIMS) {
    const info = animInfo(sheet, anim);
    const key = textureKey(sheet, anim);
    if (!info || !scene.textures.exists(key) || scene.anims.exists(key)) continue;
    scene.textures.get(key).setFilter(Phaser.Textures.FilterMode.NEAREST);
    scene.anims.create({ key, frames: scene.anims.generateFrameNumbers(key, { start: 0, end: info.frames - 1 }), frameRate: info.fps, repeat: anim === 'attack' ? 0 : -1 });
  }
}

/** Eine gezeichnete Figur. `gap` ist die Lücke der gewählten Animation oder des Reittiers (null = vollständig). */
export class FigureView {
  readonly gap: Gap | null;
  private readonly root: Phaser.GameObjects.Container;
  private readonly rider: Phaser.GameObjects.Sprite;
  private readonly mount: Phaser.GameObjects.Sprite | null;
  private readonly anim: Anim;
  constructor(scene: Phaser.Scene, private readonly item: FigureItem) {
    const resolved = resolveAnim(item.spec.sheet, item.anim);
    this.anim = resolved.anim;
    this.rider = makeSprite(scene, item.spec);
    let mountGap: Gap | null = null;
    if (item.mount) {
      const m = mountPoseOf(item.mount.key, item.spec, item.mount.refSpeed, item.mount.speed);
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
    const { pose } = mountPoseOf(this.item.mount.key, this.item.spec, this.item.mount.refSpeed, this.item.mount.speed);
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
    const { pose } = mountPoseOf(this.item.mount.key, this.item.spec, this.item.mount.refSpeed, this.item.mount.speed, Number.isFinite(frame) ? frame : 0);
    if (!pose) return;
    const mount = this.mount;
    face(mount, 1);
    mount.setScale(pose.scale);
    if (pose.tint) mount.setTint(Phaser.Display.Color.HexStringToColor(pose.tint).color);
    const r = pose.rider;
    this.rider.setFlipX(pose.flip).setOrigin(r.originX, r.originY).setScale(r.scale).setPosition(r.x, r.y);
    this.rider.setCrop(0, 0, this.rider.frame.width, r.cropHeight);
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
