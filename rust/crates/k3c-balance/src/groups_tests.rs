//! Tests zu `groups.rs` (Port von `engine/balance/groups_test.go`).

use k3c_data::balance_targets::{FILE, load_targets};

use crate::groups::{GROUP_BASIS, GROUP_HOLZ, GROUP_SCHIFF, is_frozen, is_group};
use crate::hints::default_hints;
use crate::island_world::shared;

/// Datei der Ziele mit einstellbarer `frozen`-Liste (Go: `frozenProbe`). Rust prüft beim Laden schon die Rollen
/// (`check_roles`, Go erst `DefaultTargets`), darum steht ein gültiges Ziel darin.
fn probe(frozen: &str) -> String {
    let target = r#"{"measure":"castleHeld","group":"basis","name":"x","kind":"share","players":2,"bot":"saver","depth":0,"days":5,"role":"goal","lower":70,"upper":90,"rule":"r"}"#;
    format!(r#"{{"version":1,"margin":{{}},"frozen":{frozen},"seeds":["1"],"targets":[{target}]}}"#)
}

fn default_targets_raw() -> String {
    shared().expect("Daten").source.read(FILE).expect("Datei der Ziele")
}

#[test]
fn jedes_ziel_hat_genau_eine_gruppe() {
    let ts = load_targets(&default_targets_raw()).expect("Ziele");
    for g in &ts.targets {
        assert!(is_group(&g.group), "ziel {:?}: gruppe {:?} unbekannt", g.name, g.group);
    }
}

#[test]
fn jeder_hebel_hat_genau_eine_gruppe() {
    let h = default_hints().expect("Richtwerte");
    let mut n = 0;
    for (key, l) in &h.levers {
        for lv in l.low.iter().chain(&l.high) {
            n += 1;
            assert!(
                is_group(&lv.group),
                "hebel {key} {}: gruppe {:?} unbekannt",
                lv.path,
                lv.group
            );
        }
    }
    assert!(n > 0, "keine Hebel gefunden");
}

#[test]
fn frozen_wird_gelesen_und_geprueft() {
    let ts = load_targets(&default_targets_raw()).expect("Ziele");
    assert!(
        is_frozen(&ts, GROUP_BASIS) && is_frozen(&ts, GROUP_HOLZ) && is_frozen(&ts, GROUP_SCHIFF),
        "Daten frieren basis, holz und schiff ein (Baseline M5, B-656/AC-04): {:?}",
        ts.frozen
    );
    let ft = load_targets(&probe(r#"["basis","schiff"]"#)).expect("Probe");
    assert!(
        is_frozen(&ft, GROUP_BASIS) && is_frozen(&ft, GROUP_SCHIFF) && !is_frozen(&ft, GROUP_HOLZ),
        "frozen falsch gelesen: {:?}",
        ft.frozen
    );
    for bad in [r#"["nix"]"#, r#"["basis","basis"]"#] {
        assert!(load_targets(&probe(bad)).is_err(), "frozen {bad} muss abgelehnt werden");
    }
}

#[test]
fn ziel_ohne_gruppe_in_der_datei_scheitert() {
    let mut v: serde_json::Value = serde_json::from_str(&default_targets_raw()).expect("JSON");
    v["targets"][0]["group"] = serde_json::Value::String(String::new());
    assert!(
        load_targets(&v.to_string()).is_err(),
        "ein Ziel ohne Gruppe muss abgelehnt werden"
    );
}
