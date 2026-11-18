//! Der Takt des Raums und die Fristen. Port von `Tick`, `pushStates` und `sweep` aus `actions.go`. Die Teilschritte laufen
//! in der Reihenfolge von Go: Eingaben, Pause, Schritte der Insel, Sieg, Zustände, Inselwechsel, Stufenwechsel, Autosave.
use std::collections::BTreeMap;

use k3c_sim::island::step_island;
use k3c_sim::state::PlayerCommand;

use super::env::Env;
use super::log::Level;
use super::types::{EMPTY_FOR_SECS, MonarchState, Room, TICK_HZ, WAIT_FOR_SECS};
use crate::store::Stamp;

/// Stufen-Indizes wie Go `%v` einer `[]int` (fehlende Stufe: −1).
fn depths_text(depths: &[Option<usize>]) -> String {
    let parts: Vec<String> = depths.iter().map(|d| d.map_or(-1, |s| s as i64).to_string()).collect();
    format!("[{}]", parts.join(" "))
}

impl Room {
    /// Rechnet einen Schritt mit 1/`TICK_HZ` Sekunden, im Zeitraffer `scale()` solche Schritte. Ein Raum ohne verbundenes
    /// Gerät ist pausiert und tickt nicht. Wechselt ein Gerät die Stufe, speichert der Raum und schickt das neue Level
    /// vor dem Zustand; dazu speichert er im Takt `SAVE_EVERY_SECS` und bei Tagesanbruch (`autosave`). `false`: Der Raum
    /// ist geschlossen.
    pub fn tick(&mut self, env: &mut Env) -> bool {
        if self.closed {
            return false;
        }
        if self.connected() == 0 {
            return true;
        }
        // Monarchen eines pausierten Geräts stehen still
        let commands: Vec<PlayerCommand> = self
            .monarchs
            .iter()
            .map(|mo| {
                if mo.state == MonarchState::Taken && !self.device_paused(&mo.device) {
                    mo.input.clone()
                } else {
                    PlayerCommand::default()
                }
            })
            .collect();
        self.sync_free();
        self.pause_tick(env);
        if let Some(f) = &mut self.before_step {
            f();
        }
        let (before, day, won) = (self.depths(), self.isl.stages[0].cycle.day, self.isl.won);
        // Zeitraffer: `scale()` Schritte mit denselben Eingaben, ein Tick; Dev- und Couch-Pause: keiner
        for _ in 0..self.steps() {
            step_island(&mut self.isl, &commands, &env.data.difficulty, 1.0 / TICK_HZ as f64);
            if let Some(met) = &mut self.met {
                met.observe(&self.isl);
            }
        }
        self.reset_scale_on_phase(env);
        self.won_now = !won && self.isl.won;
        self.tick += 1;
        // der Tag zählt bei Tagesanbruch weiter (world_cycle.go)
        let dawn = self.isl.stages[0].cycle.day > day;
        let mut travelled = self.push_states_saved(env);
        if self.switch_island(env) {
            // erst die Ereignisse des letzten Ticks der alten Insel, dann Level und Zustand der neuen (B-385)
            self.push_states(env);
            travelled = true;
        }
        let after = self.depths();
        if before != after {
            // irgendein Monarch hat die Stufe gewechselt
            travelled = true;
            let (vorher, nachher, tick) = (depths_text(&before), depths_text(&after), self.tick);
            self.log(
                env,
                Level::Info,
                "🪜 Stufenwechsel",
                &[("vorher", &vorher), ("nachher", &nachher), ("tick", &tick)],
            );
            self.broadcast_seats();
        }
        self.autosave(env, travelled, dawn);
        true
    }

    /// Schickt jedem verbundenen Gerät den Zustand seiner Stufen; ein Zustand je Stufe, alle Geräte der Stufe teilen ihn
    /// (B-276). `true`: Mindestens ein Gerät bekam ein Level (neue Stufe).
    pub fn push_states(&mut self, env: &Env) -> bool {
        let ids: Vec<String> = self
            .devices
            .iter()
            .filter(|(_, d)| d.connected)
            .map(|(id, _)| id.clone())
            .collect();
        let mut states = BTreeMap::new();
        let mut level_sent = false;
        for id in ids {
            if self.push_state(env, &id, Some(&mut states)) {
                level_sent = true;
            }
        }
        level_sent
    }

    /// Prüft die Fristen nach Wanduhr, auch im pausierten Raum. `(changed, remove)`; `remove`: Der Raum ist seit
    /// `EMPTY_FOR_SECS` leer.
    pub fn sweep(&mut self, env: &mut Env, now: Stamp) -> (bool, bool) {
        let wait = i128::from(WAIT_FOR_SECS) * 1_000_000_000;
        let due: Vec<usize> = (0..self.monarchs.len())
            .filter(|&i| {
                let mo = &self.monarchs[i];
                mo.state == MonarchState::Waiting && mo.since.is_none_or(|s| now.since_nanos(s) >= wait)
            })
            .collect();
        let changed = !due.is_empty();
        for idx in due {
            self.release(idx);
        }
        let gone: Vec<String> = self
            .devices
            .iter()
            .filter(|(id, d)| !d.connected && self.all_free(id))
            .map(|(id, _)| id.clone())
            .collect();
        for id in gone {
            self.devices.remove(&id);
        }
        if changed {
            self.log(env, Level::Info, "⏳ Wartende Monarchen nach Frist frei", &[]);
            self.sync_free();
            self.broadcast_seats();
        }
        let Some(empty_since) = self.empty_since else {
            return (changed, false);
        };
        if self.connected() > 0 || self.watching() > 0 {
            return (changed, false);
        }
        if self.is_test() {
            // B-204: ein Test-Raum wartet nur auf wartende Monarchen, nicht auf `EMPTY_FOR_SECS`
            return (changed, self.count(MonarchState::Waiting) == 0);
        }
        (
            changed,
            now.since_nanos(empty_since) >= i128::from(EMPTY_FOR_SECS) * 1_000_000_000,
        )
    }
}
