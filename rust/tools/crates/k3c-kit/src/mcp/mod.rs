//! Gemeinsames Gerüst der MCP-Server von Workbench und Studio (Port von `workkit/mcpkit`): [`Server`] auf `rmcp`
//! (Streamable HTTP nur an 127.0.0.1), Zähler und Aufruf-Log, Zuordnung zum Checkout (Header, Argument `checkout`,
//! Worktrees), Parameter-Hinweis, Fortschritt, Pfadprüfung und Schreibschutz.
//!
//! Die Tools schreiben die Apps mit `#[tool_router]` und `#[tool]` auf ihrem eigenen Typ; den Checkout und den
//! Fortschritt eines Aufrufs lesen sie als `Extension<Workspace>` und `Extension<Progress>`.

mod checkout;
mod chunked;
mod observe;
mod param_hint;
mod paths;
mod progress;
mod server;
mod service;
mod stats;
mod text;
mod tool;
mod workspace;
mod writeguard;

pub use checkout::{Resolver, split_checkout, with_checkout};
pub use paths::inside_root;
pub use progress::{Progress, ProgressFn};
pub use server::{MountFn, Options, Server, StopFn, ToolPredicate, TrackFn, resolve_port};
pub use stats::{Call, CallHook, Outcome, Snapshot, Stats, TargetFn, ToolStats};
pub use text::{clip, clip_args, or_dash, outcome_of, ref_name, result_text, text_result};
pub use tool::{image_result, read_only};
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
