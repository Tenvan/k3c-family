//! Tests zu `assets.go`. Portiert ist `TestAssetsJeLevelOhneDoppelte` (ohne den Drawer-Teil `d.Assets`, der kommt mit
//! RU16.5). `TestJederTypAusDenDatenHatEineGrafik` ist seit RU16.2 portiert; `TestJedeGewaehlteGrafikLiegtVorUndIstGemeldet`
//! braucht `site_size` (RU16.3) und die Manifeste (`art`) und folgt dort.

use k3c_data::Source;
use serde_json::Value;

use super::assets::level_assets;
use super::nature::{entity_look, node_look, pickup_look};

#[test]
fn assets_je_level_ohne_doppelte() {
    let wald = level_assets("forest");
    let hoehle = level_assets("cave");
    let mut sortiert = wald.clone();
    sortiert.sort();
    sortiert.dedup();
    assert_eq!(sortiert, wald, "sortiert, jedes einmal");
    let has = |v: &[String], a: &str| v.iter().any(|x| x == a);
    assert!(
        has(&wald, "objekte/tanne") && !has(&hoehle, "objekte/tanne") && has(&hoehle, "objekte/wurzel"),
        "Natur nach dem Look des Bioms"
    );
}

/// AC-03: Jeder Natur- und Objekt-Typ aus data/ hat eine Grafik (Knoten-Arten aus data/economy.json und den Biomen,
/// Ereignis-Chunks als Entität oder Pickup).
#[test]
fn jeder_typ_aus_den_daten_hat_eine_grafik() {
    let src = Source::from_env();
    let eco: Value = serde_json::from_str(&src.read("economy.json").expect("economy.json")).expect("economy.json JSON");
    let keys = |v: &Value, k: &str| -> Vec<String> {
        v.get(k)
            .and_then(Value::as_object)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    };
    let eco_kinds: Vec<String> = keys(&eco, "gatherables")
        .into_iter()
        .chain(keys(&eco, "veins"))
        .collect();
    for id in k3c_data::data::biome_ids(&src).expect("biome_ids") {
        let raw = src.read(&format!("biomes/{id}.json")).expect("Biom lesen");
        let b: Value = serde_json::from_str(&raw).expect("Biom JSON");
        let mut kinds = eco_kinds.clone();
        if let Some(chunks) = b.get("resourcesPerChunk").and_then(Value::as_object) {
            for res in chunks.values().filter_map(Value::as_object) {
                kinds.extend(res.keys().cloned());
            }
        }
        for k in &kinds {
            assert!(node_look(k, 3.0, &id).ok(), "{id}: Knoten {k} ohne Grafik");
        }
        for k in keys(&b, "eventChunks") {
            assert!(
                entity_look(&k, &id).is_some() || pickup_look(&k).0.ok(),
                "{id}: Ereignis {k} ohne Grafik"
            );
        }
    }
    for k in ["portal", "exit", "harbor", "recruitCamp", "bush"] {
        assert!(entity_look(k, "forest").is_some(), "Entität {k} ohne Grafik");
    }
    for k in ["chest", "skillPoint"] {
        assert!(pickup_look(k).0.ok(), "Pickup {k} ohne Grafik");
    }
}
