//! Port von `offline_test.go` (WZ28.4/AC-04, B-924). Falsche Engine: das Rechnen schreibt einen festen Bericht und
//! meldet Events wie der Aufruf der EXE; die Rechnung selbst prüft die Spiel-EXE.

use std::fs;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use super::offline::{Proc, ProcResult};
use super::registry::Registry;
use super::reports::newest_file;
use super::run::{CANCELED, CancelToken, DONE, FAILED, RUNNING, WAITING};
use super::spec::Spec;
use super::test_helpers::{flag, registry, temp_root, wait_for, write_report};

const REPORT: &str = concat!(
    r#"{"version":1,"seeds":2,"pass":true,"varianten":["gegner.json:a.b=2"],"targets":["#,
    r#"{"name":"Ziel","kind":"share","value":80,"lower":75,"upper":90,"pass":true}]}"#
);

/// Register, dessen Rechnen je nach `mode` einen Lauf spielt (ok, slow, crash, error).
fn fake_registry(mode: &'static str) -> Arc<Registry> {
    let g = registry();
    g.set_exec(Arc::new(move |cancel: CancelToken, p: Proc| {
        let dir = p.dir.join(flag(&p.args, "--report-dir"));
        fs::create_dir_all(&dir).expect("Ordner");
        fs::write(dir.join("args.txt"), p.args.join("\n")).expect("Argumente");
        let write = || write_report(&dir, "balance-1.json", REPORT);
        let exit = |exit| ProcResult {
            exit,
            ..ProcResult::default()
        };
        match mode {
            "slow" => {
                write();
                (p.out)("stdout", r#"{"event":"progress","done":1,"total":3}"#);
                while !cancel.wait_timeout(Duration::from_millis(5)) {}
                exit(130)
            }
            "crash" => {
                (p.out)("stdout", r#"{"event":"progress","done":1,"total":2}"#);
                (p.out)("stderr", "❌ k3c-game: 💥 Absturz in der Engine: boom");
                exit(1)
            }
            "error" => {
                (p.out)("stdout", r#"{"event":"error","message":"welt \"x\" fehlt"}"#);
                (p.out)("stderr", r#"❌ k3c-game: welt "x" fehlt"#);
                exit(1)
            }
            _ => {
                (p.out)("stdout", r#"{"event":"progress","done":1,"total":2}"#);
                (p.out)("stdout", r#"{"event":"progress","done":2,"total":2}"#);
                write();
                ProcResult::default()
            }
        }
    }));
    g
}

#[test]
fn offline_lauf_mit_fortschritt_und_bericht() {
    let (_dir, root) = temp_root();
    let g = fake_registry("ok");
    let spec = Spec {
        seeds: 2,
        depth: Some(0),
        bot: "saver".to_owned(),
        vary: vec!["gegner.json:a.b=2".to_owned()],
        label: "Probe".to_owned(),
        ..Spec::default()
    };
    let info = g.start_offline(&root, "Repo-Wurzel", &spec).expect("Start");
    // die Attrappe kann schon fertig sein, bevor der Stand gelesen ist
    assert!(info.state == RUNNING || info.state == DONE, "{info:?}");
    assert_eq!(info.label, "Probe");
    let info = wait_for(&g, &root, &info.id, |i| !i.report.is_empty());
    assert_eq!((info.state.as_str(), info.verdict.as_str()), (DONE, "Pass"), "{info:?}");
    assert_eq!((info.done, info.total), (2, 2));
    assert!(
        info.lines.iter().any(|l| l == "Pass Ziel: 80 % (75–90 %)"),
        "{:?}",
        info.lines
    );
    assert!(root.join(&info.report).exists(), "{}", info.report);
    let folder = root.join("reports").join(format!("simtest-{}", info.id));
    let args = fs::read_to_string(folder.join("args.txt")).expect("Argumente");
    for want in [
        "balance\n--targets",
        "--progress",
        "--seeds\n2",
        "--depth\n0",
        "--bot\nsaver",
        "--vary\ngegner.json:a.b=2",
    ] {
        assert!(args.contains(want), "argumente ohne {want:?}:\n{args}");
    }
}

#[test]
fn offline_abbruch_liefert_teilbericht() {
    let (_dir, root) = temp_root();
    let g = fake_registry("slow");
    let info = g.start_offline(&root, "Repo-Wurzel", &Spec::default()).expect("Start");
    // balance-1.json steht vor der Fortschrittszeile
    wait_for(&g, &root, &info.id, |i| i.done == 1 && i.total == 3);
    let begin = Instant::now();
    assert_eq!(g.stop(&root, &info.id), Ok(RUNNING));
    let info = wait_for(&g, &root, &info.id, |i| !i.report.is_empty());
    assert!(begin.elapsed() < Duration::from_secs(15));
    assert_eq!(info.state, CANCELED, "{info:?}");
    assert!(info.lines.iter().any(|l| l == "Teilbericht:"), "{:?}", info.lines);
    assert_eq!(info.verdict, "Pass");
}

#[test]
fn offline_alter_bericht_zaehlt_nicht() {
    let (_dir, root) = temp_root();
    write_report(
        &root,
        "reports/simtest-run-1/balance-0.json",
        r#"{"seeds":1,"pass":true}"#,
    );
    let dir = root.join("reports").join("simtest-run-1");
    let past = SystemTime::now() - Duration::from_secs(3600);
    let old = fs::File::options()
        .write(true)
        .open(dir.join("balance-0.json"))
        .expect("öffnen");
    old.set_modified(past).expect("Zeit setzen");
    drop(old);
    assert!(newest_file(&dir, "balance-", SystemTime::now()).is_err());
    assert!(newest_file(&dir, "balance-", past).is_ok());
}

/// RS2.2: Ein Absturz oder Fehler der Spiel-EXE beendet den Lauf als fehlgeschlagen; der Status nennt Exit und Grund.
#[test]
fn offline_absturz_nennt_stderr() {
    for (mode, want) in [
        ("crash", "💥 Absturz in der Engine: boom"),
        ("error", r#"❌ k3c-game: welt "x" fehlt"#),
    ] {
        let (_dir, root) = temp_root();
        let g = fake_registry(mode);
        let info = g.start_offline(&root, "Repo-Wurzel", &Spec::default()).expect("Start");
        let info = wait_for(&g, &root, &info.id, |i| {
            i.state != RUNNING && i.state != WAITING && !i.report.is_empty()
        });
        let text = info.lines.join("\n");
        assert_eq!(info.state, FAILED, "{mode}: {info:?}");
        assert!(
            text.contains("rechnen: Exit 1") && text.contains(want),
            "{mode}: {text}"
        );
    }
}
