//! Port von `native/k3c-game/game/voyage.go`: die Schiffsfahrt beim Inselwechsel (B-977). Meldet die Simulation den
//! Wechsel (Ereignis `islandSwitch`), zeigt die Schleife statt des Ladebildschirms die Seekarte im Vollbild
//! (`view::voyage::VoyageDrawer`, hud). Start ist die Insel, die der Wechselpunkt im Frame davor als aktuelle nannte
//! (letzte der Abgeschlossenen), Ziel das Ziel des Ereignisses. Während der Fahrt taktet die Simulation nicht und keine
//! Spiel-Eingabe wirkt (kein Befehl, kein Overlay, keine Pause, kein Beitritt; nur der Vollbild-Wechsel bleibt). Die
//! Zeit zählt in Takten der Schleife; der letzte Frame samt seiner Uhr steht, damit Hinweise und Banner nicht hinter der
//! Karte ablaufen (die Ankunft blendet erst nach der Fahrt ein, R6). Die Fahrt endet frühestens nach der Fahrzeit
//! (`data/native.json` › `voyage.seconds`) und erst, wenn die Grafik der Zielinsel geladen ist. Ohne Zeichner, ohne
//! Assets, im Aufnahme- und im Messmodus bleibt der Ladebildschirm. Laden eines Spielstands und neues Spiel melden kein
//! `islandSwitch` und zeigen ihn wie bisher. Die Schleife rechnet dabei keine Regel: Wann gewechselt wird und wohin,
//! entscheidet die Simulation.
//!
//! Abweichung von Go: `draw_voyage` bekommt `alpha` als Parameter von `App::draw` (die Zeitsammlung der Plattform), Go
//! las `blend.alpha()` aus der Wanduhr.

use k3c_platform::{Canvas, Rect, log};

use super::{CLEAR_COLOR, Game, TPS};
use crate::art;
use crate::view::frame::{Frame, Seat};
use crate::view::input::{Action, Input};
use crate::view::voyage::Voyage as View;

/// Der Zustand der Fahrt. `done`, `open` und `choice` merkt `note` vor jedem Takt aus dem Wechselpunkt (Puffer, keine
/// Allokation je Takt).
#[derive(Default)]
pub struct Voyage {
    pub on: bool,
    pub v: View,
    /// Takte seit dem Ablegen
    pub ticks: i64,
    done: Vec<String>,
    open: Vec<String>,
    choice: String,
}

/// Go: `voyageSeconds`: die Fahrzeit aus `data/native.json` › `voyage.seconds` (B-977/R3: Startwert 3 s, vorläufig bis
/// zur Abstimmung am TV).
pub fn voyage_seconds() -> f64 {
    art::voyage().seconds
}

impl Voyage {
    /// Merkt den Stand des Wechselpunkts im Frame `f`: die Abgeschlossenen (die aktuelle Insel zuletzt), die Ziele und
    /// das Ziel der Fahrt (Wahl, sonst das erste der Liste).
    pub fn note(&mut self, f: &Frame) {
        self.done.clear();
        self.open.clear();
        self.choice.clear();
        for st in &f.stages {
            let Some(sw) = st.world.as_ref().and_then(|w| w.island_switch.as_ref()) else {
                continue;
            };
            self.done.extend(sw.completed.iter().cloned());
            self.choice.push_str(&sw.choice);
            self.open.extend(sw.targets.iter().map(|t| t.id.clone()));
            if self.choice.is_empty()
                && let Some(t) = sw.targets.first()
            {
                self.choice.push_str(&t.id);
            }
            return;
        }
    }
}

/// Das Ziel des Ereignisses `islandSwitch` im Frame `f`; `None`, wenn der Frame das Ereignis nicht trägt.
pub fn switch_target(f: &Frame) -> Option<&str> {
    f.events
        .iter()
        .find(|e| e.type_() == "islandSwitch")
        .map(|e| e.data.get("target").and_then(|v| v.as_str()).unwrap_or(""))
}

