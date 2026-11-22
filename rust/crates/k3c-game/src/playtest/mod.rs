//! Spieltest-Modus (WZ66, B-1220): `k3c-game --playtest <ordner>` zeigt ein REC-Zeichen und schreibt Marker, die jeder
//! Spieler per Pad (LT + RT gleichzeitig) oder Tastatur (Einfg) setzt, samt Spielkontext nach `<ordner>/markers.jsonl`.
//!
//! Logik und Datei liegen hier, das Spiel zeichnet nur (`hud::rec`). Ohne den Schalter bleibt der Spielablauf
//! unverändert: `game::run::PlaytestApp` umhüllt das Spiel nur mit dem Schalter. Das Cheat-Menü (View + Y, F9) bleibt
//! unberührt. Taste B bleibt unbelegt. LT + RT und Einfg lösen ihre Spielbefehle (Skill 3, Galopp, Skill 1 im Profil
//! `pfeile`) mit aus, siehe `detect`. Austausch mit der Workbench nur über Dateien (Ordner-Vertrag B-1218).
//!
//! B-1221: Ruckler (Frame über der Schwelle, Standard 25 ms, `--playtest-jank-ms`) setzen selbst einen Marker
//! `kind: "ruckler"`, höchstens einer je 5 s, nur im laufenden Spiel (nicht beim Laden). Jeder Pad-Marker bekommt einen
//! Spielstand: das Spiel speichert über das Schnellspeichern (F5 in der Eingabe, `game_input`), ein Thread kopiert die
//! Datei nach `<ordner>/saves/marker-<nr>.json` (`savejob`); die Zeile steht danach in `markers.jsonl` und nennt die
//! Datei im Feld `save`. Am Spielstand-Format ändert das nichts.

pub mod context;
pub mod detect;
pub mod flag;
pub mod marker;
pub mod savejob;
pub mod sink;
pub mod survey;
pub mod survey_draw;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_save;
#[cfg(test)]
mod tests_survey;

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::SystemTime;

use k3c_platform::{InputState, Key, log};

use self::detect::{Detector, Trigger};
use self::marker::{Context, FrameTimes, JANK_MS, Jank, Kind, Marker, RateLimit};
use self::sink::Sink;
use crate::view::MAX_PLAYERS;
use crate::view::frame::{Frame, Seat};
use crate::view::input::{DeviceID, DeviceKind};

/// Takte (30 Hz), die „Markiert“ in der Ansicht des Spielers steht.
pub const FLASH_TICKS: u64 = 60;

/// Ein Pad-Marker, der auf seinen Spielstand wartet.
#[derive(Debug)]
struct Pending {
    marker: Marker,
    rx: Receiver<Result<String, String>>,
}

/// Zustand des Spieltests: Datei, Drosselung, Frame-Zeiten und die Rückmeldung je Spieler.
#[derive(Debug)]
pub struct Playtest {
    dir: PathBuf,
    sink: Sink,
    detector: Detector,
    limit: RateLimit,
    times: FrameTimes,
    tick: u64,
    flash_until: [u64; MAX_PLAYERS],
    jank_ms: f64,
    jank: Jank,
    /// Ordner der Spielstände des Spiels; `None`: keine Spielstände an Markern
    saves_src: Option<PathBuf>,
    save_nr: u32,
    want_save: bool,
    pending: Vec<Pending>,
    /// läuft ein Spiel (Plätze, kein Laden) im Frame des letzten Takts, jetzt und beim Bild davor
    live: bool,
    prev_live: bool,
    jank_ctx: Context,
    /// Spieler `(Slot + 1, Gerät)` des letzten laufenden Spiels, für den Fragebogen
    players: Vec<(i64, DeviceID)>,
    survey: Option<survey::Survey>,
    survey_begun: bool,
}

