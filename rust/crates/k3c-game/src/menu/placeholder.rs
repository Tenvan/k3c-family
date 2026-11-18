//! Platzhalter-Daten für die Menüs (kein Go-Gegenstück): `PlaceholderSession` füllt den Vertrag `view::Session` mit festen
//! Einträgen, solange das lokale Spiel fehlt (RU27): zwei Spielstände, ein kaputter, drei Welten. Dient dem Bildvergleich
//! und dem Anspiel-Bild; Spielstände lassen sich laden und löschen, ein Spiel läuft nicht.

use std::time::{Duration, UNIX_EPOCH};

use crate::view::frame::Seat;
use crate::view::input::DeviceID;
use crate::view::session::{Fehler, NewGame, SaveInfo, Session, WorldInfo};
use crate::view::settings::{Settings, default_settings};

/// PlaceholderSession ist eine `Session` mit festen Daten.
pub struct PlaceholderSession {
    saves: Vec<SaveInfo>,
    settings: Settings,
    active: bool,
    world: String,
}

impl Default for PlaceholderSession {
    fn default() -> Self {
        Self::new()
    }
}

impl PlaceholderSession {
    /// Die festen Einträge: „burg“ (Tag 12, Stufen 0 und 2) und „insel“ (Tag 3, Welt sengoku), dazu der kaputte Stand.
    pub fn new() -> PlaceholderSession {
        let at = |secs| Some(UNIX_EPOCH + Duration::from_secs(secs));
        PlaceholderSession {
            saves: vec![
                SaveInfo {
                    name: "burg".to_string(),
                    saved_at: at(1_735_787_040), // 02.01.2025 03:04 UTC
                    day: 12,
                    depths: vec![0, 2],
                    loadable: true,
                    ..SaveInfo::default()
                },
                SaveInfo {
                    name: "insel".to_string(),
                    saved_at: at(1_735_000_000),
                    day: 3,
                    depths: vec![0],
                    world: "sengoku".to_string(),
                    loadable: true,
                    ..SaveInfo::default()
                },
                SaveInfo {
                    name: "kaputt".to_string(),
                    problem: "Spielstand nicht ladbar: Version 3".to_string(),
                    ..SaveInfo::default()
                },
            ],
            settings: default_settings(),
            active: false,
            world: String::new(),
        }
    }
}

impl Session for PlaceholderSession {
    fn new_game(&mut self, g: NewGame) -> Result<(), Fehler> {
        self.world = g.world;
        self.active = true;
        Ok(())
    }

    fn continue_game(&mut self, devices: &[DeviceID]) -> Result<(), Fehler> {
        match self.saves.iter().find(|e| e.loadable).map(|e| e.name.clone()) {
            Some(n) => self.load(&n, devices),
            None => Err(Fehler::Session("Kein ladbarer Spielstand vorhanden".to_string())),
        }
    }

    fn load(&mut self, name: &str, _devices: &[DeviceID]) -> Result<(), Fehler> {
        let e = self
            .saves
            .iter()
            .find(|e| e.name == name)
            .ok_or_else(|| Fehler::Session(format!("Spielstand {name:?} nicht gefunden")))?;
        if !e.loadable {
            return Err(Fehler::Session(e.problem.clone()));
        }
        self.world = e.world.clone();
        self.active = true;
        Ok(())
    }

    fn list(&mut self) -> Result<Vec<SaveInfo>, Fehler> {
        Ok(self.saves.clone())
    }

    fn delete(&mut self, name: &str) -> Result<(), Fehler> {
        self.saves.retain(|e| e.name != name);
        Ok(())
    }

    fn save(&mut self) -> Result<(), Fehler> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), Fehler> {
        self.active = false;
        self.world.clear();
        Ok(())
    }

    fn active(&self) -> bool {
        self.active
    }

    fn pause(&mut self, _on: bool) {}

    fn join(&mut self, _d: DeviceID) -> Result<i64, Fehler> {
        Err(Fehler::Session("Kein Spiel (Platzhalter)".to_string()))
    }

    fn leave(&mut self, _player: i64) -> Result<(), Fehler> {
        Ok(())
    }

    fn rebind(&mut self, _player: i64, _d: DeviceID) -> Result<(), Fehler> {
        Ok(())
    }

    fn learn(&mut self, _player: i64, _skill: &str) -> Result<(), Fehler> {
        Ok(())
    }

    fn respec(&mut self, _player: i64) -> Result<(), Fehler> {
        Ok(())
    }

    fn choose_island(&mut self, _player: i64, _island: &str) -> Result<(), Fehler> {
        Ok(())
    }

    fn set_settings(&mut self, s: Settings) -> Result<(), Fehler> {
        self.settings = s;
        Ok(())
    }

    fn settings(&self) -> Settings {
        self.settings.clone()
    }

    fn seats(&self) -> Vec<Seat> {
        Vec::new()
    }

    fn world_list(&self) -> Vec<WorldInfo> {
        [("standard", "Standard"), ("sengoku", "Sengoku"), ("wald", "Wald")]
            .into_iter()
            .map(|(id, name)| WorldInfo {
                id: id.to_string(),
                name: name.to_string(),
            })
            .collect()
    }

    fn world(&self) -> String {
        self.world.clone()
    }
}
