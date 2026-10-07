import { describe, expect, it } from 'vitest';
import type { PlanSession, PlanSprint, PlanTicket } from '../api';
import { backlogPrompt, openSessions, promptSession, promptSessions, promptSprint } from './prompts';

const s = (nr: string, status: string, agent = 'autonom', env = 'offline'): PlanSession =>
  ({ nr, typ: 'Umsetzung', agent, env, status, titel: `Titel ${nr}`, text: 'Geheimer Inhalt' });
const sp = (id: string, sessions: PlanSession[], worktree?: string): PlanSprint =>
  ({ id, title: `Titel ${id}`, domain: 'SRV', status: 'aktiv', reife: 'bereit', spec: 'freigegeben', tickets: [], sessions, worktree });
const tk = (nr: string): PlanTicket => ({ nr, title: `Titel ${nr}`, domain: 'SRV', typ: 'Idee', prio: 'hoch', env: 'offline', status: 'offen', sprint: '–', spec: 'Entwurf' });

const M8 = sp('M8', [s('M8.1', 'fertig'), s('M8.2', 'offen'), s('M8.3', 'offen', 'Mensch'), s('M8.4', 'entwurf', ''), s('M8.5', 'offen', 'autonom', 'live')], 'sprint/m8');
const W1 = sp('W1', [s('W1.1', 'in Arbeit')]);

describe('Prompts nennen nur IDs und Anweisung', () => {
  it('Sprint und Session ohne Titel und Inhalt, Worktree genannt', () => {
    expect(promptSprint(M8)).toContain('Sprint M8 (Worktree sprint/m8)');
    const p = promptSession(W1, W1.sessions[0]);
    expect(p).toContain('Sprint W1, Session W1.1');
    expect(p + promptSprint(M8)).not.toMatch(/Titel|Geheimer/);
  });

  it('markierbar sind nur offene Sessions mit Datei, gruppiert je Sprint', () => {
    const items = [...openSessions(M8), ...openSessions(W1)];
    expect(items.map((it) => it.session.nr)).toEqual(['M8.2', 'M8.3', 'M8.5', 'W1.1']);
    expect(promptSessions(items)).toContain('Sprint M8 (Worktree sprint/m8):\n- M8.2\n- M8.3\n- M8.5\nSprint W1:\n- W1.1');
  });

  it('Backlog-Prompt: Sprint oder neuer Sprint, Singular und Plural', () => {
    const one = backlogPrompt([tk('B-007')], { kind: 'sprint', id: 'M8' });
    expect(one).toContain('Nimm das Backlog-Ticket B-007 in den Sprint M8 auf.');
    expect(one).toContain('Sprint `M8`');
    expect(one).not.toContain('Titel');
    const many = backlogPrompt([tk('B-007'), tk('B-011')], { kind: 'new' });
    expect(many).toContain('Forme die Backlog-Tickets B-007, B-011 zu einem neuen Sprint.');
    expect(many).toContain('in jedem Ticket');
    expect(many).toContain('`plan_create`');
  });
});
