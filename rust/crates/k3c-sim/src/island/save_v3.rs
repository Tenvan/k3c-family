//! Spielstand Version 2 und 3 (Port der Reste von `engine/sim/island_save_v3.go`, S1.4, B-022, Beschluss Q42): Fund-Pool der
//! Insel (`skillPool`) statt eines Zählers je Stufe, je Spieler die gelernten Skills und die Slots. Version 2 wird überführt:
//! Pool = Summe der Zähler, Verteilung leer. Der Truhen-Zähler steht nicht im Stand, er folgt aus den geöffneten Truhen aller
//! Stufen (`chest@…`).
//!
//! `decode_island`, `parse_island_v3`, `validate_skills` (mit `validate_learned`, `validate_slots`, `skillsOf`) liefert seit
//! RU10.2 `save_parse.rs`; hier stehen `parse_island_v2` und `restore_pool`.

use serde_json::Value;

use super::options::default_options;
use super::save_parse::{decode_island, validate_island_save};
use super::{ISLAND_SAVE_VERSION, Island, IslandSave, SaveTables};
use crate::state::Fehler;

fn fail(s: String) -> Fehler {
    Fehler::Spielstand(s)
}

/// Go: `islandV2.StageSkillPoints`: der Skill-Punkt-Zähler je Stufe eines Stands der Version 2.
fn stage_skill_points(raw: &str) -> Result<Vec<i64>, Fehler> {
    let head: Value = serde_json::from_str(raw).map_err(|e| fail(format!("spielstand: {e}")))?;
    let bad =
        || fail("spielstand: json: cannot unmarshal into Go struct field islandV2.stageSkillPoints of type int".into());
    let Some(list) = head.get("stageSkillPoints").and_then(Value::as_array) else {
        return Err(bad());
    };
    list.iter().map(|n| n.as_i64().ok_or_else(bad)).collect()
}

/// Go: `parseIslandV2` liest einen Stand der Version 2: statt des Pools ein Skill-Punkt-Zähler je Stufe, Pool = Summe der
/// Zähler (Annahme: jeder Zähler zählte eigene Funde).
pub fn parse_island_v2(raw: &str, t: &SaveTables) -> Result<IslandSave, Fehler> {
    let mut s = decode_island(raw, default_options(&t.data.difficulty), &["stageSkillPoints"])?;
    let counts = stage_skill_points(raw)?;
    if counts.len() != s.stages.len() {
        return Err(fail(format!(
            "spielstand: {} Skill-Punkt-Zähler für {} Stufen",
            counts.len(),
            s.stages.len()
        )));
    }
    s.version = ISLAND_SAVE_VERSION;
    s.skill_pool += counts.iter().sum::<i64>();
    validate_island_save(&s, None, t)?;
    Ok(s)
}

/// Go: `restorePool` setzt den Pool der Insel, spiegelt ihn in alle Stufen und leitet den Truhen-Zähler aus den geöffneten
/// Truhen ab.
pub fn restore_pool(isl: &mut Island, s: &IslandSave) {
    isl.skill_pool = s.skill_pool;
    for w in &mut isl.stages {
        w.skill_points = s.skill_pool;
    }
    isl.chests_opened = s
        .stages
        .iter()
        .flat_map(|h| &h.pickups_taken)
        .filter(|k| k.starts_with("chest@"))
        .count() as i64;
}
