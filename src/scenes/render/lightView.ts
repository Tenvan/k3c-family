import Phaser from 'phaser';
import { GROUND_Y, UNIT_PX } from '../../core/constants';
import { clientLog } from '../../core/clientLog';
import { LIGHT } from '../../model/data';
import type { World } from '../../model/types';
import { glowAlpha, lightSources } from './nightLight';

const GLOW_KEY = 'light-glow';
/** Kantenlänge der erzeugten Licht-Textur; gezeichnet wird sie auf den Radius der Quelle skaliert */
const GLOW_PX = 128;
/** Mitte des Lichtscheins über dem Boden (Höhe der Flammen) */
const GLOW_LIFT_PX = 40;
/** Über Figuren und Gebäuden, damit das Licht sie mit aufhellt */
const GLOW_DEPTH = 12;

/** Weicher, runder Lichtschein als kleine Textur (einmal erzeugt): Mitte voll, zum Rand auf 0, kein Shader je Pixel */
function glowTexture(scene: Phaser.Scene): string {
  if (scene.textures.exists(GLOW_KEY)) return GLOW_KEY;
  const tex = scene.textures.createCanvas(GLOW_KEY, GLOW_PX, GLOW_PX)!;
  const c = tex.getContext();
  const r = GLOW_PX / 2;
  const g = c.createRadialGradient(r, r, 0, r, r, r);
  g.addColorStop(0, 'rgba(255,255,255,1)');
  g.addColorStop(0.5, 'rgba(255,255,255,0.45)');
  g.addColorStop(1, 'rgba(255,255,255,0)');
  c.fillStyle = g;
  c.fillRect(0, 0, GLOW_PX, GLOW_PX);
  tex.refresh();
  return GLOW_KEY;
}

/**
 * Licht der Quellen in Nacht und Morgengrauen (B-472, W13.1): je Quelle ein Sprite mit Mischmodus ADD über der Welt, Deckkraft
 * nach `glowAlpha` (am Tag unsichtbar). Die Abdunklung der Kamera (Farbmatrix) liegt danach darüber, jede Kamera zeichnet ihr
 * Licht so selbst (Split-Screen). Liegt in der Ebene der Stufe, Spiel und Welt-Betrachter zeichnen es gleich.
 */
export class LightView {
  private readonly glows = new Map<string, Phaser.GameObjects.Image>();
  private warned = false;

  constructor(
    private readonly scene: Phaser.Scene,
    private readonly root: Phaser.GameObjects.Layer,
  ) {}

  sync(world: World): void {
    const { lit, missing } = lightSources(world);
    if (missing.length > 0 && !this.warned) {
      this.warned = true;
      clientLog('warn', '🤒 Licht-Werte fehlen in data/light.json, Quelle bleibt dunkel', { missing });
    }
    const seen = new Set<string>();
    for (const s of lit) {
      const key = `${s.kind}:${s.x}`;
      seen.add(key);
      const glow = this.glows.get(key) ?? this.create(key, s.x, s.radius);
      const alpha = glowAlpha(s.strength, world.cycle);
      glow.setAlpha(alpha).setVisible(alpha > 0);
    }
    for (const [key, glow] of this.glows) {
      if (seen.has(key)) continue;
      glow.destroy();
      this.glows.delete(key);
    }
  }

  private create(key: string, x: number, radius: number): Phaser.GameObjects.Image {
    const glow = this.scene.add
      .image(x * UNIT_PX, GROUND_Y - GLOW_LIFT_PX, glowTexture(this.scene))
      .setScale((2 * radius * UNIT_PX) / GLOW_PX)
      .setTint(Phaser.Display.Color.HexStringToColor(LIGHT.glowColor).color)
      .setBlendMode(Phaser.BlendModes.ADD)
      .setDepth(GLOW_DEPTH);
    this.root.add(glow);
    this.glows.set(key, glow);
    return glow;
  }
}
