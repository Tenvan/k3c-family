//! Tests zu `abilities.rs` (Port von `engine/sim/boss_abilities_test.go`, K2.1b, B-130/AC-01): Auslöser, Flammenspur,
//! Splitter-Wurf, Belohnung, ein Miniboss je Stufe der Insel 1, Determinismus. Tests, die `CreateIsland` (RU10),
//! `Step` (RU14) oder die Ereignisse (RU19) brauchen, tragen `ignore`; `real_tests` deckt den Rest ohne Insel ab.

use super::abilities::{arm_boss, step_hazards};
use super::boss_ability_with;
use super::data::mini_boss;
use super::place_boss;
use super::testutil::{boss_stage, boss_values, bosses_in, deep_island, on_stage_mut, start_wave, wave_until};
use crate::enemies::testutil::run;
use crate::enemies::waves::remove_dead_enemies;
use crate::island::step_island;
use crate::island::testutil::DT;
use crate::state::Hazard;

// (a) Lava-Golem und Splitter-Titan kommen mit Welle 3 ihrer Stufe über das erste Portal, nicht mit Welle 2.
#[test]
fn boss_miniboss_eisenstollen_und_kristallhoehle() {
    let mut isl = deep_island(1, 0);
    for (stage, id) in [(3, "lavaGolem"), (4, "shardTitan")] {
        wave_until(&mut isl, stage, 2);
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

/// Go: `playerHits` (crown_helpers_test.go): tickt die Insel `seconds` lang und zählt je Spieler-ID die `hit`-Ereignisse
/// mit Schaden in Stufe `stage`.
fn player_hits(isl: &mut crate::island::Island, stage: usize, seconds: f64) -> std::collections::BTreeMap<i64, i64> {
    let data = crate::island::testutil::test_data();
    let mut out = std::collections::BTreeMap::new();
    for _ in 0..(seconds / DT) as usize {
        step_island(isl, &[], &data.difficulty, DT);
        for ev in &isl.stages[stage].events {
            if ev["type"] == "hit" && ev["target"] == "player" && ev["damage"].as_f64().is_some_and(|d| d > 0.0) {
                *out.entry(ev["id"].as_i64().unwrap_or(-1)).or_insert(0) += 1;
            }
        }
    }
    out
}

fn run_island_for(isl: &mut crate::island::Island, seconds: f64) {
    let data = crate::island::testutil::test_data();
    crate::island::testutil::run_island(isl, &data, &[], seconds);
}

// (b) Flammenspur: Die Fläche trifft beide Spieler darin je Sekunde, eine Truppe außerhalb nicht; ein stehender Boss
// stapelt keine Flächen; nach Ablauf ist die Fläche weg und schadet nicht mehr.
#[test]
fn boss_flammenspur() {
    let mut isl = deep_island(2, 3);
    let v = boss_values();
    let b = mini_boss(&v.bosses, 3).expect("Miniboss");
    let a = &b.ability;
    let x = isl.stages[3].hub_x + 20.0;
    let w = &mut isl.stages[3];
    let boss = super::spawn_boss(w, &v, b, x, None);
    w.enemies[boss].aoe_in = 0.0;
    boss_ability_with(w, &v, boss, None);
    w.enemies[boss].aoe_in = 0.0;
    boss_ability_with(w, &v, boss, None);
    assert!(
        w.hazards.len() == 1 && w.hazards[0].x == x && w.hazards[0].radius == a.radius,
        "Flächen {:?}, erwartet eine bei {x}",
        w.hazards
    );
    w.enemies.clear(); // nur die Fläche wirkt
    (w.players[0].x, w.players[1].x) = (x - 1.0, x + 1.0);
    for p in &mut w.players {
        p.gold = 20; // Go: `probe`
    }
    let far = crate::enemies::testutil::warrior_at(w, x + a.radius + 0.5);
    run_island_for(&mut isl, 0.5);
    let w = &isl.stages[3];
    for p in &w.players {
        assert!(
            p.gold < 20 || p.crown.is_some(),
            "Spieler {} in der Fläche ohne Schaden",
            p.index
        );
    }
    assert_eq!(w.troops[far].hp, 1000.0, "Truppe außerhalb der Fläche getroffen");
    assert!(
        player_hits(&mut isl, 3, 1.0).get(&0).copied().unwrap_or(0) > 0,
        "kein Schaden in der nächsten Sekunde"
    );
    run_island_for(&mut isl, a.duration_seconds);
    assert!(
        isl.stages[3].hazards.is_empty(),
        "Fläche nach {} s nicht ausgelaufen",
        a.duration_seconds
    );
    let hits = player_hits(&mut isl, 3, 2.0);
    assert!(
        hits.values().all(|&n| n == 0),
        "Schaden nach Ablauf der Fläche: {hits:?}"
    );
}

// (b) Der gehende Golem hinterlässt eine Spur aus mehreren Flächen.
#[test]
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
fn boss_splitter_wurf() {
    let mut isl = deep_island(1, 0);
    let v = boss_values();
    let b = mini_boss(&v.bosses, 4).expect("Miniboss");
    let a = &b.ability;
    let w = &mut isl.stages[4];
    let x = w.hub_x + 30.0;
    let boss = super::spawn_boss(w, &v, b, x, None);
    let target = crate::enemies::testutil::warrior_at(w, x - 6.5);
    let near = crate::enemies::testutil::warrior_at(w, x - 6.5 - 1.5);
    let far = crate::enemies::testutil::warrior_at(w, x - 6.5 - a.radius - 1.0);
    boss_ability_with(w, &v, boss, None);
    let tid = w.troops[target].id;
    assert!(
        w.projectiles.len() == 1 && w.projectiles[0].splash == a.radius && w.projectiles[0].target_id == tid,
        "Geschosse {:?}, erwartet ein Splitter auf {tid}",
        w.projectiles
    );
    assert!(
        w.enemies[boss].cooldown == a.interval_seconds && w.enemies[boss].aoe_in == a.interval_seconds,
        "Abklingzeit"
    );
    for _ in 0..30 {
        crate::enemies::waves::step_projectiles(w, None, DT);
    }
    let dmg = w.enemies[boss].damage;
    assert!(
        w.troops[target].hp == 1000.0 - dmg && w.troops[near].hp == 1000.0 - dmg,
        "Ziel {}, nahe Truppe {}, erwartet je {}",
        w.troops[target].hp,
        w.troops[near].hp,
        1000.0 - dmg
    );
    assert_eq!(w.troops[far].hp, 1000.0, "ferne Truppe getroffen");
    let data = crate::island::testutil::test_data();
    for _ in 0..200 {
        step_island(&mut isl, &[], &data.difficulty, DT);
        let w = &isl.stages[4];
        assert!(
            !w.projectiles.iter().any(|p| p.team == "enemy" && p.splash == 0.0),
            "Splitter-Titan greift ohne Splash an"
        );
    }
}

// (d) Belohnung: 14 Gold als Münzen und 50 Eisen bzw. Kristall im Vorrat der Insel.
#[test]
fn boss_belohnung_eisen_und_kristall() {
    let mut isl = deep_island(2, 0);
    let v = boss_values();
    for stage in [3usize, 4] {
        let b = mini_boss(&v.bosses, stage as i64).expect("Miniboss");
        let before = if stage == 3 { isl.stock.iron } else { isl.stock.crystal };
        let coins = isl.stages[stage].coins.len();
        on_stage_mut(&mut isl, stage, |w, r| {
            let x = w.hub_x + 15.0;
            let boss = super::spawn_boss(w, &v, b, x, Some(&*r));
            w.enemies[boss].hp = 0.0;
            remove_dead_enemies(w, Some(r));
        });
        let after = if stage == 3 { isl.stock.iron } else { isl.stock.crystal };
        assert_eq!(isl.stages[stage].coins.len() - coins, 14, "Stufe {stage}: Münzen");
        assert_eq!(after - before, 50, "Stufe {stage}: Material");
    }
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
fn boss_flaechen_deterministisch() {
    let data = crate::island::testutil::test_data();
    let trace = || {
        let mut isl = deep_island(1, 3);
        crate::island::add_island_player(&mut isl, 4);
        let v = boss_values();
        for s in [3usize, 4] {
            let w = &mut isl.stages[s];
            let b = mini_boss(&v.bosses, s as i64).expect("Miniboss");
            let x = w.hub_x + 15.0;
            super::spawn_boss(w, &v, b, x, None);
        }
        let (mut hazards, mut shards) = (0, 0);
        for _ in 0..1200 {
            step_island(&mut isl, &[], &data.difficulty, DT);
            hazards += isl.stages[3].hazards.len();
            shards += isl.stages[4].projectiles.len();
        }
        assert!(
            hazards != 0 && shards != 0,
            "Flächen {hazards}, Splitter {shards}: Bosse wirken nicht"
        );
        let mut out = String::new();
        for s in [3usize, 4] {
            let w = &isl.stages[s];
            for e in &w.enemies {
                out += &format!(" {}:{:.3}:{:.0}", e.kind, e.x, e.hp);
            }
            for h in &w.hazards {
                out += &format!(" h{:.3}:{:.3}", h.x, h.seconds_left);
            }
            for p in &w.players {
                out += &format!(" p{}:{:.3}:{}", p.index, p.x, p.gold);
            }
        }
        out
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
        step_hazards(&mut w, &v, None, 4.0);
        assert_eq!(w.hazards.len(), 1);
        assert_eq!(w.hazards[0].seconds_left, 1.0);
        step_hazards(&mut w, &v, None, 1.0);
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
        step_hazards(&mut w, &v, None, 0.5); // erster Takt zündet sofort (tick_in startet bei 0) und springt auf 0,5
        assert_eq!(w.hazards[0].tick_in, 0.5);
        step_hazards(&mut w, &v, None, 0.5);
        assert_eq!(w.hazards[0].tick_in, 1.0, "zündet erneut und springt auf eine Sekunde");
    }
}
