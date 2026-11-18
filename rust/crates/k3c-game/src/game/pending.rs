//! Platzhalter für Teile, die spätere Sprints liefern (Leitfaden § 7). RU30.2 liefert `script.go` samt `ParseScript`,
//! tauscht die `pub use`-Zeile in `mod.rs` aus; RU30.1 liefert `Loader`; RU29.3 tauscht `devtools_part` samt Eintrag in
//! `WAITING`. Ist alles geliefert, wird diese Datei gelöscht. RU22 prüft, dass keiner übrig ist. Die Platzhalter der
//! Schleife (`Game`, RU26.4) stehen am Ende der Datei.
#![allow(clippy::unimplemented)] // Platzhalter nach Leitfaden § 7: wartet auf RU26.5 und RU30
#![allow(dead_code)] // Platzhalter, die erst der liefernde Sprint benutzt

use super::parts::Part;
use super::scene::Hold;
use super::{Args, Game, Measure, Scene};
use crate::art::Library;
use crate::view::input::{Input, Source};
use k3c_platform::Canvas;
use std::cell::RefCell;
use std::rc::Rc;

/// Go: `ParseScript` (`script.go`, RU30.2): liest die feste Eingabe-Folge „p@t=…,…“.
pub fn parse_script(_s: &str) -> Result<Vec<Hold>, String> {
    unimplemented!("RU30: parse_script")
}

/// Go: `Loader` (`levels.go`, RU30.1): lädt die Grafik der Level (`Parts::art`).
pub trait Loader {}

/// Fachmodule, die `packages()` noch nicht liefern kann: Name und liefernder Sprint. `parts_tests` zählt sie als
/// „wartet auf RUnn“ und prüft, dass der Platzhalter verschwindet, sobald der Modulordner existiert.
#[cfg_attr(not(test), allow(dead_code))]
pub const WAITING: &[(&str, &str)] = &[("devtools", "RU29")];

/// Go: `devtools.New` mit Chronik (Cheat-Menü, Testing, Sitzungsbericht; RU29): bis dahin ein leerer Beitrag mit dem
/// Namen des Moduls (ein `unimplemented!` ließe jeden Zusammenbau abstürzen). RU29.3 tauscht diese Funktion.
pub fn devtools_part(_lib: Option<Rc<RefCell<Library>>>) -> Part {
    Part::named("devtools")
}

// ---- Platzhalter der Schleife (RU26.4): Teile, die RU26.5 und RU30 liefern ----------------------------------------
//
// Platzhalter mit `unimplemented!` sind nur über Zustand erreichbar, den `Game::new` nie setzt (`Option` bleibt
// `None`, Schalter aus). Platzhalter ohne Wirkung (`toggle_window`, `apply_window`, `open_menu`, `play`, `dev_values`,
// `embark`, `arrive`) laufen in jedem Takt und tun bis zu ihrem Sprint nichts.

/// Go: `voyage` (`voyage.go`, RU30.2): Schiffsfahrt beim Inselwechsel; `Some` nur während der Fahrt.
pub struct Voyage;

/// Go: `shooter` (`shots.go`, RU30.2): Aufnahme-Modus (`--shots`).
pub struct Shooter;

impl Shooter {
    pub fn new(_a: &Args) -> Result<Shooter, String> {
        unimplemented!("RU30.2: Shooter::new")
    }

    pub fn update(&mut self, _g: &mut Game) {
        unimplemented!("RU30.2: Shooter::update")
    }

    pub fn capture(&mut self, _c: &mut dyn Canvas) {
        unimplemented!("RU30.2: Shooter::capture")
    }
}

/// Go: `Fault` (`fault.go`, RU30.3): Fehler im Fenster (fehlende Grafik).
pub struct Fault;

impl Fault {
    pub fn draw(&self, _c: &mut dyn Canvas) {
        unimplemented!("RU30.3: Fault::draw")
    }
}

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

/// Go: `view.DevSession` (RU30.3): Entwickler-Rolle der Sitzung.
pub trait DevSession {
    /// Go: `Halt`: hält die Simulation an (F7) bzw. gibt sie frei.
    fn halt(&mut self, on: bool);
}

/// Go: `newScripted` (`script.go`, RU30.2): Quelle mit der festen Eingabe-Folge der Szene.
pub fn scripted(_s: &Scene) -> Box<dyn Source> {
    unimplemented!("RU30.2: scripted")
}

impl Game {
    /// Go: `syncLevels` (`levels.go`, RU30.1); erreicht nur mit `Parts::art`.
    pub(super) fn sync_levels(&mut self) {
        unimplemented!("RU30.1: sync_levels")
    }

    /// Go: `drawLoading` (`loading.go`, RU30.1); erreicht nur mit `loading`.
    pub(super) fn draw_loading(&mut self, _c: &mut dyn Canvas) {
        unimplemented!("RU30.1: draw_loading")
    }

    /// Go: `sail` (`voyage.go`, RU30.2); erreicht nur mit `voy`.
    pub(super) fn sail(&mut self, _input: &Input) {
        unimplemented!("RU30.2: sail")
    }

    /// Go: `drawVoyage` (`voyage.go`, RU30.2); erreicht nur mit `voy`.
    pub(super) fn draw_voyage(&mut self, _c: &mut dyn Canvas) {
        unimplemented!("RU30.2: draw_voyage")
    }

    /// Go: `devShot` (`devshot.go`, RU30.3); erreicht nur mit `shot_next`.
    pub(super) fn dev_shot(&mut self, _c: &mut dyn Canvas) {
        unimplemented!("RU30.3: dev_shot")
    }

    /// Go: `applyDevArgs` (`devargs.go`, RU30.3); erreicht nur mit `--river` oder `--stats`.
    pub(super) fn apply_dev_args(&mut self) {
        unimplemented!("RU30.3: apply_dev_args")
    }

    /// Go: `faultUpdate` (`fault.go`, RU30.3); erreicht nur mit `fault`.
    pub(super) fn fault_update(&mut self) {
        unimplemented!("RU30.3: fault_update")
    }

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

    /// Go: `toggleWindow` (`window.go`, RU26.5): Vollbild-Taste. Bis dahin ohne Wirkung.
    pub(super) fn toggle_window(&mut self, _input: &Input) {}

    /// Go: `applyWindow` (`window.go`, RU26.5): Anzeige-Modus aus den Einstellungen. Bis dahin ohne Wirkung.
    pub(super) fn apply_window(&mut self) {}

    /// Go: `openMenu` (`menu.go`, RU26.5): legt das Menü der Szene (`--menu`) auf. Bis dahin ohne Wirkung.
    pub(super) fn open_menu(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Go: `play` (`play.go`, RU26.5): Overlays, Befehle, Beitritt im laufenden Spiel. Bis dahin ohne Wirkung.
    pub(super) fn play(&mut self, _input: &Input) {}

    /// Go: `devValues` (`devvalues.go`, RU30.3). Bis dahin ohne Wirkung.
    pub(super) fn dev_values(&mut self) {}

    /// Go: `embark` (`voyage.go`, RU30.2): beginnt die Schiffsfahrt beim Inselwechsel. Bis dahin ohne Wirkung.
    pub(super) fn embark(&mut self) {}

    /// Go: `arrive` (`voyage.go`, RU30.2). Bis dahin ohne Wirkung.
    pub(super) fn arrive(&mut self) {}
}
