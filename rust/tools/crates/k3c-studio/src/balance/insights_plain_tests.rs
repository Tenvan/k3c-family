//! Go `insights_plain_test.go`.

use super::evaluate::{STATUS_OK, STATUS_VIOLATED, Verdict};
use super::hints::{Hints, Lever};
use super::insights::{Insights, TargetInsight, target_insight};
use super::insights_levers::{LeverGroup, lever_groups, recommendations};
use super::insights_next::WHERE_CHAT;
use super::insights_plain::{LeverPlan, lever_range};
use super::summary::Entry;
use super::targets::{KIND_MEDIAN, KIND_SHARE, Target};

/// Ein Ziel samt Eintrag, wie `build_insights` es für den Bericht bildet.
fn plain_target(
    measure: &str,
    name: &str,
    kind: &str,
    lower: Option<f64>,
    upper: Option<f64>,
    value: f64,
) -> TargetInsight {
    let violated = lower.is_some_and(|lo| value < lo) || upper.is_some_and(|hi| value > hi);
    let e = Entry {
        verdict: Verdict {
            target: Target {
                measure: measure.to_owned(),
                name: name.to_owned(),
                kind: kind.to_owned(),
                lower,
                upper,
                ..Target::default()
            },
            value,
            status: if violated { STATUS_VIOLATED } else { STATUS_OK }.to_owned(),
            ..Verdict::default()
        },
        ..Entry::default()
    };
    let mut t = target_insight(&e, &Hints::default());
    t.runs = 5;
    t
}

fn castle(lower: f64, upper: f64, value: f64) -> TargetInsight {
    plain_target(
        "castleHeld",
        "Burg hält Nacht 1–5",
        KIND_SHARE,
        Some(lower),
        Some(upper),
        value,
    )
}

fn lever(path: &str, direction: &str, label: &str) -> Lever {
    Lever {
        path: path.to_owned(),
        direction: direction.to_owned(),
        step: 0.1,
        label: label.to_owned(),
        ..Lever::default()
    }
}

/// Das Beispiel aus der Beschwerde: Burg hält 100 % bei Korridor 75–90 %.
#[test]
fn anteil_zu_hoch() {
    let g = castle(75.0, 90.0, 100.0);
    assert_eq!(
        g.plain(),
        "In 5 von 5 Läufen hielt die Burg Nacht 1 bis 5 (100 %). Sollwert: 75–90 % der Läufe. Damit 10 \
         Prozentpunkte über dem Korridor, also zu leicht."
    );
    assert_eq!(g.gap_text(), "10 Pkt. zu hoch");
}

/// Der Hinweis „0,0 % bei Korridor 90–100 %, Abstand −900 %“ wird lesbar.
#[test]
fn anteil_zu_niedrig() {
    let got = castle(90.0, 100.0, 0.0).plain();
    for want in [
        "In 0 von 5 Läufen",
        "90–100 % der Läufe",
        "90 Prozentpunkte unter dem Korridor, also zu schwer.",
    ] {
        assert!(got.contains(want), "{want} fehlt in {got}");
    }
    assert!(!got.contains("900"), "Abstand in % der Korridorbreite: {got}");
}

#[test]
fn median_und_im_korridor() {
    let m = plain_target(
        "destroyedPerWave",
        "Zerstörte Gebäude je Welle",
        KIND_MEDIAN,
        None,
        Some(1.0),
        2.0,
    )
    .plain();
    assert!(
        m.contains("2 Gebäude")
            && m.contains("höchstens 1.")
            && m.contains("1 Gebäude über dem Korridor, also zu schwer"),
        "Median: {m}"
    );
    let ok = plain_target("castleHeld", "Burg", KIND_SHARE, Some(75.0), Some(90.0), 80.0);
    let got = ok.plain();
    assert!(
        got.contains("Im Korridor, 5 Prozentpunkte von der unteren Grenze entfernt."),
        "im Korridor: {got}"
    );
    assert_eq!(ok.gap_text(), "5 Pkt. Reserve");
}

#[test]
fn bereich_je_schritt() {
    let cases = [
        ("senken, Kommazahl", 100.0, false, "-", 0.1, Some((90.0, 70.0, 100.0))),
        ("erhöhen, Kommazahl", 10.0, false, "+", 0.2, Some((12.0, 10.0, 16.0))),
        ("ganze Zahl rundet", 14.0, true, "+", 0.1, Some((16.0, 14.0, 19.0))),
        ("kleine ganze Zahl", 2.0, true, "-", 0.1, Some((1.0, 1.0, 2.0))),
        ("Wert 0", 0.0, false, "+", 0.1, None),
    ];
    for (name, base, int, direction, step, want) in cases {
        assert_eq!(lever_range(base, int, direction, step), want, "{name}");
    }
}

