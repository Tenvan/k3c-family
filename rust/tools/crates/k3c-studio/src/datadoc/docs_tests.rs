//! Port von `docs_test.go` (WZ28.11/AC-12, B-608): Jede Datei der Liste hat eine Beschreibung unter `docs/data/` mit
//! Zweck; jeder Pfad der Aufbau-Tabelle und der wichtigsten Werte existiert in der Datei (bei Ordnern in mindestens
//! einer), jedes Varianten-Beispiel `datei.json:pfad=wert` zeigt auf eine Zahl. Nur lesend.

use std::fs;

use regex::Regex;
use serde_json::Value;

use super::datadoc_tests::repo_root;
use super::{doc, list};

/// Pfade der ersten Tabellenspalte unter „## Aufbau“.
fn section_paths(md: &str) -> Vec<String> {
    let table = Regex::new(r"^\|\s*`([^`]+)`\s*\|").expect("Muster");
    let mut inside = false;
    let mut out = Vec::new();
    for line in md.split('\n') {
        if line.starts_with("## ") {
            inside = line.trim() == "## Aufbau";
        } else if inside && let Some(c) = table.captures(line) {
            out.push(c[1].to_owned());
        }
    }
    out
}

/// Die JSON-Dateien zu einem Namen der Liste.
fn load(name: &str) -> Vec<Value> {
    let data = repo_root().join("data");
    let paths = match name.strip_suffix("/*.json") {
        Some(folder) => {
            let mut paths: Vec<_> = fs::read_dir(data.join(folder))
                .expect("Ordner")
                .map(|e| e.expect("Eintrag").path())
                .filter(|p| p.extension().is_some_and(|x| x == "json"))
                .collect();
            paths.sort();
            paths
        }
        None => vec![data.join(name)],
    };
    assert!(!paths.is_empty(), "data/{name} fehlt");
    paths
        .iter()
        .map(|p| serde_json::from_slice(&fs::read(p).expect("Datei")).expect("JSON"))
        .collect()
}

fn child<'a>(v: &'a Value, key: &str) -> Option<&'a Value> {
    match v {
        Value::Object(map) => map.get(key),
        Value::Array(list) => key.parse::<usize>().ok().and_then(|i| list.get(i)),
        _ => None,
    }
}

/// Pfad existiert; `*` passt auf jeden Schlüssel bzw. Index (mindestens einer muss weiterführen).
fn has(v: &Value, keys: &[&str]) -> bool {
    let Some((first, rest)) = keys.split_first() else {
        return true;
    };
    if *first != "*" {
        return child(v, first).is_some_and(|c| has(c, rest));
    }
    match v {
        Value::Object(map) => map.values().any(|c| has(c, rest)),
        Value::Array(list) => list.iter().any(|c| has(c, rest)),
        _ => false,
    }
}

#[test]
fn beschreibungen_vollstaendig() {
    let root = repo_root();
    let tick = Regex::new(r"`([^`]+)`").expect("Muster");
    let example = Regex::new(r"`([a-z0-9-]+\.json):([^`=]+)=[^`]+`").expect("Muster");
    let mut errors = Vec::new();
    for f in list(&root) {
        if f.missing {
            errors.push(format!(
                "docs/data/{}.md fehlt (Beschreibung für data/{})",
                f.doc, f.name
            ));
            continue;
        }
        if f.purpose.is_empty() || f.key.is_empty() {
            errors.push(format!(
                "docs/data/{}.md: Zeilen „Zweck“ und „Wichtigste Werte“ nötig",
                f.doc
            ));
        }
        let md = doc(&root, f.doc).expect("Beschreibung");
        let docs = load(f.name);
        let mut paths: Vec<(&str, String)> = section_paths(&md).into_iter().map(|p| ("Aufbau", p)).collect();
        paths.extend(
            tick.captures_iter(&f.key)
                .map(|c| ("Wichtigste Werte", c[1].to_owned())),
        );
        for (place, path) in paths {
            let keys: Vec<&str> = path.split('.').collect();
            if !docs.iter().any(|d| has(d, &keys)) {
                errors.push(format!(
                    "docs/data/{}.md ({place}): Pfad {path:?} fehlt in data/{}",
                    f.doc, f.name
                ));
            }
        }
        for c in example.captures_iter(&md) {
            if !f.vary || &c[1] != f.name {
                errors.push(format!(
                    "docs/data/{}.md: Beispiel {} gehört nicht zu data/{}",
                    f.doc, &c[0], f.name
                ));
                continue;
            }
            let value = c[2].split('.').try_fold(&docs[0], child);
            if !value.is_some_and(Value::is_number) {
                errors.push(format!(
                    "docs/data/{}.md: Beispiel {} zeigt nicht auf eine Zahl",
                    f.doc, &c[0]
                ));
            }
        }
    }
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}
