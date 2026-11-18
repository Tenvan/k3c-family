//! Aktionen eines Geräts im Raum (`docs/protocol.md` › Lokale Spieler hinzufügen/entfernen). Port von `actions.go`
//! (ohne `Tick`, das steht in `tick.rs`). Gos Peer-Vergleich `d.peer == peer` ist der Vergleich der `conn_id`: der Raum
//! besitzt die Verbindung, der Aufrufer nennt sie über ihre Kennung. Eine ersetzte Verbindung darf nichts mehr am
//! neuen Gerät ändern.
use std::collections::BTreeMap;

use k3c_sim::state::PlayerCommand;

use super::env::Env;
use super::log::{Level, short};
use super::types::{MAX_MONARCHS, MAX_SLOTS, MonarchState, Room, WAIT_FOR_SECS};
use crate::fehler::Fehler;

impl Room {
    /// Ist das Gerät `id` mit der Verbindung `conn` im offenen Raum und verbunden (Gos `own`)?
    pub fn own(&self, id: &str, conn: u64) -> bool {
        !self.closed
            && self
                .devices
                .get(id)
                .is_some_and(|d| d.connected && d.peer.conn_id() == conn)
    }

    /// Nimmt einen lokalen Spieler dazu. Ein Beobachter bekommt keinen Platz (B-352).
    pub fn add_slot(&mut self, id: &str, conn: u64, slot: i64) -> Result<(), Fehler> {
        if slot >= MAX_SLOTS {
            return Err(Fehler::TooManySlots);
        }
        let observer = self.devices.get(id).is_some_and(|d| d.observer);
        if !self.own(id, conn) || observer || slot < 0 {
            return Err(Fehler::BadRequest);
        }
        if self.devices.get(id).is_some_and(|d| d.slots.contains_key(&slot)) {
            return Err(Fehler::BadRequest);
        }
        if self.count(MonarchState::Free) == 0 && self.monarchs.len() >= MAX_MONARCHS {
            return Err(Fehler::RoomFull);
        }
        let stage = self.device_stage(id);
        let idx = self.assign(id, slot, stage);
        self.take(id, slot, idx);
        self.sync_free();
        self.broadcast_seats();
        Ok(())
    }

    /// Lässt einen lokalen Spieler gehen; sein Monarch ist sofort frei. `true`: Das war der letzte Slot, das Gerät hat
    /// den Raum verlassen.
    pub fn remove_slot(&mut self, env: &mut Env, id: &str, conn: u64, slot: i64) -> Result<bool, Fehler> {
        if !self.own(id, conn) {
            return Err(Fehler::BadRequest);
        }
        let Some(idx) = self.devices.get(id).and_then(|d| d.slots.get(&slot)).copied() else {
            return Err(Fehler::BadRequest);
        };
        self.release(idx);
        let left = self.devices.get_mut(id).is_some_and(|d| {
            d.slots.remove(&slot);
            d.slots.is_empty()
        });
        if left {
            self.remove_device(env, id);
            return Ok(true);
        }
        self.sync_free();
        self.broadcast_seats();
        Ok(false)
    }

    /// Setzt die Eingaben mehrerer Slots, alles oder nichts; der Raum rechnet mit der zuletzt empfangenen. `move_x`
    /// wird auf −1…1 begrenzt, NaN ist ungültig.
    pub fn input(&mut self, id: &str, conn: u64, input: &BTreeMap<i64, PlayerCommand>) -> Result<(), Fehler> {
        if !self.own(id, conn) || input.is_empty() {
            return Err(Fehler::BadRequest);
        }
        let Some(d) = self.devices.get(id) else {
            return Err(Fehler::BadRequest);
        };
        if input
            .iter()
            .any(|(slot, cmd)| !d.slots.contains_key(slot) || cmd.move_x.is_nan())
        {
            return Err(Fehler::BadRequest);
        }
        let targets: Vec<(usize, PlayerCommand)> = input
            .iter()
            .filter_map(|(slot, cmd)| {
                let mut cmd = cmd.clone();
                cmd.move_x = cmd.move_x.clamp(-1.0, 1.0);
                d.slots.get(slot).map(|&idx| (idx, cmd))
            })
            .collect();
        for (idx, cmd) in targets {
            self.monarchs[idx].input = cmd;
        }
        Ok(())
    }

