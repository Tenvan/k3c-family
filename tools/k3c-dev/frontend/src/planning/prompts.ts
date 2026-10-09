// Prompt-Texte zum Weiterarbeiten im Chat, je Sprint, Session, markierter Auswahl und Backlog-Ticket (wie die
// ErpApi-Workbench). Sie nennen nur IDs und die Anweisung; Inhalt und Stand liest der Agent selbst (plan_*-Tools).
import type { PlanProject, PlanSession, PlanSprint, PlanTicket } from '../api';
import { byRank } from './projects';

/** Eine markierte Session samt Sprint. */
export interface Picked {
  sprint: PlanSprint;
  session: PlanSession;
}

const where = (s: PlanSprint) => `Sprint ${s.id}${s.worktree ? ` (Worktree ${s.worktree})` : ''}`;

/** Offen und mit Session-Datei: markierbar und mit eigenem Prompt. */
export const pickable = (x: PlanSession) => x.status !== 'fertig' && x.status !== 'entwurf';

/** Darf autonom im Worktree laufen: offen oder in Arbeit, `Agent: autonom` und `Umgebung: offline` (arbeitsweise.md › Umgebung). */
export const worktreeOk = (x: PlanSession) =>
  (x.status === 'offen' || x.status === 'in Arbeit') && x.agent === 'autonom' && x.env === 'offline';

export const openSessions = (s: PlanSprint): Picked[] => s.sessions.filter(pickable).map((session) => ({ sprint: s, session }));

export function promptSprint(s: PlanSprint): string {
  return (
    `Wir arbeiten weiter an ${where(s)}.\n` +
    'Lies zuerst die Sprint-README (`plan_get`), fasse den Stand in drei Sätzen zusammen und mach bei der nächsten offenen Session weiter.'
  );
}

export function promptSession(s: PlanSprint, x: PlanSession): string {
  return (
    `Wir arbeiten weiter an ${where(s)}, Session ${x.nr}.\n` +
    'Lies die Session-Datei (`plan_get`) und mach an dieser Stelle weiter (docs/arbeitsweise.md › Autonomer Ablauf). Status nur per `plan_set` ändern.'
  );
}

/** Sessions je Sprint gruppiert, in der Reihenfolge der Liste. */
function groups(items: Picked[]): string {
  const bySprint = new Map<string, Picked[]>();
  items.forEach((it) => bySprint.set(it.sprint.id, [...(bySprint.get(it.sprint.id) ?? []), it]));
  return [...bySprint.values()].map((its) => `${where(its[0].sprint)}:\n${its.map((it) => `- ${it.session.nr}`).join('\n')}`).join('\n');
}

const these = (n: number) => (n > 1 ? `diese ${n} Sessions` : 'diese Session');

/** Prompt für mehrere markierte Sessions im Hauptcheckout, nach Rang der Projekte geordnet (byRank). */
export function promptSessions(items: Picked[], projects: PlanProject[] = []): string {
  return (
    `Arbeite ${these(items.length)} der Reihe nach ab:\n${groups(byRank(items, projects))}\n` +
    'Lies jede Session-Datei (`plan_get`), bevor du mit ihr beginnst. Status nur per `plan_set` ändern.'
  );
}

/** Ziel der Übernahme aus dem Backlog: ein vorhandener Sprint (ID) oder ein neu zu formender. */
export type SprintTarget = { kind: 'sprint'; id: string } | { kind: 'new' };
export const NEW_SPRINT = '__new__';

/** Prompt zum Einplanen von Backlog-Tickets: nur IDs und Anweisung, den Inhalt liest der Agent selbst. */
export function backlogPrompt(tickets: PlanTicket[], target: SprintTarget): string {
  const many = tickets.length > 1;
  const ids = `${many ? 'die Backlog-Tickets' : 'das Backlog-Ticket'} ${tickets.map((t) => t.nr).join(', ')}`;
  const read = `Lies ${many ? 'die Tickets' : 'das Ticket'} (\`plan_get\`) und prüfe eine schon eingetragene Sprint-Zuordnung`;
  const fields = (id: string) =>
    `Setze danach per \`plan_set\` ${many ? 'in jedem Ticket' : 'im Ticket'} Status \`eingeplant\` und Sprint \`${id}\` und ergänze das Feld Tickets des Sprints. Ändere keinen Code.`;
  if (target.kind === 'new') {
    return (
      `Forme ${ids} zu einem neuen Sprint.\n` +
      `${read}, lies docs/vorlagen/sprint.md, interviewe mich zu Ziel, Domäne und Sessions und lege den Sprint mit \`plan_create\` an. ${fields('<neue Sprint-ID>')}`
    );
  }
  return (
    `Nimm ${ids} in den Sprint ${target.id} auf.\n` +
    `${read}, lies die Sprint-README und ordne ${many ? 'jedes' : 'es'} einer passenden Session zu oder lege eine neue an (\`plan_create\`, SDD-Kriterien). ` +
    `${fields(target.id)} Zeig mir danach die Änderungen.`
  );
}