/// Meldet, ob der Frame `f` weiter den Wechselpunkt der Insel `from` zeigt: Der Raum hat die Insel nicht getauscht
/// (Fehler beim Bau der Zielinsel), es gibt keine Fahrt. Es zählen nur Stufen, in denen ein Spieler steht: Der Frame
/// des Wechsels trägt noch den letzten Zustand der verlassenen Stufen der alten Insel (der Raum schickt erst ihn,
/// dann die neue Insel; die Spieler landen in Stufe 0), ein Hafen in einer tieferen Stufe gälte sonst als nicht
/// gewechselt.
pub fn still_at(f: &Frame, from: &str) -> bool {
    for st in &f.stages {
        let Some(sw) = st.world.as_ref().and_then(|w| w.island_switch.as_ref()) else {
            continue;
        };
        if !seated(&f.seats, st.index) {
            continue;
        }
        return sw.completed.last().is_some_and(|c| c == from);
    }
    false
}

/// Meldet, ob ein Spieler in der Stufe `stage` steht.
pub fn seated(seats: &[Seat], stage: i64) -> bool {
    seats.iter().any(|s| s.stage == stage)
}

impl Game {
    /// Beginnt die Fahrt, wenn der Frame dieses Takts den Inselwechsel meldet und sie zu zeichnen ist.
    pub(super) fn embark(&mut self) {
        let Some(target) = switch_target(&self.frame) else {
            return;
        };
        let y = &self.voy;
        let to = if target.is_empty() {
            y.choice.clone()
        } else {
            target.to_string()
        };
        let Some(from) = y.done.last().cloned() else {
            return;
        };
        if !self.defer_load() || to.is_empty() || still_at(&self.frame, &from) {
            return;
        }
        let Some(vd) = self.parts.voyage.as_ref() else {
            return;
        };
        let v = View {
            from,
            to,
            done: y.done.clone(),
            open: y.open.clone(),
            elapsed: 0.0,
            duration: voyage_seconds(),
        };
        if !vd.voyage_ready(&self.frame, &v) {
            return; // Assets fehlen (R5): Ladebildschirm wie bisher
        }
        log::info("🚀 Schiffsfahrt", &[("von", &v.from), ("nach", &v.to)]);
        self.voy.v = v;
        self.voy.on = true;
        self.voy.ticks = 0;
    }

    /// Der Takt der Schleife während der Fahrt: Die Simulation steht, der letzte Frame gilt samt Uhr weiter; Ereignisse
    /// und Cues des Takts davor sind gemeldet und entfallen (Geräte-Cues dieses Takts hängt `advance` an). Von der
    /// Eingabe wirkt nur der Vollbild-Wechsel (kein Spielzug); die Einstellungen des Frames folgen ihm sofort.
    pub(super) fn sail(&mut self, input: &Input) {
        self.voy.ticks += 1;
        self.frame.events.clear();
        self.frame.cues.clear();
        self.frame.step_ms = 0.0;
        if input.has(Action::Fullscreen).1 {
            self.toggle_window(input);
            self.frame.settings = self.sim.settings();
        }
    }

    /// Beendet die Fahrt nach der Fahrzeit, sobald nichts mehr lädt; der nächste Takt rechnet die Zielinsel.
    pub(super) fn arrive(&mut self) {
        let y = &mut self.voy;
        if !y.on || (y.ticks as f64) / (TPS as f64) < y.v.duration || self.loading {
            return;
        }
        y.on = false;
        log::info("✅ Schiffsfahrt beendet", &[("nach", &y.v.to), ("takte", &y.ticks)]);
    }

    /// Zeichnet die Fahrt zur Zeit des Bilds (Takte seit dem Ablegen plus der Anteil `alpha` bis zum nächsten Takt) und
    /// merkt wie die Ladeanzeige, dass ein Bild zu sehen war (erst danach lädt das nächste Update). Zeichnet der
    /// Zeichner nichts, erscheint der Ladebildschirm.
    pub(super) fn draw_voyage(&mut self, c: &mut dyn Canvas, alpha: f64) {
        self.voy.v.elapsed = ((self.voy.ticks as f64 - 1.0 + alpha) / TPS as f64).max(0.0);
        let drawn = self
            .parts
            .voyage
            .as_ref()
            .is_some_and(|d| d.draw_voyage(c, &self.frame, &self.voy.v));
        if !drawn {
            if self.loading && self.load.msg.is_some() {
                self.draw_loading(c);
            } else {
                let (w, h) = c.size();
                c.rect(Rect::new(0.0, 0.0, w, h), CLEAR_COLOR);
            }
        }
        self.load.shown = true;
    }
}

#[cfg(test)]
#[path = "voyage_tests.rs"]
mod voyage_tests;

#[cfg(test)]
#[path = "voyage_review_tests.rs"]
mod voyage_review_tests;
