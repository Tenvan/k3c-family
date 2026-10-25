import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { clientLog } from '../core/clientLog';
import { DIAG_EVERY_MS, collectPerf, startDiag, type PerfValues } from './clientPerf';

vi.mock('../core/clientLog', () => ({ clientLog: vi.fn() }));
const log = vi.mocked(clientLog);

describe('collectPerf (B-351)', () => {
  it('rundet wie das Debug-Overlay und nimmt das Mittel der Latenz', () => {
    expect(collectPerf({ fps: 59.6, latency: { mean: 41.4, p95: 80 }, delayMs: 99.5 })).toEqual({ fps: 60, latencyMs: 41, bufferMs: 100 });
  });

  it('ohne Latenz-Messung ist latencyMs null', () => {
    expect(collectPerf({ fps: 30, latency: null, delayMs: 33 }).latencyMs).toBeNull();
  });
});

describe('startDiag (AC-01)', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    log.mockClear();
  });
  afterEach(() => vi.useRealTimers());

  const values: PerfValues = { fps: 58, latencyMs: 42, bufferMs: 100 };

  it('meldet im Takt eine info-Zeile mit ctx {fps, latencyMs, bufferMs}', () => {
    const stop = startDiag(() => values);
    vi.advanceTimersByTime(DIAG_EVERY_MS - 1);
    expect(log).not.toHaveBeenCalled();
    vi.advanceTimersByTime(1);
    expect(log).toHaveBeenCalledTimes(1);
    expect(log).toHaveBeenLastCalledWith('info', expect.stringMatching(/^💓 /), { fps: 58, latencyMs: 42, bufferMs: 100 });
    vi.advanceTimersByTime(DIAG_EVERY_MS * 2);
    expect(log).toHaveBeenCalledTimes(3);
    stop();
    vi.advanceTimersByTime(DIAG_EVERY_MS * 3);
    expect(log).toHaveBeenCalledTimes(3);
  });

  it('jede Zeile hat einen eigenen Text, damit die Drosselung von clientLog sie nicht schluckt', () => {
    const stop = startDiag(() => values);
    vi.advanceTimersByTime(DIAG_EVERY_MS * 2);
    stop();
    const [first, second] = log.mock.calls.map((c) => c[1] as string);
    expect(first).not.toBe(second);
  });

  it('ohne Messwerte (kein Raum, Tab im Hintergrund) entfällt die Zeile', () => {
    const stop = startDiag(() => null);
    vi.advanceTimersByTime(DIAG_EVERY_MS * 3);
    stop();
    expect(log).not.toHaveBeenCalled();
  });

  it('der Text nennt FPS, Latenz und Puffer', () => {
    const stop = startDiag(() => values);
    vi.advanceTimersByTime(DIAG_EVERY_MS * 3);
    stop();
    expect(log.mock.calls[2]?.[1]).toBe('💓 Diagnose #3: 58 FPS, Latenz 42 ms, Puffer 100 ms');
  });
});
