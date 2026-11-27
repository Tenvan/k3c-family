//! Die Asset-Schicht der MCP-Tools (Port von `internal/paperassets`; Vertrag docs/standards/paper-mcp.md,
//! paper-spec.md): Aufruf der Render-CLI (gemeinsam mit `review` über `review::aufruf`), JSON-Merge-Patch, atomares
//! Schreiben einer Spec, Git-Basis und Render-Jobs. `K3C_PAPER_CLI` zeigt in Tests auf die Attrappe
//! `tools/paper/fixtures/cli-stub.ts`. Alle Aufrufe blockieren; async-Aufrufer nutzen `spawn_blocking`.

mod anlegen;
mod basis;
mod batch;
mod cli;
mod golden;
mod gueltig;
mod job;
mod patch;
mod reiter;
mod rigs;
mod set;
mod set_plan;
mod stil;
mod teile;

#[cfg(test)]
pub(crate) mod test_helpers;
#[cfg(test)]
#[path = "paperassets_tests.rs"]
mod tests;

use std::fmt;
use std::io;

pub use anlegen::{AnlegenErgebnis, LoeschErgebnis, anlegen, ausgabe_pngs, loesche};
pub use basis::{Asset, GRAFIK, MehrdeutigFehler, STIL_PFAD, assets, check_basis, finde, geaendert};
pub use batch::{Schritt, SchrittErgebnis};
pub use cli::{Cli, DiffErgebnis, FrameArgs, FrameErgebnis, PixelErgebnis, RenderErgebnis};
pub use gueltig::Gueltigkeit;
pub use job::{AssetErgebnis, Jobs, Renderer, Stand, render_eins};
pub use patch::{felder, format, merge_patch, schluessel};
pub use reiter::reiter_von;
pub use rigs::{Rig, finde_rig, rig_namen, vorlage};
pub use set::{Pixel, SetErgebnis, SetFehler, schreibe, set, set_overlay, temp_ordner};
pub use stil::{StilErgebnis, feld_diff, materialien, stil_hash, stil_set, wert};

/// Eine verletzte Regel `{feld, regel, text}` (paper-spec.md › Regeln).
pub use crate::review::{Fehler, FeldAenderung};

/// Fehler der Asset-Schicht.
#[derive(Debug, Clone)]
pub enum Error {
    /// Fachlich abgelehnt (Exit 1 der CLI oder eigene Regel); er ändert nichts.
    Abgelehnt(Vec<Fehler>),
    /// Die ID ohne Gruppe kommt in mehreren Gruppen vor.
    Mehrdeutig(MehrdeutigFehler),
    /// Alles andere, als Satz für die Antwort.
    Sonst(String),
}

impl Error {
    /// Die verletzten Regeln, wenn der Fehler eine Ablehnung ist.
    pub fn abgelehnt(&self) -> Option<&[Fehler]> {
        match self {
            Error::Abgelehnt(fehler) => Some(fehler),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Abgelehnt(fehler) => {
                let teile: Vec<String> = fehler
                    .iter()
                    .map(|e| format!("{} · {} · {}", e.feld, e.regel, e.text))
                    .collect();
                write!(f, "abgelehnt: {}", teile.join("; "))
            }
            Error::Mehrdeutig(m) => m.fmt(f),
            Error::Sonst(text) => f.write_str(text),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Sonst(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Sonst(e.to_string())
    }
}

impl From<crate::review::Error> for Error {
    fn from(e: crate::review::Error) -> Self {
        match e {
            crate::review::Error::Abgelehnt(fehler) => Error::Abgelehnt(fehler),
            e => Error::Sonst(e.to_string()),
        }
    }
}
