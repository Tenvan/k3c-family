import { mockInsights } from './mockInsights';
import { mockBaselines, mockManageReports, type BalanceEvents } from './mockBalanceManage';
import type { BalanceBackend, BalanceBaseline, BalanceEntry, BalanceFigures, BalanceReportInfo, BalanceRun, BalanceSpec, BalanceSummary, NightFigures, Tuning } from './types';

// Mock der Balancing-RPC (WZ28.5): Warteschlange mit einem rechnenden Lauf, Fortschritt per Timer, Beispielbericht.
// Wie Go meldet er Änderungen als Ereignis (B-662): balance:runs je Änderung, balance:reports nach neuem Bericht.

const TICK_MS = 250;

/** Gesicherter Basis-Lauf im Mock (B-640): Meilenstein 1. */
const BASELINES: BalanceBaseline[] = [{
  name: 'M1', saved: '2026-10-28T10:00:00+01:00', run: 'simtest-run-7', seeds: 1, seedMode: 'dev', bots: ['coop2'], worlds: ['standard'], pass: true, frozen: ['basis'],
  spec: { seeds: 1, bot: 'coop2', players: 2, world: 'standard', days: 40, label: 'Meilenstein 1' },
}];
const TOTAL = 6;

const target = (name: string, value: number, lower: number | null, upper: number | null, status = 'ok'): BalanceEntry => ({
  measure: 'Überleben', name, kind: 'median', players: 2, bot: 'standard', depth: 1, days: 10, lower, upper, rule: 'docs/rules/nacht.md',
  value, status, pass: status === 'ok' || status === 'knapp',
});

const res = (k: number) => ({ wood: 10 * k, stone: 6 * k, copper: 3 * k, iron: 2 * k, crystal: k });

function nights(shift: number): NightFigures[] {
  return [1, 2, 3, 4, 5].map((n) => ({
    grade: 'normal', night: n, runs: 8, survival: Math.max(0.3, 1 - 0.1 * n - shift), dayIncome: 40 + 12 * n, dayExpense: 30 + 10 * n,
    nightIncome: 8 + 2 * n, nightExpense: 12 + 3 * n, resourceIn: res(n), resourceOut: res(n * 0.8), idleShare: 0.1 + 0.03 * n,
  }));
}

/** Beispielbericht; `shift` verschiebt Überleben, `fail` lässt ein Ziel scheitern. */
export function exampleSummary(shift = 0, fail = false, varianten?: string[]): BalanceSummary {
  const targets = [
    target('Überleben Nacht 3', 0.7 - shift, 0.6, null),
    target('Gold nach Tag 5', 140, 100, 200),
    fail ? target('Erste Mauer (Tick)', 4200, null, 3600, 'verletzt') : target('Erste Mauer (Tick)', 3400, null, 3600, 'knapp'),
  ];
  const kennzahlen: BalanceFigures = { runs: 8, firstWallTickMedian: 3400, firstWorkshopTickMedian: 5200, nights: nights(shift) };
  return { version: 1, seeds: 8, pass: targets.every((t) => t.pass), targets, varianten, kennzahlen };
}

/** Beispiel-Abstimmen: ein Parameter (Raster, Score über Wert) oder zwei (Kombinationen); der Tiefpunkt liegt bei 0,6. */
export function exampleTuning(params = 1): Tuning {
  const mk = (file: string, path: string, min: number, max: number, base: number) => ({ file, path, min, max, base, int: false });
  const axis = [0.2, 0.4, 0.6, 0.8, 1];
  const list = params === 1 ? axis.map((a) => [a]) : axis.slice(0, 3).flatMap((a) => [0.5, 1].map((b) => [a, b]));
  const points = list.map((values) => {
    const score = Math.round(Math.abs(values[0] - 0.6) * 25) / 100;
    return { values, score, pass: score === 0, failed: score === 0 ? [] : ['Überleben Nacht 3'] };
  });
  const best = points.reduce((a, b) => (b.score < a.score ? b : a));
  return {
    version: 1, mode: 'sweep', params: [mk('buildings.json', 'wall.cost', 0.2, 1, 0.8), mk('hub.json', 'tree.wood', 0.5, 1, 1)].slice(0, params),
    points, best: { values: best.values, score: best.score, pass: best.pass },
    reason: `Bester von ${points.length} Punkten (sweep): alle Korridore getroffen (Score 0).`,
    diff: '--- a/data/buildings.json\n+++ b/data/buildings.json\n@@ -3 +3 @@\n-  "cost": 0.8,\n+  "cost": 0.6,\n',
  };
}

