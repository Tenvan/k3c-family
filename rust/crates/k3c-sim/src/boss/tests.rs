//! Tests zu `mod.rs` (Port von `engine/sim/boss_test.go`, K2.1a, B-130): Minibosse von Wald, Höhle und Mine, Auslöser,
//! Fähigkeit, Belohnung, Skalierung, Merker. Tests, die `CreateIsland` (RU10), `Step`/`startWave` (RU14) oder die
//! Ereignisse (RU19) brauchen, tragen `ignore`; ihre Fälle deckt zum Teil `real_tests` darunter ohne Insel ab.

use k3c_data::{Source, load_all};

use super::data::mini_boss;
use super::testutil::{
    boss_island, boss_stage, boss_values, bosses_in, count_kind, on_stage_mut, start_wave, wave_until,
};
use super::{boss_ability_with, place_boss, trigger_boss_with};
use crate::enemies::testutil::{count_events, run, step_for};
use crate::enemies::waves::remove_dead_enemies;
use crate::state::{Troop, World};

// (a) Wald ab Welle 5, Höhle und Mine ab Welle 3, über das erste Portal; kein zweiter, solange einer lebt.
#[test]
fn boss_miniboss_kommt_mit_seiner_welle() {
    let mut isl = boss_island(1);
    for (i, (wave, id)) in [(5, "goblinLeader"), (3, "trollKing"), (3, "ratQueen")]
        .into_iter()
        .enumerate()
    {
        wave_until(&mut isl, i, wave - 1);
        assert!(bosses_in(&isl.stages[i]).is_empty(), "{id} schon vor Welle {wave}");
        start_wave(&mut isl, i);
        let w = &isl.stages[i];
        let b = bosses_in(w);
        assert!(
            b.len() == 1 && w.enemies[b[0]].kind == id && w.enemies[b[0]].x == w.portals[0],
            "Welle {wave}: erwartet {id} am ersten Portal"
        );
        start_wave(&mut isl, i);
        assert_eq!(
            bosses_in(&isl.stages[i]).len(),
            1,
            "{id}: zweiter Boss, obwohl einer lebt"
        );
    }
}

// (b) Goblin-Anführer ruft 2 Goblins je 10 s an seinen Ort.
#[test]
fn boss_goblin_anfuehrer_ruft_goblins() {
    let mut isl = boss_island(1);
    let w = &mut isl.stages[0];
    let v = boss_values();
    let b = mini_boss(&v.bosses, 0).expect("Miniboss");
    let x = w.portals[0];
    let boss = super::spawn_boss(w, &v, b, x, None);
    let home = w.enemies[boss].home_x;
    run(w, 9.9);
    assert_eq!(count_kind(w, "goblin"), 0, "vor 10 s Goblins");
    run(w, 0.2);
    assert_eq!(count_kind(w, "goblin"), 2, "nach 10 s");
    run(w, 10.0);
    assert_eq!(count_kind(w, "goblin"), 4, "nach 20 s");
    for e in w.enemies.iter().filter(|e| e.kind == "goblin") {
        assert_eq!(e.home_x, home, "Goblin flieht nicht zum Portal des Bosses");
    }
}

// (b) Ratten-Königin ruft Rattenschwärme: 2 Schwärme je swarmSize Ratten.
#[test]
fn boss_ratten_koenigin_ruft_schwaerme() {
    let (mut w, isl) = boss_stage(2, 1);
    let v = boss_values();
    let b = mini_boss(&v.bosses, 2).expect("Miniboss");
    let x = w.portals[0];
    let boss = super::spawn_boss(&mut w, &v, b, x, Some(&isl));
    w.enemies[boss].aoe_in = 0.0;
    boss_ability_with(&mut w, &v, boss, Some(&isl));
    let want = 2 * v.enemies["ratSwarm"].swarm_size;
    assert_eq!(count_kind(&w, "ratSwarm") as i64, want, "Ratten");
    assert_eq!(w.enemies[boss].aoe_in, 10.0, "nächste Beschwörung in 10 s");
}

