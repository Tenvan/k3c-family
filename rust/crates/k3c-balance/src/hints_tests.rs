//! Tests zu `hints.rs` (Port von `engine/balance/hints_test.go`).

use k3c_det::golden;

use crate::hints::{FIGURE_KEYS, default_hints, lever_path, load_hints};
use crate::insights_files::{meta_of, read_baseline, reevaluate};

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

/// AC-06: Für einen vorhandenen Bericht entstehen .md und .html nachträglich (`Reevaluate`). Der Teil mit dem Inhalt der
/// Auswertung (`## Kurzauswertung`, Lauf und Zeit im HTML) steht in `reevaluate_inhalt`.
#[test]
fn reevaluate_schreibt_dateien() {
    let dir = std::env::temp_dir()
        .join(format!("k3c-reevaluate-{}", std::process::id()))
        .join("simtest-run-9");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("Ordner");
    let src = dir.join("balance-20261025-062117.json");
    std::fs::copy(golden::testdata().join("rust/balance/insights-run7.json"), &src).expect("Bericht");
    let root = golden::testdata().join("..").to_string_lossy().into_owned();
    let tuning = dir.join("tuning-1.json").to_string_lossy().into_owned();
    assert!(
        reevaluate(&tuning, &root).is_err(),
        "tuning-*.json darf nicht ausgewertet werden"
    );
    let page = reevaluate(&src.to_string_lossy(), &root).expect("Auswertung");
    assert!(page.ends_with("balance-20261025-062117.html"), "{page}");
    let md = std::fs::read_to_string(dir.join("balance-20261025-062117.md")).expect("Markdown");
    assert!(
        md.starts_with("# Balancing-Bericht") && md.contains("## Ziele im Detail"),
        "{md}"
    );
    assert!(std::path::Path::new(&page).exists());
    let _ = std::fs::remove_dir_all(dir.parent().expect("Wurzel"));
}

/// AC-06: Lauf, Zeit und Kurzauswertung stehen in den neuen Dateien.
#[test]
#[ignore = "wartet auf RU31.4"]
fn reevaluate_inhalt() {}

/// `meta_of`: Lauf = Ordner des Berichts, Zeit aus dem Dateinamen; ohne Zeit im Namen die aktuelle (Go: `MetaOf`).
#[test]
fn meta_aus_dateiname() {
    let m = meta_of("reports/simtest-run-9/balance-20261025-062117", "b.json");
    assert_eq!(
        (m.run.as_str(), m.generated.as_str(), m.baseline.as_str()),
        ("simtest-run-9", "25.10.2026 06:21", "b.json")
    );
    let jetzt = meta_of("reports/simtest-run-9/balance-gestern", "").generated;
    assert_eq!(jetzt.len(), "25.10.2026 06:21".len(), "{jetzt}");
    assert_ne!(jetzt, "25.10.2026 06:21");
    assert_ne!(
        meta_of("x/balance-20261340-062117", "").generated,
        "40.13.2026 06:21",
        "Monat 13 gibt es nicht"
    );
    assert_eq!(
        meta_of("x/balance-20240229-235959", "").generated,
        "29.02.2024 23:59",
        "Schaltjahr"
    );
    assert!(
        read_baseline(std::path::Path::new("gibt/es/nicht.json"))
            .expect("fehlt still")
            .is_none()
    );
}
