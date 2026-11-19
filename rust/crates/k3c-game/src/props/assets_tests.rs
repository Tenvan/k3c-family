//! Tests zu `assets.go`. Portiert sind `TestAssetsJeLevelOhneDoppelte` (ohne den Drawer-Teil `d.Assets`, der kommt mit
//! RU16.5), `TestJederTypAusDenDatenHatEineGrafik` (RU16.2) und `TestJedeGewaehlteGrafikLiegtVorUndIstGemeldet` (RU16.4;
//! läuft nur mit gerenderter Grafik unter `public/grafik`, sonst wie `needGrafik` in Go übersprungen).

use k3c_data::Source;
use serde_json::Value;

use super::assets::level_assets;
use super::buildings::{HUB_SCAFFOLD, Look, back_wall_look, hub_look_of, scaffold_look, site_look};
use super::nature::{
    COIN_LOOK, chest_open_look, entity_look, mine_sign_look, node_look, pickup_look, plant_tree_look, seedling_look,
    variant_look,
};
use super::plan_sites::SITE_SIZE;
use super::testutil::{fake_sheets, grafik_dir};
use crate::art::{plan_level, read_manifest};

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

/// `chosen` sind alle Grafiken, die die Auswahl-Regeln in einem Biom liefern können.
fn chosen(biome: &str) -> Vec<Look> {
    let mut out = vec![
        COIN_LOOK,
        chest_open_look(),
        seedling_look(),
        plant_tree_look(),
        HUB_SCAFFOLD,
    ];
    for (kind, _) in SITE_SIZE {
        out.push(scaffold_look(kind));
        for s in 1..=5 {
            out.push(site_look(kind, "built", s));
        }
    }
    for lv in 1..=5 {
        out.push(back_wall_look(lv));
        for step in 1..=3 {
            let h = hub_look_of(lv, step, "");
            out.push(h.main);
            out.push(h.overlay);
        }
    }
    for kind in [
        "tree",
        "rock",
        "copperOre",
        "bush",
        "stoneVein",
        "copperVein",
        "ironVein",
        "crystalVein",
    ] {
        out.push(mine_sign_look(kind));
        for x in 0..60 {
            out.push(node_look(kind, f64::from(x), biome));
        }
    }
    for variant in ["blau", "violett", "gruen"] {
        out.push(variant_look("crystalVein", variant, biome).unwrap_or_default());
    }
    for k in ["portal", "exit", "harbor", "recruitCamp", "chest", "skillPoint"] {
        out.push(entity_look(k, biome).map(|(l, _)| l).unwrap_or_default());
        out.push(pickup_look(k).0);
    }
    out
}

#[test]
fn jede_gewaehlte_grafik_liegt_vor_und_ist_gemeldet() {
    if fake_sheets().is_none() {
        return; // needGrafik: `task gen:paper` hat die Grafik nicht gerendert
    }
    let dir = grafik_dir();
    for biome in ["forest", "cave", "mine", "ironhold", "crystal"] {
        let listed = level_assets(biome);
        for l in chosen(biome) {
            if !l.ok() {
                continue;
            }
            assert!(
                listed.iter().any(|a| a == l.asset),
                "{biome}: {} nicht in Assets gemeldet",
                l.asset
            );
            let m = read_manifest(&dir, l.asset).unwrap_or_else(|e| panic!("{biome}: {e}"));
            assert!(
                m.anims.contains_key(l.anim),
                "{biome}: Animation {} fehlt in {}",
                l.anim,
                l.asset
            );
        }
        if let Err(e) = plan_level(&dir, biome, &listed) {
            panic!("{biome}: Level-Plan: {e}");
        }
    }
}

/// RU62.2 (B-997): Die Fracht-Objekte sind in data/paper.json zugeordnet; fehlt eine Zuordnung, scheitert der Test
/// (anders als die Tests mit gerenderter Grafik läuft er ohne `task gen:paper`).
#[test]
fn fracht_objekte_sind_in_paper_json_zugeordnet() {
    use super::plan_sites_cargo::{bay_asset, load_asset};
    let src = Source::from_env();
    let paper: Value = serde_json::from_str(&src.read("paper.json").expect("paper.json")).expect("paper.json JSON");
    let objekte = paper
        .get("objekte")
        .and_then(Value::as_object)
        .expect("Abschnitt objekte");
    for id in ["coins", "troops", "crates"].iter().filter_map(|k| bay_asset(k)) {
        assert!(objekte.contains_key(id), "{id} fehlt in data/paper.json › objekte");
    }
    for id in ["coins", "crate"].iter().filter_map(|k| load_asset(k)) {
        assert!(objekte.contains_key(id), "{id} fehlt in data/paper.json › objekte");
    }
}
