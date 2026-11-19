// Maßstab nach docs/standards/bildaufbau.md § 2: Figuren, Schiff und Hub-Gebäude in Monarchenhöhen (M).
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

// BB1.2: Hub-Gebäude. Höhe = ausgabe.hoehe_px × skalierung ÷ M (Gebäude zeichnen 1 : 1 in Spiel-px), Toleranz 20 %.
const KLASSEN: Record<string, { min: number; max: number; ids: string[] }> = {
  burg: { min: 3.5, max: 4.5, ids: ['burg', 'burg-ausbau'] },
  turm: { min: 2, max: 2.5, ids: ['turm', 'pagode'] },
  mauer: { min: 0.8, max: 1.2, ids: ['mauer', 'tor'] },
  werkstatt: {
    min: 1,
    max: 1.4,
    ids: [
      'schmiede', 'taverne', 'werkstatt', 'kaserne', 'bogner', 'barke', 'lager', 'ruestkammer', 'heilplatz',
      'dojo', 'schrein', 'einsiedler-huette', 'farm', 'rundhaus', 'rekrutierungslager', 'hafen',
    ],
  },
};
const TOLERANZ = 0.2;

// Ausnahmen mit Begründung (Asset → Grund); sie zählen nicht gegen die Klasse. Derzeit keine: alles liegt in Toleranz.
const AUSNAHMEN: Record<string, string> = {};

const hoeheM = (id: string): number => {
  const spec = json(`public/grafik/gebaeude-paper/${id}.paper.json`);
  const faktor: number = paper.skalierung[`gebaeude-paper/${id}`] ?? 1;
  return (spec.ausgabe.hoehe_px * faktor) / M;
};

describe('Hub-Gebäude-Maßstab (BB1.2)', () => {
  for (const [klasse, k] of Object.entries(KLASSEN)) {
    it(`${klasse}: ${k.min}–${k.max} M (±20 %)`, () => {
      for (const id of k.ids) {
        if (AUSNAHMEN[id]) continue;
        const m = hoeheM(id);
        expect(m, id).toBeGreaterThanOrEqual(k.min * (1 - TOLERANZ));
        expect(m, id).toBeLessThanOrEqual(k.max * (1 + TOLERANZ));
      }
    });
  }

  it('Burg ist das größte Gebäude des Hubs', () => {
    const alle = Object.values(KLASSEN).flatMap((k) => k.ids);
    const burg = Math.min(hoeheM('burg'), hoeheM('burg-ausbau'));
    for (const id of alle.filter((i) => !i.startsWith('burg'))) expect(burg, id).toBeGreaterThan(hoeheM(id));
  });

  it('Gerüste skalieren wie ihr Gebäude (Burg, Mauer)', () => {
    const f = (id: string): number => paper.skalierung[`gebaeude-paper/${id}`] ?? 1;
    expect(f('hub-geruest')).toBe(f('burg'));
    expect(f('geruest-1')).toBe(f('mauer'));
  });
});

// BB1.3: Waldbäume in drei Höhen je Art plus dunkler Variante, Waldwand im Mittelgrund (Standard § 2, § 5).
const BAEUME: Record<string, { min: number; max: number; ids: string[] }> = {
  tanne: { min: 4, max: 6, ids: ['tanne', 'tanne-mittel', 'tanne-hoch', 'tanne-dunkel'] },
  pappel: { min: 2.5, max: 3.5, ids: ['pappel-niedrig', 'pappel', 'pappel-hoch', 'pappel-dunkel'] },
  birke: { min: 2.5, max: 3.5, ids: ['birke', 'birke-mittel', 'birke-hoch', 'birke-dunkel'] },
};

// Höhe = rahmen-Höhe (Spiel-px) × skalierung ÷ M.
const baumHoeheM = (id: string): number => {
  const e = paper.objekte[id];
  const faktor: number = paper.skalierung[`objekte/${id}`] ?? 1;
  return (e.rahmen[3] * faktor) / M;
};

describe('Waldbäume und Waldwand (BB1.3)', () => {
  for (const [art, k] of Object.entries(BAEUME)) {
    it(`${art}: ${k.min}–${k.max} M, mindestens drei verschiedene Höhen`, () => {
      const hoehen = k.ids.map((id) => {
        const m = baumHoeheM(id);
        expect(m, id).toBeGreaterThanOrEqual(k.min);
        expect(m, id).toBeLessThanOrEqual(k.max);
        return Math.round(m * 10);
      });
      expect(new Set(hoehen).size).toBeGreaterThanOrEqual(3);
    });

    it(`${art}: dunkle Variante im Katalog, alle Specs vorhanden`, () => {
      expect(k.ids.some((id) => id.endsWith('-dunkel'))).toBe(true);
      for (const id of k.ids) {
        expect(paper.objekte[id], id).toBeDefined();
        expect(() => json(`public/grafik/objekte/${id}.paper.json`), id).not.toThrow();
      }
    });
  }

  it('waldwand zeigt auf eine vorhandene Spec in hintergruende', () => {
    const ziel: string = paper.waldwand;
    expect(ziel).toMatch(/^hintergruende\//);
    const spec = json(`public/grafik/${ziel}.paper.json`);
    expect(spec.rig).toBe('hintergrund');
    expect(spec.ausgabe.hoehe_px).toBe(480);
  });
});
