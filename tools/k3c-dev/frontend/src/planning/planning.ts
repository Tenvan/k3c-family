// Reine Logik der Ansicht „Sprints & Backlog“: Filter, Gruppierung, nächste Session. Kein React, damit Vitest sie prüft.
import type { GitHubSprint, PlanProject, PlanSession, PlanSprint, PlanTicket, PlanningData } from '../api';
import type { Tone } from '../ui/parts';

/** Filter und Auswahl; überleben das Neuladen nach `planning:changed` und werden gemerkt (lib/prefs). */
export interface PlanFilter {
  /** Backlog-Filter (B-401): wirkt auf Backlog-Liste und Sprint-Liste, nicht auf die Projekte. */
  q: string;
  quick: string[];
  prios: string[];
  envs: string[];
  projs: string[];
  done: boolean;
  /** Projekt-Filter (B-401): wirkt nur auf die Projekte. `doms` = Domäne ihrer Sprints und Tickets. */
  pstatus: string[];
  pquick: string[];
  doms: string[];
  sel: string;
}

export const EMPTY_FILTER: PlanFilter = { q: '', quick: [], prios: [], envs: [], projs: [], done: false, pstatus: [], pquick: [], doms: [], sel: '' };

/** Anteil des Detailbereichs an der rechten Spalte in Prozent (B-400); Grenzen wie auf der Tasks-Seite. */
export const SPLIT_DEFAULT = 48;

/** Zustände der Projekte für den Projekt-Filter; „inaktiv“ ist abgeleitet (siehe projectState). */
export const PROJECT_STATES = ['aktiv', 'inaktiv', 'ruht', 'erledigt'];

/** Zustand eines Projekts: Status, nur „aktiv“ ohne offenen (nicht erledigten) Sprint heißt „inaktiv“. */
export function projectState(p: PlanProject, sprints: PlanSprint[]): string {
  const open = p.sprints.some((id) => sprints.find((s) => s.id === id)?.status !== 'erledigt');
  return p.status === 'aktiv' && !open ? 'inaktiv' : p.status;
}

const PRIO = ['hoch', 'mittel', 'niedrig'];

export const isNext = (x: PlanSession) => x.status === 'offen' && x.agent === 'autonom';

/** Schnellfilter des Backlog-Filters: nur Tickets, mehrere zugleich verknüpft mit „und“. Prio und Umgebung haben eigene Chips. */
export const QUICK: { id: string; label: string; sprint?: (s: PlanSprint) => boolean; ticket?: (t: PlanTicket) => boolean }[] = [
  { id: 'offen', label: 'Offen', ticket: (t) => t.status === 'offen' },
  { id: 'eingeplant', label: 'Eingeplant', ticket: (t) => t.status === 'eingeplant' },
  { id: 'ungeplant', label: 'Ungeplant', ticket: (t) => !t.sprint || t.sprint === '–' },
];

/** Schnellfilter des Projekt-Filters (B-401): ein Projekt bleibt, wenn mindestens einer seiner Sprints passt. */
export const PQUICK: { id: string; label: string; sprint: (s: PlanSprint) => boolean }[] = [
  { id: 'aktiv', label: 'Aktive Sprints', sprint: (s) => s.status === 'aktiv' },
  { id: 'agent', label: 'Agent offen', sprint: (s) => s.sessions.some(isNext) },
  { id: 'autonom', label: 'Autonom', sprint: (s) => s.sessions.some((x) => x.agent === 'autonom' && x.status !== 'fertig' && x.status !== 'entwurf') },
];

const has = (q: string, ...xs: (string | undefined)[]) => q === '' || xs.some((x) => (x ?? '').toLowerCase().includes(q));
const oneOf = (list: string[], v: string | undefined) => list.length === 0 || list.includes(v || '–');

export function domains(d: PlanningData): string[] {
  return [...new Set([...d.sprints.map((s) => s.domain), ...d.tickets.map((t) => t.domain)].map((x) => x || '–'))].sort();
}

