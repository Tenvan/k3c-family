//! Tests zu `view.rs` (Go: `economy_view_test.go`): W9.1 (B-323/AC-01, Sprint W9 AC-01) Lager-Maximum, Hub-Ausbau mit Kosten und
//! Wartegrund „Gefahr“ im JSON; W10.1 (B-332, Sprint W10 AC-02) Kämpfer-Zahl und Truppen-Limit. Die Tests über
//! `create_world` und `create_island` warten auf `new_hub_site` (RU18.3); die Wartegründe prüft der erste Test ohne Welt.

use k3c_data::buildings::Cost;
use k3c_data::{Source, load_all};
use serde_json::{Value, json};

use super::pending::hub_gate;
use super::view::{WAITING_NEXT_ISLAND, economy_of, hub_wait_state, material_of};
use crate::enemies::testutil::{bare_world, build_site};
use crate::island::testutil::{must_island, on_stage, storage_site};
use crate::state::common::destroy_site;
use crate::state::island_ref::IslandRef;
use crate::state::pending::next_level;
use crate::state::{Site, World};
use crate::unit::jobs::make_archer;
use crate::unit::spawn_vagrant;
use crate::world::pending::upgrade_payable;
use crate::world::testutil::quiet_world;
use crate::world::vals;

/// Go: `economyJSON` ist `EconomyOf(w)` als JSON-Objekt, wie W5.1 es in den Zustand übernimmt.
fn economy_json(w: &World, isl: Option<&IslandRef>) -> Value {
    let v = vals(w);
    serde_json::to_value(economy_of(w, &v, isl)).expect("JSON")
}

#[test]
fn material_of_nimmt_nur_das_material() {
    let c = Cost {
        gold: 9,
        wood: 1,
        stone: 2,
        copper: 3,
        iron: 4,
        crystal: 5,
    };
    let m = material_of(&c);
    assert_eq!((m.wood, m.stone, m.copper, m.iron, m.crystal), (1, 2, 3, 4, 5));
}

#[test]
fn economy_ohne_hub_platz_und_ohne_insel() {
    let mut w = bare_world();
    w.hub_level = 2;
    let got = economy_json(&w, None);
    assert_eq!(
        got,
        json!({"hubLevel": 2, "fighters": 0, "troopLimit": got["troopLimit"]})
    );
    w.cycle.phase = "night".into();
    assert_eq!(economy_json(&w, None)["danger"], json!(true), "Nacht ist Gefahr");
    let isl = IslandRef {
        stock_bonus: 50,
        stages: 1,
        ..IslandRef::default()
    };
    let got = economy_json(&w, Some(&isl));
    assert!(
        got["stockMax"].as_i64().is_some_and(|m| m > 0),
        "Insel: Lager-Maximum {got}"
    );
    assert!(
        got["purseMax"].as_i64().is_some_and(|m| m > 50),
        "Dev-Bonus: Gold-Maximum {got}"
    );
}

#[test]
fn wartegrund_ohne_hub_ausbau_ist_der_stand_des_platzes() {
    let w = World::default();
    let h = Site {
        upgrade: "waitingWorker".into(),
        ..Site::default()
    };
    assert_eq!(hub_wait_state(&w, None, &h, &Cost::default()), "waitingWorker");
    assert_eq!(WAITING_NEXT_ISLAND, "waitingNextIsland");
}

#[test]
fn wartegrund_nur_tor_anteil_fehlt_ist_wartet_auf_naechste_insel() {
    // Schritt 3 von Hub-Stufe 1: Der Stein des Schritts ist da, nur der Tor-Stein fehlt (wie `hub_step_test.go`).
    let mut w = crate::world::testutil::quiet_world();
    w.hub_step = 3;
    let hub_site = w.hub_site.clone().expect("Hub-Platz");
    let step = next_level(&w, None, &hub_site).expect("Schritt 3");
    let gate = hub_gate(&w, None);
    assert!(gate.stone > 0, "Tor von Hub-Stufe 2 ist Stein: {gate:?}");
    let h = Site {
        upgrade: "waitingMaterial".into(),
        ..Site::default()
    };
    w.stock = material_of(&Cost {
        stone: step.cost.stone - gate.stone,
        ..Cost::default()
    });
    assert_eq!(hub_wait_state(&w, None, &h, &step.cost), WAITING_NEXT_ISLAND);
    w.stock.stone = 0; // auch vom Stein des Schritts fehlt mehr als das Tor
    assert_eq!(hub_wait_state(&w, None, &h, &step.cost), "waitingMaterial");
}

