//! Lese-Typen der Berichte von `k3c-game sim balance` (RS2.2, B-726): Feldnamen, Reihenfolge, `omitempty` und `null`
//! wie Go, weil beide Studios dieselben Dateien unter `reports/` lesen und schreiben.
//!
//! Gos `nil`-Listen stehen als `null` in der Datei: Listen, die Go nur durch `append` füllt, schreibt [`nil_empty`].
//! Gelesen wird über [`read_go`] (`null` und fehlende Felder sind leer).

use std::path::Path;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;

use super::datafiles;

/// Eine leere Liste als `null` (Go `nil`-Slice).
pub(crate) fn nil_empty<T: Serialize, S: Serializer>(list: &[T], s: S) -> std::result::Result<S::Ok, S::Error> {
    if list.is_empty() {
        s.serialize_none()
    } else {
        list.serialize(s)
    }
}

/// Go `omitempty` einer Zahl.
pub(crate) fn is_zero(n: &i64) -> bool {
    *n == 0
}

/// Entfernt `null`-Felder, die Go beim Lesen wie fehlende behandelt.
fn drop_nulls(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|_, v| !v.is_null());
            map.values_mut().for_each(drop_nulls);
        }
        Value::Array(list) => list.iter_mut().for_each(drop_nulls),
        _ => {}
    }
}

/// Liest JSON annähernd wie Go `json.Unmarshal`: fehlende und `null`-Felder sind leer, unbekannte zählen nicht
/// (Feldnamen anders als in Go mit Groß- und Kleinschreibung).
pub(crate) fn read_go<T: DeserializeOwned + Default>(raw: &[u8]) -> std::result::Result<T, String> {
    let mut value: Value = serde_json::from_slice(raw).map_err(|e| e.to_string())?;
    if value.is_null() {
        return Ok(T::default());
    }
    drop_nulls(&mut value);
    serde_json::from_value(value).map_err(|e| e.to_string())
}

/// Vorrat an Baumaterial.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Stock {
    pub wood: i64,
    pub stone: i64,
    pub copper: i64,
    #[serde(skip_serializing_if = "is_zero")]
    pub iron: i64,
    #[serde(skip_serializing_if = "is_zero")]
    pub crystal: i64,
}

/// Kosten in Gold und Baumaterial; Go hat keine JSON-Namen (geschrieben groß, gelesen auch klein).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Cost {
    #[serde(alias = "gold")]
    pub gold: i64,
    #[serde(alias = "wood")]
    pub wood: i64,
    #[serde(alias = "stone")]
    pub stone: i64,
    #[serde(alias = "copper")]
    pub copper: i64,
    #[serde(alias = "iron")]
    pub iron: i64,
    #[serde(alias = "crystal")]
    pub crystal: i64,
}

/// Kennzahlen eines Laufs. Ticks zählen ab 1; `None`, solange es den Zeitpunkt im Lauf nicht gab.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Metrics {
    /// Erster Tick, an dem das Gold aller Monarchen zusammen die Schwelle erreicht.
    pub gold_threshold_tick: Option<i64>,
    pub first_wall_tick: Option<i64>,
    /// Erstes Ereignis `armed`; Start-Bogenschützen zählen nicht.
    pub first_bow_tick: Option<i64>,
    pub castle_fall_tick: Option<i64>,
    /// `crown`: Lauf durch Kronenraub verloren.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lost: String,
    pub first_workshop_tick: Option<i64>,
    pub first_tower_tick: Option<i64>,
    pub hub_level2_tick: Option<i64>,
    pub hub_level3_tick: Option<i64>,
    /// Tiefste Stufe, auf der am Ende ein Monarch steht.
    pub reached_depth: i64,
    #[serde(serialize_with = "nil_empty")]
    pub days: Vec<DayMetrics>,
    #[serde(serialize_with = "nil_empty")]
    pub waves: Vec<WaveMetrics>,
    #[serde(serialize_with = "nil_empty")]
    pub dawns: Vec<DawnMetrics>,
    #[serde(flatten)]
    pub island: IslandMetrics,
}

/// Stand zu Tagesbeginn (BAL6): Tag 1 ist der Start, jeder weitere das Ereignis `dawn`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DawnMetrics {
    pub day: i64,
    pub tick: i64,
    /// Gold je Monarch in der Reihenfolge der Spieler.
    #[serde(serialize_with = "nil_empty")]
    pub gold: Vec<i64>,
    pub stock: Stock,
    pub hub_level: i64,
    pub fighters: i64,
}

