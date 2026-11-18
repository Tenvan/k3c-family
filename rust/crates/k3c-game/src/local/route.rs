//! Port von `native/k3c-game/local/route.go`: die Inseln der Welt für die Seekarte (`Frame::route`).

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;

use k3c_room::room::Log as _;
use k3c_sim::island::{STANDARD_WORLD_ID, WorldDef, parse_world, standard_world};
use k3c_sim::world::base::base;

use super::session::Session;
use super::worlds::{game_data, world_file};
use crate::view::frame::RouteIsland;

/// Hält die Inseln der Welt des laufenden Spiels für `Frame::route`; neu gelesen wird nur bei anderer Welt-ID.
#[derive(Debug, Default)]
pub struct RouteCache {
    pub world: String,
    pub list: Vec<RouteIsland>,
    pub set: bool,
}

impl Session {
    /// Liefert die Inseln der Welt des laufenden Spiels mit Etappe (leer ohne Spiel oder bei unlesbarer Welt-Datei, dann
    /// zeigt die Seekarte nur die Ziele). Die Liste gilt, solange das Spiel läuft.
    pub fn route_list(&mut self) -> &[RouteIsland] {
        if self.m.is_none() {
            self.route = RouteCache::default();
            return &[];
        }
        if self.route.set && self.route.world == self.world {
            return &self.route.list;
        }
        self.route = RouteCache {
            world: self.world.clone(),
            list: Vec::new(),
            set: true,
        };
        let def = match read_world_def(self.worlds_dir.as_deref(), &self.world) {
            Ok(def) => def,
            Err(e) => {
                let world = self.world.clone();
                self.log.warn(
                    "💥 Welt für die Seekarte nicht gelesen",
                    &[("world", &world), ("err", &e)],
                );
                return &[];
            }
        };
        for (i, d) in def.islands.iter().enumerate() {
            // wie WorldDef::etappe_of: ohne Angabe gilt der Listenplatz + 1
            let etappe = if d.etappe <= 0 { i as i64 + 1 } else { d.etappe };
            let chunks = d.chunk_counts.values().copied().fold(0, i64::max); // Aussehen auf der Seekarte (B-964)
            self.route.list.push(RouteIsland {
                id: d.id.clone(),
                name: d.name.clone(),
                etappe,
                world: self.world.clone(),
                stages: d.depths.len() as i64,
                chunks,
            });
        }
        &self.route.list
    }
}

/// Liest die Welt `id`: leer oder `standard` aus den Spieldaten, sonst aus dem Welten-Ordner `dir` (nur Dev-Mode).
pub fn load_world_def(dir: Option<&Path>, id: &str) -> Result<WorldDef, String> {
    let data = game_data().map_err(|e| e.to_string())?;
    let b = base()?;
    if id.is_empty() || id == STANDARD_WORLD_ID {
        return standard_world(&data, &b.rules);
    }
    let Some(dir) = dir else {
        return Err("kein Welten-Ordner".to_string());
    };
    let Some(file) = world_file(dir, id) else {
        return Err(format!("Welt-ID {id:?} ungültig (a-z, 0-9, -, höchstens 32 Zeichen)"));
    };
    let raw = std::fs::read_to_string(file).map_err(|e| e.to_string())?;
    parse_world(&raw, id, &data, &b.rules)
}

/// `load_world_def`, das einen Abbruch der Welt-Daten als Fehler meldet: `standard_world` bricht bis RU19
/// (`decode_world_story`) mit `unimplemented!` ab; Seekarte und Geschichte entfallen dann wie bei einer unlesbaren Datei,
/// das Spiel läuft (Go: Fehler statt Abbruch). RU19 macht den Fang überflüssig.
pub fn read_world_def(dir: Option<&Path>, id: &str) -> Result<WorldDef, String> {
    catch_unwind(AssertUnwindSafe(|| load_world_def(dir, id)))
        .unwrap_or_else(|_| Err("Welt-Daten noch nicht portiert (RU19)".to_string()))
}
