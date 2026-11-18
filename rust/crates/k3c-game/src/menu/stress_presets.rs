//! Port von `native/k3c-game/menu/stress_presets.go`: Vorgaben der Seite Testing (B-896) aus `data/stresstests.json`:
//! Werte wie `view::stress::StressTest`, dazu der Name je Sprache. Fehlt die Datei oder ist sie kaputt, gibt es keine
//! Vorgaben und eine Meldung (die Seite bleibt bedienbar). Die Datei liest `k3c_data::stresstests`.

use k3c_data::{Fehler, Source, stresstests};

use crate::view::settings::LANGUAGE_ENGLISH;
use crate::view::stress::StressTest;

/// Eine Vorgabe: Werte und Name (deutsch, englisch).
#[derive(Clone, Debug, PartialEq)]
pub struct StressPreset {
    pub stress: StressTest,
    pub name: String,
    pub name_en: String,
}

impl StressPreset {
    /// Der Name der Vorgabe in der Sprache `lang`.
    pub fn name(&self, lang: &str) -> &str {
        if lang == LANGUAGE_ENGLISH && !self.name_en.is_empty() {
            &self.name_en
        } else {
            &self.name
        }
    }
}

/// Liest die Vorgaben aus `raw`; jede Vorgabe ist normalisiert, ohne Id oder Name verworfen.
pub fn parse_stress_presets(raw: &[u8]) -> Result<Vec<StressPreset>, Fehler> {
    let text = std::str::from_utf8(raw).map_err(|e| Fehler::Json(format!("data/{}: {e}", stresstests::FILE)))?;
    let file = stresstests::load(text)?;
    Ok(file
        .presets
        .into_iter()
        .filter(|p| !p.id.is_empty() && !p.name.is_empty())
        .map(|p| StressPreset {
            stress: StressTest {
                preset: p.id,
                players: p.players,
                depth: p.depth,
                seed: p.seed,
                enemies: p.enemies,
                growth: p.growth,
                factor: p.factor,
                build: p.build,
                troops: p.troops,
                timescale: p.timescale,
                waves: p.waves,
                minutes: p.minutes,
                enemy: p.enemy,
                troop: p.troop,
                pure: false,
            }
            .normalized(),
            name: p.name,
            name_en: p.name_en,
        })
        .collect())
}

/// Liest `data/stresstests.json`; ein Fehler steht im Log und kommt als `Err` zurück.
pub fn load_stress_presets() -> Result<Vec<StressPreset>, Fehler> {
    let res = Source::from_env()
        .read(stresstests::FILE)
        .and_then(|raw| parse_stress_presets(raw.as_bytes()));
    if let Err(e) = &res {
        k3c_platform::log::warn(
            "💥 Stresstest-Vorgaben nicht lesbar",
            &[("datei", &stresstests::FILE), ("err", e)],
        );
    }
    res
}
