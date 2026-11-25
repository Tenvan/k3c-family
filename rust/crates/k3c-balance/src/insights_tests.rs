//! Tests zu `insights.rs`, `insights_figures.rs` und `insights_files.rs` (Port von `engine/balance/insights_test.go`, B-632).
//! Die Fixture ist der echte Bericht simtest-run-7 (13 Ziele, 4 verletzt, 2 knapp, mit Kennzahlen). Tests, die Texte und
//! Vorschläge der Auswertung brauchen, tragen `ignore` mit der Session, die sie liefert (RU31.2 bis RU31.5).

use k3c_det::golden;

use crate::evaluate::{STATUS_INVALID, STATUS_OK};
use crate::hints::{Hints, default_hints};
use crate::insights::{
    Insights, LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, Meta, TargetInsight, build_insights,
};
use crate::summary::{Summary, read_summary};

/// Der Bericht simtest-run-7 (Go: `fixture`).
pub(crate) fn fixture() -> Summary {
    let raw = std::fs::read_to_string(golden::testdata().join("rust/balance/insights-run7.json")).expect("Bericht");
    read_summary(&raw).expect("Zusammenfassung")
}

pub(crate) fn test_hints() -> Hints {
    default_hints().expect("Richtwerte")
}

fn find<'a>(inn: &'a Insights, measure: &str, grade: &str) -> &'a TargetInsight {
    inn.targets
        .iter()
        .find(|t| t.measure == measure && t.grade == grade)
        .unwrap_or_else(|| panic!("Ziel {measure}/{grade} fehlt"))
}

fn run7() -> Insights {
    let m = Meta {
        run: "simtest-run-7".into(),
        ..Meta::default()
    };
    build_insights(&fixture(), None, &test_hints(), &m)
}

/// B-632/AC-01: Ampel je Ziel mit Abstand in % der Korridorbreite und Hebeln in die nötige Richtung.
#[test]
fn ampeln() {
    let inn = run7();
    let c = &inn.counts;
    assert_eq!(
        (c[LIGHT_GREEN], c[LIGHT_YELLOW], c[LIGHT_RED], c[LIGHT_GREY]),
        (7, 2, 4, 0),
        "Ampeln: {c:?}"
    );
    let hub = find(&inn, "hubLevel2BeforeDay5", "normal");
    assert_eq!(hub.light, LIGHT_RED);
    assert_eq!(hub.distance, Some(-240.0));
    assert_eq!(hub.levers.first().map(|l| l.direction.as_str()), Some("-"));
    let castle = find(&inn, "castleHeld", "normal");
    assert_eq!((castle.light.as_str(), castle.distance), (LIGHT_YELLOW, Some(33.3)));
}

/// AC-01: Kurzauswertung (3 bis 6 Sätze), Stärken, Schwächen und Tuning-Hinweise, verletzte Ziele zuerst.
#[test]
#[ignore = "wartet auf RU31.4"]
fn texte() {}

/// AC-01: Ampel je Kennzahl (der Teil ohne den Vorschlag für den nächsten Lauf).
#[test]
fn kennzahlen_ampeln() {
    let inn = run7();
    let first = inn.figures.lights.first().expect("Kennzahl-Ampeln");
    assert_eq!(
        (first.key.as_str(), first.light.as_str()),
        ("firstWallTick", LIGHT_GREEN)
    );
    assert!(inn.figures.note.contains("gültigen Läufen"), "{}", inn.figures.note);
    assert!(!inn.figures.nights.is_empty());
    assert!(inn.figures.bands.contains_key("idleShare") && inn.figures.bands.contains_key("goldBalance"));
}

/// AC-01: Der Vorschlag für den nächsten Lauf mit Ziel je Befehl.
#[test]
#[ignore = "wartet auf RU31.2"]
fn naechster_lauf() {}

