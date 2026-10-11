import { describe, expect, it } from 'vitest';
import { mockPlanningData } from '../api/mockPlanning';
import { EMPTY_FILTER, SPLIT_DEFAULT, filterProjects, filterSprints, filterTickets, parseSplit } from './planning';

const d = mockPlanningData();
const ids = (xs: { id: string }[]) => xs.map((x) => x.id);

describe('getrennte Filter (B-401)', () => {
  const projects = ids(filterProjects(d, EMPTY_FILTER));
  const sprints = ids(filterSprints(d, EMPTY_FILTER));
  const tickets = filterTickets(d, EMPTY_FILTER).map((t) => t.nr);

  it('AC-01: der Backlog-Filter ändert Backlog und Sprint-Liste, nicht die Projekte', () => {
    const f = { ...EMPTY_FILTER, q: 'zzz-kein-treffer', prios: ['hoch'], envs: ['live'], projs: ['GRA'] };
    expect(filterTickets(d, f)).toHaveLength(0);
    expect(filterSprints(d, f)).toHaveLength(0);
    expect(ids(filterProjects(d, f))).toEqual(projects);
  });

  it('AC-02: der Projekt-Filter ändert nur die Projekte, nicht Backlog und Sprint-Liste', () => {
    const f = { ...EMPTY_FILTER, pstatus: ['ruht'], doms: ['SIM'] };
    expect(ids(filterProjects(d, f))).not.toEqual(projects);
    expect(ids(filterSprints(d, f))).toEqual(sprints);
    expect(filterTickets(d, f).map((t) => t.nr)).toEqual(tickets);
  });

  it('der Projekt-Filter schränkt nach Status und Domäne der Projekte ein', () => {
    const byStatus = filterProjects(d, { ...EMPTY_FILTER, pstatus: ['ruht'] });
    expect(byStatus.length).toBeGreaterThan(0);
    expect(byStatus.every((p) => p.status === 'ruht')).toBe(true);
    const byDom = filterProjects(d, { ...EMPTY_FILTER, doms: ['SIM'] });
    for (const p of byDom) {
      const inDom = d.sprints.some((s) => p.sprints.includes(s.id) && s.domain === 'SIM') || d.tickets.some((t) => t.project === p.id && t.domain === 'SIM');
      expect(inDom).toBe(true);
    }
  });

  it('der Backlog-Filter wirkt nach Prio, Umgebung und Projekt', () => {
    const hoch = filterTickets(d, { ...EMPTY_FILTER, prios: ['hoch'] });
    expect(hoch.length).toBeGreaterThan(0);
    expect(hoch.every((t) => t.prio === 'hoch')).toBe(true);
    expect(filterTickets(d, { ...EMPTY_FILTER, envs: ['live'] }).every((t) => t.env === 'live')).toBe(true);
    expect(filterTickets(d, { ...EMPTY_FILTER, projs: ['BET'] }).every((t) => t.project === 'BET')).toBe(true);
  });
});

describe('Splitter-Anteil (B-400)', () => {
  it('AC-02: gemerkter Anteil bleibt zwischen 20 und 80 %, sonst Vorgabe', () => {
    expect(parseSplit('30')).toBe(30);
    expect(parseSplit('5')).toBe(20);
    expect(parseSplit('95')).toBe(80);
    for (const bad of ['', 'abc', 'NaN', 'null']) expect(parseSplit(bad)).toBe(SPLIT_DEFAULT);
  });
});
