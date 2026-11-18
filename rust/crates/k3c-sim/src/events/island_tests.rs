//! Go: `events_island_test.go` (B-139/AC-04): Auf der Insel entsteht ein Ereignis nur in seiner Stufe und trägt deren Index
//! als `stage`.

use crate::island::testutil::{must_island, test_data};
use crate::island::{add_island_player, step_island};
use crate::state::{Coin, PlayerCommand};

const DT: f64 = 1.0 / 30.0;

#[test]
fn ereignis_traegt_die_stufe() {
    let data = test_data();
    let mut isl = must_island(&data, "ereignis-insel", &[0, 1]);
    let a = add_island_player(&mut isl, 0);
    let b = add_island_player(&mut isl, 1);
    isl.stages[0].players[a].gold = 0;
    isl.stages[1].players[b].gold = 0;
    let (bx, bindex) = (isl.stages[1].players[b].x, isl.stages[1].players[b].index);
    let cave = &mut isl.stages[1];
    let id = cave.new_id();
    cave.coins.push(Coin {
        id,
        x: bx,
        ..Default::default()
    });
    step_island(
        &mut isl,
        &[PlayerCommand::default(), PlayerCommand::default()],
        &data.difficulty,
        DT,
    );
    let in_cave = isl.stages[1].events.iter().find(|e| e["type"] == "coinPickup");
    let ev = in_cave.expect("Ereignis in Stufe 1");
    assert!(
        ev["stage"] == 1 && ev["player"] == bindex,
        "Ereignis in Stufe 1: {ev:?}"
    );
    assert!(
        isl.stages[0].events.iter().all(|e| e["type"] != "coinPickup"),
        "Ereignis der Höhle auch in Stufe 0"
    );
}
