import { Theme } from '@radix-ui/themes';
import type { ReactElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import type { PlanningData, PlanSprint, WorktreeDiff } from '../api';
import { mockPlanningData } from '../api/mockPlanning';
import { BacklogView } from './Backlog';
import { PlanDetail } from './Detail';
import { ProjectCard } from './ProjectCard';
import { groupProjects, rankedCount } from './projects';
import { Reader } from './Reader';
import { withWorktreeEntries } from './wtPills';

const planningGet = vi.fn(() => Promise.resolve(''));
vi.mock('../api', () => ({ backend: { planningGet: () => planningGet() } }));

// B-483 (WZ20.2): Einträge nur im Worktree tragen `wt` und sind nur lesend.

const render = (el: ReactElement) => renderToStaticMarkup(<Theme>{el}</Theme>);
const count = (html: string, label: string) => html.split(`aria-label="${label}`).length - 1;
const base = mockPlanningData();

describe('withWorktreeEntries markiert Worktree-Einträge (AC-01)', () => {
  const diffs: WorktreeDiff[] = [
    { branch: 'sprint/a', path: 'p1', changes: [
      { kind: 'neu', item: 'Projekt', id: 'NEU', text: 'Neues Projekt' },
      { kind: 'neu', item: 'Sprint', id: 'X9', text: 'Neuer Sprint', project: 'NEU' },
      { kind: 'neu', item: 'Session', id: 'X9.1', text: 'Eins' },
      { kind: 'neu', item: 'Ticket', id: 'B-900', text: 'Neues Ticket' },
    ] },
    { branch: 'sprint/b', path: 'p2', changes: [{ kind: 'neu', item: 'Ticket', id: 'B-900', text: 'Neues Ticket' }] },
  ];
  const out = withWorktreeEntries(base, diffs);

  it('alle vier Typen tragen den Branch, bei mehreren Worktrees den ersten', () => {
    expect(out.projects?.find((p) => p.id === 'NEU')?.wt).toBe('sprint/a');
    const sprint = out.sprints.find((s) => s.id === 'X9');
    expect(sprint?.wt).toBe('sprint/a');
    expect(sprint?.sessions[0]?.wt).toBe('sprint/a');
    expect(out.tickets.find((t) => t.nr === 'B-900')?.wt).toBe('sprint/a');
  });

  it('Wurzel-Einträge bleiben ohne Marke', () => {
    const root = [...(out.projects ?? []), ...out.sprints, ...out.sprints.flatMap((s) => s.sessions), ...out.tickets]
      .filter((x) => !['NEU', 'X9', 'X9.1', 'B-900'].includes('id' in x ? x.id : x.nr));
    expect(root.length).toBeGreaterThan(0);
    expect(root.every((x) => x.wt === undefined)).toBe(true);
  });
});

/** Mock-Daten, in denen das erste Ticket und eine offene Session nur im Worktree liegen. */
function marked(): { data: PlanningData; ticket: string; session: string } {
  const sprint = base.sprints.find((s) => s.sessions.some((x) => x.status === 'offen'))!;
  const session = sprint.sessions.find((x) => x.status === 'offen')!;
  const sprints: PlanSprint[] = base.sprints.map((s) => s !== sprint ? s
    : { ...s, sessions: s.sessions.map((x) => (x === session ? { ...x, wt: 'sprint/x' } : x)) });
  const tickets = base.tickets.map((t, i) => (i === 0 ? { ...t, wt: 'sprint/x' } : t));
  return { data: { ...base, sprints, tickets }, ticket: base.tickets[0]!.nr, session: session.nr };
}

describe('Menüs nur für Wurzel-Einträge (AC-02)', () => {
  const { data, ticket, session } = marked();
  const detail = (sel: string, d = data) => render(<PlanDetail data={d} sel={sel} onSelect={() => {}} onClose={() => {}} target={{ kind: 'new' }} />);

  it('Backlog: markiertes Ticket ohne Prio-, Projekt- und Umgebungs-Menü, die anderen mit', () => {
    const html = render(<BacklogView data={data} tickets={data.tickets} sel="" onPick={() => {}} target={{ kind: 'new' }} goal="__new__" choose={() => {}} />);
    for (const label of ['Prio: ', 'Umgebung: ', 'Projekt: ']) expect(count(html, label)).toBe(data.tickets.length - 1);
  });

  it('TicketDetail: markiert ohne Prio- und Umgebungs-Menü, Wurzel-Ticket mit', () => {
    expect(count(detail(ticket), 'Prio: ') + count(detail(ticket), 'Umgebung: ')).toBe(0);
    expect(count(detail(ticket, base), 'Prio: ')).toBe(1);
    expect(count(detail(ticket, base), 'Umgebung: ')).toBe(1);
  });

  it('SessionDetail: markierte offene Session ohne Agent- und Umgebungs-Menü, Wurzel-Session mit', () => {
    expect(count(detail(session), 'Agent: ') + count(detail(session), 'Umgebung: ')).toBe(0);
    expect(count(detail(session, base), 'Agent: ')).toBe(1);
    expect(count(detail(session, base), 'Umgebung: ')).toBe(1);
  });
});

describe('Rang-Knöpfe nur für Wurzel-Projekte (AC-03)', () => {
  const g = groupProjects(base);
  const view = g.active[0]!;
  const card = (wt?: string) => render(<ProjectCard view={{ ...view, project: { ...view.project, wt } }} count={rankedCount(base.projects)}
    data={base} sel="" renderSprint={(s) => <div key={s.id} />} />);

  it('markiertes Projekt ohne hoch/runter, Wurzel-Projekt mit', () => {
    expect(card('sprint/x')).not.toMatch(/>(hoch|runter)</);
    expect(card()).toMatch(/>hoch</);
  });
});

describe('Reader für Worktree-Einträge (AC-04)', () => {
  it('zeigt „nur im Worktree <branch>“, ohne Lese-Knopf und ohne planningGet', () => {
    const html = render(<Reader id="B-900" title="B-900 · Neu" prompt="" pill={null} wt="sprint/x" />);
    expect(html).toContain('nur im Worktree sprint/x');
    expect(html).not.toContain('Volltext lesen');
    expect(planningGet).not.toHaveBeenCalled();
    expect(render(<Reader id="B-1" title="B-1" prompt="" pill={null} />)).toContain('Volltext lesen');
  });
});
