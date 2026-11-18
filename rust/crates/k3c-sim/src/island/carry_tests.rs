//! Träger in Inseln (Go: `island_carry_test.go`, SP13.3, prüft `carry` aus `unit_common.go` und `carryToStock`): Material geht
//! zum nächsten Lager oder zur Burg; bei vollem Maximum wartet der Bauer. Die Go-Tests bauen eine Insel (`create_world_with`,
//! RU14); wo ein Ende des Auftrags `add_pool` (RU14) oder ein Bauplatz `each_site` (RU18) braucht, tragen sie `ignore`.
//! `own_*` läuft an einer von Hand gebauten Stufe, solange der Träger kein Hammer-Handwerker ist und nichts abschließt.

#![allow(clippy::unimplemented)] // Rümpfe der ignorierten Tests: warten auf den genannten Sprint

use super::storage::carry_to_stock;
use super::testutil::*;
use crate::enemies::testutil::{bare_world, built_site, test_values};
use crate::state::pending::values;
use crate::state::{IslandRef, Job, Troop, World};

/// Go: `carrier`: ein tagsüber wartender Träger mit Holz bei `x`; `hammer` false (Go: true) hält `build_job` aus `each_site`.
fn carrier(w: &mut World, x: f64, amount: i64, hammer: bool) -> usize {
    w.cycle.phase = "day".into();
    w.enemies.clear();
    w.troops.push(Troop {
        id: 900,
        kind: "peasant".into(),
        x,
        hp: 1.0,
        max_hp: 1.0,
        hammer,
        job: Some(Job {
            r#type: "carry".into(),
            resource: "wood".into(),
            amount,
            ..Job::default()
        }),
        ..Troop::default()
    });
    w.troops.len() - 1
}

fn island_one() -> IslandRef {
    IslandRef {
        stages: 1,
        ..IslandRef::default()
    }
}

/// Go: `TestIslandCarryWaitsWhenFull` (Teilaufnahme und Warten): Maximum 300, Vorrat 290: Teilaufnahme, der Rest bleibt im
/// Job, bei vollem Lager geht nichts verloren und es kommt kein Ereignis.
#[test]
fn own_carry_teilaufnahme_und_warten_bei_vollem_lager() {
    let (v, r) = (test_values(), island_one());
    let mut w = bare_world();
    w.stock.wood = 290;
    let hub = w.hub_x;
    let ti = carrier(&mut w, hub, 25, false);
    carry_to_stock(&mut w, &v, ti, Some(&r), super::testutil::DT);
    let job = w.troops[ti].job.as_ref().expect("Rest im Job");
    assert_eq!((job.amount, w.stock.wood), (15, 300), "Teilaufnahme");
    assert_eq!(count(&w.events, "gathered"), 1);
    assert_eq!(w.events[0]["amount"], 10);
    for _ in 0..200 {
        w.events.clear();
        carry_to_stock(&mut w, &v, ti, Some(&r), super::testutil::DT);
        assert_eq!(count(&w.events, "gathered"), 0, "voll: kein Ereignis");
    }
    assert_eq!(
        (w.troops[ti].job.as_ref().map(|j| j.amount), w.stock.wood),
        (Some(15), 300),
        "Warten"
    );
}

/// Go: `TestIslandCarryDanger`: Bei Gefahr geht der Träger zur Burg, nicht zum nahen Lager.
#[test]
fn own_carry_gefahr_geht_zur_burg() {
    let (v, r) = (test_values(), island_one());
    let mut w = bare_world(); // Hub bei 200
    built_site(&mut w, "storage", 230.0);
    w.stock.wood = 300; // voll: der Auftrag bleibt bestehen
    let ti = carrier(&mut w, 229.0, 10, false);
    w.cycle.phase = "night".into();
    for _ in 0..60 {
        carry_to_stock(&mut w, &v, ti, Some(&r), super::testutil::DT);
    }
    assert!(
        w.troops[ti].x < 229.0,
        "Gefahr: Träger läuft zur Burg (x {})",
        w.troops[ti].x
    );
}

/// Go: `TestIslandCarryNearestDeposit`: Lager näher als die Burg, dann die Burg, Gleichstand (siehe
/// `own_deposit_point_gleichstand_burg_vor_lager`). Der Auftrag endet mit `add_pool` (RU14).
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_carry_nearest_deposit() {
    let data = test_data();
    let mut isl = must_island(&data, "trag-a", &[0]);
    let v = values(&isl.stages[0]);
    let si = storage_site(&isl.stages[0]).expect("Lager");
    let hub = isl.stages[0].hub_x;
    isl.stages[0].sites[si].state = "built".into();
    isl.stages[0].sites[si].x = hub + 30.0;
    let ti = carrier(&mut isl.stages[0], hub + 25.0, 40, true);
    for _ in 0..2000 {
        if isl.stages[0].troops[ti].job.is_none() {
            break;
        }
        on_stage(&mut isl, &data, 0, |w, r| carry_to_stock(w, &v, ti, Some(r), DT));
    }
    assert!(
        isl.stages[0].troops[ti].job.is_none() && isl.stock.wood == 40,
        "Holz {}",
        isl.stock.wood
    );
}

