//! Das Raummodell aus `docs/protocol.md` (Protokoll v2): Räume, Geräte mit lokalen Spielern (Slots), Monarchen (taken,
//! waiting, free), Fristen und Speichern. Port von `engine/room/room.go`. Es kennt kein Netz; das native Spiel setzt
//! `Peer` für das lokale Gerät um.
//!
//! Gos `sync.Mutex` und der Rückverweis `m *Manager` entfallen: `Room` ist ein Struct mit `&mut self`-Methoden, was der
//! Manager besitzt (Uhr, Store, Log, Snapshot), kommt als `&mut Env` herein (`env.rs`).
use std::collections::BTreeMap;
use std::fmt::Display;

use k3c_sim::island::{Island, add_island_player};
use k3c_sim::state::PlayerCommand;

#[cfg(test)]
mod determinism_tests;
mod env;
mod info;
mod log;
mod manager;
#[cfg(test)]
mod manager_tests;
#[cfg(test)]
mod options_tests;
mod pending;
#[cfg(test)]
mod room_tests;
mod run;
#[cfg(test)]
mod run_tests;
#[cfg(test)]
mod safetick_tests;
mod stages;
#[cfg(test)]
mod testutil;
mod types;
#[cfg(test)]
mod victory_tests;

pub use env::{Env, SnapshotFn, Store};
pub use info::Info;
pub use log::{Level, Log, NullLog, RecordingLog, list_text, short};
pub use manager::{CODE_LETTERS, Manager};
pub use pending::Metrics;
pub use run::Failure;
pub use stages::{free_monarchs, start_stage};
pub use types::*;

use crate::fehler::Fehler;

/// Slot 4 oder höher: `too_many_slots`, sonst leer, negativ oder doppelt: `bad_request`.
pub fn valid_slots(slots: &[i64]) -> Result<(), Fehler> {
    if slots.iter().any(|&s| s >= MAX_SLOTS) {
        return Err(Fehler::TooManySlots);
    }
    for (i, &s) in slots.iter().enumerate() {
        if s < 0 || slots[..i].contains(&s) {
            return Err(Fehler::BadRequest);
        }
    }
    if slots.is_empty() {
        return Err(Fehler::BadRequest);
    }
    Ok(())
}

/// Die Plätze des Geräts: Slot, Monarch, Tiefe und Stufe (nach Slot sortiert, `BTreeMap`).
fn seats_of(isl: &Island, d: &Device) -> Vec<Seat> {
    d.slots
        .iter()
        .filter_map(|(&slot, &idx)| {
            let st = isl.stage_of(idx as i64)?;
            Some(Seat {
                slot,
                monarch: idx as i64,
                depth: isl.stages[st].biome.depth,
                stage: st as i64,
            })
        })
        .collect()
}

impl Room {
    pub fn new(code: &str, name: &str, isl: Island, opts: Options, start: usize, monarchs: Vec<Monarch>) -> Room {
        Room {
            code: code.to_string(),
            name: name.to_string(),
            isl,
            won_now: false,
            opts,
            start,
            monarchs,
            devices: BTreeMap::new(),
            tick: 0,
            empty_since: None,
            closed: false,
            timescale: 0,
            auto_reset: false,
            reset_phase: String::new(),
            paused: false,
            durations: Vec::new(),
            before_step: None,
            met: None,
        }
    }

    /// Meldet mit den Feldern des Raums (`ns`, `room`, `save`) vorweg (Go `r.log()`).
    pub fn log(&self, env: &mut Env, level: Level, msg: &str, fields: &[(&str, &dyn Display)]) {
        let mut all: Vec<(&str, &dyn Display)> = vec![("ns", &"room"), ("room", &self.code), ("save", &self.name)];
        all.extend_from_slice(fields);
        env.log.log(level, msg, &all);
    }

