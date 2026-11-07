// RD2.3 (B-714/AC-02, AC-03): Pfeil der Bogenschützen nur in `angriff` bis zum Abschuss, Lavaschleim-Zunge nur in `angriff`.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import '../rig/register';
import { bildVon } from '../render/bild';
import { ladeStil, ROOT } from '../render/stil';
import type { Spec } from '../schema/types';

const stil = ladeStil();
const teileIn = (gruppe: string, id: string, animation: string): string[][] => {
  const pfad = join(ROOT, 'public/grafik', gruppe, `${id}.paper.json`);
  const spec = JSON.parse(readFileSync(pfad, 'utf8')) as Spec;
  return bildVon(spec, { schluessel: animation, animation, stufe: 1 }, { stil, specPfad: pfad }).frames.map((f) => f.geometrie.map((g) => g.teil.id));
};

describe('Pfeil der Bogenschützen (AC-02)', () => {
  it.each([['truppen', 'bogenschuetze'], ['truppen', 'elite-bogenschuetze'], ['gegner', 'goblin-bogenschuetze']])('%s/%s', (gruppe, id) => {
    const angriff = teileIn(gruppe, id, 'angriff').map((t) => t.includes('pfeil'));
    // Phase n/8: Abschuss bei t = 0,7, also Frame 6 (0,75) und 7 (0,875) ohne Pfeil
    expect(angriff).toEqual([true, true, true, true, true, true, false, false]);
    for (const a of ['idle', 'lauf']) expect(teileIn(gruppe, id, a).every((t) => !t.includes('pfeil')), a).toBe(true);
  });
});

describe('Lavaschleim-Zunge (AC-03)', () => {
  it('nur in angriff zwischen t = 0,35 und 0,8', () => {
    const angriff = teileIn('gegner', 'lavaschleim', 'angriff').map((t) => t.includes('zunge'));
    expect(angriff).toEqual([false, false, false, true, true, true, true, false]);
    for (const a of ['idle', 'lauf']) expect(teileIn('gegner', 'lavaschleim', a).every((t) => !t.includes('zunge')), a).toBe(true);
  });
});
