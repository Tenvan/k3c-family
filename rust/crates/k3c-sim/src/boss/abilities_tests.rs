//! Tests zu `abilities.rs` (Port von `engine/sim/boss_abilities_test.go`, K2.1b, B-130/AC-01): Auslöser, Flammenspur,
//! Splitter-Wurf, Belohnung, ein Miniboss je Stufe der Insel 1, Determinismus. Tests, die `CreateIsland` (RU10),
//! `Step` (RU14) oder die Ereignisse (RU19) brauchen, tragen `ignore`; `real_tests` deckt den Rest ohne Insel ab.

use super::abilities::{arm_boss, step_hazards};
use super::boss_ability_with;
use super::data::mini_boss;
use super::place_boss;
use super::testutil::{boss_stage, boss_values, bosses_in, deep_island, start_wave, wave_until};
use crate::enemies::testutil::{run, step_for};
use crate::enemies::waves::remove_dead_enemies;
use crate::state::Hazard;

// (a) Lava-Golem und Splitter-Titan kommen mit Welle 3 ihrer Stufe über das erste Portal, nicht mit Welle 2.
#[test]
#[ignore = "wartet auf RU10 (deep_island (CreateIsland, AddIslandPlayer))"]
fn boss_miniboss_eisenstollen_und_kristallhoehle() {
    let mut isl = deep_island(1, 0);
    for (stage, id) in [(3, "lavaGolem"), (4, "shardTitan")] {
        wave_until(&mut isl.stages[stage], 2);
        assert!(bosses_in(&isl.stages[stage]).is_empty(), "{id} schon in Welle 2");
        start_wave(&mut isl, stage);
        let w = &isl.stages[stage];
        let b = bosses_in(w);
        assert!(
            b.len() == 1 && w.enemies[b[0]].kind == id && w.enemies[b[0]].x == w.portals[0],
            "Welle 3: erwartet {id} am ersten Portal"
        );
    }
}

// (b) Flammenspur: Die Fläche trifft beide Spieler darin je Sekunde, eine Truppe außerhalb nicht; ein stehender Boss
// stapelt keine Flächen; nach Ablauf ist die Fläche weg und schadet nicht mehr.
#[test]
#[ignore = "wartet auf RU10 (deep_island (CreateIsland, AddIslandPlayer))"]
fn boss_flammenspur() {
    let mut isl = deep_island(2, 3);
    let w = &mut isl.stages[3];
    let v = boss_values();
    let b = mini_boss(&v.bosses, 3).expect("Miniboss");
    let x = w.hub_x + 20.0;
    let boss = super::spawn_boss(w, &v, b, x, None);
    let a = &b.ability;
    w.enemies[boss].aoe_in = 0.0;
    boss_ability_with(w, &v, boss, None);
    w.enemies[boss].aoe_in = 0.0;
    boss_ability_with(w, &v, boss, None);
    assert!(
        w.hazards.len() == 1 && w.hazards[0].x == x && w.hazards[0].radius == a.radius,
        "Flächen"
    );
    w.enemies.clear(); // nur die Fläche wirkt
    run(w, 0.5);
    run(w, a.duration_seconds);
    assert!(
        w.hazards.is_empty(),
        "Fläche nach {} s nicht ausgelaufen",
        a.duration_seconds
    );
    // Schaden an beiden Spielern je Sekunde, keiner an einer Truppe außerhalb, keiner nach Ablauf: braucht Spieler
    // (`probe`, `wasHit`, `playerHits`) und `Step` (`step`).
    panic!("B-1103: playerHits und Step (step_players)");
}

// (b) Der gehende Golem hinterlässt eine Spur aus mehreren Flächen.
#[test]
#[ignore = "wartet auf RU10 (deep_island (CreateIsland, AddIslandPlayer))"]
fn boss_flammenspur_beim_gehen() {
    let mut isl = deep_island(1, 0);
    let w = &mut isl.stages[3];
    let v = boss_values();
    let b = mini_boss(&v.bosses, 3).expect("Miniboss");
    let x = w.portals[0];
    super::spawn_boss(w, &v, b, x, None);
    run(w, 5.0);
    assert!(
        w.hazards.len() >= 2,
        "{} Flächen nach 5 s Gehen, erwartet eine Spur",
        w.hazards.len()
    );
}

