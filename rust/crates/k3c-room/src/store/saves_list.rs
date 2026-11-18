//! Die Liste der Spielstände (B-147, S2.2, `docs/protocol.md` › HTTP: Spielstände). Port von `engine/store/saves_list.go`.
use serde::{Deserialize, Serialize};

use super::saves::{Saves, is_slot, parse_save};

fn is_zero(n: &i64) -> bool {
    *n == 0
}

/// Ein Eintrag der Spielstand-Liste. Fehlende Felder fehlen auch in der Antwort; eine unlesbare Datei hat nur Name und
/// `error`.
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct SaveInfo {
    pub name: String,
    #[serde(rename = "savedAt", skip_serializing_if = "String::is_empty")]
    pub saved_at: String,
    #[serde(skip_serializing_if = "is_zero")]
    pub version: i64,
    #[serde(skip_serializing_if = "is_zero")]
    pub day: i64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub phase: String,
    /// Welt-ID (Stand ab Version 8); fehlt = standard (B-506).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub world: String,
    /// Tiefen der Spieler, sortiert, jede einmal.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depths: Vec<i64>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
}

/// Die Felder, die die Liste zusätzlich liest (generisch, ohne `k3c-sim`); `null` zählt wie ein fehlendes Feld.
#[derive(Deserialize, Default)]
struct Extra {
    day: Option<i64>,
    phase: Option<String>,
    world: Option<String>,
    players: Option<Vec<Option<PlayerDepth>>>,
}

#[derive(Deserialize, Default)]
struct PlayerDepth {
    depth: Option<i64>,
}

impl Saves {
    /// Liefert alle Spielstände (ohne Sicherungen) nach Name sortiert.
    pub fn list(&self) -> Vec<SaveInfo> {
        let Ok(entries) = std::fs::read_dir(&self.dir) else {
            return Vec::new();
        };
        let mut list: Vec<SaveInfo> = entries
            .flatten()
            .filter_map(|e| {
                let file = e.file_name().to_string_lossy().into_owned();
                let name = file.strip_suffix(".json")?;
                is_slot(name).then(|| save_info(name, &e.path()))
            })
            .collect();
        list.sort_by(|a, b| a.name.cmp(&b.name)); // Namen sind eindeutig (je Datei einer)
        list
    }
}

fn save_info(name: &str, path: &std::path::Path) -> SaveInfo {
    let bad = || SaveInfo {
        name: name.to_string(),
        error: "ungültig".to_string(),
        ..SaveInfo::default()
    };
    let Ok(data) = std::fs::read(path) else {
        return bad();
    };
    let Ok(h) = parse_save(&data) else {
        return bad();
    };
    let Ok(extra) = serde_json::from_slice::<Extra>(&data) else {
        return bad();
    };
    let mut depths: Vec<i64> = extra
        .players
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .filter_map(|p| p.depth)
        .collect();
    depths.sort_unstable();
    depths.dedup();
    SaveInfo {
        name: name.to_string(),
        saved_at: h.saved_at,
        version: h.version as i64,
        day: extra.day.unwrap_or_default(),
        phase: extra.phase.unwrap_or_default(),
        world: extra.world.unwrap_or_default(),
        depths,
        error: String::new(),
    }
}
