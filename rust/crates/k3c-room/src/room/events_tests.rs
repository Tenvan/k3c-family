//! Test aus `events_test.go` (F4/AC-01, B-140/AC-01): Ein Ereignis einer Stufe erreicht nur das Gerät in dieser Stufe.
use k3c_sim::state::Coin;

use super::testutil::{create_room, join_room, manager, move_to_exit, rm, ticks};

fn count(events: &[(i64, i64, Vec<serde_json::Value>)], typ: &str) -> (usize, Option<serde_json::Value>) {
    let Some((_, _, evs)) = events.last() else {
        return (0, None);
    };
    let hits: Vec<&serde_json::Value> = evs.iter().filter(|e| e["type"] == typ).collect();
    (hits.len(), hits.last().map(|e| (*e).clone()))
}

// F4/AC-01 (B-140/AC-01): Ein Ereignis aus Stufe 1 erreicht nur das Gerät in Stufe 1, mit `stage` 1 und genau einmal.
#[test]
fn events_nur_im_zustand_der_betroffenen_stufe() {
    let (mut m, _) = manager();
    let (code, mut x) = create_room(&mut m, "xbox", 1, "ereignis", 0, &[0]);
    let mut h = join_room(&mut m, "handy", 2, &code, &[0]);
    move_to_exit(rm(&mut m, &code), 0);
    ticks(&mut m, &code, 70);
    let r = rm(&mut m, &code);
    assert_eq!((r.isl.stage_of(0), r.isl.stage_of(1)), (Some(1), Some(0)));
    let cave = &mut r.isl.stages[1];
    let Some(monarch) = cave.players.iter_mut().find(|p| p.index == 0) else {
        panic!("kein Monarch 0 in Stufe 1")
    };
    monarch.gold = 0;
    let mx = monarch.x;
    cave.coins.push(Coin {
        id: 1 << 30,
        x: mx,
        ..Default::default()
    });
    let (xs, hs) = (x.events().len(), h.events().len());
    ticks(&mut m, &code, 1);
    let (xe, he) = (x.events(), h.events());
    assert_eq!(
        (xe.len(), he.len()),
        (xs + 1, hs + 1),
        "je Gerät ein Zustand je Tick erwartet"
    );
    let (n, ev) = count(&xe, "coinPickup");
    let ev = ev.unwrap_or_default();
    assert!(
        n == 1 && ev["stage"] == 1 && ev["player"] == 0,
        "Xbox (Stufe 1): {n}x coinPickup, {:?}",
        xe.last()
    );
    assert_eq!(
        count(&he, "coinPickup").0,
        0,
        "Handy (Stufe 0) bekam das Ereignis der Stufe 1: {:?}",
        he.last()
    );
    ticks(&mut m, &code, 1);
    assert_eq!(
        count(&x.events(), "coinPickup").0,
        0,
        "Ereignis im nächsten Tick wiederholt"
    );
}
