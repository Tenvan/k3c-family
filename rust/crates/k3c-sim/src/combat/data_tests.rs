//! Tests zu `data.rs` (Go hat für `combat_data.go` keinen eigenen Test; das Soll ist aus dem Go-Text von Hand abgeleitet).

use super::data::{
    COMBAT_GENERAL, COMBAT_GROUPS, CombatMode, DEFAULT_GROUP_SIZE, chain_combat, combat_mode, group_size, settings,
};
use crate::state::WorldValues;
use crate::state::shared::CombatSettings;
use k3c_data::world::RawCombat;

fn cs(mode: &str, size: i64) -> CombatSettings {
    CombatSettings {
        mode: mode.into(),
        group_size: size,
    }
}

#[test]
fn settings_ohne_combat_sind_die_standardwerte() {
    assert_eq!(settings(None), Ok(CombatSettings::default()));
    assert_eq!(settings(Some(&RawCombat::default())), Ok(CombatSettings::default()));
}

#[test]
fn settings_uebernimmt_gueltige_felder() {
    let r = RawCombat {
        mode: Some(COMBAT_GENERAL.into()),
        group_size: Some(7),
    };
    assert_eq!(settings(Some(&r)), Ok(cs("general", 7)));
    let r = RawCombat {
        mode: Some(COMBAT_GROUPS.into()),
        group_size: None,
    };
    assert_eq!(settings(Some(&r)), Ok(cs("groups", 0)));
}

#[test]
fn settings_lehnt_unbekannte_variante_und_kleine_gruppen_ab() {
    let bad_mode = RawCombat {
        mode: Some("gruppe".into()),
        group_size: None,
    };
    let e = settings(Some(&bad_mode)).expect_err("unbekannte Variante");
    assert_eq!(e, "combat.mode \"gruppe\" unbekannt (groups oder general)");
    let bad_size = RawCombat {
        mode: None,
        group_size: Some(0),
    };
    let e = settings(Some(&bad_size)).expect_err("Größe 0");
    assert_eq!(e, "combat.groupSize muss eine ganze Zahl ≥ 1 sein");
}

#[test]
fn chain_combat_kind_gewinnt_feldweise() {
    assert_eq!(chain_combat(&[]), CombatSettings::default());
    let chain = [cs("general", 4), cs("", 0), cs("groups", 0), cs("", 9)];
    assert_eq!(
        chain_combat(&chain),
        cs("groups", 9),
        "Modus von der dritten, Größe von der letzten Welt"
    );
    assert_eq!(chain_combat(&[cs("general", 0), cs("", 0)]), cs("general", 0));
}

#[test]
fn combat_mode_und_group_size_mit_standard() {
    let mut v = WorldValues::default();
    assert_eq!(combat_mode(&v), CombatMode::Groups);
    assert_eq!(group_size(&v), DEFAULT_GROUP_SIZE);
    v.combat = cs("general", 3);
    assert_eq!((combat_mode(&v), group_size(&v)), (CombatMode::General, 3));
    v.combat.mode = "irgendwas".into();
    assert_eq!(combat_mode(&v), CombatMode::Groups, "alles außer general ist groups");
}
