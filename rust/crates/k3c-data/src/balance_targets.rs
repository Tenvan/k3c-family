//! data/balance-targets.json: Go `balance.Targets` (engine/balance/targets.go) mit `LoadTargets` und `checkRoles`.

use serde::{Deserialize, Serialize};

use crate::balance_hints::GROUPS;
use crate::{Fehler, check, decode};

/// Go: `balance.Targets`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Targets {
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(rename = "margin")]
    pub margin: TargetsMargin,
    #[serde(rename = "frozen")]
    pub frozen: Vec<String>,
    #[serde(rename = "seeds")]
    pub seeds: Vec<String>,
    #[serde(rename = "seedMode", skip_serializing_if = "String::is_empty")]
    pub seed_mode: String,
    #[serde(rename = "targets")]
    pub targets: Vec<Target>,
}

/// Go: `struct { SharePp float64 "json:\"sharePp\""; MedianFraction float64 "json:\"medianFraction\"" }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TargetsMargin {
    #[serde(rename = "sharePp")]
    pub share_pp: f64,
    #[serde(rename = "medianFraction")]
    pub median_fraction: f64,
}

/// Go: `balance.Target`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Target {
    #[serde(rename = "measure")]
    pub measure: String,
    #[serde(rename = "group", skip_serializing_if = "String::is_empty")]
    pub group: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "players")]
    pub players: i64,
    #[serde(rename = "bot")]
    pub bot: String,
    #[serde(rename = "depth")]
    pub depth: i64,
    #[serde(rename = "days")]
    pub days: i64,
    #[serde(rename = "grade", skip_serializing_if = "String::is_empty")]
    pub grade: String,
    #[serde(rename = "world", skip_serializing_if = "String::is_empty")]
    pub world: String,
    #[serde(rename = "helps", skip_serializing_if = "String::is_empty")]
    pub helps: String,
    #[serde(rename = "role", skip_serializing_if = "String::is_empty")]
    pub role: String,
    #[serde(rename = "lower")]
    pub lower: Option<f64>,
    #[serde(rename = "upper")]
    pub upper: Option<f64>,
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<f64>,
    #[serde(rename = "rule")]
    pub rule: String,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "balance-targets.json";

/// Art eines Anteils in Prozent (engine/balance `KindShare`).
const KIND_SHARE: &str = "share";
/// Seed-Modi der Datei (engine/balance `SeedModeDev`, `SeedModeAll`).
const SEED_MODE_DEV: &str = "dev";
const SEED_MODE_ALL: &str = "all";

/// `load_targets` liest und prüft eine Korridor-Datei wie `balance.DefaultTargets` (`LoadTargets`, dann
/// `checkRoles`). Nicht geprüft, weil die Tabellen in engine/balance stehen (B-1024): Kennzahl und Art
/// (`measures`), Bot und Spielerzahl, Grad, Welt und Dev-Hilfen (`checkHelps`).
pub fn load_targets(raw: &str) -> Result<Targets, Fehler> {
    let t: Targets = decode("balance-targets", raw)?;
    check("balance-targets", t.validate())?;
    check("balance-targets", t.check_roles())?;
    Ok(t)
}

impl Targets {
    fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err(format!("version {} unbekannt, erwartet 1", self.version));
        }
        if self.margin.share_pp < 0.0 || self.margin.median_fraction < 0.0 {
            return Err("margin darf nicht negativ sein".into());
        }
        self.validate_seeds()?;
        check_frozen(&self.frozen)?;
        for (i, g) in self.targets.iter().enumerate() {
            g.validate()
                .map_err(|e| format!("ziel {} ({:?}): {e}", i + 1, g.name))?;
        }
        Ok(())
    }

    fn validate_seeds(&self) -> Result<(), String> {
        let m = self.seed_mode.as_str();
        if !(m.is_empty() || m == SEED_MODE_DEV || m == SEED_MODE_ALL) {
            return Err(format!(
                "seedMode {:?}: {SEED_MODE_DEV} oder {SEED_MODE_ALL}",
                self.seed_mode
            ));
        }
        if self.seeds.is_empty() {
            return Err("seeds: mindestens ein Seed nötig".into());
        }
        for (i, s) in self.seeds.iter().enumerate() {
            if s.is_empty() || self.seeds[..i].contains(s) {
                return Err(format!("seeds: {s:?} ist leer oder doppelt"));
            }
        }
        Ok(())
    }

    /// `check_roles` verlangt eine gültige Gruppe und Rolle je Ziel und mindestens ein goal.
    fn check_roles(&self) -> Result<(), String> {
        let mut goals = 0;
        for (i, g) in self.targets.iter().enumerate() {
            if !GROUPS.contains(&g.group.as_str()) {
                return Err(format!(
                    "ziel {} ({:?}): group {:?}, erlaubt: {}",
                    i + 1,
                    g.name,
                    g.group,
                    GROUPS.join(", ")
                ));
            }
            match g.role.as_str() {
                "goal" => goals += 1,
                "guard" | "info" => {}
                _ => {
                    return Err(format!(
                        "ziel {} ({:?}): role {:?}, erlaubt: goal, guard, info",
                        i + 1,
                        g.name,
                        g.role
                    ));
                }
            }
        }
        if goals == 0 {
            return Err("mindestens ein Ziel mit role goal nötig".into());
        }
        Ok(())
    }
}

/// `check_frozen`: nur bekannte Gruppen, keine doppelten.
fn check_frozen(frozen: &[String]) -> Result<(), String> {
    for (i, g) in frozen.iter().enumerate() {
        if !GROUPS.contains(&g.as_str()) {
            return Err(format!(
                "frozen: gruppe {g:?} unbekannt (bekannt: {})",
                GROUPS.join(", ")
            ));
        }
        if frozen[..i].contains(g) {
            return Err(format!("frozen: gruppe {g:?} doppelt"));
        }
    }
    Ok(())
}

impl Target {
    fn validate(&self) -> Result<(), String> {
        let r = self.role.as_str();
        if !matches!(r, "" | "goal" | "guard" | "info") {
            return Err(format!("role {:?}, erlaubt: goal, guard, info", self.role));
        }
        if self.name.is_empty() || self.rule.is_empty() {
            return Err("name und rule sind Pflicht".into());
        }
        if self.players < 1 || self.players > 4 || self.days < 1 {
            return Err(format!(
                "players 1 bis 4 und days ab 1 nötig, war {} und {}",
                self.players, self.days
            ));
        }
        self.validate_bounds()
    }

    fn validate_bounds(&self) -> Result<(), String> {
        let g = k3c_det::golden::go_float;
        match (self.lower, self.upper) {
            (None, None) => return Err("mindestens eine Grenze nötig".into()),
            (Some(lo), Some(hi)) if lo > hi => {
                return Err(format!("untergrenze {} über Obergrenze {}", g(lo), g(hi)));
            }
            _ => {}
        }
        if self.margin.is_some_and(|m| m < 0.0) {
            return Err("margin darf nicht negativ sein".into());
        }
        for b in [self.lower, self.upper].into_iter().flatten() {
            if self.kind == KIND_SHARE && (b < 0.0 || b > 100.0) {
                return Err(format!("grenze {} eines Anteils muss in 0 bis 100 liegen", g(b)));
            }
        }
        Ok(())
    }
}
