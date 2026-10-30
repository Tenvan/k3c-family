import { describe, expect, it } from 'vitest';
import { mockHubPlan } from '../api/mockHub';
import { depthText, levelColumns, offsetText, rowAt, rowExtra, rowLabel, rowPlace } from './plannerLogic';

describe('Anzeige der Matrix des Gebäude-Planers', () => {
  it('benennt Bauplätze, den Baugrund der Zone 1 und Händler', () => {
    expect(rowLabel({ kind: 'armory', line: 0 })).toBe('Rüstkammer');
    expect(rowLabel({ kind: 'wall', line: 1 })).toBe('Baugrund Zone 1');
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
    expect(rowExtra({ kind: 'wall', source: 'sitePlacement', towerUnits: 52, jitterUnits: 0 })).toBe('Turm ±52');
    expect(rowExtra({ kind: 'storage', source: 'islandSites', towerUnits: 0, jitterUnits: 0 })).toBe('je Insel');
  });

  it('Spalten für beliebig viele Hub-Stufen', () => {
    expect(levelColumns(7)).toEqual([1, 2, 3, 4, 5, 6, 7]);
    expect(levelColumns(0)).toEqual([]);
  });
});

describe('Zeile zu einem Bauplatz der Bühne (B-670)', () => {
  const rows = mockHubPlan().matrix.rows;
  const at = (kind: string, x: number, level = 7) => rowAt(rows, level, { kind, x }, 450)?.key;

  it('gleiche Art und Abstand zur Hub-Mitte; Turm gehört zum Baugrund; sonst keine Zeile', () => {
    expect(at('tavern', 418)).toBe('tavern@-32');
    expect(at('armory', 490)).toBe('armory@40');
    expect(at('wall', 390)).toBe('zone1');
    expect(at('wall', 510)).toBe('zone1');
    expect(at('tower', 398)).toBe('zone1');
    expect(at('wall', 300)).toBeUndefined();
    expect(at('armory', 490, 6)).toBeUndefined(); // in Stufe 6 noch nicht verfügbar
  });

  it('Platz und Liste unter dem Namen', () => {
    expect(rowPlace(rows.find((r) => r.key === 'armory@40')!, 7)).toBe('+40 · sites');
    expect(rowPlace(rows.find((r) => r.key === 'storage@-26')!, 7)).toBe('−26 · islandSites');
    expect(rowPlace(rows.find((r) => r.key === 'zone1')!, 7)).toBe('±60 · Baugrund');
  });
});