impl Playtest {
    /// Öffnet `<dir>/markers.jsonl`; ein nicht beschreibbarer Ordner ergibt den Fehlerzustand, das Spiel läuft weiter.
    pub fn open(dir: &Path) -> Playtest {
        Playtest {
            dir: dir.to_path_buf(),
            sink: Sink::open(dir),
            detector: Detector::default(),
            limit: RateLimit::default(),
            times: FrameTimes::default(),
            tick: 0,
            flash_until: [0; MAX_PLAYERS],
            jank_ms: JANK_MS,
            jank: Jank::default(),
            saves_src: None,
            save_nr: 0,
            want_save: false,
            pending: Vec::new(),
            live: false,
            prev_live: false,
            jank_ctx: Context::default(),
            players: Vec::new(),
            survey: None,
            survey_begun: false,
        }
    }

    /// Schwelle des Ruckler-Markers in ms und Ordner der Spielstände des Spiels (`None`: keine Spielstände an Markern).
    pub fn configure(mut self, jank_ms: f64, saves_src: Option<PathBuf>) -> Playtest {
        self.jank_ms = jank_ms;
        self.saves_src = saves_src;
        self
    }

    /// Meldet der Workbench, dass das Spiel sein erstes richtiges Bild zeigt: Datei `<ordner>/ready` mit Unix-ms. Erst dann
    /// startet die Aufnahme (Bild und Ton beginnen mit dem Spiel, nicht beim Laden). Ein Fehler wird ignoriert: Die
    /// Workbench startet nach einer Frist auch ohne die Datei.
    pub fn announce_ready(&self, now_ms: u64) {
        let _ = std::fs::write(self.dir.join("ready"), now_ms.to_string());
    }

    /// Fehlerzustand (REC durchgestrichen).
    pub fn failed(&self) -> bool {
        self.sink.failed()
    }

