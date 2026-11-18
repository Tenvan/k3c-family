//! Welten als Dateien (Port von `engine/sim/island_worlds.go`; docs/rules/welten.md, B-488): Eine Welt ist eine Datei
//! data/worlds/<id>.json mit ihren Inseln in Spielreihenfolge. Gelesen wird streng (unbekannte Felder und fehlende
//! Pflichtfelder sind Fehler), geprüft wird jede Insel (`check_island`); bei einem Fehler wird die Welt nicht geladen.
//! Eingebettet ist nur die Standard-Welt. In Rust heißt die Welt-Datei `WorldDef`, `World` ist eine Stufe (wie in Go).
//!
//! Der Kopf der Datei (ID, Pflichtfelder, `extends`, `overrides`, `combat`) liest `k3c_data::world::parse`; die Inseln
//! `worlds_decode.rs`. Gos Paket-Variable `standardWorld` wird `standard_world(&Data, &Rules)`; `fs.FS` wird `&Source`.
use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_data::world::{self, WorldFile};
use k3c_data::{Data, Source};
use k3c_level::rules::Rules;
use serde::Deserialize;
use serde_json::Value;

use super::keys::Keys;
use super::worlds_decode::decode_islands;
use crate::island::IslandDef;
use crate::state::pending::{WorldStory, check_story, decode_world_skins, decode_world_story};
use crate::state::shared::{CombatSettings, WorldValues};

/// Go: `StandardWorldID`, die ID der Standard-Welt (data/worlds/standard.json).
pub const STANDARD_WORLD_ID: &str = "standard";

/// Go: `WorldDef`, eine Welt aus einer Welt-Datei.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WorldDef {
    pub id: String,
    pub name: String,
    pub notes: String,
    pub islands: Vec<IslandDef>,
    /// Kampfvariante der Welt (combat.mode, combat.groupSize; kampf.md § 3); leer = groups, 5.
    pub combat: CombatSettings,
    /// Die Eltern-Welt (leer = Basis data/).
    pub extends: String,
    /// Übersteuerte Basis-Werte je Datei (Schlüssel = Dateiname ohne .json, welten.md § 1); Go: `json.RawMessage`.
    pub overrides: BTreeMap<String, serde_json::Value>,
    /// Die Geschichte der Welt (story.go, B-965); `extends` vererbt sie nicht.
    pub story: WorldStory,
    /// Basis-Asset → Ersatz-Asset (world_skins.go, B-984).
    pub skins: BTreeMap<String, String>,
    /// Der aufgelöste Wertesatz aus `ResolveWorld`; `None` = beim Anlegen auflösen (`values_of`).
    pub values: Option<Arc<WorldValues>>,
}

/// Go: `worldPath`, der Pfad der Welt in Meldungen.
pub fn world_path(id: &str) -> String {
    world::path(id)
}

/// Go: `mustStandardWorld` und `StandardWorld`: die eingebettete Standard-Welt, aus dem schon gelesenen Kopf
/// (`Data::standard_world`) und ihren Inseln gebaut. Ohne eigene Welt spielt die Sim sie. Der Fehler trägt wie in Go den
/// Pfad der Datei (Go bricht beim Start mit `panic` ab; die Aufrufer melden ihn).
pub fn standard_world(data: &Data, rules: &Rules) -> Result<WorldDef, String> {
    let file = &data.standard_world;
    let keys: Vec<Keys> = file.islands.iter().flatten().map(Keys::from_value).collect();
    build_world(file, &keys, data, rules).map_err(|e| format!("{}: {e}", world_path(STANDARD_WORLD_ID)))
}

/// Go: `LoadWorld` liest die Welt `id` aus `source` (Datei `worlds/<id>.json`) und prüft sie wie `parse_world`. Eine ID
/// außerhalb des Musters wird abgelehnt, bevor gelesen wird.
pub fn load_world(source: &Source, id: &str, data: &Data, rules: &Rules) -> Result<WorldDef, String> {
    check_world_id(id)?;
    let raw = source.read(&format!("worlds/{id}.json")).map_err(|e| e.to_string())?;
    parse_world(&raw, id, data, rules)
}

/// Go: `ParseWorld` liest den Text der Welt-Datei der Welt `id` streng und prüft ihn (welten.md § 3). Fehler nennen
/// Datei, Insel, Tiefe und Platz, soweit vorhanden: `<Datei>: Insel <insel-id>: <Grund>`.
pub fn parse_world(raw: &str, id: &str, data: &Data, rules: &Rules) -> Result<WorldDef, String> {
    check_world_id(id)?;
    let file: WorldFile = world::parse(raw, id).map_err(|e| e.to_string())?;
    let keys = island_keys(raw);
    build_world(&file, &keys, data, rules).map_err(|e| format!("{}: {e}", world_path(id)))
}

/// Go: `checkWorldID`.
fn check_world_id(id: &str) -> Result<(), String> {
    if world::valid_id(id) {
        return Ok(());
    }
    Err(format!("Welt-ID {id:?} ungültig (a-z, 0-9, -, höchstens 32 Zeichen)"))
}

/// Die Schlüsselbäume der Inseln aus dem Text der Datei (doppelte Schlüssel sieht nur der Text).
fn island_keys(raw: &str) -> Vec<Keys> {
    #[derive(Deserialize, Default)]
    #[serde(default)]
    struct Top {
        islands: Vec<Keys>,
    }
    serde_json::from_str::<Top>(raw).map(|t| t.islands).unwrap_or_default()
}

/// Baut die Welt aus dem gelesenen Kopf: Kampfvariante, Geschichte, Skins, Inseln, Prüfung der Geschichte (Go:
/// `decodeWorld` ab `combat.settings`, `decodeIslands`, `checkStory`). Der Kopf ist geprüft (`k3c_data::world::parse`).
fn build_world(file: &WorldFile, keys: &[Keys], data: &Data, rules: &Rules) -> Result<WorldDef, String> {
    let (Some(id), Some(name), Some(islands)) = (&file.id, &file.name, &file.islands) else {
        return Err("id, name oder islands fehlt (Pflichtfeld)".to_string());
    };
    let combat = file
        .combat
        .as_ref()
        .map_or_else(CombatSettings::default, |c| CombatSettings {
            mode: c.mode.clone().unwrap_or_default(),
            group_size: c.group_size.unwrap_or(0),
        });
    let mut w = WorldDef {
        id: id.clone(),
        name: name.clone(),
        notes: file.notes.clone(),
        combat,
        extends: file.extends.clone().unwrap_or_default(),
        overrides: file.overrides.clone().unwrap_or_default(),
        ..WorldDef::default()
    };
    if !file.story.is_null() {
        w.story = decode_world_story(&file.story)?;
    }
    if !file.skins.is_null() {
        w.skins = decode_world_skins(&file.skins)?;
    }
    w.islands = decode_islands(islands, keys, data, rules)?;
    if has_story(file, islands) {
        check_story(&w, data)?;
    }
    Ok(w)
}

/// Hat die Welt oder eine ihrer Inseln ein Feld `story`? Ohne Geschichte ist `checkStory` ohne Wirkung (story_check.go).
fn has_story(file: &WorldFile, islands: &[Value]) -> bool {
    !file.story.is_null() || islands.iter().any(|d| d.get("story").is_some_and(|s| !s.is_null()))
}
