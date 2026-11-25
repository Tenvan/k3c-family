//! Tests zu `tune.rs`, `tune_diff.rs` und `tuneparam.rs` (Port von `engine/balance/tune_test.go`, WZ28.6). Die Läufe sind
//! die kleinen des Go-Tests (ein Spieler, passive, ein Tag, Seed 1); die Zahlen kommen aus data/.

use std::sync::{Arc, Mutex};

use k3c_data::balance_targets::Targets;

use crate::sensitivity_tests::gold_targets;
use crate::tune::{TUNE_SEARCH, TUNE_SWEEP, Tune, TuneOptions, Tuning};
use crate::tune_diff::{locate_number, proposal_diff};
use crate::tuneparam::parse_tune_param;
use crate::variant_tests::data_on_disk;

const START_GOLD: &str = "economy.json:purse.startGold=5..35";

/// Das Testziel „Gold zur Dämmerung von Tag 1“ mit weitem Korridor 10–25. Mit Startgold 5 liegt der Wert bei 3 (verletzt),
/// mit 20 bei 18 im Korridor, mit 35 bei 33 darüber.
fn wide_gold() -> Targets {
    let mut ts = gold_targets();
    ts.targets[0].lower = Some(10.0);
    ts.targets[0].upper = Some(25.0);
    ts
}

fn specs(p: &[&str]) -> Vec<String> {
    p.iter().map(ToString::to_string).collect()
}

/// WZ28.6/AC-06: Sweep über 3 Punkte findet einen Punkt mit Score 0, der Vorschlag hat Begründung und Diff mit dem neuen Wert;
/// data/ bleibt unverändert.
#[test]
fn tune_sweep_vorschlag_mit_diff() {
    let before = data_on_disk();
    let progress = Arc::new(Mutex::new(Vec::new()));
    let log = progress.clone();
    let tn = wide_gold()
        .tune(TuneOptions {
            mode: TUNE_SWEEP.into(),
            params: specs(&[START_GOLD]),
            steps: 3,
            progress: Some(Box::new(move |done, total| log.lock().unwrap().push(done * 10 + total))),
            ..TuneOptions::default()
        })
        .expect("Abstimmen");
    let pts = &tn.points;
    assert!(
        pts.len() == 3 && pts[0].score != 0.0 && pts[1].score == 0.0 && pts[2].score != 0.0,
        "punkte: {pts:?}"
    );
    assert!(
        tn.best.values[0] == 20.0 && tn.best.pass && tn.params[0].base == 14.0 && tn.params[0].int,
        "bester: {:?}, params {:?}",
        tn.best,
        tn.params
    );
    check_proposal(&tn);
    let p = progress.lock().unwrap();
    assert!(p.len() == 3 && p[2] == 33, "fortschritt: {p:?}");
    assert!(data_on_disk() == before, "data/ wurde geändert");
    let json: serde_json::Value = serde_json::from_str(&tn.json().expect("JSON")).expect("lesbar");
    assert_eq!(json["best"]["values"][0], 20.0);
    assert_eq!(json["params"][0]["int"], true);
}

