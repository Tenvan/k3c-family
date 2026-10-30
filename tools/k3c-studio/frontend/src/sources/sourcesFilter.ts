import type { SourceProposal } from '../api';
import { usage } from './sourcesLogic';

// Filter der Seite Quellen (B-673): ein Reiter je Quelle, Suche, Art und Status als Segmente, Slot, Lizenz und
// „gesperrte zeigen“ unter „Filter“. Reine Funktionen; die Oberfläche steht in SourcesFilterBar.tsx.

export type ArtFilter = 'alle' | 'sound' | 'grafik';
export type StatusFilter = 'offen' | 'geladen' | 'verwendet' | 'alle';

export interface SourceFilter {
  query: string;
  /** Name der Quelle, "" = alle */
  tab: string;
  art: ArtFilter;
  status: StatusFilter;
  /** "" = alle Slots */
  slot: string;
  /** "" = alle Lizenzen */
  lizenz: string;
  /** gesperrte (Status gesperrt oder Lizenz nicht erlaubt) mit anzeigen */
  gesperrte: boolean;
}

export const DEFAULT_FILTER: SourceFilter = { query: '', tab: '', art: 'alle', status: 'alle', slot: '', lizenz: '', gesperrte: false };

export const ART_OPTIONS: readonly { value: ArtFilter; label: string }[] = [{ value: 'alle', label: 'Alle' }, { value: 'sound', label: 'Ton' }, { value: 'grafik', label: 'Grafik' }];
export const STATUS_OPTIONS: readonly { value: StatusFilter; label: string }[] = [
  { value: 'offen', label: 'Offen' }, { value: 'geladen', label: 'Geladen' }, { value: 'verwendet', label: 'Verwendet' }, { value: 'alle', label: 'Alle' },
];

/** Prüfung für den gemerkten Filter (ein veralteter Wert gilt als nicht gemerkt). */
export function isSourceFilter(v: unknown): v is SourceFilter {
  const f = v as Partial<SourceFilter> | null;
  return !!f && typeof f.tab === 'string' && typeof f.slot === 'string' && typeof f.lizenz === 'string' && typeof f.gesperrte === 'boolean'
    && ART_OPTIONS.some((o) => o.value === f.art) && STATUS_OPTIONS.some((o) => o.value === f.status);
}

const blocked = (s: SourceProposal): boolean => s.status === 'gesperrt' || !s.lizenzErlaubt;

function statusMatches(s: SourceProposal, status: StatusFilter): boolean {
  if (status === 'alle') return true;
  if (status === 'verwendet') return s.verwendet;
  if (status === 'geladen') return s.status === 'geladen' && !s.verwendet;
  return s.status === 'offen';
}

const searchText = (s: SourceProposal): string => `${s.titel} ${s.slot} ${s.quelle} ${s.lizenz} ${s.urheber ?? ''} ${s.hinweis ?? ''} ${usage(s)}`.toLowerCase();

/** Trifft der Eintrag den Filter? ignoreTab = ohne den Reiter (für die Zähler der Reiter). */
export function matchesFilter(s: SourceProposal, f: SourceFilter, ignoreTab = false): boolean {
  if (!ignoreTab && f.tab && s.quelle !== f.tab) return false;
  if (f.art !== 'alle' && s.art !== f.art) return false;
  if (f.slot && s.slot !== f.slot) return false;
  if (f.lizenz && s.lizenz !== f.lizenz) return false;
  if (!f.gesperrte && blocked(s)) return false;
  if (!statusMatches(s, f.status)) return false;
  return f.query.toLowerCase().split(/\s+/).filter(Boolean).every((w) => searchText(s).includes(w));
}

/** Die Einträge, die der Filter durchlässt, in der Reihenfolge der Liste. */
export const applyFilter = (items: readonly SourceProposal[], f: SourceFilter): SourceProposal[] => items.filter((s) => matchesFilter(s, f));

/** Zahl der Einschränkungen unter „Filter“ (Slot, Lizenz, gesperrte). */
export const extraFilters = (f: SourceFilter): number => (f.slot ? 1 : 0) + (f.lizenz ? 1 : 0) + (f.gesperrte ? 1 : 0);

export interface SlotGroup {
  slot: string;
  items: SourceProposal[];
  /** Lücke: noch keine Datei des Slots ist verwendet */
  gap: boolean;
}

/** Gruppiert nach Slot; Lücken zuerst, dann belegte, je alphabetisch; Einträge nach Priorität. `all` bestimmt, ob ein Slot belegt ist. */
export function groupBySlot(items: readonly SourceProposal[], all: readonly SourceProposal[]): SlotGroup[] {
  const used = new Set(all.filter((s) => s.verwendet).map((s) => s.slot));
  const by = new Map<string, SourceProposal[]>();
  for (const s of items) by.set(s.slot, [...(by.get(s.slot) ?? []), s]);
  return [...by.entries()]
    .map(([slot, list]) => ({ slot, gap: !used.has(slot), items: [...list].sort((a, b) => a.prioritaet - b.prioritaet || a.titel.localeCompare(b.titel)) }))
    .sort((a, b) => Number(b.gap) - Number(a.gap) || a.slot.localeCompare(b.slot));
}

/** Auftrag an den Chat „Claude suchen lassen“: Quelle (leer = alle erlaubten), Slot (leer = alle Lücken) und Art. */
export function searchPrompt(source: string, slot: string, art: ArtFilter): string {
  const what = art === 'sound' ? 'Töne' : art === 'grafik' ? 'Grafiken' : 'Töne und Grafiken';
  return `Suche freie ${what} von ${source || 'den erlaubten Quellen'} für ${slot ? `den Slot ${slot}` : 'alle Lücken'}. `
    + 'Nenne je Ziel bis zu drei Kandidaten mit direkter Datei-URL, Seite, Urheber und Lizenz (CC0, CC-BY, OGA-BY oder CC-BY-SA).';
}
