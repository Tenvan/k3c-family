import type { WorldValueEntry } from '../api';
import { valueText } from '@webkit/lib/valueEdit';

export { valueText };

// Reine Logik der Ansicht „Werte“ im Welten-Editor (B-879, B-1173, docs/rules/welten.md § 1.1): Zeilen, Gruppen und
// Filter. Eingabe und Prüfung des einzelnen Werts stehen in @webkit/lib/valueEdit; gesetzt und endgültig geprüft wird im
// Backend (worldOverrideSet).

export type ValueStatus = 'geerbt' | 'überschrieben';

/** Eine Zeile der Ansicht „Werte“: Feld mit Basiswert, Override der Welt und Status. */
export interface ValueRow {
  file: string;
  path: string[];
  /** eindeutiger Schlüssel der Zeile */
  key: string;
  /** Pfad zum Lesen, z. B. „wall.cost.wood“ */
  label: string;
  /** Thema: erster Pfadteil („wall“), Felder ohne Verschachtelung stehen unter „allgemein“ */
  group: string;
  base: unknown;
  override: unknown;
  status: ValueStatus;
}

export const GENERAL = 'allgemein';

/** Zeilen der Ansicht: jedes Feld mit dem Override der Welt und seinem Status. */
export const valueRows = (entries: readonly WorldValueEntry[]): ValueRow[] =>
  entries.map((e) => ({
    file: e.file, path: e.path, key: `${e.file}:${e.path.join('\u0000')}`, label: e.path.join('.'),
    group: e.path.length > 1 ? e.path[0]! : GENERAL, base: e.base,
    override: e.override, status: e.override === undefined ? 'geerbt' : 'überschrieben',
  }));

/** Welche Zeilen die Tabelle zeigt: alle, nur gegen die Basis überschriebene oder die in dieser Sitzung geänderten. */
export type RowMode = 'alle' | 'ueberschrieben' | 'sitzung';

export interface RowFilter {
  /** Basis-Datei; '' = alle Dateien */
  file: string;
  query: string;
  mode: RowMode;
  /** Schlüssel der Zeilen, die in dieser Sitzung geschrieben wurden */
  touched: ReadonlySet<string>;
}

/** Zeilen nach Datei, Suchtext (alle Wörter, in Pfad oder Basiswert) und Modus. */
export function filterRows(rows: readonly ValueRow[], f: RowFilter): ValueRow[] {
  const words = f.query.toLowerCase().split(/\s+/).filter(Boolean);
  return rows.filter((r) => {
    if (f.file && r.file !== f.file) return false;
    if (f.mode === 'ueberschrieben' && r.status !== 'überschrieben') return false;
    if (f.mode === 'sitzung' && !f.touched.has(r.key)) return false;
    const hay = `${r.file} ${r.label} ${valueText(r.base)}`.toLowerCase();
    return words.every((w) => hay.includes(w));
  });
}

export interface RowGroup {
  /** Schlüssel für Ein- und Ausklappen: Datei und Thema */
  key: string;
  file: string;
  group: string;
  rows: ValueRow[];
  /** Zahl der überschriebenen Zeilen in der Gruppe */
  own: number;
}

/** Gruppen nach Datei und Thema in der Reihenfolge der Felder. */
export function groupRows(rows: readonly ValueRow[]): RowGroup[] {
  const map = new Map<string, RowGroup>();
  for (const r of rows) {
    const key = `${r.file}\u0000${r.group}`;
    const g = map.get(key) ?? map.set(key, { key, file: r.file, group: r.group, rows: [], own: 0 }).get(key)!;
    g.rows.push(r);
    if (r.status === 'überschrieben') g.own++;
  }
  return [...map.values()];
}

/** Basis-Dateien in der Reihenfolge der Felder, mit der Zahl der überschriebenen je Datei. */
export function filesOf(rows: readonly ValueRow[]): { file: string; own: number }[] {
  const out = new Map<string, number>();
  for (const r of rows) out.set(r.file, (out.get(r.file) ?? 0) + (r.status === 'überschrieben' ? 1 : 0));
  return [...out].map(([file, own]) => ({ file, own }));
}
