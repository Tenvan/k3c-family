import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import { buildingFit, buildingSlot } from '../src/scenes/render/buildingSize';
import { BUILDING_TEXTURES, textureScale } from '../src/scenes/render/buildingSprites';
import { isPaperFile } from '../src/scenes/render/paperAssets';

/** Breite und Höhe aus dem IHDR-Block einer PNG-Datei */
function pngSize(file: string): { w: number; h: number } {
  const b = readFileSync(resolve(__dirname, '../public/grafik', file));
  return { w: b.readUInt32BE(16), h: b.readUInt32BE(20) };
}

// B-685/AC-02: Die Vorbelegung der Zielmaße hält die heutige Darstellung: Jede bisherige Datei eines Gebäude-Slots erscheint
// mit Zielmaß genau so groß und mit demselben Maßstab und Filter wie vorher (Maßstab der Datei, Pixel scharf).
describe('Vorbelegung der Zielmaße (B-685/AC-02)', () => {
  it('heutige Datei im Zielmaß = heutige Darstellung', () => {
    const keys = Object.keys(BUILDING_TEXTURES).filter((k) => buildingSlot(k));
    expect(keys.length).toBeGreaterThanOrEqual(71);
    for (const key of keys) {
      const src = pngSize(BUILDING_TEXTURES[key]!);
      const fit = buildingFit(key, src);
      const today = textureScale(key);
      // Papierschnitt (US1) wird weich gezeichnet, Pixel-Grafik scharf
      const paper = isPaperFile(BUILDING_TEXTURES[key]!);
      expect({ key, w: fit.w, h: fit.h, scale: fit.scale, smooth: fit.smooth }).toEqual({ key, w: src.w * today, h: src.h * today, scale: today, smooth: paper });
    }
  });
});
