import { useEffect, useState } from 'react';
import { loadJSON, saveJSON } from '../lib/prefs';
import {
  BUILTIN_CHIPS, FACET_KEYS, GRUPPEN, SICHTEN, facetsOf, noFacets, type CustomChip, type Facets, type FocusChip, type Gruppe, type Sicht,
} from './filter';

// Zustand der Assets-Seite (B-677): Facetten, Sicht, Gruppierung und Suche; die Suche wird nicht gemerkt (Freitext), der Rest
// übersteht ein Neuladen. Eigene Fokus-Chips liegen getrennt, damit ein defekter Chip die Ansicht nicht stört.

export interface ViewState {
  query: string;
  facets: Facets;
  sicht: Sicht;
  gruppe: Gruppe;
}

export const DEFAULT_VIEW: ViewState = { query: '', facets: noFacets(), sicht: 'slots', gruppe: 'kategorie' };

const VIEW_KEY = 'assets.view';
const CHIPS_KEY = 'assets.chips';

const isList = (v: unknown): v is string[] => Array.isArray(v) && v.every((x) => typeof x === 'string');
const isFacets = (v: unknown): v is Facets => !!v && typeof v === 'object' && FACET_KEYS.every((k) => isList((v as Record<string, unknown>)[k]));

export function isViewState(v: unknown): v is ViewState {
  const s = v as Partial<ViewState> | null;
  return !!s && isFacets(s.facets) && SICHTEN.includes(s.sicht as Sicht) && GRUPPEN.includes(s.gruppe as Gruppe);
}

export function isCustomChip(v: unknown): v is CustomChip {
  const c = v as Partial<CustomChip> | null;
  return !!c && typeof c.id === 'string' && typeof c.label === 'string' && !!c.facets && typeof c.facets === 'object'
    && FACET_KEYS.every((k) => c.facets![k] === undefined || isList(c.facets![k])) && SICHTEN.includes(c.sicht as Sicht) && GRUPPEN.includes(c.gruppe as Gruppe) && typeof c.query === 'string';
}

const isChips = (v: unknown): v is CustomChip[] => Array.isArray(v) && v.every(isCustomChip);

/** Gemerkte Ansicht (ohne Suche); ein veralteter Wert gilt als nicht gemerkt. */
export const loadView = (): ViewState => ({ ...loadJSON(VIEW_KEY, isViewState, DEFAULT_VIEW), query: '' });

/** Ansicht der Seite; jede Änderung außer der Suche wird gemerkt. */
export function useViewState(): [ViewState, (patch: Partial<ViewState>) => void] {
  const [view, setView] = useState<ViewState>(loadView);
  useEffect(() => saveJSON(VIEW_KEY, { ...view, query: '' }), [view]);
  return [view, (patch) => setView((v) => ({ ...v, ...patch }))];
}

/** Eigene Chips, gemerkt. */
export function useCustomChips(): { chips: CustomChip[]; add: (c: CustomChip) => void; remove: (id: string) => void } {
  const [chips, setChips] = useState<CustomChip[]>(() => loadJSON(CHIPS_KEY, isChips, []));
  useEffect(() => saveJSON(CHIPS_KEY, chips), [chips]);
  return { chips, add: (c) => setChips((l) => [...l.filter((x) => x.id !== c.id), c]), remove: (id) => setChips((l) => l.filter((c) => c.id !== id)) };
}

/** Alle Chips der Leiste: feste, dann eigene. */
export const allChips = (custom: readonly CustomChip[]): FocusChip[] => [...BUILTIN_CHIPS, ...custom];

/** Ansicht nach Klick auf einen Chip: dessen Facetten; Suche bleibt, Sicht und Gruppierung bleiben bei festen Chips. */
export function applyChip(view: ViewState, chip: FocusChip): ViewState {
  const own = chip as Partial<CustomChip>;
  return { ...view, facets: facetsOf(chip), ...(own.sicht && { sicht: own.sicht, gruppe: own.gruppe ?? view.gruppe, query: own.query ?? view.query }) };
}

/** Chip mit freier ID: ein anderer Name, der zur selben ID führt („Foo!“ und „foo?“), überschreibt den vorhandenen nicht still. */
export function uniqueChip(chip: CustomChip, existing: readonly CustomChip[]): CustomChip {
  let id = chip.id;
  for (let n = 2; existing.some((c) => c.id === id && c.label !== chip.label); n += 1) id = `${chip.id}-${n}`;
  return { ...chip, id };
}

/** Neuer eigener Chip aus dem aktuellen Zustand. */
export const chipFromView = (name: string, view: ViewState): CustomChip => ({
  id: `eigen:${name.toLowerCase().replace(/[^a-z0-9äöüß]+/g, '-')}`, label: name, facets: { ...view.facets }, sicht: view.sicht, gruppe: view.gruppe, query: view.query,
});
