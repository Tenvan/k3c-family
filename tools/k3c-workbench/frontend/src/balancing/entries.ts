import type { ListFilterConfig } from '@webkit/lib/listFilter';
import type { BalanceBaseline, BalanceReportInfo, BalanceRun, BalanceSpec, ReportRating } from '../api';
import { isActive } from './spec';

// Einträge der Liste (B-660): laufende und wartende Läufe plus fertige Berichte in einer Liste. Ein Lauf, dessen Bericht
// schon in der Berichtsliste steht, erscheint nur als Bericht.

export type ListEntry =
  | { type: 'run'; id: string; run: BalanceRun }
  | { type: 'report'; id: string; report: BalanceReportInfo; baseline?: string };

export type EntryStatus = 'läuft' | 'wartet' | 'Pass' | 'Fail' | 'abgebrochen' | 'fehler';
export type EntryKind = 'Balance' | 'Abstimmung' | 'Baseline';
export type DayGroup = 'Läuft' | 'Heute' | 'Gestern' | 'Älter';

/** Ab so vielen Einträgen ist die Gruppe „Älter“ eingeklappt. */
export const OLDER_COLLAPSE = 30;
const RUN_PREFIX = 'run:';

export const runEntryId = (run: BalanceRun) => RUN_PREFIX + run.id;

const time = (iso: string | undefined) => {
  const t = Date.parse(iso ?? '');
  return Number.isNaN(t) ? 0 : t;
};

export const entryTime = (e: ListEntry): number => (e.type === 'run' ? time(e.run.ended || e.run.started) : time(e.report.modified));
export const entrySpec = (e: ListEntry): BalanceSpec | undefined => (e.type === 'run' ? e.run.spec : e.report.spec);
export const entryActive = (e: ListEntry) => e.type === 'run' && isActive(e.run);

/** Der Lauf steht schon als Bericht in der Liste: Pfad des Laufs, sonst fertiger Lauf mit gleicher Lauf-ID und neuerem Bericht. */
function merged(run: BalanceRun, reports: readonly BalanceReportInfo[]): boolean {
  if (run.report && reports.some((r) => r.path === run.report)) return true;
  return !isActive(run) && reports.some((r) => r.runId === run.id && time(r.modified) >= time(run.started));
}

/** Aktive Läufe zuerst (laufend vor wartend), dann alles nach Zeit, neueste zuerst. */
export function compareEntries(a: ListEntry, b: ListEntry): number {
  const rank = (e: ListEntry) => (e.type === 'run' && e.run.state === 'läuft' ? 0 : entryActive(e) ? 1 : 2);
  return rank(a) - rank(b) || entryTime(b) - entryTime(a);
}

export function buildEntries(runs: readonly BalanceRun[], reports: readonly BalanceReportInfo[], baselines: readonly BalanceBaseline[]): ListEntry[] {
  const out: ListEntry[] = runs.filter((r) => !merged(r, reports)).map((run) => ({ type: 'run', id: runEntryId(run), run }));
  for (const report of reports) {
    const baseline = report.run ? baselines.find((b) => b.run === report.run)?.name : undefined;
    out.push({ type: 'report', id: report.path, report, baseline });
  }
  return out.sort(compareEntries);
}

/** Auswahl nach einem Neuladen: ein Lauf, der inzwischen einen Bericht hat, wird durch ihn ersetzt; Unbekanntes wird leer. */
export function resolveSelection(selected: string, runs: readonly BalanceRun[], entries: readonly ListEntry[]): string {
  if (!selected || entries.some((e) => e.id === selected)) return selected;
  const run = selected.startsWith(RUN_PREFIX) ? runs.find((r) => runEntryId(r) === selected) : undefined;
  return run?.report && entries.some((e) => e.id === run.report) ? run.report : '';
}

/** Nächste Auswahl nach dem Löschen: der Eintrag dahinter, sonst der davor, sonst keiner. */
export function selectionAfterRemoval(entries: readonly ListEntry[], removed: readonly string[], selected: string): string {
  if (!removed.includes(selected)) return selected;
  const i = entries.findIndex((e) => e.id === selected);
  const rest = (list: readonly ListEntry[]) => list.find((e) => !removed.includes(e.id))?.id;
  return rest(entries.slice(i + 1)) ?? rest(entries.slice(0, i).reverse()) ?? '';
}

export function entryStatus(e: ListEntry): EntryStatus {
  if (e.type === 'run') return e.run.state === 'fertig' ? 'Pass' : e.run.state;
  return e.report.pass ? 'Pass' : 'Fail';
}

export function entryKind(e: ListEntry): EntryKind {
  if (e.type === 'run') return e.run.spec?.tune ? 'Abstimmung' : 'Balance';
  if (e.report.kind === 'tuning') return 'Abstimmung';
  return e.baseline ? 'Baseline' : 'Balance';
}

export const entryRating = (e: ListEntry): ReportRating => (e.type === 'report' ? e.report.rating ?? '' : '');

const baseName = (path: string) => path.split('/').pop() || path;
const day = (ms: number) => new Date(ms).toLocaleDateString('de-DE', { day: '2-digit', month: '2-digit' });
const clock = (ms: number) => new Date(ms).toLocaleTimeString('de-DE', { hour: '2-digit', minute: '2-digit' });

