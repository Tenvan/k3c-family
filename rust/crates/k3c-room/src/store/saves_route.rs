//! Route der Welt für die Spielstand-Liste (GS3.7, B-978/R1): Die Liste nennt je ladbarem Stand zusätzlich die Inseln der
//! Welt (ID, Name, Etappe); aktuelle Insel, abgeschlossene Inseln und Chronik stehen schon im Stand (`SaveInfo`). Es wird
//! nur der Stand und die Welt-Datei gelesen: keine Insel gebaut, keine Simulation gerechnet.

use k3c_sim::island::WorldDef;
use serde::Serialize;

use super::saves::Saves;
use super::saves_list::SaveInfo;

/// Eintrag der Chronik in der Liste: derselbe wie im Spielstand (`islandId`, `days`, `players`).
pub type ChronicleInfo = k3c_sim::island::chronicle::ChronicleEntry;

/// Eine Insel der Route: ID und Name aus der Welt-Datei, Etappe (fehlt sie dort, gilt Listenplatz + 1, `welten.md` § 6.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct RouteIsland {
    pub id: String,
    pub name: String,
    pub etappe: i64,
}

/// Die Route einer Welt: ihre Inseln in der Reihenfolge der Welt-Datei.
pub fn route_of(world: &WorldDef) -> Vec<RouteIsland> {
    world
        .islands
        .iter()
        .zip(1_i64..)
        .map(|(d, place)| RouteIsland {
            id: d.id.clone(),
            name: d.name.clone(),
            etappe: if d.etappe > 0 { d.etappe } else { place },
        })
        .collect()
}

impl Saves {
    /// Wie `list`, mit der Route der Welt je ladbarem Stand. `world` liefert die Welt zur ID (Welt-Datei lesen, kein
    /// Insel-Bau); ein Stand ohne Welt-Angabe gilt als `standard` (B-506). Ein nicht ladbarer Stand (kaputt) und ein Stand
    /// mit unbekannter Welt behalten ihre Route leer (keine Vorschau, B-978/R4).
    pub fn list_with_routes(&self, world: &dyn Fn(&str) -> Option<WorldDef>) -> Vec<SaveInfo> {
        let mut list = self.list();
        for e in list.iter_mut().filter(|e| e.error.is_empty()) {
            let id = if e.world.is_empty() {
                "standard"
            } else {
                e.world.as_str()
            };
            if let Some(w) = world(id) {
                e.route = route_of(&w);
            }
        }
        list
    }
}
