// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { STAMP_POLL_MS, useRepoStamp } from './repoStamp';

// B-580 Rev. 5: Ändert sich das Repo (auch von außen), lädt die Seite die PRs neu und meldet „Repo geändert“.

const stamp = { value: 'a' };
vi.mock('../api', () => ({ backend: { gitStamp: () => Promise.resolve(stamp.value) } }));

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let state: ReturnType<typeof useRepoStamp>;
function Probe({ active, onChange }: { active: boolean; onChange: () => void }) {
  state = useRepoStamp(active, onChange);
  return null;
}

describe('useRepoStamp', () => {
  let root: Root;
  beforeEach(() => {
    vi.useFakeTimers();
    stamp.value = 'a';
    root = createRoot(document.createElement('div'));
  });
  afterEach(() => {
    act(() => root.unmount());
    vi.useRealTimers();
  });

  const tick = () => act(async () => {
    await vi.advanceTimersByTimeAsync(STAMP_POLL_MS);
  });

  it('meldet eine Änderung erst nach dem ersten Laden und nur, solange der Reiter offen ist', async () => {
    const onChange = vi.fn();
    act(() => root.render(<Probe active onChange={onChange} />));
    stamp.value = 'b';
    await tick();
    expect(onChange).not.toHaveBeenCalled(); // noch nie geladen: kein Vergleichsstand

    await act(async () => (await state.mark())());
    await tick();
    expect(onChange).not.toHaveBeenCalled();
    expect(state.stale).toBe(false);

    stamp.value = 'c'; // Fetch im Terminal
    await tick();
    expect(onChange).toHaveBeenCalledTimes(1);
    expect(state.stale).toBe(true);
    await tick();
    expect(onChange).toHaveBeenCalledTimes(1); // gleicher Stand: kein zweites Mal

    await act(async () => (await state.mark())()); // „Neu laden“
    expect(state.stale).toBe(false);

    act(() => root.render(<Probe active={false} onChange={onChange} />));
    stamp.value = 'd';
    await tick();
    expect(onChange).toHaveBeenCalledTimes(1);
  });
});
