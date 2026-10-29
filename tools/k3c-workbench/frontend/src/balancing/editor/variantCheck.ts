import type { BalanceDataFileValue } from '../../api';
import type { VariantRow } from './variants';

// Prüfung der Varianten-Zeilen (B-661): Form wie in Go (simrun/spec.go: varyArg, tuneParam), Datei und Pfad gegen die
// Werte aus data/ (balanceDataValuesAll), Zahlen, Bereichsgrenzen. Fehler sperren das Starten, Hinweise nicht.

export const MAX_RANGES = 2;
export const MAX_FEST = 20;

const FILE = /^[A-Za-z0-9_-]+\.json$/;
const PATH = /^[A-Za-z0-9_.[\]-]+$/;
const NUM = /^-?\d+(\.\d+)?$/;
const FEST = /^-?\d+(\.\d+)?([eE][+-]?\d+)?$/;

/** Werte aus data/ zum Nachschlagen: Basiswert je `datei:pfad`, Pfade je Datei. */
export interface ValueIndex {
  base: Map<string, number>;
  paths: Map<string, string[]>;
}

export const keyOf = (file: string, path: string) => `${file}:${path}`;

export function buildIndex(list: readonly BalanceDataFileValue[]): ValueIndex {
  const base = new Map<string, number>();
  const paths = new Map<string, string[]>();
  for (const v of list) {
    base.set(keyOf(v.file, v.path), v.value);
    const own = paths.get(v.file);
    if (own) own.push(v.path);
    else paths.set(v.file, [v.path]);
  }
  return { base, paths };
}

/** Vorschlag für eine Zeile mit falscher Datei oder falschem Pfad. */
export interface Fix {
  file: string;
  path: string;
  base: number;
}

export interface RowCheck {
  error?: string;
  fix?: Fix;
  /** Hinweis, der das Starten nicht sperrt. */
  note?: string;
  /** Basiswert aus data/; fehlt, wenn die Werte nicht geladen sind oder die Zeile falsch ist. */
  base?: number;
}

/** Bearbeitungsabstand zweier Texte (Levenshtein). */
export function distance(a: string, b: string): number {
  let prev = Array.from({ length: b.length + 1 }, (_, j) => j);
  for (let i = 1; i <= a.length; i++) {
    const cur = [i];
    for (let j = 1; j <= b.length; j++) {
      cur[j] = Math.min(prev[j] + 1, cur[j - 1] + 1, prev[j - 1] + (a[i - 1] === b[j - 1] ? 0 : 1));
    }
    prev = cur;
  }
  return prev[b.length];
}

/** Ähnlichster Eintrag der Liste, wenn er nah genug liegt (höchstens ein Drittel der Länge, mindestens 2 Änderungen). */
export function closest(word: string, list: readonly string[]): string | undefined {
  const limit = Math.max(2, Math.floor(word.length / 3));
  let best: string | undefined;
  let bestD = limit + 1;
  for (const c of list) {
    if (Math.abs(c.length - word.length) > limit) continue;
    const d = distance(word, c);
    if (d < bestD) [best, bestD] = [c, d];
  }
  return best;
}

const FORM_ERROR = 'Form datei.json:pfad.zum.wert, z. B. economy.json:purse.startGold';

function unknownFile(row: VariantRow, index: ValueIndex): RowCheck {
  const files = [...index.paths.keys()];
  const near = closest(row.file, files);
  const base = near === undefined ? undefined : index.base.get(keyOf(near, row.path));
  if (near !== undefined && base !== undefined) return { error: `Datei unbekannt. Meinten Sie ${near}?`, fix: { file: near, path: row.path, base } };
  return { error: `Datei unbekannt. Variierbar: ${files.join(', ')}.` };
}

/** Datei und Pfad der Zeile: Form, dann Nachschlagen in data/ mit Vorschlag; ohne Werte bleibt nur die Form. */
function locate(row: VariantRow, index: ValueIndex | null): RowCheck {
  if (row.raw !== undefined) return { error: `„${row.raw}“ nicht lesbar: Form datei.json:pfad=wert oder datei.json:pfad=min..max` };
  if (!FILE.test(row.file) || !PATH.test(row.path) || row.path.includes('..')) return { error: FORM_ERROR };
  if (!index) return {};
  const base = index.base.get(keyOf(row.file, row.path));
  if (base !== undefined) return { base };
  const list = index.paths.get(row.file);
  if (!list) return unknownFile(row, index);
  const near = closest(row.path, list);
  const nearBase = near === undefined ? undefined : index.base.get(keyOf(row.file, near));
  if (near === undefined || nearBase === undefined) return { error: 'Pfad unbekannt. Im Daten-Browser suchen.' };
  return { error: `Pfad unbekannt. Meinten Sie ${near} (${nearBase})?`, fix: { file: row.file, path: near, base: nearBase } };
}

function numberError(v: string, re: RegExp, what: string): string | undefined {
  if (v === '') return `${what} fehlt.`;
  if (/^-?\d+,\d+$/.test(v)) return `„${v}“: Dezimalpunkt statt Komma verwenden (${v.replace(',', '.')}).`;
  return re.test(v) && v.length <= 64 ? undefined : `„${v}“ ist kein Zahlenwert.`;
}

function valueError(row: VariantRow): string | undefined {
  if (row.kind === 'fest') return numberError(row.value, FEST, 'Neuer Wert');
  const err = numberError(row.lo, NUM, 'Von') ?? numberError(row.hi, NUM, 'Bis');
  if (err) return err;
  return Number(row.lo) >= Number(row.hi) ? 'Bereich: „von“ muss kleiner sein als „bis“.' : undefined;
}

function intNote(row: VariantRow, base: number | undefined): string | undefined {
  if (row.kind !== 'bereich' || base === undefined || !Number.isInteger(base)) return undefined;
  const frac = [row.lo, row.hi].some((v) => NUM.test(v) && !Number.isInteger(Number(v)));
  return frac ? 'Die Datei hat eine ganze Zahl: Die Abstimmung behält ganze Zahlen.' : undefined;
}

/** Prüft alle Zeilen; Ergebnis in derselben Reihenfolge. Ohne `index` (Werte nicht ladbar) nur Form und Zahlen. */
export function checkRows(rows: readonly VariantRow[], index: ValueIndex | null): RowCheck[] {
  const seen = new Set<string>();
  let ranges = 0;
  let fests = 0;
  return rows.map((row) => {
    const where = locate(row, index);
    const own = row.kind === 'bereich' ? ++ranges : ++fests;
    const key = keyOf(row.file, row.path);
    const dup = row.raw === undefined && seen.has(key);
    seen.add(key);
    const limit = row.kind === 'bereich' && own > MAX_RANGES ? 'Höchstens zwei Bereiche (Grenze der Abstimmung).' : row.kind === 'fest' && own > MAX_FEST ? `Höchstens ${MAX_FEST} feste Varianten.` : undefined;
    const error = where.error ?? (dup ? 'Dieser Wert steht schon in einer Zeile darüber.' : undefined) ?? (row.raw === undefined ? valueError(row) : undefined) ?? limit;
    return { ...where, error, note: error ? undefined : intNote(row, where.base) };
  });
}
