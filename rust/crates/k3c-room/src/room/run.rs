//! Der Lauf der Räume: Takt, Absturz-Schutz, Sieg, Tick-Zeiten. Port von `engine/room/run.go`.
//!
//! Gos `Manager.Run` (Goroutinen, `time.Ticker`, `context`) entfällt: der Aufrufer (RU27) ruft `tick_all` mit 30 Hz und
//! `sweep_if_due` regelmäßig; `Room::tick` steht in `actions.rs` (RU21.4). Ein Panic im Tick schließt nur diesen Raum
//! (`room_closed` an seine Geräte), der Fehler bleibt im Log und in `Manager::failures`.
use std::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind};

use super::env::{Env, Store};
use super::log::{Level, Log};
use super::manager::Manager;
use super::types::{DURATIONS, MonarchState, Room};
use crate::store::Stamp;

/// Sekunden zwischen zwei Aufräum-Läufen (Go: Ticker der Sweeps).
pub const SWEEP_EVERY_SECS: i64 = 1;
/// Sekunden zwischen zwei Takt-Statistiken je Raum (Go `statsEvery`).
pub const STATS_EVERY_SECS: i64 = 60;

/// Ein abgestürzter Raum.
#[derive(Debug, Clone, PartialEq)]
pub struct Failure {
    pub code: String,
    pub name: String,
    pub error: String,
    pub at: Stamp,
}

/// Der Text einer Panic wie Go `fmt.Sprint(p)`.
fn panic_text(p: &(dyn Any + Send)) -> String {
    if let Some(s) = p.downcast_ref::<&str>() {
        return (*s).to_string();
    }
    if let Some(s) = p.downcast_ref::<String>() {
        return s.clone();
    }
    "unbekannter Fehler".to_string()
}

impl Room {
    /// Meldet, ob die Insel im letzten Tick gewonnen wurde (`Island::won` wurde wahr). Das Ereignis `victory` liegt in
    /// Stufe 0 und erreicht ein Gerät ohne Spieler dort nicht; Zeitraffer rechnet mehrere Schritte je Tick (B-813).
    pub fn victory(&self) -> bool {
        self.won_now
    }

    /// Letzte Tick-Dauer und p99 der letzten Ticks in Millisekunden.
    pub fn tick_ms(&self) -> (f64, f64) {
        let Some(&last) = self.durations.last() else {
            return (0.0, 0.0);
        };
        let mut sorted = self.durations.clone();
        sorted.sort_unstable();
        let ms = |d: i64| d as f64 / 1_000_000.0;
        (ms(last), ms(sorted[(sorted.len() * 99) / 100]))
    }

    /// Merkt die Dauer eines Ticks (Ring von `DURATIONS` Werten) und meldet langsame (`note_tick`).
    pub fn record_tick(&mut self, env: &mut Env, took_nanos: i64) {
        self.note_tick(env, took_nanos);
        self.durations.push(took_nanos);
        if self.durations.len() > DURATIONS {
            self.durations.remove(0);
        }
    }

    /// Schließt einen abgestürzten Raum, ohne ihn zu speichern (sein Zustand ist nicht mehr sicher).
    fn close_crashed(&mut self) {
        self.closed = true;
        for d in self.devices.values_mut().filter(|d| d.connected) {
            d.peer.closed(false);
        }
    }
}

impl<S: Store, L: Log> Manager<S, L> {
    /// Ruft `f` auf dem Raum `code` auf; ein Panic schließt den Raum ohne Speichern (`crash`) und ergibt `false`.
    /// Sonst das Ergebnis von `f` (`false` auch, wenn es den Raum nicht gibt).
    pub fn guard(&mut self, code: &str, f: impl FnOnce(&mut Room, &mut Env) -> bool) -> bool {
        let (rooms, mut env) = self.split();
        let Some(room) = rooms.get_mut(code) else {
            return false;
        };
        match catch_unwind(AssertUnwindSafe(|| f(room, &mut env))) {
            Ok(alive) => alive,
            Err(p) => {
                let msg = panic_text(&*p);
                self.crash(code, &msg);
                false
            }
        }
    }

