//! Welten aus Welt-Dateien (Port von `engine/sim/island_worlds.go`). Rumpf aus RU10.1: nur der Typ `WorldDef`; Laden,
//! Prüfen und `ResolveWorld` kommen mit RU10.5. In Rust heißt die Welt-Datei `WorldDef`, `World` ist eine Stufe (wie in Go).

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::island::IslandDef;
use crate::state::pending::WorldStory;
use crate::state::shared::{CombatSettings, WorldValues};

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
