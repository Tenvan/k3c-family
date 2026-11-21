//! Port von `engine/sim/world_vein_test.go` (W2.1, B-114/AC-02, AC-04): Adern liefern mit 2 Bauern die Zielrate,
//! höchstens 2 Bauern, einmal markiert. Werte aus data/economy.json › veins. Die Go-Tests rechnen eine Stufe mit `Step`
//! (`create_world`: `new_hub_site`, `step_players`: RU18) und tragen `ignore`; die `own_*`-Tests prüfen `gather_of`,
//! `is_vein`, `node_free` und `gather_vein` ohne Takt.

use std::sync::Arc;

use super::testutil::{DT, bare_world, new_world, pay_at};
use super::values_room::default_values;
use super::vein::{gather_of, gather_vein, is_vein, node_free, vein_workers};
use super::{Options, add_player, step};
use crate::enemies::testutil::run;
use crate::island::storage::{capacity, stock_get};
use crate::island::testutil::{must_island, on_stage, run_island, test_data};
use crate::state::common::node_by_id;
use crate::state::{Job, ResourceNode, Troop, World};
use crate::unit::spawn_vagrant;

/// Go: `veinWorld`: eine ruhige Höhle ohne Nacht mit der ersten Ader direkt neben der Burg und `peasants` Bauern; liefert
/// die Welt und den Index der Ader in `w.nodes`.
fn vein_world(peasants: usize) -> (World, usize) {
    let mut w = new_world(1, "adern", Options::default());
    w.troops.clear();
    w.camps.clear();
    w.pickups.clear();
    w.cycle_speed = 1e-6;
    w.aggression = None; // ohne Aggressionspool: Wellen stören die Abbau-Rate nicht
    let v = default_values();
    let Some(vi) = w.nodes.iter().position(|n| is_vein(&v, n)) else {
        panic!("keine Ader in der Höhle");
    };
    w.nodes[vi].x = w.hub_x + 1.0;
    let hp = v.troops["peasant"].hp;
    for _ in 0..peasants {
        let hub = w.hub_x;
        let ti = spawn_vagrant(&mut w, hub, hub);
        let t = &mut w.troops[ti];
        (t.kind, t.hp, t.max_hp) = ("citizen".into(), hp, hp);
        t.wild = false;
        t.hammer = true; // ECO1.4: nur Handwerker (Werkzeuggürtel) hauen an Adern
    }
    (w, vi)
}

#[test]
fn ader_zielrate_mit_zwei_bauern() {
    let minutes = 10.0;
    for (kind, v) in &default_values().economy.veins {
        let (mut w, vi) = vein_world(2);
        (w.nodes[vi].kind, w.nodes[vi].marked) = (kind.clone(), true);
        let id = w.nodes[vi].id;
        run(&mut w, minutes * 60.0 + 2.0); // 2 s für die Wege zwischen Ader und Burg
        let got = stock_get(&w.stock, &v.gatherable.resource).unwrap_or(0) as f64 / minutes;
        let want = 2.0 * v.gatherable.amount as f64 / v.gatherable.work_seconds * 60.0;
        assert!(
            got >= 0.9 * want && got <= 1.1 * want,
            "{kind}: {got:.1}/min, Zielrate {want:.1}/min"
        );
        assert!(
            node_by_id(&w, id).is_some_and(|n| n.marked),
            "{kind}: Ader weg oder nicht mehr markiert"
        );
    }
}

#[test]
fn ader_hoechstens_zwei_bauern() {
    let (mut w, vi) = vein_world(3);
    w.nodes[vi].marked = true;
    let mut most = 0;
    for _ in 0..30 * 60 {
        step(&mut w, &[], None, DT);
        most = most.max(vein_workers(&w, &w.nodes[vi]));
    }
    let limit = default_values().economy.veins[&w.nodes[vi].kind].max_workers;
    assert_eq!(most, limit, "höchstens {limit} Bauern gleichzeitig");
}

#[test]
fn ader_ruht_ohne_bauern() {
    let (mut w, vi) = vein_world(0);
    w.nodes[vi].marked = true;
    run(&mut w, 60.0);
    assert_eq!(w.stock, k3c_data::hub::Stock::default(), "ohne Bauern geliefert");
}

/// Zwei Spieler markieren eine Ader gemeinsam für ihren markCost; sie bleibt danach markiert.
#[test]
fn ader_einmal_markieren() {
    let (mut w, vi) = vein_world(0);
    let x = w.hub_x + 8.0; // außer Reichweite der Burg (Hub-Ausbau wäre das nähere Zahlziel)
    (w.nodes[vi].kind, w.nodes[vi].x) = ("copperVein".into(), x);
    let cost = default_values().economy.veins["copperVein"].gatherable.mark_cost;
    let (p0, p1) = (add_player(&mut w), add_player(&mut w));
    let gold = w.players[p0].gold + w.players[p1].gold;
    pay_at(&mut w, &[p0, p1], x, 10.0, |w| w.nodes[vi].marked);
    run(&mut w, 1.0);
    let paid = gold - w.players[p0].gold - w.players[p1].gold;
    assert!(
        w.nodes[vi].marked && paid == cost,
        "markiert {}, bezahlt {paid}, markCost {cost}",
        w.nodes[vi].marked
    );
}

