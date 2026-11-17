//! Port von `native/k3c-game/view/stress.go`.
//!
//! Stresstests im Spielmenü (Testing, B-896): menu stellt einen StressTest ein und startet ihn über die optionale
//! Rolle StressStarter der Session (game erfüllt sie); game startet das Spiel, führt die Werte über Dev-Aktionen aus
//! und schreibt am Ende einen Leistungsbericht nach ReportsDir. Die Vorgaben stehen in data/stresstests.json.

use serde::{Deserialize, Serialize};

use super::input::DeviceID;

// Anstieg der Gegnerzahl je Welle (StressTest.Growth).
/// Gegner + Faktor je weitere Welle
pub const GROWTH_LINEAR: &str = "linear";
/// Gegner × Faktor je weitere Welle
pub const GROWTH_EXP: &str = "exponentiell";

/// StressMaxEnemies ist die Obergrenze zusätzlicher Gegner je Welle (Obergrenze der Dev-Aktion spawn,
/// engine/sim › DevMaxSpawn).
pub const STRESS_MAX_ENEMIES: i64 = 200;

/// StressTest sind die Werte eines Stresstests; vergleichbar (steht in Settings.Stress). Null-Werte: Standard-Welt,
/// Oberfläche, kein Ausbau, keine Truppen, kein Zeitraffer, kein Abbruch.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StressTest {
    /// Vorgabe, aus der die Werte stammen ("" = eigene)
    #[serde(rename = "id")]
    pub preset: String,
    /// Spieler 1 … MaxPlayers; ohne Gerät wartet der Platz auf eins
    pub players: i64,
    /// Stufe (Tiefe), in der die Spieler beginnen
    pub depth: i64,
    /// 0 … 999; 0 = Name des Spielstands
    pub seed: i64,
    /// zusätzliche Gegner in der ersten Welle
    pub enemies: i64,
    /// GrowthLinear oder GrowthExp
    pub growth: String,
    /// linear: + Faktor je Welle; exponentiell: × Faktor je Welle
    pub factor: f64,
    /// Ausbaustufe vorab (Dev-Aktion buildAll), 0 = keiner
    pub build: i64,
    /// Truppen vorab (Dev-Aktion spawn troop)
    pub troops: i64,
    /// Zeitraffer (Timescales), 0 oder 1 = normal
    pub timescale: i64,
    /// Abbruch nach so vielen Wellen, 0 = nie
    pub waves: i64,
    /// Abbruch nach so vielen Minuten Wanduhr, 0 = nie
    pub minutes: i64,
    /// Gegner-Sorte der zusätzlichen Gegner (data/enemies.json)
    pub enemy: String,
    /// Truppen-Sorte vorab (data/troops.json)
    pub troop: String,
    /// reiner Stressmodus (B-921): Gegner entstehen an der Burg, sonst nichts von selbst
    pub pure: bool,
}

impl StressTest {
    /// EnemiesFor liefert die zusätzlichen Gegner der Welle wave (1 = erste) nach Anstieg und Faktor,
    /// 0 … StressMaxEnemies.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn enemies_for(&self, wave: i64) -> i64 {
        if wave < 1 || self.enemies <= 0 {
            return 0;
        }
        let mut n = self.enemies as f64;
        let steps = (wave - 1) as f64;
        if self.growth == GROWTH_EXP {
            n *= go_max(self.factor, 0.0).powf(steps);
        } else {
            n += self.factor * steps;
        }
        if n.is_nan() || n < 0.0 {
            return 0;
        }
        go_min(n.round(), STRESS_MAX_ENEMIES as f64) as i64
    }
}

/// StressStarter ist eine optionale Rolle der Session, die menu bekommt (game erfüllt sie, B-896): StartStress startet
/// ein neues Spiel mit den Werten t; by ist das Gerät von Spieler 1. Danach liefert der Screen StepPlay.
pub trait StressStarter {
    fn start_stress(&mut self, t: &StressTest, by: DeviceID) -> Result<(), String>;
}

/// go_max rechnet wie `math.Max`: NaN gewinnt, +0 schlägt -0 (anders als `f64::max`).
pub(super) fn go_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        return f64::NAN;
    }
    if a == 0.0 && b == 0.0 {
        return if a.is_sign_positive() { a } else { b };
    }
    if a > b { a } else { b }
}

/// go_min rechnet wie `math.Min`: NaN gewinnt, -0 schlägt +0 (anders als `f64::min`).
pub(super) fn go_min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        return f64::NAN;
    }
    if a == 0.0 && b == 0.0 {
        return if a.is_sign_negative() { a } else { b };
    }
    if a < b { a } else { b }
}
