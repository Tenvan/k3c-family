//! Platzhalter für Teile, die spätere Sprints liefern (Leitfaden § 7). RU30.2 hat `script`, `voyage`, `shots` und
//! `assets` geliefert, RU30.3 Entwickler-Tasten, Werte, Bildschirmfoto und Fehlerfenster; Ist alles geliefert (RU30.4
//! und RU30.5), wird diese Datei gelöscht. RU22 prüft, dass keiner übrig
//! ist. Die Platzhalter der Schleife (`Game`, RU26.4) stehen am Ende der Datei.
#![allow(clippy::unimplemented)] // Platzhalter nach Leitfaden § 7: wartet auf RU30
#![allow(dead_code)] // Platzhalter, die erst der liefernde Sprint benutzt

use super::{Game, Measure};

/// Fachmodule, die `packages()` noch nicht liefern kann: Name und liefernder Sprint. `parts_tests` zählt sie als
/// „wartet auf RUnn“ und prüft, dass der Platzhalter verschwindet, sobald der Modulordner existiert.
#[cfg_attr(not(test), allow(dead_code))]
pub const WAITING: &[(&str, &str)] = &[];

// ---- Platzhalter der Schleife (RU26.4): Teile, die RU30 liefert ----------------------------------------
//
// Platzhalter mit `unimplemented!` sind nur über Zustand erreichbar, den `Game::new` nie setzt (`Option` bleibt
// `None`, Schalter aus). Platzhalter ohne Wirkung (`dev_values`) läuft in jedem Takt und tun bis zu ihrem Sprint nichts.

/// Go: `meter` (`measure.go`, RU30.4): Messmodus (`--measure`).
pub struct Meter;

impl Meter {
    pub fn new(_m: &Measure) -> Meter {
        unimplemented!("RU30.4: Meter::new")
    }

    pub fn done(&self) -> bool {
        unimplemented!("RU30.4: Meter::done")
    }

    pub fn started(&mut self, _t0: f64) {
        unimplemented!("RU30.4: Meter::started")
    }

    pub fn tick(&mut self, _dur: f64) {
        unimplemented!("RU30.4: Meter::tick")
    }

    pub fn frame(&mut self, _now: f64, _dur: f64) {
        unimplemented!("RU30.4: Meter::frame")
    }
}

/// Go: `stressRun` (`stress.go`, RU30.5): laufender Stresstest.
pub struct StressRun;

impl Game {
    /// Go: `refillMeasureLoad` (`measure_load.go`, RU30.4); erreicht nur mit `meter`.
    pub(super) fn refill_measure_load(&mut self) {
        unimplemented!("RU30.4: refill_measure_load")
    }

    /// Go: `measureLoad` (`measure_load.go`, RU30.4); erreicht nur mit `meter`.
    pub(super) fn measure_load(&mut self, _spec: &str) -> Result<(), String> {
        unimplemented!("RU30.4: measure_load")
    }

    /// Go: `stressStep` (`stress.go`, RU30.5); erreicht nur mit `stress`.
    pub(super) fn stress_step(&mut self) {
        unimplemented!("RU30.5: stress_step")
    }
}
