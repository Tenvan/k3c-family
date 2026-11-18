//! Port von `native/k3c-game/local/step.go` (ohne den Typ `peer`, der steht in `peer.rs`): Befehle, der Takt der Sitzung,
//! der Ausgang des Spiels und das Melden der Speicherungen. `impl Sim for Session` verbindet es mit der Schleife.
//!
//! Abweichungen von Go: (1) Der Raum übergibt `Peer::state` nur JSON; `Session::step` kopiert die Welt jeder aktiven Stufe
//! je Takt tief aus dem Raum in einen `Rc<World>` (`Frame::stages[i].world`), Go gibt den Zeiger ohne Kopie weiter.
//! (2) `Room::tick` speichert synchron (Autosave, Stufenwechsel); `collect_saves` im selben Step sieht die Ergebnisse, ein
//! Warten auf eine Schreib-Goroutine (`awaitSaves`, `SAVE_WAIT`, Aufnahme-Modus) entfällt. (3) `Frame` besitzt seine Listen:
//! `stages`, `seats`, `events` sind je Takt neue `Vec`, die Puffer der Session (`seat_buf`, `StageState`) behalten ihre
//! Kapazität. (4) `running` und `timescale` sind Teil von `dev.go` (RU27.3); sie stehen hier, weil `step` sie braucht.

use std::any::Any;
use std::collections::BTreeMap;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::Instant;

use k3c_room::room::{Log as _, Seat as RoomSeat, TICK_HZ};

use super::error::{UserError, describe, err_no_game, user_err};
use super::session::{CONN, DEVICE_ID, Session};
use super::watched::lock;
use crate::game::sim::Sim;
use crate::view::frame::{CueKind, Frame, Outcome, Seat};
use crate::view::input::{Command, DeviceID};
use crate::view::session::Fehler;

/// Der Takt von `step` (Entscheidung B-793: Simulation 30 Hz ohne Zwischenbilder).
pub const STEP_HZ: i64 = TICK_HZ;

/// Der Text einer Panic wie Go `fmt.Sprint(p)`.
fn panic_text(p: &(dyn Any + Send)) -> String {
    if let Some(s) = p.downcast_ref::<&str>() {
        return (*s).to_string();
    }
    p.downcast_ref::<String>()
        .cloned()
        .unwrap_or_else(|| "unbekannter Fehler".to_string())
}

/// Die Plätze der lokalen Spieler nach Slot (Go: `appendSeats`).
fn fill_seats(dst: &mut Vec<Seat>, seats: &[RoomSeat], devs: &BTreeMap<i64, DeviceID>) {
    dst.extend(seats.iter().map(|st| Seat {
        player: st.slot,
        device: devs.get(&st.slot).copied().unwrap_or_default(),
        monarch: st.monarch,
        stage: st.stage,
        lost: false,
    }));
}

impl Sim for Session {
    /// Setzt die gehaltene Eingabe des lokalen Spielers; sie gilt ab dem nächsten Step (wie `Room::input`). Ohne laufendes
    /// Spiel geschieht nichts.
    fn command(&mut self, player: i64, c: Command) -> Result<(), Fehler> {
        self.send_command(player, &c).map_err(Fehler::from)
    }

    /// Rechnet einen Takt (1/30 s), außer ohne Spiel, in der Pause und im Halt (außer Einzelschritt), und liefert den
    /// Frame. Die Schleife ruft `step` einmal je Update bei 30 Updates je Sekunde.
    fn step(&mut self) -> Frame {
        self.step_frame()
    }
}

impl Session {
    fn send_command(&mut self, player: i64, c: &Command) -> Result<(), UserError> {
        if self.m.is_none() {
            return Ok(());
        }
        if !self.devs.contains_key(&player) {
            return Err(user_err(format!("Spieler {} spielt nicht", player + 1), None));
        }
        self.cmd.clear();
        self.cmd.insert(player, c.sim());
        let cmd = &self.cmd;
        let res = self
            .m
            .as_mut()
            .and_then(|m| m.rooms.get_mut(&self.code))
            .map(|r| r.input(DEVICE_ID, CONN, cmd));
        match res {
            Some(res) => res.map_err(|e| describe(&e, &self.name)),
            None => Err(err_no_game()),
        }
    }

    fn step_frame(&mut self) -> Frame {
        self.steps += 1;
        self.step_ms = 0.0;
        if self.running() {
            let t0 = Instant::now();
            self.tick();
            self.step_ms = t0.elapsed().as_secs_f64() * 1000.0;
        }
        self.collect_saves();
        let timescale = self.timescale();
        let mut f = Frame {
            clock: self.steps as f64 / STEP_HZ as f64,
            paused: self.paused || self.halt,
            timescale,
            cues: std::mem::take(&mut self.cues),
            settings: self.settings.clone(),
            step_ms: self.step_ms,
            outcome: self.ended,
            ..Frame::default()
        };
        let Some(peer) = &self.peer else {
            return f;
        };
        {
            let d = lock(peer);
            self.camp.follow(&self.stages, &d.seats);
            f.tick = d.tick;
            f.stages = self.stages.stages().to_vec();
            self.seat_buf.clear();
            fill_seats(&mut self.seat_buf, &d.seats, &self.devs);
            f.seats.extend_from_slice(&self.seat_buf);
            f.events.append(&mut self.stages.events);
        }
        f.route = self.route_list().to_vec();
        f.campaign = self.camp.frame(!f.stages.is_empty() && !f.seats.is_empty());
        f
    }

