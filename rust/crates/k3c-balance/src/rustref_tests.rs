//! Vergleich mit Go (RU28.1): Dieselben kurzen Matrizen wie `engine/balance/rustref_test.go` (ein Tag, kein Balancing-Lauf,
//! B-1014); die Berichte aus Go liegen unter `testdata/rust/balance/`. Go kennt die nativen Regeln des Rust-Spiels nicht
//! (Hafenstart, Wächter-Bäume, Besorgung, `WorldValues::native_rules`), darum rechnet der Vergleich mit `Matrix::legacy`.

use k3c_det::golden;

use crate::evaluate::STATUS_OK;
use crate::insights::{Meta, build_insights};
use crate::insights_markdown::insights_markdown;
use crate::insights_tests::{fixture, test_hints};
use crate::run::{Matrix, run};
use crate::summary::{Summary, read_summary};

fn matrix(players: i64, bot: &str) -> Matrix {
    Matrix {
        seeds: vec!["1".into()],
        players: vec![players],
        bots: vec![bot.into()],
        depths: vec![0],
        days: 1,
        legacy: true,
        ..Matrix::default()
    }
}

fn gleich_wie_go(file: &str, m: &Matrix) {
    let want = golden::read(golden::testdata().join("rust/balance").join(file));
    let got = golden::tree(&run(m).expect("Lauf"));
    let at = golden::diff("", &want, &got);
    assert!(at.is_empty(), "{file}: Abweichung von Go bei {at}");
}

/// RU28.3: Die Zusammenfassung aus dem echten Bericht simtest-run-7 (kein Lauf) hat dasselbe JSON und dasselbe Markdown wie
/// Go (`TestRustRefSummary`); der Vergleich geht über den JSON-Baum, weil Zahlen als `12.0` statt `12` erscheinen.
#[test]
fn summary_run7() {
    let dir = golden::testdata().join("rust/balance");
    let raw = std::fs::read_to_string(dir.join("insights-run7.json")).expect("Bericht");
    let s = read_summary(&raw).expect("Zusammenfassung");
    let at = golden::diff("", &golden::read(dir.join("summary-run7.json")), &golden::tree(&s));
    assert!(at.is_empty(), "summary-run7.json: Abweichung von Go bei {at}");
    let md = std::fs::read_to_string(dir.join("summary-run7.md")).expect("Markdown");
    assert_eq!(s.markdown_with(None, None), md.replace("\r\n", "\n"), "summary-run7.md");
}

/// Nimmt die Felder aus der Auswertung, die RU31.4 liefert (Kurzauswertung, Stärken, Schwächen); sie fehlen in beiden
/// Bäumen. Zeilen, Pläne, Hinweise, Hebel-Gruppen und der Empfehlungsblock stehen seit RU31.3 im Vergleich.
fn ohne_spaetere_felder(mut v: serde_json::Value) -> serde_json::Value {
    let Some(o) = v.as_object_mut() else { return v };
    for k in ["summary", "strengths", "weaknesses"] {
        o.remove(k);
    }
    v
}

/// RU31.1: Die Auswertung aus dem echten Bericht simtest-run-7 (kein Lauf), ohne und mit Baseline, hat dasselbe JSON wie Go
/// (`TestRustRefInsights`), soweit RU31.1 bis RU31.3 sie liefern; `generated` ist in beiden leer.
#[test]
fn insights_run7() {
    let dir = golden::testdata().join("rust/balance");
    let s = fixture();
    let mut base = fixture();
    for e in &mut base.targets {
        if e.verdict.target.measure == "goldAtDawnPerPlayer" {
            e.verdict.value = 22.0;
            e.verdict.status = STATUS_OK.into();
            e.pass = true;
        }
    }
    let h = test_hints();
    let run = |b: Option<&Summary>, baseline: &str| {
        let m = Meta {
            run: "simtest-run-7".into(),
            baseline: baseline.into(),
            ..Meta::default()
        };
        ohne_spaetere_felder(golden::tree(&build_insights(&s, b, &h, &m)))
    };
    for (file, got) in [
        ("insights-run7-out.json", run(None, "")),
        ("insights-run7-trend.json", run(Some(&base), "b.json")),
    ] {
        let want = ohne_spaetere_felder(golden::read(dir.join(file)));
        let at = golden::diff("", &want, &got);
        assert!(at.is_empty(), "{file}: Abweichung von Go bei {at}");
    }
}

/// Der Abschnitt ab `heading` bis zum nächsten `## ` (oder zum Ende).
fn abschnitt<'a>(md: &'a str, heading: &str) -> &'a str {
    let Some(i) = md.find(heading) else { return "" };
    let rest = &md[i + heading.len()..];
    &md[i..i + heading.len() + rest.find("\n## ").unwrap_or(rest.len())]
}

/// RU31.2: Der Markdown-Text der Auswertung aus `insights_markdown` hat dieselben Abschnitte wie Go (`insights-run7.md`,
/// `insights-run7-trend.md`), soweit sie nicht von RU31.4 abhängen: Zählung der Ampeln, Delta, nächster Lauf
/// (mit dem Empfehlungsblock) und Kennzahlen.
#[test]
fn insights_markdown_run7() {
    let dir = golden::testdata().join("rust/balance");
    let (s, h) = (fixture(), test_hints());
    let mut base = fixture();
    for e in &mut base.targets {
        if e.verdict.target.measure == "goldAtDawnPerPlayer" {
            e.verdict.value = 22.0;
            e.verdict.status = STATUS_OK.into();
            e.pass = true;
        }
    }
    for (file, b, baseline) in [
        ("insights-run7.md", None, ""),
        ("insights-run7-trend.md", Some(&base), "b.json"),
    ] {
        let m = Meta {
            run: "simtest-run-7".into(),
            baseline: baseline.into(),
            ..Meta::default()
        };
        let got = insights_markdown(&build_insights(&s, b, &h, &m));
        let want = std::fs::read_to_string(dir.join(file))
            .expect("Markdown")
            .replace("\r\n", "\n");
        let zaehlung = |t: &str| abschnitt(t, "## Ampeln").lines().nth(2).unwrap_or_default().to_string();
        assert_eq!(zaehlung(&want), zaehlung(&got), "{file}: Ampeln");
        for head in ["## Delta je Werte-Gruppe", "## Kennzahlen"] {
            assert_eq!(abschnitt(&want, head), abschnitt(&got, head), "{file}: {head}");
        }
        let (w, g) = (
            abschnitt(&want, "## Nächster Lauf"),
            abschnitt(&got, "## Nächster Lauf"),
        );
        assert_eq!(w, g, "{file}: Nächster Lauf");
    }
}

/// RU31.4: Das ganze Markdown stimmt mit Go überein, wenn `plain`, `plans`, `summary` und die Stärken da sind.
#[test]
#[ignore = "wartet auf RU31.4"]
fn insights_markdown_ganz() {}

/// Ein Spieler, Bot `saver`, Tiefe 0, ein Tag: der ganze Bericht stimmt mit Go überein.
#[test]
fn matrix_saver_1() {
    gleich_wie_go("matrix-saver-1.json", &matrix(1, "saver"));
}

/// Zwei Spieler, Bot `wirtschaft`, Tiefe 0, ein Tag: der ganze Bericht stimmt mit Go überein.
#[test]
fn matrix_wirtschaft_2() {
    gleich_wie_go("matrix-wirtschaft-2.json", &matrix(2, "wirtschaft"));
}