/** Sprint-Liste nach dem Backlog-Filter: Prio über die Tickets des Sprints, Umgebung über seine Sessions (B-401). */
export function filterSprints(d: PlanningData, f: PlanFilter): PlanSprint[] {
  const q = f.q.trim().toLowerCase();
  const quick = QUICK.filter((x) => f.quick.includes(x.id) && x.sprint);
  const prio = new Map(d.tickets.map((t) => [t.nr, t.prio]));
  const prioOk = (s: PlanSprint) => f.prios.length === 0 || s.tickets.some((x) => f.prios.includes(prio.get(x.split(' ')[0]) ?? ''));
  const envOk = (s: PlanSprint) => f.envs.length === 0 || s.sessions.some((x) => f.envs.includes(x.env ?? ''));
  return d.sprints.filter((s) =>
    (f.done || s.status !== 'erledigt') && oneOf(f.projs, s.project) && prioOk(s) && envOk(s) &&
    quick.every((x) => x.sprint!(s)) &&
    has(q, s.id, s.title, s.tickets.join(' '), ...s.sessions.map((x) => `${x.nr} ${x.titel}`)));
}

const STATES = ['aktiv', 'geplant', 'erledigt'];
export const prioRank = (p = '') => (PRIO.includes(p) ? PRIO.indexOf(p) : PRIO.length);

/** Sprints mit Worktree zuerst, dann mit offenem Branch (PR offen/Entwurf), dann je Status wie geliefert: Go ordnet
 *  nach Abhängigkeit und Prio, eine Voraussetzung erbt die Prio ihrer Abnehmer (planning.rank). */
export function sortSprints(sprints: PlanSprint[], gh?: Record<string, GitHubSprint>): PlanSprint[] {
  const rank = (s: PlanSprint) => {
    const st = gh?.[s.id.toUpperCase()]?.state;
    return s.worktree ? 0 : st === 'offen' || st === 'Entwurf' ? 1 : 2;
  };
  return [...sprints].sort((a, b) => rank(a) - rank(b) || STATES.indexOf(a.status) - STATES.indexOf(b.status));
}

export function filterTickets(d: PlanningData, f: PlanFilter): PlanTicket[] {
  const q = f.q.trim().toLowerCase();
  const quick = QUICK.filter((x) => f.quick.includes(x.id) && x.ticket);
  return d.tickets.filter((t) => oneOf(f.prios, t.prio) && oneOf(f.envs, t.env) && oneOf(f.projs, t.project) &&
    quick.every((x) => x.ticket!(t)) && has(q, t.nr, t.title, t.sprint));
}

/** Projekte nach dem Projekt-Filter: Status, Domäne (eines ihrer Sprints oder Tickets) und Sprint-Schnellfilter; der
 *  Backlog-Filter wirkt nicht. */
export function filterProjects(d: PlanningData, f: PlanFilter): PlanProject[] {
  const inDomain = (p: PlanProject) => f.doms.length === 0 ||
    d.sprints.some((s) => p.sprints.includes(s.id) && f.doms.includes(s.domain || '–')) ||
    d.tickets.some((t) => t.project === p.id && f.doms.includes(t.domain || '–'));
  const mine = (p: PlanProject) => d.sprints.filter((s) => p.sprints.includes(s.id));
  const pquick = PQUICK.filter((x) => f.pquick.includes(x.id));
  return (d.projects ?? []).filter((p) => oneOf(f.pstatus, projectState(p, d.sprints)) && inDomain(p) &&
    pquick.every((x) => mine(p).some(x.sprint)));
}

/** Engt der Backlog-Filter ein (Suche, Prio, Umgebung, Projekt, Einplanung)? Dann zeigt die Projekt-Spalte nur Treffer. */
export const narrowed = (f: PlanFilter) => f.q.trim() !== '' || f.prios.length + f.envs.length + f.projs.length + f.quick.length > 0;

/** Projekte mit Treffer: ein gezeigter Sprint, ein gezeigtes Ticket ohne Sprint oder die Suche passt auf Kürzel und Titel. */
export function projectHits(projects: PlanProject[], sprints: PlanSprint[], tickets: PlanTicket[], f: PlanFilter): PlanProject[] {
  if (!narrowed(f)) return projects;
  const q = f.q.trim().toLowerCase();
  const shown = new Set(sprints.map((s) => s.id));
  return projects.filter((p) => p.sprints.some((id) => shown.has(id)) || tickets.some((t) => t.project === p.id) ||
    (q !== '' && has(q, p.id, p.title)));
}

