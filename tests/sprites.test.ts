import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import enemies from '../src/data/enemies.json';
import sprites from '../src/data/sprites.json';
import troops from '../src/data/troops.json';

/** Breite/Höhe aus dem PNG-Header (IHDR) lesen */
function pngSize(file: string): [number, number] {
  const b = readFileSync(file);
  return [b.readUInt32BE(16), b.readUInt32BE(20)];
}

const sheets = sprites.sheets as Record<string, { frameWidth: number; frameHeight: number; anims: Record<string, { frames: number }> }>;

describe('sprites.json', () => {
  it('jede Animation hat ein Bild mit passender Frame-Anzahl', () => {
    for (const [name, sheet] of Object.entries(sheets)) {
      expect(sheet.anims.idle, `${name}: idle fehlt`).toBeDefined();
      for (const [anim, a] of Object.entries(sheet.anims)) {
        const [w, h] = pngSize(resolve('public/sprites', name, `${anim}.png`));
        expect([w, h], `${name}/${anim}.png`).toEqual([a.frames * sheet.frameWidth, sheet.frameHeight]);
      }
    }
  });

  it('jede Truppe, jeder Gegner und jeder Spieler hat ein vorhandenes Sprite', () => {
    const specs = [...sprites.players, ...Object.values(sprites.troops), ...Object.values(sprites.enemies)];
    for (const s of specs) expect(sheets[s.sheet], s.sheet).toBeDefined();
    for (const kind of Object.keys(troops)) expect(sprites.troops, kind).toHaveProperty(kind);
    for (const kind of Object.keys(enemies)) expect(sprites.enemies, kind).toHaveProperty(kind);
  });
});
