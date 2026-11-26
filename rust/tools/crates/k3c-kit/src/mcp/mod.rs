//! Serverunabhängige Bausteine der MCP-Server (Port von `workkit/mcpkit`, RU66.3): Zähler und Aufruf-Log, Text-Hilfen,
//! Zuordnung zum Checkout (Header, Argument `checkout`, Worktrees), Pfadprüfung und Schreibschutz. Server, Middleware
//! und Fortschritt folgen mit RU66.4.

mod checkout;
mod paths;
mod stats;
mod text;
mod workspace;
mod writeguard;

pub use checkout::{Resolver, split_checkout, with_checkout};
pub use paths::inside_root;
pub use stats::{Call, CallHook, Outcome, Snapshot, Stats, TargetFn, ToolStats};
pub use text::{clip, clip_args, or_dash, outcome_of, ref_name, result_text, text_result};
pub use workspace::{Workspace, find_root, same_dir, same_path};
pub use writeguard::WriteGuard;

#[cfg(all(test, windows))]
#[path = "canon_tests.rs"]
mod canon_tests;

/// Name des Arguments, mit dem ein Aufruf seinen Checkout nennt (B-388).
pub const CHECKOUT_ARG: &str = "checkout";
/// Kopfzeile mit dem Arbeitsverzeichnis des Clients; `.mcp.json` setzt sie per `headersHelper`.
pub const HEADER_ROOT: &str = "X-K3C-Root";
/// Grenze der Argumente im eigenen Log.
pub const ARGS_RUNES: usize = 200;
/// Grenze für Argumente, Fehler und Zusammenfassungen im Aufruf-Log.
pub const TEXT_RUNES: usize = 120;
/// Länge des Aufruf-Logs.
pub const CALL_RING_SIZE: usize = 200;
/// Steht im Ergebnis, wenn ein Handler in Panik gerät; der Server läuft weiter.
pub const PANIC_TEXT: &str = "Tool-Aufruf durch Panik abgebrochen";
