import { describe, expect, it } from 'vitest';
import {
  buildView, chipOptions, emptyState, filterItems, groupItems, isFiltered, isListFilterState, NO_VALUE, sortItems, toggleChip,
  type FilterDimension, type ListFilterConfig,
} from './listFilter';

interface Row {
  id: string;
  art: string;
  status?: string;
  tags?: string[];
}

const art: FilterDimension<Row> = { key: 'art', label: 'Art', values: (r) => r.art, order: ['sfx', 'musik'] };
const status: FilterDimension<Row> = { key: 'status', label: 'Status', values: (r) => r.status };
const tags: FilterDimension<Row> = { key: 'tags', label: 'Tags', values: (r) => r.tags };
const cfg: ListFilterConfig<Row> = {
  searchText: (r) => `${r.id} ${r.art}`,
  dimensions: [art, status, tags],
  sorts: [
    { key: 'name', label: 'Name', compare: (a, b) => a.id.localeCompare(b.id) },
    { key: 'art', label: 'Art', compare: (a, b) => a.art.localeCompare(b.art) },
  ],
};
const rows: Row[] = [
  { id: 'b-hit', art: 'sfx', status: 'Lücke', tags: ['heiler', 'nah'] },
  { id: 'a-bow', art: 'sfx', status: 'zugeordnet' },
  { id: 'c-tag', art: 'musik', status: 'Lücke', tags: ['heiler'] },
];
const ids = (list: Row[]) => list.map((r) => r.id);

describe('listFilter', () => {
  it('Suche: alle Wörter, ohne Groß-/Kleinschreibung', () => {
    expect(ids(filterItems(rows, cfg, { ...emptyState(), query: 'SFX hit' }))).toEqual(['b-hit']);
    expect(filterItems(rows, cfg, { ...emptyState(), query: '  ' })).toHaveLength(3);
  });

  it('Chips: ODER in einer Dimension, UND zwischen Dimensionen, Mehrfachwerte', () => {
    let s = toggleChip(emptyState(), 'status', 'Lücke');
    expect(ids(filterItems(rows, cfg, s))).toEqual(['b-hit', 'c-tag']);
    s = toggleChip(toggleChip(s, 'tags', 'heiler'), 'art', 'musik');
    expect(ids(filterItems(rows, cfg, s))).toEqual(['c-tag']);
    s = toggleChip(toggleChip(s, 'art', 'musik'), 'art', 'sfx');
    expect(ids(filterItems(rows, cfg, s))).toEqual(['b-hit']);
  });

  it('Einträge ohne Wert zählen unter „–“', () => {
    expect(ids(filterItems(rows, cfg, toggleChip(emptyState(), 'tags', NO_VALUE)))).toEqual(['a-bow']);
  });

  it('Chips: Reihenfolge, Zähler berücksichtigen die anderen Filter, Auswahl', () => {
    const s = toggleChip(emptyState(), 'status', 'Lücke');
    expect(chipOptions(rows, cfg, s, art).map((o) => [o.value, o.count])).toEqual([['sfx', 1], ['musik', 1]]);
    expect(chipOptions(rows, cfg, s, status).find((o) => o.value === 'Lücke')).toMatchObject({ count: 2, selected: true });
    expect(chipOptions(rows, cfg, emptyState(), tags).map((o) => o.value)).toEqual(['heiler', 'nah', NO_VALUE]);
  });

  it('Sortierung stabil, unbekannter Schlüssel = erste', () => {
    expect(ids(sortItems(rows, cfg.sorts, 'name'))).toEqual(['a-bow', 'b-hit', 'c-tag']);
    expect(ids(sortItems(rows, cfg.sorts, 'art'))).toEqual(['c-tag', 'b-hit', 'a-bow']);
    expect(sortItems(rows, cfg.sorts, 'xx')[0]!.id).toBe('a-bow');
    expect(sortItems(rows, [], 'x')).toEqual(rows);
  });

  it('Gruppen nach Dimension in Chip-Reihenfolge, ohne Dimension eine Gruppe', () => {
    expect(groupItems(rows, art).map((g) => [g.key, g.items.length])).toEqual([['sfx', 2], ['musik', 1]]);
    expect(groupItems(rows, undefined)).toHaveLength(1);
  });

  it('buildView: gefiltert, sortiert, gruppiert, Zähler', () => {
    const v = buildView(rows, cfg, { ...toggleChip(emptyState('name'), 'status', 'Lücke'), group: 'art' });
    expect(v).toMatchObject({ total: 3, shown: 2 });
    expect(v.groups.map((g) => ids(g.items))).toEqual([['b-hit'], ['c-tag']]);
  });

  it('isFiltered und isListFilterState', () => {
    expect(isFiltered(emptyState())).toBe(false);
    expect(isFiltered(toggleChip(toggleChip(emptyState(), 'art', 'sfx'), 'art', 'sfx'))).toBe(false);
    expect(isFiltered({ ...emptyState(), query: 'x' })).toBe(true);
    expect(isListFilterState(emptyState())).toBe(true);
    expect(isListFilterState({ query: 1 })).toBe(false);
    expect(isListFilterState(null)).toBe(false);
  });

  it('1000 Einträge: Filter, Zähler, Sortierung und Gruppen in unter 200 ms', () => {
    const many: Row[] = Array.from({ length: 1000 }, (_, i) => ({ id: `slot-${i}`, art: i % 2 ? 'sfx' : 'musik', status: i % 3 ? 'Lücke' : 'zugeordnet', tags: [`t${i % 7}`] }));
    const t0 = performance.now();
    const s = { ...toggleChip(emptyState('name'), 'status', 'Lücke'), query: 'slot-1', group: 'art' };
    const v = buildView(many, cfg, s);
    for (const d of cfg.dimensions) chipOptions(many, cfg, s, d);
    expect(performance.now() - t0).toBeLessThan(200);
    expect(v.shown).toBeGreaterThan(50);
    expect(v.shown).toBeLessThan(1000);
  });
});
