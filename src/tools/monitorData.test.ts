import { describe, expect, it } from 'vitest';
import {
  applyDelta,
  emptyState,
  HOUR_MS,
  outlierLimit,
  percentile,
  roomSummaries,
  stats,
  windowValues,
  type MetricsResponse,
} from './monitorData';

const T0 = 1_791_212_400_000;

function delta(startedAt: number, now: number, part: Partial<MetricsResponse> = {}): MetricsResponse {
  return { startedAt, now, server: [], rooms: {}, devices: {}, events: [], ...part };
}

const room = (t: number, maxMs: number, over = 0) => ({ t, maxMs, p99Ms: maxMs, over });
const rtt = (t: number, rttMs: number) => ({ t, rttMs, queue: 0, dropped: 0 });

describe('Perzentile und Ausreißer (B-282/AC-01)', () => {
  // feste Reihe: 1..20 ms plus eine Spitze von 90 ms
  const series = [...Array.from({ length: 20 }, (_, i) => i + 1), 90];

  it('Nearest-Rank über eine feste Reihe', () => {
    expect(stats(series)).toEqual({ n: 21, p50: 11, p95: 20, p99: 90, max: 90 });
    expect(percentile([4, 8, 15, 16, 23, 42], 50)).toBe(15);
    expect(percentile([7], 99)).toBe(7);
    expect(Number.isNaN(percentile([], 50))).toBe(true);
    expect(stats([])).toBeNull();
  });

  it('Reihenfolge der Eingabe spielt keine Rolle', () => {
    expect(stats([...series].reverse())).toEqual(stats(series));
  });

  it('Tukey-Zaun markiert nur die Spitze', () => {
    const limit = outlierLimit(series); // Q1 6, Q3 16 → 16 + 15 = 31
    expect(limit).toBe(31);
    expect(series.filter((v) => v > limit)).toEqual([90]);
    expect(outlierLimit([])).toBe(Infinity);
  });
});

describe('Delta-Puffer (B-282/AC-02)', () => {
  it('hängt Deltas an und behält das Gerät mit seinem Raum', () => {
    let s = applyDelta(emptyState(), delta(T0, T0 + 2000, { rooms: { KRNZ: [room(T0 + 1000, 4)] } }));
    s = applyDelta(
      s,
      delta(T0, T0 + 3000, {
        rooms: { KRNZ: [room(T0 + 2000, 5)] },
        devices: { 'xbox-a1b': { room: 'KRNZ', points: [rtt(T0 + 2500, 12)] } },
        events: [{ t: T0 + 2600, room: 'KRNZ', kind: 'drop', text: 'handy-7f' }],
      }),
    );
    expect(s.rooms.KRNZ.map((p) => p.maxMs)).toEqual([4, 5]);
    expect(s.devices['xbox-a1b'].room).toBe('KRNZ');
    expect(s.events).toHaveLength(1);
    expect(s.restarts).toEqual([]);
  });

  it('neuer startedAt verwirft alte Punkte und merkt den Neustart', () => {
    const s1 = applyDelta(emptyState(), delta(T0, T0 + 5000, { rooms: { KRNZ: [room(T0 + 4000, 4)] } }));
    const T1 = T0 + 60_000;
    const s2 = applyDelta(s1, delta(T1, T1 + 2000, { rooms: { ABCD: [room(T1 + 1000, 3)] } }));
    expect(Object.keys(s2.rooms)).toEqual(['ABCD']);
    expect(s2.startedAt).toBe(T1);
    expect(s2.restarts).toEqual([T1]);
  });

  it('begrenzt das Fenster auf 1 h vor now und entfernt leere Reihen', () => {
    const old = T0 + 1000;
    let s = applyDelta(
      emptyState(),
      delta(T0, T0 + 2000, {
        rooms: { KRNZ: [room(old, 4)] },
        devices: { weg: { room: 'KRNZ', points: [rtt(old, 10)] } },
        server: [{ t: old, heapMB: 6, gcPauseMs: 0, goroutines: 20, cpu: -1, saveMs: 0 }],
      }),
    );
    const now = old + HOUR_MS + 1;
    s = applyDelta(s, delta(T0, now, { rooms: { KRNZ: [room(now - 1000, 6)] } }));
    expect(s.rooms.KRNZ.map((p) => p.maxMs)).toEqual([6]);
    expect(s.devices.weg).toBeUndefined();
    expect(s.server).toEqual([]);
  });
});

describe('Auswertung im Fenster und Ampel', () => {
  const now = T0 + 10 * 60_000;
  const s = applyDelta(
    emptyState(),
    delta(T0, now, {
      rooms: { GRUN: [room(now - 1000, 4)], ROTX: [room(now - 1000, 40, 3), room(now - 9 * 60_000, 5, 1)] },
      devices: {
        a: { room: 'GRUN', points: [rtt(now - 500, 12)] },
        b: { room: 'GRUN', points: [rtt(now - 500, 40)] },
        c: { room: 'GELB', points: [rtt(now - 500, -1)] },
      },
      events: [{ t: now - 2000, room: 'ROTX', kind: 'slow', text: '🐢' }],
    }),
  );

  it('Fenster wählt Punkte, zählt Überschreitungen und lässt „kein Pong“ weg', () => {
    expect(windowValues(s, 5 * 60_000)).toEqual({ tick: [4, 40], rtt: [12, 40], over: 3 });
    expect(windowValues(s, HOUR_MS, 'ROTX')).toEqual({ tick: [40, 5], rtt: [], over: 4 });
  });

  it('Ampel je Raum', () => {
    expect(roomSummaries(s)).toEqual([
      { code: 'GELB', light: 'grey', tickP99: null, worstRtt: -1, errors: 0 },
      { code: 'GRUN', light: 'green', tickP99: 4, worstRtt: 40, errors: 0 },
      { code: 'ROTX', light: 'red', tickP99: 40, worstRtt: null, errors: 1 },
    ]);
  });
});
