//! Tests zu `plan_sites_cargo.rs` (RU62.2, B-997): Ladeplätze am Steg und Fracht an Deck als Zeichen-Einträge.

use k3c_sim::state::cargo_view::{CargoBayState, CargoEntry, CargoState};
use k3c_sim::state::{Site, World};

use super::plan::{Item, Planner, Shape};
use super::plan_sites_cargo::{bay_asset, deck_x, load_asset};
use super::testutil::{StubSheets, camera};

fn bay(kind: &str, x: f64, price: i64, state: &str) -> CargoBayState {
    CargoBayState {
        kind: kind.to_string(),
        x,
        price,
        state: state.to_string(),
    }
}

fn entry(kind: &str, index: i64) -> CargoEntry {
    CargoEntry {
        kind: kind.to_string(),
        id: String::new(),
        amount: 1,
        index,
    }
}

fn world(load: Vec<CargoEntry>, bays: Vec<CargoBayState>) -> World {
    World {
        sites: vec![Site {
            id: 1,
            kind: "ship".to_string(),
            x: 10.0,
            state: "built".to_string(),
            level: 4,
            ..Default::default()
        }],
        cargo: Some(CargoState {
            slots: 6,
            used: load.len() as i64,
            hold: true,
            load,
            bays,
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn sprites(items: &[Item]) -> Vec<String> {
    items
        .iter()
        .filter(|it| it.shape == Shape::Sprite)
        .map(|it| it.look.asset.to_string())
        .collect()
}

#[test]
fn ladeplaetze_und_fracht_ergeben_die_erwarteten_eintraege() {
    let w = world(
        vec![entry("coins", 0), entry("crate", 1), entry("troop", 2)],
        vec![
            bay("coins", 8.0, 5, "ready"),
            bay("troops", 10.0, 3, "empty"),
            bay("crates", 12.0, 2, "ready"),
        ],
    );
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.cargo(&w, &camera(10.0));
    let s = sprites(&p.body);
    for id in ["ladeplatz-muenzen", "ladeplatz-truppen", "ladeplatz-material"] {
        assert!(s.iter().any(|a| a.ends_with(id)), "Ladeplatz {id} fehlt: {s:?}");
    }
    assert_eq!(
        s.iter().filter(|a| a.ends_with("fracht-beutel")).count(),
        1,
        "ein Beutel an Deck"
    );
    assert_eq!(
        s.iter().filter(|a| a.ends_with("fracht-kiste")).count(),
        1,
        "eine Kiste an Deck"
    );
    // die Truppe hat kein Asset: Platzhalter mit Rahmen, drei Preise am Steg
    assert_eq!(
        p.body.iter().filter(|it| it.shape == Shape::Frame).count(),
        1,
        "Truppe an Deck"
    );
    let preise: Vec<&str> = p
        .top
        .iter()
        .filter(|it| it.shape == Shape::Text)
        .map(|it| it.text.as_str())
        .collect();
    assert_eq!(preise, ["5", "3", "2"]);
}

#[test]
fn volles_schiff_zeigt_den_preis_rot() {
    let w = world(Vec::new(), vec![bay("coins", 10.0, 5, "full")]);
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.cargo(&w, &camera(10.0));
    let t = p.top.iter().find(|it| it.shape == Shape::Text).expect("Preis");
    assert_eq!(t.clr, [230, 57, 70, 255]);
}

#[test]
fn ohne_fracht_zustand_nichts_und_ausserhalb_der_kamera_nichts() {
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.cargo(&World::default(), &camera(10.0));
    assert!(p.body.is_empty() && p.top.is_empty());
    let w = world(vec![entry("coins", 0)], vec![bay("coins", 10.0, 5, "ready")]);
    p.cargo(&w, &camera(200.0));
    assert!(p.body.is_empty() && p.top.is_empty(), "weit weg gezeichnet");
}

#[test]
fn plaetze_liegen_geordnet_auf_dem_deck() {
    let (a, b) = (deck_x(10.0, 6, 0), deck_x(10.0, 6, 5));
    assert!(a < b && a > 7.0 && b < 13.0, "{a} {b}");
    assert_eq!(deck_x(10.0, 0, 0), deck_x(10.0, 1, 0), "slots 0 teilt nicht durch null");
}

#[test]
fn assets_der_ladeplaetze_und_fracht_sind_zugeordnet() {
    assert_eq!(bay_asset("coins"), Some("ladeplatz-muenzen"));
    assert_eq!(bay_asset("troops"), Some("ladeplatz-truppen"));
    assert_eq!(bay_asset("crates"), Some("ladeplatz-material"));
    assert_eq!(bay_asset("x"), None);
    assert_eq!(load_asset("coins"), Some("fracht-beutel"));
    assert_eq!(load_asset("crate"), Some("fracht-kiste"));
    assert_eq!(load_asset("troop"), None);
}
