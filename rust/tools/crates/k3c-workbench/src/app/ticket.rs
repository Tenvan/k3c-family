//! Ticket-Button der Oberfläche (Port von `app_ticket.go`, WZ28.3) auf `k3c_assetkit::ticket`: legt Tickets im Repo an
//! und nennt die zuletzt angelegten.

use k3c_assetkit::ticket::{self, Created, Input};

use super::App;

/// So viele Tickets nennt [`App::ticket_recent`] ohne Angabe.
const RECENT: usize = 5;

#[cfg(test)]
#[path = "ticket_tests.rs"]
mod tests;

impl App {
    /// Legt ein Ticket mit der nächsten freien Nummer an und trägt es in den Backlog-Index ein.
    pub fn ticket_create(&self, input: Input) -> Result<Created, String> {
        ticket::create(&self.root, input).map_err(|e| e.to_string())
    }

    /// Die `n` zuletzt angelegten Tickets (`n <= 0`: 5).
    pub fn ticket_recent(&self, n: i64) -> Result<Vec<Created>, String> {
        let n = usize::try_from(n).ok().filter(|n| *n > 0).unwrap_or(RECENT);
        ticket::recent(&self.root, n).map_err(|e| e.to_string())
    }
}
