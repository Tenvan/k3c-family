//! Port von `native/k3c-game/game/fault.go`: Fehlerzustand (B-810). Ein Fehler im Spiel (fehlende Grafik) beendet das
//! Programm nicht still, sondern zeigt die Meldung im Fenster (`Fault`); Enter, Esc, Leertaste oder A bzw. B am Pad
//! beenden. `Game::fault` reicht den Fehler danach an `main` (stderr, Exit 1). Das Fehlerfenster vor dem Spiel baut
//! RU30.6 mit demselben `Fault`.
//!
//! Abweichungen von Go: Der Fehler ist ein Text (`errors.Is(err, art.ErrMissing)` heißt `Fault::missing`, Aufruf
//! `Game::fail` mit `ArtError`); `Fault::update` bekommt die Eingabe des Takts, weil die Plattform keine globale
//! Abfrage kennt (`inpututil`).

use k3c_platform::{Canvas, InputState, Key, PadButton, log};

use super::notice::{Notice, cap_lines};
use super::{Game, Mode};
use crate::art::ArtError;

/// Zeilen, die Titel und Fehlertext höchstens belegen (der Rest steht im Log).
pub const FAULT_MAX_LINES: usize = 12;

/// Die Meldung zu einem Fehler: Titel („Grafik fehlt“ bei fehlender Grafik, sonst „Fehler“), Text mit Pfad, Hinweis zum
/// Beenden.
pub struct Fault {
    err: String,
    n: Notice,
}

impl Fault {
    /// Baut die Meldung zu `err` mit dem Titel „Fehler“.
    pub fn new(err: impl Into<String>) -> Fault {
        Fault::build(err.into(), false)
    }

    /// Baut die Meldung zu fehlender Grafik (Titel „Grafik fehlt“).
    pub fn missing(err: impl Into<String>) -> Fault {
        Fault::build(err.into(), true)
    }

    fn build(err: String, missing: bool) -> Fault {
        let title = if missing {
            "Grafik fehlt"
        } else {
            "Fehler: Das Spiel kann nicht weiterlaufen"
        };
        let (first, more) = split_err(&err);
        let rest = format!("(und {more} weitere, alle im Log)");
        let mut paras = vec![title, "", first];
        if more > 0 {
            paras.push(&rest);
        }
        let mut n = Notice::new(&paras);
        n.lines = cap_lines(n.lines, FAULT_MAX_LINES);
        n.lines.extend(
            [
                "",
                "Enter, Esc oder A beendet das Spiel.",
                "Die Meldung steht auch im Log (stderr).",
            ]
            .map(String::from),
        );
        Fault { err, n }
    }

    /// Der Fehler hinter der Meldung.
    pub fn err(&self) -> &str {
        &self.err
    }

    /// Die Zeilen der Meldung (Tests).
    pub fn lines(&self) -> &[String] {
        &self.n.lines
    }

    /// Wahr, wenn eine Beenden-Taste frisch gedrückt ist (Go: `Update` gibt `ebiten.Termination` zurück).
    pub fn update(&self, raw: &InputState) -> bool {
        exit_pressed(raw)
    }

    /// Zeichnet die Meldung.
    pub fn draw(&self, c: &mut dyn Canvas) {
        self.n.draw(c);
    }
}

/// Trennt einen zusammengesetzten Fehler (eine Meldung je Zeile, z. B. eine je fehlendem Sheet) in die erste Meldung
/// und die Zahl der weiteren.
fn split_err(err: &str) -> (&str, usize) {
    let mut lines = err.split('\n').filter(|l| !l.trim().is_empty());
    match lines.next() {
        Some(first) => (first, lines.count()),
        None => (err, 0),
    }
}

/// Frische Beenden-Taste (Tastatur: Enter, Num-Enter, Esc, Leertaste; Pad: A oder B).
fn exit_pressed(raw: &InputState) -> bool {
    [Key::Enter, Key::NumpadEnter, Key::Escape, Key::Space]
        .iter()
        .any(|k| raw.just_pressed(*k))
        || raw
            .pads()
            .iter()
            .any(|p| p.just_pressed(PadButton::A) || p.just_pressed(PadButton::B))
}

impl Game {
    /// Meldet einen Fehler der Grafik (Go: `fail`).
    pub(super) fn fail(&mut self, err: &ArtError) {
        self.report_fault(err.to_string(), err.is_missing());
    }

    /// Meldet einen Fehler der Schleife: Im Messmodus beendet er das Programm (`err`), sonst zeigt die Schleife ihn im
    /// Fenster (`fault`; auch im Aufnahme-Modus, damit das Bild die Meldung zeigt; den Exit-Code setzt `main` über
    /// `fault()`). Ein erster Fehler bleibt stehen.
    pub(super) fn report_fault(&mut self, msg: String, missing: bool) {
        if self.err.is_some() || self.fault.is_some() {
            return;
        }
        if self.args.measure.seconds > 0.0 {
            self.err = Some(msg);
            return;
        }
        log::error("💥 Fehler im Fenster gemeldet", &[("err", &msg)]);
        self.fault = Some(if missing { Fault::missing(msg) } else { Fault::new(msg) });
    }

    /// Der Fehler, den die Schleife gemeldet hat (`None`: keiner); `main` macht daraus stderr und Exit 1.
    pub fn fault(&self) -> Option<&str> {
        self.fault.as_ref().map(Fault::err).or(self.err.as_deref())
    }

    /// Bedient den Fehlerzustand: Beenden-Taste → Spiel schließen (`Session::close` speichert) und beenden.
    pub(super) fn fault_update(&mut self, raw: &InputState) {
        if self.fault.as_ref().is_some_and(|f| f.update(raw)) {
            self.mode = Mode::Quit;
            if self.sim.active() {
                self.close_sim();
            }
        }
    }

    /// Ein Fehler im Messmodus beendet das Programm (Go: `Update` gab `g.err` zurück); wahr: beendet.
    pub(super) fn err_exit(&mut self) -> bool {
        let quit = self.err.is_some();
        if quit {
            self.mode = Mode::Quit;
        }
        quit
    }
}

#[cfg(test)]
#[path = "fault_tests.rs"]
mod fault_tests;