    /// Nimmt ein Gerät auf oder verbindet es wieder (`docs/protocol.md` › Beitreten, Wiederverbinden). Alles oder nichts.
    pub fn join(&mut self, env: &mut Env, id: &str, peer: Box<dyn Peer>, slots: &[i64]) -> Result<(), Fehler> {
        valid_slots(slots)?;
        let mut keep: BTreeMap<i64, usize> = BTreeMap::new();
        let old_slots = self.devices.get(id).map(|o| o.slots.clone());
        for (slot, idx) in old_slots.iter().flatten() {
            if slots.contains(slot) {
                keep.insert(*slot, *idx);
            }
        }
        let released = old_slots.as_ref().map_or(0, |o| o.len() - keep.len());
        let spare = (self.count(MonarchState::Free) + released + MAX_MONARCHS) as i64 - self.monarchs.len() as i64;
        if (slots.len() - keep.len()) as i64 > spare {
            return Err(Fehler::RoomFull);
        }
        if let Some(old) = self.devices.get_mut(id)
            && old.connected
            && old.peer.conn_id() != peer.conn_id()
        {
            old.peer.replaced();
        }
        for (slot, idx) in old_slots.iter().flatten() {
            if !keep.contains_key(slot) {
                self.release(*idx);
            }
        }
        self.devices.insert(id.to_string(), Device::new(peer, keep));
        for &slot in slots {
            let kept = self.devices.get(id).and_then(|d| d.slots.get(&slot)).copied();
            let idx = match kept {
                Some(idx) => idx,
                None => {
                    let stage = self.device_stage(id);
                    self.assign(id, slot, stage)
                }
            };
            self.take(id, slot, idx);
        }
        let rejoined = old_slots.is_some();
        self.announce_join(env, id, slots, rejoined);
        Ok(())
    }

    /// Nach dem Beitritt: Frist löschen, Metrik, `Player::free`, Log, `joined`, Zustand und Plätze an alle Geräte.
    fn announce_join(&mut self, env: &mut Env, id: &str, slots: &[i64], rejoined: bool) {
        self.empty_since = None;
        if let Some(met) = &mut self.met {
            met.join(id);
        }
        self.sync_free();
        let (device, slot_text, devices) = (short(id), list_text(slots), self.connected());
        self.log(
            env,
            Level::Info,
            "👑 Gerät im Raum",
            &[
                ("device", &device),
                ("slots", &slot_text),
                ("wiederverbunden", &rejoined),
                ("geraete", &devices),
            ],
        );
        let world = self.isl.world_id().to_string();
        let Room {
            isl,
            devices,
            code,
            name,
            ..
        } = self;
        if let Some(d) = devices.get_mut(id) {
            let you = seats_of(isl, d);
            d.peer.joined(code, name, &world, &you);
        }
        self.push_state(env, id, None);
        self.broadcast_seats();
    }

    /// Wählt einen freien Monarchen: den, den (Gerät, Slot) zuletzt hatte, sonst den kleinsten freien, sonst einen neuen
    /// Insel-Spieler in der Stufe `stage` (die Stufe des Geräts).
    pub fn assign(&mut self, id: &str, slot: i64, stage: usize) -> usize {
        let mut best = None;
        for (i, mo) in self.monarchs.iter().enumerate() {
            if mo.state != MonarchState::Free {
                continue;
            }
            if mo.device == id && mo.slot == slot {
                return i;
            }
            best.get_or_insert(i);
        }
        if let Some(i) = best {
            return i;
        }
        add_island_player(&mut self.isl, stage);
        self.monarchs.push(Monarch::default());
        self.monarchs.len() - 1
    }

    pub fn take(&mut self, id: &str, slot: i64, idx: usize) {
        if let Some(d) = self.devices.get_mut(id) {
            d.slots.insert(slot, idx);
        }
        self.monarchs[idx] = Monarch {
            state: MonarchState::Taken,
            device: id.to_string(),
            slot,
            ..Monarch::default()
        };
    }

    pub fn release(&mut self, idx: usize) {
        let mo = &mut self.monarchs[idx];
        mo.state = MonarchState::Free;
        mo.input = PlayerCommand::default();
    }

    pub fn count(&self, state: MonarchState) -> usize {
        self.monarchs.iter().filter(|mo| mo.state == state).count()
    }

