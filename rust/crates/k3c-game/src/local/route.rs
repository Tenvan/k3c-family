//! Port von `native/k3c-game/local/route.go`: die Inseln der Welt für die Seekarte (`Frame::route`).

use std::path::Path;

use k3c_room::room::Log as _;
use k3c_sim::island::chronicle::ChronicleEntry;
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
        let def = match load_world_def(self.worlds_dir.as_deref(), &self.world) {
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
        self.route.list = route_of_def(&def, &self.world);
        &self.route.list
    }

    /// Liefert die Chronik der abgeschlossenen Inseln des laufenden Spiels aus dem Raum (GS3.8; leer ohne Spiel oder
    /// Abschluss, ohne Allokation). Sie steht nicht im Cache der Route, weil sie sich beim Inselwechsel ändert.
    pub fn chronicle_list(&mut self) -> Vec<ChronicleEntry> {
        let Some(m) = self.m.as_mut() else {
            return Vec::new();
        };
        m.with_room(&self.code, |r, _| r.isl.chronicle.clone())
            .unwrap_or_default()
    }
}

/// Die Inseln der Welt `def` (Welt-ID `world`) in Listenreihenfolge für die Seekarte und die Spielstand-Vorschau.
pub fn route_of_def(def: &WorldDef, world: &str) -> Vec<RouteIsland> {
    def.islands
        .iter()
        .enumerate()
        .map(|(i, d)| RouteIsland {
            id: d.id.clone(),
            name: d.name.clone(),
            // wie WorldDef::etappe_of: ohne Angabe gilt der Listenplatz + 1
            etappe: if d.etappe <= 0 { i as i64 + 1 } else { d.etappe },
            world: world.to_string(),
            stages: d.depths.len() as i64,
            chunks: d.chunk_counts.values().copied().fold(0, i64::max), // Aussehen auf der Seekarte (B-964)
            done: first_line(d.story.done.as_deref()),
        })
        .collect()
}

/// Die erste nicht leere Zeile einer Geschichte; leer ohne Zeile.
fn first_line(lines: Option<&[String]>) -> String {
    lines
        .and_then(|l| l.iter().find(|s| !s.trim().is_empty()))
        .cloned()
        .unwrap_or_default()
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
