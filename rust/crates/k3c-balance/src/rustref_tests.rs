//! Vergleich mit Go (RU28.1): Dieselben kurzen Matrizen wie `engine/balance/rustref_test.go` (ein Tag, kein Balancing-Lauf,
//! B-1014); die Berichte aus Go liegen unter `testdata/rust/balance/`. Go kennt die nativen Regeln des Rust-Spiels nicht
//! (Hafenstart, Wächter-Bäume, Besorgung, `WorldValues::native_rules`), darum rechnet der Vergleich mit `Matrix::legacy`.

use k3c_det::golden;

use crate::evaluate::STATUS_OK;
use crate::insights::{Meta, build_insights};
use crate::insights_markdown::insights_markdown;
use crate::insights_tests::{fixture, test_hints};
use crate::report_html::report_html;
use crate::report_html_tests::{pixels, zerlegt};
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

/// RU31.1: Die Auswertung aus dem echten Bericht simtest-run-7 (kein Lauf), ohne und mit Baseline, hat dasselbe JSON wie Go
/// (`TestRustRefInsights`), alle Felder; `generated` ist in beiden leer.
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
        golden::tree(&build_insights(&s, b, &h, &m))
    };
    for (file, got) in [
        ("insights-run7-out.json", run(None, "")),
        ("insights-run7-trend.json", run(Some(&base), "b.json")),
    ] {
        let want = golden::read(dir.join(file));
        let at = golden::diff("", &want, &got);
        assert!(at.is_empty(), "{file}: Abweichung von Go bei {at}");
    }
}

/// RU31.2 und RU31.4: Der Markdown-Text der Auswertung aus `insights_markdown` ist Zeichen für Zeichen der von Go
/// (`insights-run7.md`, `insights-run7-trend.md`): Kurzauswertung, Ampeln, Stärken, Schwächen, Hinweise, nächster Lauf,
/// Kennzahlen und Delta.
#[test]
fn insights_markdown_ganz() {
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
        assert_eq!(want, got, "{file}");
    }
}

/// RU31.4: Der HTML-Bericht aus dem echten Bericht simtest-run-7 (`TestRustRefHTML`): gleicher Text, gleiche Daten (über den
/// JSON-Baum, weil Zahlen als `12.0` statt `12` erscheinen), und die Grafiken aus `root/public/` haben dieselben Pixel
/// (Breite, Höhe, RGBA); die PNG-Bytes der beiden Kodierer sind verschieden.
#[test]
fn html_run7() {
    let dir = golden::testdata().join("rust/balance");
    let inn = build_insights(
        &fixture(),
        None,
        &test_hints(),
        &Meta {
            run: "simtest-run-7".into(),
            ..Meta::default()
        },
    );
    let root = dir.join("root");
    let got = String::from_utf8(report_html(&inn, &root.to_string_lossy()).expect("Bericht")).expect("UTF-8");
    let want = std::fs::read_to_string(dir.join("insights-run7.html")).expect("HTML");
    let (want, got) = (
        zerlegt(&want.replace("\r\n", "\n")),
        zerlegt(&got.replace("\r\n", "\n")),
    );
    assert_eq!(want.rest, got.rest, "Seite ohne Daten und Grafiken");
    let at = golden::diff("", &want.data, &got.data);
    assert!(at.is_empty(), "Daten: Abweichung von Go bei {at}");
    assert_eq!(
        want.assets.keys().collect::<Vec<_>>(),
        got.assets.keys().collect::<Vec<_>>(),
        "Grafiken"
    );
    assert_eq!(want.assets.len(), 3, "castle, axe, coin");
    for (key, w) in &want.assets {
        let (a, b) = (pixels(w), pixels(&got.assets[key]));
        assert_eq!((a.w, a.h), (b.w, b.h), "{key}: Größe");
        assert_eq!(a.px, b.px, "{key}: Pixel");
    }
}

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