/** Tickets je Domäne (alphabetisch), darin nach Prio und Nummer. */
export function groupTickets(tickets: PlanTicket[]): [string, PlanTicket[]][] {
  const rank = (t: PlanTicket) => prioRank(t.prio);
  const groups = new Map<string, PlanTicket[]>();
  for (const t of tickets) groups.set(t.domain || '–', [...(groups.get(t.domain || '–') ?? []), t]);
  return [...groups.entries()]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([dom, ts]) => [dom, [...ts].sort((a, b) => rank(a) - rank(b) || a.nr.localeCompare(b.nr))]);
}

/** Gibt es die Auswahl (Session, Sprint, Projekt oder Ticket) noch? */
export const exists = (d: PlanningData, id: string) =>
  !!findSession(d, id) || d.sprints.some((s) => s.id === id) || !!d.projects?.some((p) => p.id === id) || d.tickets.some((t) => t.nr === id);

export function findSession(d: PlanningData, nr: string): { sprint: PlanSprint; session: PlanSession } | null {
  for (const sprint of d.sprints) for (const session of sprint.sessions) if (session.nr === nr) return { sprint, session };
  return null;
}

/** Tickets, die zur ausgewählten Session gehören: die des Sprints und alle, die auf den Sprint zeigen. */
export function linkedTickets(d: PlanningData, sel: string): Set<string> {
  const hit = findSession(d, sel);
  if (!hit) return new Set();
  return new Set([...hit.sprint.tickets, ...d.tickets.filter((t) => t.sprint === hit.sprint.id).map((t) => t.nr)]);
}

export const toggle = (arr: string[], v: string) => (arr.includes(v) ? arr.filter((x) => x !== v) : [...arr, v]);

/** Gemerkten Filter lesen; alles Unpassende fällt auf die Vorgabe zurück. */
export function parseFilter(raw: string): PlanFilter {
  try {
    const v = JSON.parse(raw) as Partial<PlanFilter>;
    const strs = (x: unknown) => (Array.isArray(x) ? x.filter((y): y is string => typeof y === 'string') : []);
    // Freitext wird nicht gemerkt (Workbench-Spec › Gemerkter Zustand).
    return { q: '', quick: strs(v.quick).filter((id) => QUICK.some((x) => x.id === id)), prios: strs(v.prios), envs: strs(v.envs),
      projs: strs(v.projs), done: v.done === true, pstatus: strs(v.pstatus),
      pquick: strs(v.pquick).filter((id) => PQUICK.some((x) => x.id === id)),
      // älterer Speicher ohne pstatus: die Domäne galt damals für Sprints und Tickets, jetzt für Projekte
      doms: Array.isArray(v.pstatus) ? strs(v.doms) : [], sel: typeof v.sel === 'string' ? v.sel : '' };
  } catch {
    return EMPTY_FILTER;
  }
}

/** Badges des GitHub-Stands auf der Sprint-Karte: PR, dann CI und Merge (nur wenn bekannt bzw. PR offen). */
export function ghBadges(p: GitHubSprint): { label: string; tone: Tone }[] {
  const pr: Record<GitHubSprint['state'], Tone> = { offen: 'info', Entwurf: 'neutral', gemergt: 'ok', geschlossen: 'neutral' };
  const ci: Record<GitHubSprint['ci'], Tone> = { grün: 'ok', rot: 'error', läuft: 'warn', '–': 'neutral' };
  const merge: Record<GitHubSprint['merge'], Tone> = { konfliktfrei: 'ok', Konflikt: 'error', unbekannt: 'neutral', '–': 'neutral' };
  const out = [{ label: `#${p.number} ${p.state}`, tone: pr[p.state] }];
  if (p.ci !== '–') out.push({ label: `CI ${p.ci}`, tone: ci[p.ci] });
  if (p.merge !== '–') out.push({ label: p.merge, tone: merge[p.merge] });
  return out;
}