/** Label, Auftrag und Lauf-ID, die Go in jeden neuen Bericht stempelt (B-662). */
const stampOf = (r: BalanceRun) => ({ label: r.label, spec: r.spec, runId: r.id });

function finishTuning(r: BalanceRun, n: number, reports: BalanceReportInfo[], tunings: Map<string, Tuning>) {
  const path = `reports/simtest-run-${n}/tuning-${n}.json`;
  tunings.set(path, exampleTuning(r.spec?.tune?.params.length));
  reports.unshift({ path, run: `simtest-run-${n}`, modified: new Date().toISOString(), seeds: 0, pass: true, kind: 'tuning', targetsPass: 0, targetsTotal: 0, ...stampOf(r) });
  Object.assign(r, { state: 'fertig', phase: 'fertig', done: r.total, ended: new Date().toISOString(), verdict: 'Pass', report: path });
}

const RUN2_SPEC: BalanceSpec = { seeds: 8, bot: 'wirtschaft', players: 1, world: 'standard', days: 40, vary: ['bot.json:a=1'], label: 'Insel 1 solo · ohne Hilfen' };

/** Zeitpunkt vor n Tagen um h Uhr, nie in der Zukunft: so zeigt die Mock-Liste immer Heute, Gestern und Älter. */
function ago(daysAgo: number, hour: number): string {
  const d = new Date();
  d.setDate(d.getDate() - daysAgo);
  d.setHours(hour, 0, 0, 0);
  return new Date(Math.min(d.getTime(), Date.now() - 60_000)).toISOString();
}

const RUN4_SPEC: BalanceSpec = { seeds: 8, bot: 'wirtschaft', players: 1, world: 'standard', days: 40, vary: ['economy.json:purse.startGold=20'], label: 'Startgold 20' };
const RUN7_SPEC: BalanceSpec = { seeds: 1, bot: 'coop2', players: 2, world: 'standard', days: 40, label: 'Meilenstein 1' };

/** Startliste: ältere Berichte ohne Label und Auftrag, drei mit (B-662), einer davon die Baseline M1. */
function seedReports(): BalanceReportInfo[] {
  return [
    { path: 'reports/tuning-1.json', run: '', modified: ago(4, 9), seeds: 0, pass: true, kind: 'tuning', targetsPass: 0, targetsTotal: 0 },
    { path: 'reports/balance-1.json', run: '', modified: ago(3, 18), seeds: 8, pass: true, kind: 'balance', rating: 'referenz', targetsPass: 3, targetsTotal: 3 },
    {
      path: 'reports/simtest-run-7/balance-7.json', run: 'simtest-run-7', modified: ago(2, 18), seeds: 1, pass: true, kind: 'balance',
      label: RUN7_SPEC.label, spec: RUN7_SPEC, runId: 'run-7', targetsPass: 3, targetsTotal: 3,
    },
    {
      path: 'reports/simtest-run-4/balance-4.json', run: 'simtest-run-4', modified: ago(1, 18), seeds: 8, pass: false, varianten: RUN4_SPEC.vary,
      kind: 'balance', label: RUN4_SPEC.label, spec: RUN4_SPEC, runId: 'run-4', targetsPass: 1, targetsTotal: 3,
    },
    {
      path: 'reports/simtest-run-2/balance-2.json', run: 'simtest-run-2', modified: ago(0, 8), seeds: 8, pass: false, varianten: ['bot.json:a=1'],
      kind: 'balance', label: RUN2_SPEC.label, spec: RUN2_SPEC, runId: 'run-2', targetsPass: 2, targetsTotal: 3,
    },
  ];
}

/** Aufträge eines Starts: mit `perVariant` Basis-Lauf und je feste Variante einer (wie simrun.perVariant in Go). */
function specsOf(spec: BalanceSpec): BalanceSpec[] {
  if (!spec.perVariant) return [spec];
  if (spec.tune) throw new Error('lauf abgelehnt: je Variante geht nicht mit Abstimmung');
  if (!spec.vary?.length) throw new Error('lauf abgelehnt: je Variante braucht mindestens eine feste Variante');
  const prefix = spec.label || 'je Variante';
  const base: BalanceSpec = { ...spec, perVariant: undefined, vary: undefined, label: `${prefix} · Basis` };
  return [base, ...spec.vary.map((v) => ({ ...base, vary: [v], label: `${prefix} · ${v}`.slice(0, 60) }))];
}

