//! Test aus `victory_test.go`: `victory` meldet den Tick, in dem die Insel gewonnen wird, genau einmal (B-813).
use super::Options;
use super::save_testutil::create_with;
use super::testutil::{einzel_manager, rm, room, ticks};

#[test]
fn victory_gibt_den_merker_des_letzten_ticks_wieder() {
    let mut r = room("KRNZ", "stand", 1);
    assert!(!r.victory());
    r.won_now = true; // setzt `Room::tick` in dem Tick, in dem `Island::won` wahr wird
    assert!(r.victory());
}

// Room::tick: Sieg im Tick des Ziels, nicht im Folgetick (Ziel `gold`).
#[test]
fn victory_einmal() {
    let (mut m, _dir) = einzel_manager("sieg"); // Welt mit einer Insel
    let opts = Options {
        goal: "gold".into(),
        world: "einzel".into(), // Welt mit einer Insel: Sieg gilt nur auf der letzten
        ..Options::default()
    };
    let (code, _xi) = create_with(&mut m, "x", 1, "stand", true, opts);
    ticks(&mut m, &code, 1);
    assert!(!rm(&mut m, &code).victory(), "Sieg ohne erreichtes Ziel");
    rm(&mut m, &code).isl.gold_collected = 1 << 20;
    ticks(&mut m, &code, 1);
    assert!(rm(&mut m, &code).victory(), "Sieg im Tick des Ziels nicht gemeldet");
    ticks(&mut m, &code, 1);
    assert!(!rm(&mut m, &code).victory(), "Sieg im Folgetick erneut gemeldet");
}
