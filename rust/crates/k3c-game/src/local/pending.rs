//! Platzhalter für Teile späterer Sessions (Leitfaden § 7). Jede Session löscht ihre Platzhalter, RU22 prüft, dass keiner
//! übrig ist. Sie stehen hier, weil `impl view::session::Session for Session` in einem Stück in `view_impl.rs` liegt und die
//! Methoden der späteren Sessions an Methoden mit Go-Namen weiterreicht.
//!
//! - RU27.2: `step_seats` (`Seats`, step.go), `begin_campaign` (campaign.go), die Trait-Methoden `command`, `step`
//! - RU27.3: `saves_list`, `saves_delete`, `saves_world_list`, `saves_world` (saves.go), `actions_rebind`, `actions_learn`,
//!   `actions_respec`, `actions_choose_island` (actions.go), `free_name` (saves.go)
#![allow(clippy::unimplemented)] // Platzhalter nach Leitfaden § 7: wartet auf RU27.2 und RU27.3

use super::error::UserError;
use super::session::Session;
use crate::view::frame::Seat;
use crate::view::input::DeviceID;
use crate::view::session::{SaveInfo, WorldInfo};

impl Session {
    /// Go: `List` (saves.go).
    pub(super) fn saves_list(&mut self) -> Result<Vec<SaveInfo>, UserError> {
        unimplemented!("RU27: saves_list")
    }

    /// Go: `Delete` (saves.go).
    pub(super) fn saves_delete(&mut self, _name: &str) -> Result<(), UserError> {
        unimplemented!("RU27: saves_delete")
    }

    /// Go: `WorldList` (saves.go).
    pub(super) fn saves_world_list(&self) -> Vec<WorldInfo> {
        unimplemented!("RU27: saves_world_list")
    }

    /// Go: `World` (saves.go).
    pub(super) fn saves_world(&self) -> String {
        unimplemented!("RU27: saves_world")
    }

    /// Go: `Rebind` (actions.go).
    pub(super) fn actions_rebind(&mut self, _player: i64, _d: DeviceID) -> Result<(), UserError> {
        unimplemented!("RU27: actions_rebind")
    }

    /// Go: `Learn` (actions.go).
    pub(super) fn actions_learn(&mut self, _player: i64, _skill: &str) -> Result<(), UserError> {
        unimplemented!("RU27: actions_learn")
    }

    /// Go: `Respec` (actions.go).
    pub(super) fn actions_respec(&mut self, _player: i64) -> Result<(), UserError> {
        unimplemented!("RU27: actions_respec")
    }

    /// Go: `ChooseIsland` (actions.go).
    pub(super) fn actions_choose_island(&mut self, _player: i64, _island: &str) -> Result<(), UserError> {
        unimplemented!("RU27: actions_choose_island")
    }

    /// Go: `Seats` (step.go).
    pub(super) fn step_seats(&self) -> Vec<Seat> {
        unimplemented!("RU27: step_seats")
    }

    /// Go: `beginCampaign` (campaign.go). Ein Platzhalter ohne Wirkung statt `unimplemented!`, denn `start` ruft es in jedem
    /// Spielstart auf; RU27.2 legt `campaign` an und ruft sie hier.
    pub(super) fn begin_campaign(&mut self, _name: &str, _fresh: bool) {}

    /// Go: `freeName` (saves.go): der erste freie Name spiel-1, spiel-2 … im Ordner. Läuft schon, weil `new_game` ohne Namen
    /// es braucht; RU27.3 verschiebt es nach `saves.rs`.
    pub(super) fn free_name(&self) -> String {
        let used: Vec<String> = self.saves.list().into_iter().map(|e| e.name).collect();
        (1..)
            .map(|i| format!("spiel-{i}"))
            .find(|n| !used.contains(n))
            .unwrap_or_default()
    }
}
