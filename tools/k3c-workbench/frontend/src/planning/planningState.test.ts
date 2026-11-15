import { describe, expect, it } from 'vitest';
import { mockPlanningData } from '../api/mockPlanning';
import type { PlanItem } from '../api';
import { applyItems, collect, FIRST, MAX_IDS, NOTHING, parseTicket } from './planningState';

const text = (status: string) => `# B-1 · Titel\n\n- **Prio:** hoch\n- **Status:** ${status}\n- **Sprint:** SP11 · x\n- **Projekt:** –\n`;
const it1 = (id: string, kind: PlanItem['kind'], t: string, error?: string): PlanItem => ({ id, kind, path: '', text: t, error });

describe('collect', () => {
  it('sammelt IDs; ohne IDs oder zu viele heißt: alles neu', () => {
    expect([...collect(NOTHING, { ids: ['a'] }).ids]).toEqual(['a']);
    expect([...collect(collect(NOTHING, { ids: ['a'] }), { ids: ['b', 'a'] }).ids]).toEqual(['a', 'b']);
    expect(collect(NOTHING, null)).toBe(FIRST);
    expect(collect(NOTHING, {})).toBe(FIRST);
    expect(collect(FIRST, { ids: ['a'] })).toBe(FIRST);
    expect(collect(NOTHING, { ids: Array.from({ length: MAX_IDS + 1 }, (_, i) => `B-${i}`) })).toBe(FIRST);
  });
});

describe('parseTicket', () => {
  it('liest Titel und Felder', () => {
    expect(parseTicket(text('offen'))).toMatchObject({ title: 'Titel', Prio: 'hoch', Status: 'offen' });
  });
});

describe('applyItems', () => {
  const d = mockPlanningData();
  const id = d.tickets[0]!.nr;
  it('ersetzt ein Ticket aus seiner Datei, ohne die Liste neu zu laden', () => {
    const r = applyItems(d, [it1(id, 'ticket', text('erledigt'))]);
    const t = r.data.tickets.find((x) => x.nr === id)!;
    expect([t.title, t.status, t.prio, t.sprint, t.project]).toEqual(['Titel', 'erledigt', 'hoch', 'SP11', '–']);
    expect(r.reload).toBe(false);
    expect(r.data.tickets).toHaveLength(d.tickets.length);
  });
  it('entfernt ein verschwundenes Ticket', () => {
    const r = applyItems(d, [it1(id, 'ticket', '', 'unbekannt')]);
    expect(r.data.tickets.some((x) => x.nr === id)).toBe(false);
    expect(r.reload).toBe(false);
  });
  it('merkt Session-Texte; Sprint, Projekt und neue Einträge laden die Liste neu', () => {
    const nr = d.sprints[0]!.sessions[0]!.nr;
    expect(applyItems(d, [it1(nr, 'session', 'T')])).toMatchObject({ texts: { [nr]: 'T' }, reload: false });
    expect(applyItems(d, [it1(d.sprints[0]!.id, 'sprint', '#')]).reload).toBe(true);
    expect(applyItems(d, [it1('B-99999', 'ticket', text('offen'))]).reload).toBe(true);
    expect(applyItems(d, [it1('ZZ9.1', 'session', 'x')]).reload).toBe(true);
    expect(applyItems(d, [it1('B-99999', 'ticket', '', 'unbekannt')]).reload).toBe(false); // archiviert, war nie in der Liste
  });
});
