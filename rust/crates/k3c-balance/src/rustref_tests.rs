//! Vergleich mit Go (RU28.1): Dieselben kurzen Matrizen wie `engine/balance/rustref_test.go` (ein Tag, kein Balancing-Lauf,
//! B-1014); die Berichte aus Go liegen unter `testdata/rust/balance/`. Go kennt die nativen Regeln des Rust-Spiels nicht
//! (Hafenstart, Wächter-Bäume, Besorgung, `WorldValues::native_rules`), darum rechnet der Vergleich mit `Matrix::legacy`.

use k3c_det::golden;

use crate::run::{Matrix, run};
use crate::summary::read_summary;

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
