// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { renderToStaticMarkup } from 'react-dom/server';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { WorktreeDiff } from '../api';
import { mockPlanningData } from '../api/mockPlanning';
import { groupProjects, rankedCount } from './projects';
import { ProjectSections } from './ProjectsView';
import { indexChanges } from './wtPills';
import { useWorktreeDiffs, WtContext } from './WorktreePills';

// B-467 (WZ12.2): Neuladen des Workflow-Checks und Pill an der Projektkarte.

const check = vi.fn();
const handlers: Record<string, () => void> = {};
vi.mock('../api', () => ({ backend: { worktreeCheck: () => check(), on: (ev: string, fn: () => void) => { handlers[ev] = fn; return () => {}; } } }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

afterEach(() => { check.mockReset(); for (const k of Object.keys(handlers)) delete handlers[k]; });

type State = ReturnType<typeof useWorktreeDiffs>;
async function mount(): Promise<{ get: () => State; done: () => void }> {
  let state!: State;
  const Probe = () => { state = useWorktreeDiffs(); return null; };
  const host = document.createElement('div');
  const root = createRoot(host);
  await act(async () => { root.render(<Probe />); });
  return { get: () => state, done: () => act(() => root.unmount()) };
}

const diff = (id: string): WorktreeDiff[] => [{ branch: 'b', path: 'p', changes: [{ kind: 'neu', item: 'Ticket', id, text: 't' }] }];

describe('useWorktreeDiffs', () => {
  it('lädt beim Öffnen, bei planning:changed und bei Fokus; ein Fehler liefert keine Pills, nur den Text', async () => {
    check.mockResolvedValueOnce(diff('B-1'));
    const m = await mount();
    expect(m.get().loaded).toBe(true);
    expect(m.get().diffs).toHaveLength(1);
    check.mockRejectedValueOnce(new Error('git: exit 128'));
    await act(async () => { handlers['planning:changed']!(); });
    expect(m.get().diffs).toEqual([]);
    expect(m.get().error).toContain('git: exit 128');
    check.mockResolvedValueOnce(diff('B-2'));
    await act(async () => { window.dispatchEvent(new Event('focus')); });
    expect(m.get().error).toBe('');
    expect(m.get().diffs[0]!.changes[0]!.id).toBe('B-2');
    expect(check).toHaveBeenCalledTimes(3);
    m.done();
    window.dispatchEvent(new Event('focus')); // nach dem Unmount kein weiterer Check
    expect(check).toHaveBeenCalledTimes(3);
  });
});

describe('Pill an der Projektkarte', () => {
  it('ein Projekt mit Änderung im Worktree trägt die Pill mit Tooltip', () => {
    const d = mockPlanningData();
    const id = d.projects![0]!.id;
    const diffs: WorktreeDiff[] = [{ branch: 'sprint/x', path: 'p', changes: [{ kind: 'geändert', item: 'Projekt', id, text: 'Status aktiv → ruht' }] }];
    const html = renderToStaticMarkup(
      <Theme>
        <WtContext.Provider value={indexChanges(diffs)}>
          <ProjectSections groups={groupProjects(d)} count={rankedCount(d.projects)} error={null} onMove={() => {}} data={d} sel="" renderSprint={() => <div />} />
        </WtContext.Provider>
      </Theme>,
    );
    expect(html).toContain('⎇ Worktree');
    expect(html).toContain('sprint/x: Status aktiv → ruht');
  });
});

describe('useWorktreeDiffs: Zustand und Reihenfolge', () => {
  it('loaded ist vor der ersten Antwort falsch; bei überlappenden Reloads zählt nur die jüngste Antwort', async () => {
    let first!: (d: WorktreeDiff[]) => void;
    let second!: (d: WorktreeDiff[]) => void;
    check.mockReturnValueOnce(new Promise<WorktreeDiff[]>((r) => { first = r; }));
    const m = await mount();
    expect(m.get().loaded).toBe(false);
    check.mockReturnValueOnce(new Promise<WorktreeDiff[]>((r) => { second = r; }));
    await act(async () => { handlers['planning:changed']!(); });
    await act(async () => { second(diff('NEU')); });
    await act(async () => { first(diff('ALT')); }); // überholte Antwort
    expect(m.get().loaded).toBe(true);
    expect(m.get().diffs[0]!.changes[0]!.id).toBe('NEU');
    m.done();
  });
});

describe('Fehlerfall der Planungsseite', () => {
  it('ohne Diffs (Fehler des Checks) gibt es keine Pills', () => {
    const d = mockPlanningData();
    const html = renderToStaticMarkup(
      <Theme>
        <WtContext.Provider value={indexChanges([])}>
          <ProjectSections groups={groupProjects(d)} count={rankedCount(d.projects)} error={null} onMove={() => {}} data={d} sel="" renderSprint={() => <div />} />
        </WtContext.Provider>
      </Theme>,
    );
    expect(html).not.toContain('⎇');
  });
});
