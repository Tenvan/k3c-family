import { createContext, useContext, useEffect, useMemo, useState, type ReactNode } from 'react';
import type { TicketCreated, TicketDomain, TicketInput, TicketPageContext } from './ticket';

export interface TicketApi {
  app: string;
  version: string;
  defaultDomain: TicketDomain;
  defaultProject?: string;
  create: (input: TicketInput) => Promise<TicketCreated>;
  recent?: () => Promise<TicketCreated[]>;
  /** Seite und Auswahl der aktuell sichtbaren Seite. */
  page: TicketPageContext;
  setPage: (page: TicketPageContext) => void;
}

const TicketContext = createContext<TicketApi | null>(null);

interface ProviderProps {
  app: string;
  version?: string;
  defaultDomain: TicketDomain;
  defaultProject?: string;
  create: (input: TicketInput) => Promise<TicketCreated>;
  recent?: () => Promise<TicketCreated[]>;
  children: ReactNode;
}

/** Hält App, Version und die Backend-Aufrufe des Ticket-Bausteins sowie Seite und Auswahl der sichtbaren Seite. */
export function TicketProvider({ app, version = '', defaultDomain, defaultProject, create, recent, children }: ProviderProps) {
  const [page, setPage] = useState<TicketPageContext>({ page: '', selection: '' });
  const value = useMemo<TicketApi>(
    () => ({ app, version, defaultDomain, defaultProject, create, recent, page, setPage }),
    [app, version, defaultDomain, defaultProject, create, recent, page],
  );
  return <TicketContext.Provider value={value}>{children}</TicketContext.Provider>;
}

/** Zugriff auf den Ticket-Baustein; nur unterhalb von TicketProvider. */
export function useTicket(): TicketApi {
  const ctx = useContext(TicketContext);
  if (!ctx) throw new Error('useTicket braucht einen TicketProvider');
  return ctx;
}

/** Meldet Seite und Auswahl der sichtbaren Seite an den Provider (Kontext der Tickets). */
export function useTicketPage(page: string, selection = ''): void {
  const { setPage } = useTicket();
  useEffect(() => setPage({ page, selection }), [setPage, page, selection]);
}
