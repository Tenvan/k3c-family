// MG9.4 (B-719): Gegner sind lesbar – Troll mit zwei Beinen und Keule im Lauf, Spinne im Goblin-Maßstab, Wächter aus Kristall, Elite-Bogenschütze größer als Goblin.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { ROOT } from '../render/stil';

const spec = (g: string, id: string) => JSON.parse(readFileSync(join(ROOT, `public/grafik/${g}/${id}.paper.json`), 'utf8')) as Spec;
const hoehe = (g: string, id: string) => spec(g, id).ausgabe!.hoehe_px;

describe('Gegner (MG9.4)', () => {
  it('Troll: Angriff bewegt beide Beine, Lauf trägt die Keule', () => {
    const a = spec('gegner', 'hoehlentroll').animationen!;
    for (const kf of a.angriff.keyframes!) expect(Object.keys(kf.winkel)).toEqual(expect.arrayContaining(['oberschenkel-v', 'oberschenkel-h']));
    expect(a.lauf.keyframes![0].winkel.keule).toBeDefined();
  });

  it('Kristallspinne ist nicht kleiner als 100 px (Goblin-Maßstab)', () => {
    expect(hoehe('gegner', 'kristallspinne')).toBeGreaterThanOrEqual(100);
  });

  it('Kristallwächter: Körper aus Kristall, kein Stein', () => {
    const m = spec('gegner', 'kristallwaechter').teile.flatMap((t) => t.lagen.map((l) => l.material));
    expect(m).toContain('kristall');
    expect(m).not.toContain('stein');
  });

  it('Feuergeist: Rahmen schneidet die Krone nicht ab', () => {
    expect(spec('gegner', 'feuergeist').ausgabe!.rahmen![1]).toBeLessThanOrEqual(-88);
  });

  it('Elite-Bogenschütze ist größer als der Goblin-Bogenschütze', () => {
    expect(hoehe('truppen', 'elite-bogenschuetze')).toBeGreaterThan(hoehe('gegner', 'goblin-bogenschuetze'));
  });
});
