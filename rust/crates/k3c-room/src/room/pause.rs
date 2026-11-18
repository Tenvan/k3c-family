//! Pause eines Geräts (`docs/rules/bedienung.md` § 1, `docs/protocol.md` › v8, B-214). Port von `pause.go`: Nachrichten
//! `pause` und `resume` setzen `Device::paused`. Ist das pausierte Gerät das einzige verbundene (Couch-Raum), rechnet der
//! Tick keinen Schritt. Sonst läuft der Raum weiter, die Monarchen des Geräts stehen still und sind `PROTECT_TICKS` lang
//! geschützt (`Player::protected`), danach verwundbar. Die Frist zählt Ticks (deterministisch), nicht die Wanduhr.
//!
//! `couch_paused`, `device_paused` und `pause_tick` hat RU21 mit `early.rs` vorgezogen (`Room::tick` und `steps` brauchen
//! sie); sie rechnen wie Go. Hier steht die Aktion `pause`.
use k3c_sim::state::PlayerCommand;

use super::env::Env;
use super::log::{Level, short};
use super::types::{Room, TICK_HZ, WAIT_FOR_SECS};
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
