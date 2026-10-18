import { describe, expect, it } from 'vitest';
import { depthText, levelColumns, offsetText, rowExtra, rowLabel } from './planerLogic';

describe('Anzeige der Matrix des Gebäude-Planers', () => {
  it('benennt Bauplätze, Mauerlinien, Tore und Händler', () => {
    expect(rowLabel({ kind: 'armory', line: 0 })).toBe('Rüstkammer');
    expect(rowLabel({ kind: 'wall', line: 3 })).toBe('Mauerlinie 3');
    expect(rowLabel({ kind: 'gate', line: 1 })).toBe('Tor Linie 1');
    expect(rowLabel({ kind: 'merchantSell', line: 0 })).toBe('Händler Verkaufen');
    expect(rowLabel({ kind: 'neu', line: 0 })).toBe('neu');
  });

  it('Platz mit Vorzeichen bzw. je Seite', () => {
    expect(offsetText(-16, false)).toBe('−16');
    expect(offsetText(40, false)).toBe('+40');
    expect(offsetText(44.5, true)).toBe('±44.5');
  });

  it('Tiefe und Zusatz', () => {
    expect(depthText({ fromDepth: 0, needsDeeper: false, onlyDepth: -1 })).toBe('');
    expect(depthText({ fromDepth: 1, needsDeeper: true, onlyDepth: -1 })).toBe('ab Tiefe 1, nur mit tieferer Stufe');
    expect(depthText({ fromDepth: 0, needsDeeper: false, onlyDepth: 0 })).toBe('nur Tiefe 0');
    expect(rowExtra({ kind: 'wall', source: 'wallLines', towerUnits: 56, jitterUnits: 4 })).toBe('Turm ±56, 0…+4 je Seed');
    expect(rowExtra({ kind: 'storage', source: 'islandSites', towerUnits: 0, jitterUnits: 0 })).toBe('je Insel');
  });

  it('Spalten für beliebig viele Hub-Stufen', () => {
    expect(levelColumns(7)).toEqual([1, 2, 3, 4, 5, 6, 7]);
    expect(levelColumns(0)).toEqual([]);
  });
});
