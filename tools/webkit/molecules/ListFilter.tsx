import { useEffect, useMemo, useRef, useState, type ReactNode } from 'react';
import { Badge, Button, Flex, SelectField, Text, TextField } from '../atoms';
import {
  buildView, chipOptions, emptyState, isFiltered, isListFilterState, toggleChip,
  type ItemGroup, type ListFilterConfig, type ListFilterState,
} from '../lib/listFilter';

/** Merken der Auswahl: passt auf `createPrefs(prefix)` aus lib/prefs.ts, jede App übergibt `{ loadJSON, saveJSON }`. */
export interface ListFilterPrefs {
  loadJSON: <T>(key: string, valid: (v: unknown) => v is T, fallback: T) => T;
  saveJSON: (key: string, value: unknown) => void;
}

export interface ListFilterProps<T> extends ListFilterConfig<T> {
  items: readonly T[];
  /** Einträge einzeln zeichnen (mit Schlüssel) oder je Gruppe gemeinsam (`renderList`, z. B. als Raster); eines von beiden. */
  itemKey?: (item: T) => string;
  renderItem?: (item: T) => ReactNode;
  renderList?: (items: T[]) => ReactNode;
  /** Merken von Chips, Sortierung und Gruppierung unter diesem Schlüssel (`<seite>.<was>`); ohne Schlüssel oder `prefs` nicht. Die Suche wird nie gemerkt. */
  storageKey?: string;
  prefs?: ListFilterPrefs;
  /** Vorgabe ohne gemerkte Auswahl: Schlüssel aus `sorts` (sonst die erste) und Schlüssel einer Dimension zum Gruppieren. */
  defaultSort?: string;
  defaultGroup?: string;
  /** Hinweis bei leerer Liste; Vorgabe „Keine Einträge.“ */
  emptyHint?: ReactNode;
  className?: string;
}

const NO_GROUP = '';
const typing = (el: EventTarget | null) =>
  el instanceof HTMLElement && (el.isContentEditable || ['INPUT', 'TEXTAREA', 'SELECT'].includes(el.tagName));

/** Gemerkte Auswahl, bereinigt um Schlüssel, die es nicht mehr gibt. */
function initialState<T>(p: ListFilterProps<T>): ListFilterState {
  const base: ListFilterState = { ...emptyState(p.sorts.find((s) => s.key === p.defaultSort)?.key ?? p.sorts[0]?.key ?? ''), group: p.dimensions.find((d) => d.key === p.defaultGroup)?.key ?? NO_GROUP };
  if (!p.storageKey || !p.prefs) return base;
  const saved = p.prefs.loadJSON(p.storageKey, isListFilterState, base);
  const chips = Object.fromEntries(Object.entries(saved.chips).filter(([k, v]) => v.length > 0 && p.dimensions.some((d) => d.key === k)));
  return {
    query: '',
    chips,
    sort: p.sorts.some((s) => s.key === saved.sort) ? saved.sort : base.sort,
    group: saved.group === NO_GROUP || p.dimensions.some((d) => d.key === saved.group) ? saved.group : base.group,
  };
}

/** Suchfeld, Chips je Dimension, Sortierung, Gruppierung, Zähler „n von m“ und Zurücksetzen über einer Liste (B-599).
 *  `/` fokussiert die Suche. Die Einträge zeichnet `renderItem`; Gruppen lassen sich einklappen. */
