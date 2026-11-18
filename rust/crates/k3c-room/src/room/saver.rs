//! Speichern (`docs/protocol.md` › Spielstand, B-276). Port von `engine/room/saver.go`.
//!
//! Gos Schreib-Goroutine je Raum (`wg`, `pending`, `busy`, „der neueste wartende Stand gewinnt“) wird die Naht
//! `SaveWriter`: Der Raum kodiert den Stand im Tick und übergibt ihn dem Schreiber. `InlineWriter` (Standard) schreibt
//! synchron über den `Store` mit derselben Zustandsmaschine wie Go; ein Schreiber mit eigenem Thread, der den Tick nicht
//! blockiert, gehört in den Aufrufer (RU27, `k3c-game`). Mit `InlineWriter` blockiert eine Speicherung den Tick (Risiko
//! Ruckler über `SLOW_SAVE_MS` hinaus, Messung RU39). Fehler landen im Log, der Raum läuft weiter.
use std::fmt::Display;
use std::panic::{AssertUnwindSafe, catch_unwind};

use k3c_sim::state::Event;

use super::env::Env;
use super::log::Level;
use super::run::panic_text;
use super::types::{Room, SAVE_TICKS, TICK_HZ};
use crate::fehler::Fehler;

/// Zielwert einer Speicherung (B-147, S2.2): ein Tick bei 30 Hz.
pub const SLOW_SAVE_MS: i64 = 1000 / TICK_HZ;

/// Schreibt die kodierten Stände eines Raums (Go: Goroutine `writeSaves`). Der Raum ruft `submit` im Tick; `flush` wartet,
/// bis kein Stand mehr aussteht; `take_written` meldet einmal, dass seit dem letzten Aufruf ein Stand gelungen geschrieben
/// wurde (Ereignis `saved`, B-186). Abweichung von der Vorentscheidung: `submit` bekommt die Umgebung (Store, Log, Uhr) und
/// den Raum-Code fürs Log, weil der Raum-Crate keinen geteilten Store kennt.
pub trait SaveWriter {
    /// Übergibt den Stand `data` vom Takt `tick` für den Spielstand `name` des Raums `code`; der neueste wartende gewinnt.
    fn submit(&mut self, env: &mut Env, code: &str, name: &str, data: Vec<u8>, tick: i64);
    /// Wartet, bis kein Stand mehr aussteht (Go `flush`).
    fn flush(&mut self);
    /// Seit dem letzten Aufruf gelungen geschrieben? Setzt den Merker zurück.
    fn take_written(&mut self) -> bool;
}

/// Der synchrone Schreiber: dieselbe Zustandsmaschine wie Go (`pending`, `busy`, `written`), geschrieben wird sofort im
/// Aufruf von `submit`. Ein Panic im `Store` beendet nur diesen Schreibvorgang (`catch_unwind`, Gos `recover`).
#[derive(Debug, Default)]
pub struct InlineWriter {
    pending: Option<(Vec<u8>, i64)>,
    busy: bool,
    written: bool,
}

impl InlineWriter {
    /// Go `writeSaves`: schreibt wartende Stände, bis keiner mehr wartet; ein Panic beendet den Vorgang.
    fn write_saves(&mut self, env: &mut Env, code: &str, name: &str) {
        while let Some((data, tick)) = self.pending.take() {
            match write_save(env, code, name, &data, tick) {
                Some(true) => self.written = true,
                Some(false) => {}
                None => break,
            }
        }
        self.busy = false;
    }
}

impl SaveWriter for InlineWriter {
    fn submit(&mut self, env: &mut Env, code: &str, name: &str, data: Vec<u8>, tick: i64) {
        self.pending = Some((data, tick));
        if !self.busy {
            self.busy = true;
            self.write_saves(env, code, name);
        }
    }

    /// Nach `submit` steht nichts mehr aus.
    fn flush(&mut self) {}

    fn take_written(&mut self) -> bool {
        std::mem::take(&mut self.written)
    }
}

/// Meldet mit den Feldern des Raums (`ns`, `room`, `save`) vorweg, wie `Room::log`.
fn room_log(env: &mut Env, level: Level, code: &str, name: &str, msg: &str, fields: &[(&str, &dyn Display)]) {
    let mut all: Vec<(&str, &dyn Display)> = vec![("ns", &"room"), ("room", &code), ("save", &name)];
    all.extend_from_slice(fields);
    env.log.log(level, msg, &all);
}

