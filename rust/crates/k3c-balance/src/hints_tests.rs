//! Tests zu `hints.rs` (Port von `engine/balance/hints_test.go`).

use crate::hints::{FIGURE_KEYS, default_hints, lever_path, load_hints};

/// B-632/AC-02: Die eingebettete Richtwert-Datei lädt; jeder Hebel zeigt auf eine Zahl in data/ (ein umbenannter Pfad
/// fällt hier auf, nicht erst im Bericht). Go prüft den Pfad mit `ParseTuneParam` (RU31.5, `tuneparam.rs`); der Teil, der
/// ohne sie läuft, steht in `richtwerte_laden_ohne_tuning`.
#[test]
#[ignore = "wartet auf RU31"]
fn richtwerte_laden() {}

/// Der Teil von `richtwerte_laden`, den `lever_path` allein erfüllt: Die Datei lädt, jede Kennzahl hat einen Richtwert,
/// jeder Hebelpfad hat die Form `datei.json:a.b.c`.
#[test]
fn richtwerte_laden_ohne_tuning() {
    let h = default_hints().expect("Richtwerte laden");
    for k in FIGURE_KEYS {
        assert!(h.figures.contains_key(k), "Richtwert für {k} fehlt");
    }
    for (measure, l) in &h.levers {
        for lv in l.low.iter().chain(&l.high) {
            assert!(lever_path(&lv.path), "hebel {measure}: pfad {:?}", lv.path);
        }
    }
}

/// AC-02: Fehler beim Laden nennen die Stelle.
#[test]
fn richtwerte_fehler() {
    let cases = [
        (r#"{"version":1,"figures":{},"levers":{},"x":1}"#, "unknown field"),
        (r#"{"version":2,"figures":{},"levers":{}}"#, "version 2"),
        (
            r#"{"version":1,"figures":{"fehlt":{"name":"a","green":[0,1],"yellow":[0,1]}},"levers":{}}"#,
            r#"kennzahl "fehlt""#,
        ),
        (
            r#"{"version":1,"figures":{"idleShare":{"name":"a","green":[0,30],"yellow":[0,20]}},"levers":{}}"#,
            "yellow muss green",
        ),
        (
            r#"{"version":1,"figures":{"idleShare":{"name":"a","green":[5,1],"yellow":[null,null]}},"levers":{}}"#,
            "von ≤ bis",
        ),
        (
            r#"{"version":1,"figures":{},"levers":{"fehlt":{"low":[],"high":[]}}}"#,
            r#""fehlt" unbekannt"#,
        ),
        (
            r#"{"version":1,"figures":{},"levers":{"castleHeld":{"low":[{"path":"a","direction":"+","step":0.1,"label":"x"}]}}}"#,
            "erwartet datei.json",
        ),
        (
            r#"{"version":1,"figures":{},"levers":{"castleHeld":{"low":[{"path":"a.json:b","direction":"*","step":0.1,"label":"x"}]}}}"#,
            "direction",
        ),
        (
            r#"{"version":1,"figures":{},"levers":{"castleHeld":{"low":[{"path":"a.json:b","direction":"+","step":2,"label":"x"}]}}}"#,
            "step",
        ),
    ];
    for (raw, want) in cases {
        match load_hints(raw) {
            Err(e) if e.to_string().contains(want) => {}
            r => panic!("{raw}: Fehler {r:?}, erwartet {want:?}"),
        }
    }
}

/// B-632/AC-04: Der HTML-Bericht ist eine Datei ohne externe Ressourcen, mit eingebetteten Spielgrafiken und den Daten.
/// Go: `TestHTMLEigenstaendig` prüft die Seite aus `BuildInsights(...).HTML` mit einer Regex auf externe Verweise (RU31.4,
/// `report_html.rs`); RU31 baut die Prüfung als Funktion mit Zeichenvergleich nach. Die Fixture dazu liegt unter
/// `testdata/rust/balance/insights-run7.json`.
#[test]
#[ignore = "wartet auf RU31"]
fn html_eigenstaendig() {}

/// AC-04 Fehlerfall: Ohne public/ fehlen die Grafiken (Platzhalter im Bericht), der Bericht entsteht trotzdem.
#[test]
#[ignore = "wartet auf RU31"]
fn html_ohne_grafiken() {}

/// AC-06: Für einen vorhandenen Bericht entstehen .md und .html nachträglich (`Reevaluate`).
#[test]
#[ignore = "wartet auf RU31"]
fn reevaluate() {}
