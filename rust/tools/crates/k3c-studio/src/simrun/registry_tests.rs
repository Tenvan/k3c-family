//! Port von `registry_test.go`.

use std::sync::Arc;

use super::*;
use crate::simrun::Info;
use crate::simrun::reportlist::reports;
use crate::simrun::reports::{read_report, report_path};
use crate::simrun::test_helpers::{Release, blocking, registry, repo_root, temp_root, wait_for, write_report};

/// Runner ohne `finish`: schreibt nichts in den Checkout.
fn idle(release: &CancelToken) -> Runner {
    let release = release.clone();
    Arc::new(
        move |cancel: CancelToken, _| {
            while !cancel.is_canceled() && !release.wait_timeout(std::time::Duration::from_millis(5)) {}
        },
    )
}

#[test]
fn warteschlange() {
    let (_dir, root) = temp_root();
    let g = registry();
    let release = CancelToken::new();
    let ids: Vec<String> = (0..4)
        .map(|_| {
            let run = g.start(&root, "Repo-Wurzel", "x", None, blocking(&g, &release), true);
            run.expect("start").id().to_owned()
        })
        .collect();
    let states: Vec<String> = ids.iter().map(|id| g.get(&root, id).expect("Lauf").state).collect();
    assert_eq!(states.join(","), "läuft,läuft,wartet,wartet");
    assert_eq!(g.stop(&root, &ids[2]), Ok(WAITING));
    assert!(matches!(g.get(&root, &ids[2]), Err(RegistryError::Unknown)));
    g.stop(&root, &ids[0]).expect("stop");
    wait_for(&g, &root, &ids[3], |i| i.state == RUNNING);
    assert_eq!(g.get(&root, &ids[0]).expect("Lauf").state, CANCELED);
    release.cancel();
    wait_for(&g, &root, &ids[3], |i| i.state == DONE && !i.report.is_empty());
    wait_for(&g, &root, &ids[1], |i| !i.report.is_empty());
    wait_for(&g, &root, &ids[0], |i| !i.report.is_empty());
    assert_eq!(g.list(&root).len(), 3);
    assert_eq!(g.count(), 3);
}

#[test]
fn ohne_warteschlange_abgelehnt() {
    let (_dir, root) = temp_root();
    let (_other_dir, other) = temp_root();
    let g = registry();
    let release = CancelToken::new();
    for _ in 0..MAX_PER_ROOT {
        g.start(&root, "", "x", None, idle(&release), false).expect("Platz");
    }
    let third = g.start(&root, "", "x", None, idle(&release), false);
    assert!(matches!(third, Err(RegistryError::Full)));
    assert!(g.start(&other, "", "x", None, idle(&release), false).is_ok());
    release.cancel();
}

#[test]
fn fenster_lauf_laeuft_allein_und_cancel_all_verwirft_wartende() {
    let (_dir, root) = temp_root();
    let g = registry();
    let release = CancelToken::new();
    let head = Spec {
        head: true,
        ..Spec::default()
    };
    let first = g.start(&root, "", "x", None, idle(&release), true).expect("start");
    let window = g
        .start(&root, "", "x", Some(head), idle(&release), true)
        .expect("start");
    let later = g.start(&root, "", "x", None, idle(&release), true).expect("start");
    assert_eq!(g.get(&root, window.id()).expect("Lauf").state, WAITING);
    // der später gekommene Lauf startet sofort, solange kein Fensterlauf arbeitet
    assert_eq!(g.get(&root, later.id()).expect("Lauf").state, RUNNING);
    g.cancel_all();
    assert!(matches!(g.get(&root, window.id()), Err(RegistryError::Unknown)));
    wait_for(&g, &root, first.id(), |i| i.state == FAILED);
}

#[test]
fn panik_im_runner_gibt_den_platz_frei() {
    let (_dir, root) = temp_root();
    let g = registry();
    let boom: Runner = Arc::new(|_, _| panic!("kaputt"));
    for _ in 0..=MAX_PER_ROOT {
        g.start(&root, "", "x", None, Arc::clone(&boom), true).expect("start");
    }
    for info in g.list(&root) {
        wait_for(&g, &root, &info.id, |i| i.state == FAILED);
    }
}

