import { describe, expect, it } from 'vitest';
import { MINE_SYMBOLS, MINE_SYMBOL_TEXTURES, mineSymbol, showsMineSign } from './mineSignView';
import { NATURE_TEXTURES } from './natureSprites';

describe('Abbau-Symbol je Knotenart (B-588)', () => {
  it('Baum zeigt die Axt', () => {
    expect(mineSymbol('tree')).toBe(MINE_SYMBOLS.axe);
  });
  it('Fels, Erzbrocken und Adern zeigen die Spitzhacke', () => {
    for (const kind of ['rock', 'copperOre', 'stoneVein', 'copperVein', 'ironVein', 'crystalVein']) expect(mineSymbol(kind), kind).toBe(MINE_SYMBOLS.pickaxe);
  });
  it('unbekannte Art und Busch: kein Symbol, nie ein falsches', () => {
    expect(mineSymbol('bush')).toBeNull();
    expect(mineSymbol('unbekannt')).toBeNull();
  });
  it('Symbole laden ihre 32-px-Grafik aus tools/buildings (Datei geprüft über data/assets.json)', () => {
    for (const [key, file] of Object.entries(MINE_SYMBOL_TEXTURES)) {
      expect(NATURE_TEXTURES[key]).toBe(file);
    }
  });
});

describe('showsMineSign (B-588)', () => {
  it('nur, wenn der Server minable meldet; ohne Feld nie', () => {
    expect(showsMineSign({ minable: true })).toBe(true);
    expect(showsMineSign({ minable: false })).toBe(false);
    expect(showsMineSign({})).toBe(false);
  });
});