    /// Meldet, ob `step` in diesem Aufruf einen Takt rechnet (Spiel läuft, keine Pause, kein Halt außer Einzelschritt).
    pub(super) fn running(&mut self) -> bool {
        if self.m.is_none() || self.paused {
            self.single = false; // kein Einzelschritt auf Vorrat
            return false;
        }
        if !self.halt {
            return true;
        }
        std::mem::take(&mut self.single)
    }

    /// Der Zeitraffer des Raums (1 ohne Spiel).
    pub(super) fn timescale(&mut self) -> i64 {
        self.with_room(|r, _| r.scale()).unwrap_or(1)
    }

    /// Taktet den Raum einmal und bestimmt den Ausgang. Ein Panic im Takt schließt der Raum selbst ohne Speichern
    /// (`Manager::safe_tick`, B-803); der letzte Spielstand bleibt. Ein Panic außerhalb des Raum-Takts (Vorbereitung,
    /// Ausgang) wird gemeldet, statt das Programm zu beenden.
    pub(super) fn tick(&mut self) {
        if let Err(p) = catch_unwind(AssertUnwindSafe(|| self.tick_room())) {
            self.crash(&format!("Fehler im Takt: {}", panic_text(&*p)));
        }
    }

    fn tick_room(&mut self) {
        if let Some(p) = &self.peer {
            lock(p).begin();
        }
        self.stages.begin();
        let alive = self.m.as_mut().is_some_and(|m| m.safe_tick(&self.code));
        if !alive {
            let msg = self.failure();
            self.crash(&msg);
            return;
        }
        self.sync_stages();
        self.judge();
    }

    /// Liest die Welten der Stufen, die seit dem letzten Aufruf einen Zustand bekamen, aus dem Raum (Go: `Peer.State`
    /// bekommt sie). Der Raum schickt Zustände im Takt und bei Beitritt, Austritt und Start; die Session ruft es nach
    /// jedem davon.
    pub(super) fn sync_stages(&mut self) {
        let Some(peer) = &self.peer else {
            return;
        };
        let mut d = lock(peer);
        self.stages.replaced.extend_from_slice(&d.replaced);
        let (tick, stages) = (d.tick, &mut self.stages);
        if let Some(m) = self.m.as_mut() {
            m.with_room(&self.code, |r, _| {
                for &i in &d.active {
                    if let Some(w) = usize::try_from(i).ok().and_then(|i| r.isl.stages.get(i)) {
                        stages.absorb(tick, i, w);
                    }
                }
            });
        }
        d.active.clear();
        d.replaced.clear();
    }

    /// Setzt den Ausgang des Spiels: `victory` bzw. `gameOver` unter den Ereignissen der Stufen mit lokalen Spielern,
    /// sonst verloren, wenn eine Stufe trotz Takt nicht weiterläuft (`step_island` rechnet nach `Island::over` nichts mehr,
    /// das Ereignis kann in einer Stufe ohne lokalen Spieler liegen). Der erste Ausgang bleibt.
    pub(super) fn judge(&mut self) {
        let stalled = self.stages.stalled();
        if self.ended != Outcome::None {
            return;
        }
        for e in &self.stages.events {
            match e.type_() {
                "gameOver" => self.ended = Outcome::Lost,
                "victory" => self.ended = Outcome::Won,
                _ => {}
            }
            if self.ended != Outcome::None {
                break;
            }
        }
        // victory liegt in Stufe 0, auch ohne lokalen Spieler dort
        if self.ended == Outcome::None && self.with_room(|r, _| r.victory()) == Some(true) {
            self.ended = Outcome::Won;
        }
        if self.ended == Outcome::None && stalled {
            self.ended = Outcome::Lost;
        }
        if self.ended != Outcome::None {
            let (name, ausgang) = (self.name.clone(), format!("{:?}", self.ended));
            self.log.info("🚦 Spielende", &[("save", &name), ("ausgang", &ausgang)]);
        }
    }

    /// Der Grund, aus dem der Raum geschlossen ist: der Panic aus `Manager::failures`, sonst „Raum geschlossen“.
    fn failure(&self) -> String {
        self.m
            .as_ref()
            .and_then(|m| m.failures.last())
            .map_or_else(|| "Raum geschlossen".to_string(), |f| f.error.clone())
    }

    /// Verwirft Raum und Manager, ohne zu speichern (der Zustand ist nicht mehr sicher), und meldet es dem Menü.
    fn crash(&mut self, msg: &str) {
        let name = self.name.clone();
        self.log.error("💥 Spiel abgestürzt", &[("save", &name), ("err", &msg)]);
        self.cue(
            CueKind::Error,
            -1,
            &format!("Spiel abgestürzt, der letzte Spielstand bleibt: {msg}"),
        );
        self.reset();
    }

    /// Meldet die Ergebnisse der Speicherungen seit dem letzten Frame (Autosave, Save, Close).
    fn collect_saves(&mut self) {
        let results = lock(&self.results).take();
        for res in results {
            match res {
                Some(err) => self.cue(
                    CueKind::SaveFailed,
                    -1,
                    &format!("Speichern fehlgeschlagen, das Spiel läuft weiter: {err}"),
                ),
                None => self.cue(CueKind::Saved, -1, ""),
            }
        }
    }

    /// Die Plätze des laufenden Spiels nach Slot (leer ohne Spiel); eine eigene Liste, nicht die von `Frame::seats`.
    pub(super) fn step_seats(&self) -> Vec<Seat> {
        let mut out = Vec::new();
        if let Some(p) = &self.peer {
            fill_seats(&mut out, &lock(p).seats, &self.devs);
        }
        out
    }
}
