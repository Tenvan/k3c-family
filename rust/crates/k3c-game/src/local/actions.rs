//! Port von `native/k3c-game/local/actions.go`: Aktionen der Spieler in der Sitzung (Gerät tauschen, Skills, Insel wählen).
//! Aktionen eines Spielers wirken nur auf seinen Slot. Die Regeln prüft die Simulation im Raum (`Room::learn`, `respec`,
//! `choose_island`); eine Ablehnung nennt ihren Grund (`Fehler::BadRequestReason`, B-803).

use k3c_room::fehler::Fehler as RoomFehler;
use k3c_room::room::{Env, Log as _, Room};

use super::error::{UserError, err_no_game, user_err};
use super::session::{CONN, DEVICE_ID, Session};
use crate::view::frame::{Cue, CueKind};
use crate::view::input::DeviceID;

/// Go: `reason`: der Grund einer Ablehnung durch die Simulation als Anhang „: …“, sonst leer.
pub fn reason(err: &RoomFehler) -> String {
    match err {
        RoomFehler::BadRequestReason { reason } => format!(": {reason}"),
        _ => String::new(),
    }
}

impl Session {
    /// Go: `seated`: Es läuft ein Spiel und der Spieler spielt darin.
    pub(super) fn seated(&self, player: i64) -> Result<(), UserError> {
        if self.m.is_none() {
            return Err(err_no_game());
        }
        if !self.devs.contains_key(&player) {
            return Err(user_err(format!("Spieler {} spielt nicht", player + 1), None));
        }
        Ok(())
    }

    /// Go: `Rebind`: Der Spieler übernimmt das Gerät `d` (verlorenes Pad, B-798); Raum und Monarch bleiben. Ein Gerät steuert
    /// höchstens einen Spieler.
    pub(super) fn actions_rebind(&mut self, player: i64, d: DeviceID) -> Result<(), UserError> {
        self.seated(player)?;
        if let Some((p, _)) = self.devs.iter().find(|&(&p, &dv)| dv == d && p != player) {
            return Err(user_err(format!("{d} steuert schon Spieler {}", p + 1), None));
        }
        if self.devs.get(&player) == Some(&d) {
            return Ok(());
        }
        self.devs.insert(player, d);
        self.last = self.devices();
        self.cues.push(Cue {
            kind: CueKind::Rebound,
            player,
            text: String::new(),
            device: d,
        });
        let dev = d.to_string();
        self.log
            .info("🔌 Gerät neu zugeordnet", &[("player", &player), ("device", &dev)]);
        Ok(())
    }

    /// Go: `Learn`: lernt für den Spieler den Skill; die Simulation prüft Punkte und Voraussetzungen.
    pub(super) fn actions_learn(&mut self, player: i64, skill: &str) -> Result<(), UserError> {
        self.seated(player)?;
        if let Err(e) = self.room_call(|r, env| r.learn(env, DEVICE_ID, CONN, player, skill)) {
            let msg = format!("Skill „{skill}“ lässt sich jetzt nicht lernen{}", reason(&e));
            return Err(user_err(msg, Some(Box::new(e))));
        }
        self.log
            .info("✅ Skill gelernt", &[("player", &player), ("skill", &skill)]);
        Ok(())
    }

    /// Go: `Respec`: setzt die Skills des Spielers zurück (nur am Tag an der Burg, prüft die Simulation).
    pub(super) fn actions_respec(&mut self, player: i64) -> Result<(), UserError> {
        self.seated(player)?;
        if let Err(e) = self.room_call(|r, env| r.respec(env, DEVICE_ID, CONN, player)) {
            let msg = format!("Skills lassen sich nur am Tag an der Burg zurücksetzen{}", reason(&e));
            return Err(user_err(msg, Some(Box::new(e))));
        }
        self.log.info("✅ Skills zurückgesetzt", &[("player", &player)]);
        Ok(())
    }

    /// Go: `ChooseIsland`: wählt für den Spieler die Zielinsel am offenen Wechselpunkt; die Simulation prüft Wechselpunkt,
    /// Krone und Ziel.
    pub(super) fn actions_choose_island(&mut self, player: i64, island: &str) -> Result<(), UserError> {
        self.seated(player)?;
        if let Err(e) = self.room_call(|r, env| r.choose_island(env, DEVICE_ID, CONN, player, island)) {
            let msg = format!("Ziel „{island}“ lässt sich jetzt nicht wählen");
            return Err(user_err(msg, Some(Box::new(e))));
        }
        self.log
            .info("🪜 Zielinsel gewählt", &[("player", &player), ("island", &island)]);
        Ok(())
    }

    /// Ruft eine Aktion des Raums auf; ohne Raum `RoomNotFound`.
    pub(super) fn room_call(
        &mut self,
        f: impl FnOnce(&mut Room, &mut Env) -> Result<(), RoomFehler>,
    ) -> Result<(), RoomFehler> {
        self.with_room(f).unwrap_or(Err(RoomFehler::RoomNotFound))
    }
}