#[test]
fn economy_insel_mit_lager_ausbau_und_nacht() {
    let data = load_all(&Source::Embedded).expect("eingebettete Daten");
    let mut isl = must_island(&data, "w9-wirtschaft", &[0, 1, 2]);
    let got = on_stage(&mut isl, &data, 0, |w, r| {
        let si = storage_site(w).expect("Lager");
        w.sites[si].state = "built".into();
        while let Some(h) = w.hub_site.clone().filter(|h| upgrade_payable(w, h)) {
            super::pending::pay_upgrade(w, None, h.id);
        }
        w.cycle.phase = "night".into();
        economy_json(w, Some(r))
    });
    assert!(
        got["stockMax"] == json!(1200) && got["hubLevel"] == json!(1) && got["danger"] == json!(true),
        "stockMax/hubLevel/danger: {got}"
    );
    let up = &got["hubUpgrade"];
    assert!(
        up["gold"] == json!(4)
            && up["paid"] == json!(4)
            && up["state"] == json!("waitingMaterial")
            && up["material"]["stone"] == json!(25)
            && up["steps"] == json!(3),
        "hubUpgrade: {up}"
    );
}

#[test]
fn economy_ohne_insel_und_stufe5() {
    let mut w = quiet_world();
    w.cycle.phase = "day".into();
    w.enemies.clear();
    let got = economy_json(&w, None);
    assert!(got.get("stockMax").is_none(), "ohne Insel kein Maximum: {got}");
    assert!(got.get("danger").is_none(), "am Tag ohne Gegner keine Gefahr: {got}");
    let up = &got["hubUpgrade"];
    assert!(
        up["gold"] == json!(4) && up["paid"] == json!(0) && up.get("state").is_none() && got["hubStep"] == json!(1),
        "Stufe 1 ohne Zahlung: {up}"
    );
    w.hub_level = vals(&w).hub.levels.len() as i64;
    let got = economy_json(&w, None);
    assert!(
        got["hubLevel"] == json!(5) && got.get("hubUpgrade").is_none() && got.get("hubStep").is_none(),
        "Stufe 5 ohne Kosten: {got}"
    );
}

/// W10.1 (B-332, Sprint W10 AC-02): Kämpfer-Zahl und Truppen-Limit aus `fighters` und `troop_limit`, noch nicht im JSON.
#[test]
fn economy_kaempfer_und_limit() {
    let mut w = quiet_world();
    let l = vals(&w).buildings["barracks"].troop_limit.clone();
    let e = economy_of(&w, &vals(&w), None);
    assert!(
        e.fighters == 0 && e.troop_limit == l.base,
        "ohne Kämpfer und Kaserne: {}/{}, erwartet 0/{}",
        e.fighters,
        e.troop_limit,
        l.base
    );
    let hub = w.hub_x;
    for _ in 0..2 {
        let ti = spawn_vagrant(&mut w, hub, hub);
        make_archer(&mut w, ti);
    }
    spawn_vagrant(&mut w, hub + 30.0, hub + 30.0); // zählt nicht
    let barracks = build_site(&mut w, "barracks");
    let e = economy_of(&w, &vals(&w), None);
    assert!(
        e.fighters == 2 && e.troop_limit == l.base + l.per_building,
        "2 Bogenschützen, Kaserne: {}/{}, erwartet 2/{}",
        e.fighters,
        e.troop_limit,
        l.base + l.per_building
    );
    let id = w.sites[barracks].id;
    destroy_site(&mut w, id);
    let e = economy_of(&w, &vals(&w), None);
    assert_eq!(e.troop_limit, l.base, "zerstörte Kaserne zählt nicht");
    let j = economy_json(&w, None);
    assert!(
        j["fighters"] == json!(2) && j["troopLimit"] == json!(l.base),
        "JSON fighters/troopLimit: {} {} (W10.2)",
        j["fighters"],
        j["troopLimit"]
    );
}