    /// Das Gerät verlässt den Raum bewusst, alle seine Monarchen sind sofort frei.
    pub fn leave(&mut self, env: &mut Env, id: &str, conn: u64) {
        if self.own(id, conn) {
            self.remove_device(env, id);
        }
    }

    /// Gos `leave`: nimmt das Gerät aus dem Raum, gibt seine Monarchen frei.
    pub(super) fn remove_device(&mut self, env: &mut Env, id: &str) {
        let Some(d) = self.devices.get(id) else {
            return;
        };
        if d.observer {
            self.unobserve(env, id);
            return;
        }
        let idxs: Vec<usize> = d.slots.values().copied().collect();
        for idx in idxs {
            self.release(idx);
        }
        self.devices.remove(id);
        let (device, devices) = (short(id), self.connected());
        self.log(
            env,
            Level::Info,
            "👋 Gerät hat den Raum verlassen",
            &[("device", &device), ("geraete", &devices)],
        );
        self.after_disconnect(env);
    }

    /// Die Verbindung `conn` ist abgebrochen. Seine Monarchen warten (stehen still), nach `WAIT_FOR_SECS` sind sie frei.
    /// Eine schon ersetzte Verbindung oder ein geschlossener Raum wird ignoriert (Gos `Drop`).
    pub fn drop_conn(&mut self, env: &mut Env, id: &str, conn: u64) {
        if !self.own(id, conn) {
            return;
        }
        if self.devices.get(id).is_some_and(|d| d.observer) {
            self.unobserve(env, id); // ein Beobachter hat keine wartenden Monarchen
            return;
        }
        let now = env.now;
        let idxs: Vec<usize> = self
            .devices
            .get(id)
            .map(|d| d.slots.values().copied().collect())
            .unwrap_or_default();
        for idx in idxs {
            let mo = &mut self.monarchs[idx];
            mo.state = MonarchState::Waiting;
            mo.since = Some(now);
            mo.input = PlayerCommand::default();
        }
        if let Some(d) = self.devices.get_mut(id) {
            // die Pause endet mit der Verbindung (B-214)
            (d.connected, d.paused, d.protect) = (false, false, 0);
        }
        if let Some(met) = &mut self.met {
            met.disconnect(id);
        }
        let (device, frist, devices) = (short(id), format!("{}m0s", WAIT_FOR_SECS / 60), self.connected());
        self.log(
            env,
            Level::Info,
            "⏳ Gerät getrennt, Monarchen warten",
            &[("device", &device), ("frist", &frist), ("geraete", &devices)],
        );
        self.after_disconnect(env);
    }

    /// Plätze melden und speichern, bei jedem Verlassen und jedem Abbruch eines Geräts (B-147, S2.2); ist kein Gerät
    /// mehr verbunden, ist der Raum zusätzlich pausiert und die Frist `EMPTY_FOR_SECS` läuft. Ein geschlossener Raum
    /// speichert nicht (nach einem Absturz ist sein Zustand nicht sicher).
    fn after_disconnect(&mut self, env: &mut Env) {
        if self.closed {
            return;
        }
        self.sync_free();
        self.broadcast_seats();
        if self.connected() > 0 {
            self.save(env);
            return;
        }
        self.log(env, Level::Info, "💾 Raum leer und pausiert, speichert", &[]);
        if self.timescale > 1 {
            let factor = self.timescale;
            self.log(env, Level::Info, "⏩ Zeitraffer beendet", &[("faktor", &factor)]);
        }
        (self.timescale, self.paused, self.auto_reset) = (1, false, false);
        self.save(env);
        self.empty_since = Some(env.now);
    }

    /// Sind alle Monarchen des Geräts frei?
    pub(super) fn all_free(&self, id: &str) -> bool {
        self.devices.get(id).is_some_and(|d| {
            d.slots
                .values()
                .all(|&idx| self.monarchs[idx].state == MonarchState::Free)
        })
    }

    /// Ist das Gerät (verbunden oder wartend) im Raum?
    pub fn has(&self, id: &str) -> bool {
        self.devices.contains_key(id)
    }
}
