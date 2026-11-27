//! Zielkorridore (B-157, BAL2.1): Die Grenzen stehen in `data/balance-targets.json`, die Zahlen sind die bestätigten
//! aus `docs/rules/zielkorridore.md`. Jedes Ziel nennt eine Kennzahl und ein Szenario; ein Ziel ohne bekannte Kennzahl
//! ist ein Ladefehler.

use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::curves::grades;
use super::datafiles;
use super::economy_measures::ECONOMY_MEASURES;
use super::groups::{GROUPS, check_frozen, is_group};
use super::names::{check_helps, check_players, known_bot, load_world};
use super::report_types::{Metrics, read_go};
use crate::kitx::go_quoted;

/// Art eines Ziels: Anteil der Läufe mit erfüllter Bedingung (in %).
pub const KIND_SHARE: &str = "share";
/// Art eines Ziels: Median über alle Messwerte.
pub const KIND_MEDIAN: &str = "median";

/// Eine Kennzahl: liest die Messwerte eines Laufs (Anteil: genau ein Wert 1 oder 0; Median: beliebig viele).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Measure {
    pub id: &'static str,
    pub kind: &'static str,
    pub values: fn(&Metrics) -> Vec<f64>,
}

impl Measure {
    pub(crate) const fn new(id: &'static str, kind: &'static str, values: fn(&Metrics) -> Vec<f64>) -> Self {
        Self { id, kind, values }
    }
}

const MEASURES: [Measure; 6] = [
    // Kein castleFallen in Nacht 1 bis 5, unabhängig von der Länge des Laufs (B-657).
    Measure::new("castleHeld", KIND_SHARE, castle_held),
    // Erste Mauer spätestens zu Dämmerungsbeginn von Tag 1.
    Measure::new("firstWallBeforeDusk1", KIND_SHARE, first_wall_before_dusk1),
    // Insel-Läufe (World, BS11).
    Measure::new("endbossDefeated", KIND_SHARE, endboss_defeated),
    Measure::new("endbossDay", KIND_MEDIAN, endboss_day),
    Measure::new("caveBeforeDay6", KIND_SHARE, cave_before_day6),
    // Zerstörte Gebäude je Welle 1–5.
    Measure::new("destroyedPerWave", KIND_MEDIAN, destroyed_per_wave),
];

fn all_measures() -> impl Iterator<Item = Measure> {
    MEASURES.into_iter().chain(ECONOMY_MEASURES)
}

/// Die Kennzahl mit der ID `id`.
pub(crate) fn measure(id: &str) -> Option<Measure> {
    all_measures().find(|m| m.id == id)
}

fn first_wall_before_dusk1(m: &Metrics) -> Vec<f64> {
    let dusk = m.days.first().and_then(|d| d.dusk_tick);
    vec![b2f(
        matches!((m.first_wall_tick, dusk), (Some(wall), Some(dusk)) if wall <= dusk),
    )]
}

fn endboss_defeated(m: &Metrics) -> Vec<f64> {
    vec![b2f(m.island.endboss_tick.is_some())]
}

/// Ohne Sieg zählt der Tag nach dem Lauf: er liegt über jeder Obergrenze, statt ungültig zu sein.
fn endboss_day(m: &Metrics) -> Vec<f64> {
    vec![m.island.endboss_day.map_or(m.days.len() as f64 + 1.0, |d| d as f64)]
}

/// Höhle (Tiefe 1) vor Beginn von Tag 6 erreicht.
fn cave_before_day6(m: &Metrics) -> Vec<f64> {
    let arrival = m.island.stages.get(1).and_then(|s| s.arrival_day);
    vec![b2f(arrival.is_some_and(|d| d < 6))]
}

fn destroyed_per_wave(m: &Metrics) -> Vec<f64> {
    m.waves
        .iter()
        .filter(|w| w.wave <= 5)
        .map(|w| w.buildings_destroyed as f64)
        .collect()
}

/// Fenster der Kennzahl `castleHeld`: Nacht 1 bis 5, unabhängig von der Länge des Szenarios.
const CASTLE_HELD_NIGHTS: i64 = 5;

/// 1, wenn die Burg bis zum Morgen nach Nacht 5 nicht fiel; 0 bei einem Fall davor. Ein Lauf, der diesen Morgen ohne
/// Burgfall nicht erreicht, ist nicht auswertbar (kein Wert, B-657).
fn castle_held(m: &Metrics) -> Vec<f64> {
    // Tick des Morgens nach dem Fenster
    let end = m
        .dawns
        .iter()
        .find(|d| d.day == CASTLE_HELD_NIGHTS + 1)
        .map(|d| d.tick)
        .filter(|t| *t >= 0);
    let fell = m.castle_fall_tick.is_some_and(|fall| end.is_none_or(|end| fall < end));
    if !fell && end.is_none() {
        return Vec::new();
    }
    vec![b2f(!fell)]
}

