//! Platzhalter für Teile späterer Sessions (Leitfaden § 7). Jede Session löscht ihre Platzhalter, RU22 prüft, dass keiner
//! übrig ist. Sie stehen hier, weil `impl view::session::Session for Session` in einem Stück in `view_impl.rs` liegt und die
//! Methoden der späteren Sessions an Methoden mit Go-Namen weiterreichen.
//!
//! - RU50.1 liefert `saves.go` und `actions.go` vollständig. RU48.3 (Smoke-Version) hat die Methoden bis dahin so weit
//!   ersetzt, dass das Hauptmenü und Neues Spiel nicht mehr abstürzen: `saves_list` (ohne vollständige Prüfung des
//!   Spielstands, ohne neueste-zuerst-Sortierung nach Zeit), `saves_delete`, `saves_world_list` (nur die Standard-Welt, keine
//!   Dev-Welten), `saves_world`, `actions_rebind`. `actions_learn`, `actions_respec` und `actions_choose_island` melden
//!   „noch nicht verfügbar“, weil `k3c-room` die Aktionen des Raums (`Learn`, `Respec`, `ChooseIsland`) noch nicht kennt.

use std::time::{Duration, UNIX_EPOCH};

use super::error::{UserError, describe, err_no_game, user_err};
use super::session::Session;
use super::worlds::game_data;
use crate::view::frame::{Cue, CueKind};
use crate::view::input::DeviceID;
use crate::view::session::{SaveInfo, WorldInfo};
use k3c_room::store::Stamp;
use k3c_sim::island::STANDARD_WORLD_ID;

/// Die Meldung der Aktionen, die der Raum in Rust noch nicht kennt.
fn not_yet(what: &str) -> UserError {
    user_err(format!("{what} ist im Rust-Spiel noch nicht verfügbar (RU50.1)"), None)
}

/// Liest `YYYY-MM-DDTHH:MM:SS` (RFC 3339, UTC) als Zeitpunkt; alles andere ist `None`.
fn parse_time(s: &str) -> Option<std::time::SystemTime> {
    let n = |r: std::ops::Range<usize>| s.get(r)?.parse::<i64>().ok();
    let st = Stamp::from_civil(n(0..4)?, n(5..7)?, n(8..10)?, n(11..13)?, n(14..16)?, n(17..19)?, 0);
    Some(UNIX_EPOCH + Duration::from_secs(u64::try_from(st.secs).ok()?))
}

impl Session {
    /// Go: `List` (saves.go): neueste zuerst (gleiche Zeit: nach Name); kaputte Einträge mit Grund.
    pub(super) fn saves_list(&mut self) -> Result<Vec<SaveInfo>, UserError> {
        let mut out: Vec<SaveInfo> = self
            .saves
            .list()
            .into_iter()
            .map(|e| {
                let problem = if e.error.is_empty() {
                    String::new()
                } else {
                    "Spielstand unlesbar".to_string()
                };
                SaveInfo {
                    saved_at: parse_time(&e.saved_at),
                    day: e.day,
                    depths: e.depths,
                    world: e.world,
                    loadable: e.error.is_empty(),
                    problem,
                    name: e.name,
                }
            })
            .collect();
        out.sort_by_key(|a| std::cmp::Reverse(a.saved_at));
        Ok(out)
    }

    /// Go: `Delete` (saves.go): nimmt den Spielstand in die Sicherungen; nicht den des laufenden Spiels.
    pub(super) fn saves_delete(&mut self, name: &str) -> Result<(), UserError> {
        if self.m.is_some() && name == self.name {
            return Err(user_err(
                "Der Spielstand des laufenden Spiels lässt sich nicht löschen",
                None,
            ));
        }
        self.saves.discard(name).map_err(|e| describe(&e, name))?;
        Ok(())
    }

    /// Go: `WorldList` (saves.go): bis RU24.4 nur die Standard-Welt.
    pub(super) fn saves_world_list(&self) -> Vec<WorldInfo> {
        let name = game_data()
            .ok()
            .and_then(|d| d.standard_world.name.clone())
            .unwrap_or_else(|| STANDARD_WORLD_ID.to_string());
        vec![WorldInfo {
            id: STANDARD_WORLD_ID.to_string(),
            name,
        }]
    }

    /// Go: `World` (saves.go): die Welt-ID des laufenden Spiels (leer ohne Spiel).
    pub(super) fn saves_world(&self) -> String {
        if self.m.is_none() {
            return String::new();
        }
        self.world.clone()
    }

    /// Go: `seated` (actions.go): Es läuft ein Spiel und der Spieler spielt darin.
    fn seated(&self, player: i64) -> Result<(), UserError> {
        if self.m.is_none() {
            return Err(err_no_game());
        }
        if !self.devs.contains_key(&player) {
            return Err(user_err(format!("Spieler {} spielt nicht", player + 1), None));
        }
        Ok(())
    }

    /// Go: `Rebind` (actions.go): Der Spieler übernimmt das Gerät `d`; ein Gerät steuert höchstens einen Spieler.
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
        self.log_info("🔌 Gerät neu zugeordnet", &[("player", &player), ("device", &dev)]);
        Ok(())
    }

    /// Go: `Learn` (actions.go): bis RU50.1 nicht verfügbar.
    pub(super) fn actions_learn(&mut self, player: i64, skill: &str) -> Result<(), UserError> {
        self.seated(player)?;
        Err(not_yet(&format!("Skill „{skill}“ lernen")))
    }

    /// Go: `Respec` (actions.go): bis RU50.1 nicht verfügbar.
    pub(super) fn actions_respec(&mut self, player: i64) -> Result<(), UserError> {
        self.seated(player)?;
        Err(not_yet("Skills zurücksetzen"))
    }

    /// Go: `ChooseIsland` (actions.go): bis RU50.1 nicht verfügbar.
    pub(super) fn actions_choose_island(&mut self, player: i64, island: &str) -> Result<(), UserError> {
        self.seated(player)?;
        Err(not_yet(&format!("Zielinsel „{island}“ wählen")))
    }

    /// Go: `freeName` (saves.go): der erste freie Name spiel-1, spiel-2 … im Ordner. Läuft schon, weil `new_game` ohne Namen
    /// es braucht; RU50.1 verschiebt es nach `saves.rs`.
    pub(super) fn free_name(&self) -> String {
        let used: Vec<String> = self.saves.list().into_iter().map(|e| e.name).collect();
        (1..)
            .map(|i| format!("spiel-{i}"))
            .find(|n| !used.contains(n))
            .unwrap_or_default()
    }

    /// Meldung ins Log der Session.
    fn log_info(&mut self, msg: &str, fields: &[(&str, &dyn std::fmt::Display)]) {
        use k3c_room::room::Log as _;
        self.log.info(msg, fields);
    }
}