/** „13:34“ am selben Tag, sonst „28.10. 22:57“. */
export function formatWhen(ms: number, now: Date): string {
  if (!ms) return '–';
  return new Date(ms).toDateString() === now.toDateString() ? clock(ms) : `${day(ms)} ${clock(ms)}`;
}

const DEPTH_PREFIX = 'Tiefe ';
const where = (s: BalanceSpec) => (s.world ? `Welt ${s.world}` : s.depth !== undefined ? `${DEPTH_PREFIX}${s.depth}` : '');

/** Name: Label, sonst Bot · Welt · Datum aus dem Auftrag, sonst der bisherige Name. */
export function entryLabel(e: ListEntry, now = new Date()): string {
  const spec = entrySpec(e);
  const label = e.type === 'run' ? e.run.label : e.report.label;
  if (label) return label;
  const legacy = e.type === 'run' ? e.run.id : e.report.run || baseName(e.report.path);
  if (!spec || e.type === 'run') return legacy;
  const parts = [spec.bot, where(spec), formatWhen(entryTime(e), now)].filter(Boolean);
  return parts.length > 1 ? parts.join(' · ') : legacy;
}

/** Bot, Spieler, Welt oder Tiefe, Tage, Seeds aus dem Auftrag. */
export function specMeta(spec: BalanceSpec | undefined, seedsFallback = 0): string {
  const s = spec ?? {};
  const seeds = s.seedList?.length ?? s.seeds ?? seedsFallback;
  const parts = [s.bot, s.players ? `${s.players} Sp.` : '', where(s), s.days ? `${s.days} T.` : '', seeds ? `${seeds} ${seeds === 1 ? 'Seed' : 'Seeds'}` : ''];
  return parts.filter(Boolean).join(' · ');
}

export function entryMeta(e: ListEntry): string {
  return specMeta(entrySpec(e), e.type === 'report' ? e.report.seeds : 0);
}

/** Zweite Zeile: Uhrzeit · Ziele · Varianten · Art. */
export function entryLine2(e: ListEntry, now: Date): string {
  if (e.type === 'run') return e.run.state === 'wartet' ? 'wartet in der Warteschlange' : `gestartet ${formatWhen(time(e.run.started), now)}`;
  const r = e.report;
  const kind = entryKind(e);
  return [
    formatWhen(entryTime(e), now),
    r.targetsTotal > 0 ? `${r.targetsPass}/${r.targetsTotal} Ziele` : '',
    r.varianten?.length ? `+${r.varianten.length} Var.` : '',
    kind === 'Baseline' ? `Baseline ${e.type === 'report' ? e.baseline : ''}` : kind === 'Abstimmung' ? kind : '',
  ].filter(Boolean).join(' · ');
}

export function dayGroup(e: ListEntry, now: Date): DayGroup {
  if (entryActive(e)) return 'Läuft';
  const start = (d: Date) => new Date(d.getFullYear(), d.getMonth(), d.getDate()).getTime();
  const days = Math.round((start(now) - start(new Date(entryTime(e)))) / 86_400_000);
  return days <= 0 ? 'Heute' : days === 1 ? 'Gestern' : 'Älter';
}

export function groupByDay(items: readonly ListEntry[], now: Date): { name: DayGroup; items: ListEntry[] }[] {
  const names: DayGroup[] = ['Läuft', 'Heute', 'Gestern', 'Älter'];
  return names.map((name) => ({ name, items: items.filter((e) => dayGroup(e, now) === name) })).filter((g) => g.items.length > 0);
}

const searchText = (e: ListEntry) => {
  const s = entrySpec(e);
  const path = e.type === 'report' ? e.report.path : e.id;
  return [entryLabel(e), s?.bot, s?.world, ...(s?.vary ?? []), ...(e.type === 'report' ? e.report.varianten ?? [] : []), path].filter(Boolean).join(' ');
};

const STATUS_ORDER = ['läuft', 'Pass', 'Fail', 'abgebrochen', 'fehler'] as const;
const STATUS_LABEL: Record<string, string> = { läuft: 'Läuft', Pass: 'Pass', Fail: 'Fail', abgebrochen: 'Abgebrochen', fehler: 'Fehler' };

/** Chip-Wert des Status: wartende Läufe zählen zu „Läuft“. */
const statusValue = (e: ListEntry) => (entryStatus(e) === 'wartet' ? 'läuft' : entryStatus(e));

/** Suche und Chips der Liste: Status (Läuft, Pass, Fail …) und Art (Balance, Abstimmung, Baseline). */
export const LIST_CONFIG: ListFilterConfig<ListEntry> = {
  searchText,
  dimensions: [
    { key: 'status', label: 'Status', values: statusValue, order: STATUS_ORDER, valueLabel: (v) => STATUS_LABEL[v] ?? v },
    { key: 'art', label: 'Art', values: entryKind, order: ['Balance', 'Abstimmung', 'Baseline'] },
  ],
  sorts: [{ key: 'neu', label: 'Neueste zuerst', compare: compareEntries }],
};