/// Go `write`: schreibt einen kodierten Stand über den Store und meldet das Ergebnis im Log; die Dauer misst die Uhr des
/// Managers. `Some(true)`: geschrieben, `Some(false)`: Fehler, `None`: der Store hat gepanikt.
pub fn write_save(env: &mut Env, code: &str, name: &str, data: &[u8], tick: i64) -> Option<bool> {
    let start = (env.clock)();
    let res = catch_unwind(AssertUnwindSafe(|| env.store.store(name, data)));
    let ms = i64::try_from((env.clock)().since_nanos(start) / 1_000_000).unwrap_or(i64::MAX);
    match res {
        Err(p) => {
            let err = panic_text(p.as_ref());
            room_log(
                env,
                Level::Error,
                code,
                name,
                "💥 Spielstand nicht gespeichert, Speichern abgestürzt",
                &[("err", &err)],
            );
            None
        }
        Ok(Err(e)) => {
            let fields: [(&str, &dyn Display); 2] = [("err", &e), ("ms", &ms)];
            room_log(
                env,
                Level::Error,
                code,
                name,
                "💥 Spielstand nicht gespeichert",
                &fields,
            );
            Some(false)
        }
        Ok(Ok(backup)) => {
            let fields: [(&str, &dyn Display); 3] = [("tick", &tick), ("sicherung", &backup), ("ms", &ms)];
            if ms > SLOW_SAVE_MS {
                let msg = "🐢 Spielstand gespeichert, langsamer als ein Tick";
                room_log(env, Level::Warn, code, name, msg, &fields);
            } else {
                room_log(env, Level::Debug, code, name, "💾 Spielstand gespeichert", &fields);
            }
            Some(true)
        }
    }
}

impl Room {
    /// Speichert im Takt `SAVE_TICKS`, bei Tagesanbruch und nach einem Stufen- oder Inselwechsel (B-186, im Tick).
    pub fn autosave(&mut self, env: &mut Env, travelled: bool, dawn: bool) {
        self.save_due += 1;
        if travelled || dawn || self.save_due >= SAVE_TICKS {
            self.save(env);
        }
    }

    /// Schickt die Zustände wie `push_states`. Wurde seit dem letzten Tick ein Spielstand geschrieben, trägt jede Stufe
    /// dabei das Ereignis `saved`; danach verschwindet es wieder aus der Welt, denn es ist kein Ereignis der Simulation.
    pub fn push_states_saved(&mut self, env: &mut Env) -> bool {
        let written = self.saves.take_written();
        if !written || self.isl.over {
            // nach Game Over meldet der Raum kein „gesichert“ mehr (B-344), auch für ältere Stände
            return self.push_states(env);
        }
        for (i, w) in self.isl.stages.iter_mut().enumerate() {
            let mut ev = Event::new();
            ev.insert("type".into(), "saved".into());
            ev.insert("stage".into(), i.into());
            w.events.push(ev);
        }
        let sent = self.push_states(env);
        for w in &mut self.isl.stages {
            w.events.pop();
        }
        sent
    }

    /// Kodiert den Spielstand und übergibt ihn dem Schreiber (im Takt, beim Verlassen und Trennen).
    pub fn save(&mut self, env: &mut Env) {
        self.save_due = 0;
        if self.game_over(env) {
            return;
        }
        let data = match self.encode(env) {
            Ok(d) => d,
            Err(e) => return self.log(env, Level::Error, "💥 Spielstand nicht gespeichert", &[("err", &e)]),
        };
        self.saves.submit(env, &self.code, &self.name, data, self.tick);
    }

    /// Wartet, bis kein Spielstand mehr aussteht.
    pub fn flush(&mut self) {
        self.saves.flush();
    }

    /// Speichert synchron (Aufräumen, Herunterfahren; Go `saveNow`): erst ausstehende Stände, dann der aktuelle.
    pub fn save_sync(&mut self, env: &mut Env) {
        self.flush();
        if self.game_over(env) {
            return;
        }
        match self.encode(env) {
            Ok(data) => {
                write_save(env, &self.code, &self.name, &data, self.tick);
            }
            Err(e) => self.log(env, Level::Error, "💥 Spielstand nicht gespeichert", &[("err", &e)]),
        }
    }

    /// Sichert den Spielstand sofort (Diagnose, B-088; Go `SaveNow`) und meldet den Fehler, statt ihn nur zu loggen.
    pub fn save_now(&mut self, env: &mut Env) -> Result<String, Fehler> {
        if self.closed {
            return Err(Fehler::Closed);
        }
        self.flush();
        if self.game_over(env) {
            return Err(Fehler::GameOver);
        }
        let data = self.encode(env)?;
        env.store.store(&self.name, &data)
    }

    /// Die Insel ist „Komplett verloren“ (`Island::over`, B-344): Der Raum schreibt dann keinen Spielstand mehr, der
    /// zuletzt gespeicherte bleibt Byte für Byte gleich.
    pub fn game_over(&self, env: &mut Env) -> bool {
        if !self.isl.over {
            return false;
        }
        self.log(
            env,
            Level::Info,
            "🚫 Spielstand nicht gespeichert, Insel komplett verloren",
            &[("tick", &self.tick)],
        );
        true
    }

    /// Der Spielstand als JSON (Format wie Go, Version 9; `savedAt` im Muster `2006-01-02T15:04:05.000Z`).
    pub fn encode(&self, env: &Env) -> Result<Vec<u8>, Fehler> {
        let save = self
            .isl
            .to_save(&env.now.format_report())
            .map_err(|e| Fehler::Io(e.to_string()))?;
        serde_json::to_vec(&save).map_err(|e| Fehler::Io(e.to_string()))
    }
}
