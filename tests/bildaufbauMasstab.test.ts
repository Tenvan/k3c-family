// Maßstab nach docs/standards/bildaufbau.md § 2: Figuren und Schiff in Monarchenhöhen (M).
import { readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const ROOT = resolve(__dirname, '..');
const json = (file: string) => JSON.parse(readFileSync(join(ROOT, file), 'utf8'));

const paper = json('data/paper.json');
const M: number = paper.figuren.spieler[0].hoehe;

describe('Bildaufbau-Maßstab (BB1.1)', () => {
  it('Bürger und Truppen sind 0,55–0,65 M hoch', () => {
    for (const [rolle, f] of Object.entries<{ hoehe: number }>(paper.figuren.truppen)) {
      const m = f.hoehe / M;
      expect(m, rolle).toBeGreaterThanOrEqual(0.55);
      expect(m, rolle).toBeLessThanOrEqual(0.65);
    }
  });

  it('Gier ist 0,35–0,45 M hoch', () => {
    const m = paper.figuren.gegner.greed.hoehe / M;
    expect(m).toBeGreaterThanOrEqual(0.35);
    expect(m).toBeLessThanOrEqual(0.45);
  });

  it('Schiff ist mit Segel 2,8–3,2 M hoch (alle Stufen im selben Rahmen und Faktor)', () => {
    const schiff = json('public/grafik/gebaeude-paper/schiff.paper.json');
    const faktor: number = paper.skalierung['gebaeude-paper/schiff'] ?? 1;
    const m = (schiff.ausgabe.hoehe_px * faktor) / M;
    expect(m).toBeGreaterThanOrEqual(2.8);
    expect(m).toBeLessThanOrEqual(3.2);
  });
});
