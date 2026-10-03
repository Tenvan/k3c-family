import { describe, expect, it } from 'vitest';
import type { PlanSprint, PlanTicket } from '../api';
import { filterTickets, groupByDomain, nextSession, progress } from './planning';

const sess = (nr: string, status: string, agent = 'autonom') => ({ nr, status, agent, typ: '', titel: '' });
const sprint = { id: 'SP', sessions: [sess('1', 'fertig'), sess('2', 'offen', 'Mensch'), sess('3', 'offen')] } as PlanSprint;
const tk = (nr: string, domain: string, prio: string, status = 'offen'): PlanTicket =>
  ({ nr, title: `Titel ${nr}`, domain, prio, status, typ: 'Idee', sprint: '–', spec: 'Entwurf' });

describe('planning', () => {
  it('zählt Fortschritt und findet die nächste Agent-Session', () => {
    expect(progress(sprint)).toEqual({ done: 1, total: 3 });
    expect(nextSession(sprint)?.nr).toBe('3');
  });

  it('filtert Tickets nach Status, Domäne und Text', () => {
    const list = [tk('B-1', 'SIM', 'hoch'), tk('B-2', 'CLI', 'mittel', 'eingeplant')];
    expect(filterTickets(list, { query: '', status: 'eingeplant', domain: 'alle' }).map((t) => t.nr)).toEqual(['B-2']);
    expect(filterTickets(list, { query: 'b-1', status: 'alle', domain: 'SIM' })).toHaveLength(1);
  });

  it('gruppiert nach Domäne, Prio zuerst', () => {
    const g = groupByDomain([tk('B-3', 'SIM', 'niedrig'), tk('B-2', 'SIM', 'hoch'), tk('B-1', 'CLI', 'mittel')]);
    expect(g.map((x) => x.domain)).toEqual(['CLI', 'SIM']);
    expect(g[1].tickets.map((t) => t.nr)).toEqual(['B-2', 'B-3']);
  });
});
