//! Platzhalter für Teile, die spätere Sprints liefern (Leitfaden § 7). RU30.2 hat `script`, `voyage`, `shots` und
//! `assets` geliefert, RU30.3 Entwickler-Tasten, Werte, Bildschirmfoto und Fehlerfenster, RU30.4 den Messmodus; Ist alles
//! geliefert (RU30.5), wird diese Datei gelöscht. RU22 prüft, dass keiner übrig
//! ist. Die Platzhalter der Schleife (`Game`, RU26.4) stehen am Ende der Datei.
#![allow(clippy::unimplemented)] // Platzhalter nach Leitfaden § 7: wartet auf RU30
#![allow(dead_code)] // Platzhalter, die erst der liefernde Sprint benutzt

use super::Game;

/// Fachmodule, die `packages()` noch nicht liefern kann: Name und liefernder Sprint. `parts_tests` zählt sie als
/// „wartet auf RUnn“ und prüft, dass der Platzhalter verschwindet, sobald der Modulordner existiert.
#[cfg_attr(not(test), allow(dead_code))]
pub const WAITING: &[(&str, &str)] = &[];

// ---- Platzhalter der Schleife (RU26.4): Teile, die RU30.5 liefert ----------------------------------------
//
// Platzhalter mit `unimplemented!` sind nur über Zustand erreichbar, den `Game::new` nie setzt (`Option` bleibt
// `None`, Schalter aus). Platzhalter ohne Wirkung (`dev_values`) läuft in jedem Takt und tun bis zu ihrem Sprint nichts.

/// Go: `stressRun` (`stress.go`, RU30.5): laufender Stresstest.
pub struct StressRun;

impl Game {
    /// Go: `stressStep` (`stress.go`, RU30.5); erreicht nur mit `stress`.
    pub(super) fn stress_step(&mut self) {
        unimplemented!("RU30.5: stress_step")
    }
}
