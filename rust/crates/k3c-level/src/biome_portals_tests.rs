use serde_json::Value;

use crate::biome::load_biome;
use crate::testutil::{biome_ids, data_dir};

/// Das Biom `forest` als JSON-Baum, `portals` nach `f` verändert; `load_biome` ist der Lader samt `checkPortals`.
fn laden(f: impl FnOnce(&mut Value)) -> Result<(), String> {
    let raw = std::fs::read_to_string(data_dir().join("biomes/forest.json")).expect("forest.json");
    let mut v: Value = serde_json::from_str(&raw).expect("JSON");
    f(&mut v["portals"]);
    load_biome("forest", &v.to_string())
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Go ruft `b.checkPortals()` auf einem leeren Biom; die Prüfung ist in `k3c-data` privat, hier über den Lader.
#[test]
fn portal_werte_sind_pflicht() {
    let ohne_hp = laden(|p| {
        p.as_object_mut().expect("portals").remove("hp");
    });
    assert!(
        ohne_hp.is_err_and(|e| e.contains("portals.hp")),
        "Biom ohne Portal-HP geladen"
    );
    let ohne_takt = laden(|p| {
        p["hp"] = 100.into();
        p["defense"] = Value::Null;
    });
    assert!(ohne_takt.is_err(), "Biom ohne Verteidiger-Takt geladen");
    let ohne_takt = laden(|p| {
        p["hp"] = 100.into();
        p["defense"]["intervalSeconds"] = 0.into();
    });
    assert!(
        ohne_takt.is_err_and(|e| e.contains("portals.defense.intervalSeconds")),
        "Takt 0 geladen"
    );
    laden(|p| {
        p["hp"] = 100.into();
        p["defense"]["intervalSeconds"] = 20.into();
    })
    .expect("gültige Portal-Werte");
    for id in biome_ids() {
        let raw = std::fs::read_to_string(data_dir().join(format!("biomes/{id}.json"))).expect("Biom");
        load_biome(&id, &raw).unwrap_or_else(|e| panic!("{id}: {e}"));
    }
}
