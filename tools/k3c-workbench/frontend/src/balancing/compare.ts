import type { BalanceEntry, BalanceSummary } from '../api';
import { corridorText } from './export';

/** Wert einer Spalte (Bericht) in einer Zeile; `delta` ist der Abstand zu A (Spalte 0). */
export interface CompareCell {
  value: number | null;
  delta: number | null;
  status?: string;
  /** Pass wechselt gegenüber A. */
  tipped: boolean;
}

/** Zeile des Vergleichs: ein Ziel oder eine Kennzahl in allen Berichten. */
export interface CompareRow {
  key: string;
  label: string;
  corridor?: string;
  cells: CompareCell[];
  /** Pass wechselt in mindestens einer Spalte. */
  tipped: boolean;
  /** Wert oder Status weicht in mindestens einer Spalte von A ab. */
  changed: boolean;
}

export interface Comparison {
  seeds: number[];
  passes: boolean[];
  targets: CompareRow[];
  figures: CompareRow[];
}

/** Schlüssel eines Ziels wie targetKey in Go (balance/summary.go): Szenario gehört dazu. */
export const targetKey = (e: BalanceEntry) => `${e.measure}|${e.name}|${e.players}|${e.bot}|${e.depth}|${e.days}`;

const diff = (a: number | null, b: number | null) => (a === null || b === null ? null : b - a);
const label = (e: BalanceEntry) => `${e.name} (${e.players} Sp., ${e.bot}, Tiefe ${e.depth}, ${e.days} T.)`;

/** Wie Go: Pass wechselt; ein neues Ziel kippt, wenn es nicht besteht, ein entfallenes, wenn es bestand. */
const tipped = (a?: BalanceEntry, b?: BalanceEntry) => (a && b ? a.pass !== b.pass : a ? a.pass : b ? !b.pass : false);

function finish(key: string, text: string, cells: CompareCell[], corridor?: string): CompareRow {
  return {
    key, label: text, corridor, cells, tipped: cells.some((c) => c.tipped),
    changed: cells.some((c, i) => i > 0 && ((c.delta !== null && c.delta !== 0) || c.status !== cells[0].status)),
  };
}

const cellOf = (value: number | null, base: number | null, i: number, status?: string, isTipped = false): CompareCell =>
  ({ value, delta: i === 0 ? null : diff(base, value), status, tipped: i > 0 && isTipped });

function compareTargets(list: BalanceSummary[]): CompareRow[] {
  const maps = list.map((s) => new Map(s.targets.map((e) => [targetKey(e), e])));
  const keys = [...new Set([...list.slice(1).flatMap((s) => s.targets.map(targetKey)), ...list[0].targets.map(targetKey)])];
  return keys.map((key) => {
    const entries = maps.map((m) => m.get(key));
    const first = [...entries.slice(1), entries[0]].find(Boolean)!;
    const base = entries[0]?.value ?? null;
    const cells = entries.map((e, i) => cellOf(e?.value ?? null, base, i, e?.status ?? (i === 0 ? 'neu' : 'entfällt'), tipped(entries[0], e)));
    return finish(key, label(first), cells, corridorText(first));
  });
}

type Night = NonNullable<BalanceSummary['kennzahlen']>['nights'][number];
const net = (n: Night | undefined, part: 'day' | 'night') =>
  n && (part === 'day' ? n.dayIncome - n.dayExpense : n.nightIncome - n.nightExpense);

const METRICS: [string, string, (n: Night | undefined) => number | undefined][] = [
  ['surv', 'Überleben', (n) => n?.survival], ['day', 'Gold-Bilanz Tag', (n) => net(n, 'day')],
  ['night', 'Gold-Bilanz Nacht', (n) => net(n, 'night')], ['idle', 'Wartezeit-Anteil', (n) => n?.idleShare],
];

function nightRows(list: BalanceSummary[]): CompareRow[] {
  const keys = [...new Set(list.flatMap((s) => (s.kennzahlen?.nights ?? []).map((n) => `${n.grade}|${n.night}`)))].sort();
  return keys.flatMap((k) => {
    const nights = list.map((s) => s.kennzahlen?.nights.find((n) => `${n.grade}|${n.night}` === k));
    const name = `${k.split('|')[0]} Nacht ${k.split('|')[1]}`;
    return METRICS.map(([id, text, pick]) => {
      const values = nights.map((n) => pick(n) ?? null);
      return finish(`${k}|${id}`, `${name}: ${text}`, values.map((v, i) => cellOf(v, values[0], i)));
    });
  });
}

function compareFigures(list: BalanceSummary[]): CompareRow[] {
  if (list.every((s) => !s.kennzahlen)) return [];
  const row = (key: string, text: string, pick: (s: BalanceSummary) => number | null | undefined) => {
    const values = list.map((s) => pick(s) ?? null);
    return finish(key, text, values.map((v, i) => cellOf(v, values[0], i)));
  };
  return [
    row('wall', 'Erste Mauer (Tick, Median)', (s) => s.kennzahlen?.firstWallTickMedian),
    row('workshop', 'Erste Werkstatt (Tick, Median)', (s) => s.kennzahlen?.firstWorkshopTickMedian),
    ...nightRows(list),
  ];
}

/** Vergleich von 2 bis 4 Berichten (wie balance.Compare in Go, dazu Werte und Kennzahlen): Spalte 0 = A (Basis). */
export function compareSummaries(list: BalanceSummary[]): Comparison {
  return { seeds: list.map((s) => s.seeds), passes: list.map((s) => s.pass), targets: compareTargets(list), figures: compareFigures(list) };
}

/** Nur die Ziele, deren Pass in mindestens einer Spalte wechselt. */
export const tippedRows = (c: Comparison) => c.targets.filter((r) => r.tipped);

/** Wie viele Ziele eine Spalte gegenüber A auf Pass bzw. auf Fail kippt. */
export function flips(c: Comparison, column: number): { toPass: number; toFail: number } {
  const cells = c.targets.filter((r) => r.cells[column].tipped);
  const toPass = cells.filter((r) => r.cells[column].status === 'ok' || r.cells[column].status === 'knapp').length;
  return { toPass, toFail: cells.length - toPass };
}

/** „tree.markCost“ aus `economy.json:gatherables.tree.markCost`: die letzten zwei Teile des Pfads. */
export const shortPath = (path: string) => path.split(':').pop()!.split('.').slice(-2).join('.');

/** Varianten einer Spalte gegenüber A: `tree.markCost 1 → 0.5`, wo A denselben Wert auch variiert, sonst `tree.markCost = 0.5`. */
export function variantTexts(base: readonly string[], own: readonly string[]): string[] {
  const split = (v: string) => [v.slice(0, v.lastIndexOf('=')), v.slice(v.lastIndexOf('=') + 1)] as const;
  const before = new Map(base.map(split));
  return own.map(split).map(([p, v]) => (before.has(p) ? `${shortPath(p)} ${before.get(p)} → ${v}` : `${shortPath(p)} = ${v}`));
}
