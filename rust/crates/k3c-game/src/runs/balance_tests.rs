//! Tests von `sim balance` (RU40.1): Port von `TestBalanceJSONWirdFlags`, `TestBalanceMatrixGleich`,
//! `TestAbbruchBeendetBalanceTargets` und `TestAbbruchSchreibtKeinenBericht` aus `engine/simcli/simcli_test.go`; dazu
//! Schalter, Replay und `--play`. Die Tests der Ziel- und Abstimmungsläufe stehen in `balance_targets_tests.rs`.

use std::io::Write;

use serde_json::Value;

use super::balance::SPECS;
use super::flags::{self, parse};
use super::input_balance::balance_args;
use super::stop::Stop;
use super::testutil::{WorkDir, call, data_dir};
use super::*;

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(ToString::to_string).collect()
}

/// Jede stdout-Zeile ist ein JSON-Objekt mit `event` (keine fremden Ausgaben).
pub(super) fn require_json_lines(out: &str) -> Vec<Value> {
    out.lines()
        .map(|l| {
            let v: Value = serde_json::from_str(l).expect("JSON-Zeile");
            assert!(v["event"].is_string(), "keine Zeile mit event: {l}");
            v
        })
        .collect()
}

/// Führt `balance` mit eigenem Stop und eigenem Ausgabe-Schreiber aus.
fn exec_with(list: &[&str], stop: &Stop, out: &mut dyn Write) -> u8 {
    let mut a = args(list);
    a.push("--data".into());
    a.push(data_dir());
    exec(&a, &mut std::io::empty(), out, &mut Vec::new(), stop)
}

/// Ein Schreiber, der den Abbruch auslöst, sobald die erste Zeile `progress` kommt (Go: `cancel()` im Rückruf).
struct CancelOnProgress {
    stop: Stop,
    out: Vec<u8>,
}

