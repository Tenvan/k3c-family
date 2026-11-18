//! Tests der Lava (Go: `world_lava_test.go`; W2.2, B-115, Beschluss Q28): Lava im Eisenstollen schadet Spielern, Truppen und
//! Bauern, nicht Gegnern. Die Go-Tests bauen eine Kampagnen-Stufe (`create_world`: Plätze, Ruinen RU18) und rechnen mit
//! Spielern und Schaden (`add_player`, `apply_damage` über `troop_damage`, RU18; `probe`, `was_hit`); sie tragen `ignore`.
//! `own_*` prüft `on_lava` und die Auswahl und den Takt von `step_lava` an einer Hand-Welt ohne Figur auf Lava.

use serde_json::Value;

use super::base::base;
use super::lava::{on_lava, step_lava};
use super::testutil::*;
use super::{Options, add_player, step};
use crate::enemies::testutil::{probe, was_hit};
use crate::state::{PlayerCommand, Troop, World};
use crate::unit::jobs::make_archer;
use crate::unit::spawn_vagrant;

/// Go: `lavaWorld`: ein Eisenstollen ohne Truppen und Camps mit der Mitte des ersten Lava-Streifens.
fn lava_world() -> (World, f64) {
    for seed in 0..50 {
        let mut w = new_world(3, &format!("lava-{seed}"), Options::default());
        w.troops.clear();
        w.camps.clear();
        w.cycle_speed = 1e-6;
        if let Some(c) = w.level.chunks.iter().find(|c| c.kind == "lava") {
            let mid = c.start_units + w.level.chunk_width_units / 2.0;
            return (w, mid);
        }
    }
    panic!("kein Lava-Chunk in 50 Seeds");
}

fn hit_ids(w: &World, target: &str) -> Vec<i64> {
    w.events
        .iter()
        .filter(|e| {
            e.get("type").and_then(Value::as_str) == Some("hit")
                && e.get("target").and_then(Value::as_str) == Some(target)
                && e.get("damage").and_then(Value::as_f64).unwrap_or(0.0) > 0.0
        })
        .filter_map(|e| e.get("id").and_then(Value::as_i64))
        .collect()
}

#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn lava_schadet_figuren() {
    let (mut w, x) = lava_world();
    let (dps, width) = (w.biome.lava.damage_per_second, w.biome.lava.width_units);
    let (p0, p1) = (add_player(&mut w), add_player(&mut w));
    w.players[p0].x = x;
    w.players[p1].x = x + width; // p1 steht neben dem Streifen
    let peasant = spawn_vagrant(&mut w, x, x);
    {
        let t = &mut w.troops[peasant];
        t.kind = "peasant".into();
        t.hp = 1000.0;
        t.max_hp = 1000.0;
        t.target_x = x;
    }
    let archer = spawn_vagrant(&mut w, x, x);
    make_archer(&mut w, archer);
    w.troops[archer].hp = 1000.0;
    w.troops[archer].anchor_x = x;
    probe(&mut w, &[p0, p1]);
    let (id0, id1) = (w.players[p0].index, w.players[p1].index);
    let (mut hits0, mut hits1) = (0, 0);
    for _ in 0..60 {
        // 2 s: zwei Lava-Treffer
        w.players[p0].x = x;
        w.players[p1].x = x + width;
        w.troops[peasant].x = x;
        w.troops[archer].x = x;
        step(&mut w, &[PlayerCommand::default(), PlayerCommand::default()], None, DT);
        for id in hit_ids(&w, "player") {
            hits0 += i32::from(id == id0);
            hits1 += i32::from(id == id1);
        }
    }
    let want = 2.0 * dps;
    // Am Spieler wirkt Lava wie ein Treffer: eine Münze je Sekunde (kampf.md § 5), daneben nichts.
    assert!(
        hits0 == 2 && hits1 == 0 && !was_hit(&w, p1),
        "Spieler auf Lava {hits0} Münz-Treffer (erwartet 2), daneben {hits1}"
    );
    let (lost_peasant, lost_archer) = (1000.0 - w.troops[peasant].hp, 1000.0 - w.troops[archer].hp);
    assert!(
        (lost_peasant - want).abs() <= 0.01 && (lost_archer - want).abs() <= 0.01,
        "Bauer -{lost_peasant}, Schütze -{lost_archer}, erwartet je {want}"
    );
}

