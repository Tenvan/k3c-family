//! Takt-Vergleich mit Go (RU10.5, AC-01): Die Datei `testdata/rust/insel/insel-1000.json` hält den Zustand einer Insel nach
//! 1000 Schritten aus Go (`engine/sim/rustref_island_test.go`, `K3C_RUSTREF=write`): Seed `rustref`, zwei Stufen (Tiefe 0
//! und 1), zwei Spieler in Stufe 0, keine Kommandos, `dt = 1/30`, Zykluszeit 60. Der Vergleich mit Rust braucht
//! `create_world_with` und `step` (RU14) und trägt `ignore`; die Form der Datei prüft ein Test, der schon läuft.
//!
//! Der Vergleich nutzt `k3c_det::golden::diff` (Zahlen bitgleich). Die Abgleich-Regel der Sim (Ereignisse in gleicher
//! Reihenfolge, Ganzzahlen gleich, Fließkomma mit Toleranz 1e-4, `docs/standards/rust-port.md` § 1) setzen RU14 und RU22,
//! wenn der Test läuft.
use k3c_det::golden;
use serde_json::{Value, json};

use super::testutil::*;
use super::{CreateOptions, Island, add_island_player, create_island, step_island};
use crate::state::World;

/// Die Referenz aus Go.
fn reference() -> Value {
    golden::read(golden::testdata().join("rust/insel/insel-1000.json"))
}

/// Go schreibt leere `nil`-Slices als `null` (`engine/sim/rustref_island_test.go:63,66`): leer wird `null`, sonst ein Array.
fn null_if_empty<T: serde::Serialize>(v: &[T]) -> Value {
    if v.is_empty() { Value::Null } else { json!(v) }
}

#[test]
fn null_if_empty_bildet_leer_auf_null_ab() {
    assert_eq!(null_if_empty::<String>(&[]), Value::Null);
    assert_eq!(null_if_empty(&["a".to_string()]), json!(["a"]));
}

/// Die Felder der Insel wie in der Datei (`island`): was Go in `rustref_island_test.go` schreibt.
fn island_fields(isl: &Island) -> Value {
    json!({
        "stock": golden::tree(&isl.stock),
        "skillPool": isl.skill_pool,
        "chestsOpened": isl.chests_opened,
        "defeatedBosses": null_if_empty(&isl.defeated_bosses),
        "endbossDefeated": isl.endboss_defeated,
        "goldCollected": isl.gold_collected,
        "won": isl.won,
        "number": isl.number,
        "gateOpen": isl.gate_open,
        "switchReady": isl.switch_ready,
        "completed": null_if_empty(&isl.completed),
        "chosen": isl.chosen,
        "over": isl.over,
        "merchantVisits": isl.merchant_visits,
    })
}

/// Go: `TestRustrefInsel`: 1000 Schritte einer Insel. Aufbau und Schritt sind RU14; der Test läuft ab dann.
#[test]
fn insel_nach_1000_schritten_wie_go() {
    let want = reference();
    let data = test_data();
    let depths: Vec<i64> = serde_json::from_value(want["depths"].clone()).expect("Tiefen");
    let speed = want["cycleSpeed"].as_f64().expect("cycleSpeed");
    let dt = want["dt"].as_f64().expect("dt");
    let world = standard_world(&data);
    let seed = want["seed"].as_str().expect("seed");
    let mut isl = create_island(&data, &world, seed, &depths, speed, CreateOptions::default()).expect("Insel");
    for _ in 0..want["players"].as_i64().expect("players") {
        add_island_player(&mut isl, 0);
    }
    for _ in 0..want["steps"].as_i64().expect("steps") {
        step_island(&mut isl, &[], &data.difficulty, dt);
    }
    let d = golden::diff("island", &want["island"], &island_fields(&isl));
    assert!(d.is_empty(), "{d}");
    assert_eq!(isl.stages.len(), want["stages"].as_array().expect("stages").len());
    for (i, w) in isl.stages.iter().enumerate() {
        let d = golden::diff(&format!("stages[{i}]"), &want["stages"][i], &golden::tree(w));
        assert!(d.is_empty(), "{d}");
    }
}

/// Die Datei ist eingecheckt, hat die Form aus Go, und jede Stufe läuft in Rust als `World` im Kreis (gelesen und wieder
/// geschrieben als derselbe JSON-Baum).
#[test]
fn own_referenz_aus_go_ist_da_und_laedt() {
    let want = reference();
    assert_eq!(want["seed"], "rustref");
    assert_eq!(want["steps"], 1000);
    assert_eq!(want["players"], 2);
    assert_eq!(want["depths"], json!([0, 1]));
    let stages = want["stages"].as_array().expect("Stufen");
    assert_eq!(stages.len(), 2, "zwei Stufen");
    let secs = want["steps"].as_f64().expect("steps") * want["dt"].as_f64().expect("dt");
    for (i, s) in stages.iter().enumerate() {
        let t = s["time"].as_f64().expect("time");
        assert!((t - secs).abs() < 1e-6, "Stufe {i}: Zeit {t}, erwartet {secs}");
        let w: World = serde_json::from_value(s.clone()).unwrap_or_else(|e| panic!("Stufe {i}: {e}"));
        let d = golden::diff(&format!("stages[{i}]"), s, &golden::tree(&w));
        assert!(d.is_empty(), "{d}");
    }
    assert_eq!(
        want["island"]["stock"]["wood"], stages[0]["stock"]["wood"],
        "der Vorrat liegt in jeder Stufe"
    );
    assert_eq!(want["island"]["over"], false);
}