/// Lager voll → der Bauer nimmt die Ader nicht an.
#[test]
fn ader_lager_voll() {
    let data = test_data();
    let mut isl = must_island(&data, "adern-voll", &[0, 1]);
    let w = &mut isl.stages[1];
    w.troops.clear();
    w.cycle_speed = 1e-6;
    let v = default_values();
    let limit = on_stage(&mut isl, &data, 1, |w, r| capacity(w, &v, Some(r))).unwrap_or(0);
    isl.stock.stone = limit;
    let w = &mut isl.stages[1];
    let Some(vi) = w.nodes.iter().position(|n| n.kind == "stoneVein") else {
        panic!("keine Steinader");
    };
    w.nodes[vi].marked = true;
    let hub = w.hub_x;
    let ti = spawn_vagrant(w, hub, hub);
    w.troops[ti].serve();
    run_island(&mut isl, &data, &[], 5.0);
    assert_eq!(
        vein_workers(&isl.stages[1], &isl.stages[1].nodes[vi]),
        0,
        "Bauer arbeitet trotz vollem Lager"
    );
}

/// Adern sind kein endlicher Startvorrat: nicht in gatherables, Level-Objekte bleiben beim Ader-Abbau gleich.
#[test]
fn ader_nicht_im_startvorrat() {
    own_adern_nicht_in_gatherables();
    let (mut w, vi) = vein_world(2);
    w.nodes[vi].marked = true;
    let before = w.nodes.len();
    run(&mut w, 120.0);
    assert_eq!(w.nodes.len(), before, "Knoten");
}

/// Teil von `TestAderNichtImStartvorrat` ohne Takt.
#[test]
fn own_adern_nicht_in_gatherables() {
    let v = default_values();
    assert!(!v.economy.veins.is_empty(), "keine Adern in economy.json");
    for kind in v.economy.veins.keys() {
        assert!(!v.economy.gatherables.contains_key(kind), "{kind} steht in gatherables");
    }
}

fn node(kind: &str, id: i64) -> ResourceNode {
    ResourceNode {
        id,
        kind: kind.into(),
        ..ResourceNode::default()
    }
}

fn gatherer(w: &mut World, node_id: i64) {
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: "citizen".into(),
        hp: 1.0,
        job: Some(Job {
            r#type: "gather".into(),
            node_id,
            ..Job::default()
        }),
        ..Troop::default()
    });
}

/// `gather_of` kennt Adern (über ihr `Gatherable`) und endliche Ressourcen, `is_vein` nur Adern.
#[test]
fn own_gather_of_und_is_vein() {
    let v = default_values();
    for (kind, vein) in &v.economy.veins {
        assert_eq!(gather_of(&v, kind), Some(&vein.gatherable), "{kind}");
        assert!(is_vein(&v, &node(kind, 1)));
    }
    for (kind, g) in &v.economy.gatherables {
        assert_eq!(gather_of(&v, kind), Some(g), "{kind}");
        assert!(!is_vein(&v, &node(kind, 1)));
    }
    assert_eq!(gather_of(&v, "unbekannt"), None);
}

/// `node_free`: eine Ader nimmt bis `max_workers` Bauern, eine endliche Ressource einen (`worker_id` lebt).
#[test]
fn own_node_free_ader_bis_max_workers() {
    let mut w = bare_world();
    let v = Arc::clone(&w.vals.clone().unwrap_or_else(default_values));
    let (kind, vein) = v.economy.veins.iter().next().unwrap_or_else(|| panic!("keine Ader"));
    let n = node(kind, 77);
    for _ in 0..vein.max_workers {
        assert!(node_free(&w, &n), "Ader mit {} Bauern", vein_workers(&w, &n));
        gatherer(&mut w, 77);
    }
    assert!(!node_free(&w, &n), "Ader voll bei {} Bauern", vein.max_workers);
    let (tree, _) = v
        .economy
        .gatherables
        .iter()
        .next()
        .unwrap_or_else(|| panic!("keine Ressource"));
    let mut t = node(tree, 78);
    assert!(node_free(&w, &t), "freie Ressource");
    t.worker_id = Some(w.troops[0].id);
    assert!(!node_free(&w, &t), "Ressource mit lebendem Bauern");
}

/// `gather_vein`: Fortschritt je Bauer, nach `work_seconds` ein Trage-Auftrag mit `amount`.
#[test]
fn own_gather_vein_fortschritt_und_tragen() {
    let v = default_values();
    let g = &v
        .economy
        .veins
        .values()
        .next()
        .unwrap_or_else(|| panic!("keine Ader"))
        .gatherable;
    let mut t = Troop {
        job: Some(Job {
            r#type: "gather".into(),
            node_id: 5,
            ..Job::default()
        }),
        ..Troop::default()
    };
    gather_vein(&mut t, g, g.work_seconds / 2.0);
    assert_eq!(
        t.job.as_ref().map(|j| (j.r#type.as_str(), j.progress)),
        Some(("gather", 0.5))
    );
    gather_vein(&mut t, g, g.work_seconds / 2.0);
    let j = t.job.unwrap_or_default();
    assert_eq!(
        (j.r#type.as_str(), j.resource.as_str(), j.amount),
        ("carry", g.resource.as_str(), g.amount)
    );
}