#[test]
fn spec_lehnt_ab() {
    // als JSON: je Zeile ein ungültiger Auftrag
    let long = format!(r#"{{"label":"{}"}}"#, "x".repeat(61));
    for bad in [
        r#"{"seeds":201}"#,
        r#"{"depth":-1}"#,
        r#"{"players":5}"#,
        r#"{"days":101}"#,
        r#"{"bot":"zauberer"}"#,
        r#"{"grade":"A B"}"#,
        r#"{"vary":["../data/x.json:a=1"]}"#,
        r#"{"vary":["x.json:a=1 2"]}"#,
        r#"{"vary":["x.txt:a=1"]}"#,
        r#"{"vary":["x.json:a"]}"#,
        &long,
        r#"{"label":"a\nb"}"#,
        r#"{"speed":2}"#,
        r#"{"helps":"voll"}"#,
        r#"{"seedList":["a","a"]}"#,
        r#"{"tune":{"mode":"sweep","params":["x.json:a=1"]}}"#,
    ] {
        let s: Spec = serde_json::from_str(bad).expect("Auftrag");
        assert!(s.check(&repo_root()).is_err(), "{bad} angenommen");
    }
    let good = r#"{"seeds":3,"bot":"saver","grade":"leicht","vary":["gegner.json:wolf.hp=12.5"]}"#;
    let good: Spec = serde_json::from_str(good).expect("Auftrag");
    assert_eq!(good.check(&repo_root()), Ok(()));
}

/// Zwei gültige Berichte, ein kaputter und eine Datei außerhalb von `reports/`.
fn reports_fixture() -> (tempfile::TempDir, PathBuf) {
    let (dir, root) = temp_root();
    write_report(
        &root,
        "reports/balance-1.json",
        r#"{"version":1,"seeds":4,"pass":false,"targets":[]}"#,
    );
    write_report(
        &root,
        "reports/simtest-run-2/balance-2.json",
        r#"{"version":1,"seeds":2,"pass":true,"varianten":["a.json:b=1"]}"#,
    );
    write_report(&root, "reports/kaputt/balance-3.json", "{");
    write_report(&root, "geheim.json", "{}");
    (dir, root)
}

#[test]
fn bericht_pfadpruefung() {
    let (_dir, root) = reports_fixture();
    let absolute = format!("{}/reports/balance-1.json", root.display());
    for bad in [
        "",
        "geheim.json",
        "../geheim.json",
        "reports/../geheim.json",
        "reports/x.md",
        &absolute,
        "/reports/balance-1.json",
        "reports/nichtda.json",
        "reports/simtest-run-2",
        "reports/../../geheim.json",
        r"\\server\reports\x.json",
        "c:reports/balance-1.json",
    ] {
        assert!(read_report(&root, bad).is_err(), "{bad:?} gelesen");
    }
    // ein Laufwerk mitten im Pfad ersetzt die Wurzel nicht
    let inside = report_path(&root, "reports/c:/x.json").expect("Pfad");
    assert!(inside.starts_with(&root), "{inside:?}");
    let sum = read_report(&root, "reports/simtest-run-2/balance-2.json").expect("Bericht");
    assert!(sum.seeds == 2 && sum.pass, "{sum:?}");
}

#[test]
fn berichte_liste() {
    let (_dir, root) = reports_fixture();
    let list = reports(&root).expect("Liste");
    assert_eq!(list.len(), 2, "{list:?}");
    for r in list {
        if r.path == "reports/simtest-run-2/balance-2.json" {
            assert!(r.run == "simtest-run-2" && r.varianten.len() == 1, "{r:?}");
        } else {
            assert_eq!((r.path.as_str(), r.run.as_str()), ("reports/balance-1.json", ""));
        }
    }
}

#[test]
fn plaetze_werden_belegt() {
    let busy = |root: &str, slot: usize| {
        let run = Run::new(String::new(), Path::new(root), "", "", None, None);
        {
            let mut s = run.state();
            s.active = true;
            s.slot = slot;
        }
        Arc::new(run)
    };
    let g = Inner {
        next: 0,
        runs: vec![busy("r", 0), busy("x", 1)],
        notify: Notify::default(),
    };
    assert_eq!(free_slot(&g, Path::new("r")), 1);
    assert_eq!(free_slot(&g, Path::new("x")), 0);
}

/// B-616 AC-01: Nach einem neuen Register im selben Checkout beginnt die nächste ID hinter dem höchsten Ordner.
#[test]
fn lauf_id_nach_neustart_hinter_hoechstem_ordner() {
    let (_dir, root) = temp_root();
    for d in ["simtest-run-2", "simtest-run-7", "simtest-run-x"] {
        std::fs::create_dir_all(root.join("reports").join(d)).expect("anlegen");
    }
    let g = registry();
    let release = CancelToken::new();
    let run = g
        .start(&root, "Repo-Wurzel", "x", None, blocking(&g, &release), true)
        .expect("start");
    let _release = Release {
        g: Arc::clone(&g),
        run: Arc::clone(&run),
        release,
    };
    assert_eq!(run.id(), "run-8");
}

#[test]
fn kopfzeile_und_status_wie_go() {
    let started = chrono::Local::now();
    let info = Info {
        id: "run-1".to_owned(),
        label: "offline·balance".to_owned(),
        place: "Repo-Wurzel".to_owned(),
        state: RUNNING.to_owned(),
        phase: "rechnen".to_owned(),
        done: 0,
        total: 0,
        started,
        ended: None,
        verdict: String::new(),
        report: "reports/simtest-run-1/simtest.md".to_owned(),
        lines: (1..=5).map(|n| n.to_string()).collect(),
        spec: None,
    };
    let now = started + chrono::TimeDelta::milliseconds(65_400);
    let head = "▶ run-1 · offline·balance · läuft · 1m5s · Repo-Wurzel · rechnen";
    assert_eq!(info.head(now), head);
    let tail = "Bericht: reports/simtest-run-1/simtest.md";
    assert_eq!(
        info.status(now, 5),
        format!("{head}\n1\n2\n… 3 weitere im Bericht\n{tail}")
    );
    let json = serde_json::to_value(&info).expect("JSON");
    assert_eq!(json["where"], "Repo-Wurzel");
    assert!(json.get("ended").is_none() && json.get("spec").is_none());
}
