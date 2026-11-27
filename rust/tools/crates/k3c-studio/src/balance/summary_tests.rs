//! Go `summary_test.go`.

use super::evaluate::{STATUS_OK, STATUS_VIOLATED};
use super::report_types::{Matrix, Metrics, Report};
use super::summary::{Summary, compare, read_summary};
use super::targets::{Margin, Target, Targets};
use super::test_helpers::{metrics_with, repo_root, results, seeds, share_target};

fn summarize_with(g: Target, ms: &[Option<Metrics>]) -> Summary {
    let ts = Targets {
        seeds: seeds(ms.len()),
        targets: vec![g],
        margin: Margin {
            share_pp: 5.0,
            ..Margin::default()
        },
        ..Targets::default()
    };
    let report = Report {
        matrix: Matrix {
            seeds: ts.seeds.clone(),
            ..Matrix::default()
        },
        results: results(ms),
    };
    ts.summarize(&repo_root(), &report)
}

fn run(fallen: bool) -> Option<Metrics> {
    Some(metrics_with(fallen, 1, &[]))
}

/// BAL2/AC-04: Der Bericht nennt je verletztem Ziel Kennzahl (Name), Szenario und die Seeds zum Nachspielen.
#[test]
fn bericht_nennt_seeds() {
    let s = summarize_with(share_target(), &[run(false), run(true), run(true), run(false)]);
    let md = s.markdown(&repo_root(), None);
    assert!(
        !s.pass && md.contains("| x | 2 Spieler, saver, Tiefe 0, 5 Tage | 50,0 % | 70–90 % | Fail (verletzt) | 2, 3 |"),
        "Zeile fehlt:\n{md}"
    );
    assert!(md.contains("Keine Baseline"), "Hinweis auf fehlende Baseline fehlt");
    let json = s.json().expect("JSON");
    assert!(
        json.contains("\"failSeeds\"") && json.contains("\"pass\": false"),
        "{json}"
    );
}

/// Ein abgebrochener Lauf erscheint mit seinem Seed als ungültig.
#[test]
fn bericht_ungueltig_mit_seed() {
    let md = summarize_with(share_target(), &[run(false), None]).markdown(&repo_root(), None);
    assert!(
        md.contains("Fail (ungültig)") && md.contains("ungültig: 2"),
        "ungültiger Lauf fehlt:\n{md}"
    );
}

/// BAL2/AC-05: Eine Wertänderung (Untergrenze 70 → 85 %) lässt das Ziel von ok nach verletzt kippen; der Vergleich
/// nennt es, ein unverändertes Ziel nicht.
#[test]
fn vergleich_nennt_gekippte() {
    let ms: Vec<Option<Metrics>> = (0..20).map(|i| run(i >= 16)).collect(); // 80 % halten
    let y = Target {
        name: "y".to_owned(),
        ..share_target()
    };
    let mut base = summarize_with(share_target(), &ms);
    base.targets.extend(summarize_with(y.clone(), &ms).targets);
    assert_eq!(base.targets[0].verdict.status, STATUS_OK, "Baseline");
    let g = Target {
        lower: Some(85.0),
        upper: Some(100.0),
        ..share_target()
    };
    let mut cur = summarize_with(g, &ms);
    cur.targets.extend(summarize_with(y, &ms).targets);
    let ch = compare(&base, &cur);
    assert!(
        ch.len() == 1
            && ch[0].name == "x"
            && ch[0].before == STATUS_OK
            && ch[0].after == STATUS_VIOLATED
            && ch[0].tipped,
        "Änderungen: {ch:?}"
    );
    // nur der Vergleichsabschnitt: die Auswertung oben (B-632) nennt y als Stärke
    let md = cur.markdown(&repo_root(), Some(&base));
    let (_, tail) = md.split_once("## Vergleich mit der Baseline").expect("Abschnitt");
    assert!(
        tail.contains("- x: ok → verletzt **gekippt**") && !tail.contains("- y:"),
        "Markdown:\n{tail}"
    );
    assert!(
        compare(&base, &base).is_empty(),
        "gleiche Berichte dürfen nichts melden"
    );
}

/// Baseline lesen und schreiben; neues bzw. entfallenes Ziel.
#[test]
fn baseline_lesen() {
    let s = summarize_with(share_target(), &[run(false), run(false)]);
    let back = read_summary(s.json().expect("JSON").as_bytes()).expect("lesen");
    assert!(
        back.targets.len() == 1 && back.targets[0].verdict.status == s.targets[0].verdict.status,
        "{back:?}"
    );
    let new = compare(&Summary::default(), &s);
    assert!(new.len() == 1 && new[0].before == "neu", "neu: {new:?}");
    let gone = compare(&s, &Summary::default());
    assert!(gone.len() == 1 && gone[0].after == "entfällt", "entfällt: {gone:?}");
}

/// Die eingecheckte Baseline stammt von Go: gelesen und neu geschrieben entsteht derselbe Text (Feldnamen,
/// Reihenfolge, `omitempty`, Zahlformen, Einrückung); nur die Zeilenenden des Checkouts zählen nicht.
#[test]
fn go_baseline_bleibt_byte_gleich() {
    let path = repo_root().join(super::insights_files::BASELINE_PATH);
    let want = std::fs::read_to_string(path).expect("Baseline").replace("\r\n", "\n");
    let got = read_summary(want.as_bytes()).expect("lesen").json().expect("JSON");
    assert_eq!(got, want);
}

/// Review RU70.21: Eine Liste ist kein Bericht (Go lehnt sie ab).
#[test]
fn baseline_liste_abgelehnt() {
    assert!(read_summary(b"[]").is_err());
    assert!(read_summary(b"null").is_ok());
}