/// Diff mit altem und neuem Wert, Begründung mit Treffern und Werten.
fn check_proposal(tn: &Tuning) {
    assert!(
        tn.diff.contains(r#"+  "purse": { "startGold": 20, "maxGold": 30 },"#)
            && tn.diff.contains(r#"-  "purse": { "startGold": 14,"#)
            && tn.diff.contains("--- data/economy.json"),
        "diff:\n{}",
        tn.diff
    );
    assert!(
        tn.reason.contains("Alle 1 Korridore getroffen") && tn.reason.contains("= 20 (bisher 14)"),
        "begründung: {}",
        tn.reason
    );
}

/// Suche mit Budget 4: höchstens 4 Auswertungen, Abbruch beim ersten Punkt mit Score 0.
#[test]
fn tune_suche_budget() {
    let tn = wide_gold()
        .tune(TuneOptions {
            mode: TUNE_SEARCH.into(),
            params: specs(&[START_GOLD]),
            budget: 4,
            ..TuneOptions::default()
        })
        .expect("Suche");
    let pts = &tn.points;
    assert!(
        pts.len() <= 4 && tn.best.score == 0.0 && pts[pts.len() - 1].score == 0.0,
        "suche: {pts:?}"
    );
    assert!(
        pts[0].values[0] == 5.0 && pts[1].values[0] == 35.0,
        "erst die Enden: {pts:?}"
    );
    // Ohne Treffer bleibt die Begründung bei den verfehlten Korridoren.
    let tn = wide_gold()
        .tune(TuneOptions {
            mode: TUNE_SEARCH.into(),
            params: specs(&["economy.json:purse.startGold=1..2"]),
            budget: 2,
            ..TuneOptions::default()
        })
        .expect("Suche ohne Treffer");
    assert!(
        tn.points.len() == 2 && !tn.best.pass && tn.reason.contains("Verfehlt: Gold Tag 1"),
        "{tn:?}"
    );
}

/// Zwei Parameter: Raster und Verfeinerung bleiben im Budget.
#[test]
fn tune_zwei_parameter() {
    let tn = wide_gold()
        .tune(TuneOptions {
            mode: TUNE_SEARCH.into(),
            steps: 2,
            budget: 6,
            params: specs(&[START_GOLD, "economy.json:dawnGoldPerPlayer=1..9"]),
            ..TuneOptions::default()
        })
        .expect("Suche");
    assert!(tn.points.len() == 6 && tn.best.values.len() == 2, "{:?}", tn.points);
}

#[test]
fn tune_ungueltig() {
    let bad = [
        ("raten", specs(&[START_GOLD]), 0),
        (TUNE_SWEEP, vec![], 0),
        (TUNE_SWEEP, specs(&[START_GOLD, START_GOLD, START_GOLD]), 0),
        (TUNE_SWEEP, specs(&[START_GOLD]), 22),
        (TUNE_SWEEP, specs(&["economy.json:purse.startGold=9..1"]), 0),
        (TUNE_SWEEP, specs(&["economy.json:purse.startGold=1"]), 0),
        (TUNE_SWEEP, specs(&["economy.json:purse=1..2"]), 0),
        (TUNE_SWEEP, specs(&["fehlt.json:a=1..2"]), 0),
    ];
    for (mode, params, steps) in bad {
        let o = TuneOptions {
            mode: mode.into(),
            params: params.clone(),
            steps,
            ..TuneOptions::default()
        };
        assert!(
            wide_gold().tune(o).is_err(),
            "{mode} {params:?} {steps}: Fehler erwartet"
        );
    }
}

/// `locate_number` findet Zahlen in Listen und verschachtelten Objekten; der Diff nennt nur geänderte Zeilen.
#[test]
fn tune_diff_verschachtelt() {
    let raw = "{\n  \"a\": [1, {\"b\": 2.5}],\n  \"c\": {\"d\": 3}\n}\n";
    for (path, want) in [("a.0", "1"), ("a.1.b", "2.5"), ("c.d", "3")] {
        let keys: Vec<&str> = path.split('.').collect();
        let (s, e) = locate_number(raw, &keys).unwrap_or_else(|err| panic!("{path}: {err}"));
        assert_eq!(&raw[s..e], want, "{path}");
    }
    assert!(locate_number(raw, &["c"]).is_err(), "objekt als Zahl gefunden");
    assert!(locate_number(raw, &["c", "x"]).is_err(), "fehlender Pfad gefunden");
    let p = parse_tune_param("economy.json:dawnGoldPerPlayer=1..9").expect("Parameter");
    let d = proposal_diff(std::slice::from_ref(&p), &[p.base]).expect("Diff");
    assert!(d.is_empty(), "diff ohne Änderung: {d:?}");
}
