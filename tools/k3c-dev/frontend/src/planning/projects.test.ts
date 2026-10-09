import { describe, expect, it, vi } from 'vitest';
import { mockPlanningData } from '../api/mockPlanning';
import { groupProjects, moveRank, nextSession, rankedCount, sessionProgress, stepRank } from './projects';

const ids = (xs: { project: { id: string } }[]) => xs.map((x) => x.project.id);

describe('Projekte nach Rang (B-358)', () => {
  it('aktive nach Rang mit Sprints in Tabellen-Reihenfolge, Fortschritt und nächster Session (AC-06)', () => {
    const d = mockPlanningData();
    d.projects!.reverse(); // Reihenfolge der Lieferung zählt nicht, nur der Rang
    const g = groupProjects(d);
    expect(ids(g.active)).toEqual(['BET', 'SKL', 'RGW']);
    const bet = g.active[0];
    expect(bet.sprints.map((s) => s.id)).toEqual(['SP11', 'F2']);
    expect([bet.done, bet.total]).toEqual([0, 2]);
    expect(sessionProgress(bet.sprints[0])).toEqual([1, 4]);
    expect(bet.next?.session.nr).toBe('SP11.3'); // SP11.2 ist Mensch, SP11.4 wartet
    expect(g.active[1].next).toBeNull(); // S1.1 wartet auf den Entwurf F2.1
    expect(g.active[1].loose.map((t) => t.nr)).toEqual(['B-007', 'B-112']);
  });

  it('ruhende und erledigte eingeklappt, ABN und „Ohne Projekt“ eigene Bereiche (AC-08)', () => {
    const g = groupProjects(mockPlanningData());
    expect(ids(g.folded)).toEqual(['GRA', 'FND']);
    expect(g.abn?.project.id).toBe('ABN');
    expect(g.abnOpen.map((it) => it.session.nr)).toEqual(['X1.1']);
    expect(g.without.sprints.map((s) => s.id)).toEqual(['M5']);
    expect(g.without.tickets.map((t) => t.nr)).toEqual(['B-011', 'B-363']);
  });

  it('ohne Projekte bleiben alle Bereiche leer (bisherige Ansicht)', () => {
    const d = { ...mockPlanningData(), projects: [] };
    const g = groupProjects(d);
    expect([g.active.length, g.folded.length, g.abn]).toEqual([0, 0, null]);
  });

  it('nächste Session: erste offene, autonome mit erledigten Abhängigkeiten im ersten nicht erledigten Sprint', () => {
    const d = mockPlanningData();
    const sp11 = d.sprints.find((s) => s.id === 'SP11')!;
    sp11.sessions[0].status = 'verworfen'; // zählt wie fertig
    expect(nextSession([sp11], d.sprints)?.session.nr).toBe('SP11.3');
    sp11.sessions[0].status = 'offen';
    expect(nextSession([sp11], d.sprints)?.session.nr).toBe('SP11.1');
  });
});

describe('Rang hoch und runter (AC-07)', () => {
  it('am Rand deaktiviert, ohne Rang nie', () => {
    const d = mockPlanningData();
    const [bet, skl, rgw] = groupProjects(d).active.map((v) => v.project);
    const n = rankedCount(d.projects);
    expect(n).toBe(3);
    expect([stepRank(bet, -1, n), stepRank(bet, 1, n)]).toEqual([null, 2]);
    expect([stepRank(skl, -1, n), stepRank(rgw, 1, n)]).toEqual([1, null]);
    expect(stepRank(d.projects!.find((p) => p.id === 'ABN')!, 1, n)).toBeNull();
  });

  it('ruft planningSet mit dem neuen Rang und lädt danach neu', async () => {
    const set = vi.fn(async () => 'ok');
    const reload = vi.fn(async () => undefined);
    expect(await moveRank(set, 'SKL', 1, reload)).toBe('');
    expect(set).toHaveBeenCalledWith('SKL', 'Rang', '1');
    expect(reload).toHaveBeenCalledOnce();
  });

  it('Ablehnung liefert den Grund und lädt nicht neu', async () => {
    const reload = vi.fn(async () => undefined);
    const why = await moveRank(async () => { throw new Error('Rang 4 außerhalb 1 … 3'); }, 'RGW', 4, reload);
    expect(why).toBe('Rang 4 außerhalb 1 … 3');
    expect(reload).not.toHaveBeenCalled();
  });
});
