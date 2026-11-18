//! Pause eines Geräts (`docs/rules/bedienung.md` § 1, `docs/protocol.md` › v8, B-214). Port von `pause.go`: Nachrichten
//! `pause` und `resume` setzen `Device::paused`. Ist das pausierte Gerät das einzige verbundene (Couch-Raum), rechnet der
//! Tick keinen Schritt. Sonst läuft der Raum weiter, die Monarchen des Geräts stehen still und sind `PROTECT_TICKS` lang
//! geschützt (`Player::protected`), danach verwundbar. Die Frist zählt Ticks (deterministisch), nicht die Wanduhr.
//!
//! `couch_paused`, `device_paused` und `pause_tick` hat RU21 vorgezogen (`Room::tick` und `steps` brauchen
//! sie); sie stehen seit RU24.1 hier neben der Aktion `pause`.
use k3c_sim::state::PlayerCommand;

use super::env::Env;
use super::log::{Level, short};
use super::types::{MonarchState, Room, TICK_HZ, WAIT_FOR_SECS};
use crate::fehler::Fehler;

/// Höchstdauer des Schutzes online: `WAIT_FOR_SECS` (60 s) in Ticks (Go `protectTicks`).
pub const PROTECT_TICKS: i64 = WAIT_FOR_SECS * TICK_HZ;

impl Room {
    /// Gos `Pause`: hält das Gerät an (`on`) oder setzt es fort. Ein zweites `pause` verlängert den Schutz nicht, ein
    /// `resume` ohne Pause ändert nichts. Beim Anhalten und Fortsetzen verfallen die letzten Eingaben seiner Monarchen
    /// (auch die während der Pause empfangenen), damit niemand mit einer alten Eingabe losläuft. Ein fremder oder
    /// ersetzter Peer und ein Beobachter sind `BadRequest`.
    pub fn pause(&mut self, env: &mut Env, id: &str, conn: u64, on: bool) -> Result<(), Fehler> {
        if !self.own(id, conn) {
            return Err(Fehler::BadRequest);
        }
        let Some(d) = self.devices.get_mut(id) else {
            return Err(Fehler::BadRequest);
        };
        if d.observer {
            return Err(Fehler::BadRequest);
        }
        if d.paused == on {
            return Ok(());
        }
        d.paused = on;
        d.protect = if on { PROTECT_TICKS } else { 0 };
        let idxs: Vec<usize> = d.slots.values().copied().collect();
        for idx in idxs {
            self.monarchs[idx].input = PlayerCommand::default();
        }
        let (device, couch) = (short(id), self.couch_paused());
        self.log(
            env,
            Level::Info,
            "🚦 Gerät pausiert",
            &[("device", &device), ("pausiert", &on), ("couch", &couch)],
        );
        Ok(())
    }
}

impl Room {
    /// Couch-Pause: genau ein Gerät mit Platz ist verbunden und pausiert, der Raum steht (B-214).
    pub fn couch_paused(&self) -> bool {
        let mut seated = self.devices.values().filter(|d| d.connected && !d.observer);
        match (seated.next(), seated.next()) {
            (Some(d), None) => d.paused,
            _ => false,
        }
    }

    /// Das Gerät `id` ist verbunden und pausiert; seine Eingaben zählen nicht.
    pub fn device_paused(&self, id: &str) -> bool {
        self.devices.get(id).is_some_and(|d| d.connected && d.paused)
    }

    /// Setzt vor den Schritten eines Ticks den Schutz je Monarch und zählt die Frist der pausierten Geräte herunter; im
    /// Couch-Raum und in der Dev-Pause steht sie.
    pub fn pause_tick(&mut self, env: &mut Env) {
        let couch = self.couch_paused();
        let protected: Vec<bool> = self
            .monarchs
            .iter()
            .map(|mo| {
                let d = self.devices.get(&mo.device);
                !couch
                    && mo.state == MonarchState::Taken
                    && self.device_paused(&mo.device)
                    && d.is_some_and(|d| d.protect > 0)
            })
            .collect();
        for p in self.isl.stages.iter_mut().flat_map(|w| w.players.iter_mut()) {
            if let Some(&on) = usize::try_from(p.index).ok().and_then(|i| protected.get(i)) {
                p.protected = on;
            }
        }
        if couch || self.paused {
            return;
        }
        let mut ended = Vec::new();
        for (id, d) in self.devices.iter_mut() {
            if !d.connected || !d.paused || d.protect == 0 {
                continue;
            }
            d.protect -= 1;
            if d.protect == 0 {
                ended.push(short(id));
            }
        }
        for device in ended {
            let frist = format!("{}m0s", WAIT_FOR_SECS / 60);
            self.log(
                env,
                Level::Info,
                "⏳ Schutz der Pause endet",
                &[("device", &device), ("frist", &frist)],
            );
        }
    }
}