export function ListFilter<T>(props: ListFilterProps<T>) {
  const { items, itemKey, renderItem, renderList, storageKey, prefs, emptyHint = 'Keine Einträge.', className } = props;
  const cfg: ListFilterConfig<T> = props;
  const [state, setState] = useState(() => initialState(props));
  const [closed, setClosed] = useState<ReadonlySet<string>>(new Set());
  const search = useRef<HTMLInputElement>(null);
  const view = useMemo(() => buildView(items, cfg, state), [items, cfg.searchText, cfg.dimensions, cfg.sorts, state]); // eslint-disable-line react-hooks/exhaustive-deps

  useEffect(() => {
    if (storageKey && prefs) prefs.saveJSON(storageKey, { ...state, query: '' });
  }, [state, storageKey, prefs]);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key !== '/' || e.ctrlKey || e.metaKey || e.altKey || typing(e.target)) return;
      e.preventDefault();
      search.current?.focus();
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, []);

  const toggleGroup = (key: string) => setClosed((c) => { const n = new Set(c); if (!n.delete(key)) n.add(key); return n; });
  const reset = () => setState((s) => ({ ...emptyState(s.sort), group: s.group }));

  return (
    <Flex direction="column" gap="2" className={className}>
      <Flex gap="2" align="center" wrap="wrap">
        <TextField.Root ref={search} aria-label="Suchen" placeholder="Suchen (/)" value={state.query} onChange={(e) => setState({ ...state, query: e.target.value })} />
        {cfg.sorts.length > 1 && (
          <SelectField aria-label="Sortierung" value={state.sort} options={cfg.sorts.map((s) => ({ value: s.key, label: `Sortiert: ${s.label}` }))} onChange={(sort) => setState({ ...state, sort })} />
        )}
        {cfg.dimensions.length > 0 && (
          <SelectField aria-label="Gruppierung" value={state.group} options={[{ value: NO_GROUP, label: 'Ungruppiert' }, ...cfg.dimensions.map((d) => ({ value: d.key, label: `Gruppiert: ${d.label}` }))]} onChange={(group) => setState({ ...state, group })} />
        )}
        <Text size="2" color="gray" role="status">{view.shown} von {view.total}</Text>
        {isFiltered(state) && <Button size="1" variant="soft" color="gray" onClick={reset}>Zurücksetzen</Button>}
      </Flex>
      <ChipRows items={items} cfg={cfg} state={state} onToggle={(dim, value) => setState(toggleChip(state, dim, value))} />
      {view.shown === 0 && (
        <Flex gap="2" align="center">
          <Text color="gray">{view.total === 0 ? emptyHint : 'Kein Treffer.'}</Text>
          {view.total > 0 && isFiltered(state) && <Button size="1" variant="soft" onClick={reset}>Filter zurücksetzen</Button>}
        </Flex>
      )}
      {view.groups.map((g) => (
        <GroupBlock key={g.key} group={g} grouped={state.group !== NO_GROUP} open={!closed.has(g.key)} onToggle={() => toggleGroup(g.key)} itemKey={itemKey} renderItem={renderItem} renderList={renderList} />
      ))}
    </Flex>
  );
}

interface ChipRowsProps<T> {
  items: readonly T[];
  cfg: ListFilterConfig<T>;
  state: ListFilterState;
  onToggle: (dim: string, value: string) => void;
}

/** Eine Zeile Chips je Dimension; Dimensionen mit nur einem Wert bleiben aus, solange nichts gewählt ist. */
function ChipRows<T>({ items, cfg, state, onToggle }: ChipRowsProps<T>) {
  return (
    <>
      {cfg.dimensions.map((d) => {
        const opts = chipOptions(items, cfg, state, d);
        if (opts.length < 2 && !opts.some((o) => o.selected)) return null;
        return (
          <Flex key={d.key} gap="1" align="center" wrap="wrap" role="group" aria-label={d.label}>
            <Text size="1" color="gray">{d.label}</Text>
            {opts.map((o) => (
              <Button key={o.value} size="1" variant={o.selected ? 'solid' : 'soft'} color={o.selected ? undefined : 'gray'} aria-pressed={o.selected} disabled={o.count === 0 && !o.selected} onClick={() => onToggle(d.key, o.value)}>
                {o.label} <Badge size="1" variant="outline">{o.count}</Badge>
              </Button>
            ))}
          </Flex>
        );
      })}
    </>
  );
}

interface GroupBlockProps<T> {
  group: ItemGroup<T>;
  grouped: boolean;
  open: boolean;
  onToggle: () => void;
  itemKey?: (item: T) => string;
  renderItem?: (item: T) => ReactNode;
  renderList?: (items: T[]) => ReactNode;
}

/** Eine Gruppe mit einklappbarer Überschrift; ungruppiert nur die Einträge. */
function GroupBlock<T>({ group, grouped, open, onToggle, itemKey, renderItem, renderList }: GroupBlockProps<T>) {
  return (
    <Flex direction="column" gap="1" data-group={group.key}>
      {grouped && (
        <Button size="1" variant="ghost" color="gray" aria-expanded={open} onClick={onToggle} style={{ justifyContent: 'flex-start' }}>
          {open ? '▾' : '▸'} {group.label} ({group.items.length})
        </Button>
      )}
      {open && renderList?.(group.items)}
      {open && !renderList && group.items.map((item, i) => <div key={itemKey?.(item) ?? i}>{renderItem?.(item)}</div>)}
    </Flex>
  );
}