    /// Die Eingabe für das Spiel: unverändert, nur wenn dieser Takt einen Spielstand braucht, mit F5 (Schnellspeichern).
    pub fn game_input<'a>(&self, input: &'a InputState) -> Cow<'a, InputState> {
        if !self.want_save {
            return Cow::Borrowed(input);
        }
        let mut i = input.clone();
        i.keys.press(Key::F5);
        Cow::Owned(i)
    }

    /// Meldet ein gezeichnetes Bild: Zeit `now_ms` (Unix-ms) und Dauer seit dem vorigen in ms. Ein Bild über der Schwelle
    /// im laufenden Spiel (nicht beim Laden, nicht das erste danach) ergibt einen Ruckler-Marker, entprellt auf 5 s.
    pub fn frame_done(&mut self, now_ms: u64, ms: f64) {
        self.times.push(now_ms, ms);
        let ok = self.live && self.prev_live;
        self.prev_live = self.live;
        if !ok || ms < self.jank_ms {
            return;
        }
        if let Some(skipped) = self.jank.hit(now_ms) {
            let m = Marker {
                ts: now_ms,
                kind: Kind::Ruckler,
                ctx: self.jank_ctx.clone(),
                worst_frame_ms: self.times.worst(now_ms),
                frame_ms: Some(ms),
                skipped: Some(skipped),
                ..Marker::default()
            };
            self.sink.write(&m);
        }
    }

    /// Zeigt der Spieler im Platz `slot` (`Seat::player`) gerade „Markiert“?
    pub fn marked(&self, slot: i64) -> bool {
        usize::try_from(slot)
            .ok()
            .and_then(|s| self.flash_until.get(s))
            .is_some_and(|&t| self.tick < t)
    }

    /// Ein Takt: erkennt Marker-Tasten in `input`, legt je erlaubtem Pad-Marker eine Zeile an und liefert die Marker.
    /// Mit Spielstand-Ordner und laufendem Spiel erscheint die Zeile erst, wenn der Spielstand kopiert ist (wenige
    /// Takte), sonst sofort. `frame` ist der Frame des vorigen Takts (`None` vor dem Start), `now_ms` die Systemuhr.
    pub fn step(&mut self, input: &InputState, frame: Option<&Frame>, now_ms: u64) -> Vec<Marker> {
        self.tick += 1;
        self.want_save = false;
        self.poll_pending();
        let seats = frame.map_or(&[][..], |f| f.seats.as_slice());
        self.live = frame.is_some_and(|f| !f.seats.is_empty() && !f.loading);
        self.jank_ctx = frame
            .zip(seats.first())
            .map(|(f, s)| context::of(f, s))
            .unwrap_or_default();
        if !seats.is_empty() {
            self.players = seats.iter().map(|s| (s.player + 1, s.device)).collect();
        }
        let can_save = self.live && frame.is_some_and(|f| !f.paused);
        let mut out = Vec::new();
        for t in self.detector.poll(input) {
            let seat = seat_of(seats, &t);
            let player = seat.map_or_else(|| fallback_player(&t), |s| s.player + 1);
            if !self.limit.allow(player, now_ms) {
                continue;
            }
            let m = Marker {
                ts: now_ms,
                player,
                ctx: frame.zip(seat).map(|(f, s)| context::of(f, s)).unwrap_or_default(),
                worst_frame_ms: self.times.worst(now_ms),
                ..Marker::default()
            };
            out.push(m.clone());
            self.emit(m, can_save);
            if let Some(s) = seat
                .and_then(|s| usize::try_from(s.player).ok())
                .filter(|s| *s < MAX_PLAYERS)
            {
                self.flash_until[s] = self.tick + FLASH_TICKS;
            }
        }
        out
    }

    /// Der Fragebogen, solange er offen ist (zu zeichnen mit `survey_draw::draw`).
    pub fn survey(&self) -> Option<&survey::Survey> {
        self.survey.as_ref()
    }

    /// Das Spiel ist beendet: öffnet einmal den Fragebogen für die Spieler des letzten Spiels (ohne Spieler: keiner,
    /// keine Datei). Solange er offen ist, darf das Fenster nicht schließen.
    pub fn survey_begin(&mut self, now_ms: u64) {
        if self.survey_begun {
            return;
        }
        self.survey_begun = true;
        if !self.players.is_empty() {
            self.survey = Some(survey::Survey::new(&self.players, now_ms));
        }
    }

    /// Ein Takt des offenen Fragebogens; sind alle fertig, steht `survey.json` im Ordner (atomar) und er schließt.
    pub fn survey_step(&mut self, input: &InputState) {
        let Some(sv) = self.survey.as_mut() else {
            return;
        };
        if !sv.step(input) {
            return;
        }
        match sv.write(&self.dir) {
            Ok(()) => log::info("✅ Fragebogen geschrieben", &[("datei", &survey::FILE)]),
            Err(e) => log::error("❌ Fragebogen nicht geschrieben", &[("fehler", &e)]),
        }
        self.survey = None;
    }

    /// Schreibt den Pad-Marker, mit Spielstand-Auftrag erst nach dem Kopieren.
    fn emit(&mut self, m: Marker, can_save: bool) {
        match self.saves_src.clone().filter(|_| can_save) {
            Some(src) => {
                self.save_nr += 1;
                self.want_save = true;
                let rx = savejob::spawn_copy(src, self.dir.clone(), self.save_nr, SystemTime::now());
                self.pending.push(Pending { marker: m, rx });
            }
            None => self.sink.write(&m),
        }
    }

    /// Schreibt die Marker, deren Spielstand fertig (oder gescheitert: dann ohne `save`, Log ❌) ist.
    fn poll_pending(&mut self) {
        for mut p in std::mem::take(&mut self.pending) {
            match p.rx.try_recv() {
                Err(TryRecvError::Empty) => {
                    self.pending.push(p);
                    continue;
                }
                Ok(Ok(name)) => p.marker.save = Some(name),
                Ok(Err(e)) => log::error("❌ Spielstand am Marker nicht abgelegt", &[("grund", &e)]),
                Err(TryRecvError::Disconnected) => log::error("❌ Spielstand am Marker abgebrochen", &[]),
            }
            self.sink.write(&p.marker);
        }
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
