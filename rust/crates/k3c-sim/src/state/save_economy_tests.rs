//! Spielstand der Wirtschaft (Go: `save_economy_test.go`, AC-04 von B-201): Plantage, Berufe, Krieger, Elite, Rüstung und
//! Schwerter überleben Speichern und Laden. Die Fehlerfälle der Truppenart stehen in `save_tests.rs`.

use super::{Plot, Site, Troop, World};

fn read_fixture(version: i64) -> String {
    let path = k3c_det::golden::testdata().join(format!("saves/v{version}/familie.json"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Go: `setEconomy`: Stufe 0 mit Baumschule samt Plantage, Werkstatt, Schmiede, Rüstkammer (Rüstung Stufe 1), Bergmann,
/// Handwerker, Bogenschütze, Krieger und beiden Elite-Figuren; Stufe 1 mit Krieger und Baumeister.
fn set_economy(isl: &mut crate::island::Island) {
    use crate::enemies::testutil::build_site;
    use crate::unit::{jobs::make_fighter, spawn_vagrant};
    let (first, rest) = isl.stages.split_at_mut(1);
    let (w, w1) = (&mut first[0], &mut rest[0]);
    let nursery = build_site(w, "nursery");
    let shop = build_site(w, "workshop");
    let smithy = build_site(w, "smithy");
    let armory = build_site(w, "armory");
    crate::building::plantation::step_plantations(w, 31.0); // alle Plätze tragen einen Baum
    let cut = w.sites[nursery].plots[0].tree;
    w.nodes.retain(|n| Some(n.id) != cut);
    w.sites[nursery].plots[0] = Plot {
        tree: None,
        grow_in: 12.5,
    }; // einer abgebaut, der nächste wächst
    (
        w.sites[shop].swords,
        w.sites[shop].sword_paid_gold,
        w.sites[shop].sword_done,
    ) = (2, 1, 20.5);
    (
        w.sites[smithy].craft_done,
        w.sites[smithy].offer_paid,
        w.sites[smithy].offer_done,
    ) = (40.5, vec![3], 50.5);
    (w.sites[armory].bow_paid_gold, w.sites[armory].craft_done) = (70, 60.5);
    w.armor_level = 1;
    let peasant = |w: &mut World, prof: &str, site: Option<i64>| {
        let hub = w.hub_x;
        let ti = spawn_vagrant(w, hub, hub + 1.0);
        let hp = crate::world::vals(w).troops.get("peasant").map_or(1.0, |d| d.hp);
        let tr = &mut w.troops[ti];
        (tr.kind, tr.hp, tr.max_hp, tr.profession) = ("citizen".into(), hp, hp, prof.into());
        tr.wild = false;
        if let Some(id) = site {
            tr.work_site = id;
        }
    };
    let shop_id = w.sites[shop].id;
    peasant(w, "miner", None);
    peasant(w, "craftsman", Some(shop_id));
    for kind in ["archer", "warrior", "eliteArcher", "eliteWarrior"] {
        let hub = w.hub_x;
        let ti = spawn_vagrant(w, hub, hub + 2.0);
        make_fighter(w, ti, kind);
    }
    peasant(w1, "builder", None);
    let hub = w1.hub_x;
    let ti = spawn_vagrant(w1, hub, hub + 2.0);
    make_fighter(w1, ti, "warrior");
}

/// Go: `added`: die Truppen, die `set_economy` zur Startbesetzung dazugibt (ab dem ersten Bauern mit Beruf).
fn added(w: &World) -> Vec<&Troop> {
    let out: Vec<&Troop> = w.troops.iter().filter(|t| !t.is_wild()).collect();
    let from = out.iter().position(|t| !t.profession.is_empty()).unwrap_or(0);
    out[from..].to_vec()
}

fn troop_kinds(w: &World) -> Vec<String> {
    added(w)
        .iter()
        .map(|t| format!("{}:{}", t.kind, t.profession))
        .collect()
}

/// Go: `checkEconomy`: prüft den Zustand aus `set_economy` an der geladenen Insel.
fn check_economy(isl: &crate::island::Island) {
    let (w, w1) = (&isl.stages[0], &isl.stages[1]);
    let at = |kind: &str| {
        w.sites
            .iter()
            .position(|s| s.kind == kind)
            .unwrap_or_else(|| panic!("kein Platz {kind}"))
    };
    let (nursery, shop, smithy, armory) = (at("nursery"), at("workshop"), at("smithy"), at("armory"));
    let (n, shop, smithy, armory) = (&w.sites[nursery], &w.sites[shop], &w.sites[smithy], &w.sites[armory]);
    check_plots(n);
    assert!(
        shop.swords == 2 && shop.sword_paid_gold == 1 && shop.sword_done == 20.5,
        "Schwerter: {}/{}/{}",
        shop.swords,
        shop.sword_paid_gold,
        shop.sword_done
    );
    assert!(
        smithy.craft_done == 40.5 && smithy.offer_paid == [3] && smithy.offer_done == 50.5,
        "Elite: {}/{:?}/{}",
        smithy.craft_done,
        smithy.offer_paid,
        smithy.offer_done
    );
    assert!(
        armory.bow_paid_gold == 70 && armory.craft_done == 60.5 && w.armor_level == 1 && w1.armor_level == 0,
        "Rüstung: {}/{}, Stufe {}/{}",
        armory.bow_paid_gold,
        armory.craft_done,
        w.armor_level,
        w1.armor_level
    );
    check_troops(w, w1, shop.id);
}

fn check_plots(n: &Site) {
    if n.plots.is_empty() {
        return;
    }
    let trees = n.plots.iter().filter(|p| p.tree.is_some()).count();
    assert!(
        trees == 5 && n.plots.len() == 6,
        "Plantage: {trees} Bäume auf {} Plätzen",
        n.plots.len()
    );
    assert!(
        n.plots[0].tree.is_none() && n.plots[0].grow_in == 12.5,
        "Plantage Platz 0: {:?}",
        n.plots[0]
    );
}

fn check_troops(w: &World, w1: &World, shop_id: i64) {
    let want = [
        "citizen:miner",
        "citizen:craftsman",
        "archer:",
        "warrior:",
        "eliteArcher:",
        "eliteWarrior:",
    ];
    assert!(
        troop_kinds(w) == want && troop_kinds(w1) == ["citizen:builder", "warrior:"],
        "Truppen {:?} / {:?}",
        troop_kinds(w),
        troop_kinds(w1)
    );
    let a = added(w);
    assert!(
        a[1].work_site == shop_id && a[0].work_site == 0,
        "Arbeitsplatz des Handwerkers {}, Werkstatt {}",
        a[1].work_site,
        shop_id
    );
    let base = crate::world::vals(w).troops.get("eliteWarrior").map_or(0.0, |d| d.hp);
    assert!(
        a[5].max_hp == crate::state::links::armored_hp(w, "eliteWarrior") && a[5].max_hp > base,
        "Elite-Krieger ohne Rüstung: {}",
        a[5].max_hp
    );
}

fn economy_island() -> (crate::island::Island, k3c_data::Data) {
    use crate::island::testutil::{must_island, test_data};
    let data = test_data();
    let mut isl = must_island(&data, "familie", &[0, 1]);
    isl.sync_stock(); // Go: alle Stufen zeigen auf denselben (hier leeren) Vorrat
    let p0 = crate::island::add_island_player(&mut isl, 0);
    isl.stages[0].players[p0].gold = 23;
    let p1 = crate::island::add_island_player(&mut isl, 1);
    isl.stages[1].players[p1].gold = 8;
    set_economy(&mut isl);
    (isl, data)
}

/// Go: `TestSpielstandWirtschaftRundlauf` (save_economy_test.go): Speichern, Laden, Speichern ist gleich.
#[test]
fn spielstand_wirtschaft_rundlauf() {
    use crate::island::testutil::{load_island, save_json};
    let (isl, data) = economy_island();
    let at = "2026-10-13T20:00:00Z";
    let first = save_json(&isl, at);
    let loaded = load_island(&data, &first);
    assert_eq!(
        save_json(&loaded, at),
        first,
        "Speichern, Laden, Speichern ergibt etwas anderes"
    );
    assert_eq!(loaded.stages.iter().map(|w| w.players.len()).sum::<usize>(), 2);
    check_economy(&loaded);
}

/// Go: `TestSpielstandWirtschaftFixture` (save_economy_test.go): Fixture v7 trägt die Wirtschaft samt Boss und Händler.
#[test]
fn spielstand_wirtschaft_fixture() {
    use crate::island::testutil::{load_island, test_data};
    let data = test_data();
    let loaded = load_island(&data, &read_fixture(7));
    check_economy(&loaded);
    assert!(
        loaded.merchant_visits == 3 && loaded.stages[0].merchant.is_some(),
        "Händler aus v6 fehlt: {}",
        loaded.merchant_visits
    );
    for v in 1..7 {
        // ältere Stände laden ohne Wirtschaft: keine Rüstung, kein Beruf
        for w in &load_island(&data, &read_fixture(v)).stages {
            assert!(
                w.armor_level == 0 && !w.troops.iter().any(|t| !t.profession.is_empty()),
                "v{v}: Rüstung {}",
                w.armor_level
            );
        }
    }
}
