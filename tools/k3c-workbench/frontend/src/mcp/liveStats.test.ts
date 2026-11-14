import { describe, expect, it } from 'vitest';
import type { UsageBucket } from '../api';
import { axisOf, barLabel, barValue, outlierNote, pointTip, stepLabel, toolTotals, unitLabel } from './liveStats';

const MIN = 60_000;
const bucket = (ts: number, calls: Record<string, number>, ms: Record<string, number>, outliers: Record<string, number> = {}): UsageBucket =>
  ({ ts, calls, errors: 0, ms, maxMs: ms, outliers });

describe('toolTotals', () => {
  const minutes = [
    bucket(0, { a: 2, b: 1 }, { a: 400, b: 3000 }, { b: 1 }),
    bucket(MIN, { a: 3 }, { a: 600 }),
    bucket(10 * MIN, { c: 9 }, { c: 90 }), // außerhalb des Fensters
  ];

  it('summiert je Tool im Fenster, meiste Aufrufe zuerst', () => {
    const t = toolTotals(minutes, { from: 0, to: 5 * MIN });
    expect(t.map((x) => x.name)).toEqual(['a', 'b']);
    expect(t[0]).toEqual({ name: 'a', calls: 5, sumMs: 1000, maxMs: 600, outliers: 0 });
    expect(t[1].outliers).toBe(1);
  });

  it('Balken mit Einheit: Aufrufe oder Ø-Laufzeit', () => {
    const [a, b] = toolTotals(minutes, { from: 0, to: 5 * MIN });
    expect(barValue(a, 'calls')).toBe(5);
    expect(barLabel(a, 'calls')).toBe('5×');
    expect(barValue(b, 'duration')).toBe(3000);
    expect(barLabel(b, 'duration')).toBe('Ø 3,0 s');
  });
});

describe('Achsen und Beschriftung', () => {
  it('Werteachse ab 0 mit runden Marken über dem Höchstwert', () => {
    const calls = axisOf(13, 'calls');
    expect(calls.max).toBeGreaterThanOrEqual(13);
    expect(calls.ticks[0]).toEqual({ v: 0, label: '0' });
    expect(calls.ticks.every((t) => Number.isInteger(t.v))).toBe(true);
    const dur = axisOf(2300, 'duration');
    expect(dur.ticks.at(-1)?.label).toMatch(/s$/);
    expect(axisOf(0, 'calls').max).toBeGreaterThan(0);
  });

  it('Einheit nennt den Punkt-Abstand', () => {
    expect(stepLabel('15m')).toBe('1 min');
    expect(stepLabel('7d')).toBe('6 h');
    expect(unitLabel('1h', 'calls')).toBe('Aufrufe je 2 min');
  });

  it('Tooltip eines Punkts und Erklärung der Ausreißer aus den Regeln', () => {
    const p = { ts: Date.parse('2026-11-14T10:00:00'), calls: 4, errors: 1, maxMs: 2500, sumMs: 4000, outliers: 1 };
    expect(pointTip(p, '15m')).toMatch(/4 Aufrufe · 1 Fehler · max 2,5 s · Ø 1,0 s · 1 Ausreißer$/);
    expect(pointTip({ ...p, calls: 0 }, '15m')).toMatch(/keine Aufrufe$/);
    expect(outlierNote({ outlierFactor: 2, outlierFloorMs: 1000, baselineCalls: 8, percentileErrorPct: 1 }))
      .toContain('2,0× das p95');
  });
});
