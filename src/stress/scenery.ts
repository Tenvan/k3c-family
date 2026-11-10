import Phaser from 'phaser';
import { GROUND_Y } from '../core/constants';
import { nodeSprite } from '../scenes/render/natureSprites';
import { objectImage, preloadWorld, prepareWorld, backgroundLayers } from '../scenes/render/objectView';
import { backgroundFile, backgroundPlan, planKeys, type BackgroundLayerPlan } from '../scenes/render/worldSprites';
import { seeded, type Scenery } from './stressPlan';

/** Art der Vordergrund-Objekte, im Wechsel (Wald: Bäume, Felsen, Büsche aus den Papierschnitt-Objekten) */
const KINDS = ['tree', 'rock', 'bush', 'tree'] as const;
/** Tiefe: hinter den Figuren (Boden ist −1) und davor */
const BEHIND = -0.5;
const FRONT = GROUND_Y + 1000;

export interface SceneryResult {
  /** Verschiedene Bilddateien des Hintergrunds, die geladen wurden */
  backgroundImages: number;
  /** Gezeichnete Vordergrund-Objekte */
  foreground: number;
}

/** Hintergrund-Plan der Szene (`null` = ohne Hintergrund oder Biom ohne Bilder) */
function planOf(s: Scenery, widthPx: number, viewPx: number): BackgroundLayerPlan[] | null {
  return s.background ? backgroundPlan(s.biome, widthPx, viewPx) : null;
}

/** Lädt Objekte und Hintergrund-Bilder; in `preload` der Szene aufrufen. Gibt die Zahl der Hintergrund-Dateien zurück. */
export function preloadScenery(scene: Phaser.Scene, s: Scenery, widthPx: number, viewPx: number): number {
  if (s.foreground > 0) preloadWorld(scene);
  const plan = planOf(s, widthPx, viewPx);
  if (!plan) return 0;
  const keys = planKeys(plan);
  for (const key of keys) {
    const f = backgroundFile(key);
    if (f) scene.load.image(key, `grafik/${f.file}`);
  }
  return keys.length;
}

/**
 * Zeichnet Hintergrund (Parallax-Schichten des Bioms, wie im Spiel) und Vordergrund (Bäume, Felsen, Büsche, je zur Hälfte hinter und vor
 * den Figuren) nach dem Laden. Die Positionen liegen im Bild der Spieler (`centers`) und kommen aus dem Seed, die Zahl bleibt über alle Stufen gleich (die Rampe steigert die Figuren).
 */
export function buildScenery(scene: Phaser.Scene, s: Scenery, widthPx: number, viewPx: number, seed: number, centers: readonly number[]): SceneryResult {
  let backgroundImages = 0;
  const plan = planOf(s, widthPx, viewPx);
  if (plan) {
    for (const key of planKeys(plan)) if (scene.textures.exists(key)) scene.textures.get(key).setFilter(Phaser.Textures.FilterMode.LINEAR);
    const layers = backgroundLayers(scene, plan);
    layers?.forEach((l) => l.container.setDepth(-2));
    backgroundImages = layers ? new Set(layers.flatMap((l) => l.parts.map((p) => p.key))).size : 0;
  }
  let foreground = 0;
  if (s.foreground > 0) {
    prepareWorld(scene);
    const r = seeded(seed ^ 0x5eed);
    for (let i = 0; i < s.foreground; i++) {
      const x = centers[i % centers.length]! + (r() - 0.5) * viewPx;
      const kind = KINDS[i % KINDS.length]!;
      const img = objectImage(scene, `stress:${i}`, x, GROUND_Y + 10 + r() * 50, nodeSprite(kind, i * 3 + 1, s.biome));
      if (!img) continue;
      img.setDepth(i % 2 === 0 ? BEHIND : FRONT + i);
      foreground++;
    }
  }
  return { backgroundImages, foreground };
}
