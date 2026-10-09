import { describe, expect, it } from 'vitest';
import type { ToolStats, UsageBucket } from '../api';
import { sortByCalls, toolLine } from './overview';
import { rangeScope } from './rangeScope';
import { kpis, rateText } from './stats';

const MIN = 60_000;
const now = Date.UTC(2026, 9, 7, 12, 0, 30);
const bucket = (ago: number, calls: Record<string, number>, ms: Record<string, number>, errors = 0): UsageBucket => ({
  ts: Math.floor(now / MIN) * MIN - ago * MIN, calls, errors, ms, maxMs: ms, outliers: {},
});

describe('Zeitraum-Statistik (B-350)', () => {
  const minutes = [
    bucket(2, { sim_test: 3 }, { sim_test: 30 }, 1),
    bucket(30, { check_run: 2 }, { check_run: 4000 }),
    bucket(60 * 30, { sim_test: 5 }, { sim_test: 50 }),
  ];
  const names = ['check_run', 'plan_get', 'sim_test'];

  it('15 min zählt nur die Minuten im Fenster, alle Tools erscheinen', () => {
    const s = rangeScope(minutes, names, '15m', now);
    expect(s.calls).toBe(3);
    expect(s.errors).toBe(1);
    expect(s.tools.map((t) => [t.name, t.calls])).toEqual([['check_run', 0], ['plan_get', 0], ['sim_test', 3]]);
    expect(s.tools.find((t) => t.name === 'sim_test')?.avgMs).toBe(10);
  });

  it('1 h, 24 h und 7 T wachsen mit dem Fenster', () => {
    expect(rangeScope(minutes, names, '1h', now).calls).toBe(5);
    expect(rangeScope(minutes, names, '24h', now).calls).toBe(5);
    expect(rangeScope(minutes, names, '7d', now).calls).toBe(10);
  });

  it('ohne Perzentile und Fehler je Tool: Anzeige „–“', () => {
    const s = rangeScope(minutes, names, '1h', now);
    const k = Object.fromEntries(kpis(s, now).map((x) => [x.label, x.value]));
    expect(k.P50).toBe('–');
    expect(k.P95).toBe('–');
    expect(k.AUFRUFE).toBe('5');
    expect(rateText(NaN, 3)).toBe('–');
  });

  it('leerer Zeitraum ergibt Nullen, keinen Fehler', () => {
    const s = rangeScope([], names, '15m', now);
    expect(s.calls).toBe(0);
    expect(s.tools).toHaveLength(3);
  });
});

describe('Tool-Kacheln (B-350)', () => {
  const tool = (name: string, calls: number, avgMs = 0, errors = 0): ToolStats => ({
    name, description: `${name} tut etwas`, calls, errors, avgMs, lastCall: '',
  });

  it('alle Tools, nach Aufrufen sortiert, auch ohne Aufruf', () => {
    const tools = [tool('b', 0), tool('a', 0), tool('c', 4, 12)];
    expect(sortByCalls(tools).map((t) => t.name)).toEqual(['c', 'a', 'b']);
  });

  it('kleine Statistik je Kachel', () => {
    expect(toolLine(tool('a', 0))).toBe('noch kein Aufruf');
    expect(toolLine(tool('a', 12, 34))).toBe('12× · Ø 34 ms');
    expect(toolLine(tool('a', 3, 1500, 1))).toBe('3× · Ø 1,5 s · 1 Fehler');
  });
});
