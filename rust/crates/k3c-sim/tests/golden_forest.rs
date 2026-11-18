//! AC-02 (Sprint RU22), Teil Wald: Die sieben Wald-Golden rechnet die Rust-Sim mit gleichem Verhalten nach (Entscheidung
//! 012: Ereignisse, Reihenfolge und Ganzzahlen exakt, Fließkomma 1e-4 relativ). Golden werden nie geschrieben.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

mod common;

use common::golden_run::assert_equal_behavior;

#[test]
fn sim_forest_ohne_spieler() {
    assert_equal_behavior("forest-ohne-spieler");
}

#[test]
fn sim_forest_tag() {
    assert_equal_behavior("forest-tag");
}

#[test]
fn sim_forest_nacht() {
    assert_equal_behavior("forest-nacht");
}

#[test]
fn sim_forest_aufbau() {
    assert_equal_behavior("forest-aufbau");
}

#[test]
fn sim_forest_raub() {
    assert_equal_behavior("forest-raub");
}

#[test]
fn sim_forest_sturm() {
    assert_equal_behavior("forest-sturm");
}

#[test]
fn sim_forest_monarch() {
    assert_equal_behavior("forest-monarch");
}
