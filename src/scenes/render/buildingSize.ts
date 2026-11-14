import { BUILDING_TEXTURES } from './buildingSprites';
import type { PxSize, TargetFit } from './targetFit';

/**
 * Gebäude-Grafik: Datei und Darstellung je Textur-Schlüssel (`BUILDING_TEXTURES`). Alle Gebäude sind Papierschnitt
 * (`gebaeude-paper/`), 32 Einheiten je Kachel = 32 px, weich gezeichnet (D-MG6-04); das alte Slot-Register
 * `data/assets.json` und sein Zielmaß (B-685) entfielen mit CU1.2. Rein, ohne Phaser.
 */

/** Datei unter `public/grafik/` eines Schlüssels; unbekannter Schlüssel `null` */
export function buildingSource(key: string): { file: string } | null {
  const file = BUILDING_TEXTURES[key];
  return file ? { file } : null; // leer = Papier-Asset fehlt (Warnung in `buildingSprites.ts`)
}

/** Darstellung bei Quellgröße `source`: Maßstab 1, weich */
export function buildingFit(_key: string, source: PxSize): TargetFit {
  return { scale: 1, w: source.w, h: source.h, smooth: true };
}
