// Prompt-Texte zum Weiterarbeiten im Chat, je Sprint, Session, markierter Auswahl und Backlog-Ticket (wie die
// ErpApi-Workbench). Sie nennen nur IDs und die Anweisung; Inhalt und Stand liest der Agent selbst (plan_*-Tools).
import type { PlanSession, PlanSprint, PlanTicket } from '../api';

/** Eine markierte Session samt Sprint. */
export interface Picked {
  sprint: PlanSprint;
  session: PlanSession;
}

/** Weitere Variante im Menü des Kopierbuttons; `blocked` nennt den Grund, warum sie nicht geht. */
export interface PromptVariant {
  label: string;
  prompt: string;
  blocked?: string;
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

/** Sessions je Sprint gruppiert, in der Reihenfolge der Planung (Abhängigkeit, dann Prio). */
function groups(items: Picked[]): string {
  const bySprint = new Map<string, Picked[]>();
  items.forEach((it) => bySprint.set(it.sprint.id, [...(bySprint.get(it.sprint.id) ?? []), it]));
  return [...bySprint.values()].map((its) => `${where(its[0].sprint)}:\n${its.map((it) => `- ${it.session.nr}`).join('\n')}`).join('\n');
}

const these = (n: number) => (n > 1 ? `diese ${n} Sessions` : 'diese Session');

/** Prompt für mehrere markierte Sessions im Hauptcheckout. */
export function promptSessions(items: Picked[]): string {
  return (
    `Arbeite ${these(items.length)} der Reihe nach ab:\n${groups(items)}\n` +
    'Lies jede Session-Datei (`plan_get`), bevor du mit ihr beginnst. Status nur per `plan_set` ändern.'
  );
}

/** Warum die Worktree-Variante nicht geht; leer, wenn sie geht. */
export function worktreeBlocked(items: Picked[]): string {
  if (items.length === 0) return 'keine offene Session';
  if (!items.some((it) => worktreeOk(it.session))) return 'keine Session ist autonom · offline';
  return '';
}

/** Prompt, die autonomen Sessions ohne Rückfrage in je einem Worktree auf dem Sprint-Branch abzuarbeiten; andere
 *  werden genannt, nicht bearbeitet. */
export function promptWorktree(items: Picked[]): string {
  const ok = items.filter((it) => worktreeOk(it.session));
  const skipped = items.filter((it) => !worktreeOk(it.session));
  return (
    `Arbeite ${these(ok.length)} autonom und ohne Rückfragen in einem eigenen Worktree ab:\n${groups(ok)}\n` +
    (skipped.length ? `Nicht im Worktree (nicht autonom · offline oder blockiert), nicht bearbeiten: ${skipped.map((it) => it.session.nr).join(', ')}.\n` : '') +
    'Regeln (docs/arbeitsweise.md › Branches und Autonomer Ablauf):\n' +
    '1. Je Sprint ein Worktree auf dem Sprint-Branch `sprint/<präfix>` (fehlt er: von `origin/develop`). Mit `workbench_status` prüfen, dass k3c-dev den Worktree bedient, sonst Shell-Befehle.\n' +
    '2. Nur offline arbeiten und nur die „Erlaubten Dateien“ der Session ändern: Code, Unit-/Mock-Tests, Doku. Keine laufenden ' +
    'Dienste (`svc_*`), kein Browser, keine Geräte. Braucht eine Session doch Live-Dienste oder eine Entscheidung: ' +
    '`Status: blockiert` mit Grund, Ticket vom Typ Frage, nicht raten und mit der nächsten weitermachen.\n' +
    '3. Reihenfolge wie gelistet. Je Session `check_run task:check` grün, `Status: fertig` per `plan_set`, ein Commit mit der Session im Titel.\n' +
    '4. Zum Schluss `git merge origin/develop` und push auf den Sprint-Branch. Kein PR, außer die letzte Session des Sprints ist dabei.\n' +
    '5. Kurz berichten: erledigt, blockiert (mit Grund), gepushte Branches.'
  );
}

/** Menüeintrag „Autonom im Worktree“ für den Kopierbutton. */
export const worktreeVariant = (items: Picked[]): PromptVariant =>
  ({ label: 'Autonom im Worktree abarbeiten', prompt: promptWorktree(items), blocked: worktreeBlocked(items) });

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