    /// Rechnet einen Tick wie `Room::tick` für Aufrufer, die den Raum selbst takten (natives Spiel, B-803): Ein Panic
    /// schließt den Raum wie im Server ohne Speichern (`crash`, steht danach in `failures`). `false`: Raum ist geschlossen.
    pub fn safe_tick(&mut self, code: &str) -> bool {
        self.guard(code, |r, env| r.tick(env))
    }

    /// Rechnet einen Tick mit Messung und fängt einen Panic. `false`: Raum ist geschlossen.
    pub fn timed_tick(&mut self, code: &str) -> bool {
        let start = (self.now)();
        if !self.guard(code, |r, env| r.tick(env)) {
            return false;
        }
        let took = (self.now)().since_nanos(start);
        let took = i64::try_from(took).unwrap_or(i64::MAX);
        self.guard(code, |r, env| {
            r.record_tick(env, took);
            true
        })
    }

    /// Schließt einen abgestürzten Raum, ohne ihn zu speichern, und merkt den Fehler (die letzten zehn).
    pub fn crash(&mut self, code: &str, msg: &str) {
        let Some(mut r) = self.rooms.remove(code) else {
            return;
        };
        self.log.error(
            "💥 Raum abgestürzt",
            &[("ns", &"room"), ("room", &r.code), ("save", &r.name), ("err", &msg)],
        );
        r.close_crashed();
        let at = (self.now)();
        self.failures.push(Failure {
            code: r.code.clone(),
            name: r.name.clone(),
            error: msg.to_string(),
            at,
        });
        if self.failures.len() > 10 {
            self.failures.remove(0);
        }
    }

    /// Ein Takt über alle Räume in Code-Reihenfolge (Go: eine Goroutine je Raum mit 30 Hz). Ein Absturz trifft nur den
    /// eigenen Raum. Den Takt gibt der Aufrufer vor.
    pub fn tick_all(&mut self) {
        let codes: Vec<String> = self.rooms.keys().cloned().collect();
        for code in codes {
            self.timed_tick(&code);
        }
    }

    /// Prüft die Fristen (jede Sekunde) und schreibt die Takt-Statistik (jede Minute), gemessen an der Uhr des
    /// Managers; der Aufrufer ruft es regelmäßig. Der erste Aufruf setzt nur die Zeitbasis.
    pub fn sweep_if_due(&mut self) {
        let now = (self.now)();
        let due = |last: &mut Option<Stamp>, every: i64| match *last {
            None => {
                *last = Some(now);
                false
            }
            Some(l) if now.since_nanos(l) >= i128::from(every) * 1_000_000_000 => {
                *last = Some(now);
                true
            }
            Some(_) => false,
        };
        let (mut sweep, mut stats) = (self.last_sweep, self.last_stats);
        let (do_sweep, do_stats) = (due(&mut sweep, SWEEP_EVERY_SECS), due(&mut stats, STATS_EVERY_SECS));
        (self.last_sweep, self.last_stats) = (sweep, stats);
        if do_sweep {
            self.safe_sweep();
        }
        if do_stats {
            self.log_stats();
        }
    }

    /// Ein Panic beim Aufräumen (z. B. beim Speichern) beendet nicht den Server.
    pub fn safe_sweep(&mut self) {
        if let Err(p) = catch_unwind(AssertUnwindSafe(|| self.sweep())) {
            self.log.error("💥 Sweep abgestürzt", &[("err", &panic_text(&*p))]);
        }
    }

    /// Schreibt die Takt-Statistik aller laufenden Räume (einmal pro `STATS_EVERY_SECS`).
    pub fn log_stats(&mut self) {
        let (rooms, mut env) = self.split();
        for r in rooms.values() {
            let n = r.connected();
            if n == 0 {
                continue;
            }
            let (last, p99) = r.tick_ms();
            r.log(
                &mut env,
                Level::Info,
                "💓 Raum-Takt",
                &[
                    ("tick", &r.tick),
                    ("geraete", &n),
                    ("spieler", &r.count(MonarchState::Taken)),
                    ("tickMs", &last),
                    ("p99Ms", &p99),
                ],
            );
        }
    }
}
