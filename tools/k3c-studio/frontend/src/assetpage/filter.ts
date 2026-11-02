import type { Facet } from '@webkit/organisms';
import { CATEGORIES } from '../assets/assets';
import { MARK_LABEL } from '../lib/license';
import type { FileRow, SlotRow } from './model';

// Filter der Assets-Seite (B-677): Facetten (je Gruppe „oder“, Gruppen untereinander „und“), Suche, Fokus-Chips als
// gespeicherte Facetten, Sicht und Gruppierung. Reine Funktionen; Zustand und Merken stehen in viewState.ts.

export const FACET_KEYS = ['art', 'kategorie', 'status', 'stufe', 'quelle', 'lizenz'] as const;
export type FacetKey = (typeof FACET_KEYS)[number];
export type Facets = Record<FacetKey, string[]>;

export const SICHTEN = ['slots', 'dateien', 'szene'] as const;
export type Sicht = (typeof SICHTEN)[number];
export const GRUPPEN = ['kategorie', 'bereich', 'quelle', 'pack', 'slot'] as const;
export type Gruppe = (typeof GRUPPEN)[number];

export const SICHT_LABEL: Record<Sicht, string> = { slots: 'Slots', dateien: 'Dateien', szene: 'Szene' };
export const GRUPPE_LABEL: Record<Gruppe, string> = { kategorie: 'Kategorie', bereich: 'Spielbereich', quelle: 'Quelle', pack: 'Pack', slot: 'Slot' };

export const noFacets = (): Facets => ({ art: [], kategorie: [], status: [], stufe: [], quelle: [], lizenz: [] });

/** Was Slots und Dateien gemeinsam für Filter und Facetten liefern; bei Dateien je ein Wert, bei Slots die ihrer Dateien. */
export interface Item {
  art: string;
  kategorie: string;
  status: string;
  stufen: readonly string[];
  quellen: readonly string[];
  marken: readonly string[];
  text: string;
}

export function slotItem(r: SlotRow): Item {
  const text = [r.slot.id, r.slot.beschreibung, r.bereich, ...r.slot.wahl.map((c) => c.datei), ...r.files.map((f) => `${f.titel} ${f.quelle}`)].join(' ');
  return {
    art: r.slot.art, kategorie: r.slot.kategorie, status: r.status, stufen: [...new Set(r.files.map((f) => f.stufe))], quellen: [...new Set(r.files.map((f) => f.quelle))],
    marken: [...new Set(r.files.map((f) => f.marke))], text: text.toLowerCase(),
  };
}

export const fileItem = (f: FileRow): Item => ({
  art: f.art, kategorie: f.kategorie, status: f.status, stufen: [f.stufe], quellen: [f.quelle], marken: [f.marke],
  text: [f.slotId, f.titel, f.datei, f.quelle, f.pack, f.lizenz].join(' ').toLowerCase(),
});

const overlaps = (wanted: readonly string[], have: readonly string[]): boolean => wanted.length === 0 || wanted.some((w) => have.includes(w));

/** Trifft der Eintrag Suche und Facetten? `skip` lässt eine Gruppe aus (für die Zähler der Facette). */
export function matches(it: Item, facets: Facets, query: string, skip?: FacetKey): boolean {
  const f = (k: FacetKey) => k === skip || overlaps(facets[k], k === 'art' || k === 'kategorie' || k === 'status' ? [it[k]] : k === 'stufe' ? it.stufen : k === 'quelle' ? it.quellen : it.marken);
  if (!FACET_KEYS.every(f)) return false;
  return query.toLowerCase().split(/\s+/).filter(Boolean).every((w) => it.text.includes(w));
}

/** Facetten-Gruppen mit Zählern: Zahl der Einträge, die mit den übrigen Facetten und der Suche zu dieser Option passen. */
export function facetGroups(items: readonly Item[], facets: Facets, query: string, quellen: readonly string[]): Facet[] {
  const count = (key: FacetKey, value: string): number => items.filter((it) => matches(it, { ...facets, [key]: [value] }, query)).length;
  const opt = (key: FacetKey, values: readonly string[], label: (v: string) => string = (v) => v) => values.map((value) => ({ value, label: label(value), count: count(key, value) }));
  const kategorien = [...CATEGORIES.grafik, ...CATEGORIES.sound];
  return [
    { id: 'art', title: 'Art', options: opt('art', ['grafik', 'sound'], (v) => (v === 'sound' ? 'Sound' : 'Grafik')) },
    { id: 'kategorie', title: 'Kategorie', options: opt('kategorie', kategorien) },
    { id: 'status', title: 'Status', options: opt('status', ['aktiv', 'luecke', 'fehlt'], (v) => ({ aktiv: 'aktiv', luecke: 'Lücke', fehlt: 'Datei fehlt' })[v] ?? v) },
    { id: 'stufe', title: 'Stufe', options: opt('stufe', ['netz', 'vorrat', 'spiel'], (v) => ({ netz: 'Netz', vorrat: 'Vorrat', spiel: 'Spiel' })[v] ?? v) },
    { id: 'quelle', title: 'Quelle', options: byCount(opt('quelle', quellen), facets.quelle) },
    { id: 'lizenz', title: 'Lizenz', options: opt('lizenz', ['frei', 'sa', 'pruefen', 'gesperrt'], (v) => MARK_LABEL[v as keyof typeof MARK_LABEL] ?? v) },
  ];
}

