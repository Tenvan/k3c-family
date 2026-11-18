//! Lager-Maximum der Insel (Go: `island_storage_test.go`, SP13.2): fünf Materialien, Maximum (300 je Hub + 300 je Lager) und
//! Lager-Bauplatz nur in Inseln. Die Go-Tests bauen eine Insel (`create_world_with`, RU14) und tragen `ignore`; `own_*` prüft
//! dieselben Zahlen an einer von Hand gebauten Stufe mit einem `IslandRef` (Stufenzahl, Lager der anderen Stufen).

#![allow(clippy::unimplemented)] // Rümpfe der ignorierten Tests: warten auf den genannten Sprint

use k3c_data::hub::Stock;
use serde_json::Value;

use super::save_tests::{Env, fixture};
use super::storage::{add_stock_capped, capacity, deposit_point, purse_max, resource_full, stock_field, stock_get};
use super::testutil::*;
use super::{IslandSave, add_island_player};
use crate::enemies::testutil::{bare_world, built_site, test_values};
use crate::state::{IslandRef, World};

fn isl_ref(stages: i64, storages_elsewhere: i64) -> IslandRef {
    IslandRef {
        stages,
        storages_elsewhere,
        ..IslandRef::default()
    }
}

/// Stufe 0 mit `own` gebauten Lagern und einem unbezahlten.
fn stage_with_storages(own: usize) -> World {
    let mut w = bare_world();
    for i in 0..own {
        built_site(&mut w, "storage", 210.0 + i as f64);
    }
    w.sites.push(crate::state::Site {
        id: 99,
        kind: "storage".into(),
        state: "unpaid".into(),
        ..Default::default()
    });
    w
}

/// Go: `TestIslandStorageCapacity`: 300 je Stufe plus 300 je gebautem Lager; die Lager der anderen Stufen kommen aus dem
/// `IslandRef`, die der eigenen Stufe liest `capacity` aus `w.sites`.
#[test]
fn own_capacity_je_hub_und_lager() {
    let v = test_values();
    assert_eq!(
        (v.economy.storage.base_per_hub, v.economy.storage.per_storage),
        (300, 300)
    );
    for stages in 1..=3_i64 {
        for built in 0..=stages.min(2) {
            let own = built.min(1); // Go baut die ersten `built` Stufen; Stufe 0 rechnet
            let w = stage_with_storages(own as usize);
            let r = isl_ref(stages, built - own);
            let want = 300 * stages + 300 * built;
            assert_eq!(capacity(&w, &v, Some(&r)), Some(want), "{stages} Stufen, {built} Lager");
        }
    }
    let r = IslandRef {
        stock_bonus: 50,
        ..isl_ref(2, 0)
    };
    assert_eq!(
        capacity(&stage_with_storages(0), &v, Some(&r)),
        Some(650),
        "Dev-Bonus (devMaxStock)"
    );
}

/// `Island::stage_ref` füllt Stufenzahl, Dev-Bonus und die gebauten Lager der anderen Stufen für `capacity`.
#[test]
fn own_stage_ref_traegt_stufenzahl_bonus_und_fremde_lager() {
    let difficulty = test_data().difficulty;
    let mut isl = super::Island {
        stages: vec![stage_with_storages(0), stage_with_storages(1), stage_with_storages(2)],
        ..super::Island::default()
    };
    isl.dev.stock_bonus = 40;
    let r0 = isl.stage_ref(0, &difficulty);
    assert_eq!((r0.stages, r0.storages_elsewhere, r0.stock_bonus), (3, 3, 40));
    let r1 = isl.stage_ref(1, &difficulty);
    assert_eq!(r1.storages_elsewhere, 2, "ohne die eigene Stufe");
    let v = test_values();
    assert_eq!(
        capacity(&isl.stages[1], &v, Some(&r1)),
        Some(300 * 3 + 40 + 300 * 3),
        "drei Lager insgesamt"
    );
}

/// Go: `TestIslandStorageSiteOnlyInIslands` (Maximum): Eine Welt ohne Insel hat kein Maximum und nie ein volles Lager.
#[test]
fn own_ohne_insel_ist_das_lager_unbegrenzt() {
    let v = test_values();
    let mut w = stage_with_storages(1);
    w.stock.wood = 1_000_000;
    assert_eq!(capacity(&w, &v, None), None);
    assert!(!resource_full(&w, &v, None, "wood"));
    assert_eq!(add_stock_capped(&mut w, &v, None, "wood", 40), (40, true));
    assert_eq!(w.stock.wood, 1_000_040, "ohne Insel kein Maximum");
    assert_eq!(
        (purse_max(&v, None), purse_max(&v, Some(&isl_ref(1, 0)))),
        (v.economy.purse.max_gold, v.economy.purse.max_gold)
    );
    let bonus = IslandRef {
        stock_bonus: 7,
        ..IslandRef::default()
    };
    assert_eq!(
        purse_max(&v, Some(&bonus)),
        v.economy.purse.max_gold + 7,
        "Dev-Bonus der Insel"
    );
}

