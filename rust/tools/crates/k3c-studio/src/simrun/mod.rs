//! Laufsteuerung der Testläufe (Port von `tools/k3c-studio/internal/simrun`, RU70.18 und RU70.19; B-348, WZ28.4): ein
//! Register je Studio für `sim_test` (MCP) und die Balancing-Seite, mit höchstens [`MAX_PER_ROOT`] gleichzeitigen
//! Läufen je Checkout und einer Warteschlange dahinter. Der Runner eines Laufs ist eine Funktion; den Offline-Runner
//! (Spiel-EXE `k3c-game sim balance`) liefert [`offline`], die Online-Runner bleiben in `mcpsrv`.
//!
//! Go `registry.go` ist auf [`registry`] (Register) und [`run`] (Lauf, Abbruch-Token) verteilt.
//!
//! [`offline`], [`reports`], [`reportlist`] und [`manage`] sind aus RU70.19 vorgezogen, soweit die Tests aus RU70.18
//! sie brauchen (V11); der Rest sind Stubs.

pub mod baseline;
pub mod info;
pub mod manage;
pub mod notify;
pub mod offline;
pub mod pervariant;
pub mod registry;
pub mod reportlist;
pub mod reports;
pub mod run;
pub mod spec;
pub mod stamp;

#[cfg(test)]
mod test_helpers;

pub use info::Info;
pub use notify::Notify;
pub use offline::{Exec, Proc, ProcResult};
pub use registry::{MAX_PER_ROOT, Registry, RegistryError, report_dir};
pub use run::{CANCELED, CancelToken, DONE, FAILED, RUNNING, Run, Runner, WAITING};
pub use spec::{Spec, Tune};
pub use stamp::Stamp;