    /// Zählt die verbundenen Geräte mit Platz; Beobachter zählen nicht (`observer`, RU24).
    pub fn connected(&self) -> usize {
        self.devices.values().filter(|d| d.connected && !d.observer).count()
    }

    /// Die Plätze des Geräts `id` (leer, wenn es fehlt).
    pub fn seats(&self, id: &str) -> Vec<Seat> {
        self.devices.get(id).map(|d| seats_of(&self.isl, d)).unwrap_or_default()
    }

    /// Setzt `Player::free` (B-059): Nur besetzte Monarchen entscheiden über den Stufenwechsel.
    pub fn sync_free(&mut self) {
        let Room { isl, monarchs, .. } = self;
        for p in isl.stages.iter_mut().flat_map(|w| w.players.iter_mut()) {
            if let Some(mo) = usize::try_from(p.index).ok().and_then(|i| monarchs.get(i)) {
                p.free = mo.state != MonarchState::Taken;
            }
        }
    }

    pub fn states(&self) -> Vec<MonarchState> {
        self.monarchs.iter().map(|mo| mo.state).collect()
    }

    pub fn broadcast_seats(&mut self) {
        let states = self.states();
        let Room { isl, devices, .. } = self;
        for d in devices.values_mut().filter(|d| d.connected) {
            let you = seats_of(isl, d);
            d.peer.seats(&you, &states);
        }
    }

    /// Der Raum ist ein Testraum (`TEST_PREFIX`).
    pub fn is_test(&self) -> bool {
        self.name.starts_with(TEST_PREFIX)
    }

    /// Löscht den Spielstand eines Testraums (`TEST_PREFIX`), nachdem der leere Raum aufgeräumt wurde.
    pub fn drop_test_save(&self, env: &mut Env) {
        if !self.is_test() {
            return;
        }
        if let Err(e) = env.store.delete(&self.name) {
            env.log.error(
                "💥 Test-Spielstand nicht gelöscht",
                &[("room", &self.code), ("save", &self.name), ("err", &e)],
            );
        }
    }

    /// Die Insel ist komplett verloren: der Raum speichert nicht mehr (B-344).
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

    /// Kodiert den Spielstand der Insel als JSON (Format wie Go, Version 9).
    pub fn encode(&self, env: &Env) -> Result<Vec<u8>, Fehler> {
        let save = self
            .isl
            .to_save(&env.now.format_report())
            .map_err(|e| Fehler::Io(e.to_string()))?;
        serde_json::to_vec(&save).map_err(|e| Fehler::Io(e.to_string()))
    }

    /// Speichert synchron (Aufräumen, Herunterfahren) und meldet das Ergebnis im Log (Go `saveNow`). Ohne Hintergrund-
    /// Schreiber (RU24) gibt es keine ausstehenden Stände; die Dauer der Speicherung misst RU24 mit dem Schreiber.
    pub fn save_sync(&mut self, env: &mut Env) {
        if self.game_over(env) {
            return;
        }
        let data = match self.encode(env) {
            Ok(d) => d,
            Err(e) => return self.log(env, Level::Error, "💥 Spielstand nicht gespeichert", &[("err", &e)]),
        };
        match env.store.store(&self.name, &data) {
            Ok(backup) => self.log(
                env,
                Level::Debug,
                "💾 Spielstand gespeichert",
                &[("tick", &self.tick), ("sicherung", &backup)],
            ),
            Err(e) => self.log(env, Level::Error, "💥 Spielstand nicht gespeichert", &[("err", &e)]),
        }
    }

    /// Sichert den Spielstand sofort (Diagnose, B-088) und meldet den Fehler, statt ihn nur zu loggen. Schreibt synchron;
    /// den Hintergrund-Schreiber (`flush`) bringt RU24.
    pub fn save_now(&mut self, env: &mut Env) -> Result<String, Fehler> {
        if self.closed {
            return Err(Fehler::Closed);
        }
        if self.game_over(env) {
            return Err(Fehler::GameOver);
        }
        let data = self.encode(env)?;
        env.store.store(&self.name, &data)
    }
}