/** Viele Optionen (Quellen): die mit Treffern oder Auswahl, häufigste zuerst. */
function byCount<T extends { value: string; count?: number }>(options: T[], chosen: readonly string[]): T[] {
  return options.filter((o) => (o.count ?? 0) > 0 || chosen.includes(o.value)).sort((a, b) => (b.count ?? 0) - (a.count ?? 0) || a.value.localeCompare(b.value, 'de'));
}

/** Quellen aller Dateien, sortiert. */
export const quellenOf = (files: readonly FileRow[]): string[] => [...new Set(files.map((f) => f.quelle))].sort((a, b) => a.localeCompare(b, 'de'));

// ── Fokus-Chips ──────────────────────────────────────────────────────────────

export interface FocusChip {
  id: string;
  label: string;
  facets: Partial<Facets>;
}

/** Eigener Chip („Ansicht merken“): Facetten, Sicht, Gruppierung und Suche zur Zeit des Merkens. */
export interface CustomChip extends FocusChip {
  sicht: Sicht;
  gruppe: Gruppe;
  query: string;
}

export const BUILTIN_CHIPS: readonly FocusChip[] = [
  { id: 'alles', label: 'Alles', facets: {} },
  { id: 'grafik', label: 'Grafik', facets: { art: ['grafik'] } },
  { id: 'sound', label: 'Sound', facets: { art: ['sound'] } },
  { id: 'luecken', label: 'Lücken', facets: { status: ['luecke', 'fehlt'] } },
  { id: 'netz', label: 'Aus dem Netz', facets: { stufe: ['netz'] } },
  { id: 'pruefen', label: 'Lizenz prüfen', facets: { lizenz: ['pruefen'] } },
  { id: 'vorrat', label: 'Vorrat', facets: { stufe: ['vorrat'] } },
];

export const facetsOf = (chip: FocusChip): Facets => ({ ...noFacets(), ...chip.facets });

const sameSet = (a: readonly string[], b: readonly string[]): boolean => a.length === b.length && a.every((x) => b.includes(x));
export const sameFacets = (a: Facets, b: Facets): boolean => FACET_KEYS.every((k) => sameSet(a[k], b[k]));

/** Der Chip, dessen Facetten genau den aktuellen entsprechen (eigene vor festen); "" = keiner. */
export function activeChip(facets: Facets, chips: readonly FocusChip[]): string {
  return [...chips].reverse().find((c) => sameFacets(facetsOf(c), facets))?.id ?? ''; // eigene Chips stehen hinten
}

/** Entfernt Facetten-Werte, die es nicht mehr gibt (z. B. eine gelöschte Quelle in einem eigenen Chip); `dropped` nennt sie für den Hinweis. */
export function pruneFacets(f: Facets, quellen: readonly string[]): { facets: Facets; dropped: string[] } {
  const known: Record<FacetKey, readonly string[] | null> = { art: ['grafik', 'sound'], kategorie: [...CATEGORIES.grafik, ...CATEGORIES.sound], status: ['aktiv', 'luecke', 'fehlt'], stufe: ['netz', 'vorrat', 'spiel'], quelle: quellen, lizenz: ['frei', 'sa', 'pruefen', 'gesperrt'] };
  const dropped: string[] = [];
  const out = noFacets();
  for (const k of FACET_KEYS) {
    out[k] = f[k].filter((v) => (known[k]!.includes(v) ? true : (dropped.push(v), false)));
  }
  return { facets: out, dropped };
}

/** Zahl der Einträge je Chip bei der Suche `query`. */
export const chipCount = (items: readonly Item[], chip: FocusChip, query: string): number => items.filter((it) => matches(it, facetsOf(chip), query)).length;

/** Hat ein Facetten-Satz irgendeine Auswahl? */
export const hasFacets = (f: Facets): boolean => FACET_KEYS.some((k) => f[k].length > 0);

/** Namen für einen neuen eigenen Chip aus den gewählten Facetten-Werten („grafik · luecke“), sonst „Ansicht“; der Name ist frei änderbar. */
export function suggestName(f: Facets, taken: readonly string[]): string {
  const base = FACET_KEYS.flatMap((k) => f[k]).join(' · ') || 'Ansicht';
  let name = base;
  for (let n = 2; taken.includes(name); n += 1) name = `${base} ${n}`;
  return name;
}

// ── Gruppen ──────────────────────────────────────────────────────────────────

export interface Group<T> {
  key: string;
  title: string;
  items: T[];
}

/** Gruppiert; die Gruppen stehen alphabetisch, Einträge in der Reihenfolge der Eingabe. `of` = null: keine Gruppen. */
export function groupItems<T>(items: readonly T[], of: ((t: T) => string) | null): Group<T>[] {
  if (!of) return items.length ? [{ key: '', title: '', items: [...items] }] : [];
  const by = new Map<string, T[]>();
  for (const it of items) by.set(of(it), [...(by.get(of(it)) ?? []), it]);
  return [...by.entries()].map(([key, list]) => ({ key, title: key, items: list })).sort((a, b) => a.title.localeCompare(b.title, 'de'));
}

export const slotGroup = (g: Gruppe): ((r: SlotRow) => string) | null =>
  g === 'slot' ? null : g === 'kategorie' ? (r) => r.slot.kategorie : g === 'bereich' ? (r) => r.bereich : g === 'quelle' ? (r) => r.quelle : (r) => r.pack;

export const fileGroup = (g: Gruppe): ((f: FileRow) => string) =>
  g === 'slot' ? (f) => f.slotId : g === 'kategorie' ? (f) => f.kategorie || '–' : g === 'bereich' ? (f) => f.slotId.split(':')[0] ?? f.slotId : g === 'quelle' ? (f) => f.quelle : (f) => f.pack;