/// AC-01: Trend zur Baseline und Vergleich; ohne Baseline kein Trend.
#[test]
fn trend() {
    let cur = fixture();
    let mut base = fixture();
    for e in &mut base.targets {
        if e.verdict.target.measure == "goldAtDawnPerPlayer" {
            e.verdict.value = 22.0;
            e.verdict.status = STATUS_OK.into();
            e.pass = true;
        }
    }
    let m = Meta {
        baseline: "b.json".into(),
        ..Meta::default()
    };
    let inn = build_insights(&cur, Some(&base), &test_hints(), &m);
    let gold = find(&inn, "goldAtDawnPerPlayer", "normal");
    let t = gold.trend.as_ref().expect("Trend");
    assert_eq!(
        (t.delta, t.better, t.before_status.as_str()),
        (-5.0, false, STATUS_OK),
        "Trend: {t:?}"
    );
    assert!(inn.baseline.present && inn.baseline.changes.len() == 1 && inn.baseline.changes[0].tipped);
    assert_eq!(inn.baseline.note, "Baseline: b.json");
    let ohne = build_insights(&cur, None, &test_hints(), &Meta::default());
    assert!(
        ohne.targets[0].trend.is_none() && !ohne.baseline.present,
        "ohne Baseline kein Trend"
    );
    assert_eq!(ohne.baseline.note, "Keine Baseline: kein Trend.");
}

/// AC-06 und Fehlerfälle: Bericht ohne Kennzahlen → graue Kennzahl-Ampeln mit Hinweis; ungültiges Ziel → grau.
#[test]
fn alter_bericht() {
    let mut s = fixture();
    s.kennzahlen = None;
    s.targets[0].verdict.status = STATUS_INVALID.into();
    s.targets[0].verdict.invalid_seeds = vec!["4".into(), "9".into()];
    let inn = build_insights(&s, None, &test_hints(), &Meta::default());
    assert!(
        inn.figures.note.contains("ohne Kennzahlen") && inn.figures.nights.is_empty(),
        "{}",
        inn.figures.note
    );
    assert!(!inn.figures.lights.is_empty());
    for l in &inn.figures.lights {
        assert!(l.light == LIGHT_GREY && l.value.is_none(), "Kennzahl nicht grau: {l:?}");
    }
    let t = &inn.targets[0];
    assert!(
        t.light == LIGHT_GREY && t.distance.is_none() && inn.counts[LIGHT_GREY] == 1,
        "ungültiges Ziel: {t:?}"
    );
}

/// Der Hinweis zu ungültigen Seeds kommt aus `hints` (Go: `hints(in)`).
#[test]
#[ignore = "wartet auf RU31.4"]
fn alter_bericht_hinweis() {}

/// AC-01: Ein bestandener Lauf ohne Gelb schlägt vor, die Baseline neu zu schreiben, und hat keine roten Hinweise.
#[test]
#[ignore = "wartet auf RU31.2"]
fn alles_gruen() {}

/// AC-03: Der Markdown-Bericht hat oben Kurzauswertung, Ampeln, Stärken/Schwächen, Hinweise, nächsten Lauf und Kennzahlen.
#[test]
#[ignore = "wartet auf RU31.2"]
fn markdown_mit_auswertung() {}

/// AC-01: Der Bereich fürs Abstimmen geht vom Wert in data/ höchstens drei Schritte in Richtung des Hebels.
#[test]
#[ignore = "wartet auf RU31.5"]
fn hebel_bereich() {}

/// BS3.4: Ein Ziel ohne Eintrag in balance-hints.json (neue Kennzahl) darf keine Hebel als null melden; die Oberfläche liest
/// `levers.length` und stürzte sonst beim Öffnen des Berichts ab.
#[test]
fn hebel_nie_null() {
    let mut s = fixture();
    for e in &mut s.targets {
        e.verdict.target.measure = "ohneHinweise".into(); // kein Eintrag in den Hinweisen
    }
    let inn = build_insights(&s, None, &test_hints(), &Meta::default());
    let raw = serde_json::to_string(&inn).expect("JSON");
    assert!(
        !raw.contains(r#""levers":null"#),
        "levers:null im Bericht der Oberfläche"
    );
    assert!(inn.targets.iter().all(|t| t.levers.is_empty()));
}

/// Die Kennzahlen eines Grads, die nicht grün sind, erscheinen mit Grad; normal immer (`grade_lights`).
#[test]
fn kennzahlen_je_grad() {
    let inn = run7();
    let normal: Vec<_> = inn
        .figures
        .lights
        .iter()
        .filter(|l| l.grade == "normal" || l.grade.is_empty())
        .collect();
    assert!(
        normal.iter().any(|l| l.key == "idleShare"),
        "idleShare für normal fehlt"
    );
    for l in inn
        .figures
        .lights
        .iter()
        .filter(|l| !l.grade.is_empty() && l.grade != "normal")
    {
        assert_ne!(
            l.light, LIGHT_GREEN,
            "{} {}: grüne Kennzahl eines anderen Grads",
            l.key, l.grade
        );
    }
}
