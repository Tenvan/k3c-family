//! Tests zu `insights_plain.rs` (Port von `engine/balance/insights_plain_test.go`). `TestLeverGroupsFassenZusammen` und
//! `TestRecommendationsEinBlock` stehen in `insights_levers_tests.rs`.

use k3c_data::balance_targets::Target;

use crate::evaluate::{STATUS_OK, STATUS_VIOLATED, Verdict};
use crate::hints::{Hints, Lever};
use crate::insights::{TargetInsight, target_insight};
use crate::insights_plain::{gap_text, lever_range, plain};
use crate::summary::Entry;
use crate::targets::{KIND_MEDIAN, KIND_SHARE};

/// Baut ein Ziel samt Eintrag, wie `build_insights` es für den Bericht bildet.
fn plain_target(
    measure: &str,
    name: &str,
    kind: &str,
    lo: Option<f64>,
    hi: Option<f64>,
    value: f64,
    runs: i64,
) -> TargetInsight {
    let violated = lo.is_some_and(|l| value < l) || hi.is_some_and(|h| value > h);
    let e = Entry {
        verdict: Verdict {
            target: Target {
                measure: measure.into(),
                name: name.into(),
                kind: kind.into(),
                lower: lo,
                upper: hi,
                ..Target::default()
            },
            value,
            status: if violated { STATUS_VIOLATED } else { STATUS_OK }.into(),
            ..Verdict::default()
        },
        ..Entry::default()
    };
    let mut t = target_insight(&e, &Hints::default());
    t.runs = runs;
    t
}

/// Das Beispiel aus der Beschwerde: Burg hält 100 % bei Korridor 75–90 %.
#[test]
fn plain_anteil_zu_hoch() {
    let g = plain_target(
        "castleHeld",
        "Burg hält Nacht 1–5",
        KIND_SHARE,
        Some(75.0),
        Some(90.0),
        100.0,
        5,
    );
    let want = "In 5 von 5 Läufen hielt die Burg Nacht 1 bis 5 (100 %). Sollwert: 75–90 % der Läufe. Damit 10 Prozentpunkte über dem Korridor, also zu leicht.";
    assert_eq!(plain(&g), want);
    assert_eq!(gap_text(&g), "10 Pkt. zu hoch");
}

/// Der Hinweis „0,0 % bei Korridor 90–100 %, Abstand −900 %“ wird lesbar.
#[test]
fn plain_anteil_zu_niedrig() {
    let g = plain_target(
        "castleHeld",
        "Burg hält Nacht 1–5",
        KIND_SHARE,
        Some(90.0),
        Some(100.0),
        0.0,
        5,
    );
    let got = plain(&g);
    for w in [
        "In 0 von 5 Läufen",
        "90–100 % der Läufe",
        "90 Prozentpunkte unter dem Korridor, also zu schwer.",
    ] {
        assert!(got.contains(w), "{w:?} fehlt in {got:?}");
    }
    assert!(
        !got.contains("900"),
        "Abstand in % der Korridorbreite darf nicht mehr vorkommen: {got:?}"
    );
}

#[test]
fn plain_median_und_im_korridor() {
    let m = plain_target(
        "destroyedPerWave",
        "Zerstörte Gebäude je Welle",
        KIND_MEDIAN,
        None,
        Some(1.0),
        2.0,
        5,
    );
    let got = plain(&m);
    assert!(
        got.contains("2 Gebäude")
            && got.contains("höchstens 1.")
            && got.contains("1 Gebäude über dem Korridor, also zu schwer"),
        "Median: {got:?}"
    );
    let ok = plain_target("castleHeld", "Burg", KIND_SHARE, Some(75.0), Some(90.0), 80.0, 5);
    let got = plain(&ok);
    assert!(
        got.contains("Im Korridor, 5 Prozentpunkte von der unteren Grenze entfernt."),
        "im Korridor: {got:?}"
    );
    assert_eq!(gap_text(&ok), "5 Pkt. Reserve");
}

#[test]
fn lever_range_faelle() {
    // (Name, Basis, ganzzahlig, Richtung, Schritt, erwartet (Start, von, bis) oder None)
    type Fall = (&'static str, f64, bool, &'static str, f64, Option<(f64, f64, f64)>);
    let cases: [Fall; 5] = [
        ("senken, Kommazahl", 100.0, false, "-", 0.1, Some((90.0, 70.0, 100.0))),
        ("erhöhen, Kommazahl", 10.0, false, "+", 0.2, Some((12.0, 10.0, 16.0))),
        ("ganze Zahl rundet", 14.0, true, "+", 0.1, Some((16.0, 14.0, 19.0))),
        ("kleine ganze Zahl", 2.0, true, "-", 0.1, Some((1.0, 1.0, 2.0))),
        ("Wert 0", 0.0, false, "+", 0.1, None),
    ];
    for (name, base, is_int, dir, step, want) in cases {
        let got = lever_range(base, is_int, dir, step);
        match (got, want) {
            (Some(g), Some(w)) => {
                let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
                assert!(close(g.0, w.0) && close(g.1, w.1) && close(g.2, w.2), "{name}: {g:?}");
            }
            (None, None) => {}
            _ => panic!("{name}: {got:?}"),
        }
    }
}

#[test]
fn plan_text() {
    let g = plain_target(
        "castleHeld",
        "Burg hält Nacht 1–5",
        KIND_SHARE,
        Some(75.0),
        Some(90.0),
        100.0,
        5,
    );
    let l = Lever {
        path: "x.json:a".into(),
        direction: "-".into(),
        step: 0.1,
        label: "Wert senken".into(),
        ..Lever::default()
    };
    let p = g.plan_for(&l, Some((100.0, false)));
    for w in [
        "Aktuell 100.",
        "Probierbereich 70 … 100 (1 bis 3 Schritte à 10 % runter)",
        "Startvorschlag: 90.",
        "Wirkung: „Burg hält Nacht 1–5“ soll von 100,0 % Richtung Korridor 75–90 % sinken",
        "nicht berechenbar",
    ] {
        assert!(p.text.contains(w), "{w:?} fehlt in {:?}", p.text);
    }
    assert!(
        p.known && p.start == "90" && p.min == "70" && p.max == "100",
        "Plan: {p:?}"
    );
    let z = g.plan_for(&l, Some((0.0, false)));
    assert!(!z.known && z.text.contains("nicht seriös abschätzbar"), "Wert 0: {z:?}");
    let n = g.plan_for(&l, None);
    assert!(!n.known && n.text.contains("nicht lesbar"), "nicht lesbar: {n:?}");
}

/// B-669: Ein Ziel ohne Unter- und Obergrenze stürzt die Auswertung nicht ab.
#[test]
fn ziel_ohne_grenzen_stuerzt_nicht_ab() {
    let ti = TargetInsight {
        name: "Ohne Grenzen".into(),
        value: 5.0,
        status: STATUS_OK.into(),
        ..TargetInsight::default()
    };
    let s = plain(&ti);
    assert!(s.contains("Kein Sollwert hinterlegt."), "plain() = {s:?}");
    let _ = gap_text(&ti);
}
