// Ticket-Baustein (B-595): Typen und Listen zwischen Oberfläche und Backend (JSON wie im Vertrag der Session WZ28.2).

export const TICKET_TYPES = ['Idee', 'Problem', 'Frage'] as const;
export const TICKET_PRIOS = ['hoch', 'mittel', 'niedrig'] as const;
export const TICKET_DOMAINS = ['REG', 'SIM', 'SRV', 'CLI', 'PLAT', 'INF', 'DEV'] as const;

export type TicketType = (typeof TICKET_TYPES)[number];
export type TicketPrio = (typeof TICKET_PRIOS)[number];
export type TicketDomain = (typeof TICKET_DOMAINS)[number];

/** Wo das Ticket entstand; `auswahl` und `version` dürfen leer sein, `zeit` ist ISO 8601. */
export interface TicketContextInfo {
  app: string;
  seite: string;
  auswahl: string;
  version: string;
  zeit: string;
}

/** Seite und Auswahl, die die sichtbare Seite meldet (useTicketPage). */
export interface TicketPageContext {
  page: string;
  selection: string;
}

export interface TicketInput {
  titel: string;
  beschreibung: string;
  typ: TicketType;
  prio: TicketPrio;
  domaene: TicketDomain;
  /** Projekt-Kürzel (z. B. WZG); leer = kein Projekt. */
  projekt?: string;
  kontext: TicketContextInfo;
}

export interface TicketCreated {
  id: string;
  pfad: string;
  titel: string;
}
