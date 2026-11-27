//! Leser der JSON-Logs unter `logs/*.jsonl` (Port von `internal/logs`): eine Zeile je Eintrag im Format von `log/slog`
//! mit `time`, `level`, `msg`, optional `ns` und weiteren Feldern als Daten. Gelesen wird von hinten und unter einem
//! Byte-Budget, damit auch große Dateien schnell antworten.

mod digest;
mod entry;
mod reader;

#[cfg(test)]
mod applog_tests;
#[cfg(test)]
mod entry_tests;
#[cfg(test)]
mod logs_tests;

pub use digest::{Group, digest, fingerprint};
pub use entry::{Entry, LEVELS, level_rank, parse_rfc3339};
pub use reader::{DEFAULT_BUDGET, Query, ScanResult, SinceResult, read_since, scan};
