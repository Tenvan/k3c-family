import type { ToolUsage, UsageBucket, UsageScope } from '../api';
import { windowOf, type Range } from './series';

// Statistik für einen Zeitraum (B-350) aus der Minuten-Zeitreihe: Aufrufe, Dauer, Max und Ausreißer je Tool. Die
// Minuten kennen Fehler nur als Summe und keine Perzentile; diese Werte sind NaN und erscheinen als „–“.

function emptyTool(name: string): ToolUsage {
  return { name, calls: 0, errors: NaN, avgMs: 0, p50Ms: NaN, p95Ms: NaN, maxMs: 0, sumMs: 0, outliers: 0, args: [], topErrors: [] };
}

/** Bereich des Zeitraums; names sind alle bekannten Tools (erscheinen auch ohne Aufruf). */
export function rangeScope(minutes: UsageBucket[], names: string[], range: Range, now: number): UsageScope {
  const w = windowOf(range, now);
  const tools = new Map(names.map((n) => [n, emptyTool(n)]));
  let errors = 0;
  for (const m of minutes) {
    if (m.ts < w.from || m.ts >= w.to) continue;
    errors += m.errors;
    for (const [name, calls] of Object.entries(m.calls)) {
      const t = tools.get(name) ?? emptyTool(name);
      tools.set(name, t);
      t.calls += calls;
      t.sumMs += m.ms[name] ?? 0;
      t.maxMs = Math.max(t.maxMs, m.maxMs[name] ?? 0);
      t.outliers += m.outliers[name] ?? 0;
    }
  }
  const list = [...tools.values()];
  let calls = 0;
  let sumMs = 0;
  let maxMs = 0;
  let outliers = 0;
  for (const t of list) {
    t.avgMs = t.calls > 0 ? t.sumMs / t.calls : 0;
    calls += t.calls;
    sumMs += t.sumMs;
    maxMs = Math.max(maxMs, t.maxMs);
    outliers += t.outliers;
  }
  return {
    since: new Date(w.from).toISOString(), calls, errors, avgMs: calls > 0 ? sumMs / calls : 0, p50Ms: NaN, p95Ms: NaN,
    maxMs, sumMs, outliers, tools: list, topErrors: [], recentOutliers: [], slowest: [],
  };
}
