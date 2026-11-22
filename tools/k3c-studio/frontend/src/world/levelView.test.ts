import { describe, expect, it } from 'vitest';
import { INCOMPLETE, levelModel, objectsIn, plannedAt, seedFromBytes, slotLabel, slotsOf } from './levelView';
import { filesOf, filterRows, groupRows, valueRows, valueText } from './worldOverrides';

describe('Zeichenmodell der Chunk-Vorschau (B-879)', () => {
  it('Plätze relativ zum Hub wie slotIndex in Go', () => {
    expect(slotsOf(['edge', 'forest', 'hub', 'hub', 'rocks', 'edge'])).toEqual([-2, -1, 0, 0, 1, 2]);
    expect(slotsOf(['forest'])).toEqual([0]);
    expect([slotLabel(-2), slotLabel(0), slotLabel(3)]).toEqual(['−2', 'Hub', '+3']);
  });

  it('baut Abschnitte, Objekte und Planung; unvollständig ergibt eine Warnung', () => {
    const m = levelModel({
      seed: 's', biomeId: 'forest', widthUnits: 60, chunkWidthUnits: 20, hubCenterUnits: 30,
      chunks: [{ index: 0, kind: 'forest', startUnits: 0 }, { index: 1, kind: 'hub', startUnits: 20 }, { index: 2, kind: 'rocks', startUnits: 40 }],
      entities: [{ kind: 'tree', x: 45 }, { kind: 'castle', x: 30 }, { kind: 'tree', x: 5 }],
      planned: [{ slot: 1, kind: 'rocks' }], warnings: [],
    });
    expect(m.chunks.map((c) => c.slot)).toEqual([-1, 0, 1]);
    expect(m.counts).toEqual([['tree', 2], ['castle', 1]]);
    expect(objectsIn(m, m.chunks[2]!).map((o) => o.kind)).toEqual(['tree']);
    expect(plannedAt(m, m.chunks[2]!)?.kind).toBe('rocks');
    expect(plannedAt(m, m.chunks[1]!)).toBeUndefined();
    expect(levelModel(null).warnings).toEqual([INCOMPLETE]);
  });

  it('Zufalls-Seed aus Bytes ist stabil', () => {
    expect(seedFromBytes([0, 1, 2, 3, 4, 5, 6, 7])).toBe('abcdefgh');
  });
});

describe('Werte-Zeilen (B-879/AC-02)', () => {
  const rows = valueRows([
    { file: 'buildings', path: ['wall', 'cost'], base: 3, override: 5 },
    { file: 'buildings', path: ['wall', 'hp'], base: 40 },
    { file: 'monarch', path: ['name'], base: 'M' },
  ]);

  const f = (over: Partial<Parameters<typeof filterRows>[1]> = {}) => ({ file: '', query: '', mode: 'alle' as const, touched: new Set<string>(), ...over });

  it('Status, Dateien, Gruppen und Filter', () => {
    expect(rows.map((r) => r.status)).toEqual(['überschrieben', 'geerbt', 'geerbt']);
    expect(rows.map((r) => r.group)).toEqual(['wall', 'wall', 'allgemein']);
    expect(filesOf(rows)).toEqual([{ file: 'buildings', own: 1 }, { file: 'monarch', own: 0 }]);
    expect(filterRows(rows, f({ file: 'buildings', query: 'HP' })).map((r) => r.label)).toEqual(['wall.hp']);
    expect(filterRows(rows, f({ mode: 'ueberschrieben' })).map((r) => r.label)).toEqual(['wall.cost']);
    // Suche trifft auch den Basiswert und mehrere Wörter
    expect(filterRows(rows, f({ query: '40' })).map((r) => r.label)).toEqual(['wall.hp']);
    expect(filterRows(rows, f({ query: 'wall cost' })).map((r) => r.label)).toEqual(['wall.cost']);
    expect(filterRows(rows, f({ mode: 'sitzung', touched: new Set([rows[1]!.key]) })).map((r) => r.label)).toEqual(['wall.hp']);
    expect(groupRows(rows).map((g) => [g.file, g.group, g.rows.length, g.own])).toEqual([['buildings', 'wall', 2, 1], ['monarch', 'allgemein', 1, 0]]);
  });

  it('Anzeige', () => {
    expect([valueText(undefined), valueText('a'), valueText([1])]).toEqual(['–', 'a', '[1]']);
  });
});
