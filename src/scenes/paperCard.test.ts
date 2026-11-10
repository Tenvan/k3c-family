import { describe, expect, it } from 'vitest';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { HUB_ART, ART_BASE } from './hubArtData';
import { menuCategories } from './menuPages';
import { FOREST_LAYERS, STAR_SPOTS, wavyBand } from './paperBackdropData';
import { MATERIAL, SHADOW_LAYERS, cutPolygon, rng, seedOf, shifted, stripHeight, toneOfColor, type MaterialId } from './paperCard';
import { HUB_COLORS, HUB_TEXT, HUB_TEXT_OFF, PAUSE_TILES, START_TILES } from './pauseHub';

const raw = import.meta.glob('../../public/grafik/stil.paper.json', { query: '?raw', import: 'default', eager: true }) as Record<string, string>;
const stil = JSON.parse(Object.values(raw)[0]!) as { materialien: Record<string, { licht: string; basis: string; schatten: string }>; einzeltoene: { linie: string }; papier: { lage: { farbe: string } } };
const hex = (s: string): number => parseInt(s.slice(1), 16);

describe('Papierschnitt-Stil der Kacheln (B-768)', () => {
  it('Materialien sind die des Spiels: gleiche Töne wie in stil.paper.json', () => {
    const names: Record<MaterialId, string> = { gold: 'gold', smaragd: 'smaragd', koenigsblau: 'koenigsblau', koenigsrot: 'koenigsrot', purpur: 'purpur', kristall: 'kristall', kupfer: 'kupfer', stoff: 'stoff', leinen: 'leinen' };
    for (const [id, file] of Object.entries(names)) {
      const m = stil.materialien[file]!;
      expect(MATERIAL[id as MaterialId], id).toEqual({ licht: hex(m.licht), basis: hex(m.basis), schatten: hex(m.schatten) });
    }
  });

  it('Kachelfarben sind Basistöne dieser Materialien, die Schrift ist die Linienfarbe des Stils', () => {
    for (const color of [HUB_COLORS.gold, HUB_COLORS.green, HUB_COLORS.blue, HUB_COLORS.purple, HUB_COLORS.teal, HUB_COLORS.coral, HUB_COLORS.pink]) {
      expect(Object.values(MATERIAL).some((m) => m.basis === color)).toBe(true);
    }
    expect(HUB_TEXT.toLowerCase()).toBe(stil.einzeltoene.linie.toLowerCase());
    expect(toneOfColor(HUB_COLORS.gold)).toBe(MATERIAL.gold);
    expect(toneOfColor(123)).toBe(MATERIAL.stoff);
    expect(HUB_TEXT_OFF).not.toBe(HUB_TEXT);
  });

  it('Schatten hat die Farbe von papier.lage und drei Schichten von blass bis dunkel', () => {
    const alphas = SHADOW_LAYERS.map((l) => l.alpha);
    const offsets = SHADOW_LAYERS.map((l) => l.d);
    expect(alphas).toEqual(alphas.slice().sort((a, b) => a - b));
    expect(offsets).toEqual(offsets.slice().sort((a, b) => b - a));
    expect(stil.papier.lage.farbe.toLowerCase()).toBe('#1a0f14');
  });
});

describe('Handgeschnittene Kante', () => {
  it('ist deterministisch: gleiche Kachel, gleicher Schnitt; andere Kachel, anderer Schnitt', () => {
    const a = cutPolygon(100, 200, 700, 640, seedOf('resume'));
    expect(cutPolygon(100, 200, 700, 640, seedOf('resume'))).toEqual(a);
    expect(cutPolygon(100, 200, 700, 640, seedOf('options'))).not.toEqual(a);
  });

  it('bleibt innerhalb von wobble um das Rechteck, ohne Ecken abzuschneiden', () => {
    const pts = cutPolygon(100, 200, 700, 640, 5, 90, 2.2, 7);
    for (const p of pts) {
      expect(p.x).toBeGreaterThanOrEqual(100 - 2.3);
      expect(p.x).toBeLessThanOrEqual(800 + 2.3);
      expect(p.y).toBeGreaterThanOrEqual(200 - 2.3);
      expect(p.y).toBeLessThanOrEqual(840 + 2.3);
    }
    expect(pts.length).toBeGreaterThan(8); // 8 Ecken plus Punkte entlang der langen Kanten
  });

  it('seedOf und rng liefern Zahlen in [0, 1)', () => {
    const rand = rng(seedOf('x'));
    for (let i = 0; i < 50; i++) {
      const v = rand();
      expect(v).toBeGreaterThanOrEqual(0);
      expect(v).toBeLessThan(1);
    }
    expect(seedOf('a')).not.toBe(seedOf('b'));
  });

  it('shifted verschiebt alle Punkte, stripHeight gibt dem Text Rand', () => {
    expect(shifted([{ x: 1, y: 2 }], 3, 4)).toEqual([{ x: 4, y: 6 }]);
    expect(stripHeight(100)).toBeGreaterThan(100);
  });
});

describe('Illustrationen aus dem Spiel', () => {
  it('jede Kachel der Startseite und des Pausenmenüs hat eine Illustration', () => {
    for (const tile of [...START_TILES, ...PAUSE_TILES]) expect(HUB_ART[tile.id], tile.id).toBeDefined();
  });

  it('jede Kategorie der Seiten hat eine; die Teile liegen innerhalb der Kachel', () => {
    for (const c of menuCategories(true)) if (c.entries.length > 0) expect(HUB_ART[c.id], c.id).toBeDefined();
    for (const art of Object.values(HUB_ART)) for (const { at } of art!.pieces) expect(at > 0 && at < 1).toBe(true);
  });

  it('das Schild lässt Platz: große Kachel weniger als kleine', () => {
    expect(ART_BASE.big).toBeLessThan(ART_BASE.small);
  });
});

describe('Hintergrund', () => {
  it('drei Waldschichten von hinten nach vorn mit steigender Fußlinie, alle aus dem Spiel (hintergruende/hg-wald-*)', () => {
    expect(FOREST_LAYERS.map((l) => l.file)).toEqual(['fern', 'mitte', 'nah'].map((e) => `hintergruende/hg-wald-${e}-v1/stand.png`));
    const feet = FOREST_LAYERS.map((l) => l.y);
    expect(feet).toEqual(feet.slice().sort((a, b) => a - b));
  });

  it('Sterne liegen im Bild; die Papierbahn deckt die ganze Breite und den unteren Rand', () => {
    for (const s of STAR_SPOTS) {
      expect(s.x).toBeGreaterThan(0);
      expect(s.x).toBeLessThan(GAME_WIDTH);
      expect(s.y).toBeLessThan(GAME_HEIGHT);
    }
    const band = wavyBand(430, 3);
    expect(Math.min(...band.map((p) => p.x))).toBeLessThanOrEqual(0);
    expect(Math.max(...band.map((p) => p.x))).toBeGreaterThanOrEqual(GAME_WIDTH);
    expect(Math.max(...band.map((p) => p.y))).toBeGreaterThanOrEqual(GAME_HEIGHT);
    expect(wavyBand(430, 3)).toEqual(band);
  });
});
