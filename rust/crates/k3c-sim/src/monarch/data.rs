//! Monarchen-Daten (Port von `engine/sim/monarch_data.go`): der Fertigkeiten-Katalog als sortierte Liste und die Prüfung
//! der Voreinstellungen.
//!
//! Die Lese-Typen (`SkillData`, `SkillEffect`, `PresetData`, `MountData`, `CrownData`, `MonarchData`) stehen in
//! `k3c_data::monarch`. Gos Paket-Variable `baseSkills` und `loadMonarch` entfallen: Der Wertesatz trägt die Tabelle
//! (`WorldValues::skills`), und das Laden samt Prüfung der Presets liegt im Wertesatz-Aufbau (`world/values_from.rs`).

use std::collections::BTreeMap;

use k3c_data::monarch::{MonarchData, SkillData};

use crate::state::WorldValues;
use crate::state::shared::SkillTable;

/// Go: `newSkillTable`: `monarch.skills` als nach ID sortierte Liste samt Stelle jeder ID; jeder Skill trägt seine ID. Jeder
/// Wertesatz hat seine eigene Tabelle: Overrides auf `monarch.skills.*` wirken je Welt (B-692). Die `BTreeMap` iteriert
/// schon nach ID.
pub fn new_skill_table(skills: &BTreeMap<String, SkillData>) -> SkillTable {
    let mut t = SkillTable::default();
    for (id, s) in skills {
        t.index.insert(id.clone(), t.list.len());
        t.list.push(SkillData {
            id: id.clone(),
            ..s.clone()
        });
    }
    t
}

/// Go: `WorldValues.skillRef`: der Skill `id` im Katalog des Wertesatzes, ohne Kopie (nur lesen); `None`, wenn es ihn nicht gibt.
pub fn skill_ref<'a>(v: &'a WorldValues, id: &str) -> Option<&'a SkillData> {
    v.skills.index.get(id).and_then(|&i| v.skills.list.get(i))
}

/// Go: `WorldValues.skillByID`: wie `skill_ref`, mit Kopie des Eintrags.
pub fn skill_by_id(v: &WorldValues, id: &str) -> Option<SkillData> {
    skill_ref(v, id).cloned()
}

/// Go: `checkPresets`: Jedes Preset nennt in `activeSkills` nur Skills aus `skills` (B-504), in fester Reihenfolge geprüft.
pub fn check_presets(m: &MonarchData) -> Result<(), String> {
    for (id, preset) in &m.presets {
        if let Some(s) = preset.active_skills.iter().find(|s| !m.skills.contains_key(*s)) {
            return Err(format!("data/monarch.json: preset {id} nennt unbekannten Skill {s}"));
        }
    }
    Ok(())
}
