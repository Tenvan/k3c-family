// Reine Logik der Projekt-Ansicht (B-358): Projekte nach Rang, Fortschritt, nächste Session, Bereiche ABN und „Ohne
// Projekt“, Rang verschieben. Kein React, damit Vitest sie prüft.
import type { PlanProject, PlanSession, PlanSprint, PlanTicket, PlanningData } from '../api';
import { errorText } from '../lib/errors';
import type { Picked } from './prompts';

/** Projekt der Abnahmen am Gerät: aktiv, ohne Rang, läuft neben der Rangfolge (arbeitsweise.md › Projekte und Rang). */
export const ABN = 'ABN';

export const hasProject = (v?: string) => !!v && v !== '–';
const closed = (x: PlanSession) => x.status === 'fertig' || x.status === 'verworfen';
const rankOf = (p: PlanProject) => (/^\d+$/.test(p.rang) ? Number(p.rang) : Infinity);

/** Ein Projekt mit seinen (gefilterten) Sprints in Tabellen-Reihenfolge. */
export interface ProjectView {
  project: PlanProject;
  sprints: PlanSprint[];
  /** Erledigte Sprints von allen der Tabelle. */
  done: number;
  total: number;
  next: Picked | null;
  /** Tickets des Projekts ohne Sprint. */
  loose: PlanTicket[];
}

export interface ProjectGroups {
  active: ProjectView[];
  abn: ProjectView | null;
  /** Offene Sessions mit `Agent: Mensch` in ABN. */
  abnOpen: Picked[];
  folded: ProjectView[];
  without: { sprints: PlanSprint[]; tickets: PlanTicket[] };
}

/** Fertige oder verworfene Sessions je Sprint und alle. */
export const sessionProgress = (s: PlanSprint) => [s.sessions.filter(closed).length, s.sessions.length] as const;

/** Erste Session mit `offen`, `autonom` und erledigten Abhängigkeiten im ersten nicht erledigten Sprint. Abhängigkeiten,
 *  die in keinem Sprint offen sind, gelten als erledigt (wie planning.Order in Go). */
export function nextSession(sprints: PlanSprint[], all: PlanSprint[]): Picked | null {
  const open = new Set(all.flatMap((s) => s.sessions.filter((x) => !closed(x)).map((x) => x.nr)));
  const sprint = sprints.find((s) => s.status !== 'erledigt');
  const session = sprint?.sessions.find((x) => x.status === 'offen' && x.agent === 'autonom' && (x.deps ?? []).every((d) => !open.has(d)));
  return sprint && session ? { sprint, session } : null;
}

/** Gruppiert die gezeigten Sprints und offenen Tickets nach Projekten; ohne Projekte sind alle Listen leer. */
export function groupProjects(d: PlanningData, shown: PlanSprint[] = d.sprints, tickets: PlanTicket[] = d.tickets): ProjectGroups {
  const projects = d.projects ?? [];
  const byId = new Map(shown.map((s) => [s.id, s]));
  const status = new Map(d.sprints.map((s) => [s.id, s.status]));
  const view = (project: PlanProject): ProjectView => {
    const sprints = project.sprints.flatMap((id) => byId.get(id) ?? []);
    const all = project.sprints.flatMap((id) => d.sprints.find((s) => s.id === id) ?? []);
    return { project, sprints, done: project.sprints.filter((id) => status.get(id) === 'erledigt').length, total: project.sprints.length,
      next: nextSession(all, d.sprints), loose: tickets.filter((t) => t.project === project.id && !hasProject(t.sprint)) };
  };
  const active = projects.filter((p) => p.status === 'aktiv' && p.id !== ABN).sort((a, b) => rankOf(a) - rankOf(b)).map(view);
  const abnProject = projects.find((p) => p.id === ABN && p.status === 'aktiv');
  const abn = abnProject ? view(abnProject) : null;
  const abnOpen = (abn?.sprints ?? []).flatMap((sprint) =>
    sprint.sessions.filter((x) => x.agent === 'Mensch' && x.status === 'offen').map((session) => ({ sprint, session })));
  const folded = [...projects.filter((p) => p.status === 'ruht'), ...projects.filter((p) => p.status === 'erledigt')].map(view);
  const listed = new Set(projects.flatMap((p) => p.sprints));
  return { active, abn, abnOpen, folded, without: {
    sprints: shown.filter((s) => !listed.has(s.id) && !hasProject(s.project)),
    tickets: tickets.filter((t) => !hasProject(t.project)),
  } };
}

/** Zahl der aktiven Projekte mit Rang (ohne ABN): Grenze für „runter“. */
export const rankedCount = (projects: PlanProject[] = []) => projects.filter((p) => p.status === 'aktiv' && rankOf(p) !== Infinity).length;

/** Neuer Rang beim Schritt dir (−1 hoch, +1 runter) oder null am Rand bzw. ohne Rang. */
export function stepRank(p: PlanProject, dir: -1 | 1, count: number): number | null {
  const to = rankOf(p) + dir;
  return Number.isFinite(to) && to >= 1 && to <= count ? to : null;
}

/** Setzt den Rang über planningSet und lädt danach neu; liefert den Grund einer Ablehnung oder ''. */
export async function moveRank(set: (id: string, field: string, value: string) => Promise<string>, id: string, to: number,
  reload: () => Promise<unknown>): Promise<string> {
  try {
    await set(id, 'Rang', String(to));
  } catch (e) {
    return errorText(e);
  }
  await reload();
  return '';
}

/** Ordnet markierte Sessions nach Rang (Projekt-Rang, dann Tabellenplatz), ABN danach, Sprints ohne Projekt zuletzt;
 *  bei Gleichstand bleibt die Reihenfolge. */
export function byRank(items: Picked[], projects: PlanProject[] = []): Picked[] {
  const key = new Map<string, number>();
  [...projects].sort((a, b) => rankOf(a) - rankOf(b)).forEach((p, i) => p.sprints.forEach((id, j) => key.set(id, i * 1000 + j)));
  const k = (it: Picked) => key.get(it.sprint.id) ?? Number.MAX_SAFE_INTEGER;
  return [...items].sort((a, b) => k(a) - k(b));
}
