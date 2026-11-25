//! Tests zu `bot_clear.rs` und dem Schiff aus `bot_material.rs` (Port von `engine/balance/bot_clear_test.go`).

use k3c_data::hub::Stock;
use k3c_sim::island::add_island_player;
use k3c_sim::state::Troop;
use k3c_sim::world::lines::{site_blockers, site_needs_clearing};
use k3c_sim::world::ship::ship_next_cost;

use crate::bot_clear::{blocked_site, clear_node};
use crate::bot_material::{ship_node, ship_x};
use crate::bot_wirtschaft::FIGHTER_WANT;
use crate::testutil::create;

/// BS8.1/AC-01 (B-636): Der Bot räumt den Freiraum der Baumschule, wenn kein Mauer- oder Turm-Platz mehr gesperrt ist, und
/// kauft sie danach über die Bauliste.
#[test]
fn bot_rodet_baumschule() {
    let mut isl = create("1", &[0]);
    let pi = add_island_player(&mut isl, 0);
    let w = &mut isl.stages[0];
    w.players[pi].gold = 100;
    w.hub_level = 5;
    for s in &mut w.sites {
        if s.kind == "wall" || s.kind == "tower" {
            s.state = "built".into(); // nur die Baumschule bleibt gesperrt
        }
    }
    let w = &isl.stages[0];
    let nursery = w
        .sites
        .iter()
        .find(|s| s.kind == "nursery" && s.state == "unpaid" && !site_blockers(w, s).is_empty())
        .expect("kein gesperrter Baumschul-Platz im Wald-Level");
    let got = blocked_site(w).expect("blockedSite: erwartet eine Baumschule");
    assert_eq!(got.kind, "nursery", "blockedSite {got:?}, erwartet eine Baumschule");
    let n = clear_node(w, &w.players[pi]);
    assert!(
        n.is_some() && site_needs_clearing(w, nursery),
        "clearNode {n:?}, erst roden {}",
        site_needs_clearing(w, nursery)
    );
}

/// BS9.1/AC-01 (B-637): Der Bot zahlt das Schiff erst mit Hub 2 und genug Kämpfern und nur, wenn das Material des Abschnitts
/// im Vorrat liegt.
#[test]
fn bot_baut_schiff() {
    let mut isl = create("1", &[0]);
    let pi = add_island_player(&mut isl, 0);
    isl.stages[0].players[pi].gold = 500;
    let ship = isl.stages[0]
        .sites
        .iter()
        .rfind(|s| s.kind == "ship")
        .expect("kein Schiffs-Platz in der Standard-Welt")
        .clone();
    let w = &isl.stages[0];
    let cost = ship_next_cost(w, w.island_ref_view().as_ref(), &ship).expect("Kiel hat einen Preis");
    assert!(cost.wood != 0, "Kiel kostet Holz: {cost:?}");
    isl.stock = Stock {
        wood: cost.wood,
        ..Stock::default()
    };
    isl.sync_stock(); // Go: alle Stufen zeigen auf denselben Vorrat
    let p = isl.stages[0].players[pi].clone();
    assert!(
        ship_x(&isl, &isl.stages[0], &p, 0).is_none(),
        "Schiff vor Hub 2 und Heer gezahlt"
    );
    isl.stages[0].hub_level = 2;
    for _ in 0..FIGHTER_WANT {
        isl.stages[0].troops.push(Troop {
            kind: "archer".into(),
            ..Troop::default()
        });
    }
    let x = ship_x(&isl, &isl.stages[0], &p, 0);
    assert_eq!(
        x,
        Some(ship.x),
        "Hub 2, Heer und Holz da: shipX {x:?}, erwartet den Schiffs-Platz {}",
        ship.x
    );
    isl.stock = Stock::default();
    isl.sync_stock();
    assert!(ship_x(&isl, &isl.stages[0], &p, 0).is_none(), "ohne Material gezahlt");
    assert!(
        ship_node(&isl, &isl.stages[0], &p).is_some(),
        "ohne Material sucht der Bot keinen Knoten"
    );
}
