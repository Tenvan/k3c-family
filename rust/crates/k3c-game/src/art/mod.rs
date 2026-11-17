//! Package art lädt die Grafik des Spiels: Papier-Sheets je Level (public/grafik über data/paper.json), die
//! Rig-Bibliothek (native/k3c-rig, ab RU7.6) und die Schrift. Fachmodule holen hier ihre Bilder; kein Fachmodul liest
//! Dateien selbst. Vorlage: `native/k3c-game/art/`.
//!
//! Aufbau: Planen (Pfade, Maße, Texturbudget; ohne GPU, testbar) getrennt vom Hochladen (`Canvas::texture`).

mod budget;
mod datafile;
mod font;
#[cfg(test)]
mod font_tests;
mod library;
#[cfg(test)]
mod library_tests;
mod manifest;
#[cfg(test)]
mod manifest_tests;
mod outline;
#[cfg(test)]
mod outline_tests;
mod plan;
#[cfg(test)]
mod plan_tests;
mod png;
mod rigs;
#[cfg(test)]
mod rigs_tests;
mod scales;
mod seamap;
#[cfg(test)]
mod seamap_tests;
mod sheet;
#[cfg(test)]
mod testutil;
mod voyage;
#[cfg(test)]
mod voyage_tests;
mod worldimages;
#[cfg(test)]
mod worldimages_tests;

use std::fmt;

pub use budget::Budget;
pub use font::{Font, LINE_SPACING, load_font};
pub use library::{
    Config, DEFAULT_FONT_PATH, DEFAULT_GRAFIK_DIR, DEFAULT_RIG_DIR, LevelLoad, Levels, Library, SheetKey, budget_of,
    default_config, diff_sheets, plan_levels,
};
pub use manifest::{Anim, Manifest, group_assets, read_manifest};
pub use outline::{OUTLINE4, OUTLINE8, OutlineCache, outline_pad, outline_stroke, scale_alpha};
pub use plan::{Plan, SheetSpec, plan_level, plan_sheet};
pub use rigs::{Actor, M, Painter, Point, State, actor_frame, rig_view};
pub use scales::{data_scales, parse_scales};
pub use seamap::{SeaLayers, SeaMap, SeaMarks};
pub use sheet::{Sheet, SheetFrame, upload};
pub use voyage::{VoyageLook, parse_voyage, voyage};
pub use worldimages::{STANDARD_WORLD, WorldImage, WorldImages};

/// Fehler beim Laden; die Meldung nennt immer den Pfad. `Missing` entspricht Gos `ErrMissing`.
#[derive(Debug)]
pub enum ArtError {
    Missing(String),
    Broken(String),
    /// Mehrere Fehler wie `errors.Join` (Meldungen durch Zeilenumbruch getrennt).
    Joined(Vec<ArtError>),
}

impl ArtError {
    /// Fasst Fehler zusammen wie `errors.Join`: keiner → `None`.
    pub fn join(errs: Vec<ArtError>) -> Option<ArtError> {
        if errs.is_empty() {
            None
        } else {
            Some(Self::Joined(errs))
        }
    }

    /// Wie `errors.Is(err, ErrMissing)`: wahr, wenn dieser oder ein enthaltener Fehler fehlende Grafik meldet.
    pub fn is_missing(&self) -> bool {
        match self {
            Self::Missing(_) => true,
            Self::Broken(_) => false,
            Self::Joined(errs) => errs.iter().any(Self::is_missing),
        }
    }
}

impl fmt::Display for ArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(m) => write!(f, "fehlende Grafik: {m}"),
            Self::Broken(m) => write!(f, "{m}"),
            Self::Joined(errs) => {
                for (i, e) in errs.iter().enumerate() {
                    if i > 0 {
                        writeln!(f)?;
                    }
                    write!(f, "{e}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ArtError {}
