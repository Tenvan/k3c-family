//! Tests zu `data.rs` mit den eingebetteten Daten (Go hat dafür keine eigene Testdatei).

use k3c_data::{Source, load_all};

use super::data::*;
use super::{WorldValues, shared::SkillTable};

#[test]
fn globaler_zyklus_ist_der_des_ersten_tag_nacht_bioms() {
    let d = load_all(&Source::Embedded).unwrap();
    let c = first_day_night(&d.biomes);
    assert_eq!(c.r#type, "dayNight");
    let first = d.biomes.iter().find(|b| b.cycle.r#type == "dayNight").unwrap();
    assert_eq!(c, first.cycle);
}

#[test]
fn has_depth_kennt_die_biom_tiefen() {
    let d = load_all(&Source::Embedded).unwrap();
    for b in &d.biomes {
        assert!(has_depth(&d.biomes, b.depth));
    }
    assert!(!has_depth(&d.biomes, 999));
}

#[test]
#[should_panic(expected = "Kein Biom mit Tag/Nacht-Zyklus")]
fn ohne_tag_nacht_biom_bricht_first_day_night_ab() {
    first_day_night(&[]);
}

#[test]
fn check_waves_wie_go() {
    let d = load_all(&Source::Embedded).unwrap();
    assert!(check_waves(&d.waves).is_ok());
    let mut w = d.waves.clone();
    w.boss_hp_per_extra_player = None;
    assert_eq!(
        check_waves(&w).unwrap_err(),
        "data/waves.json: bossHpPerExtraPlayer fehlt"
    );
    w.boss_hp_per_extra_player = Some(-1.0);
    assert!(check_waves(&w).unwrap_err().contains("dürfen nicht negativ sein"));
}

#[test]
fn wertesatz_baut_aus_den_daten() {
    let d = load_all(&Source::Embedded).unwrap();
    let v = WorldValues {
        buildings: d.buildings.clone(),
        troops: d.troops.clone(),
        economy: d.economy.clone(),
        hub: d.hub.clone(),
        monarch: d.monarch.clone(),
        enemies: d.enemies.clone(),
        waves: d.waves.clone(),
        goals: d.goals.clone(),
        events: d.events.events.clone(),
        skills: SkillTable::default(),
        ..Default::default()
    };
    assert!(v.buildings.contains_key("wall"));
    let shared = std::sync::Arc::new(v);
    assert_eq!(std::sync::Arc::strong_count(&std::sync::Arc::clone(&shared)), 2);
}
