import type { PlanSession, PlanSprint, PlanTicket } from '../api';
import type { Tone } from '../ui/parts';

// Reine Funktionen der Planungs-Seite: Fortschritt eines Sprints, Zustand → Farbe, Tickets filtern und nach Domäne
// gruppieren.

export interface Progress {
  done: number;
  total: number;
}

/** Erledigte Sessions eines Sprints; Entwürfe zählen mit, damit ein geplanter Sprint 0 von n zeigt. */
export function progress(s: PlanSprint): Progress {
  return { done: s.sessions.filter((x) => x.status === 'fertig').length, total: s.sessions.length };
}

/** Farbe einer Session: fertig grün, in Arbeit blau, blockiert rot, offen/Entwurf neutral. */
export function sessionTone(x: PlanSession): Tone {
  switch (x.status) {
    case 'fertig':
      return 'ok';
    case 'in Arbeit':
      return 'info';
    case 'blockiert':
      return 'error';
    default:
      return 'neutral';
  }
}

/** Die erste offene Session, die ein Agent nehmen darf: Status offen, Agent autonom (Arbeitsweise › Autonomer Ablauf). */
export function nextSession(s: PlanSprint): PlanSession | undefined {
  return s.sessions.find((x) => x.status === 'offen' && x.agent === 'autonom');
}

export interface TicketFilter {
  query: string;
  status: string; // 'alle' oder ein Status
  domain: string; // 'alle' oder ein Kürzel
}

export function filterTickets(list: PlanTicket[], f: TicketFilter): PlanTicket[] {
  const q = f.query.trim().toLowerCase();
  return list.filter(
    (t) =>
      (f.status === 'alle' || t.status === f.status) &&
      (f.domain === 'alle' || t.domain === f.domain) &&
      (q === '' || t.nr.toLowerCase().includes(q) || t.title.toLowerCase().includes(q)),
  );
}

const PRIO = ['hoch', 'mittel', 'niedrig'];

/** Gruppen nach Domäne (alphabetisch), darin Prio hoch → niedrig, dann Nummer. */
export function groupByDomain(list: PlanTicket[]): { domain: string; tickets: PlanTicket[] }[] {
  const by = new Map<string, PlanTicket[]>();
  for (const t of list) by.set(t.domain || '–', [...(by.get(t.domain || '–') ?? []), t]);
  const rank = (t: PlanTicket) => (PRIO.indexOf(t.prio) === -1 ? PRIO.length : PRIO.indexOf(t.prio));
  return [...by.entries()]
    .sort(([a], [b]) => (a < b ? -1 : 1))
    .map(([domain, tickets]) => ({ domain, tickets: tickets.sort((a, b) => rank(a) - rank(b) || (a.nr < b.nr ? -1 : 1)) }));
}
