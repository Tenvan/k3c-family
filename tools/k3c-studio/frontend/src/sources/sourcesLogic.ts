import type { Tone } from '@webkit/atoms';
import type { ListFilterConfig } from '@webkit/lib/listFilter';
import type { SourceProgress, SourceProposal, SourceStatus } from '../api';

/** Größe in Bytes („1,4 MB“, „86 kB“). */
export function formatBytes(bytes: number): string {
  if (bytes >= 1_000_000) return `${(bytes / 1_000_000).toFixed(1).replace('.', ',')} MB`;
  return `${Math.max(1, Math.round(bytes / 1000))} kB`;
}

export const STATUS_LABEL: Record<SourceStatus, string> = { offen: 'offen', geladen: 'geladen', verworfen: 'verworfen', gesperrt: 'gesperrt' };
export const STATUS_TONE: Record<SourceStatus, Tone> = { offen: 'info', geladen: 'ok', verworfen: 'neutral', gesperrt: 'error' };

/** Ladbar: offen und Lizenz erlaubt (auch nach einem Fehler, Wiederholen ist möglich). */
export const downloadable = (s: SourceProposal): boolean => s.status === 'offen' && s.lizenzErlaubt;

/** Verwendet oder nicht (Zustand der Datei im Vorrat), nur für geladene Einträge. */
export const usage = (s: SourceProposal): string => (s.status !== 'geladen' ? '–' : s.verwendet ? 'verwendet' : 'unverwendet');

const PRIO = ['', 'hoch', 'mittel', 'niedrig'];

/** Filter der Vorschlagsliste nach Quelle, Art, Slot, Lizenz, Status (B-601, B-599). */
export const sourceFilter: ListFilterConfig<SourceProposal> = {
  searchText: (s) => `${s.titel} ${s.slot} ${s.quelle} ${s.lizenz} ${s.urheber ?? ''} ${s.hinweis ?? ''}`,
  dimensions: [
    { key: 'quelle', label: 'Quelle', values: (s) => s.quelle },
    { key: 'art', label: 'Art', values: (s) => s.art, valueLabel: (v) => (v === 'sound' ? 'Ton' : 'Grafik'), order: ['sound', 'grafik'] },
    { key: 'slot', label: 'Slot', values: (s) => s.slot },
    { key: 'lizenz', label: 'Lizenz', values: (s) => s.lizenz },
    { key: 'status', label: 'Status', values: (s) => s.status, order: ['offen', 'geladen', 'verworfen', 'gesperrt'] },
    { key: 'verwendet', label: 'Verwendung', values: (s) => (s.status === 'geladen' ? usage(s) : undefined) },
  ],
  sorts: [
    { key: 'prio', label: 'Priorität', compare: (a, b) => a.prioritaet - b.prioritaet || a.titel.localeCompare(b.titel) },
    { key: 'titel', label: 'Titel', compare: (a, b) => a.titel.localeCompare(b.titel) },
    { key: 'groesse', label: 'Größe', compare: (a, b) => b.groesse - a.groesse },
  ],
};

export const prioLabel = (n: number): string => PRIO[n] ?? String(n);

export interface Selection {
  count: number;
  bytes: number;
  /** Quellen mit Anzahl und Größe (Bestätigung nennt sie) */
  quellen: { name: string; count: number; bytes: number }[];
}

/** Zusammenfassung der gewählten, ladbaren Einträge: Anzahl, Größensumme und Quellen. */
export function summarize(items: readonly SourceProposal[], ids: ReadonlySet<string>): Selection {
  const chosen = items.filter((s) => ids.has(s.id) && downloadable(s));
  const by = new Map<string, { name: string; count: number; bytes: number }>();
  for (const s of chosen) {
    const q = by.get(s.quelle) ?? { name: s.quelle, count: 0, bytes: 0 };
    by.set(s.quelle, { name: q.name, count: q.count + 1, bytes: q.bytes + s.groesse });
  }
  return { count: chosen.length, bytes: chosen.reduce((a, s) => a + s.groesse, 0), quellen: [...by.values()].sort((a, b) => a.name.localeCompare(b.name)) };
}

/** Fortschritt in Prozent (0–100). */
export const percent = (p: SourceProgress): number => (p.gesamt <= 0 ? 0 : Math.min(100, Math.round((p.geladen / p.gesamt) * 100)));

/** Läuft noch ein Download (wartet oder lädt)? */
export const running = (jobs: readonly SourceProgress[]): boolean => jobs.some((j) => j.status === 'wartet' || j.status === 'laedt');
