//! Zielkorridore (Port von `engine/balance/targets.go`, B-157, BAL2.1): Die Grenzen stehen in data/balance-targets.json. Jedes
//! Ziel nennt eine Kennzahl (Tabelle `measures`) und ein Szenario; ein Ziel ohne bekannte Kennzahl ist ein Ladefehler.
//!
//! Die Typen `Targets` und `Target` liegen in `k3c_data::balance_targets` (dort auch Version, Seeds, Gruppen und Rollen
//! beim Laden); hier kommen die Kennzahlen-Tabelle und die Prüfung der Ziele dazu, die `engine/balance` kennt (Kennzahl und
//! Art, Bot, Grad, Welt, Dev-Hilfen; `scope::validate_target`).

use k3c_data::balance_targets::{FILE, Targets};

use crate::economy_measures::{ECONOMY_MEASURES, MeasureKind, b2f, economy_measure};
use crate::fehler::Fehler;
use crate::island_world::shared;
use crate::metrics::Metrics;
use crate::scope::validate_target;

/// Die Art „Anteil“ eines Ziels (Go: `KindShare`).
pub const KIND_SHARE: &str = "share";
/// Die Art „Median“ eines Ziels (Go: `KindMedian`).
pub const KIND_MEDIAN: &str = "median";

/// Die Kennzahl-IDs aus `targets.go`; die Wirtschafts-Kennzahlen stehen in `economy_measures.rs`.
const BASE_MEASURES: [(&str, MeasureKind); 6] = [
    ("castleHeld", MeasureKind::Share),
    ("firstWallBeforeDusk1", MeasureKind::Share),
    ("endbossDefeated", MeasureKind::Share),
    ("endbossDay", MeasureKind::Median),
    ("caveBeforeDay6", MeasureKind::Share),
    ("destroyedPerWave", MeasureKind::Median),
];

/// Die Art der Kennzahl `key` aus der Tabelle `measures`; `None` für eine unbekannte.
pub fn measure_kind(key: &str) -> Option<MeasureKind> {
    if let Some((_, k)) = BASE_MEASURES.iter().find(|(n, _)| *n == key) {
        return Some(*k);
    }
    if ECONOMY_MEASURES.contains(&key) {
        return economy_measure(key, &Metrics::default()).map(|(k, _)| k);
    }
    None
}

/// Alle Kennzahl-IDs, sortiert (Go: die Schlüssel von `measures`).
pub fn measure_names() -> Vec<&'static str> {
    let mut names: Vec<&str> = BASE_MEASURES.iter().map(|(n, _)| *n).chain(ECONOMY_MEASURES).collect();
    names.sort_unstable();
    names
}

/// `key` ist eine Kennzahl der Tabelle `measures`.
pub fn measure_known(key: &str) -> bool {
    measure_kind(key).is_some()
}

/// Die Messwerte der Kennzahl `key` für einen Lauf (Go: `measures[key].values(m)`); bei einem Anteil genau ein Wert (1 =
/// Bedingung erfüllt, 0 = nicht), bei einem Median beliebig viele (z. B. einer je Welle); ein fremder Schlüssel liefert keinen.
pub fn measure_values(key: &str, m: &Metrics) -> Vec<f64> {
    match key {
        "castleHeld" => castle_held(m),
        "firstWallBeforeDusk1" => {
            // Erste Mauer spätestens zu Dämmerungsbeginn von Tag 1 (Ende der hellen Phase).
            let dusk = m.days.first().and_then(|d| d.dusk_tick);
            vec![b2f(matches!((m.first_wall_tick, dusk), (Some(w), Some(d)) if w <= d))]
        }
        "endbossDefeated" => vec![b2f(m.endboss_tick.is_some())],
        // Ohne Sieg zählt der Tag nach dem Lauf (Tageslimit + 1): ein Lauf ohne Sieg liegt über jeder Obergrenze, statt
        // ungültig zu sein.
        "endbossDay" => vec![m.endboss_day.map_or(m.days.len() as f64 + 1.0, |d| d as f64)],
        "caveBeforeDay6" => {
            let ok = m.stages.get(1).and_then(|s| s.arrival_day).is_some_and(|d| d < 6);
            vec![b2f(ok)]
        }
        // Zerstörte Gebäude je Welle 1–5.
        "destroyedPerWave" => m
            .waves
            .iter()
            .filter(|w| w.wave <= 5)
            .map(|w| w.buildings_destroyed as f64)
            .collect(),
        _ => economy_measure(key, m).map(|(_, v)| v).unwrap_or_default(),
    }
}

/// Das Fenster der Kennzahl castleHeld: Nacht 1 bis 5 (Ziel „Burg hält Nacht 1–5“). Es hängt nicht an `Target.days`, weil
/// ein Lauf über 40 Tage (`sim_test days=40`) die Tage aller Ziele überschreibt.
const CASTLE_HELD_NIGHTS: i64 = 5;

/// B-657: 1, wenn die Burg bis zum Morgen nach Nacht 5 nicht fiel; 0 bei einem Fall davor. Ein Lauf, der diesen Morgen ohne
/// Burgfall nicht erreicht, ist nicht auswertbar (kein Wert statt „verletzt“).
fn castle_held(m: &Metrics) -> Vec<f64> {
    let end = m.dawns.iter().find(|d| d.day == CASTLE_HELD_NIGHTS + 1).map(|d| d.tick);
    let fell = m.castle_fall_tick.is_some_and(|t| end.is_none_or(|e| t < e));
    if !fell && end.is_none() {
        return Vec::new();
    }
    vec![b2f(!fell)]
}

/// Lädt die eingebettete data/balance-targets.json (Go: `DefaultTargets`).
pub fn default_targets() -> Result<Targets, Fehler> {
    let raw = shared()?.source.read(FILE).map_err(|e| Fehler::Daten(e.to_string()))?;
    load_targets(&raw)
}

/// Liest und prüft eine Korridor-Datei (Go: `LoadTargets` samt `checkRoles`): Datei und Gruppen prüft `k3c-data`, jedes Ziel
/// zusätzlich Kennzahl, Art, Bot, Grad, Welt und Hilfen.
pub fn load_targets(raw: &str) -> Result<Targets, Fehler> {
    let t = k3c_data::balance_targets::load_targets(raw).map_err(|e| Fehler::Daten(e.to_string()))?;
    for (i, g) in t.targets.iter().enumerate() {
        validate_target(g)
            .map_err(|e| Fehler::Daten(format!("balance-targets: ziel {} ({:?}): {e}", i + 1, g.name)))?;
    }
    Ok(t)
}