// (b) Troll-König trifft alle Truppen im Radius 3 um sich, nicht weiter weg.
#[test]
fn boss_troll_koenig_flaechenschlag() {
    let (mut w, isl) = boss_stage(1, 1);
    let v = boss_values();
    let b = mini_boss(&v.bosses, 1).expect("Miniboss");
    let x = w.hub_x + 20.0;
    let boss = super::spawn_boss(&mut w, &v, b, x, Some(&isl));
    let troop = |w: &mut World, dx: f64| {
        let id = w.new_id();
        w.troops.push(Troop {
            id,
            kind: "warrior".into(),
            x: x + dx,
            hp: 1000.0,
            max_hp: 1000.0,
            ..Troop::default()
        });
    };
    for dx in [-2.9, 0.0, 3.0] {
        troop(&mut w, dx);
    }
    troop(&mut w, 3.5);
    w.enemies[boss].aoe_in = 0.0;
    boss_ability_with(&mut w, &v, boss, Some(&isl));
    let damage = w.enemies[boss].damage;
    for t in &w.troops[..3] {
        assert_eq!(t.hp, 1000.0 - damage, "Truppe bei {:+.1}", t.x - x);
    }
    assert_eq!(w.troops[3].hp, 1000.0, "Truppe außerhalb des Radius getroffen");
    assert_eq!(w.enemies[boss].aoe_in, 4.0, "nächster Schlag in 4 s");
}

// (c) Belohnung: 14 Gold als Münzen, 50 Material in den Insel-Vorrat, 1 Skill-Punkt für beide Spieler.
#[test]
fn boss_belohnung_fuer_alle() {
    let mut isl = boss_island(2);
    let v = boss_values();
    let b = mini_boss(&v.bosses, 0).expect("Miniboss");
    let (coins, wood, pool) = (isl.stages[0].coins.len(), isl.stock.wood, isl.skill_pool);
    on_stage_mut(&mut isl, 0, |w, r| {
        let x = w.hub_x + 15.0;
        let boss = super::spawn_boss(w, &v, b, x, Some(&*r));
        w.enemies[boss].hp = 0.0;
        remove_dead_enemies(w, Some(r));
    });
    assert_eq!(isl.stages[0].coins.len() - coins, 14, "Münzen");
    assert_eq!(isl.stock.wood - wood, 50, "Holz");
    assert_eq!(isl.skill_pool, pool + 1, "Skill-Pool");
    for pi in 0..isl.stages[0].players.len() {
        assert_eq!(
            crate::monarch::available_points(&isl.stages[0], pi),
            pool + 1,
            "Spieler {pi}"
        );
    }
    assert!(
        isl.stages.iter().all(|s| s.skill_points == isl.skill_pool),
        "Stufe spiegelt den Pool nicht"
    );
}

// (c) Über dem Lager-Maximum verfällt der Rest des Materials.
#[test]
fn boss_material_bis_zum_lager_maximum() {
    let mut isl = boss_island(1);
    let v = boss_values();
    let b = mini_boss(&v.bosses, 0).expect("Miniboss");
    let limit = on_stage_mut(&mut isl, 0, |w, r| {
        crate::island::storage::capacity(w, &crate::world::vals(w), Some(&*r)).expect("Lager-Maximum")
    });
    isl.stock.wood = limit - 10;
    on_stage_mut(&mut isl, 0, |w, r| {
        let x = w.hub_x + 15.0;
        let boss = super::spawn_boss(w, &v, b, x, Some(&*r));
        w.enemies[boss].hp = 0.0;
        remove_dead_enemies(w, Some(r));
    });
    assert_eq!(isl.stock.wood, limit, "Holz");
}

