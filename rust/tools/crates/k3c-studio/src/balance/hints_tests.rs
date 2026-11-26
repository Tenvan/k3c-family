//! Go `hints_test.go`. Die drei Tests zum HTML-Bericht und zum nachträglichen Auswerten (`TestHTMLEigenstaendig`,
//! `TestHTMLOhneGrafiken`, `TestReevaluate`) brauchen `report_html` und `insights_files` und folgen mit diesen.

use serde_json::Value;

use super::super::datafiles;
use super::super::test_helpers::repo_root;
use super::{FIGURE_KEYS, default_hints, load_hints};

/// Liegt unter `datei.json:a.b.c` eine Zahl? Listen über den Index. Die Prüfung über `ParseTuneParam` wie in Go
/// folgt mit `tuneparam`.
fn number_at(path: &str) -> bool {
    let Some((file, keys)) = path.split_once(':') else {
        return false;
    };
    let raw = datafiles::read(&repo_root(), file).expect(path);
    let value: Value = serde_json::from_slice(&raw).expect(path);
    let found = keys.split('.').try_fold(&value, |v, key| match v {
        Value::Array(list) => key.parse::<usize>().ok().and_then(|i| list.get(i)),
        _ => v.get(key),
    });
    found.is_some_and(Value::is_number)
}

/// B-632/AC-02: Die Richtwert-Datei lädt; jeder Hebel zeigt auf eine Zahl in `data/` (ein umbenannter Pfad fällt hier
/// auf, nicht erst im Bericht).
#[test]
fn richtwerte_laden() {
    let h = default_hints(&repo_root()).expect("Richtwerte");
    for key in FIGURE_KEYS {
        assert!(h.figures.contains_key(key), "Richtwert für {key} fehlt");
    }
    for (measure, levers) in &h.levers {
        for lever in levers.low.iter().chain(&levers.high) {
            assert!(number_at(&lever.path), "hebel {measure}: {}", lever.path);
        }
    }
}

/// AC-02: Fehler beim Laden nennen die Stelle.
#[test]
fn richtwerte_fehler() {
    let lever = |path: &str, direction: &str, step: &str| {
        format!(
            r#"{{"version":1,"figures":{{}},"levers":{{"castleHeld":{{"low":[{{"path":"{path}","direction":"{direction}","step":{step},"label":"x"}}]}}}}}}"#
        )
    };
    let figure = |key: &str, green: &str, yellow: &str| {
        format!(
            r#"{{"version":1,"figures":{{"{key}":{{"name":"a","green":{green},"yellow":{yellow}}}}},"levers":{{}}}}"#
        )
    };
    for (raw, want) in [
        (
            r#"{"version":1,"figures":{},"levers":{},"x":1}"#.to_owned(),
            "unknown field",
        ),
        (r#"{"version":2,"figures":{},"levers":{}}"#.to_owned(), "version 2"),
        (figure("fehlt", "[0,1]", "[0,1]"), r#"kennzahl "fehlt""#),
        (figure("idleShare", "[0,30]", "[0,20]"), "yellow muss green"),
        (figure("idleShare", "[5,1]", "[null,null]"), "von ≤ bis"),
        (
            r#"{"version":1,"figures":{},"levers":{"fehlt":{"low":[],"high":[]}}}"#.to_owned(),
            r#""fehlt" unbekannt"#,
        ),
        (lever("a", "+", "0.1"), "erwartet datei.json"),
        (lever("a.json:b", "*", "0.1"), "direction"),
        (lever("a.json:b", "+", "2"), "step"),
    ] {
        let err = load_hints(raw.as_bytes()).expect_err(&raw);
        assert!(err.contains(want), "{raw}: Fehler {err}, erwartet {want}");
    }
}
