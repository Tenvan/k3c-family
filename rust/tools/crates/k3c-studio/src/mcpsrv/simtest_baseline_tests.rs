//! B-640/AC-02: `sim_test` sichert durchgelaufene Läufe als Baseline und vergleicht gegen eine benannte (Port von
//! `simtest_baseline_test.go`).

use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use super::super::simtest_spec::SimTestIn;
use super::super::test_helpers::{FakeSim, aktion, bericht_schreiben, fake_sim, nichts};
use crate::balance::baselines::BaselineInfo;
use crate::simrun::baseline::run_report;
use crate::simrun::{Proc, ProcResult};

const PASSING_REPORT: &str = r#"{"version":1,"seeds":1,"seedMode":"dev","pass":true,"targets":[{"name":"Endboss","measure":"endbossDefeated","kind":"share","players":2,"bot":"coop2","depth":0,"days":40,"world":"standard","value":100,"lower":100,"upper":100,"status":"ok","pass":true}]}"#;

type Seen = Arc<Mutex<Vec<Vec<String>>>>;

/// Lauf, der durchläuft und sich die Argumente merkt.
fn passing() -> (FakeSim, Seen) {
    let seen: Seen = Arc::default();
    let record = Arc::clone(&seen);
    let s = fake_sim(move |_, proc: Proc| {
        record.lock().expect("Liste").push(proc.args.clone());
        bericht_schreiben(&proc, PASSING_REPORT);
        ProcResult::default()
    });
    (s, seen)
}

fn coop2() -> SimTestIn {
    SimTestIn {
        bots: "coop2".to_owned(),
        ..SimTestIn::default()
    }
}

fn sichern(id: &str, name: &str) -> SimTestIn {
    SimTestIn {
        save_baseline: name.to_owned(),
        ..aktion("baseline", id)
    }
}

fn flag(args: &[String]) -> &str {
    let at = args.iter().position(|a| a == "--baseline").expect("--baseline");
    &args[at + 1]
}

#[tokio::test(flavor = "multi_thread")]
async fn sichern_und_vergleichen() {
    let (s, seen) = passing();
    let empty = s.call(aktion("baseline", "")).await.expect("leere Liste");
    assert!(empty.contains("keine gesicherten Baselines"), "{empty}");
    let id = s.start(coop2()).await;
    s.wait_done(&id).await;
    let out = s.call(sichern(&id, "M1")).await.expect("sichern");
    assert!(out.contains("Baseline M1 gesichert") && out.contains("coop2"), "{out}");
    assert!(s.root.join("reports/baseline/M1/balance.json").exists());
    let list = s.call(aktion("baseline", "")).await.expect("Liste");
    assert!(list.starts_with("M1 · "), "{list}");
    assert!(s.call(sichern(&id, "M1")).await.is_err(), "M1 wurde überschrieben");

    let second = s
        .start(SimTestIn {
            baseline: "M1".to_owned(),
            ..coop2()
        })
        .await;
    s.wait_done(&second).await;
    let seen = seen.lock().expect("Liste");
    assert_eq!(flag(&seen[seen.len() - 1]), "reports/baseline/M1/balance.json");
    assert_eq!(flag(&seen[0]), "testdata/balance/baseline.json");
}

#[tokio::test(flavor = "multi_thread")]
async fn lehnt_ab() {
    let s = fake_sim(nichts);
    let start = |baseline: &str, save: &str, mode: &str| SimTestIn {
        baseline: baseline.to_owned(),
        save_baseline: save.to_owned(),
        mode: mode.to_owned(),
        ..aktion("start", "")
    };
    let cases = [
        start("M9", "", ""),       // gibt es nicht
        start("../M1", "", ""),    // ungültiger Name
        start("M1", "", "online"), // nur offline
        start("", "M1", ""),       // nur bei action baseline
        sichern("", "M1"),         // id fehlt
        sichern("run-99", "M1"),   // Lauf unbekannt
    ];
    for input in cases {
        let shown = format!("{input:?}");
        match s.call(input).await {
            Ok(out) => panic!("{shown} angenommen: {out}"),
            Err(e) => assert!(!e.contains('\n'), "{shown}: Grund nicht in einer Zeile: {e}"),
        }
    }
    assert_eq!(s.started.load(Ordering::SeqCst), 0);
}

/// B-662/AC-03: „Als Baseline sichern“ der Balancing-Seite (`Registry::save_baseline` mit dem Berichtspfad) und
/// `sim_test {action: baseline}` gehen denselben Weg und schreiben dieselben Dateien.
#[tokio::test(flavor = "multi_thread")]
async fn seite_wie_sim_test() {
    let (s, _) = passing();
    let id = s.start(coop2()).await;
    s.wait_done(&id).await;
    s.call(sichern(&id, "MCP")).await.expect("sichern");
    let report = run_report(&s.root, &id).expect("Bericht");
    s.sim
        .registry
        .save_baseline(&s.root, &report, "SEITE", false)
        .expect("Seite");
    let read =
        |name: &str, file: &str| std::fs::read(s.root.join("reports/baseline").join(name).join(file)).expect("Datei");
    assert_eq!(read("MCP", "balance.json"), read("SEITE", "balance.json"));
    let info = |name: &str| serde_json::from_slice::<BaselineInfo>(&read(name, "run.json")).expect("run.json");
    let (mut a, b) = (info("MCP"), info("SEITE"));
    a.name.clone_from(&b.name);
    a.saved.clone_from(&b.saved);
    let (a, b) = (a.json().expect("a"), b.json().expect("b"));
    assert_eq!(a, b);
    assert!(b.contains("coop2"), "{b}");
}