/// Go: `TestIslandStorageAddCapped` (ohne Zerstörung eines Lagers: `destroy_site`, RU18): Aufnahme bis zum Maximum, der
/// Überschuss über dem Maximum bleibt, Gold ist kein Baumaterial.
#[test]
fn own_add_stock_capped_bis_zum_maximum() {
    let v = test_values();
    let mut w = stage_with_storages(0);
    let r = isl_ref(2, 0); // Maximum 600
    assert_eq!(add_stock_capped(&mut w, &v, Some(&r), "wood", 250), (250, true));
    assert_eq!(add_stock_capped(&mut w, &v, Some(&r), "wood", 500), (350, true));
    assert_eq!(w.stock.wood, 600);
    assert_eq!(
        add_stock_capped(&mut w, &v, Some(&r), "wood", 5),
        (0, true),
        "voller Vorrat"
    );
    assert!(resource_full(&w, &v, Some(&r), "wood"));
    assert!(!resource_full(&w, &v, Some(&r), "stone"));
    // Zerstörtes Lager: Der Vorrat liegt über dem Maximum, der Überschuss bleibt, aufgenommen wird erst darunter.
    w.stock.wood = 700;
    assert_eq!(
        add_stock_capped(&mut w, &v, Some(&r), "wood", 5),
        (0, true),
        "über Kapazität"
    );
    assert_eq!(w.stock.wood, 700, "Überschuss gelöscht");
    assert!(resource_full(&w, &v, Some(&r), "wood"));
    w.stock.wood -= 150;
    assert_eq!(add_stock_capped(&mut w, &v, Some(&r), "wood", 100), (50, true));
    assert_eq!(
        add_stock_capped(&mut w, &v, Some(&r), "gold", 1),
        (0, false),
        "gold ist kein Baumaterial"
    );
    assert!(!resource_full(&w, &v, Some(&r), "gold"));
}

/// Go: `stockField`: der Zähler zu jedem der fünf Rohstoffe, `None` bei unbekanntem Namen.
#[test]
fn own_stock_field_kennt_fuenf_rohstoffe() {
    let mut s = Stock::default();
    for (i, r) in ["wood", "stone", "copper", "iron", "crystal"].iter().enumerate() {
        *stock_field(&mut s, r).expect("Rohstoff") += i as i64 + 1;
    }
    assert_eq!(
        s,
        Stock {
            wood: 1,
            stone: 2,
            copper: 3,
            iron: 4,
            crystal: 5
        }
    );
    assert_eq!(stock_get(&s, "crystal"), Some(5));
    assert!(stock_field(&mut s, "gold").is_none() && stock_get(&s, "gold").is_none());
}

/// Go: `depositPoint`: die Burg oder das nächste gebaute Lager; bei Gleichstand gewinnt die Burg.
#[test]
fn own_deposit_point_gleichstand_burg_vor_lager() {
    let mut w = bare_world(); // Hub bei 200
    let i = built_site(&mut w, "storage", 230.0);
    assert_eq!(deposit_point(&w, 225.0), 230.0, "Lager näher als die Burg");
    assert_eq!(deposit_point(&w, 205.0), 200.0, "Burg näher");
    w.sites[i].x = 210.0;
    assert_eq!(deposit_point(&w, 205.0), 200.0, "Gleichstand: Burg vor Lager");
    w.sites[i].state = "unpaid".into();
    assert_eq!(deposit_point(&w, 209.0), 200.0, "ein ungebautes Lager zählt nicht");
}

