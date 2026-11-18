//! Ein Eintrag der Raumliste (`docs/protocol.md` › Nachrichten, rooms). Port von `Info` und `Room.info` aus `actions.go`
//! (vorgezogen aus RU21.4, weil `Manager::rooms` sie braucht).
use serde::Serialize;

use super::types::{MAX_MONARCHS, MonarchState, Room};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Info {
    pub code: String,
    pub name: String,
    pub depth: i64,
    pub grade: String,
    /// Besetzt und wartend.
    pub taken: i64,
    /// 4 minus `taken`.
    pub free: i64,
    pub running: bool,
    /// Welt-ID des Raums (BD9.1, B-604).
    pub world: String,
}

impl Room {
    pub fn info(&self) -> Info {
        let taken = (self.count(MonarchState::Taken) + self.count(MonarchState::Waiting)) as i64;
        Info {
            code: self.code.clone(),
            name: self.name.clone(),
            depth: self.isl.stages.first().map_or(0, |w| w.biome.depth),
            grade: self.isl.options.grade.clone(),
            taken,
            free: MAX_MONARCHS as i64 - taken,
            running: self.connected() > 0,
            world: self.isl.world_id().to_string(),
        }
    }
}
