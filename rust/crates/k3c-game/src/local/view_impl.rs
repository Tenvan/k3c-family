//! `view::session::Session` für die lokale Sitzung (Go: die exportierten Methoden von `session.go`); die Arbeit steht in
//! `session.rs`, `settings.rs` und, bis RU27.2/RU27.3 sie liefern, in `pending.rs`.

use k3c_room::room::Options;

use super::error::user_err;
use super::session::Session;
use crate::view::frame::Seat;
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::session::{Fehler, NewGame, SaveInfo, Session as ViewSession, WorldInfo};
use crate::view::settings::Settings;

impl ViewSession for Session {
    /// Legt ein neues Spiel an und startet es; ein laufendes wird vorher gespeichert und beendet.
    fn new_game(&mut self, g: NewGame) -> Result<(), Fehler> {
        let name = if g.name.is_empty() { self.free_name() } else { g.name };
        let opts = Options {
            grade: g.grade,
            goal: g.goal,
            defeat: g.defeat,
            seed: g.seed,
            world: g.world,
        };
        self.start(&name, true, g.depth, opts, &g.devices).map_err(Fehler::from)
    }

    /// Lädt den zuletzt gespeicherten ladbaren Spielstand mit den Geräten `devices` (leer wie bei `load`).
    fn continue_game(&mut self, devices: &[DeviceID]) -> Result<(), Fehler> {
        let list = self.list()?;
        match list.into_iter().find(|e| e.loadable) {
            Some(e) => self.load(&e.name, devices),
            None => Err(user_err("Kein ladbarer Spielstand vorhanden", None).into()),
        }
    }

    /// Lädt den Spielstand `name` mit den Geräten `devices` (leer: die des letzten Spiels, ohne: Tastatur links) und
    /// startet ihn.
    fn load(&mut self, name: &str, devices: &[DeviceID]) -> Result<(), Fehler> {
        let devs = if !devices.is_empty() {
            devices.to_vec()
        } else if !self.last.is_empty() {
            self.last.clone()
        } else {
            vec![DeviceID::new(DeviceKind::Keyboard, 0)]
        };
        self.start(name, false, 0, Options::default(), &devs)
            .map_err(Fehler::from)
    }

    fn list(&mut self) -> Result<Vec<SaveInfo>, Fehler> {
        self.saves_list().map_err(Fehler::from)
    }

    fn delete(&mut self, name: &str) -> Result<(), Fehler> {
        self.saves_delete(name).map_err(Fehler::from)
    }

    fn save(&mut self) -> Result<(), Fehler> {
        self.save_game().map_err(Fehler::from)
    }

    fn close(&mut self) -> Result<(), Fehler> {
        self.close_game().map_err(Fehler::from)
    }

    fn active(&self) -> bool {
        self.m.is_some()
    }

    fn pause(&mut self, on: bool) {
        self.pause_game(on);
    }

    fn join(&mut self, d: DeviceID) -> Result<i64, Fehler> {
        self.join_device(d).map_err(Fehler::from)
    }

    fn leave(&mut self, player: i64) -> Result<(), Fehler> {
        self.leave_player(player).map_err(Fehler::from)
    }

    fn rebind(&mut self, player: i64, d: DeviceID) -> Result<(), Fehler> {
        self.actions_rebind(player, d).map_err(Fehler::from)
    }

    fn learn(&mut self, player: i64, skill: &str) -> Result<(), Fehler> {
        self.actions_learn(player, skill).map_err(Fehler::from)
    }

    fn respec(&mut self, player: i64) -> Result<(), Fehler> {
        self.actions_respec(player).map_err(Fehler::from)
    }

    fn choose_island(&mut self, player: i64, island: &str) -> Result<(), Fehler> {
        self.actions_choose_island(player, island).map_err(Fehler::from)
    }

    fn set_settings(&mut self, s: Settings) -> Result<(), Fehler> {
        self.apply_settings(s).map_err(Fehler::from)
    }

    fn settings(&self) -> Settings {
        self.settings.clone()
    }

    fn seats(&self) -> Vec<Seat> {
        self.step_seats()
    }

    fn world_list(&self) -> Vec<WorldInfo> {
        self.saves_world_list()
    }

    fn world(&self) -> String {
        self.saves_world()
    }
}
