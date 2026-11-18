//! Skills über das Protokoll (`docs/protocol.md` › Skills und Aktionen, B-123): Lernen und Respec für einen Slot des
//! Geräts, dazu die Inselwahl (B-961). Port von `skills.go`. Die Regeln prüft die Sim (`learn_skill`, `respec`,
//! `choose_island`); jeder Fehler ist `bad_request`, eine Ablehnung der Sim trägt ihren Grund (`BadRequestReason`, B-803).
use k3c_sim::island::{Island, choose_island};
use k3c_sim::monarch::{learn_skill, respec};
use k3c_sim::world::vals;

use super::env::Env;
use super::log::Level;
use super::types::Room;
use crate::fehler::Fehler;

impl Room {
    /// Lernt für den Monarchen im Slot `slot` des Geräts (Verbindung `conn`) den Skill `skill` (ID aus monarch.json › skills).
    pub fn learn(&mut self, env: &mut Env, id: &str, conn: u64, slot: i64, skill: &str) -> Result<(), Fehler> {
        self.with_player(env, id, conn, slot, |isl, stage, pi| {
            let w = &mut isl.stages[stage];
            let v = vals(w);
            learn_skill(w, &v, pi, skill)
        })
    }

    /// Setzt die Skills des Monarchen im Slot `slot` des Geräts zurück (nur am Tag an der Burg, prüft die Sim).
    pub fn respec(&mut self, env: &mut Env, id: &str, conn: u64, slot: i64) -> Result<(), Fehler> {
        self.with_player(env, id, conn, slot, |isl, stage, pi| {
            let w = &mut isl.stages[stage];
            let v = vals(w);
            respec(w, &v, pi)
        })
    }

    /// Wählt für den Monarchen im Slot `slot` des Geräts das Ziel des Inselwechsels (Insel-ID, B-961): nur aus den Zielen
    /// am offenen Wechselpunkt, sonst `bad_request` mit dem Grund der Sim. Mehrere Wahlen im selben Takt: höchster
    /// Spieler-Index.
    pub fn choose_island(
        &mut self,
        env: &mut Env,
        id: &str,
        conn: u64,
        slot: i64,
        island_id: &str,
    ) -> Result<(), Fehler> {
        self.with_player(env, id, conn, slot, |isl, stage, pi| {
            choose_island(isl, stage, pi, island_id)
        })
    }

    /// Ruft `f` mit der Insel, der Stufe und dem Platz des Spielers im Slot auf. Abweichung von Go (`f(*World, *Player)`):
    /// die Inselwahl braucht die Insel, darum `(Insel, Stufe, Platz in `players`)`. Fremder Slot oder fremdes Gerät:
    /// `BadRequest`; Fehler von `f` (Text der Sim): `BadRequestReason`.
    fn with_player(
        &mut self,
        env: &mut Env,
        id: &str,
        conn: u64,
        slot: i64,
        f: impl FnOnce(&mut Island, usize, usize) -> Result<(), String>,
    ) -> Result<(), Fehler> {
        if !self.own(id, conn) {
            return Err(Fehler::BadRequest);
        }
        let idx = self
            .devices
            .get(id)
            .and_then(|d| d.slots.get(&slot))
            .copied()
            .ok_or(Fehler::BadRequest)?;
        let stage = self.isl.stage_of(idx as i64).ok_or(Fehler::BadRequest)?;
        let pi = self.isl.stages[stage]
            .players
            .iter()
            .position(|p| p.index == idx as i64)
            .ok_or(Fehler::BadRequest)?;
        match f(&mut self.isl, stage, pi) {
            Ok(()) => Ok(()),
            Err(reason) => {
                self.log(
                    env,
                    Level::Debug,
                    "🚫 Skill-Aktion abgelehnt",
                    &[("slot", &slot), ("err", &reason)],
                );
                Err(Fehler::BadRequestReason { reason })
            }
        }
    }
}