// (d) HP × (1 + bossHpPerExtraPlayer je Zusatzspieler der Insel) × enemyHp des Grads, nicht × waveSize; Tiefenfaktor
// greift. Ohne `CreateIsland`: Der Bezug zur Insel (`IslandRef`) trägt Grad und Spielerzahl; `place_boss` stellt den
// Boss ohne das Ereignis auf.
#[test]
fn boss_skalierung() {
    let d = load_all(&Source::Embedded).expect("eingebettete Daten");
    let v = boss_values();
    let hp_of = |players: i64, depth: i64, grade: &str| -> f64 {
        let (mut w, mut isl) = boss_stage(depth, players);
        isl.grade = (&d.difficulty.grades[grade]).into();
        let b = mini_boss(&v.bosses, depth).expect("Miniboss");
        let x = w.portals[0];
        let ei = place_boss(&mut w, &v, b, x, Some(&isl));
        w.enemies[ei].max_hp
    };
    let (one, two) = (hp_of(1, 0, "normal"), hp_of(2, 0, "normal"));
    let want = one * (1.0 + v.waves.boss_hp_per_extra_player.expect("Pflichtfeld"));
    assert!((two - want).abs() <= 1.0, "2 Spieler: HP {two}, erwartet {want}");
    assert_eq!(one, v.enemies["goblin"].hp * 8.0, "Wald");
    let g = &d.difficulty.grades["hard"];
    assert_ne!(
        g.wave_size, g.enemy_hp,
        "Testannahme: waveSize und enemyHp von hard verschieden"
    );
    assert_eq!(hp_of(1, 0, "hard"), (one * g.enemy_hp).round(), "hard: nur enemyHp");
    let cave = (v.enemies["skeleton"].hp * v.waves.depth_scaling.hp * 8.0).round();
    assert_eq!(hp_of(1, 1, "normal"), cave, "Höhle: Tiefenfaktor");
}

// (e) Ein Boss flieht weder bei Tagesanbruch noch bei halber HP; seine Beschwörungen schon.
#[test]
fn boss_flieht_nie() {
    let mut isl = boss_island(1);
    let w = &mut isl.stages[0];
    let v = boss_values();
    let b = mini_boss(&v.bosses, 0).expect("Miniboss");
    let x = w.hub_x + 30.0;
    let boss = super::spawn_boss(w, &v, b, x, None);
    crate::enemies::spawn_with(w, &v, "goblin", x, 1.0, 1.0, None);
    w.enemies[boss].hp = w.enemies[boss].max_hp * 0.4;
    crate::enemies::send_enemies_home(w);
    run(w, 0.5);
    assert!(!w.enemies[boss].fleeing, "Boss flieht");
    assert!(
        w.enemies.iter().filter(|e| !e.boss).all(|e| e.fleeing),
        "Goblin flieht bei Tagesanbruch nicht"
    );
}

// (f) Ein besiegter Boss kommt nie wieder, auch nicht nach einem Burgfall; ein lebender kommt nach dem Burgfall
// mit der nächsten Welle erneut.
#[test]
fn boss_besiegt_kommt_nie_wieder() {
    let mut isl = boss_island(1);
    wave_until(&mut isl, 1, 3);
    wave_until(&mut isl, 2, 3);
    on_stage_mut(&mut isl, 2, |w, r| crate::world::castle_fallen(w, Some(r)));
    start_wave(&mut isl, 2);
    assert_eq!(
        bosses_in(&isl.stages[2]).len(),
        1,
        "unbesiegter Boss kommt nach dem Burgfall nicht wieder"
    );
    let cave = bosses_in(&isl.stages[1])[0];
    isl.stages[1].enemies[cave].hp = 0.0;
    on_stage_mut(&mut isl, 1, |w, r| remove_dead_enemies(w, Some(r)));
    assert_eq!(isl.defeated_bosses, ["trollKing"], "Merker");
    start_wave(&mut isl, 1);
    on_stage_mut(&mut isl, 1, |w, r| crate::world::castle_fallen(w, Some(r)));
    start_wave(&mut isl, 1);
    start_wave(&mut isl, 1);
    assert!(bosses_in(&isl.stages[1]).is_empty(), "besiegter Boss kehrt zurück");
}

