//! Fälle von `boss_test.go`, die ohne Insel und Takt laufen: Auslöser-Bedingungen, Beschwörung, Fliehen.

use super::data::mini_boss;
use super::testutil::{boss_stage, boss_values, count_kind};
use super::{boss_ability_with, place_boss, trigger_boss_with};
use crate::state::island_ref::IslandRef;

fn island(defeated: &[&str]) -> IslandRef {
    let (_, mut isl) = boss_stage(0, 1);
    isl.defeated_bosses = defeated.iter().map(|s| s.to_string()).collect();
    isl
}

#[test]
fn trigger_wartet_auf_seine_welle() {
    let (mut w, isl) = boss_stage(0, 1);
    w.wave = 4; // Goblin-Anführer kommt mit Welle 5
    trigger_boss_with(&mut w, &boss_values(), Some(&isl));
    assert!(w.enemies.is_empty(), "schon vor Welle 5");
}

#[test]
fn trigger_braucht_ein_portal() {
    let (mut w, isl) = boss_stage(0, 1);
    w.wave = 5;
    w.portals.clear();
    trigger_boss_with(&mut w, &boss_values(), Some(&isl));
    assert!(w.enemies.is_empty(), "Boss ohne Portal");
}

#[test]
fn trigger_kommt_nach_dem_sieg_nie_wieder() {
    let (mut w, _) = boss_stage(0, 1);
    w.wave = 9;
    trigger_boss_with(&mut w, &boss_values(), Some(&island(&["goblinLeader"])));
    assert!(w.enemies.is_empty(), "besiegter Boss kehrt zurück");
}

#[test]
fn trigger_kein_zweiter_boss_solange_einer_lebt() {
    let (mut w, isl) = boss_stage(0, 1);
    w.wave = 9;
    w.enemies.push(crate::state::Enemy {
        boss: true,
        ..Default::default()
    });
    trigger_boss_with(&mut w, &boss_values(), Some(&isl));
    assert_eq!(w.enemies.len(), 1, "zweiter Boss, obwohl einer lebt");
}

#[test]
fn place_boss_stellt_wie_go_auf() {
    let (mut w, isl) = boss_stage(0, 1);
    let v = boss_values();
    let b = mini_boss(&v.bosses, 0).expect("Miniboss");
    let ei = place_boss(&mut w, &v, b, 20.0, Some(&isl));
    let e = &w.enemies[ei];
    assert!(e.boss && e.kind == "goblinLeader" && e.x == 20.0 && e.range == 1.0);
    assert_eq!(e.speed, 3.0, "Tempo aus den Daten ohne Tiefen-Skalierung in Tiefe 0");
    assert_eq!(e.aoe_in, 10.0, "erste Beschwörung nach dem Intervall");
    assert!(e.traits.is_empty(), "Boss ohne Traits");
    assert_eq!(e.hp, e.max_hp);
}

#[test]
fn goblin_anfuehrer_ruft_goblins_an_seinen_ort() {
    let (mut w, isl) = boss_stage(0, 1);
    let v = boss_values();
    let b = mini_boss(&v.bosses, 0).expect("Miniboss");
    let boss = place_boss(&mut w, &v, b, 20.0, Some(&isl));
    boss_ability_with(&mut w, &v, boss, Some(&isl));
    assert_eq!(count_kind(&w, "goblin"), 0, "vor Ablauf des Intervalls nichts");
    w.enemies[boss].aoe_in = 0.0;
    boss_ability_with(&mut w, &v, boss, Some(&isl));
    assert_eq!(count_kind(&w, "goblin"), 2);
    assert_eq!(w.enemies[boss].aoe_in, 10.0, "nächste Beschwörung in 10 s");
    let home = w.enemies[boss].home_x;
    for e in w.enemies.iter().filter(|e| e.kind == "goblin") {
        assert_eq!(
            (e.x, e.home_x),
            (20.0, home),
            "Goblin am Ort des Bosses, flieht zu seinem Portal"
        );
    }
}

#[test]
fn ratten_koenigin_ruft_ganze_schwaerme() {
    let (mut w, isl) = boss_stage(2, 1);
    let v = boss_values();
    let b = mini_boss(&v.bosses, 2).expect("Miniboss");
    let boss = place_boss(&mut w, &v, b, 20.0, Some(&isl));
    w.enemies[boss].aoe_in = 0.0;
    boss_ability_with(&mut w, &v, boss, Some(&isl));
    assert_eq!(count_kind(&w, "ratSwarm") as i64, 2 * v.enemies["ratSwarm"].swarm_size);
    assert_eq!(w.enemies[boss].aoe_in, 10.0);
}

#[test]
fn nur_bosse_nutzen_faehigkeiten() {
    let (mut w, isl) = boss_stage(0, 1);
    let v = boss_values();
    let e = crate::enemies::spawn_with(&mut w, &v, "goblin", 20.0, 1.0, 1.0, Some(&isl));
    boss_ability_with(&mut w, &v, e, Some(&isl));
    assert_eq!(w.enemies.len(), 1);
}
