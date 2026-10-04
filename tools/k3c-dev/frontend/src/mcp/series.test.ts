import { describe, expect, it } from 'vitest';
import type { UsageBucket } from '../api';
import { avgOf, header, pointsOf, RANGES, RANGE_SPEC, ticks, windowOf } from './series';

const MIN = 60_000;
const now = Date.parse('2026-09-30T12:00:30Z');
const minute = (agoMin: number, calls: Record<string, number>, over: Partial<UsageBucket> = {}): UsageBucket => {
  const ts = Math.floor(now / MIN) * MIN - agoMin * MIN;
  const ms = Object.fromEntries(Object.entries(calls).map(([t, n]) => [t, n * 100]));
  return { ts, calls, errors: 0, ms, maxMs: ms, outliers: {}, ...over };
};

describe('Live-Monitore', () => {
  it('Fenster je Zeitraum wandert mit der Uhr, der letzte Punkt enthält jetzt', () => {
    for (const r of RANGES) {
      const w = windowOf(r, now);
      expect(w.to - w.from).toBe(RANGE_SPEC[r].span);
      expect(w.count).toBe(RANGE_SPEC[r].span / RANGE_SPEC[r].step);
      expect(now).toBeGreaterThanOrEqual(w.to - w.step);
      expect(now).toBeLessThan(w.to);
    }
    expect(windowOf('15m', now).count).toBe(15);
    expect(windowOf('1h', now).count).toBe(30);
    expect(windowOf('24h', now).count).toBe(48);
    expect(windowOf('7d', now).count).toBe(28);
  });

  it('Punkte aus Minuten, außerhalb fällt weg', () => {
    const w = windowOf('15m', now);
    const pts = pointsOf([
      minute(0, { check_run: 2 }, { errors: 1, maxMs: { check_run: 900 }, outliers: { check_run: 1 } }),
      minute(3, { logs_query: 1 }),
      minute(20, { logs_query: 5 }),
    ], w);
    expect(pts.length).toBe(15);
    expect(pts.at(-1)).toMatchObject({ calls: 2, errors: 1, maxMs: 900, sumMs: 200, outliers: 1 });
    expect(pts.at(-4)?.calls).toBe(1);
    expect(pts.reduce((a, p) => a + p.calls, 0)).toBe(3);
  });

  it('in 7 T fassen Punkte 6 h zusammen', () => {
    const w = windowOf('7d', now);
    // jetzt 12:00:30 UTC: 12:00 liegt im laufenden Punkt (12–18 Uhr), 11:50 im vorigen
    const pts = pointsOf([minute(0, { a: 1 }), minute(10, { a: 1 }), minute(60 * 24 * 6, { a: 4 })], w);
    expect([pts.at(-2)?.calls, pts.at(-1)?.calls]).toEqual([1, 1]);
    expect(pts.reduce((a, p) => a + p.calls, 0)).toBe(6);
  });

  it('Kopfzeilen beider Metriken, leer ohne Division durch null', () => {
    const w = windowOf('15m', now);
    const pts = pointsOf([minute(0, { a: 3 }, { errors: 1, maxMs: { a: 1500 }, outliers: { a: 1 } }), minute(1, { a: 1 })], w);
    expect(header(pts, 'calls')).toBe('4 Aufrufe · 1 Fehler · max 3/Punkt');
    expect(header(pts, 'duration')).toBe('max 1,5 s · 1 Ausreißer · Ø 100 ms');
    const empty = pointsOf([], w);
    expect(header(empty, 'calls')).toBe('0 Aufrufe · 0 Fehler · max 0/Punkt');
    expect(header(empty, 'duration')).toBe('max – · 0 Ausreißer · Ø –');
    expect(avgOf(empty[0])).toBe(null);
  });

  it('Achse mit Uhrzeit, bei 7 T Datum, rechts jetzt', () => {
    const t = ticks('24h', windowOf('24h', now));
    expect(t.length).toBe(5);
    expect(t[4]).toEqual({ at: 1, label: 'jetzt' });
    expect(t[0].label).toMatch(/^\d\d:\d\d$/);
    expect(ticks('7d', windowOf('7d', now))[0].label).toMatch(/^\d\d\.\d\d\.$/);
  });
});
