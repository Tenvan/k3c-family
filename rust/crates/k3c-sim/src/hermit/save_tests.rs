//! Spielstand des Einsiedlers (RU62.10, `ISLAND_SAVE_VERSION` 12): Speichern und Laden nach der Zahlung im Bau und danach,
//! Bonus bleibt, Zerstörung merkt sich die Zahlung, Zustand für die Anzeige, ungültige und ältere Stände.

use super::testutil::{add_deep_player, count, hall, hermit_data, hermit_island, hut, pay_at_hut, run};
use super::{BUILDING_STATE, publish};
use crate::island::testutil::ISLAND_SPEED;
use crate::island::{ISLAND_SAVE_VERSION, Island, SaveTables, WorldDef, from_island_save_in, parse_island_save};
use crate::state::common::destroy_site;
use crate::state::hermit_view::HermitPhase;
use crate::state::links::values_of;
use k3c_data::Data;

fn tables<'a>(data: &'a Data, world: &'a WorldDef, defaults: &'a crate::state::WorldValues) -> SaveTables<'a> {
    SaveTables {
        data,
        defaults,
        standard_world: world,
    }
}

fn reload(data: &Data, world: &WorldDef, isl: &Island) -> Island {
    let json = serde_json::to_string(&isl.to_save("2026-11-19T00:00:00Z").expect("Spielstand")).expect("json");
    let defaults = values_of(world).expect("Wertesatz");
    let t = tables(data, world, &defaults);
    let s = parse_island_save(&json, &t).expect("lesen");
    from_island_save_in(&t, &s, ISLAND_SPEED, world).expect("laden")
}

#[test]
fn zahlung_im_bau_uebersteht_speichern_und_laden() {
    let data = hermit_data("swordsmith");
    let (world, mut isl) = hermit_island(&data, 1);
    let m = add_deep_player(&mut isl);
    pay_at_hut(&mut isl, &data, m);
    run(&mut isl, &data, 8.0);
    let s = isl.to_save("x").expect("Spielstand");
    assert_eq!(s.version, ISLAND_SAVE_VERSION);
    assert_eq!(ISLAND_SAVE_VERSION, 13);
    assert!(s.hermit_paid);
    let json = serde_json::to_value(&s).expect("json");
    assert_eq!(json["hermitPaid"], true);
    let mut loaded = reload(&data, &world, &isl);
    assert!(loaded.hermit.paid);
    let (before, after) = (hall(&isl), hall(&loaded));
    assert_eq!(after.state, BUILDING_STATE);
    assert!(
        (after.build_progress - before.build_progress).abs() < 1e-12,
        "Fortschritt bleibt"
    );
    assert_eq!(after.x, before.x);
    // die Hütte kommt aus dem Seed an dieselbe Stelle und bleibt bezahlt
    assert_eq!(hut(&loaded), hut(&isl));
    assert_eq!(
        loaded.stages[1].hermit.as_ref().map(|h| h.phase),
        Some(HermitPhase::Walking)
    );
    // und der Bau läuft nach dem Laden zu Ende
    let ev = run(&mut loaded, &data, 14.0);
    assert_eq!(count(&ev, "built"), 1);
    assert_eq!(hall(&loaded).state, "built");
}

#[test]
fn bonus_und_bezahlt_uebersteht_speichern_und_laden() {
    let data = hermit_data("swordsmith");
    let (world, mut isl) = hermit_island(&data, 1);
    let m = add_deep_player(&mut isl);
    pay_at_hut(&mut isl, &data, m);
    run(&mut isl, &data, 21.0);
    let loaded = reload(&data, &world, &isl);
    let s = hall(&loaded);
    assert_eq!((s.state.as_str(), s.hp), ("built", 250.0));
    assert!(
        loaded
            .stages
            .iter()
            .all(|w| crate::hermit::bonus::damage_factor(w) == 1.15),
        "Bonus gilt nach dem Laden"
    );
    let h = loaded.stages[0].hermit.as_ref().expect("Zustand");
    assert!(h.bonus && h.phase == HermitPhase::Built);
}

#[test]
fn zerstoertes_bauwerk_bleibt_nach_dem_laden_bezahlt() {
    let data = hermit_data("priestess");
    let (world, mut isl) = hermit_island(&data, 1);
    let m = add_deep_player(&mut isl);
    pay_at_hut(&mut isl, &data, m);
    run(&mut isl, &data, 21.0);
    let id = hall(&isl).id;
    let (wi, _) = isl.hall_site().expect("Platz");
    destroy_site(&mut isl.stages[wi], id);
    publish(&mut isl);
    let s = isl.to_save("x").expect("Spielstand");
    assert!(s.hermit_paid, "bezahlt bleibt auch ohne stehendes Bauwerk gespeichert");
    let loaded = reload(&data, &world, &isl);
    assert!(loaded.hermit.paid);
    assert_eq!(
        loaded.stages[0].hermit.as_ref().map(|h| h.phase),
        Some(HermitPhase::Fallen)
    );
}

#[test]
fn ohne_zahlung_steht_kein_feld_im_spielstand() {
    let data = hermit_data("swordsmith");
    let (world, isl) = hermit_island(&data, 1);
    let json = serde_json::to_value(isl.to_save("x").expect("Spielstand")).expect("json");
    assert!(json.get("hermitPaid").is_none());
    let loaded = reload(&data, &world, &isl);
    assert!(!loaded.hermit.paid && loaded.hermit.cfg.is_some());
    assert_eq!(hall(&loaded).state, "unpaid");
}

#[test]
fn feld_in_einer_welt_ohne_einsiedler_scheitert() {
    let data = hermit_data("swordsmith");
    let (world, isl) = hermit_island(&data, 1);
    let mut json = serde_json::to_value(isl.to_save("x").expect("Spielstand")).expect("json");
    json["hermitPaid"] = serde_json::json!(true);
    let plain = crate::island::testutil::test_data(); // `test` ohne Einsiedler
    let defaults = values_of(&world).expect("Wertesatz");
    let t = tables(&plain, &world, &defaults);
    let s = parse_island_save(&json.to_string(), &t).expect("lesen");
    let e = from_island_save_in(&t, &s, ISLAND_SPEED, &world)
        .expect_err("Fehler")
        .to_string();
    assert!(e.starts_with("spielstand:") && e.contains("Einsiedler"), "{e}");
}

#[test]
fn staende_der_version_11_bleiben_lesbar() {
    let data = hermit_data("swordsmith");
    let (world, isl) = hermit_island(&data, 1);
    let mut json = serde_json::to_value(isl.to_save("x").expect("Spielstand")).expect("json");
    json["version"] = serde_json::json!(11);
    let defaults = values_of(&world).expect("Wertesatz");
    let t = tables(&data, &world, &defaults);
    let s = parse_island_save(&json.to_string(), &t).expect("Version 11 lesen");
    assert_eq!(s.version, ISLAND_SAVE_VERSION);
    assert!(!s.hermit_paid);
}