// (c) Splitter-Wurf: Der Einschlag trifft das Ziel und eine nahe Truppe, eine ferne nicht; jeder Angriff des
// Splitter-Titans ist ein Wurf mit Splash.
#[test]
#[ignore = "wartet auf RU10 (deep_island (CreateIsland, AddIslandPlayer))"]
fn boss_splitter_wurf() {
    let mut isl = deep_island(1, 0);
    let w = &mut isl.stages[4];
    let v = boss_values();
    let b = mini_boss(&v.bosses, 4).expect("Miniboss");
    let a = &b.ability;
    let x = w.hub_x + 30.0;
    let boss = super::spawn_boss(w, &v, b, x, None);
    boss_ability_with(w, &v, boss, None);
    assert!(
        w.projectiles.len() == 1 && w.projectiles[0].splash == a.radius,
        "Geschosse {:?}, erwartet ein Splitter",
        w.projectiles
    );
    assert!(
        w.enemies[boss].cooldown == a.interval_seconds && w.enemies[boss].aoe_in == a.interval_seconds,
        "Abklingzeit"
    );
    crate::enemies::waves::step_projectiles(w, 1.0 / 30.0);
    // Ziel und nahe Truppe verlieren den Schaden, die ferne nicht; danach 200 Takte lang nur Würfe mit Splash:
    // braucht Truppen (`warriorAt`) und `Step` (`step`).
    panic!("B-1103: Step (step_players)");
}

// (d) Belohnung: 100 Gold als Münzen und 50 Eisen bzw. Kristall im Vorrat der Insel.
#[test]
#[ignore = "wartet auf RU10 (deep_island (CreateIsland, AddIslandPlayer))"]
fn boss_belohnung_eisen_und_kristall() {
    let mut isl = deep_island(2, 0);
    let v = boss_values();
    for stage in [3, 4] {
        let w = &mut isl.stages[stage];
        let b = mini_boss(&v.bosses, stage as i64).expect("Miniboss");
        let x = w.hub_x + 15.0;
        let boss = super::spawn_boss(w, &v, b, x, None);
        let coins = w.coins.len();
        w.enemies[boss].hp = 0.0;
        remove_dead_enemies(w, None);
        assert_eq!(w.coins.len() - coins, 14, "Stufe {stage}: Münzen");
    }
    panic!("RU10: Insel-Vorrat Eisen und Kristall");
}

// (e) Jede Stufe der Insel 1 hat genau einen Miniboss.
#[test]
fn boss_jede_stufe_ein_miniboss() {
    let v = boss_values();
    let want = ["goblinLeader", "trollKing", "ratQueen", "lavaGolem", "shardTitan"]; // Wald bis Kristallhöhle
    for (depth, id) in want.iter().enumerate() {
        let n = v
            .bosses
            .bosses
            .iter()
            .filter(|b| b.kind == "mini" && b.depth == depth as i64)
            .count();
        let first = mini_boss(&v.bosses, depth as i64).map(|b| b.id.as_str());
        assert!(
            n == 1 && first == Some(*id),
            "Tiefe {depth}: {n} Minibosse, erwartet genau {id}"
        );
    }
}

// (f) Gleicher Seed, gleicher Verlauf: zwei Spieler, beide Bosse kämpfen, Flächen und Splitter wirken.
#[test]
#[ignore = "wartet auf RU10 (deep_island (CreateIsland, AddIslandPlayer))"]
fn boss_flaechen_deterministisch() {
    let trace = || {
        let mut isl = deep_island(1, 3);
        let v = boss_values();
        for s in [3usize, 4] {
            let w = &mut isl.stages[s];
            let b = mini_boss(&v.bosses, s as i64).expect("Miniboss");
            let x = w.hub_x + 15.0;
            super::spawn_boss(w, &v, b, x, None);
        }
        let (mut hazards, mut shards) = (0, 0);
        for _ in 0..40 {
            step_for(&mut isl.stages[3], 1.0);
            hazards += isl.stages[3].hazards.len();
            shards += isl.stages[4].projectiles.len();
        }
        assert!(
            hazards != 0 && shards != 0,
            "Flächen {hazards}, Splitter {shards}: Bosse wirken nicht"
        );
        format!("{hazards} {shards}")
    };
    assert_eq!(trace(), trace(), "Verlauf weicht ab");
}

