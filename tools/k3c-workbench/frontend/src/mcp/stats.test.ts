import { describe, expect, it } from 'vitest';
import type { ToolUsage, UsageScope } from '../api';
import { DEFAULT_SORT, kpis, nextSort, rateText, ruleLine, sortTools, timesP95, whenText } from './stats';

const tool = (name: string, calls: number, errors = 0, p95Ms = 100): ToolUsage => ({
  name, calls, errors, avgMs: 50, p50Ms: 40, p95Ms, maxMs: 200, sumMs: 50 * calls, outliers: 0, args: [], topErrors: [],
});
const now = new Date(2026, 8, 30, 12, 0).getTime();
const scope = (over: Partial<UsageScope> = {}): UsageScope => ({
  since: new Date(now - 2 * 3_600_000).toISOString(), calls: 10, errors: 1, avgMs: 50, p50Ms: 40, p95Ms: 1200,
  maxMs: 4000, sumMs: 500, outliers: 2, tools: [tool('a', 7), tool('b', 3), tool('c', 0)], topErrors: [],
  recentOutliers: [], slowest: [], ...over,
});

describe('Statistik', () => {
  it('Kennzahlen mit Fehlerquote, Aufrufe/h und aktiven Tools', () => {
    const k = Object.fromEntries(kpis(scope(), now).map((x) => [x.label, x]));
    expect(k['AUFRUFE'].value).toBe('10');
    expect(k['FEHLERQUOTE']).toEqual({ label: 'FEHLERQUOTE', value: '10,0 %', bad: true });
    expect(k['AUFRUFE/H'].value).toBe('5,0');
    expect(k['AKTIVE TOOLS'].value).toBe('2');
    expect(k['P95'].value).toBe('1,2 s');
    expect(k['AUSREISSER'].bad).toBe(true);
  });

  it('ohne Aufrufe `–` statt Division durch null', () => {
    const k = kpis(scope({ calls: 0, errors: 0, outliers: 0, tools: [] }), now);
    expect(k.map((x) => x.value)).toEqual(['0', '–', '–', '0', '–', '–', '–', '–', '–', '0']);
    expect(rateText(0, 0)).toBe('–');
  });

  it('Erklärzeile aus den Regeln', () => {
    const line = ruleLine({ outlierFactor: 2, outlierFloorMs: 1000, baselineCalls: 8, percentileErrorPct: 12 },
      new Date(now - 3_600_000).toISOString(), now);
    expect(line).toBe('seit 11:00 · Perzentile aus Histogramm, ±12 % · Ausreißer: über 2× p95 der Vergleichsgruppe ' +
      '(gleiches Tool und gleiche Argumente, sonst das Tool) und mindestens 1,0 s; Vergleichsgruppe ab 8 Aufrufen');
  });

  it('Zeitangaben: heute Uhrzeit, sonst Datum und Uhrzeit', () => {
    expect(whenText(new Date(2026, 8, 30, 8, 3).toISOString(), now)).toBe('08:03');
    expect(whenText(new Date(2026, 8, 23, 8, 3).toISOString(), now)).toBe('23.09.2026 08:03');
    expect(whenText('kaputt', now)).toBe('–');
  });

  it('Sortierung: Standard Aufrufe absteigend, Klick dreht, Name aufsteigend', () => {
    const tools = [tool('b', 3, 3), tool('a', 7), tool('c', 3)];
    expect(sortTools(tools, DEFAULT_SORT).map((t) => t.name)).toEqual(['a', 'b', 'c']);
    const byName = nextSort(DEFAULT_SORT, 'name');
    expect(byName).toEqual({ key: 'name', desc: false });
    expect(sortTools(tools, byName).map((t) => t.name)).toEqual(['a', 'b', 'c']);
    expect(sortTools(tools, nextSort(byName, 'name')).map((t) => t.name)).toEqual(['c', 'b', 'a']);
    expect(sortTools(tools, { key: 'rate', desc: true })[0].name).toBe('b');
    expect(nextSort(DEFAULT_SORT, 'calls')).toEqual({ key: 'calls', desc: false });
  });

  it('x× p95', () => {
    const slow = { at: '', tool: 'check_run', args: '{}', durationMs: 32000, ok: true, baselineMs: 10000 };
    expect(timesP95(slow)).toBe('3,2× p95');
    expect(timesP95({ ...slow, baselineMs: 0 })).toBe('');
  });
});
