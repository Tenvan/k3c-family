import type { Overrides, WorldValueEntry } from '../api';

// Reine Logik der Werte-Overrides im Welten Editor (WZ37.1, B-682, docs/rules/welten.md § 1.1): Override am Pfad lesen,
// setzen und zurücksetzen. Im Entwurf stehen nur Abweichungen vom Basiswert; ein Wert gleich dem Basiswert ist „geerbt“.
// Geprüft wird im Backend wie im Server (planWorldCheck).

export type ValueStatus = 'geerbt' | 'überschrieben';

/** Eine Zeile der Ansicht „Werte“: Feld mit Basiswert, Override des Entwurfs und Status. */
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

type Node = Record<string, unknown>;
const isNode = (v: unknown): v is Node => typeof v === 'object' && v !== null && !Array.isArray(v);

/** JSON mit sortierten Schlüsseln: gleiche Overrides sind gleich, egal in welcher Reihenfolge sie entstanden. */
export function canonical(v: unknown): string {
  if (Array.isArray(v)) return `[${v.map(canonical).join(',')}]`;
  if (isNode(v)) return `{${Object.keys(v).sort().map((k) => `${JSON.stringify(k)}:${canonical(v[k])}`).join(',')}}`;
  return JSON.stringify(v) ?? 'null';
}

const same = (a: unknown, b: unknown) => canonical(a) === canonical(b);

/** Override am Pfad (undefined = geerbt). */
export function overrideAt(ov: Overrides | undefined, file: string, path: readonly string[]): unknown {
  let cur: unknown = ov?.[file];
  for (const k of path) cur = isNode(cur) ? cur[k] : undefined;
  return cur;
}

/** node mit value am Pfad (undefined entfernt das Blatt); leere Objekte entfallen. */
function withLeaf(node: unknown, path: readonly string[], value: unknown): unknown {
  if (path.length === 0) return value;
  const [k, ...rest] = path as [string, ...string[]];
  const obj: Node = isNode(node) ? { ...node } : {};
  const next = withLeaf(obj[k], rest, value);
  if (next === undefined) delete obj[k];
  else obj[k] = next;
  return Object.keys(obj).length > 0 ? obj : undefined;
}

const putLeaf = (ov: Overrides | undefined, file: string, path: readonly string[], value: unknown): Overrides =>
  (withLeaf(ov ?? {}, [file, ...path], value) as Overrides | undefined) ?? {};

/** Override setzen; gleich dem Basiswert wird er entfernt (nur Abweichungen landen in der Datei). */
export const setOverride = (ov: Overrides | undefined, file: string, path: readonly string[], value: unknown, base: unknown): Overrides =>
  putLeaf(ov, file, path, same(value, base) ? undefined : value);

/** Override entfernen: das Feld erbt wieder den Basiswert. */
export const resetOverride = (ov: Overrides | undefined, file: string, path: readonly string[]): Overrides => putLeaf(ov, file, path, undefined);

/** Zeilen der Ansicht: jedes Feld mit dem Override des Entwurfs und seinem Status. */
export const valueRows = (entries: readonly WorldValueEntry[], ov: Overrides | undefined): ValueRow[] =>
  entries.map((e) => {
    const override = overrideAt(ov, e.file, e.path);
    return { file: e.file, path: e.path, key: `${e.file}:${e.path.join('\u0000')}`, label: e.path.join('.'), base: e.base, override, status: override === undefined ? 'geerbt' : 'überschrieben' };
  });

/** Blätter eines Overrides als Pfad → JSON (Listen sind Blätter). */
function leaves(v: unknown, prefix = '', out = new Map<string, string>()): Map<string, string> {
  if (isNode(v)) for (const k of Object.keys(v)) leaves(v[k], `${prefix}\u0000${k}`, out);
  else if (v !== undefined) out.set(prefix, canonical(v));
  return out;
}

/** Zahl der geänderten Felder zwischen zwei Overrides (gesetzt, geändert oder zurückgesetzt). */
export function overrideChanges(a: Overrides | undefined, b: Overrides | undefined): number {
  const x = leaves(a ?? {});
  const y = leaves(b ?? {});
  let n = 0;
  for (const [k, v] of x) if (y.get(k) !== v) n++;
  for (const k of y.keys()) if (!x.has(k)) n++;
  return n;
}

/** Wert als Text im Feld: Text bleibt Text, sonst JSON; ohne Wert „–“. */
export const valueText = (v: unknown): string => (v === undefined || v === null ? '–' : typeof v === 'string' ? v : JSON.stringify(v));

/** Eingabe als Wert: zu einem Text-Basiswert bleibt sie Text, sonst JSON (Zahl, true/false, Liste), nicht lesbar = Text
 *  (die Prüfung meldet dann den falschen Typ mit Pfad). */
export function parseValue(text: string, base: unknown): unknown {
  if (typeof base === 'string') return text;
  try { return JSON.parse(text) as unknown; } catch { return text; }
}

/** Filter der Ansicht: Basis-Datei, Suchtext im Pfad, nur überschriebene. */
export const filterRows = (rows: readonly ValueRow[], file: string, query: string, onlyOwn: boolean): ValueRow[] =>
  rows.filter((r) => r.file === file && r.label.toLowerCase().includes(query.trim().toLowerCase()) && (!onlyOwn || r.status === 'überschrieben'));