/// Go: `TestIslandCarryWaitsWhenFull`: der volle Ablauf mit Abgabe des Rests, sobald Platz ist.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_carry_waits_when_full() {
    let data = test_data();
    let mut isl = must_island(&data, "trag-b", &[0]);
    let v = values(&isl.stages[0]);
    isl.stock.wood = 290;
    let hub = isl.stages[0].hub_x;
    let ti = carrier(&mut isl.stages[0], hub, 25, true);
    for _ in 0..5 {
        on_stage(&mut isl, &data, 0, |w, r| carry_to_stock(w, &v, ti, Some(r), DT));
    }
    assert_eq!(isl.stock.wood, 300);
    isl.stock.wood = 100; // Platz frei: Rest wird abgegeben
    on_stage(&mut isl, &data, 0, |w, r| carry_to_stock(w, &v, ti, Some(r), DT));
    assert!(isl.stages[0].troops[ti].job.is_none() && isl.stock.wood == 115);
}

/// Go: `TestIslandCarryDanger`: dasselbe an einer gebauten Insel mit Lager bei Hub + 30.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_carry_danger() {
    let data = test_data();
    let mut isl = must_island(&data, "trag-c", &[0]);
    let v = values(&isl.stages[0]);
    let hub = isl.stages[0].hub_x;
    let si = storage_site(&isl.stages[0]).expect("Lager");
    isl.stages[0].sites[si].state = "built".into();
    isl.stages[0].sites[si].x = hub + 30.0;
    let ti = carrier(&mut isl.stages[0], hub + 29.0, 10, true);
    isl.stages[0].cycle.phase = "night".into();
    for _ in 0..2000 {
        if isl.stages[0].troops[ti].job.is_none() {
            break;
        }
        on_stage(&mut isl, &data, 0, |w, r| carry_to_stock(w, &v, ti, Some(r), DT));
    }
    assert_eq!(isl.stock.wood, 10, "Gefahr: Abgabe an der Burg");
}

/// Go: `TestIslandCarryNoIslandUnchanged`: Ohne Insel gilt kein Maximum (`carry` ohne `carry_to_stock`).
#[test]
#[ignore = "wartet auf RU18 (create_world: new_hub_site; carry ist privat in unit)"]
fn island_carry_no_island_unchanged() {
    unimplemented!("RU18: create_world, carry (Tragen ohne Insel)")
}

/// Go: `TestIslandCarryDeterministic`: Zwei Läufe ergeben dieselben Ereignisse.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_carry_deterministic() {
    let data = test_data();
    let run = || {
        let mut isl = must_island(&data, "trag-e", &[0, 1]);
        let v = values(&isl.stages[0]);
        isl.stock.wood = 280;
        let hub = isl.stages[0].hub_x;
        let ti = carrier(&mut isl.stages[0], hub + 8.0, 50, true);
        let mut ev = Vec::new();
        for _ in 0..300 {
            if isl.stages[0].troops[ti].job.is_none() {
                break;
            }
            isl.stages[0].events.clear();
            on_stage(&mut isl, &data, 0, |w, r| carry_to_stock(w, &v, ti, Some(r), DT));
            ev.extend(isl.stages[0].events.clone());
        }
        ev
    };
    assert_eq!(run(), run(), "nicht deterministisch");
}

/// Go: `TestIslandFullCarrierStillBuildsAndKeepsMaterial` (Review SP13.4): Ein wartender Träger baut und behält sein Material.
#[test]
#[ignore = "wartet auf RU18"]
fn island_full_carrier_still_builds_and_keeps_material() {
    unimplemented!("RU18: each_site (build_job), step_peasant im Bau")
}

/// Go: `TestIslandFindJobSkipsFullResource`: Ein volles Holzlager vergibt keine Sammelaufträge für Holz (`find_job` mit
/// `resource_full`, `gather_of`, `node_free`).
#[test]
#[ignore = "wartet auf RU19 (decode_world_story) und RU18 (create_world: new_hub_site)"]
fn island_find_job_skips_full_resource() {
    let data = test_data();
    let mut isl = must_island(&data, "trag-d", &[0]);
    let w = &mut isl.stages[0];
    w.cycle.phase = "day".into();
    w.enemies.clear();
    for n in &mut w.nodes {
        n.marked = true;
    }
    for s in &mut w.sites {
        s.state = "unpaid".into();
    }
    let hub = w.hub_x;
    w.troops.push(Troop {
        id: 901,
        kind: "peasant".into(),
        x: hub,
        hp: 1.0,
        max_hp: 1.0,
        ..Troop::default()
    });
    let ti = w.troops.len() - 1;
    isl.stock.wood = 300; // Bäume liefern Holz: voll
    let v = values(&isl.stages[0]);
    let job = on_stage(&mut isl, &data, 0, |w, r| {
        crate::unit::jobs::find_job(w, &v, ti, Some(r))
    });
    let wood = job.filter(|j| j.r#type == "gather").is_some_and(|j| {
        let kind = isl.stages[0]
            .nodes
            .iter()
            .find(|n| n.id == j.node_id)
            .map(|n| n.kind.clone());
        kind.and_then(|k| v.economy.gatherables.get(&k))
            .is_some_and(|g| g.resource == "wood")
    });
    assert!(
        !wood,
        "ein volles Holzlager darf keine Sammelaufträge für Holz vergeben"
    );
}

/// Go: `TestIslandFullCarrierDoesNotFetchBow`: Ein wartender Träger holt keinen Bogen.
#[test]
#[ignore = "wartet auf RU18"]
fn island_full_carrier_does_not_fetch_bow() {
    unimplemented!("RU18: step_peasant, bowyer")
}