// (g) bossSpawned und bossDefeated je genau einmal (kein `kill` für den Boss).
#[test]
fn boss_ereignisse_genau_einmal() {
    let mut isl = boss_island(2);
    wave_until(&mut isl, 1, 5);
    let b = bosses_in(&isl.stages[1]);
    assert!(!b.is_empty(), "kein Boss in Stufe 1");
    isl.stages[1].enemies[b[0]].hp = 0.0;
    on_stage_mut(&mut isl, 1, |w, r| remove_dead_enemies(w, Some(r)));
    start_wave(&mut isl, 1);
    let w = &isl.stages[1];
    let (s, d, k) = (
        count_events(w, "bossSpawned"),
        count_events(w, "bossDefeated"),
        count_events(w, "kill"),
    );
    assert_eq!((s, d, k), (1, 1, 0), "bossSpawned, bossDefeated, kill");
    for ev in w.events.iter().filter(|e| e["type"] == "bossDefeated") {
        assert_eq!(ev["boss"], "trollKing", "Ereignis {ev:?}");
    }
}

// (h) Gleicher Seed, gleicher Verlauf (zwei Spieler, Boss kämpft, beschwört und fällt).
#[test]
fn boss_deterministisch() {
    let trace = || {
        let mut isl = boss_island(2);
        wave_until(&mut isl, 0, 5);
        step_for(&mut isl.stages[0], 15.0);
        let w = &isl.stages[0];
        let mut out = w.enemies.len().to_string();
        for e in &w.enemies {
            out += &format!(" {}:{:.3}:{:.0}", e.kind, e.x, e.hp);
        }
        out
    };
    assert_eq!(trace(), trace(), "Verlauf weicht ab");
}

// (i) Jeder Boss hat einen gültigen Bezug und Werte; Wald, Höhle und Mine haben einen Miniboss; Fehler beim Laden.
#[test]
fn boss_daten() {
    let d = load_all(&Source::Embedded).expect("eingebettete Daten");
    for depth in [0, 1, 2] {
        assert!(mini_boss(&d.bosses, depth).is_some(), "Stufe {depth} ohne Miniboss");
    }
    for b in &d.bosses.bosses {
        assert!(
            b.hp_factor == 8.0 && b.damage_factor == 2.0 && b.reward.gold == 14 && b.reward.material == 50,
            "{}: Werte weichen von bosse.md § 1 ab: {b:?}",
            b.id
        );
    }
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../data/bosses.json");
    let raw = std::fs::read_to_string(path).expect("data/bosses.json");
    let depths: Vec<i64> = d.biomes.iter().map(|b| b.depth).collect();
    let got = k3c_data::bosses::load(&raw, &d.enemies, &depths).expect("eingebettete Daten");
    assert_eq!(got.bosses.len(), d.bosses.bosses.len());
    let broken = [
        (r#""base": "goblin""#, r#""base": "nope""#),
        (r#""wave": 5"#, r#""wave": 0"#),
        (r#""depth": 0"#, r#""depth": 9"#),
        (r#""gold": 14"#, r#""gold": -1"#),
        (r#""enemy": "goblin""#, r#""enemy": "nope""#),
    ];
    for (bad, repl) in broken {
        let raw_bad = raw.replacen(bad, repl, 1);
        assert_ne!(raw_bad, raw, "Testdaten: {bad} nicht gefunden");
        assert!(
            k3c_data::bosses::load(&raw_bad, &d.enemies, &depths).is_err(),
            "{bad}: ungültige Daten geladen"
        );
    }
}

// Ohne Insel (Golden-Läufe, Kampagne) gibt es keine Bosse.
#[test]
fn boss_nur_auf_inseln() {
    let (mut w, _) = boss_stage(0, 1);
    w.wave = 6;
    trigger_boss_with(&mut w, &boss_values(), None);
    assert!(bosses_in(&w).is_empty(), "Boss ohne Insel");
}
