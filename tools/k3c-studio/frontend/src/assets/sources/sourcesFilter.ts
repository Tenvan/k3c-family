// Katalog und Recherche-Treffer der Seite Quellen als eine filterbare Liste (B-729). Reine Funktionen.
import type { ListFilterConfig } from '@webkit/lib/listFilter';
import type { KatalogDatei, Quellen, Treffer } from '../api';

/** Eine Datei des Katalogs oder ein Recherche-Treffer, mit den Werten für Filter und Prompt. */
export interface QuellenEintrag {
  key: string;
  herkunft: 'katalog' | 'treffer';
  /** Datei unter public/ bzw. Datei oder Link des Treffers. */
  ort: string;
  /** Pack der Datei bzw. Quelle des Treffers. */
  pack: string;
  /** Lizenz wie angegeben, „unbekannt“ ohne Angabe. */
  lizenzText: string;
  /** Lizenz-Familie für die Chips: „CC BY-NC 4.0“ → „CC-BY-NC“, ohne Angabe „unbekannt“. */
  lizenz: string;
  art?: string;
  slot?: string;
  datei?: KatalogDatei;
  treffer?: Treffer;
}

export const UNBEKANNT = 'unbekannt';

/** Lizenz-Familie ohne Version, Leerzeichen als Bindestrich; ohne Angabe „unbekannt“ (B-729, eigener Chip). */
export function lizenzFamilie(lizenz: string | undefined): string {
  const t = (lizenz ?? '').trim();
  return t ? t.replace(/\s+v?\d+(\.\d+)*$/i, '').replace(/\s+/g, '-') : UNBEKANNT;
}

export function eintraege(q: Quellen): QuellenEintrag[] {
  const katalog = q.katalog.map((d): QuellenEintrag => ({
    key: `datei:${d.datei}`, herkunft: 'katalog', ort: d.datei, pack: d.pack, lizenzText: d.lizenz.trim() || UNBEKANNT,
    lizenz: lizenzFamilie(d.lizenz), art: d.art, datei: d,
  }));
  const treffer = Object.entries(q.treffer).flatMap(([slot, hits]) => hits.map((h): QuellenEintrag => ({
    key: `treffer:${slot}|${h.id}`, herkunft: 'treffer', ort: h.datei || h.link, pack: h.quelle, lizenzText: h.lizenz?.trim() || UNBEKANNT,
    lizenz: lizenzFamilie(h.lizenz), slot, treffer: h,
  })));
  return [...treffer, ...katalog];
}

const HERKUNFT: Record<QuellenEintrag['herkunft'], string> = { treffer: 'Recherche-Treffer', katalog: 'Katalog' };

/** Filter nach Herkunft, Lizenz, Pack und Art (B-729/AC-01); Treffer haben keine Art (Chip „–“). */
export const QUELLEN_FILTER: ListFilterConfig<QuellenEintrag> = {
  searchText: (e) => [e.ort, e.pack, e.lizenzText, e.slot, e.treffer?.titel, e.datei?.titel].filter(Boolean).join(' '),
  dimensions: [
    { key: 'herkunft', label: 'Herkunft', values: (e) => e.herkunft, valueLabel: (v) => HERKUNFT[v as QuellenEintrag['herkunft']] ?? v, order: ['treffer', 'katalog'] },
    { key: 'lizenz', label: 'Lizenz', values: (e) => e.lizenz },
    { key: 'pack', label: 'Pack', values: (e) => e.pack },
    { key: 'art', label: 'Art', values: (e) => e.art },
  ],
  sorts: [{ key: 'ort', label: 'Datei', compare: (a, b) => a.ort.localeCompare(b.ort) }],
};
