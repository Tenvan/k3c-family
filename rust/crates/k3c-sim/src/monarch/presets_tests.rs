//! Tests zu `check_presets` (Go: `monarch_presets_test.go`, B-504/AC-01): Jedes Preset nennt nur Skills aus
//! monarch.json › skills; sonst scheitert das Laden mit Preset und Skill.

use std::collections::BTreeMap;

use k3c_data::monarch::PresetData;

use super::*;
use crate::world::values_room::default_values;

#[test]
fn presets_nennen_nur_vorhandene_skills() {
    let monarch = default_values().monarch.clone();
    if let Err(e) = check_presets(&monarch) {
        panic!("eingebettete Daten: {e}");
    }
    let mut bad = monarch;
    bad.presets = BTreeMap::from([(
        "thief".to_string(),
        PresetData {
            name: "Dieb".into(),
            active_skills: vec!["taunt".into(), "shadowStrike".into()],
        },
    )]);
    let err = check_presets(&bad).expect_err("unbekannter Skill");
    assert!(
        err.contains("thief") && err.contains("shadowStrike"),
        "unbekannter Skill: {err}"
    );
    assert_eq!(
        err,
        "data/monarch.json: preset thief nennt unbekannten Skill shadowStrike"
    );
}
