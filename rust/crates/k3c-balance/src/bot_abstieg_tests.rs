//! Tests zu `bot_abstieg.rs` (Port von `engine/balance/bot_abstieg_test.go`).

use crate::bot_abstieg::{may_descend, mini_boss_id, mini_defeated};
use crate::testutil::scenario_island;

/// BS11.2/AC-03 (B-642/AC-03): Der Bot steigt erst ab, wenn der Miniboss seiner Stufe besiegt ist, und nur mit ungeradem
/// Index, wenn mehrere Monarchen spielen. Den ganzen Abstieg bis zur tiefsten Stufe sichert der Stufen-Test
/// `task balance:fullgame STUFE=2` (Build-Tag fullgame, nicht in check:diff).
#[test]
fn bots_steigen_erst_nach_dem_miniboss() {
    let mut isl = scenario_island("k3c-dev", 2);
    assert_eq!(mini_boss_id(0), Some("goblinLeader"), "Miniboss des Waldes");
    assert!(
        !mini_defeated(&isl, 0),
        "Miniboss noch nicht besiegt, Abstieg trotzdem frei"
    );
    isl.defeated_bosses.push("goblinLeader".into());
    assert!(mini_defeated(&isl, 0), "Miniboss besiegt, Abstieg gesperrt");
    let w = &isl.stages[0];
    let (p0, p1) = (&w.players[0], &w.players[1]);
    assert!(
        !may_descend(&isl, w, p0) && may_descend(&isl, w, p1),
        "bei zwei Monarchen steigt zuerst nur der mit ungeradem Index ab: {} {}",
        p0.index,
        p1.index
    );
    assert!(
        mini_boss_id(99).is_none() && mini_defeated(&isl, 99),
        "Stufe ohne Miniboss muss frei sein"
    );
}

/// Go: `TestOhneWorldKeinAbstieg` läuft über `mustRun` (RU25.4, `run.rs`): ohne `World` im Szenario erreicht `coop2` Tiefe 0.
#[test]
#[ignore = "wartet auf RU25.4"]
fn ohne_world_kein_abstieg() {}
