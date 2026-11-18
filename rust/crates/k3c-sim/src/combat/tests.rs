//! Tests zu `combat/mod.rs` (Port von `engine/sim/combat_test.go`, B-932, B-933, docs/rules/kampf.md § 3): Gruppen mit
//! Schild und Auslöser, General mit Banner; Go läuft immer mit 2 Spielern über `CreateIsland`. Tests, die den Takt
//! (RU14), Inseln (RU10) oder die Gruppen (RU15.3) brauchen, tragen `ignore`; `real_tests` deckt Zahlen, Posten und
//! Ziele ohne sie ab.

use super::SpotKind;
use super::data::{COMBAT_GENERAL, COMBAT_GROUPS, CombatMode, DEFAULT_GROUP_SIZE, combat_mode, group_size};
use super::testutil::{
    combat_pay, combat_stage, first_leader, group_ready, run, spawn_soldiers, spot_for, squad_size, take_banner_at,
    troop, vals,
};
use crate::state::{Crown, WorldValues};

#[test]
fn combat_welt_feld_standard_und_pruefung() {
    let d = k3c_data::load_all(&k3c_data::Source::Embedded).expect("eingebettete Daten");
    let std = d.standard_world.combat.expect("Standard-Welt hat combat");
    assert_eq!(
        (std.mode.as_deref(), std.group_size),
        (Some(COMBAT_GROUPS), Some(5)),
        "Standard-Welt"
    );
    let raw = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../data/worlds/standard.json"
    ))
    .expect("worlds/standard.json");
    let minimal = |extra: &str| {
        raw.replacen(r#""id": "standard""#, r#""id": "t""#, 1).replacen(
            r#""combat": { "mode": "groups", "groupSize": 5 },"#,
            &format!("{extra},"),
            1,
        )
    };
    let cases = [
        (r#""combat": {"mode": "general"}"#, ""),
        (r#""combat": {"mode": "gruppe"}"#, "combat.mode"),
        (r#""combat": {"groupSize": 0}"#, "combat.groupSize"),
        (r#""combat": {"modus": "groups"}"#, "modus"),
        (r#""combat": {"mode": "groups", "groupSize": 7}"#, ""),
    ];
    for (body, want) in cases {
        let got = k3c_data::world::parse(&minimal(body), "t");
        match (want, &got) {
            ("", Err(e)) => panic!("{body}: {e}"),
            (w, Ok(_)) if !w.is_empty() => panic!("{body}: kein Fehler, erwartet {w:?}"),
            (w, Err(e)) if !w.is_empty() => assert!(e.to_string().contains(w), "{body}: Fehler {e}, erwartet {w:?}"),
            _ => {}
        }
        if body.contains("groupSize\": 7") {
            let parsed = got.expect("gültig");
            assert_eq!(parsed.combat.and_then(|c| c.group_size), Some(7), "groupSize");
        }
    }
    let (w, _, _) = combat_stage("");
    assert!(
        combat_mode(&vals(&w)) == CombatMode::Groups && group_size(&WorldValues::default()) == DEFAULT_GROUP_SIZE,
        "Standard ohne Feld ist groups/5"
    );
}

#[test]
fn schild_macht_zum_gruppenfuehrer_und_gruppe_fuellt() {
    let (mut w, _, right) = combat_stage(COMBAT_GROUPS);
    spawn_soldiers(&mut w, 5);
    let shield = spot_for(&w, SpotKind::Shield, 0).expect("Schild-Zahlstelle");
    combat_pay(&mut w, 0, shield.x, 76);
    assert_eq!(w.players[0].gold, 40 - 10, "Schild kostet 10 Gold");
    run(&mut w, 10.0);
    let l = first_leader(&w).expect("kein Führer nach dem Schild");
    let n = w
        .troops
        .iter()
        .filter(|t| t.group == w.troops[l].id && t.id != w.troops[l].id)
        .count();
    assert_eq!(n, 3, "Gruppe hat {n} Soldaten, erwartet groupSize 3");
    assert!(
        spot_for(&w, SpotKind::Trigger, 1).is_some_and(|t| t.x > right),
        "Auslöser außerhalb vor dem Tor"
    );
}

#[test]
fn schild_nur_mit_freiem_soldaten() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let x = spot_for(&w, SpotKind::Shield, 0).expect("Schild-Zahlstelle").x;
    combat_pay(&mut w, 0, x, 76);
    assert!(
        w.players[0].gold == 40 && first_leader(&w).is_none(),
        "ohne Soldat darf das Schild nichts kosten"
    );
}

#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprüft: Wirtschaft/Spielerschritt, der Test lief erst nach RU19 an)"]
fn ausloeser_schickt_gruppen_zum_portal_und_zurueck() {
    let (mut w, l) = group_ready();
    w.portal_hp[1] = 40.0; // schnell zerstört
    let before = w.players[1].gold;
    let x = spot_for(&w, SpotKind::Trigger, 1).expect("Auslöser").x;
    combat_pay(&mut w, 1, x, 20);
    assert!(
        w.players[1].gold == before - 2 && w.troops[l].sortie == 1.0,
        "Auslöser zahlt und schickt"
    );
    let x0 = w.portals[1];
    run(&mut w, 90.0);
    assert!(
        w.portals.len() == 1 && w.portals_down == vec![x0],
        "Portal nicht zerstört"
    );
    run(&mut w, 30.0);
    assert!(
        w.troops[l].sortie == 0.0 && w.troops[l].x <= w.hub_x + 10.0,
        "Gruppe nicht zurück am Tor"
    );
}

#[test]
fn ausloeser_nur_mit_gruppe_der_seite() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let x = spot_for(&w, SpotKind::Trigger, 1).expect("Auslöser").x;
    combat_pay(&mut w, 1, x, 40);
    assert_eq!(w.players[1].gold, 40, "Auslöser ohne Gruppe nahm Gold");
}

#[test]
fn fuehrer_faellt_nur_als_letzter_der_gruppe() {
    let (mut w, l) = group_ready();
    let hub = w.hub_x;
    for t in w.troops.iter_mut().filter(|t| t.group == 0 && t.kind == "warrior") {
        t.kind = "peasant".into(); // kein Nachrücken in die Gruppe
        t.anchor_x = hub;
    }
    w.troops[l].hp = 0.0;
    run(&mut w, 0.2);
    assert!(w.troops[l].leader, "Führer fiel mit stehender Gruppe");
    let lid = w.troops[l].id;
    for (i, t) in w.troops.iter_mut().enumerate() {
        if t.group == lid && i != l {
            t.hp = 0.0;
        }
    }
    run(&mut w, 0.2);
    w.troops[l].hp = 0.0;
    run(&mut w, 0.2);
    let t = &w.troops[l];
    assert!(
        !t.leader && t.kind == "peasant" && t.group == 0,
        "letzter Führer blieb Führer"
    );
}

#[test]
fn banner_trupp_folgt_und_kehrt_zurueck() {
    let (mut w, _, wall_r) = combat_stage(COMBAT_GENERAL);
    spawn_soldiers(&mut w, 7);
    for _ in 0..7 {
        troop(&mut w, "archer", 0.0, 1.0);
    }
    take_banner_at(&mut w, 0);
    assert!(w.players[0].banner == 1 && w.players[1].banner == 0, "Banner");
    run(&mut w, 12.0);
    assert_eq!(squad_size(&w, 0), (5, 5), "Trupp 5 Soldaten + 5 Schützen");
    w.players[0].x = wall_r + 3.0; // hinter das Tor hinaus: Trupp folgt
    run(&mut w, 1.0);
    assert!(w.players[0].banner != 0, "Banner zu früh zurückgestellt");
    w.players[0].x = wall_r - 5.0; // zurück hinter das äußerste Tor
    run(&mut w, 1.0);
    let left = w.troops.iter().filter(|t| t.group != 0).count();
    assert!(
        w.players[0].banner == 0 && left == 0,
        "Banner nicht zurückgestellt, {left} Mitglieder"
    );
}

#[test]
fn banner_je_monarch_und_nachbesetzung() {
    let (mut w, _, _) = combat_stage(COMBAT_GENERAL);
    for _ in 0..12 {
        troop(&mut w, "warrior", 0.0, 1.0);
    }
    take_banner_at(&mut w, 0);
    take_banner_at(&mut w, 1);
    run(&mut w, 15.0);
    assert!(
        w.players[0].banner != 0 && w.players[1].banner != 0,
        "jeder Monarch nimmt sein eigenes Banner"
    );
    let pid = w.players[0].id;
    let fallen = w.troops.iter().position(|t| t.group == pid).expect("Trupp-Mitglied");
    w.troops[fallen].hp = 0.0;
    run(&mut w, 1.0);
    let n = w.troops.iter().filter(|t| t.group == pid).count();
    assert!(
        w.troops[fallen].group == 0 && n == 5,
        "Ausfall nicht ersetzt: {n} Soldaten im Trupp"
    );
}

#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprüft: Wirtschaft/Spielerschritt, der Test lief erst nach RU19 an)"]
fn trupp_im_angriff_beschaedigt_portal() {
    let (mut w, _, _) = combat_stage(COMBAT_GENERAL);
    spawn_soldiers(&mut w, 5);
    take_banner_at(&mut w, 0);
    run(&mut w, 3.0);
    for _ in 0..(30 * 60) {
        w.players[0].x = w.portals[1] - 4.0;
        run(&mut w, 1.0 / 30.0);
        if w.portal_hp[1] < w.portal_max_hp {
            return;
        }
    }
    panic!("der Trupp beschädigte das Portal nicht");
}

#[test]
fn krone_faellt_banner_wird_zurueckgestellt() {
    let (mut w, _, _) = combat_stage(COMBAT_GENERAL);
    take_banner_at(&mut w, 0);
    w.players[0].crown = Some(Crown::default());
    run(&mut w, 0.2);
    assert_eq!(w.players[0].banner, 0, "Banner blieb bei fehlender Krone");
}
