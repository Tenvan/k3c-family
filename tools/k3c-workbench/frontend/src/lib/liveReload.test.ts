import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { liveReload, type Visibility } from './liveReload';

function fakeDoc(): Visibility & { hidden: boolean; show: () => void } {
  const fns = new Set<() => void>();
  return {
    hidden: false,
    addEventListener: (_t, fn) => fns.add(fn),
    removeEventListener: (_t, fn) => fns.delete(fn),
    show() { this.hidden = false; fns.forEach((f) => f()); },
  };
}

describe('liveReload', () => {
  beforeEach(() => { vi.useFakeTimers(); });
  afterEach(() => { vi.useRealTimers(); });

  it('B-895/AC-01: 10 000 Ereignisse in 10 s ergeben höchstens ein Nachladen je Takt', () => {
    const run = vi.fn();
    const r = liveReload(run, 5000, fakeDoc());
    for (let i = 0; i < 10_000; i++) {
      r.soon();
      vi.advanceTimersByTime(1);
    }
    vi.advanceTimersByTime(5000);
    expect(run.mock.calls.length).toBeGreaterThanOrEqual(2);
    expect(run.mock.calls.length).toBeLessThanOrEqual(4); // 0 s, 5 s, 10 s
    r.stop();
  });

  it('das erste Ereignis nach einer Pause lädt sofort', () => {
    const run = vi.fn();
    const r = liveReload(run, 5000, fakeDoc());
    r.soon();
    vi.advanceTimersByTime(0);
    expect(run).toHaveBeenCalledTimes(1);
    r.stop();
  });

  it('verborgen lädt nichts und holt es beim Sichtbarwerden einmal nach', () => {
    const run = vi.fn();
    const doc = fakeDoc();
    doc.hidden = true;
    const r = liveReload(run, 5000, doc);
    for (let i = 0; i < 100; i++) {
      r.soon();
      vi.advanceTimersByTime(1000);
    }
    expect(run).not.toHaveBeenCalled();
    doc.show();
    vi.advanceTimersByTime(0);
    expect(run).toHaveBeenCalledTimes(1);
    r.stop();
  });

  it('stop verwirft ein ausstehendes Nachladen', () => {
    const run = vi.fn();
    const r = liveReload(run, 5000, fakeDoc());
    r.soon();
    r.stop();
    vi.advanceTimersByTime(10_000);
    expect(run).not.toHaveBeenCalled();
  });
});
