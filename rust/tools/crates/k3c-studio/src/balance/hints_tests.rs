//! Go `hints_test.go`.

use std::fs;
use std::path::Path;

use regex::Regex;

use super::super::insights::{Insights, Meta, build_insights};
use super::super::insights_files::reevaluate;
use super::super::test_helpers::{fixture, repo_root, temp_root};
use super::super::tuneparam::parse_tune_param;
use super::{FIGURE_KEYS, default_hints, load_hints};

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
            let parsed = parse_tune_param(&repo_root(), &format!("{}=0..1", lever.path));
            assert!(parsed.is_ok(), "hebel {measure}: {parsed:?}");
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

fn fixture_insights(run: &str) -> Insights {
    let root = repo_root();
    let hints = default_hints(&root).expect("Richtwerte");
    let meta = Meta {
        run: run.to_owned(),
        ..Meta::default()
    };
    build_insights(&root, &fixture(), None, &hints, &meta)
}

/// Der Bericht ist eigenständig: keine externen Ressourcen, Grafiken als data-URI.
#[test]
fn html_eigenstaendig() {
    let root = repo_root();
    // Die Papier-PNG sind nicht eingecheckt (B-737); ohne gen:paper fehlen sie.
    if !root.join("public/grafik/gebaeude-paper/burg/stand.png").exists() {
        eprintln!("übersprungen: Papier-Grafik nicht gerendert (task gen:paper)");
        return;
    }
    let html = fixture_insights("simtest-run-7").html(&root).expect("HTML");
    let external = Regex::new(r#"(?i)(src|href)\s*=\s*["']?\s*(https?:)?//|url\(\s*["']?(https?:)?//|@import|<link\b"#)
        .expect("Muster");
    assert_eq!(external.find(&html).map(|m| m.as_str()), None, "externe Ressource");
    for key in ["castle", "wall", "tower", "coin", "enemy"] {
        assert!(
            html.contains(&format!(r#""{key}":"data:image/png;base64,"#)),
            "Grafik {key} fehlt"
        );
    }
    for placeholder in ["__CSS__", "__JS__", "__DATA__", "__ASSETS__", "__TITLE__"] {
        assert!(!html.contains(placeholder), "Platzhalter {placeholder} nicht ersetzt");
    }
    assert!(
        html.matches("</script>").count() == 3 && html.contains("Hub-Stufe 2"),
        "Skript-Blöcke oder Daten fehlen"
    );
}

/// AC-04 Fehlerfall: ohne `public/` fehlen die Grafiken (Platzhalter im Bericht), der Bericht entsteht trotzdem.
#[test]
fn html_ohne_grafiken() {
    let (_tmp, root) = temp_root();
    let html = fixture_insights("").html(&root).expect("HTML");
    assert!(html.contains(r#"id="report-assets">{}</script>"#));
}

fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("Ordner");
    for entry in fs::read_dir(from).expect("lesen") {
        let entry = entry.expect("Eintrag");
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("kopieren");
        }
    }
}

/// Legt einen Bericht unter `reports/` eines Wurzel-Doubles ab und wertet ihn neu aus. Go schreibt dafür in einen
/// Ordner außerhalb der Wurzel, was der Wächter ablehnt; darum liegt hier alles samt `data/` in der Temp-Wurzel.
fn reevaluated() -> (String, String) {
    let (_tmp, root) = temp_root();
    copy_dir(&repo_root().join("data"), &root.join("data"));
    let dir = root.join("reports").join("simtest-run-9");
    fs::create_dir_all(&dir).expect("Ordner");
    let source = repo_root().join("tools/k3c-studio/internal/balance/testdata/insights-run7.json");
    let report = dir.join("balance-20261025-062117.json");
    fs::copy(source, &report).expect("Fixture");
    assert!(
        reevaluate(&dir.join("tuning-1.json"), &root).is_err(),
        "tuning-*.json darf nicht ausgewertet werden"
    );
    let page = reevaluate(&report, &root).expect("Auswertung");
    let html = fs::read_to_string(page).expect("HTML");
    let md = fs::read_to_string(dir.join("balance-20261025-062117.md")).expect("Markdown");
    assert!(dir.join("result.json").is_file(), "result.json fehlt");
    (html, md)
}

/// AC-06: für einen vorhandenen Bericht entstehen `.md` und `.html` nachträglich.
#[test]
fn reevaluate_schreibt_dateien() {
    let (html, md) = reevaluated();
    assert!(
        html.contains(r#""run":"simtest-run-9""#) && html.contains(r#""generated":"25.10.2026 06:21""#),
        "Auswertung nicht geschrieben"
    );
    assert!(md.starts_with("# Balancing-Bericht\n"), "{md}");
}

#[test]
#[ignore = "RU70.17: Insights::markdown schreibt die Kurzauswertung"]
fn reevaluate_schreibt_kurzauswertung() {
    assert!(reevaluated().1.contains("## Kurzauswertung"));
}
