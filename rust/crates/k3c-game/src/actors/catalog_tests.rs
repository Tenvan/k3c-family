//! Port von `native/k3c-game/actors/catalog_test.go`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::*;

// Pfade relativ zum Crate (Go: relativ zum Paket).
fn repo_grafik() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/grafik")
}

fn repo_rigs() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/rig")
}

fn load() -> Catalog {
    load_catalog(&Source::Embedded).unwrap()
}

/// dataKinds liest die Schlüssel einer Daten-Datei (troops.json, enemies.json), sortiert.
fn data_kinds(name: &str) -> Vec<String> {
    let raw = Source::Embedded.read(name).unwrap();
    let m: BTreeMap<String, serde_json::Value> = serde_json::from_str(&raw).unwrap();
    m.into_keys().collect()
}

/// specRig liest den Rig-Typ der Papier-Spec eines Assets (figur, vierbeiner, requisite).
fn spec_rig(asset: &str) -> String {
    let path = repo_grafik().join(format!("{asset}.paper.json"));
    let raw = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Spec fehlt: {e}"));
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    v["rig"].as_str().unwrap_or("").to_string()
}

/// checkLook: Rig-Typ der Spec passt zu Sheet, Rig-Datei genau ohne Sheet, Höhe gesetzt (das Reittier trägt den
/// Maßstab des Reiters und hat keine eigene).
fn check_look(name: &str, l: &Look, height: bool) {
    let requisite = spec_rig(&l.asset) == "requisite";
    assert_eq!(
        requisite, l.sheet,
        "{name} ({}): Sheet {}, Spec requisite {requisite}",
        l.asset, l.sheet
    );
    let has_rig = repo_rigs().join(&l.asset).join("rig.json").exists();
    assert_ne!(
        has_rig, l.sheet,
        "{name} ({}): Rig-Datei vorhanden = {has_rig}, Sheet = {}",
        l.asset, l.sheet
    );
    if height {
        assert!(l.height > 0.0, "{name}: ohne Höhe");
    }
}

/// AC-03 (B-796/AC-02): Jeder Figurentyp aus data/ hat eine Zuordnung, Figuren mit Teile-Baum ein Rig, `requisite`
/// ein Sheet; die Liste sheetAssets stimmt mit den Specs überein.
#[test]
fn every_figure_has_rig_or_sheet() {
    let c = load();
    check_look(
        "reittier",
        &Look {
            asset: c.mount.clone(),
            ..Look::default()
        },
        false,
    );
    let mut looks: BTreeMap<String, Look> = BTreeMap::new();
    looks.insert("monarch".to_string(), c.player(0));
    for k in data_kinds("troops.json") {
        let l = c.troops.get(&k).unwrap_or_else(|| panic!("Truppe {k} ohne Look"));
        looks.insert(format!("truppe {k}"), l.clone());
    }
    for k in data_kinds("enemies.json") {
        let l = c.enemy(&k, false).unwrap_or_else(|| panic!("Gegner {k} ohne Look"));
        looks.insert(format!("gegner {k}"), l);
    }
    for (k, l) in &c.bosses {
        looks.insert(format!("boss {k}"), l.clone());
    }
    for (name, l) in &looks {
        check_look(name, l, true);
    }
    // Endbosse ohne eigenes Asset zeigen die Basisart (B-993): jeder Eintrag in bossAssets ist ein Boss der Daten.
    for (id, _) in BOSS_ASSETS {
        assert!(c.bosses.contains_key(id), "bossAssets kennt {id:?}, die Daten nicht");
    }
}

#[test]
fn catalog_values() {
    let c = load();
    assert!(
        c.mount == "reittiere/pferd" && c.mount_ref == 5.0,
        "Reittier {:?}, Tempo {}",
        c.mount,
        c.mount_ref
    );
    let p = c.player(1);
    assert!(
        p.asset == "figuren/monarch" && p.variant == "spieler2" && p.height == 121.5 / UNIT_PX,
        "Spieler 2: {p:?}"
    );
    let b = c.enemy("bat", false);
    assert!(
        b.as_ref().is_some_and(|b| b.lift == 130.0 / UNIT_PX && b.sheet),
        "Fledermaus: {b:?}"
    );
    let g = c.enemy("mineGhost", false);
    assert!(g.as_ref().is_some_and(|g| g.alpha == 0.85), "Grubengeist: {g:?}");
    assert!(c.enemy("goblinLeader", true).is_some(), "Boss goblinLeader fehlt");
    assert!(c.enemy("gibtsnicht", false).is_none(), "unbekannter Gegner gefunden");
    let mut want: Vec<String> = SHEET_ASSETS.iter().map(|s| (*s).to_string()).collect();
    want.sort_unstable();
    assert_eq!(c.sheet_assets(), want, "Sheets");
}

#[test]
fn catalog_reports_gaps() {
    let files: BTreeMap<&str, &str> = BTreeMap::from([
        (
            "paper.json",
            r#"{"spieler": ["figuren/monarch"], "reittiere": {}, "truppen": {"vagrant": "truppen/landstreicher"},
				"figuren": {"spieler": [], "truppen": {}}}"#,
        ),
        (
            "monarch.json",
            r#"{"base": {"speed": 5}, "mount": {"sprite": "horse", "speedFactor": 1}}"#,
        ),
        ("bosses.json", r#"{"bosses": [{"id": "neu"}], "endBosses": []}"#),
        ("enemies.json", r#"{"notes": "", "wolf": {"depth": 0}}"#),
    ]);
    let read = |name: &str| {
        files
            .get(name)
            .map(|s| (*s).to_string())
            .ok_or_else(|| Fehler::Lesen(format!("data/{name}: open {name}: file does not exist")))
    };
    let err = load_catalog_from(&read).expect_err("Lücken nicht gemeldet").to_string();
    for want in [
        "reittiere",
        "figuren.spieler",
        "figuren.truppen.vagrant",
        "Boss \"neu\"",
    ] {
        assert!(err.contains(want), "Meldung ohne {want:?}: {err}");
    }
}
