// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { describe, expect, it, vi } from 'vitest';
import { WorktreeBar } from './WorktreeBar';

const worktreeCheck = vi.fn();
vi.mock('../api', () => ({ backend: { worktreeCheck: () => worktreeCheck() } }));

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

async function clickCheck(): Promise<string> {
  const host = document.createElement('div');
  document.body.appendChild(host);
  const root = createRoot(host);
  act(() => root.render(<WorktreeBar />));
  const btn = [...host.querySelectorAll('button')].find((b) => b.textContent?.includes('Workflow-Check')) as HTMLElement;
  await act(async () => { btn.click(); });
  const text = host.textContent ?? '';
  act(() => root.unmount());
  host.remove();
  return text;
}

describe('WorktreeBar: Fehler sind sichtbar (Review M2)', () => {
  it('ein Fehler des Backends erscheint als Meldung, nicht als „keine weiteren Worktrees“', async () => {
    worktreeCheck.mockRejectedValueOnce(new Error('git worktree list: exit status 128'));
    const text = await clickCheck();
    expect(text).toContain('git worktree list: exit status 128');
    expect(text).not.toContain('keine weiteren Worktrees');
  });

  it('eine leere Liste heißt „keine weiteren Worktrees“', async () => {
    worktreeCheck.mockResolvedValueOnce([]);
    expect(await clickCheck()).toContain('keine weiteren Worktrees');
  });
});
