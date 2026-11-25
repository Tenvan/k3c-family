//! Tests zu `summary.rs` und `summary_text.rs` (Port von `engine/balance/summary_test.go`, BAL2).

use k3c_data::balance_targets::{Target, Targets};

use crate::evaluate::{STATUS_OK, STATUS_VIOLATED};
use crate::evaluate_tests::{metrics_with, report, seeds, share_target};
use crate::metrics::Metrics;
use crate::summary::{Summarize, Summary, compare, read_summary};

fn summarize_with(g: Target, ms: Vec<Option<Metrics>>) -> Summary {
    let mut ts = Targets {
        seeds: seeds(ms.len()),
        targets: vec![g],
        ..Targets::default()
    };
    ts.margin.share_pp = 5.0;
    let mut rep = report(ms);
    rep.matrix.seeds = ts.seeds.clone();
    ts.summarize(&rep)
}

fn held(fallen: bool) -> Option<Metrics> {
    Some(metrics_with(fallen, 1, &[]))
}

/// BAL2/AC-04: Der Bericht nennt je verletztem Ziel Kennzahl (Name), Szenario und die Seeds zum Nachspielen.
#[test]
fn bericht_nennt_seeds() {
    let s = summarize_with(share_target(), vec![held(false), held(true), held(true), held(false)]);
    let md = s.markdown_with(None, None);
    assert!(
        !s.pass && md.contains("| x | 2 Spieler, saver, Tiefe 0, 5 Tage | 50,0 % | 70–90 % | Fail (verletzt) | 2, 3 |"),
        "Zeile fehlt:\n{md}"
    );
    assert!(md.contains("Keine Baseline"), "Hinweis auf fehlende Baseline fehlt");
    let json = s.json().expect("JSON");
    assert!(
        json.contains("\"failSeeds\"") && json.contains("\"pass\": false"),
        "JSON: {json}"
    );
}

/// Ein abgebrochener Lauf erscheint mit seinem Seed als ungültig.
#[test]
fn bericht_ungueltig_mit_seed() {
    let md = summarize_with(share_target(), vec![held(false), None]).markdown_with(None, None);
    assert!(
        md.contains("Fail (ungültig)") && md.contains("ungültig: 2"),
        "ungültiger Lauf fehlt:\n{md}"
    );
}

/// BAL2/AC-05: Eine Wertänderung (hier die Untergrenze 70 → 85 %, als Test-Option statt Änderung an data/) lässt das Ziel von
/// ok nach verletzt kippen; der Vergleich nennt es, ein unverändertes Ziel nicht.
#[test]
fn vergleich_nennt_gekippte() {
    let ms = || (0..20).map(|i| held(i >= 16)).collect::<Vec<_>>(); // 80 % halten
    let y = Target {
        name: "y".into(),
        ..share_target()
    };
    let mut base = summarize_with(share_target(), ms());
    base.targets.extend(summarize_with(y.clone(), ms()).targets);
    assert_eq!(base.targets[0].verdict.status, STATUS_OK, "Baseline");
    let g = Target {
        lower: Some(85.0),
        upper: Some(100.0),
        ..share_target()
    };
    let mut cur = summarize_with(g, ms());
    cur.targets.extend(summarize_with(y, ms()).targets);
    let ch = compare(&base, &cur);
    assert!(
        ch.len() == 1
            && ch[0].name == "x"
            && ch[0].before == STATUS_OK
            && ch[0].after == STATUS_VIOLATED
            && ch[0].tipped,
        "Änderungen: {ch:?}"
    );
    let md = cur.markdown_with(Some(&base), None);
    let (_, md) = md.split_once("## Vergleich mit der Baseline").expect("Abschnitt");
    assert!(
        md.contains("- x: ok → verletzt **gekippt**") && !md.contains("- y:"),
        "Markdown:\n{md}"
    );
    assert!(
        compare(&base, &base).is_empty(),
        "gleiche Berichte dürfen nichts melden"
    );
}

/// Baseline lesen und schreiben; neues bzw. entfallenes Ziel.
#[test]
fn baseline_lesen() {
    let s = summarize_with(share_target(), vec![held(true), held(true)]);
    let back = read_summary(&s.json().expect("JSON")).expect("Baseline");
    assert!(
        back.targets.len() == 1 && back.targets[0].verdict.status == s.targets[0].verdict.status,
        "{back:?}"
    );
    assert_eq!(back, s, "Schreiben und Lesen ergeben denselben Bericht");
    let ch = compare(&Summary::default(), &s);
    assert!(ch.len() == 1 && ch[0].before == "neu", "neu: {ch:?}");
    let ch = compare(&s, &Summary::default());
    assert!(ch.len() == 1 && ch[0].after == "entfällt", "entfällt: {ch:?}");
}
