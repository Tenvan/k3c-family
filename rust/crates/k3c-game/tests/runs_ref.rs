//! RU33.1 (AC-02): Die Läufe des Spiels (`k3c_game::runs::exec`, im Prozess) geben für dieselben Aufrufe dieselben JSON-Zeilen
//! und Exit-Codes aus wie `k3c-sim` (Go). Die Vergleichsdateien schreibt der Go-Test `TestRustRef` aus `engine/simcli` mit
//! dem Schalter `-write` nach `testdata/rust/runs/`. Bei `error`-Zeilen, deren Text aus Gos `flag`-Paket oder JSON-Fehlern
//! stammt, zählen nur Zeile `event` und Code.
//!
//! RU40.1: Die Fälle `balance-*` laufen mit `--legacy` (Regeln des Go-Orakels, ohne die nativen Regeln des Rust-Spiels);
//! `pre` ist ein Vorlauf, der Dateien nach `{tmp}` schreibt (`balance --replay-dir`), `{tmp}` liegt unter `.work/`.

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

/// Der Endzustand-Hash einer Wiedergabe (`balance --play`) gleicht nicht dem von Go: Die Rust-Insel hat kein JSON, gehasht
/// wird ihre `Debug`-Ausgabe (`k3c_balance::replay::end_hash`). Die Zeile verliert ihn vor dem Vergleich; Ticks, Burgfall und
/// Warnungen zählen.
fn without_end_hash(line: &str) -> String {
    let key = "\"endHash\":\"";
    let Some(at) = line.find(key) else {
        return line.to_string();
    };
    let value = at + key.len();
    let end = line[value..].find('"').map_or(line.len(), |n| value + n + 1);
    let end = if line[end..].starts_with(',') { end + 1 } else { end };
    format!("{}{}", &line[..at], &line[end..])
}

/// Die erste Stelle, an der zwei Zeilen auseinandergehen, mit Umgebung.
fn first_diff(got: &str, want: &str) -> String {
    let at = got.chars().zip(want.chars()).take_while(|(a, b)| a == b).count();
    let cut = |s: &str| s.chars().skip(at.saturating_sub(60)).take(160).collect::<String>();
    format!("ab Zeichen {at}\n  Rust: {}\n  Go:   {}", cut(got), cut(want))
}

/// Die Argumente eines Falls unter `key` (`args` oder `pre`): `{tmp}` ersetzt, `--data` und bei `balance` `--legacy` dahinter.
fn call_args(want: &Value, key: &str, tmp: &str) -> Vec<String> {
    let Some(list) = want[key].as_array() else {
        return Vec::new();
    };
    let mut args: Vec<String> = list
        .iter()
        .map(|a| a.as_str().expect("Text").replace("{tmp}", tmp))
        .collect();
    if !args.is_empty() {
        args.push("--data".into());
        args.push(format!("{}/data", root()));
        if args[0] == "balance" {
            // Go kennt die nativen Regeln des Rust-Spiels nicht (Hafenstart, Wächter-Bäume): Vergleich mit den Regeln des
            // Orakels
            args.push("--legacy".into());
        }
    }
    args
}

/// Die Zeilen unter `out` der Vergleichsdatei.
fn want_lines(want: &Value) -> Vec<String> {
    want["out"]
        .as_array()
        .expect("out")
        .iter()
        .map(|l| l.as_str().expect("Zeile").to_string())
        .collect()
}

/// Führt einen Fall aus und vergleicht Exit-Code und Zeilen mit Go.
fn check_case(name: &str, want: &Value, tmp: &str) {
    let pre = call_args(want, "pre", tmp);
    if !pre.is_empty() {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = runs::exec(&pre, &mut "".as_bytes(), &mut out, &mut err, &Stop::never());
        assert_eq!(code, 0, "{name}: Vorlauf {}", String::from_utf8_lossy(&err));
    }
    let args = call_args(want, "args", tmp);
    let stdin = want["stdin"].as_str().expect("stdin").to_string();
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = runs::exec(&args, &mut stdin.as_bytes(), &mut out, &mut err, &Stop::never());
    assert_eq!(
        i64::from(code),
        want["code"].as_i64().expect("code"),
        "{name}: Exit-Code"
    );
    let normalize = |l: &String| match name {
        n if n.starts_with("world-values") => without_welt_events(l),
        "balance-play.json" => without_end_hash(l),
        _ => l.clone(),
    };
    let got: Vec<String> = lines(&out).iter().map(normalize).collect();
    let want_lines: Vec<String> = want_lines(want).iter().map(normalize).collect();
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

#[test]
fn laeufe_gleich_go() {
    let dir = format!("{}/testdata/rust/runs", root());
    // Ordner für `{tmp}` in den Fällen, die Dateien schreiben (balance --play); im Repo, nie im Temp (CLAUDE.md)
    let tmp = format!("{}/.work/runs-ref-{}", root(), std::process::id());
    std::fs::create_dir_all(&tmp).expect("Ordner");
    let mut names: Vec<_> = std::fs::read_dir(&dir)
        .expect("Vergleichsdateien (Go-Test RustRef mit -write, siehe Kopf der Datei)")
        .filter_map(|e| e.ok()?.file_name().to_str().map(ToString::to_string))
        .filter(|n| n.ends_with(".json"))
        .collect();
    names.sort();
    assert!(names.len() >= 8, "nur {} Fälle", names.len());
    for name in names {
        let want: Value =
            serde_json::from_str(&std::fs::read_to_string(format!("{dir}/{name}")).expect("Datei")).expect("JSON");
        check_case(&name, &want, &tmp);
    }
    let _ = std::fs::remove_dir_all(&tmp);
}
