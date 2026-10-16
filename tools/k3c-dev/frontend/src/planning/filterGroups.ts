// Gruppen der Filterleiste (wie die erp-qs-Workbench, B-439): jeder Filter liegt in einer Gruppe, die Gruppen in drei
// Bereichen nach Wirkung. Reine Funktionen, damit die Leiste nur zeichnet: Gruppen lesen, Wert umschalten, Chips.
import type { PlanningData } from '../api';
import { domains, EMPTY_FILTER, PQUICK, PROJECT_STATES, QUICK, toggle, type PlanFilter } from './planning';

export type FilterArea = 'common' | 'projects' | 'backlog';
export type GroupKey = 'prios' | 'envs' | 'projs' | 'pstatus' | 'pquick' | 'doms' | 'quick';
/** Verknüpfung in der Gruppe; zwischen Gruppen gilt immer UND. */
export type GroupJoin = 'and' | 'or';

/** Gemeinsam wirkt auf die Sprints der Projekte und den Backlog (Backlog-Filter), Projekte nur auf die Projekte. */
export const AREA_LABEL: Record<FilterArea, string> = { common: 'Gemeinsam', projects: 'Projekte', backlog: 'Backlog' };
export const JOIN_HINT: Record<GroupJoin, string> = { and: 'alle angehakten zugleich (UND)', or: 'einer der angehakten (ODER)' };

/** Wert „Erledigte zeigen“ der Gruppe Sprints; steht im Filter als `done`. */
export const SHOW_DONE = 'erledigt';

export interface FilterOption {
  id: string;
  label: string;
  /** Anzahl hinter dem Wert in der Checkliste. */
  count?: number;
}

export interface FilterGroup {
  key: GroupKey;
  area: FilterArea;
  label: string;
  join: GroupJoin;
  options: FilterOption[];
  /** Angehakte Werte, nur solche, die es unter `options` gibt. */
  selected: string[];
}

export interface FilterChip {
  group: GroupKey;
  groupLabel: string;
  id: string;
  label: string;
}

const counted = (ids: string[], values: (string | undefined)[]): FilterOption[] =>
  ids.map((id) => ({ id, label: id, count: values.filter((v) => (v || '–') === id).length }));

/** Alle Gruppen in Leistenreihenfolge; eine Gruppe ohne Werte zeichnet die Leiste nicht. */
export function filterGroups(f: PlanFilter, d: PlanningData): FilterGroup[] {
  const projects = d.projects ?? [];
  const groups: Omit<FilterGroup, 'selected'>[] = [
    { key: 'prios', area: 'common', label: 'Prio', join: 'or', options: counted(['hoch', 'mittel', 'niedrig'], d.tickets.map((t) => t.prio)) },
    { key: 'envs', area: 'common', label: 'Umgebung', join: 'or', options: counted(['offline', 'live'], d.tickets.map((t) => t.env)) },
    { key: 'projs', area: 'common', label: 'Projekt', join: 'or', options: counted(projects.map((p) => p.id), d.tickets.map((t) => t.project)) },
    { key: 'pstatus', area: 'projects', label: 'Zustand', join: 'or', options: counted(PROJECT_STATES, projects.map((p) => p.status)) },
    { key: 'pquick', area: 'projects', label: 'Sprints', join: 'and',
      options: [...PQUICK.map((x) => ({ id: x.id, label: x.label })), { id: SHOW_DONE, label: 'Erledigte zeigen', count: d.done }] },
    { key: 'doms', area: 'projects', label: 'Domänen', join: 'or', options: domains(d).map((x) => ({ id: x, label: x })) },
    { key: 'quick', area: 'backlog', label: 'Einplanung', join: 'and', options: QUICK.map((x) => ({ id: x.id, label: x.label })) },
  ];
  const picked = (key: GroupKey) => (key === 'pquick' && f.done ? [...f.pquick, SHOW_DONE] : f[key]);
  // Häkchen in Reihenfolge der Werte; Gemerktes ohne Daten (wirkt im Filter weiter) hängt hinten an und bleibt als Chip entfernbar.
  return groups.map((g) => {
    const ids = g.options.map((o) => o.id);
    return { ...g, selected: [...ids.filter((id) => picked(g.key).includes(id)), ...picked(g.key).filter((id) => !ids.includes(id))] };
  });
}

/** Schaltet einen Wert einer Gruppe um; „Erledigte zeigen“ ist der Schalter `done`. */
export function toggleValue(f: PlanFilter, key: GroupKey, id: string): PlanFilter {
  if (key === 'pquick' && id === SHOW_DONE) return { ...f, done: !f.done };
  return { ...f, [key]: toggle(f[key], id) };
}

/** Die angehakten Werte aller Gruppen als entfernbare Chips, in Leistenreihenfolge. */
export function filterChips(groups: FilterGroup[]): FilterChip[] {
  return groups.flatMap((g) =>
    g.selected.map((id) => ({ group: g.key, groupLabel: g.label, id, label: g.options.find((o) => o.id === id)?.label ?? id })));
}

/** Weicht der Filter vom Standard ab (Freitext oder ein Häkchen)? Dann zeigt die Leiste „alle zurücksetzen“. */
export const isDirty = (f: PlanFilter) =>
  f.q.trim() !== '' || f.done || [f.prios, f.envs, f.projs, f.pstatus, f.pquick, f.doms, f.quick].some((x) => x.length > 0);

/** Standard der Leiste; die Auswahl bleibt stehen. */
export const resetFilter = (f: PlanFilter): PlanFilter => ({ ...EMPTY_FILTER, sel: f.sel });