// Gegner nehmen keinen Lava-Schaden (Startwert, BR1 bestätigt).
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn lava_nicht_fuer_gegner() {
    let (mut w, x) = lava_world();
    let ei = spawn(&mut w, "goblin", x);
    w.enemies[ei].stun = f64::INFINITY;
    let hp = w.enemies[ei].hp;
    run_until(&mut w, 1.0, |_| false);
    assert_eq!(w.enemies[ei].hp, hp, "Gegner auf Lava -{}", hp - w.enemies[ei].hp);
}

// Lava liegt nie im Hub, am Eingang oder am Rand (500 Seeds).
#[test]
fn lava_nicht_im_hub_oder_am_rand() {
    let data = &base().expect("Daten").data;
    let b = data.biomes.iter().find(|b| b.depth == 3).expect("Biom Tiefe 3");
    for seed in 0..500 {
        let w = new_world(3, &seed.to_string(), Options::default());
        for c in w.level.chunks.iter().filter(|c| c.kind == "lava") {
            let mid = c.start_units + w.level.chunk_width_units / 2.0;
            assert!(
                (mid - w.hub_x).abs() >= b.hub_width_units / 2.0
                    && c.index != 0
                    && c.index != w.level.chunks.len() as i64 - 1,
                "Seed {seed}: Lava-Chunk {} bei {mid} (Hub {})",
                c.index,
                w.hub_x
            );
        }
    }
}

/// Eine Hand-Welt mit einem Lava-Chunk (Breite 4 Units, Chunk 10 Units breit ab 100): Mitte bei 105.
fn hand_lava_world() -> World {
    let mut w = bare_world();
    w.biome.lava.damage_per_second = 5.0;
    w.biome.lava.width_units = 4.0;
    w.level.chunk_width_units = 10.0;
    for (index, kind, start) in [(0, "forest", 90.0), (1, "lava", 100.0), (2, "forest", 110.0)] {
        w.level.chunks.push(k3c_level::types::Chunk {
            index,
            kind: kind.into(),
            start_units: start,
        });
    }
    w
}

#[test]
fn own_on_lava_trifft_den_streifen_um_die_mitte() {
    let w = hand_lava_world();
    assert!(on_lava(&w, 105.0));
    assert!(on_lava(&w, 107.0), "Rand des Streifens zählt");
    assert!(on_lava(&w, 103.0));
    assert!(!on_lava(&w, 107.5));
    assert!(!on_lava(&w, 95.0), "Chunk ohne Lava");
    assert!(!on_lava(&w, 0.0));
}

#[test]
fn own_step_lava_ohne_schaden_wert_tut_nichts() {
    let mut w = hand_lava_world();
    w.biome.lava.damage_per_second = 0.0;
    w.lava_in = 0.25;
    step_lava(&mut w, 1.0);
    assert_eq!(w.lava_in, 0.25, "ohne Lava-Schaden läuft kein Takt");
}

#[test]
fn own_step_lava_zaehlt_je_sekunde() {
    let mut w = hand_lava_world();
    w.lava_in = 1.0;
    step_lava(&mut w, 0.25);
    assert_eq!(w.lava_in, 0.75);
    step_lava(&mut w, 0.5);
    assert_eq!(w.lava_in, 0.25);
    step_lava(&mut w, 0.25); // fällig: eine Sekunde dazu, niemand steht auf Lava
    assert_eq!(w.lava_in, 1.0);
}

#[test]
fn own_step_lava_trifft_nur_gekrönte_gesteuerte_spieler_und_lebende_truppen() {
    let mut w = hand_lava_world();
    let id = w.new_id();
    w.players.push(crate::state::Player {
        id,
        x: 105.0,
        free: true, // nicht gesteuert
        gold: 7,
        ..crate::state::Player::default()
    });
    let id = w.new_id();
    w.players.push(crate::state::Player {
        id,
        x: 105.0,
        crown: Some(crate::state::shared::Crown::default()), // Krone gefallen
        gold: 7,
        ..crate::state::Player::default()
    });
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: "peasant".into(),
        x: 105.0,
        hp: 0.0, // tot
        ..Troop::default()
    });
    w.lava_in = 0.0;
    step_lava(&mut w, 1.0 / 30.0);
    assert!(w.players.iter().all(|p| p.gold == 7), "kein Spieler getroffen");
    assert_eq!(w.troops[0].hp, 0.0);
    assert!(w.events.is_empty(), "keine Ereignisse: {:?}", w.events);
}
