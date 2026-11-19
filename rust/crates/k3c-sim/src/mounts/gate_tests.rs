//! Das Gatter (RU62.8, AC-04): genau eines je Insel an der Oberfläche, Mindestabstand zum Hub, gleicher Seed gleiche
//! Stelle; Welten ohne Reittier bleiben unverändert.

use super::testutil::{gate, mounts_data, mounts_island};
use super::{EDGE_MARGIN_UNITS, gate_x, world_mount};
use crate::island::switch_testutil::{gate_island, test_islands};
use crate::island::testutil::test_data;

#[test]
fn gatter_genau_eines_an_der_oberflaeche() {
    let data = mounts_data();
    let (_, isl, _) = mounts_island(&data, 1);
    let gates: Vec<_> = isl
        .stages
        .iter()
        .filter(|w| w.mount_gate.is_some())
        .map(|w| w.biome.depth)
        .collect();
    assert_eq!(gates, vec![0], "ein Gatter, in der Stufe der Tiefe 0");
    let g = gate(&isl);
    assert_eq!((g.mount.as_str(), g.sprite.as_str(), g.price), ("camel", "camel", 30));
    assert!(g.range > 0.0);
    assert!(
        isl.stages.iter().all(|w| w.mounts_cfg.is_some()),
        "Katalog in jeder Stufe"
    );
}

#[test]
fn gatter_mindestabstand_zum_hub_und_im_spielfeld() {
    let data = mounts_data();
    let (world, _) = test_islands(&data);
    let min = data.mounts.mount_gate.min_hub_distance_units;
    for n in 0..24 {
        let isl = crate::island::create_island_in(
            &data,
            &world,
            &format!("gatter-{n}"),
            &[0, 1],
            crate::island::testutil::ISLAND_SPEED,
            crate::island::CreateOptions::default(),
        )
        .expect("Insel");
        let w = &isl.stages[0];
        let g = w.mount_gate.clone().expect("Gatter");
        assert!(
            (g.x - w.hub_x).abs() >= min,
            "Seed {n}: Abstand {}",
            (g.x - w.hub_x).abs()
        );
        assert!(
            g.x >= EDGE_MARGIN_UNITS && g.x <= w.width_units - EDGE_MARGIN_UNITS,
            "Seed {n}: x {} außerhalb 0..{}",
            g.x,
            w.width_units
        );
    }
}

#[test]
fn gatter_gleicher_seed_gleiche_stelle() {
    let data = mounts_data();
    let (world, _) = test_islands(&data);
    let make = |seed: &str| {
        let isl = crate::island::create_island_in(
            &data,
            &world,
            seed,
            &[0, 1],
            crate::island::testutil::ISLAND_SPEED,
            crate::island::CreateOptions::default(),
        )
        .expect("Insel");
        isl.stages[0].mount_gate.clone().expect("Gatter").x
    };
    assert_eq!(make("alpha"), make("alpha"));
    let xs: Vec<f64> = (0..8).map(|n| make(&format!("s{n}"))).collect();
    assert!(
        xs.windows(2).any(|p| p[0] != p[1]),
        "verschiedene Seeds streuen: {xs:?}"
    );
}

#[test]
fn gatter_x_ohne_platz_steht_am_fernen_rand() {
    // Welt 100 breit, Hub in der Mitte, Mindestabstand 60: links und rechts passt nichts
    let x = gate_x("s", 0, 100.0, 50.0, 60.0);
    assert!(x == EDGE_MARGIN_UNITS || x == 100.0 - EDGE_MARGIN_UNITS, "{x}");
    // Hub nahe links: nur rechts ist Platz
    let x = gate_x("s", 0, 400.0, 20.0, 40.0);
    assert!((60.0..=390.0).contains(&x), "{x}");
    // gleiche Eingabe, gleiches Ergebnis
    assert_eq!(gate_x("t", 0, 300.0, 150.0, 40.0), gate_x("t", 0, 300.0, 150.0, 40.0));
}

#[test]
fn welt_ohne_reittier_hat_kein_gatter() {
    let data = test_data(); // `test` steht nicht in mounts.json
    let (world, _) = test_islands(&data);
    let isl = gate_island(&data, &world, 1);
    assert!(
        isl.stages
            .iter()
            .all(|w| w.mount_gate.is_none() && w.mounts_cfg.is_none())
    );
    // die echten Daten: standard hat keins, sengoku und nil haben je eins
    assert_eq!(world_mount(&data, "standard"), None);
    assert_eq!(world_mount(&data, "britannien"), None);
    assert_eq!(world_mount(&data, "sengoku"), Some("sengokuHorse"));
    assert_eq!(world_mount(&data, "nil"), Some("camel"));
}
