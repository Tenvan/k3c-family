import type { Tone } from '@webkit/atoms';
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

export type PreviewKind = 'bild' | 'archiv' | 'ton';

const IMAGE_EXT = ['png', 'jpg', 'jpeg', 'gif', 'webp'];
const ARCHIVE_EXT = ['zip', '7z', 'rar', 'tar', 'gz', 'tgz'];

/** Endung einer Datei in Kleinschreibung, ohne Query und Anker ("" ohne Endung). */
const extOf = (file: string): string => /\.([a-z0-9]+)$/i.exec(file.split(/[?#]/)[0])?.[1].toLowerCase() ?? '';

/** Art der Vorschau: Ton bleibt Ton; bei Grafik zeigen nur Bild-Endungen ein Bild, Archive (.zip, .7z …) und Unbekanntes nicht. */
export function previewKind(s: Pick<SourceProposal, 'art' | 'vorschau'>): PreviewKind {
  if (s.art === 'sound') return 'ton';
  return IMAGE_EXT.includes(extOf(s.vorschau ?? '')) ? 'bild' : 'archiv';
}

/** Endung eines Archivs für den Text „Archiv (zip)“ ("Archiv" ohne bekannte Endung). */
export function archiveLabel(file: string): string {
  const ext = extOf(file);
  return ARCHIVE_EXT.includes(ext) ? `Archiv (${ext})` : 'Archiv';
}
