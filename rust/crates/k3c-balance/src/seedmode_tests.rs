//! Tests zu `seedmode.rs` (Port von `engine/balance/seedmode_test.go`, B-634, BS1.1): k3c-dev steht vorn, dev rechnet
//! ohne Angabe nur ihn, ausdrückliche Seeds gehen vor.

use k3c_data::balance_targets::{FILE, Targets, load_targets};

use crate::island_world::shared;
use crate::seedmode::{DEV_SEED, SEED_MODE_ALL, SEED_MODE_DEV, SEED_MODE_PICK, SeedModes, seed_text};
use crate::summary::Summary;

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

fn file() -> String {
    shared().expect("Daten").source.read(FILE).expect("Datei der Ziele")
}

#[test]
fn dev_seed_steht_vorn() {
    let ts = load_targets(&file()).expect("Ziele");
    assert_eq!(ts.seeds[0], DEV_SEED, "seeds[0]");
}

#[test]
fn seed_modus() {
    let mut ts = Targets {
        seeds: names(&[DEV_SEED, "1", "2", "3"]),
        seed_mode: SEED_MODE_DEV.into(),
        ..Targets::default()
    };
    assert_eq!(ts.seed_list(0), names(&[DEV_SEED]), "dev ohne Angabe");
    assert_eq!(ts.seed_list(3).len(), 3, "dev mit --seeds 3");
    assert_eq!(ts.seed_mode_of(ts.seed_list(0)), SEED_MODE_DEV);
    ts.seed_mode = SEED_MODE_ALL.into();
    assert_eq!(ts.seed_list(0).len(), 4, "all ohne Angabe");
    assert_eq!(ts.seed_mode_of(ts.seed_list(2)), "auswahl");
    ts.seed_mode = String::new();
    assert_eq!(ts.seed_list(0).len(), 4, "ohne Modus wie all");
}

#[test]
fn with_seeds() {
    let ts = Targets {
        seeds: names(&[DEV_SEED, "1"]),
        seed_mode: SEED_MODE_DEV.into(),
        ..Targets::default()
    };
    let got = ts.with_seeds(&names(&["7", DEV_SEED])).expect("Seeds");
    assert_eq!(got.seed_mode_of(got.seed_list(0)), SEED_MODE_PICK);
    assert_eq!(got.seed_list(0), names(&["7", DEV_SEED]));
    assert_eq!(ts.seeds[0], DEV_SEED, "Original verändert");
    for bad in [names(&[]), names(&["7", "7"]), names(&[""])] {
        assert!(ts.with_seeds(&bad).is_err(), "{bad:?} angenommen");
    }
}

#[test]
fn all_seeds_hebt_dev_auf() {
    let ts = Targets {
        seeds: names(&[DEV_SEED, "1"]),
        seed_mode: SEED_MODE_DEV.into(),
        ..Targets::default()
    };
    assert_eq!(ts.all_seeds().seed_list(0).len(), 2);
    assert_eq!(ts.seed_list(0).len(), 1, "Original verändert");
}

#[test]
fn seed_modus_unbekannt() {
    let raw = file();
    let bad = raw.replacen("\"seedMode\": \"all\"", "\"seedMode\": \"raten\"", 1);
    assert_ne!(bad, raw, "seedMode nicht in der Datei");
    assert!(load_targets(&bad).is_err(), "seedMode raten angenommen");
}

#[test]
fn bericht_nennt_seed_modus() {
    assert_eq!(
        seed_text(1, SEED_MODE_DEV, &names(&[DEV_SEED])),
        "1 Seed, Seed-Modus dev (k3c-dev)"
    );
    assert_eq!(seed_text(100, "", &[]), "100 Seeds", "alter Bericht");
}

/// Go: `s.MarkdownWith(nil, nil)` nennt den Seed-Modus im Bericht.
#[test]
fn bericht_markdown_nennt_seed_modus() {
    let s = Summary {
        seeds: 1,
        seed_mode: SEED_MODE_DEV.into(),
        seed_names: names(&[DEV_SEED]),
        ..Summary::default()
    };
    assert_eq!(s.seed_text(), "1 Seed, Seed-Modus dev (k3c-dev)");
    assert!(
        s.markdown_with(None, None).contains("Seed-Modus dev (k3c-dev)"),
        "Markdown ohne Seed-Modus"
    );
    let old = Summary {
        seeds: 100,
        ..Summary::default()
    };
    assert_eq!(old.seed_text(), "100 Seeds", "alter Bericht");
}
