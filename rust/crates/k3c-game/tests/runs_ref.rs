//! RU33.1 (AC-02): Die Läufe des Spiels (`k3c_game::runs::exec`, im Prozess) geben für dieselben Aufrufe dieselben JSON-Zeilen
//! und Exit-Codes aus wie `k3c-sim` (Go). Die Vergleichsdateien schreibt `go test ./engine/simcli/ -run RustRef -write`
//! nach `testdata/rust/runs/`. Bei `error`-Zeilen, deren Text aus Gos `flag`-Paket oder JSON-Fehlern stammt, zählen nur
//! Zeile `event` und Code.

#![allow(clippy::expect_used)]

use k3c_game::runs::{self, gojson, stop::Stop};
use serde_json::Value;

fn root() -> String {
    format!("{}/../../..", env!("CARGO_MANIFEST_DIR"))
}

fn lines(out: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(out).lines().map(ToString::to_string).collect()
}

fn event(line: &str) -> String {
    let v: Value = serde_json::from_str(line).expect("JSON-Zeile");
    v["event"].as_str().unwrap_or_default().to_string()
}

/// Rust kennt die Basis-Datei `welt-events` (RU62), Go nicht: Die Zeile `world values` verliert sie vor dem Vergleich.
fn without_welt_events(line: &str) -> String {
    let report = gojson::raw_field(line, "report").unwrap_or_default();
    let base = gojson::raw_field(report, "base").unwrap_or_default();
    match gojson::raw_entries(base).into_iter().find(|(k, _)| k == "welt-events") {
        Some((_, raw)) => line.replacen(&format!(",\"welt-events\":{raw}"), "", 1),
        None => line.to_string(),
    }
}

/// Die erste Stelle, an der zwei Zeilen auseinandergehen, mit Umgebung.
fn first_diff(got: &str, want: &str) -> String {
    let at = got.chars().zip(want.chars()).take_while(|(a, b)| a == b).count();
    let cut = |s: &str| s.chars().skip(at.saturating_sub(60)).take(160).collect::<String>();
    format!("ab Zeichen {at}\n  Rust: {}\n  Go:   {}", cut(got), cut(want))
}

#[test]
fn laeufe_gleich_go() {
    let dir = format!("{}/testdata/rust/runs", root());
    let mut names: Vec<_> = std::fs::read_dir(&dir)
        .expect("Vergleichsdateien (go test ./engine/simcli/ -run RustRef -write)")
        .filter_map(|e| e.ok()?.file_name().to_str().map(ToString::to_string))
        .filter(|n| n.ends_with(".json"))
        .collect();
    names.sort();
    assert!(names.len() >= 8, "nur {} Fälle", names.len());
    for name in names {
        let want: Value =
            serde_json::from_str(&std::fs::read_to_string(format!("{dir}/{name}")).expect("Datei")).expect("JSON");
        let mut args: Vec<String> = want["args"]
            .as_array()
            .expect("args")
            .iter()
            .map(|a| a.as_str().expect("Text").to_string())
            .collect();
        if !args.is_empty() {
            args.push("--data".into());
            args.push(format!("{}/data", root()));
        }
        let stdin = want["stdin"].as_str().expect("stdin").to_string();
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = runs::exec(&args, &mut stdin.as_bytes(), &mut out, &mut err, &Stop::never());
        assert_eq!(
            i64::from(code),
            want["code"].as_i64().expect("code"),
            "{name}: Exit-Code"
        );
        let mut got = lines(&out);
        if name.starts_with("world-values") {
            got = got.iter().map(|l| without_welt_events(l)).collect();
        }
        let want_lines: Vec<String> = want["out"]
            .as_array()
            .expect("out")
            .iter()
            .map(|l| l.as_str().expect("Zeile").to_string())
            .collect();
        assert_eq!(got.len(), want_lines.len(), "{name}: Zahl der Zeilen\n{got:?}");
        for (g, w) in got.iter().zip(&want_lines) {
            let json_error =
                event(w) == "error" && (w.contains("flag") || w.contains("Eingabe-JSON: invalid") || w.contains("EOF"));
            if json_error {
                assert_eq!(event(g), event(w), "{name}");
            } else {
                assert!(g == w, "{name}: {}", first_diff(g, w));
            }
        }
    }
}
