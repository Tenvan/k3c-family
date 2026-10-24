// Suchen, Filtern, Sortieren und Gruppieren beliebiger Listen (B-599). Reine Funktionen ohne React; das Molecule
// `ListFilter` zeigt sie an. Eine Dimension ist eine Eigenschaft der Einträge (Art, Status, Bus …); ihre Werte stehen
// in den Einträgen selbst, es gibt keine feste Liste. Innerhalb einer Dimension gilt ODER, zwischen Dimensionen UND.

export interface FilterDimension<T> {
  key: string;
  label: string;
  /** Wert(e) des Eintrags in dieser Dimension; leer = Eintrag hat keinen Wert (zählt unter „–“). */
  values: (item: T) => string | readonly string[] | undefined;
  /** Anzeige je Wert, Vorgabe der Wert selbst. */
  valueLabel?: (value: string) => string;
  /** Feste Reihenfolge der Chips; unbekannte Werte folgen alphabetisch. */
  order?: readonly string[];
}

export interface SortKey<T> {
  key: string;
  label: string;
  compare: (a: T, b: T) => number;
}

export interface ListFilterConfig<T> {
  /** Text, in dem die Suche (alle Wörter, ohne Groß-/Kleinschreibung) trifft. */
  searchText: (item: T) => string;
  dimensions: readonly FilterDimension<T>[];
  sorts: readonly SortKey<T>[];
}

export interface ListFilterState {
  query: string;
  /** Gewählte Werte je Dimension-Schlüssel; fehlt oder leer = keine Einschränkung. */
  chips: Record<string, string[]>;
  /** Schlüssel aus `sorts`; unbekannt = erste Sortierung. */
  sort: string;
  /** Schlüssel einer Dimension; '' = ungruppiert. */
  group: string;
}

export interface ChipOption {
  value: string;
  label: string;
  /** Treffer, wenn dieser Wert (zusätzlich zu allen anderen Filtern) gewählt wäre. */
  count: number;
  selected: boolean;
}

export interface ItemGroup<T> {
  key: string;
  label: string;
  items: T[];
}

/** Wert für Einträge ohne Wert in der Dimension. */
export const NO_VALUE = '–';

export const emptyState = (sort = ''): ListFilterState => ({ query: '', chips: {}, sort, group: '' });

export function valuesOf<T>(item: T, dim: FilterDimension<T>): string[] {
  const v = dim.values(item);
  const list = v === undefined ? [] : typeof v === 'string' ? [v] : [...v];
  return list.length > 0 ? list : [NO_VALUE];
}

/** Reihenfolge der Werte: erst `order`, dann alphabetisch, „–“ zuletzt. */
function valueOrder<T>(dim: FilterDimension<T>): (a: string, b: string) => number {
  const rank = (v: string) => {
    const i = dim.order?.indexOf(v) ?? -1;
    return v === NO_VALUE ? Number.MAX_SAFE_INTEGER : i < 0 ? Number.MAX_SAFE_INTEGER - 1 : i;
  };
  return (a, b) => rank(a) - rank(b) || a.localeCompare(b, 'de');
}

const words = (q: string) => q.toLowerCase().split(/\s+/).filter(Boolean);

function matchesDimension<T>(item: T, dim: FilterDimension<T>, chosen: readonly string[] | undefined): boolean {
  if (!chosen || chosen.length === 0) return true;
  return valuesOf(item, dim).some((v) => chosen.includes(v));
}

/** Einträge, die Suche und alle Chips erfüllen; `skip` lässt eine Dimension aus (für die Zähler der Chips). */
export function filterItems<T>(items: readonly T[], cfg: ListFilterConfig<T>, state: ListFilterState, skip?: string): T[] {
  const ws = words(state.query);
  return items.filter((item) => {
    if (ws.length > 0) {
      const text = cfg.searchText(item).toLowerCase();
      if (!ws.every((w) => text.includes(w))) return false;
    }
    return cfg.dimensions.every((d) => d.key === skip || matchesDimension(item, d, state.chips[d.key]));
  });
}

/** Chips einer Dimension aus den Werten der Einträge, mit Zähler und Auswahl. */
export function chipOptions<T>(items: readonly T[], cfg: ListFilterConfig<T>, state: ListFilterState, dim: FilterDimension<T>): ChipOption[] {
  const chosen = state.chips[dim.key] ?? [];
  const counts = new Map<string, number>();
  for (const item of items) for (const v of valuesOf(item, dim)) counts.set(v, (counts.get(v) ?? 0) + 1);
  const visible = new Map<string, number>();
  for (const item of filterItems(items, cfg, state, dim.key)) for (const v of valuesOf(item, dim)) visible.set(v, (visible.get(v) ?? 0) + 1);
  return [...counts.keys()]
    .sort(valueOrder(dim))
    .map((value) => ({ value, label: dim.valueLabel?.(value) ?? value, count: visible.get(value) ?? 0, selected: chosen.includes(value) }));
}

/** Sortiert stabil nach dem gewählten Schlüssel; ohne Sortierungen bleibt die Reihenfolge. */
export function sortItems<T>(items: readonly T[], sorts: readonly SortKey<T>[], key: string): T[] {
  const s = sorts.find((x) => x.key === key) ?? sorts[0];
  return s ? [...items].sort(s.compare) : [...items];
}

/** Gruppen nach einer Dimension (erster Wert eines Eintrags); Reihenfolge der Chips, Einträge behalten ihre Reihenfolge. */
export function groupItems<T>(items: readonly T[], dim: FilterDimension<T> | undefined): ItemGroup<T>[] {
  if (!dim) return [{ key: '', label: '', items: [...items] }];
  const map = new Map<string, T[]>();
  for (const item of items) {
    const k = valuesOf(item, dim)[0]!;
    (map.get(k) ?? map.set(k, []).get(k)!).push(item);
  }
  return [...map.keys()]
    .sort(valueOrder(dim))
    .map((k) => ({ key: k, label: dim.valueLabel?.(k) ?? k, items: map.get(k)! }));
}

export interface ListView<T> {
  /** gefiltert und sortiert */
  items: T[];
  groups: ItemGroup<T>[];
  total: number;
  shown: number;
}

export function buildView<T>(items: readonly T[], cfg: ListFilterConfig<T>, state: ListFilterState): ListView<T> {
  const shownItems = sortItems(filterItems(items, cfg, state), cfg.sorts, state.sort);
  const groups = groupItems(shownItems, cfg.dimensions.find((d) => d.key === state.group));
  return { items: shownItems, groups, total: items.length, shown: shownItems.length };
}

export const isFiltered = (state: ListFilterState): boolean =>
  state.query.trim() !== '' || Object.values(state.chips).some((c) => c.length > 0);

export function toggleChip(state: ListFilterState, dim: string, value: string): ListFilterState {
  const cur = state.chips[dim] ?? [];
  const next = cur.includes(value) ? cur.filter((v) => v !== value) : [...cur, value];
  return { ...state, chips: { ...state.chips, [dim]: next } };
}

/** Gemerkte Auswahl prüfen: nur Form; Werte, die es nicht mehr gibt, wirken nicht und stören nicht. */
export function isListFilterState(v: unknown): v is ListFilterState {
  if (typeof v !== 'object' || v === null) return false;
  const s = v as Record<string, unknown>;
  const chips = s.chips as Record<string, unknown> | null;
  return (
    typeof s.query === 'string' && typeof s.sort === 'string' && typeof s.group === 'string' &&
    typeof chips === 'object' && chips !== null &&
    Object.values(chips).every((c) => Array.isArray(c) && c.every((x) => typeof x === 'string'))
  );
}
