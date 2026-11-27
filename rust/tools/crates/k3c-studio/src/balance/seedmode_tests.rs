//! Go `seedmode_test.go` (B-634, BS1.1): k3c-dev steht vorn, dev rechnet ohne Angabe nur ihn, ausdrückliche Seeds gehen
//! vor.

use std::fs;

use super::super::summary::Summary;
use super::super::targets::{Targets, default_targets, load_targets};
use super::super::test_helpers::repo_root;
use super::{DEV_SEED, SEED_MODE_ALL, SEED_MODE_DEV, SEED_MODE_PICK};

fn list(seeds: &[&str]) -> Vec<String> {
    seeds.iter().map(|s| (*s).to_owned()).collect()
}

fn dev_targets(seeds: &[&str]) -> Targets {
    Targets {
        seeds: list(seeds),
        seed_mode: SEED_MODE_DEV.to_owned(),
        ..Targets::default()
    }
}

#[test]
fn dev_seed_steht_vorn() {
    let ts = default_targets(&repo_root()).expect("Ziele");
    assert_eq!(ts.seeds.first().map(String::as_str), Some(DEV_SEED));
}

#[test]
fn seed_modus() {
    let mut ts = dev_targets(&[DEV_SEED, "1", "2", "3"]);
    assert_eq!(ts.seed_list(0), [DEV_SEED], "dev ohne Angabe");
    assert_eq!(ts.seed_list(3).len(), 3, "dev mit --seeds 3");
    assert_eq!(ts.seed_mode_of(ts.seed_list(0)), SEED_MODE_DEV);
    ts.seed_mode = SEED_MODE_ALL.to_owned();
    assert_eq!(ts.seed_list(0).len(), 4, "all ohne Angabe");
    assert_eq!(ts.seed_mode_of(ts.seed_list(2)), "auswahl");
    ts.seed_mode = String::new();
    assert_eq!(ts.seed_list(0).len(), 4, "ohne Modus wie all");
}

#[test]
fn with_seeds() {
    let ts = dev_targets(&[DEV_SEED, "1"]);
    let got = ts.with_seeds(list(&["7", DEV_SEED])).expect("Seeds");
    assert_eq!(got.seed_mode_of(got.seed_list(0)), SEED_MODE_PICK);
    assert_eq!(got.seed_list(0), ["7", DEV_SEED]);
    assert_eq!(ts.seeds[0], DEV_SEED, "Original unverändert");
    for bad in [&[][..], &["7", "7"][..], &[""][..]] {
        assert!(ts.with_seeds(list(bad)).is_err(), "{bad:?} angenommen");
    }
}

#[test]
fn seed_modus_unbekannt() {
    let root = repo_root();
    let raw = fs::read_to_string(root.join("data/balance-targets.json")).expect("balance-targets.json");
    let bad = raw.replacen(r#""seedMode": "all""#, r#""seedMode": "raten""#, 1);
    assert_ne!(bad, raw, "seedMode nicht in der Datei");
    assert!(
        load_targets(&root, bad.as_bytes()).is_err(),
        "seedMode raten angenommen"
    );
}

#[test]
fn bericht_nennt_seed_modus() {
    let s = Summary {
        seeds: 1,
        seed_mode: SEED_MODE_DEV.to_owned(),
        seed_names: list(&[DEV_SEED]),
        ..Summary::default()
    };
    assert_eq!(s.seed_text(), "1 Seed, Seed-Modus dev (k3c-dev)");
    assert!(s.markdown_with(None, None).contains("Seed-Modus dev (k3c-dev)"));
    let old = Summary {
        seeds: 100,
        ..Summary::default()
    };
    assert_eq!(old.seed_text(), "100 Seeds", "alter Bericht");
}