impl Write for CancelOnProgress {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        if String::from_utf8_lossy(b).contains("\"progress\"") {
            self.stop.trigger();
        }
        self.out.extend_from_slice(b);
        Ok(b.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Das JSON hinter `--input` wird zu Schaltern (sortiert), Schalter der Befehlszeile stehen danach.
#[test]
fn balance_json_wird_flags() {
    let json = r#"{"targets":true,"seed-list":"k3c-dev","vary":["a.json:x=1","b.json:y=2"],"seeds":2}"#;
    let got = balance_args(&args(&["--input", "-", "--days", "3"]), &mut json.as_bytes()).expect("Schalter");
    let want = "--seed-list=k3c-dev --seeds=2 --targets=true --vary=a.json:x=1 --vary=b.json:y=2 --days 3";
    assert_eq!(got.join(" "), want);
}

/// Ein Wert ohne Flag-Form (Objekt, `null`) und kaputtes JSON sind Aufruffehler.
#[test]
fn balance_json_fehler() {
    for json in [r#"{"a":{"b":1}}"#, r#"{"a":null}"#, "[1]", "{"] {
        let (code, out, _) = call(json, &["balance", "--input", "-"]);
        assert!(
            code == EXIT_USAGE && out.contains("\"event\":\"error\""),
            "{json}: Exit {code}, {out}"
        );
    }
}

/// Die Matrix (1 Seed) schreibt mit `--out` dieselben Bytes wie `k3c_balance::run::run`; Eingabe als JSON.
#[test]
fn balance_matrix_gleich() {
    let m = k3c_balance::run::Matrix {
        seeds: vec!["k3c-dev".into()],
        bots: vec!["saver".into()],
        players: vec![1],
        depths: vec![0],
        days: 1,
        ..Default::default()
    };
    let want = k3c_balance::run::run(&m).expect("Lauf").json().expect("JSON");
    let dir = WorkDir::new("matrix");
    let file = format!("{}/matrix.json", dir.path());
    let json = format!(
        r#"{{"seed-list":"k3c-dev","bots":"saver","players":"1","depths":"0","days":1,"out":{}}}"#,
        serde_json::to_string(&file).expect("Text")
    );
    let (code, out, err) = call(&json, &["balance", "--input", "-"]);
    assert_eq!(code, EXIT_OK, "{err}");
    assert!(out.is_empty(), "stdout bei --out: {out}");
    assert_eq!(std::fs::read_to_string(&file).expect("Bericht"), want);
}

/// Ohne `--out` steht der Bericht als Zeile `result`; `--replay-dir` schreibt je gültigem Lauf eine Replay-Datei, die
/// `--play` wieder abspielt.
#[test]
fn balance_result_zeile_replay_und_play() {
    let dir = WorkDir::new("replay");
    let base = [
        "balance",
        "--seed-list",
        "7",
        "--players",
        "1",
        "--days",
        "1",
        "--replay-dir",
        &dir.path(),
    ];
    let (code, out, err) = call("", &base);
    assert_eq!(code, EXIT_OK, "{err}");
    let lines = require_json_lines(&out);
    assert_eq!((lines.len(), lines[0]["event"].as_str()), (1, Some("result")));
    assert_eq!(lines[0]["report"]["results"][0]["valid"], true);
    assert_eq!(dir.files("", ".replay.json"), ["7-p1-saver-d0.replay.json"]);
    let file = format!("{}/7-p1-saver-d0.replay.json", dir.path());
    let (code, out, err) = call("", &["balance", "--play", &file]);
    assert_eq!(code, EXIT_OK, "{err}");
    let play = &require_json_lines(&out)[0]["report"];
    assert!(play["ticks"].as_i64().unwrap_or(0) > 0 && play["endHash"].as_str().is_some_and(|h| h.len() == 64));
    let (code, out, _) = call("", &["balance", "--play", &format!("{}/gibtsnicht.json", dir.path())]);
    assert!(
        code == EXIT_FAILED && out.contains("\"event\":\"error\""),
        "Exit {code}, {out}"
    );
}

/// B-924: Ein Abbruch beendet `balance --targets` nach dem nächsten fertigen Lauf; es entsteht kein Bericht. Zeile
/// `aborted`, Exit 130.
#[test]
fn abbruch_beendet_balance_targets() {
    let dir = WorkDir::new("abbruch-ziele");
    let stop = Stop::never();
    let mut out = CancelOnProgress {
        stop: stop.clone(),
        out: Vec::new(),
    };
    let code = exec_with(
        &[
            "balance",
            "--targets",
            "--seeds",
            "2",
            "--days",
            "1",
            "--progress",
            "--report-dir",
            &dir.path(),
        ],
        &stop,
        &mut out,
    );
    let text = String::from_utf8_lossy(&out.out).into_owned();
    assert_eq!(code, EXIT_ABORTED, "{text}");
    assert!(text.ends_with("{\"event\":\"aborted\"}\n"), "{text}");
    assert!(dir.files("balance-", "").is_empty(), "Bericht trotz Abbruch");
}

/// B-924: Nach einem Abbruch schreiben Matrix- und Sensitivitätsläufe keinen Bericht.
#[test]
fn abbruch_schreibt_keinen_bericht() {
    let dir = WorkDir::new("abbruch-bericht");
    let dir_s = dir.path();
    let matrix = format!("{dir_s}/matrix.json");
    let cases: [(&str, Vec<&str>); 2] = [
        (
            "matrix",
            vec!["balance", "--seeds", "2", "--days", "1", "--out", &matrix],
        ),
        ("sensitivity", vec!["balance", "--curves", "--report-dir", &dir_s]),
    ];
    for (name, list) in cases {
        let stop = Stop::never();
        stop.trigger();
        let mut out = Vec::new();
        let code = exec_with(&list, &stop, &mut out);
        assert_eq!(code, EXIT_ABORTED, "{name}: {}", String::from_utf8_lossy(&out));
        assert_eq!(String::from_utf8_lossy(&out), "{\"event\":\"aborted\"}\n", "{name}");
    }
    assert!(
        dir.files("", "").is_empty(),
        "Bericht trotz Abbruch: {:?}",
        dir.files("", "")
    );
}

/// `--stop-stdin` ohne Eingabe auf stdin oder mit Datei liest stdin als Signal; `--input -` schließt es aus.
#[test]
fn stop_stdin_nur_ohne_stdin_eingabe() {
    assert!(balance::wants_stop(&args(&["--stop-stdin"])));
    assert!(balance::wants_stop(&args(&["--stop-stdin", "--input", "datei.json"])));
    assert!(!balance::wants_stop(&args(&["--days", "1"])));
    assert!(!balance::wants_stop(&args(&["--stop-stdin", "--input", "-"])));
    let (code, out, _) = call("{}", &["balance", "--stop-stdin", "--input", "-"]);
    assert!(
        code == EXIT_USAGE && out.contains("--stop-stdin braucht --input"),
        "Exit {code}, {out}"
    );
}

/// Der Schalter-Parser folgt Gos `flag`-Paket: ein und zwei Striche, `=`, Schalter ohne Wert, wiederholbar, Fehler.
#[test]
fn schalter_wie_go() {
    let p = parse(
        SPECS,
        &args(&[
            "-days",
            "3",
            "--seeds=4",
            "--targets",
            "--vary",
            "a:b",
            "--vary=c:d,e:f",
            "--curves=false",
        ]),
    )
    .expect("Schalter");
    assert_eq!(
        (p.int("days"), p.int("seeds"), p.flag("targets"), p.flag("curves")),
        (3, 4, true, false)
    );
    assert_eq!(p.list("vary"), ["a:b", "c:d,e:f"]);
    assert!(p.is_set("days") && !p.is_set("depth") && p.int("depth") == 0);
    assert_eq!(
        (p.text("players"), p.text("report-dir")),
        ("2".to_string(), "reports".to_string())
    );
    let p = parse(SPECS, &args(&["--days", "1", "rest", "--seeds", "2"])).expect("Schalter");
    assert_eq!(p.rest, ["rest", "--seeds", "2"]);
    assert!(parse(SPECS, &args(&["--h"])).expect("Hilfe").help);
    for bad in [
        &["--gibtsnicht"][..],
        &["--days"],
        &["--days", "x"],
        &["--targets=vielleicht"],
    ] {
        assert!(matches!(parse(SPECS, &args(bad)), Err(RunError::Usage(_))), "{bad:?}");
    }
    assert!(flags::defaults_text(SPECS).contains("-gold-threshold int"));
}

/// Die Standardwerte von `--steps` und `--budget` sind die von `k3c-balance`.
#[test]
fn tune_standardwerte_aus_balance() {
    let p = parse(SPECS, &[]).expect("Schalter");
    assert_eq!(p.int("steps"), k3c_balance::tune::DEFAULT_TUNE_STEPS);
    assert_eq!(p.int("budget"), k3c_balance::tune::DEFAULT_TUNE_BUDGET);
}

/// `--speed` ohne Fenster ist ein Aufruffehler (`--window` selbst: `balance_window_tests.rs`).
#[test]
fn speed_ohne_fenster() {
    let (code, out, _) = call("", &["balance", "--speed", "2"]);
    assert!(
        code == EXIT_USAGE && out.contains("--speed gibt es nur"),
        "Exit {code}, {out}"
    );
}
