//! AC-04 (Sprint RU8): Ein Zustand aus einem Sim-Golden lädt in Rust und wird geschrieben als derselbe JSON-Baum.
//!
//! Gelesen werden alle Sim- und Kampagnen-Golden (`testdata/golden/sim-*.json.gz`, `campaign-*.json.gz`), nur lesend; jeder
//! Snapshot (`snapshots[i].world`) wird als `World` geladen und als JSON-Baum verglichen (`k3c_det::golden::diff`: Zahlen
//! als `f64` mit denselben Bits, `null` und leere Liste gelten als gleich). Die Spielstände der Kampagnen-Golden (`save`)
//! laufen ebenso durch `SaveGame`. Golden werden nie geschrieben.

use k3c_det::golden;
use k3c_sim::state::World;
use k3c_sim::state::save::SaveGame;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// `golden_files` sind die Golden mit Zustand: Sim- und Kampagnen-Läufe, sortiert nach Namen.
fn golden_files() -> Vec<(String, Value)> {
    let dir = golden::testdata().join("golden");
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| (n.starts_with("sim-") || n.starts_with("campaign-")) && n.ends_with(".json.gz"))
        .collect();
    names.sort();
    names
        .into_iter()
        .map(|n| (n.clone(), golden::read(dir.join(&n))))
        .collect()
}

fn roundtrip<T: DeserializeOwned + Serialize>(at: &str, v: &Value) {
    let t: T = serde_json::from_value(v.clone()).unwrap_or_else(|e| panic!("{at}: {e}"));
    let d = golden::diff(at, v, &golden::tree(&t));
    assert!(d.is_empty(), "{d}");
}

#[test]
fn alle_sim_golden_zustaende_laufen_im_kreis() {
    let (mut files, mut worlds) = (0, 0);
    for (name, g) in golden_files() {
        files += 1;
        for (i, s) in g["snapshots"].as_array().unwrap().iter().enumerate() {
            roundtrip::<World>(&format!("{name} snapshots[{i}].world"), &s["world"]);
            worlds += 1;
        }
    }
    assert!(files >= 13, "nur {files} Golden mit Zustand gefunden");
    assert!(worlds > 500, "nur {worlds} Zustände geprüft");
}

#[test]
fn kampagnen_spielstaende_laufen_im_kreis() {
    let mut saves = 0;
    for (name, g) in golden_files() {
        if g.get("save").is_some() {
            roundtrip::<SaveGame>(&format!("{name} save"), &g["save"]);
            saves += 1;
        }
    }
    assert_eq!(saves, 2);
}
