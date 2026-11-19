//! Hütte und Platz des Einsiedlers (RU62.10, AC-05): genau eine Hütte in Tiefe 1 am fernen Rand, deterministisch aus dem Seed,
//! keine bei einer Stufe oder in Welten ohne Einsiedler; der Hub-Platz besteht die Hub-Prüfung.

use super::testutil::{hall, hermit_data, hermit_island, hut};
use super::validate::check_hall_site;
use crate::island::switch_testutil::test_islands;
use crate::island::testutil::{ISLAND_SPEED, test_data};
use crate::island::{CreateOptions, create_island_in};

#[test]
fn genau_eine_huette_in_tiefe_1_am_fernen_rand() {
    let data = hermit_data("swordsmith");
    let (_, isl) = hermit_island(&data, 1);
    let huts: Vec<_> = isl.stages.iter().filter(|w| w.hermit_hut().is_some()).collect();
    assert_eq!(huts.len(), 1, "genau eine Hütte");
    let w = huts[0];
    assert_eq!(w.biome.depth, 1, "in Tiefe 1");
    let x = hut(&isl).x;
    let cfg = &data.hermits.hut;
    let far_is_right = w.hub_x <= w.width_units - w.hub_x;
    let margin = if far_is_right { w.width_units - x } else { x };
    assert!(
        margin >= cfg.edge_margin_min_units - 1e-9 && margin <= cfg.edge_margin_max_units + 1e-9,
        "Abstand {margin} zum Rand"
    );
    assert!(
        (x - w.hub_x).abs() > w.width_units / 2.0 - cfg.edge_margin_max_units - 1.0,
        "am Rand, der vom Hub am weitesten entfernt ist"
    );
    assert_eq!(hut(&isl).range, cfg.range_units);
    assert!(isl.stages[0].hermit_hut().is_none(), "an der Oberfläche keine Hütte");
}

#[test]
fn huette_liegt_bei_gleichem_seed_an_derselben_stelle() {
    let data = hermit_data("swordsmith");
    let (world, _) = test_islands(&data);
    let build = |seed: &str| {
        let isl =
            create_island_in(&data, &world, seed, &[0, 1], ISLAND_SPEED, CreateOptions::default()).expect("Insel");
        hut(&isl).x
    };
    assert_eq!(build("seed-a"), build("seed-a"));
    let xs: Vec<f64> = ["a", "b", "c", "d"].iter().map(|s| build(s)).collect();
    assert!(
        xs.windows(2).any(|p| p[0] != p[1]),
        "der Seed bestimmt den Abstand: {xs:?}"
    );
}

#[test]
fn insel_mit_einer_stufe_hat_keine_huette_und_keinen_platz() {
    let data = hermit_data("swordsmith");
    let (world, _) = test_islands(&data);
    let isl = create_island_in(&data, &world, "eins", &[0], ISLAND_SPEED, CreateOptions::default()).expect("Insel");
    assert!(isl.hermit.cfg.is_none());
    assert!(
        isl.stages
            .iter()
            .all(|w| w.hermit.is_none() && w.hermit_hut().is_none())
    );
    assert!(isl.hall_site().is_none(), "kein Platz für das Bauwerk");
}

#[test]
fn welten_ohne_einsiedler_haben_weder_huette_noch_platz() {
    let data = test_data(); // `test` steht nicht in hermits.json
    let (world, _) = test_islands(&data);
    let isl = create_island_in(&data, &world, "ohne", &[0, 1], ISLAND_SPEED, CreateOptions::default()).expect("Insel");
    assert!(isl.hermit.cfg.is_none() && isl.hall_site().is_none());
    assert!(isl.stages.iter().all(|w| w.hermit.is_none()));
    for id in ["standard", "britannien"] {
        assert!(!data.hermits.worlds.contains_key(id), "{id} ohne Einsiedler");
    }
}

#[test]
fn platz_des_bauwerks_liegt_an_der_oberflaeche_ab_hub_stufe_2() {
    let data = hermit_data("priestess");
    let (_, isl) = hermit_island(&data, 1);
    let top = &isl.stages[0];
    let s = hall(&isl);
    assert_eq!(s.kind, "hermitHall");
    assert_eq!(s.x, top.hub_x + data.hermits.hub_site.offset_units);
    assert_eq!(
        (s.state.as_str(), s.max_hp),
        ("unpaid", 200.0),
        "Priesterin: Schrein mit 200 HP"
    );
    assert_eq!(data.hermits.hub_site.hub_level, 2);
    let n = isl
        .stages
        .iter()
        .flat_map(|w| &w.sites)
        .filter(|s| s.kind == "hermitHall")
        .count();
    assert_eq!(n, 1, "genau ein Platz auf der ganzen Insel");
}

/// AC-05: Der Platz `hermitHall` besteht die Hub-Prüfung (Abstände, Hub-Breite, Treppen); ein Platz auf einer Treppe nicht.
#[test]
fn platz_besteht_die_hub_pruefung() {
    let data = hermit_data("swordsmith");
    assert_eq!(check_hall_site(&data), Ok(()));
    let mut bad = data.clone();
    bad.hermits.hub_site.offset_units = 16.0; // Lage der Treppe nach oben
    let e = check_hall_site(&bad).expect_err("auf der Treppe");
    assert!(e.starts_with("data/hub.json:") && e.contains("hermitHall"), "{e}");
    bad.hermits.hub_site.offset_units = 60.0; // außerhalb der Hub-Breite
    let e = check_hall_site(&bad).expect_err("außerhalb");
    assert!(e.contains("hub-breite"), "{e}");
}

#[test]
fn zerbrochene_pruefung_laesst_das_bauen_der_insel_scheitern() {
    let mut data = hermit_data("swordsmith");
    data.hermits.hub_site.offset_units = 16.0;
    let (world, _) = test_islands(&data);
    let e = create_island_in(&data, &world, "x", &[0, 1], ISLAND_SPEED, CreateOptions::default()).expect_err("Fehler");
    assert!(e.contains("Einsiedler") && e.contains("hermitHall"), "{e}");
}
