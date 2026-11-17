//! Port von `native/k3c-game/view/stress_limits.go`.
//!
//! Grenzen der Stresstest-Werte (Testing, B-896); menu blättert innerhalb, Normalized begrenzt gespeicherte Werte.

use super::MAX_PLAYERS;
use super::devkeys::TIMESCALES;
use super::stress::{GROWTH_EXP, GROWTH_LINEAR, STRESS_MAX_ENEMIES, StressTest, go_max, go_min};

/// tiefste Stufe
pub const STRESS_MAX_DEPTH: i64 = 3;
/// wie die Lobby
pub const STRESS_MAX_SEED: i64 = 999;
/// Ausbaustufen der Dev-Aktion buildAll
pub const STRESS_MAX_BUILD: i64 = 5;
/// Obergrenze der Dev-Aktion spawn
pub const STRESS_MAX_TROOPS: i64 = 200;
/// linear: + bis 50 je Welle; exponentiell: × bis 50
pub const STRESS_MAX_FACTOR: f64 = 50.0;
pub const STRESS_MAX_WAVES: i64 = 99;
pub const STRESS_MAX_MINUTES: i64 = 120;
/// Standard-Sorte der zusätzlichen Gegner
pub const STRESS_ENEMY: &str = "goblin";
/// Standard-Sorte der Truppen vorab
pub const STRESS_TROOP: &str = "archer";

/// DefaultStress sind die Werte ohne Vorgabe: ein Spieler, Oberfläche, 10 Gegner +5 je Welle, Abbruch nach 5 Wellen.
pub fn default_stress() -> StressTest {
    StressTest {
        players: 1,
        enemies: 10,
        growth: GROWTH_LINEAR.to_string(),
        factor: 5.0,
        timescale: 1,
        waves: 5,
        enemy: STRESS_ENEMY.to_string(),
        troop: STRESS_TROOP.to_string(),
        ..StressTest::default()
    }
}

impl StressTest {
    /// Normalized begrenzt alle Werte auf ihren Bereich; unbekannter Anstieg wird linear, unbekannter Zeitraffer 1,
    /// leere Sorten die Standard-Sorten.
    #[allow(clippy::cast_possible_wrap)]
    #[must_use]
    pub fn normalized(mut self) -> StressTest {
        self.players = clamp_int(self.players, 1, MAX_PLAYERS as i64);
        self.depth = clamp_int(self.depth, 0, STRESS_MAX_DEPTH);
        self.seed = clamp_int(self.seed, 0, STRESS_MAX_SEED);
        self.enemies = clamp_int(self.enemies, 0, STRESS_MAX_ENEMIES);
        if self.growth != GROWTH_EXP {
            self.growth = GROWTH_LINEAR.to_string();
        }
        if self.factor.is_nan() {
            self.factor = 0.0;
        }
        self.factor = go_max(0.0, go_min(STRESS_MAX_FACTOR, self.factor));
        self.build = clamp_int(self.build, 0, STRESS_MAX_BUILD);
        self.troops = clamp_int(self.troops, 0, STRESS_MAX_TROOPS);
        if !TIMESCALES.contains(&self.timescale) {
            self.timescale = 1;
        }
        self.waves = clamp_int(self.waves, 0, STRESS_MAX_WAVES);
        self.minutes = clamp_int(self.minutes, 0, STRESS_MAX_MINUTES);
        if self.enemy.is_empty() {
            self.enemy = STRESS_ENEMY.to_string();
        }
        if self.troop.is_empty() {
            self.troop = STRESS_TROOP.to_string();
        }
        self
    }
}

fn clamp_int(v: i64, lo: i64, hi: i64) -> i64 {
    lo.max(hi.min(v))
}
