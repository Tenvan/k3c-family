import { describe, expect, it } from 'vitest';
import type { PlanSession, PlanSprint, PlanTicket } from '../api';
import { backlogPrompt, openSessions, promptProject, promptSession, promptSessions, promptSprint } from './prompts';

const s = (nr: string, status: string, agent = 'autonom', env = 'offline'): PlanSession =>
  ({ nr, typ: 'Umsetzung', agent, env, status, titel: `Titel ${nr}`, text: 'Geheimer Inhalt' });
const sp = (id: string, sessions: PlanSession[], worktree?: string): PlanSprint =>
  ({ id, title: `Titel ${id}`, domain: 'SRV', status: 'aktiv', reife: 'bereit', spec: 'freigegeben', tickets: [], sessions, worktree });
const tk = (nr: string): PlanTicket => ({ nr, title: `Titel ${nr}`, domain: 'SRV', typ: 'Idee', prio: 'hoch', env: 'offline', status: 'offen', sprint: '–', spec: 'Entwurf' });

const M8 = sp('M8', [s('M8.1', 'fertig'), s('M8.2', 'offen'), s('M8.3', 'offen', 'Mensch'), s('M8.4', 'entwurf', ''), s('M8.5', 'offen', 'autonom', 'live')], 'sprint/m8');
const W1 = sp('W1', [s('W1.1', 'in Arbeit')]);

describe('Prompts in Kurzform ohne Anweisungen (B-639)', () => {
  it('Sprint, Projekt und Session: eine Zeile mit ID, Worktree genannt', () => {
    expect(promptSprint(M8)).toBe('Sprint M8 (Worktree sprint/m8)');
    expect(promptProject({ id: 'BAS', title: 'B', status: 'aktiv', rang: '1', sprints: [] })).toBe('Projekt BAS');
    expect(promptSession(W1, W1.sessions[0])).toBe('Session W1.1');
    for (const p of [promptSprint(M8), promptSession(W1, W1.sessions[0])]) {
      expect(p).not.toMatch(/Titel|Geheimer|plan_|\n/);
    }
  });

  it('markierbar sind nur offene Sessions mit Datei', () => {
    const items = [...openSessions(M8), ...openSessions(W1)];
    expect(items.map((it) => it.session.nr)).toEqual(['M8.2', 'M8.3', 'M8.5', 'W1.1']);
    expect(promptSessions(openSessions(W1))).toBe('Session W1.1');
  });

  it('markierte Sessions nach Rang der Projekte, ohne Projekt zuletzt (AC-09)', () => {
    const projects = [{ id: 'BET', title: 'B', status: 'aktiv', rang: '2', sprints: ['W1'] },
      { id: 'SKL', title: 'S', status: 'aktiv', rang: '1', sprints: ['M9', 'M8'] }];
    const M9 = sp('M9', [s('M9.1', 'offen')]);
    const X2 = sp('X2', [s('X2.1', 'offen')]);
    const items = [...openSessions(X2), ...openSessions(W1), ...openSessions(M8), ...openSessions(M9)];
    expect(promptSessions(items, projects)).toBe(
      'Sessions M9.1, M8.2 (Worktree sprint/m8), M8.3 (Worktree sprint/m8), M8.5 (Worktree sprint/m8), W1.1, X2.1');
  });

  it('Backlog-Prompt: Sprint oder neuer Sprint, Singular und Plural', () => {
    expect(backlogPrompt([tk('B-007')], { kind: 'sprint', id: 'M8' })).toBe('Ticket B-007 in Sprint M8');
    expect(backlogPrompt([tk('B-007'), tk('B-011')], { kind: 'new' })).toBe('Neuer Sprint aus Tickets B-007, B-011');
  });
});