#[test]
fn plan_text() {
    let g = castle(75.0, 90.0, 100.0);
    let l = lever("x.json:a", "-", "Wert senken");
    let p = g.plan_for(&l, Some((100.0, false)));
    for want in [
        "Aktuell 100.",
        "Probierbereich 70 … 100 (1 bis 3 Schritte à 10 % runter)",
        "Startvorschlag: 90.",
        "Wirkung: „Burg hält Nacht 1–5“ soll von 100,0 % Richtung Korridor 75–90 % sinken",
        "nicht berechenbar",
    ] {
        assert!(p.text.contains(want), "{want} fehlt in {}", p.text);
    }
    assert!(
        p.known && p.start == "90" && p.min == "70" && p.max == "100",
        "Plan: {p:?}"
    );
    let zero = g.plan_for(&l, Some((0.0, false)));
    assert!(
        !zero.known && zero.text.contains("nicht seriös abschätzbar"),
        "Wert 0: {zero:?}"
    );
    let missing = g.plan_for(&l, None);
    assert!(
        !missing.known && missing.text.contains("nicht lesbar"),
        "nicht lesbar: {missing:?}"
    );
}

/// Derselbe Wert bei mehreren Verletzungen steht einmal, mit allen Verletzungen; Gegenrichtung wird gemeldet.
#[test]
fn hebel_gruppen_fassen_zusammen() {
    let down = lever("x.json:a", "-", "A senken");
    let up = lever("x.json:a", "+", "A erhöhen");
    let other = lever("y.json:b", "-", "B senken");
    let with = |measure: &str, name: &str, value: f64, levers: &[&Lever]| {
        let mut g = plain_target(measure, name, KIND_SHARE, Some(75.0), Some(90.0), value);
        g.levers = levers.iter().map(|l| (*l).clone()).collect();
        g.plans = g
            .levers
            .iter()
            .map(|l| LeverPlan {
                path: l.path.clone(),
                label: l.label.clone(),
                short: "Kurz.".to_owned(),
                ..LeverPlan::default()
            })
            .collect();
        g
    };
    let insights = Insights {
        targets: vec![
            with("castleHeld", "Ziel A", 100.0, &[&down]),
            with("endbossDefeated", "Ziel B", 100.0, &[&other, &down]),
            with("caveBeforeDay6", "Ziel C", 10.0, &[&up]),
        ],
        ..Insights::default()
    };
    let groups = lever_groups(&insights);
    assert_eq!(groups.len(), 3, "Gruppen: {groups:?}");
    let first = &groups[0];
    assert!(
        first.path == "x.json:a" && first.direction == "-" && first.targets.len() == 2 && !first.conflict.is_empty(),
        "erste Gruppe: {first:?}"
    );
    assert!(
        first.text.contains("soll 2 Verletzung(en) verbessern: Ziel A; Ziel B") && first.text.contains("Gegenrichtung"),
        "Text: {}",
        first.text
    );
    assert!(
        groups.iter().all(|g| g.path != "y.json:b" || g.conflict.is_empty()),
        "y.json:b hat keinen Konflikt: {groups:?}"
    );
}

/// Ein Kopierblock fasst alle Startvorschläge zusammen; Gegenrichtung und unbekannte Startwerte stehen nur als
/// Kommentar.
#[test]
fn empfehlungen_als_ein_block() {
    let group = |path: &str, direction: &str, targets: &[&str], plan: LeverPlan| LeverGroup {
        path: path.to_owned(),
        direction: direction.to_owned(),
        targets: targets.iter().map(|t| (*t).to_owned()).collect(),
        plan,
        ..LeverGroup::default()
    };
    let known = LeverPlan {
        label: "A senken".to_owned(),
        known: true,
        current: "100".to_owned(),
        min: "70".to_owned(),
        max: "100".to_owned(),
        start: "90".to_owned(),
        ..LeverPlan::default()
    };
    let against = LeverPlan {
        known: true,
        start: "110".to_owned(),
        ..LeverPlan::default()
    };
    let unknown = LeverPlan {
        label: "B erhöhen".to_owned(),
        ..LeverPlan::default()
    };
    let groups = [
        group("x.json:a", "-", &["Ziel A", "Ziel B"], known),
        group("x.json:a", "+", &["Ziel C"], against),
        group("y.json:b", "+", &["Ziel D"], unknown),
    ];
    let c = recommendations(&groups).expect("Befehl");
    assert_eq!(c.place, WHERE_CHAT);
    for want in [
        "x.json:a=90  # A senken; aktuell 100, Bereich 70 … 100; verbessert: Ziel A; Ziel B",
        "# x.json:a: Gegenrichtung",
        "# y.json:b: Startwert nicht seriös",
    ] {
        assert!(c.text.contains(want), "{want} fehlt in\n{}", c.text);
    }
    assert!(!c.text.contains("=110"), "Gegenrichtung darf keinen Wert liefern");
    assert_eq!(recommendations(&[]), None, "ohne Stellschrauben kein Block");
}

/// B-669: Ein Ziel ohne Unter- und Obergrenze stürzt die Auswertung nicht ab.
#[test]
fn ziel_ohne_grenzen() {
    let t = TargetInsight {
        name: "Ohne Grenzen".to_owned(),
        value: 5.0,
        status: STATUS_OK.to_owned(),
        ..TargetInsight::default()
    };
    assert_eq!(t.target(), "Kein Sollwert hinterlegt.");
    assert!(t.plain().contains("Kein Sollwert hinterlegt."), "{}", t.plain());
    assert_eq!(t.gap_text(), "–");
}
