//! Port von `pervariant_test.go`.

use super::*;
use crate::simrun::info::Info;
use crate::simrun::run::DONE;
use crate::simrun::spec::Tune;
use crate::simrun::test_helpers::{repo_root, temp_root, wait_for, writing_registry};

/// Label, Varianten, Seeds und Bot eines Laufs in einer Zeile.
fn line(i: &Info) -> String {
    let s = i.spec.as_ref().expect("Auftrag");
    format!("{}|{:?}|{:?}|{}|{}", i.label, s.vary, s.seed_list, s.bot, s.per_variant)
}

/// B-662/AC-05: n + 1 Läufe: Basis und je Variante einer, gemeinsames Label-Präfix, gleiche Seeds.
#[test]
fn je_variante_legt_laeufe_an() {
    let (_dir, root) = temp_root();
    let g = writing_registry(r#"{"version":1,"seeds":1,"pass":true}"#);
    let vary: Vec<String> = [
        "economy.json:purse.startGold=120",
        "hub.json:tree.wood=2",
        "troops.json:archer.cost=5",
    ]
    .map(str::to_owned)
    .to_vec();
    let seeds = vec!["k3c-dev".to_owned(), "wald-1".to_owned()];
    let spec = Spec {
        seed_list: seeds.clone(),
        bot: "coop2".to_owned(),
        vary: vary.clone(),
        label: "Delta M2".to_owned(),
        per_variant: true,
        ..Spec::default()
    };
    let base = g.start_offline(&root, "Repo-Wurzel", &spec).expect("start");
    let mut want = vec![format!("Delta M2 · Basis|[]|{seeds:?}|coop2|false")];
    for v in &vary {
        want.push(format!("Delta M2 · {v}|[{v:?}]|{seeds:?}|coop2|false"));
    }
    let mut runs = g.list(&root);
    assert_eq!(base.id, runs.last().expect("Läufe").id);
    runs.reverse(); // list: neueste zuerst
    assert_eq!(runs.iter().map(line).collect::<Vec<_>>(), want);
    for r in runs {
        wait_for(&g, &root, &r.id, |i| i.state == DONE && !i.report.is_empty());
    }
}

/// B-662/AC-05: Bereiche (Abstimmung) und ein Auftrag ohne feste Variante werden abgelehnt, ohne Lauf.
#[test]
fn je_variante_lehnt_ab() {
    let (_dir, root) = temp_root();
    let g = writing_registry(r#"{"version":1,"seeds":1,"pass":true}"#);
    let tune = Tune {
        mode: "sweep".to_owned(),
        params: vec!["economy.json:purse.startGold=100..200".to_owned()],
        ..Tune::default()
    };
    let with_tune = Spec {
        vary: vec!["hub.json:tree.wood=2".to_owned()],
        per_variant: true,
        tune: Some(tune),
        ..Spec::default()
    };
    let without = Spec {
        per_variant: true,
        ..Spec::default()
    };
    for (spec, why) in [
        (with_tune, "je Variante geht nicht mit Abstimmung"),
        (without, "mindestens eine feste Variante"),
    ] {
        let err = g.start_offline(&root, "Repo-Wurzel", &spec).expect_err("abgelehnt");
        assert!(err.contains(why), "{err}, erwartet „{why}“");
    }
    assert_eq!(g.list(&root).len(), 0, "abgelehnte Aufträge legten Läufe an");
}

/// Labels bleiben in der Grenze: zu lang kürzt zuerst das Präfix, die Variante bleibt lesbar.
#[test]
fn je_variante_label_grenze() {
    let long: String = "Langes Label ".repeat(5).chars().take(MAX_LABEL).collect();
    let v = "buildings.json:castle.levels.2.cost=12".to_owned();
    let spec = Spec {
        label: long,
        vary: vec![v.clone()],
        per_variant: true,
        ..Spec::default()
    };
    let specs = spec.per_variant_specs();
    for s in &specs {
        let n = s.label.chars().count();
        assert!(n <= MAX_LABEL, "Label {:?} ({n} Zeichen)", s.label);
        assert_eq!(s.check(&repo_root()), Ok(()));
    }
    assert!(specs[1].label.ends_with(&format!(" · {v}")), "{:?}", specs[1].label);
    let plain = Spec {
        vary: vec![v],
        per_variant: true,
        ..Spec::default()
    };
    assert_eq!(plain.per_variant_specs()[0].label, "je Variante · Basis");
}
