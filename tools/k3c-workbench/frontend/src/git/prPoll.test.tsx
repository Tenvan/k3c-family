// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { GitPR, GitPRList } from '../api';
import { PR_POLL_BUSY_MS, PR_POLL_MS, prPollMs, usePrPoll } from './prPoll';

vi.mock('../api', () => ({ backend: {} }));

// B-580 Rev. 4: Die PR-Karte folgt GitHub im Takt, solange der Reiter offen ist.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const pr = (p: Partial<GitPR>): GitPR => ({ number: 1, title: 't', url: '', head: 'h', author: '', updated: '', state: 'offen', ci: 'grün', merge: 'konfliktfrei', ...p });

describe('Takt', () => {
  it('ruhig 30 s, schneller bei laufender CI oder ungerechneter Mergebarkeit', () => {
    expect(prPollMs({ prs: [pr({})] })).toBe(PR_POLL_MS);
    expect(prPollMs({ prs: [] })).toBe(PR_POLL_MS);
    expect(prPollMs({ prs: [pr({}), pr({ ci: 'läuft' })] })).toBe(PR_POLL_BUSY_MS);
    expect(prPollMs({ prs: [pr({ merge: 'unbekannt' })] })).toBe(PR_POLL_BUSY_MS);
  });
});

describe('usePrPoll', () => {
  let root: Root;
  beforeEach(() => vi.useFakeTimers());
  afterEach(() => {
    act(() => root.unmount());
    vi.useRealTimers();
  });

  function Probe({ active, list, refresh }: { active: boolean; list: GitPRList | null; refresh: () => Promise<unknown> }) {
    usePrPoll(active, list, refresh);
    return null;
  }

  const render = (active: boolean, list: GitPRList | null, refresh: () => Promise<unknown>) =>
    act(() => root.render(<Probe active={active} list={list} refresh={refresh} />));

  it('fragt nur bei offenem Reiter und geladener Liste, jede neue Liste stellt den nächsten Abruf', () => {
    root = createRoot(document.createElement('div'));
    const refresh = vi.fn(() => Promise.resolve());
    const list: GitPRList = { prs: [pr({ ci: 'läuft' })] };
    render(true, null, refresh);
    vi.advanceTimersByTime(PR_POLL_MS * 2);
    expect(refresh).not.toHaveBeenCalled();
    render(false, list, refresh);
    vi.advanceTimersByTime(PR_POLL_MS * 2);
    expect(refresh).not.toHaveBeenCalled();
    render(true, list, refresh);
    vi.advanceTimersByTime(PR_POLL_BUSY_MS);
    expect(refresh).toHaveBeenCalledTimes(1);
    render(true, { prs: [pr({})] }, refresh); // CI fertig: ruhiger Takt
    vi.advanceTimersByTime(PR_POLL_BUSY_MS);
    expect(refresh).toHaveBeenCalledTimes(1);
    vi.advanceTimersByTime(PR_POLL_MS - PR_POLL_BUSY_MS);
    expect(refresh).toHaveBeenCalledTimes(2);
    render(false, { prs: [] }, refresh); // Reiter zu: kein Abruf mehr
    vi.advanceTimersByTime(PR_POLL_MS * 3);
    expect(refresh).toHaveBeenCalledTimes(2);
  });
});