mod real_tests {
    //! Fälle von `boss_abilities_test.go`, die ohne Insel, Spieler und Ereignisse laufen.

    use super::*;

    #[test]
    fn arm_boss_wartet_das_intervall_ab() {
        let (mut w, isl) = boss_stage(3, 1);
        let v = boss_values();
        let b = mini_boss(&v.bosses, 3).expect("Lava-Golem");
        let ei = place_boss(&mut w, &v, b, 20.0, Some(&isl));
        assert_eq!(w.enemies[ei].aoe_in, 1.0, "erste Flammenspur nach dem Intervall");
    }

    #[test]
    fn arm_boss_splitter_wurf_ist_sofort_bereit_und_fernkampf() {
        let (mut w, isl) = boss_stage(4, 1);
        let v = boss_values();
        let b = mini_boss(&v.bosses, 4).expect("Splitter-Titan");
        let ei = place_boss(&mut w, &v, b, 20.0, Some(&isl));
        assert_eq!(w.enemies[ei].aoe_in, 0.0);
        assert_eq!(w.enemies[ei].traits, vec!["ranged".to_string()]);
        let mut e = w.enemies[ei].clone();
        arm_boss(&mut e, &mini_boss(&v.bosses, 3).expect("Lava-Golem").ability);
        assert_eq!(e.aoe_in, 1.0, "anderer Typ: wieder Wartezeit");
    }

    #[test]
    fn flammenspur_legt_eine_flaeche_und_stapelt_keine() {
        let (mut w, isl) = boss_stage(3, 1);
        let v = boss_values();
        let b = mini_boss(&v.bosses, 3).expect("Lava-Golem");
        let x = 120.0;
        let ei = place_boss(&mut w, &v, b, x, Some(&isl));
        w.enemies[ei].aoe_in = 0.0;
        boss_ability_with(&mut w, &v, ei, Some(&isl));
        w.enemies[ei].aoe_in = 0.0;
        boss_ability_with(&mut w, &v, ei, Some(&isl));
        let a = &b.ability;
        assert_eq!(w.hazards.len(), 1, "stehender Boss stapelt keine Flächen");
        let h = &w.hazards[0];
        assert_eq!(
            (h.x, h.radius, h.damage_per_second, h.seconds_left),
            (x, a.radius, a.damage_per_second, a.duration_seconds)
        );
        assert_eq!(h.cause, "lavaGolem");
        assert_eq!(w.enemies[ei].aoe_in, 1.0);
        w.enemies[ei].x = x + 2.0; // aus der Fläche heraus: eine zweite Fläche (Spur beim Gehen)
        w.enemies[ei].aoe_in = 0.0;
        boss_ability_with(&mut w, &v, ei, Some(&isl));
        assert_eq!(w.hazards.len(), 2);
    }

    #[test]
    fn flaechen_laufen_nach_ihrer_dauer_aus() {
        let (mut w, _) = boss_stage(3, 1);
        let v = boss_values();
        w.hazards.push(Hazard {
            x: 120.0,
            radius: 1.5,
            damage_per_second: 10.0,
            seconds_left: 5.0,
            cause: "lavaGolem".into(),
            ..Hazard::default()
        });
        step_hazards(&mut w, &v, 4.0);
        assert_eq!(w.hazards.len(), 1);
        assert_eq!(w.hazards[0].seconds_left, 1.0);
        step_hazards(&mut w, &v, 1.0);
        assert!(w.hazards.is_empty(), "Fläche nicht ausgelaufen");
    }

    #[test]
    fn flaechen_ohne_ziele_halten_den_takt_der_sekunde() {
        let (mut w, _) = boss_stage(3, 1);
        let v = boss_values();
        w.hazards.push(Hazard {
            x: 120.0,
            radius: 1.5,
            seconds_left: 5.0,
            ..Hazard::default()
        });
        step_hazards(&mut w, &v, 0.5); // erster Takt zündet sofort (tick_in startet bei 0) und springt auf 0,5
        assert_eq!(w.hazards[0].tick_in, 0.5);
        step_hazards(&mut w, &v, 0.5);
        assert_eq!(w.hazards[0].tick_in, 1.0, "zündet erneut und springt auf eine Sekunde");
    }
}
