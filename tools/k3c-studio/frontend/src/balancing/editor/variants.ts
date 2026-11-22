import { rangeAround } from '../help';

// Varianten-Modell des Lauf-Editors (B-661): eine Zeile je Wert, fest (`=wert`) oder Bereich (`=min..max`, Abstimmen).
// Reine Logik ohne React; Prüfung gegen die Werte aus data/ steht in variantCheck.ts, die Umwandlung in ein
// BalanceForm in variantForm.ts.

export type VariantKind = 'fest' | 'bereich';

/** Eine Zeile der Varianten-Tabelle; Zahlen bleiben Text, wie getippt (Prüfung und Umwandlung kommen später). */
export interface VariantRow {
  id: number;
  file: string;
  path: string;
  kind: VariantKind;
  /** Neuer Wert einer festen Variante. */
  value: string;
  /** Untergrenze und Obergrenze eines Bereichs. */
  lo: string;
  hi: string;
  /** Zeile, die das Zeilenformat nicht lesen konnte (nur aus „Als Text bearbeiten“); bleibt im Text unverändert. */
  raw?: string;
}

let seq = 0;
/** Nächste Zeilen-ID; nur zum Unterscheiden der Zeilen, nie Teil von Text oder Auftrag. */
export const nextRowId = () => ++seq;

const RANGE = /^(-?\d+(?:\.\d+)?)\.\.(-?\d+(?:\.\d+)?)$/;

/** Eine Zeile im heutigen Format `datei.json:pfad=wert` bzw. `=min..max` → Zeile; Unlesbares bleibt als `raw` erhalten. */
export function parseLine(line: string): VariantRow {
  const text = line.trim();
  const colon = text.indexOf(':');
  const eq = text.indexOf('=');
  const row: VariantRow = { id: nextRowId(), file: '', path: '', kind: 'fest', value: '', lo: '', hi: '' };
  if (colon < 1 || eq < colon) return { ...row, raw: text };
  const head = { ...row, file: text.slice(0, colon), path: text.slice(colon + 1, eq) };
  const value = text.slice(eq + 1);
  const range = RANGE.exec(value);
  return range ? { ...head, kind: 'bereich', lo: range[1], hi: range[2] } : { ...head, value };
}

/** Zeile → Text im heutigen Format. */
export const lineOf = (r: VariantRow): string => r.raw ?? `${r.file}:${r.path}=${r.kind === 'bereich' ? `${r.lo}..${r.hi}` : r.value}`;

/** Text (eine Variante je Zeile, leere Zeilen fallen weg) → Zeilen. */
export const rowsFromText = (text: string): VariantRow[] => text.split('\n').map((l) => l.trim()).filter(Boolean).map(parseLine);

/** Zeilen → Text; `rowsFromText(rowsToText(r))` ergibt dieselben Eingaben. */
export const rowsToText = (rows: readonly VariantRow[]): string => rows.map(lineOf).join('\n');

/** Bereich ±25 % um einen Wert als zwei Texte (wie `rangeAround`); leer, wenn der Wert keine Zahl ist. */
function rangeParts(v: number): [string, string] {
  if (!Number.isFinite(v)) return ['', ''];
  const [lo, hi] = rangeAround(v).split('..');
  return [lo, hi];
}

/** Neue Zeile aus dem Daten-Browser: der Wert beginnt beim Basiswert, der Bereich bei ±25 %. */
export function newRow(file: string, path: string, base: number, kind: VariantKind): VariantRow {
  const [lo, hi] = rangeParts(base);
  return { id: nextRowId(), file, path, kind, value: String(base), lo, hi };
}

/** Art umschalten ohne Neueingabe: fest nimmt den Basiswert, Bereich ±25 % um den Basiswert (sonst um den festen Wert). */
export function switchKind(row: VariantRow, kind: VariantKind, base: number | undefined): VariantRow {
  if (row.kind === kind || row.raw !== undefined) return row;
  if (kind === 'fest') return { ...row, kind, value: row.value || (base === undefined ? '' : String(base)) };
  if (row.lo !== '' && row.hi !== '') return { ...row, kind };
  const [lo, hi] = rangeParts(base ?? Number(row.value));
  return { ...row, kind, lo, hi };
}

export type DeviationTone = 'up' | 'down' | 'same' | 'range' | 'none';

export interface Deviation {
  text: string;
  tone: DeviationTone;
}

const NONE: Deviation = { text: '–', tone: 'none' };

/** „+25 %“, „−50 %“, „±0 %“ (auf ganze Prozent gerundet). */
export function formatPercent(p: number): string {
  const r = Math.round(p);
  return `${r > 0 ? '+' : r < 0 ? '−' : '±'}${Math.abs(r)} %`;
}

const percent = (v: string, base: number): number | undefined => {
  const n = v.trim() === '' ? NaN : Number(v);
  return Number.isFinite(n) ? ((n - base) / base) * 100 : undefined;
};

/** Abweichung vom Basiswert: fest (neu − Basis) / Basis, Bereich als Spanne „−25 % … +25 %“; Basis 0, unbekannt oder keine Zahl → „–“. */
export function deviation(row: VariantRow, base: number | undefined): Deviation {
  if (base === undefined || base === 0 || row.raw !== undefined) return NONE;
  if (row.kind === 'fest') {
    const p = percent(row.value, base);
    if (p === undefined) return NONE;
    return { text: formatPercent(p), tone: Math.round(p) > 0 ? 'up' : Math.round(p) < 0 ? 'down' : 'same' };
  }
  const [lo, hi] = [percent(row.lo, base), percent(row.hi, base)];
  return lo === undefined || hi === undefined ? NONE : { text: `${formatPercent(lo)} … ${formatPercent(hi)}`, tone: 'range' };
}

/** Zahl der festen Varianten und der Bereiche. */
export function countKinds(rows: readonly VariantRow[]): { fest: number; bereich: number } {
  const bereich = rows.filter((r) => r.kind === 'bereich').length;
  return { fest: rows.length - bereich, bereich };
}