/// Stand zu Dämmerungsbeginn und Gold-Fluss eines Tags (Morgen bis Morgen).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DayMetrics {
    pub day: i64,
    pub dusk_tick: Option<i64>,
    pub gold_at_dusk: Option<i64>,
    pub stock_at_dusk: Option<Stock>,
    pub income: i64,
    pub expense: i64,
    pub night_income: i64,
    pub night_expense: i64,
    pub stock_in: Stock,
    pub stock_out: Stock,
    pub castle_fallen: bool,
    pub peasant_ticks: i64,
    pub peasant_idle_ticks: i64,
}

/// Eine Welle vom Ereignis `wave` bis zum nächsten `dawn` (oder zur nächsten Welle).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct WaveMetrics {
    pub wave: i64,
    pub start_tick: i64,
    /// `null`: Lauf endete in der Welle.
    pub end_tick: Option<i64>,
    pub enemies: i64,
    pub troops_at_start: i64,
    pub troops_lost: i64,
    pub buildings_destroyed: i64,
    pub players_down: i64,
    pub survived: bool,
}

/// Eine Stufe der Insel (Reihenfolge = Stufenindex).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct StageMetrics {
    pub depth: i64,
    pub arrival_day: Option<i64>,
    pub arrival_tick: Option<i64>,
    pub castle_fall_tick: Option<i64>,
    #[serde(skip_serializing_if = "is_zero")]
    pub downs: i64,
    #[serde(skip_serializing_if = "is_zero")]
    pub hub_level: i64,
    #[serde(skip_serializing_if = "is_zero")]
    pub fighters: i64,
}

/// Ein besiegter oder erschienener Boss mit Tag und Tick.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BossMetrics {
    pub boss: String,
    pub day: i64,
    pub tick: i64,
}

/// Die Felder, die nur ein Insel-Lauf füllt.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct IslandMetrics {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<StageMetrics>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bosses: Vec<BossMetrics>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub boss_spawns: Vec<BossMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endboss_day: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endboss_tick: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub victory_tick: Option<i64>,
    /// Je Schiff-Abschnitt der Tag, ab dem er bezahlbar gewesen wäre.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ship: Vec<ShipNeed>,
}

/// Alle Läufe: jedes Paar aus Seed × Spieleranzahl × Bot × Tiefe läuft `days` Tage.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Matrix {
    #[serde(serialize_with = "nil_empty")]
    pub seeds: Vec<String>,
    #[serde(serialize_with = "nil_empty")]
    pub players: Vec<i64>,
    #[serde(serialize_with = "nil_empty")]
    pub bots: Vec<String>,
    #[serde(serialize_with = "nil_empty")]
    pub depths: Vec<i64>,
    pub days: i64,
    pub gold_threshold: i64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub grade: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub world: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub helps: String,
}

/// Ein einzelner Lauf.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Scenario {
    pub seed: String,
    pub players: i64,
    pub bot: String,
    pub depth: i64,
    pub days: i64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub grade: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub world: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub helps: String,
}

/// Ein Lauf mit Kennzahlen (Go `Result`); ein ungültiger Lauf hat `error` statt `metrics`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RunResult {
    #[serde(flatten)]
    pub scenario: Scenario,
    pub valid: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
    pub ticks: i64,
    pub metrics: Option<Metrics>,
}

/// Die Ausgabe: die Matrix und alle Läufe in der Reihenfolge der Matrix.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Report {
    pub matrix: Matrix,
    #[serde(serialize_with = "nil_empty")]
    pub results: Vec<RunResult>,
}

/// Ein Abschnitt aus `data/ship.json`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ShipStage {
    pub id: String,
    pub name: String,
    pub cost: Cost,
}

/// Frühester Tag, an dem ein Abschnitt (samt denen davor) bezahlbar gewesen wäre; `null`: nie im Lauf.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ShipNeed {
    pub id: String,
    pub affordable_day: Option<i64>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct ShipFile {
    stages: Vec<ShipStage>,
}

/// Liest die Abschnitte aus `data/ship.json` (BS3.2).
pub fn load_ship(root: &Path) -> std::result::Result<Vec<ShipStage>, String> {
    datafiles::read(root, "ship.json")
        .and_then(|raw| read_go::<ShipFile>(&raw))
        .map(|f| f.stages)
        .map_err(|e| format!("ship.json: {e}"))
}
