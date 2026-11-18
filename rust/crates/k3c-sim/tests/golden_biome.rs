//! AC-02 (Sprint RU22), Teil Biome: Die Golden von Höhle, Mine, Kristall und Ironhold rechnet die Rust-Sim mit gleichem
//! Verhalten nach (Entscheidung 012). Golden werden nie geschrieben.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

mod common;

use common::golden_run::assert_equal_behavior;

#[test]
fn sim_cave_aggression() {
    assert_equal_behavior("cave-aggression");
}

#[test]
fn sim_cave_aufbau() {
    assert_equal_behavior("cave-aufbau");
}

#[test]
fn sim_cave_belagerung() {
    assert_equal_behavior("cave-belagerung");
}

#[test]
fn sim_mine_welle() {
    assert_equal_behavior("mine-welle");
}

#[test]
fn sim_crystal_welle() {
    assert_equal_behavior("crystal-welle");
}

#[test]
fn sim_ironhold_welle() {
    assert_equal_behavior("ironhold-welle");
}
