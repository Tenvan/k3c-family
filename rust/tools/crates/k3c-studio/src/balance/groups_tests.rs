//! Go `groups_test.go`.

use super::groups::{GROUP_BASIS, GROUP_HOLZ, GROUP_SCHIFF, is_group};
use super::hints::default_hints;
use super::targets::{default_targets, load_targets};
use super::test_helpers::repo_root;

#[test]
fn jedes_ziel_hat_genau_eine_gruppe() {
    let ts = default_targets(&repo_root()).expect("Ziele");
    for g in &ts.targets {
        assert!(is_group(&g.group), "ziel {:?}: gruppe {:?} unbekannt", g.name, g.group);
    }
    assert!(
        ts.frozen.is_some(),
        "frozen fehlt in data/balance-targets.json (leere Liste erwartet)"
    );
}

#[test]
fn jeder_hebel_hat_genau_eine_gruppe() {
    let h = default_hints(&repo_root()).expect("Richtwerte");
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

fn frozen_probe(frozen: &str) -> Vec<u8> {
    format!(r#"{{"version":1,"margin":{{}},"frozen":{frozen},"seeds":["1"],"targets":[]}}"#).into_bytes()
}

#[test]
fn frozen_wird_gelesen_und_geprueft() {
    let root = repo_root();
    let ts = default_targets(&root).expect("Ziele");
    assert!(
        ts.is_frozen(GROUP_BASIS) && ts.is_frozen(GROUP_HOLZ) && ts.is_frozen(GROUP_SCHIFF),
        "Daten frieren basis, holz und schiff ein (Baseline M5, B-656/AC-04): {:?}",
        ts.frozen
    );
    let ft = load_targets(&root, &frozen_probe(r#"["basis","schiff"]"#)).expect("laden");
    assert!(
        ft.is_frozen(GROUP_BASIS) && ft.is_frozen(GROUP_SCHIFF) && !ft.is_frozen(GROUP_HOLZ),
        "frozen falsch gelesen: {:?}",
        ft.frozen
    );
    for bad in [r#"["nix"]"#, r#"["basis","basis"]"#] {
        assert!(
            load_targets(&root, &frozen_probe(bad)).is_err(),
            "frozen {bad} muss abgelehnt werden"
        );
    }
}

#[test]
fn ziel_ohne_gruppe_in_der_datei_scheitert() {
    let mut ts = default_targets(&repo_root()).expect("Ziele");
    ts.targets[0].group.clear();
    assert!(ts.check_roles().is_err(), "ein Ziel ohne Gruppe muss abgelehnt werden");
}