pub(crate) fn b2f(b: bool) -> f64 {
    if b { 1.0 } else { 0.0 }
}

/// Breite von „knapp“ (Standardwert, je Ziel überschreibbar).
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Margin {
    /// Prozentpunkte bei Anteilen.
    pub share_pp: f64,
    /// Anteil der Korridorbreite bei Medianen.
    pub median_fraction: f64,
}

/// Inhalt von `data/balance-targets.json`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Targets {
    pub version: i64,
    pub margin: Margin,
    /// Eingefrorene Werte-Gruppen (B-640); `None`: die Datei nennt keine.
    pub frozen: Option<Vec<String>>,
    /// Die festen Seeds.
    pub seeds: Vec<String>,
    /// dev: ohne Seed-Angabe nur der erste Seed; all bzw. leer: alle.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub seed_mode: String,
    pub targets: Vec<Target>,
}

/// Rolle eines Ziels (B-648): bestimmt Erfolg oder Misserfolg des Laufs.
pub const ROLE_GOAL: &str = "goal";
/// Warnt bei Verletzung.
pub const ROLE_GUARD: &str = "guard";
/// Nur Diagnose.
pub const ROLE_INFO: &str = "info";

/// Ein Ziel: Kennzahl, Szenario (ohne Seed) und Korridor. Eine fehlende Grenze heißt: keine in diese Richtung.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Target {
    pub measure: String,
    /// Werte-Gruppe; die Datei der Ziele verlangt eine.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub group: String,
    pub name: String,
    pub kind: String,
    pub players: i64,
    pub bot: String,
    pub depth: i64,
    pub days: i64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub grade: String,
    /// Leer: eine Stufe der Tiefe `depth`; sonst die ganze erste Insel der Welt.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub world: String,
    /// Dev-Hilfen, nur mit `world`.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub helps: String,
    /// Leer = goal.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub role: String,
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    /// Breite von „knapp“ statt des Standardwerts der Datei.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<f64>,
    /// Regel-Datei und Abschnitt der Zahlen.
    pub rule: String,
}

const TARGETS_KEYS: &[&str] = &["version", "margin", "frozen", "seeds", "seedMode", "targets"];
const MARGIN_KEYS: &[&str] = &["sharePp", "medianFraction"];
const TARGET_KEYS: &[&str] = &[
    "measure", "group", "name", "kind", "players", "bot", "depth", "days", "grade", "world", "helps", "role", "lower",
    "upper", "margin", "rule",
];

/// Wie Go `DisallowUnknownFields`: jeder Schlüssel eines Objekts muss bekannt sein.
pub(crate) fn known_keys(value: Option<&Value>, known: &[&str]) -> Result<(), String> {
    let Some(map) = value.and_then(Value::as_object) else {
        return Ok(());
    };
    match map.keys().find(|k| !known.contains(&k.as_str())) {
        Some(k) => Err(format!("json: unknown field {}", go_quoted(k))),
        None => Ok(()),
    }
}

fn parse_strict(raw: &[u8]) -> Result<Targets, String> {
    let value: Value = serde_json::from_slice(raw).map_err(|e| e.to_string())?;
    known_keys(Some(&value), TARGETS_KEYS)?;
    known_keys(value.get("margin"), MARGIN_KEYS)?;
    for target in value.get("targets").and_then(Value::as_array).into_iter().flatten() {
        known_keys(Some(target), TARGET_KEYS)?;
    }
    read_go(raw)
}

/// Lädt `data/balance-targets.json` samt Prüfung der Rollen und Gruppen.
pub fn default_targets(root: &Path) -> Result<Targets, String> {
    let raw = datafiles::read(root, "balance-targets.json")?;
    let t = load_targets(root, &raw)?;
    t.check_roles().map_err(|e| format!("balance-targets: {e}"))?;
    Ok(t)
}

/// Liest und prüft eine Korridor-Datei; `root` liefert Grade und Welten.
pub fn load_targets(root: &Path, raw: &[u8]) -> Result<Targets, String> {
    let wrap = |e: String| format!("balance-targets: {e}");
    let t = parse_strict(raw).map_err(wrap)?;
    t.validate(root).map_err(wrap)?;
    Ok(t)
}

