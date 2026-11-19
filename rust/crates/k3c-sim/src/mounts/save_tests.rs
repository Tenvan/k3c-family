//! Spielstand der Reittiere (RU62.8, `ISLAND_SAVE_VERSION` 11): Speichern und Laden, unbekannte ID, ältere Stände.

use super::mount_factors;
use super::testutil::{mounts_data, mounts_island, stand_at_gate, tick};
use crate::island::testutil::ISLAND_SPEED;
use crate::island::{ISLAND_SAVE_VERSION, Island, SaveTables, WorldDef, from_island_save_in, parse_island_save};
use crate::state::links::values_of;
use crate::world::vals;
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
fn reittier_uebersteht_speichern_und_laden() {
    let data = mounts_data();
    let (world, mut isl, _) = mounts_island(&data, 2);
    stand_at_gate(&mut isl, 0);
    tick(&mut isl, &data, &[0]);
    let s = isl.to_save("x").expect("Spielstand");
    assert_eq!(s.version, ISLAND_SAVE_VERSION);
    assert_eq!(ISLAND_SAVE_VERSION, 11);
    assert_eq!(s.players[0].mount, "camel");
    assert_eq!(s.players[1].mount, "");
    let json = serde_json::to_value(&s).expect("json");
    assert_eq!(json["players"][0]["mount"], "camel");
    assert!(json["players"][1].get("mount").is_none(), "ohne Reittier kein Feld");
    let loaded = reload(&data, &world, &isl);
    let w = &loaded.stages[0];
    assert_eq!(
        (w.players[0].mount.as_str(), w.players[1].mount.as_str()),
        ("camel", "")
    );
    let v = vals(w);
    assert!(mount_factors(w, &v, 0).1 > mount_factors(w, &v, 1).1);
    // das Gatter kommt aus dem Seed an dieselbe Stelle
    assert_eq!(w.mount_gate, isl.stages[0].mount_gate);
}

#[test]
fn unbekanntes_reittier_im_spielstand_scheitert() {
    let data = mounts_data();
    let (world, mut isl, _) = mounts_island(&data, 1);
    isl.stages[0].players[0].mount = "drache".to_string();
    let json = serde_json::to_string(&isl.to_save("x").expect("Spielstand")).expect("json");
    let defaults = values_of(&world).expect("Wertesatz");
    let t = tables(&data, &world, &defaults);
    let e = parse_island_save(&json, &t).expect_err("unbekannt").to_string();
    assert!(e.starts_with("spielstand:") && e.contains("drache"), "{e}");
}

#[test]
fn staende_der_version_10_bleiben_lesbar() {
    let data = mounts_data();
    let (world, isl, _) = mounts_island(&data, 1);
    let mut json = serde_json::to_value(isl.to_save("x").expect("Spielstand")).expect("json");
    json["version"] = serde_json::json!(10);
    let defaults = values_of(&world).expect("Wertesatz");
    let t = tables(&data, &world, &defaults);
    let s = parse_island_save(&json.to_string(), &t).expect("Version 10 lesen");
    assert_eq!(s.version, ISLAND_SAVE_VERSION);
    assert_eq!(s.players[0].mount, "");
}
