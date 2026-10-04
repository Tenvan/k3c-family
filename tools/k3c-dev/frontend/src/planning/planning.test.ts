import { describe, expect, it } from 'vitest';
import type { PlanSession, PlanningData } from '../api';
import { EMPTY_FILTER, domains, filterSprints, filterTickets, findSession, ghBadges, groupTickets, isNext, linkedTickets, parseFilter, sortSprints } from './planning';

const s = (nr: string, status: string, agent = 'autonom', titel = ''): PlanSession => ({ nr, typ: 'Umsetzung', agent, status, titel });
const tk = (nr: string, domain: string, prio: string, status = 'offen', sprint = '–') =>
  ({ nr, title: `Ticket ${nr}`, domain, typ: 'Idee', prio, status, sprint, spec: 'Entwurf' });

const DATA: PlanningData = {
  done: 1,
  sprints: [
    { id: 'M8', title: 'Planung über MCP', domain: 'SRV', status: 'aktiv', reife: 'bereit', spec: 'freigegeben', tickets: ['B-210'],
      sessions: [s('M8.1', 'fertig'), s('M8.2', 'offen'), s('M8.3', 'offen', 'Mensch')] },
    { id: 'W1', title: 'Hub-Ausbau', domain: 'SIM', status: 'geplant', reife: 'bereit', spec: 'Entwurf', tickets: [],
      sessions: [s('W1.1', 'offen', 'Mensch')] },
    { id: 'R1', title: 'Regelwerk', domain: 'REG', status: 'erledigt', reife: 'bereit', spec: 'freigegeben', tickets: [], sessions: [] },
  ],
  tickets: [tk('B-210', 'SRV', 'hoch', 'eingeplant', 'M8'), tk('B-011', 'CLI', 'mittel'), tk('B-007', 'SIM', 'hoch'),
    tk('B-012', 'SIM', 'niedrig'), tk('B-099', 'SIM', '?'), tk('B-300', 'SRV', 'niedrig', 'offen', 'M8')],
};

describe('Sortierung', () => {
  it('Worktree zuerst, dann offener Branch, Rest stabil', () => {
    const pr = (state: 'offen' | 'gemergt') => ({ number: 1, title: '', url: '', state, ci: '–', merge: '–' } as const);
    const xs = [DATA.sprints[0], DATA.sprints[1], { ...DATA.sprints[2], worktree: 'sprint/r1' }];
    expect(sortSprints(xs, { W1: pr('offen'), M8: pr('gemergt') }).map((x) => x.id)).toEqual(['R1', 'W1', 'M8']);
    expect(sortSprints(DATA.sprints).map((x) => x.id)).toEqual(['M8', 'W1', 'R1']);
  });
});

describe('Filter', () => {
  it('blendet Erledigte aus, außer mit Schalter oder Schnellfilter', () => {
    expect(filterSprints(DATA, EMPTY_FILTER).map((x) => x.id)).toEqual(['M8', 'W1']);
    expect(filterSprints(DATA, { ...EMPTY_FILTER, done: true })).toHaveLength(3);
    expect(filterSprints(DATA, { ...EMPTY_FILTER, quick: ['erledigt'] }).map((x) => x.id)).toEqual(['R1']);
  });
  it('Suche trifft ID, Titel, Tickets und Sessions; Domäne und Schnellfilter verknüpft', () => {
    expect(filterSprints(DATA, { ...EMPTY_FILTER, q: ' MCP ' }).map((x) => x.id)).toEqual(['M8']);
    expect(filterSprints(DATA, { ...EMPTY_FILTER, q: 'w1.1' }).map((x) => x.id)).toEqual(['W1']);
    expect(filterSprints(DATA, { ...EMPTY_FILTER, quick: ['agent'] }).map((x) => x.id)).toEqual(['M8']);
    expect(filterTickets(DATA, { ...EMPTY_FILTER, doms: ['SIM'], quick: ['hoch'] }).map((t) => t.nr)).toEqual(['B-007']);
    expect(filterTickets(DATA, { ...EMPTY_FILTER, q: 'm8' }).map((t) => t.nr)).toEqual(['B-210', 'B-300']);
    expect(filterTickets(DATA, { ...EMPTY_FILTER, quick: ['ungeplant'] }).map((t) => t.nr)).toEqual(['B-011', 'B-007', 'B-012', 'B-099']);
  });
  it('Domänen ohne Doppel, sortiert', () => {
    expect(domains(DATA)).toEqual(['CLI', 'REG', 'SIM', 'SRV']);
  });
});

describe('Gruppierung und Auswahl', () => {
  it('Backlog je Domäne, darin hoch vor mittel vor niedrig vor unbekannt', () => {
    const g = groupTickets(DATA.tickets);
    expect(g.map(([d]) => d)).toEqual(['CLI', 'SIM', 'SRV']);
    expect(g[1][1].map((t) => t.nr)).toEqual(['B-007', 'B-012', 'B-099']);
  });
  it('nächste Session ist die erste offene autonome', () => {
    expect(DATA.sprints[0].sessions.find(isNext)?.nr).toBe('M8.2');
    expect(DATA.sprints[1].sessions.find(isNext)).toBeUndefined();
  });
  it('Auswahl findet Session und verknüpfte Tickets; verschwundene Auswahl ist leer', () => {
    expect(findSession(DATA, 'M8.2')?.sprint.id).toBe('M8');
    expect([...linkedTickets(DATA, 'M8.2')].sort()).toEqual(['B-210', 'B-300']);
    expect(findSession(DATA, 'X9.9')).toBeNull();
    expect(linkedTickets(DATA, 'X9.9').size).toBe(0);
  });
  it('gemerkter Filter: kaputt oder falsch getypt ergibt die Vorgabe', () => {
    expect(parseFilter('{kaputt')).toEqual(EMPTY_FILTER);
    expect(parseFilter('{"q":1,"doms":["SRV",2],"sel":"M8.2","done":"ja"}')).toEqual({ ...EMPTY_FILTER, doms: ['SRV'], sel: 'M8.2' });
  });
});

describe('GitHub-Stand', () => {
  const pr = { number: 110, title: 'M8', url: 'u' };
  it('offener PR: PR, CI und Merge mit passendem Ton', () => {
    expect(ghBadges({ ...pr, state: 'offen', ci: 'rot', merge: 'Konflikt' })).toEqual([
      { label: '#110 offen', tone: 'info' }, { label: 'CI rot', tone: 'error' }, { label: 'Konflikt', tone: 'error' }]);
    expect(ghBadges({ ...pr, state: 'Entwurf', ci: 'läuft', merge: 'konfliktfrei' }).map((b) => b.tone)).toEqual(['neutral', 'warn', 'ok']);
  });
  it('gemergter PR ohne Checks: nur der PR', () => {
    expect(ghBadges({ ...pr, state: 'gemergt', ci: '–', merge: '–' })).toEqual([{ label: '#110 gemergt', tone: 'ok' }]);
  });
});
