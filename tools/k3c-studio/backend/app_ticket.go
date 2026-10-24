package backend

import "k3c/tools/assetkit/ticket"

// Ticket-Button der Oberfläche (WZ28.3): legt Tickets im Repo an und nennt die zuletzt angelegten.

// TicketCreate legt ein Ticket mit der nächsten freien Nummer an und trägt es in den Backlog-Index ein.
func (a *App) TicketCreate(in ticket.Input) (ticket.Created, error) {
	return ticket.Create(a.root, in)
}

// TicketRecent nennt die n zuletzt angelegten Tickets (n <= 0: 5).
func (a *App) TicketRecent(n int) ([]ticket.Created, error) {
	if n <= 0 {
		n = 5
	}
	return ticket.Recent(a.root, n)
}
