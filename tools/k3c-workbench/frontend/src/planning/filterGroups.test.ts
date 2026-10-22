import { describe, expect, it } from 'vitest';
import { mockDoc, mockPlanningData } from '../api/mockPlanning';
import { AREA_LABEL, filterChips, filterGroups, isDirty, resetFilter, SHOW_DONE, toggleValue } from './filterGroups';
import { EMPTY_FILTER, exists, filterProjects, filterSprints, filterTickets, projectHits } from './planning';
import { parseDoc } from './readerDoc';

const d = mockPlanningData();

describe('Filterleiste nach Wirkung (B-439/AC-01)', () => {
  it('drei Bereiche, Gruppen in Leistenreihenfolge mit Zählern', () => {
    const g = filterGroups(EMPTY_FILTER, d);
    expect(g.map((x) => `${x.area}:${x.label}`)).toEqual(['common:Prio', 'common:Umgebung', 'common:Projekt', 'projects:Zustand',
      'projects:Sprints', 'projects:Domänen', 'backlog:Einplanung']);
    expect(Object.values(AREA_LABEL)).toEqual(['Gemeinsam', 'Projekte', 'Backlog']);
    expect(g[0].options.find((o) => o.id === 'hoch')?.count).toBe(d.tickets.filter((t) => t.prio === 'hoch').length);
    expect(g.every((x) => x.selected.length === 0)).toBe(true);
  });

  it('Umschalten setzt Häkchen und Chips, ein zweites Mal nimmt sie zurück', () => {
    let f = toggleValue(EMPTY_FILTER, 'prios', 'hoch');
    f = toggleValue(f, 'pquick', SHOW_DONE);
    f = toggleValue(f, 'doms', 'SIM');
    expect(f.prios).toEqual(['hoch']);
    expect(f.done).toBe(true);
    expect(filterChips(filterGroups(f, d)).map((c) => `${c.groupLabel}: ${c.label}`)).toEqual(['Prio: hoch', 'Sprints: Erledigte zeigen', 'Domänen: SIM']);
    expect(toggleValue(toggleValue(f, 'prios', 'hoch'), 'pquick', SHOW_DONE)).toEqual({ ...EMPTY_FILTER, doms: ['SIM'] });
  });

  it('gemerkte Werte ohne Daten bleiben als entfernbarer Chip sichtbar (sie filtern weiter)', () => {
    const f = { ...EMPTY_FILTER, doms: ['XYZ'] };
    expect(filterChips(filterGroups(f, d)).map((c) => `${c.groupLabel}: ${c.label}`)).toEqual(['Domänen: XYZ']);
    expect(toggleValue(f, 'doms', 'XYZ').doms).toEqual([]);
  });

  it('„alle zurücksetzen“ nur bei Abweichung, behält die Auswahl', () => {
    expect(isDirty(EMPTY_FILTER)).toBe(false);
    expect(isDirty({ ...EMPTY_FILTER, q: 'pi' })).toBe(true);
    expect(isDirty({ ...EMPTY_FILTER, done: true })).toBe(true);
    expect(resetFilter({ ...EMPTY_FILTER, prios: ['hoch'], q: 'x', sel: 'SP11' })).toEqual({ ...EMPTY_FILTER, sel: 'SP11' });
  });

  it('engt der Backlog-Filter ein, bleiben nur Projekte mit Treffer', () => {
    const f = { ...EMPTY_FILTER, q: 'raspberry' };
    const hits = projectHits(filterProjects(d, f), filterSprints(d, f), filterTickets(d, f), f).map((p) => p.id);
    expect(hits).toEqual(['BET']);
    expect(projectHits(d.projects ?? [], [], [], EMPTY_FILTER)).toHaveLength(d.projects?.length ?? 0);
  });

  it('Auswahl existiert für Session, Sprint, Projekt und Ticket', () => {
    for (const id of ['SP11.2', 'SP11', 'BET', 'B-007']) expect(exists(d, id), id).toBe(true);
    expect(exists(d, 'B-999')).toBe(false);
  });
});

describe('Lesemodus (B-439/AC-05)', () => {
  it('trennt Pfad und H1 ab, zerlegt in Rubriken und zählt Häkchen', () => {
    const doc = parseDoc(mockDoc(d, 'SP11.2'));
    expect(doc.path).toBe('docs/sprints/SP11/SP11.2.md');
    expect(doc.sections.filter((s) => s.title).map((s) => s.title)).toEqual(['Ziel', 'Erlaubte Dateien', 'Schritte', 'Fertig, wenn']);
    expect(doc.sections[0].text).toContain('**Status:** offen');
    expect(doc.sections.some((s) => s.text.includes('# SP11.2'))).toBe(false);
    expect([doc.done, doc.total]).toEqual([1, 2]);
  });

  it('Rubrik im Codeblock zählt nicht, doppelte Titel bekommen eigene IDs', () => {
    const doc = parseDoc('# T\n\n## A `x`\n\n```\n## kein\n- [x] kein\n```\n\n## A x\n- [ ] eins\n');
    expect(doc.sections.map((s) => s.id)).toEqual(['rd-a-x', 'rd-a-x-2']);
    expect(doc.sections[0].title).toBe('A x');
    expect([doc.done, doc.total]).toEqual([0, 1]);
  });

  it('Ticket und Sprint haben einen Volltext im Mock, Unbekanntes wirft', () => {
    expect(parseDoc(mockDoc(d, 'B-007')).total).toBe(2);
    expect(parseDoc(mockDoc(d, 'SP11')).sections.map((s) => s.title)).toContain('Sessions');
    expect(() => mockDoc(d, 'B-999')).toThrow('B-999');
  });
});
