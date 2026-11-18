//! Test aus `victory_test.go`: `victory` meldet den Tick, in dem die Insel gewonnen wird, genau einmal (B-813).
use super::testutil::room;

#[test]
fn victory_gibt_den_merker_des_letzten_ticks_wieder() {
    let mut r = room("KRNZ", "stand", 1);
    assert!(!r.victory());
    r.won_now = true; // setzt `Room::tick` in dem Tick, in dem `Island::won` wahr wird
    assert!(r.victory());
}

// Room::tick (RU21.4): Sieg im Tick des Ziels, nicht im Folgetick.
#[test]
#[ignore = "wartet auf RU21"]
fn victory_einmal() {}
