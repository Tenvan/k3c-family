//! Review-Fixes KS4 (Port von `engine/sim/combat_review_test.go`): Zahlziel-Vorrang, Ziel-Prüfung, Banner je Monarch,
//! Stufenwechsel, Spielstand. Tests, die Takt (RU14), Inseln (RU10) oder Zahlziele (RU18) brauchen, tragen `ignore`.

use super::SpotKind;
use super::data::COMBAT_GENERAL;
use super::testutil::{combat_pay, combat_stage, group_ready, run, spawn_soldiers, spot_for};

#[test]
#[ignore = "wartet auf RU18 (speed_of)"]
fn ausloeser_baner_nicht_von_bauplatz_ueberdeckt() {
    for mode in [super::data::COMBAT_GROUPS, super::data::COMBAT_GENERAL] {
        let (mut w, _, _) = combat_stage(mode);
        w.hub_level = 9;
        let kind = if mode == super::data::COMBAT_GENERAL {
            SpotKind::Banner
        } else {
            spawn_soldiers(&mut w, 2);
            let shield = spot_for(&w, SpotKind::Shield, 0).expect("Schild").x;
            combat_pay(&mut w, 0, shield, 76);
            run(&mut w, 10.0);
            SpotKind::Trigger
        };
        let spot = spot_for(&w, kind, 1).expect("Zahlziel");
        // Turm-Bauplatz in Zahl-Reichweite, außerhalb von spotClearUnits (sonst wiche das Ziel aus, B-941)
        let s = crate::world::empty_site(&mut w, "tower", spot.x - 1.6);
        w.sites.push(s);
        w.players[1].x = spot.x;
        let tg = crate::economy::find_pay_target(&w, &super::testutil::vals(&w), None, &w.players[1]);
        let covered = !matches!(&tg, Some(crate::economy::PayTarget::Combat { spot: s, .. }) if s.kind == kind);
        assert!(!covered, "{mode}: Zahlziel {tg:?} überdeckt vom Bauplatz");
    }
}

#[test]
#[ignore = "wartet auf RU18.5 (speed_of)"]
fn ausloeser_nur_mit_ziel() {
    let (mut w, l) = group_ready();
    w.portals = vec![w.hub_x - 40.0]; // kein Portal und kein Gegner auf der rechten Seite
    w.portal_hp.truncate(1);
    let before = w.players[1].gold;
    super::testutil::combat_pay(&mut w, 1, 0.0, 30);
    assert!(
        w.players[1].gold == before && w.troops[l].sortie == 0.0,
        "Auslöser ohne Ziel nahm Gold"
    );
}

#[test]
fn banner_schluessel_je_monarch() {
    let (w, _, _) = combat_stage(COMBAT_GENERAL);
    let s = spot_for(&w, SpotKind::Banner, 1).expect("Banner");
    assert_ne!(
        s.key(&w.players[0]),
        s.key(&w.players[1]),
        "Banner-Schlüssel muss je Monarch verschieden sein"
    );
    assert_eq!(s.key(&w.players[1]), "banner:1:1");
}

#[test]
fn schluessel_von_schild_und_ausloeser_ohne_monarch() {
    let (w, _, _) = combat_stage(super::data::COMBAT_GROUPS);
    let trigger = spot_for(&w, SpotKind::Trigger, -1).expect("Auslöser links");
    assert_eq!(trigger.key(&w.players[0]), "trigger:-1");
    assert_eq!(trigger.key(&w.players[1]), "trigger:-1");
    let shield = spot_for(&w, SpotKind::Shield, 0).expect("Schild");
    assert_eq!(shield.key(&w.players[0]), "shield:0");
}

#[test]
#[ignore = "wartet auf RU10"]
fn banner_bleibt_beim_stufenwechsel_am_tor() {
    panic!("RU10: mustIsland, moveToStage (Banner, bannerOut)");
}

#[test]
#[ignore = "wartet auf RU10"]
fn gruppenfuehrer_und_schild_im_spielstand() {
    panic!("RU10: mustIsland, ToSave, loadIsland (Leader, Group, combat.shields)");
}
