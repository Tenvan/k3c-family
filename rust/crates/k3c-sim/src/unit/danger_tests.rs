//! B-1253: Gefahr für Handwerker.

use crate::enemies::testutil::*;

/// B-1253: Nach Sonnenaufgang arbeitet der Handwerker, obwohl ein Wellen-Gegner fern noch lebt; nachts und bei
/// Feindkontakt gilt Gefahr (mit 2 Spielern).
#[test]
fn handwerker_arbeiten_ab_sonnenaufgang() {
    use crate::state::common::{danger_for, is_dangerous};
    let mut w = quiet_world();
    let _players = [add_player(&mut w), add_player(&mut w)];
    let x = w.hub_x;
    let worker = add_peasant(&mut w, x, "");
    w.troops[worker].hammer = true;
    let enemy = spawn(&mut w, "wolf", x + 60.0);
    w.cycle.phase = "day".into();
    assert!(is_dangerous(&w) && !danger_for(&w, worker), "ferner Gegner am Tag");
    w.enemies[enemy].x = w.troops[worker].x + 1.0;
    assert!(danger_for(&w, worker), "Feindkontakt");
    w.enemies.clear();
    w.cycle.phase = "night".into();
    assert!(danger_for(&w, worker), "Nacht");
}
