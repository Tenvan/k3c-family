import type { BalanceEntry, BalanceSummary } from '../api';

/** Zeile des Vergleichs: ein Wert in Bericht A und B mit Differenz (B − A). */
export interface CompareRow {
  key: string;
  label: string;
  a: number | null;
  b: number | null;
  delta: number | null;
  statusA?: string;
  statusB?: string;
  /** Pass wechselt zwischen A und B. */
  tipped?: boolean;
}

export interface Comparison {
  seedsA: number;
  seedsB: number;
  passA: boolean;
  passB: boolean;
  targets: CompareRow[];
  figures: CompareRow[];
}

/** Schlüssel eines Ziels wie targetKey in Go (balance/summary.go): Szenario gehört dazu. */
export const targetKey = (e: BalanceEntry) => `${e.measure}|${e.name}|${e.players}|${e.bot}|${e.depth}|${e.days}`;

const diff = (a: number | null, b: number | null) => (a === null || b === null ? null : b - a);
const label = (e: BalanceEntry) => `${e.name} (${e.players} Sp., ${e.bot}, Tiefe ${e.depth}, ${e.days} T.)`;

/** Wie Go: Pass wechselt; ein neues Ziel kippt, wenn es nicht besteht, ein entfallenes, wenn es bestand. */
const tipped = (a?: BalanceEntry, b?: BalanceEntry) => (a && b ? a.pass !== b.pass : a ? a.pass : b ? !b.pass : false);

function compareTargets(a: BalanceSummary, b: BalanceSummary): CompareRow[] {
  const mapA = new Map(a.targets.map((e) => [targetKey(e), e]));
  const mapB = new Map(b.targets.map((e) => [targetKey(e), e]));
  const keys = [...b.targets.map(targetKey), ...a.targets.map(targetKey).filter((k) => !mapB.has(k))];
  return keys.map((key) => {
    const ea = mapA.get(key);
    const eb = mapB.get(key);
    return {
      key, label: label((eb ?? ea)!), a: ea?.value ?? null, b: eb?.value ?? null, delta: diff(ea?.value ?? null, eb?.value ?? null),
      statusA: ea?.status ?? 'neu', statusB: eb?.status ?? 'entfällt', tipped: tipped(ea, eb),
    };
  });
}

type Night = NonNullable<BalanceSummary['kennzahlen']>['nights'][number];
const net = (n: Night | undefined, part: 'day' | 'night') =>
  n && (part === 'day' ? n.dayIncome - n.dayExpense : n.nightIncome - n.nightExpense);

function nightRows(a: BalanceSummary, b: BalanceSummary): CompareRow[] {
  const all = [...(a.kennzahlen?.nights ?? []), ...(b.kennzahlen?.nights ?? [])];
  const keys = [...new Set(all.map((n) => `${n.grade}|${n.night}`))].sort();
  const find = (s: BalanceSummary, k: string) => s.kennzahlen?.nights.find((n) => `${n.grade}|${n.night}` === k);
  return keys.flatMap((k) => {
    const [na, nb] = [find(a, k), find(b, k)];
    const name = `${k.split('|')[0]} Nacht ${k.split('|')[1]}`;
    const row = (id: string, text: string, x: number | undefined, y: number | undefined): CompareRow =>
      ({ key: `${k}|${id}`, label: `${name}: ${text}`, a: x ?? null, b: y ?? null, delta: diff(x ?? null, y ?? null) });
    return [
      row('surv', 'Überleben', na?.survival, nb?.survival), row('day', 'Gold-Bilanz Tag', net(na, 'day'), net(nb, 'day')),
      row('night', 'Gold-Bilanz Nacht', net(na, 'night'), net(nb, 'night')), row('idle', 'Wartezeit-Anteil', na?.idleShare, nb?.idleShare),
    ];
  });
}

function compareFigures(a: BalanceSummary, b: BalanceSummary): CompareRow[] {
  const [fa, fb] = [a.kennzahlen, b.kennzahlen];
  if (!fa && !fb) return [];
  const row = (key: string, label: string, x?: number | null, y?: number | null): CompareRow =>
    ({ key, label, a: x ?? null, b: y ?? null, delta: diff(x ?? null, y ?? null) });
  return [
    row('wall', 'Erste Mauer (Tick, Median)', fa?.firstWallTickMedian, fb?.firstWallTickMedian),
    row('workshop', 'Erste Werkstatt (Tick, Median)', fa?.firstWorkshopTickMedian, fb?.firstWorkshopTickMedian),
    ...nightRows(a, b),
  ];
}

/** Vergleich zweier Berichte (wie balance.Compare in Go, dazu Werte und Kennzahlen): A = Basis, B = neu. */
export function compareSummaries(a: BalanceSummary, b: BalanceSummary): Comparison {
  return {
    seedsA: a.seeds, seedsB: b.seeds, passA: a.pass, passB: b.pass, targets: compareTargets(a, b), figures: compareFigures(a, b),
  };
}

/** Nur die Ziele, deren Pass wechselt. */
export const tippedRows = (c: Comparison) => c.targets.filter((r) => r.tipped);
