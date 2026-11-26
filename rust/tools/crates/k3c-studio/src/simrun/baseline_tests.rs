//! Port von `baseline_test.go`.

use std::sync::Arc;
use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

use super::*;
use crate::simrun::notify::Notify;
use crate::simrun::run::CancelToken;
use crate::simrun::spec::Spec;
use crate::simrun::test_helpers::{Release, blocking, registry, temp_root, wait_for, write_report, writing_registry};

/// Ein durchgelaufener Bericht mit gültiger Messung (wie in `mcpsrv/simtest_baseline_test.go`).
const PASSING_REPORT: &str = concat!(
    r#"{"version":1,"seeds":1,"seedMode":"dev","pass":true,"targets":["#,
    r#"{"name":"Endboss","measure":"endbossDefeated","kind":"share","players":2,"bot":"coop2","depth":0,"days":40,"world":"standard","value":100,"lower":100,"upper":100,"status":"ok","pass":true}]}"#
);

/// Wartet auf `n` Meldungen „Berichte geändert“.
fn drain(rx: &Receiver<()>, n: usize) {
    for _ in 0..n {
        rx.recv_timeout(Duration::from_secs(5)).expect("zu wenige Meldungen");
    }
}

fn spec_text(info: &BaselineInfo) -> &str {
    info.spec.as_ref().map_or("", |s| s.get())
}

/// B-662/AC-03: Der Auftrag kommt vom Lauf oder, bei einem alten Lauf ohne Eintrag im Register, aus dem Bericht. Beides
/// meldet „Berichte geändert“.
#[test]
fn baseline_aus_bericht() {
    let (_dir, root) = temp_root();
    let g = writing_registry(PASSING_REPORT);
    let (tx, rx) = channel();
    g.set_notify(Notify {
        run: None,
        reports: Some(Arc::new(move |_| {
            let _ = tx.send(());
        })),
    });
    let spec = Spec {
        seeds: 1,
        bot: "coop2".to_owned(),
        label: "M1-Lauf".to_owned(),
        ..Spec::default()
    };
    let info = g.start_offline(&root, "Repo-Wurzel", &spec).expect("start");
    wait_for(&g, &root, &info.id, |i| i.state == DONE && !i.report.is_empty());
    let rel = format!("reports/simtest-{}/balance-1.json", info.id);
    assert_eq!(run_report(&root, &info.id), Ok(rel.clone()));
    let saved = g.save_baseline(&root, &rel, "M1", false).expect("sichern");
    assert_eq!(saved.run, format!("simtest-{}", info.id));
    assert!(spec_text(&saved).contains(r#""coop2""#), "{saved:?}");
    for f in ["balance.json", "run.json"] {
        assert!(root.join("reports/baseline/M1").join(f).is_file(), "{f} fehlt");
    }
    drain(&rx, 2); // Lauf fertig, Baseline gesichert

    let old = "reports/simtest-run-7/balance-1.json";
    let body = format!(
        r#"{{"label":"alt","runId":"run-7","spec":{{"bot":"saver","days":30}},{}"#,
        &PASSING_REPORT[1..]
    );
    write_report(&root, old, &body);
    let saved = g.save_baseline(&root, old, "M2", false).expect("alter Bericht");
    assert!(spec_text(&saved).contains(r#""saver""#), "{saved:?}");
    let raw = std::fs::read(root.join("reports/baseline/M2/run.json")).expect("run.json");
    let meta: serde_json::Value = serde_json::from_slice(&raw).expect("JSON");
    assert_eq!(meta["run"], "simtest-run-7");
}

/// B-662/AC-03: Fremde Pfade, laufende Läufe, ungültige und vergebene Namen mit Grund ablehnen.
#[test]
fn baseline_lehnt_ab() {
    let (_dir, root) = temp_root();
    let g = registry();
    let saved = "reports/simtest-run-7/balance-1.json";
    write_report(&root, saved, PASSING_REPORT);
    g.save_baseline(&root, saved, "M1", false).expect("sichern");
    let release = CancelToken::new();
    let running = g
        .start(&root, "Repo-Wurzel", "x", None, blocking(&g, &release), true)
        .expect("start");
    let _release = Release {
        g: Arc::clone(&g),
        run: Arc::clone(&running),
        release,
    };
    let own = format!("reports/simtest-{}/balance-1.json", running.id());
    write_report(&root, &own, PASSING_REPORT);
    write_report(&root, "reports/balance-1.json", PASSING_REPORT);
    for (path, name, why) in [
        (saved, "M1", "gibt es schon"),
        (saved, "../M3", "Buchstaben"),
        ("reports/balance-1.json", "M3", "Ordner eines Laufs"),
        ("../x/balance-1.json", "M3", "reports/"),
        ("reports/simtest-run-99/balance-1.json", "M3", "gibt es nicht"),
        (own.as_str(), "M3", "läuft"),
    ] {
        let err = g.save_baseline(&root, path, name, false).expect_err("abgelehnt");
        assert!(err.contains(why), "{path} als {name}: {err}, erwartet „{why}“");
    }
}

#[test]
fn bericht_eines_laufs_ist_kein_pfad() {
    let (_dir, root) = temp_root();
    write_report(&root, "reports/simtest-run-1/balance-1.json", "{}");
    write_report(&root, "reports/simtest-run-1/balance-2.json", "{}");
    assert_eq!(
        run_report(&root, "run-1"),
        Ok("reports/simtest-run-1/balance-2.json".to_owned())
    );
    for bad in ["run-2", "x/../simtest-run-1", r"x\..\simtest-run-1", "c:x"] {
        assert!(run_report(&root, bad).is_err(), "{bad}");
    }
}