impl Targets {
    fn validate(&self, root: &Path) -> Result<(), String> {
        if self.version != 1 {
            return Err(format!("version {} unbekannt, erwartet 1", self.version));
        }
        if self.margin.share_pp < 0.0 || self.margin.median_fraction < 0.0 {
            return Err("margin darf nicht negativ sein".to_owned());
        }
        self.validate_seeds()?;
        check_frozen(self.frozen.as_deref().unwrap_or_default())?;
        for (i, g) in self.targets.iter().enumerate() {
            g.validate(root)
                .map_err(|e| format!("ziel {} ({}): {e}", i + 1, go_quoted(&g.name)))?;
        }
        Ok(())
    }

    /// Verlangt in der Datei der Ziele eine gültige Rolle und Gruppe je Ziel und mindestens ein goal.
    pub fn check_roles(&self) -> Result<(), String> {
        let mut goals = 0;
        for (i, g) in self.targets.iter().enumerate() {
            let at = || format!("ziel {} ({})", i + 1, go_quoted(&g.name));
            if !is_group(&g.group) {
                return Err(format!(
                    "{}: group {}, erlaubt: {}",
                    at(),
                    go_quoted(&g.group),
                    GROUPS.join(", ")
                ));
            }
            match g.role.as_str() {
                ROLE_GOAL => goals += 1,
                ROLE_GUARD | ROLE_INFO => {}
                role => {
                    return Err(format!(
                        "{}: role {}, erlaubt: goal, guard, info",
                        at(),
                        go_quoted(role)
                    ));
                }
            }
        }
        if goals == 0 {
            return Err("mindestens ein Ziel mit role goal nötig".to_owned());
        }
        Ok(())
    }
}

impl Target {
    /// Die Rolle; ohne Angabe gilt das Ziel als goal (ältere Dateien und Berichte).
    pub fn role(&self) -> &str {
        if self.role.is_empty() { ROLE_GOAL } else { &self.role }
    }

    fn validate_role(&self) -> Result<(), String> {
        if !["", ROLE_GOAL, ROLE_GUARD, ROLE_INFO].contains(&self.role.as_str()) {
            return Err(format!("role {}, erlaubt: goal, guard, info", go_quoted(&self.role)));
        }
        check_helps(&self.helps, &self.world)
    }

    pub(crate) fn validate(&self, root: &Path) -> Result<(), String> {
        self.validate_role()?;
        let Some(m) = measure(&self.measure) else {
            let mut names: Vec<&str> = all_measures().map(|m| m.id).collect();
            names.sort_unstable();
            return Err(format!(
                "kennzahl {} unbekannt (bekannt: {})",
                go_quoted(&self.measure),
                names.join(", ")
            ));
        };
        if self.name.is_empty() || self.rule.is_empty() {
            return Err("name und rule sind Pflicht".to_owned());
        }
        if self.kind != m.kind {
            return Err(format!(
                "art {} passt nicht zur Kennzahl {} ({})",
                go_quoted(&self.kind),
                self.measure,
                m.kind
            ));
        }
        if !known_bot(&self.bot) {
            return Err(format!("unbekannter Bot {}", go_quoted(&self.bot)));
        }
        if !(1..=4).contains(&self.players) || self.days < 1 {
            return Err(format!(
                "players 1 bis 4 und days ab 1 nötig, war {} und {}",
                self.players, self.days
            ));
        }
        check_players(&self.bot, self.players)?;
        check_grade(root, &self.grade)?;
        if !self.world.is_empty() {
            load_world(root, &self.world).map_err(|e| format!("welt {}: {e}", go_quoted(&self.world)))?;
        }
        self.validate_bounds()
    }

    fn validate_bounds(&self) -> Result<(), String> {
        match (self.lower, self.upper) {
            (None, None) => return Err("mindestens eine Grenze nötig".to_owned()),
            (Some(lo), Some(hi)) if lo > hi => return Err(format!("untergrenze {lo} über Obergrenze {hi}")),
            _ => {}
        }
        if self.margin.is_some_and(|m| m < 0.0) {
            return Err("margin darf nicht negativ sein".to_owned());
        }
        for b in [self.lower, self.upper].into_iter().flatten() {
            if self.kind == KIND_SHARE && !(0.0..=100.0).contains(&b) {
                return Err(format!("grenze {b} eines Anteils muss in 0 bis 100 liegen"));
            }
        }
        Ok(())
    }
}

/// Leer (normal) oder ein Grad aus `data/difficulty.json`.
fn check_grade(root: &Path, grade: &str) -> Result<(), String> {
    if grade.is_empty() {
        return Ok(());
    }
    let known = grades(root)?;
    if !known.iter().any(|g| g == grade) {
        return Err(format!(
            "unbekannter Grad {} (bekannt: [{}])",
            go_quoted(grade),
            known.join(" ")
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "targets_tests.rs"]
mod tests;