/// Go: `TestIslandStorageFiveMaterials` (Spielstand-Teil): Rundlauf mit fünf Materialien; ein alter Stand ohne Eisen und
/// Kristall liest unverändert.
#[test]
fn own_fuenf_materialien_im_stand() {
    let env = Env::standard();
    let mut s: IslandSave = env.parse(&fixture(9)).expect("v9");
    s.stock = Stock {
        wood: 9,
        stone: 8,
        copper: 7,
        iron: 6,
        crystal: 5,
    };
    let raw = serde_json::to_string(&s).expect("schreiben");
    assert!(
        raw.contains(r#","iron":6,"crystal":5"#),
        "Eisen und Kristall stehen im Stand"
    );
    assert_eq!(env.parse(&raw).expect("Rundlauf").stock, s.stock);
    let old = raw.replacen(r#","iron":6,"crystal":5"#, "", 1);
    assert_eq!(
        env.parse(&old).expect("alter Stand").stock,
        Stock {
            wood: 9,
            stone: 8,
            copper: 7,
            iron: 0,
            crystal: 0
        }
    );
    let zero: Value = serde_json::to_value(Stock::default()).expect("Vorrat");
    assert!(
        zero.get("iron").is_none() && zero.get("crystal").is_none(),
        "Eisen und Kristall fallen ohne Wert weg"
    );
}

/// Go: `TestIslandStorageCapacity`: dieselben Zahlen an einer gebauten Insel.
#[test]
fn island_storage_capacity() {
    let data = test_data();
    let v = test_values();
    for stages in 1..=3_usize {
        let mut isl = must_island(&data, "lager-a", &[0, 1, 2][..stages]);
        for built in 0..=stages.min(2) {
            for (i, w) in isl.stages.iter_mut().enumerate() {
                let si = storage_site(w).expect("Lager-Bauplatz");
                w.sites[si].state = if i < built { "built" } else { "unpaid" }.into();
            }
            let got = on_stage(&mut isl, &data, 0, |w, r| capacity(w, &v, Some(r)));
            assert_eq!(
                got,
                Some((300 * stages + 300 * built) as i64),
                "{stages} Stufen, {built} Lager"
            );
        }
    }
}

/// Go: `TestIslandStorageAddCapped`: Aufnahme über zwei Stufen, Zerstörung eines Lagers lässt den Überschuss stehen.
#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprueft: not implemented: RU18: add_stock, destroy_site (Zerstörung des Lagers))"]
fn island_storage_add_capped() {
    let data = test_data();
    let v = test_values();
    let mut isl = must_island(&data, "lager-b", &[0, 1]);
    assert_eq!(
        on_stage(&mut isl, &data, 0, |w, r| add_stock_capped(w, &v, Some(r), "wood", 250)).0,
        250
    );
    assert_eq!(
        on_stage(&mut isl, &data, 1, |w, r| add_stock_capped(w, &v, Some(r), "wood", 500)).0,
        350
    );
    assert_eq!(isl.stock.wood, 600);
    assert_eq!(
        on_stage(&mut isl, &data, 0, |w, r| add_stock_capped(w, &v, Some(r), "wood", 5)).0,
        0
    );
    unimplemented!("RU18: add_stock, destroy_site (Zerstörung des Lagers)")
}

/// Go: `TestIslandStorageFiveMaterials`: Kosten über fünf Materialien (`can_afford`, `spend`, RU18) und Rundlauf.
#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprueft: not implemented: RU18: add_stock, can_afford, spend)"]
fn island_storage_five_materials() {
    let data = test_data();
    let isl = must_island(&data, "lager-c", &[0]);
    let loaded = load_island(&data, &save_json(&isl, "t"));
    assert_eq!(loaded.stock, isl.stock, "Rundlauf");
    unimplemented!("RU18: add_stock, can_afford, spend")
}

/// Go: `TestIslandStorageSiteOnlyInIslands`: Lager-Bauplatz nur in Insel-Stufen, die Campaign-Welt hat keinen.
#[test]
fn island_storage_site_only_in_islands() {
    let data = test_data();
    let isl = must_island(&data, "lager-d", &[0, 1]);
    for w in &isl.stages {
        assert!(storage_site(w).is_some(), "Insel-Stufe ohne Lager-Bauplatz");
    }
    let b = super::data::biome_for_depth(&data.biomes, 0).expect("Biom");
    let opts = crate::world::Options {
        cycle_speed: ISLAND_SPEED,
        ..crate::world::Options::default()
    };
    let camp = crate::world::create_world(b, "lager-d", &opts).expect("Welt");
    assert!(storage_site(&camp).is_none(), "Campaign-Welt hat einen Lager-Bauplatz");
    let v = crate::world::vals(&camp);
    assert!(capacity(&camp, &v, None).is_none(), "Campaign-Welt hat ein Maximum");
    let stock = serde_json::to_string(&camp.stock).expect("Vorrat als JSON");
    assert!(
        stock.contains(r#""copper":0}"#),
        "Eisen/Kristall müssen ohne Wert wegfallen (omitempty): {stock}"
    );
}

/// Go: `TestIslandStorageDeterministic`: Zwei Läufe einer Insel mit Lager ergeben denselben Zustand.
#[test]
fn island_storage_deterministic() {
    let data = test_data();
    let run = || {
        let mut isl = must_island(&data, "lager-e", &[0, 1]);
        add_island_player(&mut isl, 0);
        isl.stock.iron += 400;
        let si = storage_site(&isl.stages[1]).expect("Lager-Bauplatz");
        isl.stages[1].sites[si].state = "built".into();
        run_island(&mut isl, &data, &[Default::default()], 20.0);
        island_json(&isl).join("|")
    };
    assert_eq!(run(), run(), "Insel mit Lager ist nicht deterministisch");
}
