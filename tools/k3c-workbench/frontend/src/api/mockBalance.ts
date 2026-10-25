import { mockInsights } from './mockInsights';
import type { BalanceBackend, BalanceEntry, BalanceFigures, BalanceReportInfo, BalanceRun, BalanceSummary, NightFigures, Tuning } from './types';

// Mock der Balancing-RPC (WZ28.5): Warteschlange mit einem rechnenden Lauf, Fortschritt per Timer, Beispielbericht.

const TICK_MS = 250;
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

function finishTuning(r: BalanceRun, n: number, reports: BalanceReportInfo[], tunings: Map<string, Tuning>) {
  const path = `reports/simtest-run-${n}/tuning-${n}.json`;
  tunings.set(path, exampleTuning(r.spec?.tune?.params.length));
  reports.unshift({ path, run: `simtest-run-${n}`, modified: new Date().toISOString(), seeds: 0, pass: true, kind: 'tuning' });
  Object.assign(r, { state: 'fertig', phase: 'fertig', done: r.total, ended: new Date().toISOString(), verdict: 'Pass', report: path });
}

function seedReports(): BalanceReportInfo[] {
  return [
    { path: 'reports/tuning-1.json', run: '', modified: '2026-10-24T09:00:00Z', seeds: 0, pass: true, kind: 'tuning' },
    { path: 'reports/balance-1.json', run: '', modified: '2026-10-23T18:00:00Z', seeds: 8, pass: true, kind: 'balance', rating: 'referenz' },
    { path: 'reports/simtest-run-2/balance-2.json', run: 'simtest-run-2', modified: '2026-10-24T08:00:00Z', seeds: 8, pass: false, varianten: ['bot.json:a=1'], kind: 'balance' },
  ];
}

/** Bewerten und Löschen im Mock (WZ28.13): ändert die Liste im Speicher. */
function manageReports(reports: BalanceReportInfo[]): Pick<BalanceBackend, 'balanceReportRate' | 'balanceReportDelete'> {
  const find = (path: string) => {
    const i = reports.findIndex((r) => r.path === path);
    if (i < 0) throw new Error(`bericht ${path} unbekannt`);
    return i;
  };
  return {
    balanceReportRate: async (path, rating) => {
      reports[find(path)].rating = rating || undefined;
    },
    balanceReportDelete: async (path) => {
      reports.splice(find(path), 1);
    },
  };
}

/** Mock-Backend: Läufe warten nacheinander, der vorderste läuft (Fortschritt je Tick) und schreibt dann einen Bericht. */
export function mockBalance(): Omit<BalanceBackend, 'balanceDataFiles' | 'balanceDataDoc' | 'balanceDataValues'> {
  const runs: BalanceRun[] = [], reports = seedReports();
  const tunings = new Map<string, Tuning>([['reports/tuning-1.json', exampleTuning(1)]]);
  const summaries = new Map<string, BalanceSummary>([
    ['reports/balance-1.json', exampleSummary()],
    ['reports/simtest-run-2/balance-2.json', exampleSummary(0.15, true, ['bot.json:a=1'])],
  ]);
  let n = 2, timer: ReturnType<typeof setInterval> | undefined;
  const finish = (r: BalanceRun) => {
    n++;
    if (r.spec?.tune) return finishTuning(r, n, reports, tunings);
    const path = `reports/simtest-run-${n}/balance-${n}.json`;
    const sum = exampleSummary(0, false, r.spec?.vary);
    summaries.set(path, sum);
    reports.unshift({ path, run: `simtest-run-${n}`, modified: new Date().toISOString(), seeds: sum.seeds, pass: sum.pass, varianten: sum.varianten, kind: 'balance' });
    Object.assign(r, { state: 'fertig', phase: 'fertig', done: r.total, ended: new Date().toISOString(), verdict: 'Pass', report: path });
  };
  const tick = () => {
    let cur = runs.find((r) => r.state === 'läuft');
    if (!cur) {
      cur = runs.find((r) => r.state === 'wartet');
      if (cur) Object.assign(cur, { state: 'läuft', phase: 'rechnen' });
    } else if (++cur.done >= cur.total) finish(cur);
    else cur.lines.push(`Lauf ${cur.done}/${cur.total}`);
    if (!runs.some((r) => r.state === 'wartet' || r.state === 'läuft')) {
      clearInterval(timer);
      timer = undefined;
    }
  };
  return {
    balanceStart: async (spec) => {
      const id = `run-${runs.length + 1}`;
      const run: BalanceRun = {
        id, label: spec.label || id, where: 'Repo-Wurzel', state: 'wartet', phase: 'wartet', done: 0, total: TOTAL,
        started: new Date().toISOString(), verdict: '', report: '', lines: [], spec,
      };
      runs.push(run);
      timer ??= setInterval(tick, TICK_MS);
      return structuredClone(run);
    },
    balanceRuns: async () => structuredClone(runs),
    balanceStop: async (id) => {
      const r = runs.find((x) => x.id === id);
      if (!r) throw new Error(`lauf ${id} unbekannt`);
      if (r.state === 'wartet' || r.state === 'läuft') Object.assign(r, { state: 'abgebrochen', phase: 'abgebrochen', ended: new Date().toISOString() });
    },
    balanceReports: async () => structuredClone(reports),
    ...manageReports(reports), ...mockInsights(summaries),
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
