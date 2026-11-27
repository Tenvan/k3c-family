//! Port von `tune_test.go` (WZ28.6): `Spec.tune` und die `tuning-*.json` in Liste, Lesen und Lauf-Ergebnis.

use std::path::Path;
use std::time::{Duration, SystemTime};

use super::reportlist::reports;
use super::reports::{read_result, read_tuning};
use super::spec::{Spec, Tune};
use super::test_helpers::{repo_root, temp_root, write_report};

fn texts(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_owned()).collect()
}

fn tune(mode: &str, params: &[&str], steps: i64, budget: i64) -> Tune {
    Tune {
        mode: mode.to_owned(),
        params: texts(params),
        steps,
        budget,
    }
}

/// `Spec.tune` wird geprüft wie vary und wird zu `--tune`, `--param`, `--steps`, `--budget`.
#[test]
fn spec_tune() {
    let root = repo_root();
    let spec = |t: Tune| Spec {
        tune: Some(t),
        ..Spec::default()
    };
    let ok = Spec {
        seeds: 1,
        ..spec(tune(
            "search",
            &[
                "economy.json:purse.startGold=50..250",
                "buildings.json:wall.levels.0.cost.gold=-1.5..2",
            ],
            3,
            4,
        ))
    };
    ok.check(&root).expect("gültig");
    let args = ok.args("dir").join(" ");
    let want = "--tune search --param economy.json:purse.startGold=50..250 \
                --param buildings.json:wall.levels.0.cost.gold=-1.5..2 --steps 3 --budget 4";
    assert!(args.ends_with(want), "{args}");
    assert!(ok.title().contains("tune search 2 Parameter"), "{}", ok.title());
    let p = "economy.json:purse.startGold=1..2";
    for bad in [
        tune("raten", &[p], 0, 0),
        tune("sweep", &[], 0, 0),
        tune("sweep", &[p, p, p], 0, 0),
        tune("sweep", &[p], 1, 0),
        tune("sweep", &[p], 22, 0),
        tune("sweep", &[p], 0, 1001),
        tune("sweep", &["economy.json:purse.startGold=1"], 0, 0),
        tune("sweep", &["economy.json:a..b=1..2"], 0, 0),
        tune("sweep", &["../economy.json:a=1..2"], 0, 0),
        tune("sweep", &["sub/economy.json:a=1..2"], 0, 0),
        tune("sweep", &["economy.json:a=1..2 --out x"], 0, 0),
    ] {
        assert!(spec(bad.clone()).check(&root).is_err(), "{bad:?} angenommen");
    }
}

const TUNING: &str = r#"{"version":1,"mode":"sweep","params":[{"file":"economy.json","path":"purse.startGold","min":50,"max":250,"base":100,"int":true}],
"points":[{"values":[150],"score":0,"pass":true,"failed":[]}],"best":{"values":[150],"score":0,"pass":true},"reason":"Alle getroffen.","diff":"+x"}"#;

/// Liest das Ergebnis und lehnt Pfade außerhalb von `reports/` und Unlesbares ab.
fn check_read_tuning(root: &Path) {
    let t = read_tuning(root, "reports/simtest-run-4/tuning-1.json").expect("Ergebnis");
    assert_eq!(t.best.values.first().map(|v| v.to_bits()), Some(150f64.to_bits()));
    assert_eq!(t.diff, "+x");
    assert_eq!(t.params.first().map(|p| p.base.to_bits()), Some(100f64.to_bits()));
    let absolute = format!("{}/reports/simtest-run-4/tuning-1.json", root.display());
    for bad in [
        "",
        "geheim.json",
        "../geheim.json",
        "reports/../geheim.json",
        &absolute,
        "reports/nichtda.json",
        "reports/simtest-run-4",
        "reports/kaputt/balance-3.json",
    ] {
        assert!(read_tuning(root, bad).is_err(), "{bad:?} gelesen");
    }
}

/// Die Liste nennt `tuning-*.json` mit `kind`, `read_tuning` liest sie mit der Pfadprüfung von `read_report`.
#[test]
fn tuning_berichte() {
    let (_dir, root) = temp_root();
    for (rel, body) in [
        (
            "reports/balance-1.json",
            r#"{"version":1,"seeds":4,"pass":false,"targets":[]}"#,
        ),
        (
            "reports/simtest-run-2/balance-2.json",
            r#"{"version":1,"seeds":2,"pass":true,"varianten":["a.json:b=1"]}"#,
        ),
        ("reports/kaputt/balance-3.json", "{"),
        ("geheim.json", "{}"),
        ("reports/simtest-run-4/tuning-1.json", TUNING),
    ] {
        write_report(&root, rel, body);
    }
    let list = reports(&root).expect("Liste");
    assert_eq!(list.len(), 3, "{list:?}");
    assert!(list.iter().all(|r| r.kind == "tuning" || r.kind == "balance"));
    let t = list.iter().find(|r| r.kind == "tuning").expect("Eintrag");
    assert_eq!(t.path, "reports/simtest-run-4/tuning-1.json");
    assert!(t.pass && t.run == "simtest-run-4", "{t:?}");
    check_read_tuning(&root);
    let dir = root.join("reports").join("simtest-run-4");
    let since = SystemTime::now() - Duration::from_secs(60);
    let (verdict, lines) = read_result(&root, &dir, since, true).expect("Lauf-Ergebnis");
    assert_eq!(verdict, "Pass");
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert_eq!(lines[1], "Alle getroffen.");
}
