//! Spieltest-Modus (WZ66, B-1220): `k3c-game --playtest <ordner>` zeigt ein REC-Zeichen und schreibt Marker, die jeder
//! Spieler per Pad (LT + RT gleichzeitig) oder Tastatur (Einfg) setzt, samt Spielkontext nach `<ordner>/markers.jsonl`.
//!
//! Logik und Datei liegen hier, das Spiel zeichnet nur (`hud::rec`). Ohne den Schalter bleibt der Spielablauf
//! unverändert: `game::run::PlaytestApp` umhüllt das Spiel nur mit dem Schalter. Das Cheat-Menü (View + Y, F9) bleibt
//! unberührt. Taste B bleibt unbelegt. LT + RT und Einfg lösen ihre
//! Spielbefehle (Skill 3, Galopp, Skill 1 im Profil `pfeile`) mit aus, siehe `detect`. Austausch mit der Workbench nur über Dateien (Ordner-Vertrag B-1218).

pub mod context;
pub mod detect;
pub mod flag;
pub mod marker;
pub mod sink;

#[cfg(test)]
mod tests;

use std::path::Path;

use k3c_platform::InputState;

use self::detect::{Detector, Trigger};
use self::marker::{FrameTimes, Marker, RateLimit};
use self::sink::Sink;
use crate::view::MAX_PLAYERS;
use crate::view::frame::{Frame, Seat};
use crate::view::input::DeviceKind;

/// Takte (30 Hz), die „Markiert“ in der Ansicht des Spielers steht.
pub const FLASH_TICKS: u64 = 60;

/// Zustand des Spieltests: Datei, Drosselung, Frame-Zeiten und die Rückmeldung je Spieler.
#[derive(Debug)]
pub struct Playtest {
    sink: Sink,
    detector: Detector,
    limit: RateLimit,
    times: FrameTimes,
    tick: u64,
    flash_until: [u64; MAX_PLAYERS],
}

impl Playtest {
    /// Öffnet `<dir>/markers.jsonl`; ein nicht beschreibbarer Ordner ergibt den Fehlerzustand, das Spiel läuft weiter.
    pub fn open(dir: &Path) -> Playtest {
        Playtest {
            sink: Sink::open(dir),
            detector: Detector::default(),
            limit: RateLimit::default(),
            times: FrameTimes::default(),
            tick: 0,
            flash_until: [0; MAX_PLAYERS],
        }
    }

    /// Fehlerzustand (REC durchgestrichen).
    pub fn failed(&self) -> bool {
        self.sink.failed()
    }

    /// Meldet ein gezeichnetes Bild: Zeit `now_ms` (Unix-ms) und Dauer seit dem vorigen in ms.
    pub fn frame_done(&mut self, now_ms: u64, ms: f64) {
        self.times.push(now_ms, ms);
    }

    /// Zeigt der Spieler im Platz `slot` (`Seat::player`) gerade „Markiert“?
    pub fn marked(&self, slot: i64) -> bool {
        usize::try_from(slot)
            .ok()
            .and_then(|s| self.flash_until.get(s))
            .is_some_and(|&t| self.tick < t)
    }

    /// Ein Takt: erkennt Marker-Tasten in `input`, schreibt je erlaubtem Marker eine Zeile und liefert die Marker.
    /// `frame` ist der Frame des vorigen Takts (`None` vor dem Start), `now_ms` die Systemuhr in Unix-ms.
    pub fn step(&mut self, input: &InputState, frame: Option<&Frame>, now_ms: u64) -> Vec<Marker> {
        self.tick += 1;
        let mut out = Vec::new();
        let seats = frame.map_or(&[][..], |f| f.seats.as_slice());
        for t in self.detector.poll(input) {
            let seat = seat_of(seats, &t);
            let player = seat.map_or_else(|| fallback_player(&t), |s| s.player + 1);
            if !self.limit.allow(player, now_ms) {
                continue;
            }
            let ctx = frame.zip(seat).map(|(f, s)| context::of(f, s)).unwrap_or_default();
            let m = Marker {
                ts: now_ms,
                player,
                ctx,
                worst_frame_ms: self.times.worst(now_ms),
            };
            self.sink.write(&m);
            if let Some(s) = seat
                .and_then(|s| usize::try_from(s.player).ok())
                .filter(|s| *s < MAX_PLAYERS)
            {
                self.flash_until[s] = self.tick + FLASH_TICKS;
            }
            out.push(m);
        }
        out
    }
}

/// Der Platz des Auslösers: der seines Geräts, bei Einfg ohne Tastatur-Platz der erste.
fn seat_of<'a>(seats: &'a [Seat], t: &Trigger) -> Option<&'a Seat> {
    seats
        .iter()
        .find(|s| s.device == t.device)
        .or_else(|| (t.device.kind == DeviceKind::Keyboard).then(|| seats.first()).flatten())
}

/// Ohne Platz (Menü vor dem Spiel): Spieler nach der Reihenfolge des Geräts.
fn fallback_player(t: &Trigger) -> i64 {
    i64::try_from(t.ordinal).unwrap_or(0).min(MAX_PLAYERS as i64 - 1) + 1
}
