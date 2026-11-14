import type { WorldValueEntry } from '../api';

// Reine Logik der Ansicht „Werte“ im Welten-Editor (B-879, Port aus dem Asset-Studio, world/worldOverrides.ts; B-680,
// docs/rules/welten.md § 1.1): Zeilen, Filter und Eingabe. Gesetzt und geprüft wird im Backend (worldOverrideSet).

export type ValueStatus = 'geerbt' | 'überschrieben';

/** Eine Zeile der Ansicht „Werte“: Feld mit Basiswert, Override der Welt und Status. */
export interface ValueRow {
  file: string;
  path: string[];
  /** eindeutiger Schlüssel der Zeile */
  key: string;
  /** Pfad zum Lesen, z. B. „wall.cost.wood“ */
  label: string;
  base: unknown;
  override: unknown;
  status: ValueStatus;
}

/** Zeilen der Ansicht: jedes Feld mit dem Override der Welt und seinem Status. */
export const valueRows = (entries: readonly WorldValueEntry[]): ValueRow[] =>
  entries.map((e) => ({
    file: e.file, path: e.path, key: `${e.file}:${e.path.join('\u0000')}`, label: e.path.join('.'), base: e.base,
    override: e.override, status: e.override === undefined ? 'geerbt' : 'überschrieben',
  }));

/** Wert als Text im Feld: Text bleibt Text, sonst JSON; ohne Wert „–“. */
export const valueText = (v: unknown): string => (v === undefined || v === null ? '–' : typeof v === 'string' ? v : JSON.stringify(v));

/** Eingabe als Wert: zu einem Text-Basiswert bleibt sie Text, sonst JSON (Zahl, true/false, Liste), nicht lesbar = Text
 *  (die Prüfung meldet dann den falschen Typ mit Pfad). */
export function parseValue(text: string, base: unknown): unknown {
  if (typeof base === 'string') return text;
  try {
    return JSON.parse(text) as unknown;
  } catch {
    return text;
  }
}

/** Filter der Ansicht: Basis-Datei, Suchtext im Pfad, nur überschriebene. */
export const filterRows = (rows: readonly ValueRow[], file: string, query: string, onlyOwn: boolean): ValueRow[] =>
  rows.filter((r) => r.file === file && r.label.toLowerCase().includes(query.trim().toLowerCase()) && (!onlyOwn || r.status === 'überschrieben'));

/** Basis-Dateien in der Reihenfolge der Felder, mit der Zahl der überschriebenen je Datei. */
export function filesOf(rows: readonly ValueRow[]): { file: string; own: number }[] {
  const out = new Map<string, number>();
  for (const r of rows) out.set(r.file, (out.get(r.file) ?? 0) + (r.status === 'überschrieben' ? 1 : 0));
  return [...out].map(([file, own]) => ({ file, own }));
}
