// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import * as hook from './useBalancing';

// B-662/AC-04: Die Balancing-Seite lädt Läufe und Berichte nur beim Sichtbarwerden und bei Ereignissen, ohne Intervall.

const fake = vi.hoisted(() => {
  const listeners = new Map<string, Set<(data: unknown) => void>>();
  return {
    listeners,
    emit: (event: string, data: unknown = null) => listeners.get(event)?.forEach((f) => f(data)),
    backend: {
      balanceRuns: vi.fn(() => Promise.resolve([])),
      balanceReports: vi.fn(() => Promise.resolve([])),
      balanceBaselines: vi.fn(() => Promise.resolve([])),
      balanceReportsDelete: vi.fn(() => Promise.resolve()),
      on: (event: string, fn: (data: unknown) => void) => {
        const set = listeners.get(event) ?? new Set();
        set.add(fn);
        listeners.set(event, set);
        return () => set.delete(fn);
      },
    },
  };
});
vi.mock('../api', () => ({ backend: fake.backend }));

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let state: ReturnType<typeof hook.useBalancing>;
function Probe({ active }: { active: boolean }) {
  state = hook.useBalancing(active);
  return null;
}

const counts = () => [fake.backend.balanceRuns, fake.backend.balanceReports, fake.backend.balanceBaselines].map((f) => f.mock.calls.length);

describe('useBalancing', () => {
  let root: Root;
  beforeEach(() => {
    vi.useFakeTimers();
    vi.clearAllMocks();
    fake.listeners.clear();
    root = createRoot(document.createElement('div'));
  });
  afterEach(() => {
    act(() => root.unmount());
    vi.useRealTimers();
  });

  const settle = () => act(async () => {
    await vi.advanceTimersByTimeAsync(10_000);
  });

  it('lädt einmal beim Sichtbarwerden, danach nur bei Ereignissen und ohne Intervall', async () => {
    expect('POLL_MS' in hook).toBe(false);
    act(() => root.render(<Probe active />));
    await settle();
    expect(counts()).toEqual([1, 1, 1]);
    expect(vi.getTimerCount()).toBe(0);

    await act(async () => fake.emit('balance:runs', { id: 'run-1' }));
    expect(counts()).toEqual([2, 1, 1]);
    await act(async () => fake.emit('balance:reports'));
    expect(counts()).toEqual([2, 2, 2]);
    await act(async () => fake.emit('ui:reconnect'));
    expect(counts()).toEqual([3, 3, 3]);
    await settle();
    expect(counts()).toEqual([3, 3, 3]);
  });

  it('hört im Hintergrund nicht zu und lädt beim Seitenwechsel neu', async () => {
    act(() => root.render(<Probe active={false} />));
    await settle();
    expect(counts()).toEqual([0, 0, 0]);
    await act(async () => fake.emit('balance:runs', { id: 'run-1' }));
    expect(counts()).toEqual([0, 0, 0]);

    act(() => root.render(<Probe active />));
    await settle();
    expect(counts()).toEqual([1, 1, 1]);
    act(() => root.render(<Probe active={false} />));
    await act(async () => fake.emit('balance:reports'));
    expect(counts()).toEqual([1, 1, 1]);
    expect(fake.listeners.get('balance:runs')?.size).toBe(0);
  });

  it('löscht gesammelt in einem Aufruf und lädt die Liste neu', async () => {
    act(() => root.render(<Probe active />));
    await settle();
    let ok = false;
    await act(async () => {
      ok = await state.removeMany(['reports/a/balance-1.json', 'reports/b/balance-2.json']);
    });
    expect(ok).toBe(true);
    expect(fake.backend.balanceReportsDelete).toHaveBeenCalledExactlyOnceWith(['reports/a/balance-1.json', 'reports/b/balance-2.json']);
    expect(counts()[1]).toBe(2);
  });
});

// Review WZ32.1: Die Antwort ersetzt die Lauf-Liste, ein abgebrochener wartender Lauf bleibt nicht als Geist stehen.
describe('useBalancing: Lauf-Liste', () => {
  let root: Root;
  beforeEach(() => {
    vi.useFakeTimers();
    vi.clearAllMocks();
    fake.listeners.clear();
    root = createRoot(document.createElement('div'));
  });
  afterEach(() => {
    act(() => root.unmount());
    vi.useRealTimers();
  });

  it('übernimmt die Liste der Antwort: ein abgebrochener wartender Lauf verschwindet', async () => {
    const run = (id: string, state: string) => ({ id, label: id, where: 'offline', state, phase: '', done: 0, total: 0, started: '', verdict: '', report: '', lines: [] });
    const settle = () => act(async () => {
      await vi.advanceTimersByTimeAsync(10_000);
    });
    fake.backend.balanceRuns.mockResolvedValueOnce([run('b-1', 'läuft'), run('b-2', 'wartet')] as never);
    act(() => root.render(<Probe active />));
    await settle();
    expect(state.runs.map((r) => r.id)).toEqual(['b-1', 'b-2']);
    fake.backend.balanceRuns.mockResolvedValueOnce([run('b-1', 'läuft')] as never);
    await act(async () => fake.emit('balance:runs'));
    await settle();
    expect(state.runs.map((r) => r.id)).toEqual(['b-1']);
  });
});
