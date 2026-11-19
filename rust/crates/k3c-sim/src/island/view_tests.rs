//! B-1125: `Island::sync_stock` spiegelt die lesbare Insel-Sicht je Stufe in `World::island_view`; `island_ref_view` liefert
//! dieselben Zahlen wie `stage_ref`, ohne Insel (vor dem Spiegeln) ist es `None`. Die Lair-Sperre folgt dem Endboss.

use super::storage::capacity;
use super::testutil::test_data;
use crate::enemies::testutil::{bare_world, built_site, test_values};
use crate::state::World;

/// Stufe mit `own` gebauten Lagern.
fn stage_with_storages(own: usize) -> World {
    let mut w = bare_world();
    for i in 0..own {
        built_site(&mut w, "storage", 210.0 + i as f64);
    }
    w
}

fn island() -> super::Island {
    super::Island {
        stages: vec![stage_with_storages(0), stage_with_storages(1), stage_with_storages(2)],
        ..super::Island::default()
    }
}

#[test]
fn own_sync_stock_spiegelt_insel_sicht_je_stufe() {
    let difficulty = test_data().difficulty;
    let mut isl = island();
    isl.dev.stock_bonus = 40;
    isl.dev.cheap_build = true;
    assert!(isl.stages[0].island_ref_view().is_none(), "ohne Insel");
    isl.sync_stock();
    for i in 0..3 {
        let view = isl.stages[i].island_ref_view().expect("Sicht");
        let r = isl.stage_ref(i, &difficulty);
        let got = (
            view.stage,
            view.stages,
            view.storages_elsewhere,
            view.stock_bonus,
            view.cheap_build,
        );
        let want = (r.stage, r.stages, r.storages_elsewhere, r.stock_bonus, r.cheap_build);
        assert_eq!(got, want, "Stufe {i}");
    }
    let v = test_values();
    let r = isl.stages[1].island_ref_view();
    assert_eq!(capacity(&isl.stages[1], &v, r.as_ref()), Some(300 * 3 + 40 + 300 * 3));
}

#[test]
fn own_sync_stock_sperrt_lair_mit_dem_endboss() {
    let mut isl = island();
    isl.sync_stock();
    assert!(isl.stages.iter().all(|w| w.island_view.is_some_and(|x| !x.lair_closed)));
    isl.endboss_defeated = true;
    isl.sync_stock();
    assert!(isl.stages.iter().all(|w| w.island_view.is_some_and(|x| x.lair_closed)));
}
