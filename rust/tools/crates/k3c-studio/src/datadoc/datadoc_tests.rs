//! Port von `datadoc_test.go`. Die Repo-Dateien werden nur gelesen.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;

use super::{BY_TESTER, FILES, Value, all_values, doc, list, values};

pub(super) fn repo_root() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../.."))
}

fn write(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    fs::create_dir_all(path.parent().expect("Ordner")).expect("anlegen");
    fs::write(path, body).expect("schreiben");
}

/// Dateinamen, die `k3c-data` als Literal lädt, samt der Ordner biomes/ und worlds/.
fn loaded_by_rust() -> BTreeSet<String> {
    let literal = Regex::new(r#""([a-z0-9-]+\.json)""#).expect("Muster");
    let mut loaded: BTreeSet<String> = ["biomes/*.json", "worlds/*.json"].map(str::to_owned).into();
    let mut folders = vec![repo_root().join("rust/crates/k3c-data/src")];
    while let Some(folder) = folders.pop() {
        for entry in fs::read_dir(folder).expect("k3c-data lesen") {
            let path = entry.expect("Eintrag").path();
            let name = path.to_string_lossy().into_owned();
            if path.is_dir() {
                folders.push(path);
            } else if name.ends_with(".rs") && !name.ends_with("_tests.rs") {
                let source = fs::read_to_string(&path).expect("Quelle");
                loaded.extend(literal.captures_iter(&source).map(|c| c[1].to_owned()));
            }
        }
    }
    loaded
}

/// WZ28.10/AC-11, seit RU41 gegen Rust: Jede Datei der Liste lädt `k3c-data` oder der Tester; jede geladene Datei steht
/// in der Liste, außer den Daten nur für das Spiel und dem Bestand beim Wechsel auf Rust.
#[test]
fn liste_wie_loader() {
    let nur_spiel = [
        // Grafik, Ton, Assets und Spiel-Einstellungen
        "paper.json",
        "sounds.json",
        "look.json",
        "render.json",
        "native.json",
        "sizes.json",
        "asset-prompts.json",
        "assets-recherche.json",
        "assets-vorschlaege.json",
        "stresstests.json",
        // Sim-Daten ohne Beschreibung unter docs/data/ (Stand RU41; der Go-Loader las sie nicht)
        "coop.json",
        "mounts.json",
        "welt-events.json",
        "festival.json",
        "cargo.json",
        "bottle.json",
        "hermits.json",
        "weather.json",
    ];
    let loaded = loaded_by_rust();
    for name in &loaded {
        let known = FILES.iter().any(|e| e.name == name) || nur_spiel.contains(&name.as_str());
        assert!(known, "k3c-data lädt {name}, die Liste kennt die Datei nicht");
    }
    for e in &FILES {
        assert!(
            loaded.contains(e.name) || e.read_by.contains(&BY_TESTER),
            "{} steht in der Liste, k3c-data lädt die Datei nicht",
            e.name
        );
        let data = repo_root().join("data");
        let there = match e.name.strip_suffix("/*.json") {
            Some(folder) => fs::read_dir(data.join(folder)).is_ok_and(|mut d| d.next().is_some()),
            None => data.join(e.name).is_file(),
        };
        assert!(there, "{} fehlt in data/", e.name);
    }
}

#[test]
fn doc_pfadpruefung() {
    let root = repo_root();
    for bad in [
        "",
        "../README",
        "economy.md",
        "Economy",
        "sprites",
        "..",
        "economy/../hub",
        r"..\x",
    ] {
        assert!(doc(&root, bad).is_err(), "doc({bad:?})");
    }
    let dir = crate::test_helpers::tmp_dir();
    let missing = doc(dir.path(), "economy").expect_err("fehlt");
    assert!(missing.contains("fehlt"), "{missing}");
    write(
        dir.path(),
        "docs/data/economy.md",
        "# economy.json\n\n- **Zweck:** Börse und Steuer.\n- **Wichtigste Werte:** `purse.startGold`\n",
    );
    let files = list(dir.path());
    assert_eq!(files.len(), FILES.len());
    assert_eq!(files[0].purpose, "Börse und Steuer.");
    assert_eq!(files[0].key, "`purse.startGold`");
    assert!(!files[0].missing && files[1].missing);
    let json = serde_json::to_string(&files[0]).expect("JSON");
    assert!(json.starts_with(r#"{"name":"economy.json","doc":"economy","readBy":["Simulation","Level-Generator"],"#));
}

#[test]
fn werte_mit_pfad_in_dateireihenfolge() {
    let dir = crate::test_helpers::tmp_dir();
    let raw = r#"{"version":1,"purse":{"startGold":100,"name":"x"},"levels":[{"cost":2.5},{"cost":-3}]}"#;
    write(dir.path(), "data/economy.json", raw);
    let value = |path: &str, value: f64| Value {
        path: path.to_owned(),
        value,
    };
    let want = vec![
        value("version", 1.0),
        value("purse.startGold", 100.0),
        value("levels.0.cost", 2.5),
        value("levels.1.cost", -3.0),
    ];
    assert_eq!(values(dir.path(), "economy.json"), Ok(want));
    for bad in [
        "biomes/*.json",
        "balance-targets.json",
        "../x.json",
        "unbekannt.json",
        "guards.json",
    ] {
        assert!(values(dir.path(), bad).is_err(), "values({bad:?})");
    }
    write(dir.path(), "data/economy.json", r#"{"a":1,"#);
    assert!(values(dir.path(), "economy.json").is_err(), "kaputtes JSON");
}

/// Als variierbar gelten genau die Dateien, die ein Balancing-Lauf neu lädt (nur dort wirken Varianten).
#[test]
fn vary_wie_der_lauf() {
    let source = fs::read_to_string(repo_root().join("rust/crates/k3c-balance/src/vary.rs")).expect("vary.rs");
    let block = Regex::new(r"(?s)pub const RELOADED: .*?\];").expect("Muster");
    let body = block.find(&source).expect("RELOADED fehlt").as_str();
    let literal = Regex::new(r#""([a-z0-9-]+\.json)""#).expect("Muster");
    let reloaded: BTreeSet<&str> = literal
        .captures_iter(body)
        .filter_map(|c| c.get(1))
        .map(|m| m.as_str())
        .collect();
    for e in &FILES {
        assert_eq!(e.vary, reloaded.contains(e.name), "{}", e.name);
    }
}

#[test]
fn alle_werte() {
    let root = repo_root();
    let all = all_values(&root).expect("alle Werte");
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for v in &all {
        *seen.entry(v.file).or_default() += 1;
    }
    for e in &FILES {
        assert_eq!(e.vary, seen.contains_key(e.name), "{}", e.name);
    }
    let economy = values(&root, "economy.json").expect("economy.json");
    assert_eq!(seen.get("economy.json"), Some(&economy.len()));
    let first = all.iter().find(|v| v.file == "economy.json").expect("Wert");
    assert_eq!(
        (&first.path, first.value.to_bits()),
        (&economy[0].path, economy[0].value.to_bits())
    );
    let empty = crate::test_helpers::tmp_dir();
    assert!(all_values(empty.path()).is_err(), "fehlende Dateien ohne Fehler");
}
