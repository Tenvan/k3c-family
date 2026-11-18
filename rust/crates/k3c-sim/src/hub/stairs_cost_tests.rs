//! Tests zu `stairs_cost_test.go` (B-631/AC-02): Treppe runter kostet auf Tiefe d 100 Material der Stufe d, Treppe hoch 100
//! Material der Stufe d-1, je plus 10 Gold (Münzskala, BS3.1) (data/buildings.json › costByDepth, `site_cost`).

use k3c_data::buildings::Cost;
use k3c_data::hub::Stock;

use super::level::{add_cost, site_cost};
use super::testutil::site_of_kind;
use crate::economy::view::material_of;
use crate::state::World;
use crate::world::Options;
use crate::world::testutil::{new_world, pay_at, quiet_world, run_until};

fn mat(wood: i64, stone: i64, copper: i64, iron: i64) -> Cost {
    Cost {
        wood,
        stone,
        copper,
        iron,
        ..Cost::default()
    }
}

#[test]
fn treppen_kosten_je_tiefe() {
    let material = [
        mat(100, 0, 0, 0),
        mat(0, 100, 0, 0),
        mat(0, 0, 100, 0),
        mat(0, 0, 0, 100),
    ];
    let gold = Cost {
        gold: 10,
        ..Cost::default()
    };
    for d in 0..=4_i64 {
        let w = new_world(d, "b631", Options::default());
        if d < 4 {
            let want = add_cost(&material[d as usize], &gold);
            let s = site_of_kind(&w, "stairsDown").expect("stairsDown");
            let got = site_cost(&w, None, s);
            assert_eq!(got, want, "Tiefe {d}: Treppe runter kostet {got:?}, erwartet {want:?}");
        }
        if d > 0 {
            let want = add_cost(&material[(d - 1) as usize], &gold);
            let s = site_of_kind(&w, "stairsUp").expect("stairsUp");
            let got = site_cost(&w, None, s);
            assert_eq!(got, want, "Tiefe {d}: Treppe hoch kostet {got:?}, erwartet {want:?}");
        }
    }
}

/// Höhle: Zwei Spieler bezahlen die Treppe hoch, der Bau zieht 100 Holz aus dem Vorrat (kein Stein). Der Test braucht
/// `step` (Go: `buildNew`).
#[test]
#[ignore = "wartet auf RU18 (step: step_passives RU18.5, merchant_dawn RU18.4)"]
fn treppe_hoch_in_der_hoehle_kostet_holz() {
    let mut w = new_world(1, "b631", Options::default());
    w.troops.clear();
    w.camps.clear();
    w.pickups.clear();
    w.cycle_speed = 1e-6;
    let hub = w.hub_x;
    let ti = crate::unit::spawn_vagrant(&mut w, hub, hub);
    let hp = crate::world::vals(&w).troops.get("peasant").map_or(0.0, |t| t.hp);
    let t = &mut w.troops[ti];
    (t.kind, t.hp, t.max_hp, t.hammer) = ("peasant".into(), hp, hp, true);
    let p0 = crate::world::add_player(&mut w);
    let p1 = crate::world::add_player(&mut w);
    w.hub_level = 2;
    let id = site_of_kind(&w, "stairsUp").expect("stairsUp").id;
    // Go: `buildNew(t, w, players, stairs)`: Gold gleich verteilt, Material genau im Vorrat, ein Bauer baut
    let cost = site_cost(&w, None, w.site(id).expect("Platz"));
    w.stock = material_of(&cost);
    let share = cost.gold / 2;
    w.players[p0].gold = share + (cost.gold - share * 2);
    w.players[p1].gold = share;
    let x = w.site(id).expect("Platz").x;
    let interval = crate::world::vals(&w).economy.pay_interval_seconds;
    let seconds = 1.2 * cost.gold as f64 * interval + 5.0;
    let waiting = |w: &World| w.site(id).is_some_and(|s| s.state == "waitingWorker");
    assert!(
        pay_at(&mut w, &[p0, p1], x, seconds, waiting),
        "stairsUp: nicht bezahlt"
    );
    for t in w.troops.iter_mut().filter(|t| t.kind == "peasant") {
        t.x = x;
    }
    run_until(&mut w, 60.0, |w| w.site(id).is_some_and(|s| s.state == "built"));
    let s = w.site(id).expect("Platz");
    assert!(
        s.state == "built" && w.stock == Stock::default(),
        "Treppe hoch: {}, Vorrat danach {:?}",
        s.state,
        w.stock
    );
}

/// Ohne Eintrag für die Tiefe gilt `cost` der Art.
#[test]
fn site_cost_rueckfall_auf_cost() {
    let w = quiet_world();
    let s = site_of_kind(&w, "workshop").expect("workshop");
    let want = crate::world::vals(&w).buildings["workshop"].cost.clone();
    assert_eq!(site_cost(&w, None, s), want);
}
