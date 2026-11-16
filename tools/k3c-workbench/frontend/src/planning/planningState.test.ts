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
    const same = { ...d, tickets: d.tickets.map((x) => (x.nr === id ? { ...x, sprint: 'SP11' } : x)) }; // gleicher Sprint: kein Neuladen
    const r = applyItems(same, [it1(id, 'ticket', text('erledigt'))]);
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

describe('applyItems: veraltete Listen', () => {
  it('ein Sprintwechsel des Tickets lädt die Liste neu (PlanSprint.tickets)', () => {
    const d = mockPlanningData();
    const t = d.tickets.find((x) => x.sprint && x.sprint !== 'ZZ99')!;
    const text = `# ${t.nr} · ${t.title}\n\n- **Sprint:** ZZ99\n`;
    expect(applyItems(d, [{ id: t.nr, kind: 'ticket', path: `docs/backlog/${t.nr}-x.md`, text }]).reload).toBe(true);
  });

  it('geänderte Kopffelder einer Session (Umgebung, Domäne, Abhängigkeiten) laden die Liste neu', () => {
    const d = mockPlanningData();
    const x = d.sprints[0]!.sessions[0]!;
    const same = `# ${x.nr}\n\n- **Umgebung:** ${x.env ?? ''}\n- **Domäne:** ${x.domain ?? ''}\n- **Abhängig von:** ${(x.deps ?? []).join(', ')}\n`;
    const it = (text: string) => ({ id: x.nr, kind: 'session' as const, path: `docs/sprints/aktiv/${x.nr}-x.md`, text });
    expect(applyItems(d, [it(same)]).reload).toBe(false);
    expect(applyItems(d, [it(same.replace('Umgebung:** ', 'Umgebung:** neu'))]).reload).toBe(true);
  });
});
