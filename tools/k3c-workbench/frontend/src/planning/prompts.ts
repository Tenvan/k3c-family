// Prompt-Texte zum Weiterarbeiten im Chat, je Sprint, Session, markierter Auswahl und Backlog-Ticket. Kurzform ohne
// Anweisungen (B-639): nur Art und IDs; den Ablauf liefert der Skill `k3c-session`, Inhalt und Stand liest der Agent selbst.
import type { PlanProject, PlanSession, PlanSprint, PlanTicket } from '../api';
import { byRank } from './projects';

/** Eine markierte Session samt Sprint. */
export interface Picked {
  sprint: PlanSprint;
  session: PlanSession;
}

const tree = (s: PlanSprint) => (s.worktree ? ` (Worktree ${s.worktree})` : '');

/** Offen und mit Session-Datei: markierbar und mit eigenem Prompt. */
export const pickable = (x: PlanSession) => x.status !== 'fertig' && x.status !== 'entwurf';

/** Darf autonom im Worktree laufen: offen oder in Arbeit, `Agent: autonom` und `Umgebung: offline` (arbeitsweise.md › Umgebung). */
export const worktreeOk = (x: PlanSession) =>
  (x.status === 'offen' || x.status === 'in Arbeit') && x.agent === 'autonom' && x.env === 'offline';

export const openSessions = (s: PlanSprint): Picked[] => s.sessions.filter(pickable).map((session) => ({ sprint: s, session }));

export const promptSprint = (s: PlanSprint): string => `Sprint ${s.id}${tree(s)}`;

export const promptProject = (p: PlanProject): string => `Projekt ${p.id}`;

export const promptSession = (s: PlanSprint, x: PlanSession): string => `Session ${x.nr}${tree(s)}`;

/** Mehrere markierte Sessions in einer Zeile, nach Rang der Projekte geordnet (byRank). */
export function promptSessions(items: Picked[], projects: PlanProject[] = []): string {
  const list = byRank(items, projects).map((it) => `${it.session.nr}${tree(it.sprint)}`);
  return `${list.length > 1 ? 'Sessions' : 'Session'} ${list.join(', ')}`;
}

/** Ziel der Übernahme aus dem Backlog: ein vorhandener Sprint (ID) oder ein neu zu formender. */
export type SprintTarget = { kind: 'sprint'; id: string } | { kind: 'new' };
export const NEW_SPRINT = '__new__';

/** Prompt zum Einplanen von Backlog-Tickets: nur IDs und Ziel. */
export function backlogPrompt(tickets: PlanTicket[], target: SprintTarget): string {
  const ids = `${tickets.length > 1 ? 'Tickets' : 'Ticket'} ${tickets.map((t) => t.nr).join(', ')}`;
  return target.kind === 'new' ? `Neuer Sprint aus ${ids}` : `${ids} in Sprint ${target.id}`;
}
