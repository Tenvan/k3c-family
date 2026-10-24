import type { TicketBackend, TicketCreated } from './types';

let ticketNo = 900;

/** Mock-Tickets: nur im Speicher, ab B-900 (WZ28.3). */
export function mockTickets(): Pick<TicketBackend, 'ticketCreate' | 'ticketRecent'> {
  const made: TicketCreated[] = [];
  return {
    ticketCreate: async (input) => {
      const id = `B-${ticketNo++}`;
      const created = { id, pfad: `docs/backlog/${id}-mock.md`, titel: input.titel };
      made.unshift(created);
      return created;
    },
    ticketRecent: async (n) => made.slice(0, n > 0 ? n : 5),
  };
}