function finishBalance(r: BalanceRun, n: number, reports: BalanceReportInfo[], summaries: Map<string, BalanceSummary>) {
  const path = `reports/simtest-run-${n}/balance-${n}.json`;
  const sum = exampleSummary(0, false, r.spec?.vary);
  summaries.set(path, sum);
  reports.unshift({
    path, run: `simtest-run-${n}`, modified: new Date().toISOString(), seeds: sum.seeds, pass: sum.pass, varianten: sum.varianten, kind: 'balance',
    targetsPass: sum.targets.filter((t) => t.pass).length, targetsTotal: sum.targets.length, ...stampOf(r),
  });
  Object.assign(r, { state: 'fertig', phase: 'fertig', done: r.total, ended: new Date().toISOString(), verdict: 'Pass', report: path });
}

/** Neuer wartender Lauf. */
const newRun = (id: string, spec: BalanceSpec): BalanceRun => ({
  id, label: spec.label || id, where: 'Repo-Wurzel', state: 'wartet', phase: 'wartet', done: 0, total: TOTAL,
  started: new Date().toISOString(), verdict: '', report: '', lines: [], spec,
});

/** Bericht und Abstimm-Ergebnis zu einem Pfad. */
function lookups(summaries: Map<string, BalanceSummary>, tunings: Map<string, Tuning>): Pick<BalanceBackend, 'balanceReport' | 'balanceTuning'> {
  return {
    balanceReport: async (path) => {
      const s = summaries.get(path);
      if (!s) throw new Error(`bericht ${path} unbekannt`);
      return structuredClone(s);
    },
    balanceTuning: async (path) => {
      const t = tunings.get(path);
      if (!t) throw new Error(`tuning ${path} unbekannt`);
      return structuredClone(t);
    },
  };
}

/** Mock-Backend: Läufe warten nacheinander, der vorderste läuft (Fortschritt je Tick) und schreibt dann einen Bericht. */
export function mockBalance(
  events: BalanceEvents,
): Omit<BalanceBackend, 'balanceDataFiles' | 'balanceDataDoc' | 'balanceDataValues' | 'balanceDataValuesAll'> {
  const runs: BalanceRun[] = [], reports = seedReports();
  const tunings = new Map<string, Tuning>([['reports/tuning-1.json', exampleTuning(1)]]);
  const summaries = new Map<string, BalanceSummary>([
    ['reports/balance-1.json', exampleSummary()],
    ['reports/simtest-run-2/balance-2.json', exampleSummary(0.15, true, ['bot.json:a=1'])],
    ['reports/simtest-run-4/balance-4.json', exampleSummary(0.3, true, ['economy.json:purse.startGold=20'])],
    ['reports/simtest-run-7/balance-7.json', exampleSummary()],
  ]);
  let n = 7, timer: ReturnType<typeof setInterval> | undefined;
  const finish = (r: BalanceRun) => {
    n++;
    if (r.spec?.tune) finishTuning(r, n, reports, tunings);
    else finishBalance(r, n, reports, summaries);
    events.reports();
  };
  const tick = () => {
    let cur = runs.find((r) => r.state === 'läuft');
    if (!cur) {
      cur = runs.find((r) => r.state === 'wartet');
      if (cur) Object.assign(cur, { state: 'läuft', phase: 'rechnen' });
    } else if (++cur.done >= cur.total) finish(cur);
    else cur.lines.push(`Lauf ${cur.done}/${cur.total}`);
    if (cur) events.run(structuredClone(cur));
    if (!runs.some((r) => r.state === 'wartet' || r.state === 'läuft')) {
      clearInterval(timer);
      timer = undefined;
    }
  };
  const enqueue = (spec: BalanceSpec): BalanceRun => {
    const run = newRun(`run-${runs.length + 1}`, spec);
    runs.push(run);
    events.run(structuredClone(run));
    return run;
  };
  return {
    balanceStart: async (spec) => {
      const [first] = specsOf(spec).map(enqueue);
      timer ??= setInterval(tick, TICK_MS);
      return structuredClone(first);
    },
    balanceRuns: async () => structuredClone(runs),
    balanceStop: async (id) => {
      const r = runs.find((x) => x.id === id);
      if (!r) throw new Error(`lauf ${id} unbekannt`);
      if (r.state === 'wartet' || r.state === 'läuft') Object.assign(r, { state: 'abgebrochen', phase: 'abgebrochen', ended: new Date().toISOString() });
      events.run(structuredClone(r));
    },
    balanceReports: async () => structuredClone(reports),
    ...mockManageReports(reports, events), ...mockInsights(summaries), ...mockBaselines(structuredClone(BASELINES), reports, events),
    ...lookups(summaries, tunings),
  };
}
