import { describe, expect, it } from 'vitest';
import { LATENCY_WINDOW_MS, LatencyMeter } from './clientLatency';

describe('LatencyMeter (B-181/AC-01)', () => {
  it('Zeit von gesendeter seq bis zum ersten Zustand mit ack ≥ seq: Mittel und p95', () => {
    const m = new LatencyMeter();
    // 20 Eingaben, Latenz 10, 20, …, 200 ms
    for (let i = 1; i <= 20; i++) {
      m.sent(i, i * 1000);
      m.acked(i, i * 1000 + i * 10);
    }
    expect(m.stats(21_000)).toEqual({ mean: 105, p95: 190 });
  });

  it('ein ack bestätigt alle älteren seq, ein späteres ack zählt nicht doppelt', () => {
    const m = new LatencyMeter();
    m.sent(1, 0);
    m.sent(2, 30);
    m.acked(0, 40); // noch nichts verrechnet
    m.acked(2, 100);
    m.acked(2, 130);
    expect(m.stats(200)).toEqual({ mean: 85, p95: 100 });
  });

  it('ohne Messung im Fenster: null (Anzeige „–“)', () => {
    const m = new LatencyMeter();
    expect(m.stats(0)).toBeNull();
    m.sent(1, 0);
    m.acked(1, 50);
    expect(m.stats(LATENCY_WINDOW_MS)).toEqual({ mean: 50, p95: 50 });
    expect(m.stats(LATENCY_WINDOW_MS + 51)).toBeNull();
  });

  it('reset vergisst offene seq (Wiederverbinden, seq beginnt neu)', () => {
    const m = new LatencyMeter();
    m.sent(5, 0);
    m.reset();
    m.acked(5, 100);
    expect(m.stats(100)).toBeNull();
  });

  it('Messungen außerhalb des Fensters fallen auch ohne stats() weg (Overlay zu)', () => {
    const m = new LatencyMeter();
    for (let i = 1; i <= 3 * 60 * 60; i++) {
      m.sent(i, i * 50);
      m.acked(i, i * 50 + 20);
    }
    const kept = (m as unknown as { samples: unknown[] }).samples.length;
    expect(kept).toBeLessThanOrEqual(LATENCY_WINDOW_MS / 50 + 1);
  });
});
