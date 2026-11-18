//! Endboss je Insel und Bau in der tiefsten Stufe (Go: `island_endboss_test.go`, B-981, WE5.2): jede Tiefe als tiefste
//! Stufe, Wahl über `endBoss`, Fehlerfälle, zwei Spieler. `insel_endboss_fehler` läuft; die zwei anderen Go-Tests bauen
//! eine Insel und lösen den Endboss aus (`create_world_with` RU14, `AtLair`, `endSteps`, `warden` RU15) und tragen
//! `ignore`; `own_*` prüft Wahl, Tiefe und Bau-Optionen an den Daten.
#![allow(clippy::unimplemented)] // Rümpfe der ignorierten Tests: warten auf den genannten Sprint

use super::endboss::{boss_by_id, check_end_boss, end_boss, lair_options};
use super::testutil::*;
use super::{Island, IslandDef, WorldDef, add_island_player, create_island_in};
use crate::state::World;

const WARDEN_ID: &str = "crystalHeartWarden";

/// Go: `lairIsland` baut eine Insel der Test-Welt mit den Tiefen `depths` (Felder wie bei `test_island`) und `players`
/// Spielern.
fn lair_island(depths: &[i64], fields: &str, players: usize) -> (Island, IslandDef) {
    let data = test_data();
    let list: Vec<String> = depths.iter().map(i64::to_string).collect();
    let world: WorldDef = parse_test_world(&format!(r#","depths":[{}]{fields}"#, list.join(",")))
        .unwrap_or_else(|e| panic!("Welt mit Tiefen {depths:?}: {e}"));
    let def = world.islands[0].clone();
    let mut isl =
        create_island_in(&data, &world, "lair", depths, 1.0, def.create_options(&data.biomes)).expect("Insel");
    for _ in 0..players {
        add_island_player(&mut isl, depths.len() - 1);
    }
    (isl, def)
}

/// Go: `lairX`, die Stelle des Baus in der Stufe.
fn lair_x(w: &World) -> Option<f64> {
    w.level.entities.iter().find(|e| e.kind == "lair").map(|e| e.x)
}

/// Go: `AtLair` (boss_endboss.go): kommt mit RU15.
fn at_lair(_w: &World, _pi: usize) -> bool {
    unimplemented!("RU15: at_lair")
}

/// Go: `warden` (boss_endboss_test.go): der Endboss in der Stufe; kommt mit RU15.
fn warden_x(_w: &World) -> Option<f64> {
    unimplemented!("RU15: warden")
}

/// Go: `endSteps` (boss_endboss_test.go): tickt die Stufe; kommt mit RU15.
fn end_steps(_w: &mut World, _seconds: f64, _both: bool) -> Vec<crate::state::Event> {
    unimplemented!("RU15: end_steps")
}

/// Go: `TestInselTiefsteStufeTraegtBauUndEndboss` (AC-01): Eine Insel mit tiefster Stufe 0, 1, 2 oder 3 lädt und hat ihren
/// Endboss; den Bau trägt nur die tiefste Stufe.
#[test]
#[ignore = "wartet auf RU15"]
fn insel_tiefste_stufe_traegt_bau_und_endboss() {
    let data = test_data();
    for deepest in 0..4i64 {
        let depths: Vec<i64> = (0..=deepest).collect();
        let (mut isl, def) = lair_island(&depths, "", 2);
        let want = end_boss(&data.bosses, deepest).cloned();
        let b = def.end_boss_of(&data.bosses).cloned();
        assert!(
            want.is_some() && b == want && b.as_ref().is_some_and(|b| b.depth == deepest),
            "tiefste Stufe {deepest}: Endboss {b:?}, erwartet {want:?}"
        );
        for (i, w) in isl.stages.iter().enumerate() {
            let has_lair = lair_x(w).is_some();
            assert!(
                has_lair == (i as i64 == deepest) && w.lair_boss.is_some() == has_lair,
                "tiefste Stufe {deepest}, Stufe {i}: Bau {has_lair}, Endboss {:?}",
                w.lair_boss
            );
        }
        let w = &mut isl.stages[deepest as usize];
        w.players[0].x = lair_x(w).expect("Bau");
        assert!(
            w.lair_boss == want && at_lair(w, 0),
            "tiefste Stufe {deepest}: lairBoss {:?}",
            w.lair_boss
        );
    }
}

/// Go: `TestInselEndbossWahl` (AC-01): `endBoss` wählt eine andere ID; der gewählte Endboss erscheint im Bau der tiefsten
/// Stufe, ausgelöst von einem von zwei Spielern.
#[test]
#[ignore = "wartet auf RU15"]
fn insel_endboss_wahl() {
    let (mut isl, def) = lair_island(&[0, 3], &format!(r#","endBoss":"{WARDEN_ID}""#), 2);
    let data = test_data();
    assert_eq!(def.end_boss_of(&data.bosses).map(|b| b.id.as_str()), Some(WARDEN_ID));
    let w = &mut isl.stages[1];
    w.troops.clear();
    w.enemies.clear();
    w.spawn_queue.clear();
    w.aggression = None;
    let x = lair_x(w).expect("Bau");
    assert_eq!(w.biome.depth, 3, "Bau in Tiefe {}", w.biome.depth);
    w.players[0].x = x - 500.0;
    w.players[1].x = x - 10.0;
    let events = end_steps(w, 5.0, true);
    let spawned = count(&events, "bossSpawned");
    assert!(
        spawned == 1 && warden_x(w) == Some(x),
        "Endboss {:?}, {spawned}x bossSpawned",
        warden_x(w)
    );
}

/// Go: `TestInselEndbossFehler` (AC-03): unbekannte ID und Miniboss in `endBoss` lehnen die Welt mit Meldung ab (zu kurzes
/// Biom: `check_lair_fit`, `lair_tests.rs`; die kleinste Inselgröße von 16 Chunks trägt den Bau in jedem Biom).
#[test]
fn insel_endboss_fehler() {
    let cases = [
        (
            r#","endBoss":"x""#,
            r#"Insel a: endBoss "x" unbekannt (erlaubt: forestChieftain, caveColossus, pitLord, smeltTitan, crystalHeartWarden)"#,
        ),
        (
            r#","endBoss":"trollKing""#,
            r#"Insel a: endBoss "trollKing" ist ein Miniboss (erlaubt: "#,
        ),
        (r#","endBoss":"CaveColossus""#, "unbekannt"),
    ];
    for (field, want) in cases {
        match parse_test_world(field) {
            Err(e) if e.contains(want) => {}
            other => panic!("{field}: Ergebnis {other:?}, erwartet {want:?}"),
        }
    }
}

// Ab hier: Tests ohne Go-Gegenstück, sie laufen schon.

/// Wahl über `endBoss` und Standard je tiefster Stufe (Teil von `TestInselTiefsteStufeTraegtBauUndEndboss` und
/// `TestInselEndbossWahl`, der ohne Insel auskommt).
#[test]
fn own_endboss_wahl_und_tiefste_stufe() {
    let data = test_data();
    for deepest in 0..4i64 {
        let depths: Vec<String> = (0..=deepest).map(|d| d.to_string()).collect();
        let w = parse_test_world(&format!(r#","depths":[{}]"#, depths.join(","))).expect("Welt");
        let def = &w.islands[0];
        let want = end_boss(&data.bosses, deepest);
        assert_eq!(def.lair_depth(), deepest);
        let got = def.end_boss_of(&data.bosses);
        assert!(want.is_some() && got == want, "tiefste Stufe {deepest}: {got:?}");
        assert_eq!(
            def.end_boss_id(&data.bosses),
            want.map(|b| b.id.clone()).unwrap_or_default()
        );
        assert_eq!(got.map(|b| b.depth), Some(deepest));
    }
    let w = parse_test_world(&format!(r#","depths":[0,3],"endBoss":"{WARDEN_ID}""#)).expect("Welt");
    let def = &w.islands[0];
    assert_eq!(def.end_boss_id(&data.bosses), WARDEN_ID);
    assert_eq!(
        def.end_boss_of(&data.bosses).map(|b| b.depth),
        Some(4),
        "der Warden gehört zu Tiefe 4"
    );
}

#[test]
fn own_lair_options_nur_die_tiefste_stufe_traegt_bau_und_endboss() {
    let data = test_data();
    let w = parse_test_world(r#","depths":[0,2,3]"#).expect("Welt");
    let def = &w.islands[0];
    assert_eq!(lair_options(def, 0, &data), (Some(false), None));
    assert_eq!(lair_options(def, 2, &data), (Some(false), None));
    let (lair, boss) = lair_options(def, 3, &data);
    assert_eq!(lair, Some(true));
    assert_eq!(boss.as_ref(), end_boss(&data.bosses, 3));
}

#[test]
fn own_check_end_boss_meldet_unbekannt_miniboss_und_ohne_endboss() {
    let data = test_data();
    let w = parse_test_world("").expect("Welt");
    let ok = &w.islands[0];
    assert_eq!(check_end_boss(ok, &data), Ok(()));
    let unknown = IslandDef {
        end_boss: "x".into(),
        ..ok.clone()
    };
    let e = check_end_boss(&unknown, &data).expect_err("unbekannt");
    assert!(
        e.starts_with(r#"endBoss "x" unbekannt (erlaubt: forestChieftain, "#),
        "{e}"
    );
    let mini = IslandDef {
        end_boss: "trollKing".into(),
        ..ok.clone()
    };
    let e = check_end_boss(&mini, &data).expect_err("Miniboss");
    assert!(e.starts_with(r#"endBoss "trollKing" ist ein Miniboss"#), "{e}");
    // Tiefe ohne Endboss: kein Endboss in der Daten-Tabelle für diese Tiefe.
    let mut bosses = data.clone();
    bosses.bosses.end_bosses.retain(|b| b.depth != 4);
    let e = check_end_boss(ok, &bosses).expect_err("kein Endboss");
    assert_eq!(e, "kein Endboss in der tiefsten Stufe 4");
}

#[test]
fn own_boss_by_id_sucht_erst_minibosse_dann_endbosse() {
    let data = test_data();
    assert_eq!(
        boss_by_id(&data.bosses, "trollKing").map(|b| b.kind.as_str()),
        Some("mini")
    );
    assert_eq!(
        boss_by_id(&data.bosses, WARDEN_ID).map(|b| b.kind.as_str()),
        Some("end")
    );
    assert!(boss_by_id(&data.bosses, "gibtsnicht").is_none());
    assert!(boss_by_id(&data.bosses, "").is_none());
}
