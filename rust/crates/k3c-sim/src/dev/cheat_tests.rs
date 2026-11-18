//! Go: `dev_cheat_test.go`: Dev-Lager, Dev-Gold-Maximum, Baukosten 1, Dev-Ausbau und nächste Tageszeit.

use k3c_data::buildings::Cost;

use super::testutil::island_with_player;
use super::{
    DEV_STOCK_BONUS_MAX, dev_build_all, dev_fill_stock, dev_next_phase, dev_raise_max_stock, dev_set_cheap_build,
};
use crate::island::step_island;
use crate::island::storage::capacity;
use crate::state::island_ref::cheap;
use crate::state::pending::values;
use crate::state::purse::purse_max_of;

const DT: f64 = 1.0 / 30.0;

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn lager_maximum_und_fuellen() {
    let (data, mut isl, _) = island_with_player("dev-lager");
    let d = &data.difficulty;
    let cap = |isl: &mut crate::island::Island| {
        let r = isl.stage_ref(0, d);
        let c = capacity(&isl.stages[0], &values(&isl.stages[0]), Some(&r)).expect("Lager-Maximum");
        isl.put_stage_ref(r);
        c
    };
    let base = cap(&mut isl);
    for _ in 0..2 {
        assert!(dev_raise_max_stock(&mut isl), "Erhöhung abgelehnt");
    }
    assert_eq!(cap(&mut isl), base + 200);
    dev_fill_stock(&mut isl, d);
    let s = &isl.stock;
    for v in [s.wood, s.stone, s.copper, s.iron, s.crystal] {
        assert_eq!(v, base + 200, "Vorrat {s:?}");
    }
    while isl.dev.stock_bonus < DEV_STOCK_BONUS_MAX {
        assert!(dev_raise_max_stock(&mut isl), "Erhöhung vor der Obergrenze abgelehnt");
    }
    assert!(!dev_raise_max_stock(&mut isl), "Obergrenze überschritten");
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn gold_maximum_und_fuellen() {
    let (data, mut isl, _) = island_with_player("dev-gold-max");
    let d = &data.difficulty;
    let max_gold = values(&isl.stages[0]).economy.purse.max_gold;
    let purse = |isl: &mut crate::island::Island| {
        let r = isl.stage_ref(0, d);
        let m = purse_max_of(&isl.stages[0], &isl.stages[0].players[0], Some(&r));
        isl.put_stage_ref(r);
        m
    };
    assert_eq!(purse(&mut isl), max_gold);
    dev_raise_max_stock(&mut isl);
    dev_raise_max_stock(&mut isl);
    assert_eq!(purse(&mut isl), max_gold + 200);
    isl.stages[0].players[0].gold = 5;
    dev_fill_stock(&mut isl, d);
    assert_eq!(isl.stages[0].players[0].gold, max_gold + 200);
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn baukosten_sind_mit_dem_schalter_hoechstens_eins() {
    let (data, mut isl, _) = island_with_player("dev-kosten");
    let c = Cost {
        gold: 20,
        wood: 50,
        ..Cost::default()
    };
    let r = isl.stage_ref(0, &data.difficulty);
    assert_eq!(cheap(Some(&r), &c), c, "Preis ohne Schalter verändert");
    isl.put_stage_ref(r);
    dev_set_cheap_build(&mut isl, true);
    let r = isl.stage_ref(0, &data.difficulty);
    let want = Cost {
        gold: 1,
        wood: 1,
        ..Cost::default()
    };
    assert_eq!(cheap(Some(&r), &c), want);
    isl.put_stage_ref(r);
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story) und RU18 (site_hub_level)"]
fn dev_ausbau_baut_hub_stufe_mauern_und_tuerme() {
    let (_, mut isl, _) = island_with_player("dev-ausbau");
    assert!(
        !dev_build_all(&mut isl, 0) && !dev_build_all(&mut isl, 6),
        "ungültige Stufe angenommen"
    );
    assert!(dev_build_all(&mut isl, 3) && isl.stages[0].hub_level == 3);
    let w = &isl.stages[0];
    let want = w.level.sites.iter().filter(|l| l.kind == "wall" && l.zone <= 3).count();
    let walls = w
        .sites
        .iter()
        .filter(|s| s.kind == "wall" && crate::world::lines::level_site(w, s).is_some_and(|l| l.zone <= 3))
        .inspect(|s| assert!(s.state == "built" && s.level == 3, "Mauer {:?}", s.id))
        .count();
    assert!(walls == want && want > 0, "{walls} Mauern, erwartet {want}");
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story) und RU18 (step_players)"]
fn naechste_tageszeit_wechselt_im_naechsten_schritt() {
    let (data, mut isl, _) = island_with_player("dev-next-phase");
    for want in ["dusk", "night", "dawn", "day", "dusk"] {
        assert_eq!(dev_next_phase(&mut isl), want);
        step_island(&mut isl, &[], &data.difficulty, DT);
        assert_eq!(isl.stages[0].cycle.phase, want);
    }
}
