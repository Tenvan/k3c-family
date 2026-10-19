import { Theme } from '@radix-ui/themes';
import type { ReactElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import type { PlanningData, WorktreeDiff } from '../api';
import { BacklogView } from './Backlog';
import { SessionRow, SprintCard } from './SprintCard';
import { indexChanges, marksFor, pillLabel, pillTitle, withWorktreeEntries } from './wtPills';
import { WtContext, WtPills } from './WorktreePills';

vi.mock('../api', () => ({ backend: {} }));

// B-467 (WZ12.2): Worktree-Pills am Eintrag statt der Leiste `WorktreeBar`.

const diffs: WorktreeDiff[] = [
  {
    branch: 'sprint/wz6', path: 'C:/wt/wz6',
    changes: [
      { kind: 'geändert', item: 'Session', id: 'WZ6.2', text: 'Status offen → in Arbeit' },
      { kind: 'neu', item: 'Ticket', id: 'B-900', text: 'Neues Ticket' },
      { kind: 'neu', item: 'Session', id: 'WZ6.9', text: 'Neue Session' },
      { kind: 'neu', item: 'Sprint', id: 'XY1', text: 'Neuer Sprint' },
      { kind: 'neu', item: 'Projekt', id: 'NEU', text: 'Neues Projekt' },
      { kind: 'geändert', item: 'Projekt', id: 'WZG', text: 'Status aktiv → ruht' },
    ],
  },
  { branch: 'claude/zwei', path: 'C:/wt/zwei', changes: [{ kind: 'geändert', item: 'Session', id: 'WZ6.2', text: 'Status offen → fertig' }] },
  { branch: 'claude/kaputt', path: 'C:/wt/kaputt', changes: [{ kind: 'geändert', item: 'Session', id: 'WZ6.2', text: 'darf nicht zählen' }], error: 'git: exit 128' },
];

const data = (): PlanningData => ({
  projects: [{ id: 'WZG', title: 'Werkzeuge', status: 'aktiv', rang: '1', sprints: ['WZ6'] }],
  sprints: [{ id: 'WZ6', title: 'Studio', domain: 'DEV', status: 'aktiv', reife: 'bereit', spec: 'freigegeben', tickets: [], sessions: [{ nr: 'WZ6.2', typ: 'Umsetzung', agent: 'autonom', status: 'offen', titel: 'Zwei' }] }],
  tickets: [{ nr: 'B-1', title: 'Alt', domain: 'DEV', typ: 'Feature', prio: 'mittel', env: 'offline', status: 'offen', sprint: '–', spec: '' }],
  done: 0,
});

const render = (el: ReactElement) => renderToStaticMarkup(<Theme>{el}</Theme>);

describe('Zuordnung Änderung → Eintrag (B-467/AC-02, AC-03)', () => {
  const index = indexChanges(diffs);

  it('Projekt, Sprint, Session und Ticket bekommen ihre Änderungen; ein Worktree mit Fehler trägt nichts bei', () => {
    expect(marksFor(index, 'Session', 'WZ6.2').map((m) => m.branch)).toEqual(['sprint/wz6', 'claude/zwei']);
    expect(marksFor(index, 'Projekt', 'WZG')).toHaveLength(1);
    expect(marksFor(index, 'Ticket', 'B-900')).toHaveLength(1);
    expect(marksFor(index, 'Sprint', 'XY1')).toHaveLength(1);
    expect(marksFor(index, 'Session', 'WZ9.9')).toEqual([]);
  });

  it('Tooltip nennt Branch und Änderung, bei mehreren Worktrees je eine Zeile', () => {
    expect(pillTitle(marksFor(index, 'Session', 'WZ6.2'))).toBe('sprint/wz6: Status offen → in Arbeit\nclaude/zwei: Status offen → fertig');
    expect(pillTitle(marksFor(index, 'Ticket', 'B-900'))).toBe('sprint/wz6: neu');
  });

  it('Beschriftung: „neu“ nur für Einträge im Worktree, sonst „Worktree“ bzw. die Zahl', () => {
    expect(pillLabel(marksFor(index, 'Ticket', 'B-900'))).toBe('neu');
    expect(pillLabel(marksFor(index, 'Projekt', 'WZG'))).toBe('Worktree');
    expect(pillLabel(marksFor(index, 'Session', 'WZ6.2'))).toBe('2 Worktrees');
  });
});

describe('Pill am Eintrag (B-467/AC-03)', () => {
  it('die Session-Zeile trägt die Pill mit Tooltip, ohne Änderung keine', () => {
    const d = data();
    const x = d.sprints[0]!.sessions[0]!;
    const row = (nr: string) => render(
      <WtContext.Provider value={indexChanges(diffs)}>
        <SessionRow sprint={d.sprints[0]!} x={{ ...x, nr }} onSelect={() => {}} checked={false} onCheck={() => {}} />
      </WtContext.Provider>,
    );
    const html = row('WZ6.2');
    expect(html).toContain('⎇ 2 Worktrees');
    expect(html).toContain('sprint/wz6: Status offen → in Arbeit');
    expect(row('WZ6.7')).not.toContain('⎇');
  });

  it('Sprint-Karte und Ticket-Zeile tragen ihre Pill, die Worktree-Leiste gibt es nicht mehr (AC-02)', () => {
    const d = withWorktreeEntries(data(), diffs);
    const wrap = (el: ReactElement) => render(<WtContext.Provider value={indexChanges(diffs)}>{el}</WtContext.Provider>);
    const sprint = wrap(<SprintCard sprint={d.sprints[1]!} sel="" onSelect={() => {}} checked={new Set()} onCheck={() => {}} />);
    expect(sprint).toContain('⎇ neu');
    expect(sprint).toContain('sprint/wz6: neu');
    const list = wrap(<BacklogView data={d} tickets={d.tickets} sel="" onPick={() => {}} target={{ kind: 'new' }} goal="" choose={() => {}} />);
    expect(list).toContain('⎇ neu');
    expect(list).not.toContain('pl-wtbar');
    expect(list).not.toContain('Workflow-Check');
  });

  it('ohne Provider und ohne Änderung entsteht nichts', () => {
    expect(render(<WtPills item="Ticket" id="B-1" />)).not.toContain('⎇');
  });
});

describe('Einträge nur im Worktree (B-467/AC-04)', () => {
  it('Ticket, Session unter ihrem Sprint, Sprint und Projekt stehen in den normalen Listen, die Wurzel-Daten bleiben unverändert', () => {
    const root = data();
    const view = withWorktreeEntries(root, diffs);
    expect(view.tickets.map((t) => t.nr)).toEqual(['B-1', 'B-900']);
    expect(view.sprints.find((s) => s.id === 'WZ6')!.sessions.map((s) => s.nr)).toEqual(['WZ6.2', 'WZ6.9']);
    expect(view.sprints.map((s) => s.id)).toEqual(['WZ6', 'XY1']);
    expect(view.projects!.map((p) => p.id)).toEqual(['WZG', 'NEU']);
    expect(root.tickets).toHaveLength(1);
    expect(root.sprints[0]!.sessions).toHaveLength(1);
  });

  it('ohne neue Einträge, bei Fehlern und bei schon vorhandenen Einträgen bleibt alles gleich', () => {
    const root = data();
    expect(withWorktreeEntries(root, [])).toBe(root);
    expect(withWorktreeEntries(root, [diffs[2]!])).toBe(root);
    const dup: WorktreeDiff[] = [{ branch: 'b', path: 'p', changes: [{ kind: 'neu', item: 'Ticket', id: 'B-1', text: 'schon da' }] }];
    expect(withWorktreeEntries(root, dup).tickets).toHaveLength(1);
  });
});

describe('neue Einträge unter ihrem Projekt', () => {
  it('Sprint kommt in die Sprint-Liste seines Projekts, Ticket trägt Projekt und Sprint', async () => {
    const { withWorktreeEntries } = await import('./wtPills');
    const data: PlanningData = { projects: [{ id: 'WZG', title: 'Werkzeuge', status: 'aktiv', rang: '1', sprints: [] }], sprints: [], tickets: [], done: 0 };
    const diffs: WorktreeDiff[] = [{ branch: 'b', path: 'p', changes: [
      { kind: 'neu', item: 'Sprint', id: 'WZ20', text: 'Dummy', project: 'WZG' },
      { kind: 'neu', item: 'Ticket', id: 'B-9', text: 'Neu', project: 'WZG', sprint: 'WZ20' },
    ] }];
    const out = withWorktreeEntries(data, diffs);
    expect(out.projects?.[0]?.sprints).toEqual(['WZ20']);
    expect(out.sprints[0]?.project).toBe('WZG');
    expect(out.tickets[0]).toMatchObject({ project: 'WZG', sprint: 'WZ20' });
  });
});
