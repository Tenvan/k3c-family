// Reine Logik der Ansicht „Sprints & Backlog“: Filter, Gruppierung, nächste Session. Kein React, damit Vitest sie prüft.
import type { GitHubSprint, PlanSession, PlanSprint, PlanTicket, PlanningData } from '../api';
import type { Tone } from '../ui/parts';

/** Filter und Auswahl; überleben das Neuladen nach `planning:changed` und werden gemerkt (lib/prefs). */
export interface PlanFilter {
  q: string;
  doms: string[];
  quick: string[];
  sel: string;
  done: boolean;
}

export const EMPTY_FILTER: PlanFilter = { q: '', doms: [], quick: [], sel: '', done: false };

const PRIO = ['hoch', 'mittel', 'niedrig'];

export const isNext = (x: PlanSession) => x.status === 'offen' && x.agent === 'autonom';

/** Schnellfilter: wirken auf Sprints oder auf Tickets, mehrere zugleich verknüpft mit „und“. */
export const QUICK: { id: string; label: string; sprint?: (s: PlanSprint) => boolean; ticket?: (t: PlanTicket) => boolean }[] = [
  { id: 'aktiv', label: 'Aktive Sprints', sprint: (s) => s.status === 'aktiv' },
  { id: 'erledigt', label: 'Erledigt', sprint: (s) => s.status === 'erledigt' },
  { id: 'agent', label: 'Agent offen', sprint: (s) => s.sessions.some(isNext) },
  { id: 'hoch', label: 'Prio hoch', ticket: (t) => t.prio === 'hoch' },
  { id: 'offen', label: 'Offen', ticket: (t) => t.status === 'offen' },
  { id: 'eingeplant', label: 'Eingeplant', ticket: (t) => t.status === 'eingeplant' },
  { id: 'ungeplant', label: 'Ungeplant', ticket: (t) => !t.sprint || t.sprint === '–' },
];

const has = (q: string, ...xs: (string | undefined)[]) => q === '' || xs.some((x) => (x ?? '').toLowerCase().includes(q));
const domOk = (f: PlanFilter, d: string) => f.doms.length === 0 || f.doms.includes(d || '–');

export function domains(d: PlanningData): string[] {
  return [...new Set([...d.sprints.map((s) => s.domain), ...d.tickets.map((t) => t.domain)].map((x) => x || '–'))].sort();
}

export function filterSprints(d: PlanningData, f: PlanFilter): PlanSprint[] {
  const q = f.q.trim().toLowerCase();
  const quick = QUICK.filter((x) => f.quick.includes(x.id) && x.sprint);
  return d.sprints.filter((s) =>
    (f.done || f.quick.includes('erledigt') || s.status !== 'erledigt') && domOk(f, s.domain) &&
    quick.every((x) => x.sprint!(s)) &&
    has(q, s.id, s.title, s.tickets.join(' '), ...s.sessions.map((x) => `${x.nr} ${x.titel}`)));
}

const STATES = ['aktiv', 'geplant', 'erledigt'];
export const prioRank = (p = '') => (PRIO.includes(p) ? PRIO.indexOf(p) : PRIO.length);

/** Sprints mit Worktree zuerst, dann mit offenem Branch (PR offen/Entwurf), dann je Status nach Prio, sonst wie geliefert. */
export function sortSprints(sprints: PlanSprint[], gh?: Record<string, GitHubSprint>): PlanSprint[] {
  const rank = (s: PlanSprint) => {
    const st = gh?.[s.id.toUpperCase()]?.state;
    return s.worktree ? 0 : st === 'offen' || st === 'Entwurf' ? 1 : 2;
  };
  return [...sprints].sort((a, b) =>
    rank(a) - rank(b) || STATES.indexOf(a.status) - STATES.indexOf(b.status) || prioRank(a.prio) - prioRank(b.prio));
}

export function filterTickets(d: PlanningData, f: PlanFilter): PlanTicket[] {
  const q = f.q.trim().toLowerCase();
  const quick = QUICK.filter((x) => f.quick.includes(x.id) && x.ticket);
  return d.tickets.filter((t) => domOk(f, t.domain) && quick.every((x) => x.ticket!(t)) && has(q, t.nr, t.title, t.sprint));
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
    return { q: typeof v.q === 'string' ? v.q : '', doms: strs(v.doms), quick: strs(v.quick),
      sel: typeof v.sel === 'string' ? v.sel : '', done: v.done === true };
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
