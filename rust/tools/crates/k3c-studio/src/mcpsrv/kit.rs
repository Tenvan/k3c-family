//! Dünne Anbindung an das Gerüst `k3c_kit::mcp` (Port von `mcpsrv/kit.go`). `ws` und `add` aus Go entfallen: die
//! Tools lesen den Checkout als `Extension<Workspace>` und registrieren sich mit den Makros von `rmcp`.

pub use k3c_kit::logger::Logger;
/// Ein Eintrag im Aufruf-Log, alle Zähler, Kopfzeile des Clients und Suche der Repo-Wurzel.
pub use k3c_kit::mcp::{Call, HEADER_ROOT, Snapshot, find_root};
/// Checkout eines Aufrufs, Server und seine Optionen.
pub use k3c_kit::mcp::{Options, Server as KitServer, Workspace, resolve_port as resolve_port_env};
pub use k3c_kit::usage::Tracker;
