//! Die eingebetteten Daten als Rückfall für Welten ohne eigene Regeln und für die Kampagne.
//!
//! Gos Paket-Variablen `biomes`, `globalDayNight` und die Regeln des Level-Generators (`guardRules`, `ruinRules` …) gibt es
//! in Rust nicht. Wer eine Stufe mit eigenen Regeln baut (`Options::rules`), reicht sie durch; sonst gelten diese
//! eingebetteten Daten, einmal gelesen und danach unveränderlich. Die Kampagne (Altpfad vor den Inseln) rechnet mit ihnen
//! wie Go mit den Paket-Variablen.

use std::sync::{Arc, OnceLock};

use k3c_data::biome::Cycle;
use k3c_data::{Data, Source, load_all};
use k3c_level::rules::Rules;

use crate::state::data::first_day_night;

/// Die eingebetteten Daten samt Regelsatz und globalem Zyklus.
#[derive(Debug)]
pub struct Base {
    pub data: Data,
    pub rules: Arc<Rules>,
    /// Go: `globalDayNight`, der Zyklus des ersten Tag/Nacht-Bioms (Oberwelt).
    pub day_night: Cycle,
}

fn load() -> Result<Base, String> {
    let data = load_all(&Source::Embedded).map_err(|e| e.to_string())?;
    let hub = Source::Embedded.read("hub.json").map_err(|e| e.to_string())?;
    let rules = Rules::from_data(&data, &hub).map_err(|e| e.to_string())?;
    let day_night = first_day_night(&data.biomes);
    Ok(Base {
        data,
        rules: Arc::new(rules),
        day_night,
    })
}

/// Die eingebetteten Daten, einmal gelesen. Ein Fehler ist wegen der Prüfung in `k3c-data` nicht zu erwarten und wird
/// bei jedem Aufruf gemeldet.
pub fn base() -> Result<&'static Base, String> {
    static BASE: OnceLock<Result<Base, String>> = OnceLock::new();
    BASE.get_or_init(load).as_ref().map_err(Clone::clone)
}
