//! Tests von `runs::exec` (Port von `engine/simcli/simcli_test.go`: `TestRunGleichSimRun`, `TestLevelGleichLevelGenerate`,
//! `TestAufrufFehler`, `TestPanikWirdFehler`; nicht portiert: `TestCallErgebnisFehlerUndAbbruch`, weil `call.go` bleibt in Go,
//! und die Balance-Tests, die `balance_tests.rs` übernimmt) und des Abbruchs (Port von `cmd/k3c-sim/main_test.go`, sinngemäß ohne Prozess).

use std::io::Read;

use super::enginetools_level::{BIOMES, level};
use super::enginetools_run::{Segment, run};
use super::stop::Stop;
use super::*;

fn data_dir() -> String {
    format!("{}/../../../data", env!("CARGO_MANIFEST_DIR"))
}

fn source() -> Source {
    Source::Dir(data_dir().into())
}

/// Führt `sim` im Prozess aus: Exit-Code, stdout, stderr.
fn call(stdin: &str, args: &[&str]) -> (u8, String, String) {
    let mut a: Vec<String> = args.iter().map(ToString::to_string).collect();
    a.push("--data".into());
    a.push(data_dir());
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = exec(&a, &mut stdin.as_bytes(), &mut out, &mut err, &Stop::never());
    (
        code,
        String::from_utf8_lossy(&out).into_owned(),
        String::from_utf8_lossy(&err).into_owned(),
    )
}

/// Das Feld `text` der einzigen result-Zeile.
fn result_text(out: &str) -> String {
    let v: serde_json::Value = serde_json::from_str(out.trim()).expect("JSON-Zeile");
    assert_eq!(v["event"], "result", "{out}");
    v["text"].as_str().expect("text").to_string()
}

#[test]
fn run_gleich_sim_run() {
    let inputs = [
        Segment {
            from_tick: 0,
            to_tick: 600,
            move_x: 1.0,
            pay: true,
            ..Segment::default()
        },
        Segment {
            player: 1,
            from_tick: 30,
            to_tick: 900,
            move_x: -1.0,
            ..Segment::default()
        },
    ];
    let want = run(&source(), "forest", "k3c-dev", 900, &inputs, &Stop::never()).expect("Lauf");
    let input = r#"{"seed":"k3c-dev","ticks":900,"inputs":[{"fromTick":0,"toTick":600,"moveX":1,"pay":true},{"player":1,"fromTick":30,"toTick":900,"moveX":-1}]}"#;
    let (code, out, err) = call(input, &["run"]);
    assert_eq!(code, EXIT_OK, "{err}");
    assert_eq!(result_text(&out), want);
}

#[test]
fn level_gleich_level_generate() {
    for biome in BIOMES {
        let want = level(&source(), biome, "familie-2026").expect("Level");
        let (code, out, err) = call(&format!(r#"{{"biome":"{biome}","seed":"familie-2026"}}"#), &["level"]);
        assert_eq!(code, EXIT_OK, "{biome}: {err}");
        assert_eq!(result_text(&out), want, "{biome}");
    }
}

#[test]
fn aufruf_fehler() {
    let missing = format!("{}/fehlt.json", data_dir());
    let cases: [(&str, Vec<&str>, u8); 9] = [
        ("", vec![], EXIT_USAGE),
        ("", vec!["fliegen"], EXIT_USAGE),
        (r#"{"seed":"1","ticks":10,"tiks":5}"#, vec!["run"], EXIT_USAGE),
        ("kein json", vec!["level"], EXIT_USAGE),
        ("", vec!["run", "--input", &missing], EXIT_USAGE),
        (r#"{"seed":"1","ticks":0}"#, vec!["run"], EXIT_FAILED),
        (r#"{"biome":"mond","seed":"1"}"#, vec!["level"], EXIT_FAILED),
        (r#"{"seed":"1","ticks":10} {"seed":"2"}"#, vec!["run"], EXIT_USAGE),
        ("", vec!["run", "--gibtsnicht"], EXIT_USAGE),
    ];
    for (stdin, args, code) in cases {
        let (got, out, _) = call(stdin, &args);
        assert!(
            got == code && out.contains("\"event\":\"error\""),
            "{args:?} {stdin:?}: Exit {got} (erwartet {code}), stdout {out:?}"
        );
    }
}

/// Neue Schalter der Schnittstelle (RU33.1): Widersprüche sind Exit 2.
#[test]
fn schalter_widersprueche() {
    let cases: [(&[&str], &str); 5] = [
        (&["run", "--headless", "--window"], "schließen sich aus"),
        (&["level", "--window"], "--window gibt es nur bei run und balance"),
        (&["world", "--window"], "--window gibt es nur bei run und balance"),
        (&["run", "--stop-stdin"], "--stop-stdin braucht --input"),
        (&["balance", "--window"], "folgt mit RU40.2"),
    ];
    for (args, text) in cases {
        let (code, out, _) = call(r#"{"seed":"1","ticks":1}"#, args);
        assert!(code == EXIT_USAGE && out.contains(text), "{args:?}: Exit {code}, {out}");
    }
}

struct PanicReader;

impl Read for PanicReader {
    fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
        panic!("boom");
    }
}

#[test]
fn panik_wird_fehler() {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let args = vec!["level".to_string()];
    let code = exec(&args, &mut PanicReader, &mut out, &mut err, &Stop::never());
    let out = String::from_utf8_lossy(&out);
    assert!(
        code == EXIT_FAILED && out.contains("\"event\":\"error\"") && out.contains("Absturz in der Engine: boom"),
        "Exit {code}, stdout {out:?}"
    );
}

/// Abbruch: gesetztes Zeichen vor dem Lauf ergibt Zeile `aborted` und Exit 130, ohne `result`.
#[test]
fn abbruch_endet_mit_130() {
    let stop = Stop::never();
    stop.trigger();
    let args: Vec<String> = ["run", "--data", &data_dir()].iter().map(ToString::to_string).collect();
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = exec(
        &args,
        &mut r#"{"seed":"1","ticks":300}"#.as_bytes(),
        &mut out,
        &mut err,
        &stop,
    );
    assert_eq!(code, EXIT_ABORTED);
    assert_eq!(String::from_utf8_lossy(&out), "{\"event\":\"aborted\"}\n");
}

/// `--stop-stdin`: Das Ende von stdin setzt das Zeichen.
#[test]
fn stop_stdin_setzt_zeichen_am_ende() {
    let stop = Stop::on_end_of(std::io::empty());
    for _ in 0..200 {
        if stop.is_set() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    panic!("Zeichen nicht gesetzt");
}
