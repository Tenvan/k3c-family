//! Tests zu `sensitivity.rs`, `curves.rs` und `vary.rs` (Port von `engine/balance/sensitivity_test.go`, BAL3, B-301). Die
//! langen Tests (Go `-short` überspringt sie) sind `ignore = "lang"`; die Sensitivität selbst läuft hier nur mit den kleinen
//! Werten des Go-Tests.

use std::collections::BTreeMap;

use k3c_data::Source;
use k3c_data::balance_targets::{Target, Targets};

use crate::curves::{curves, grades};
use crate::island_world::{base_shared, load_shared_from};
use crate::prices::{load_prices, prices};
use crate::run::{Matrix, run};
use crate::sensitivity::{Sensitivity, SensitivityOptions};
use crate::targets::{KIND_MEDIAN, TEST_GOLD_MEASURE};
use crate::variant::{variant_source, with_variants};
use crate::variant_tests::start_gold;
use crate::vary::{RELOADED, STEPS, varied_source};

/// Ein Testziel „Median Gold zur Dämmerung von Tag 1“ (1 Spieler, passive, 1 Tag, Seed 1) mit Obergrenze 15 ohne Randbreite.
/// Der passive Bot gibt nichts aus (saver räumt seit B-652 erst Bäume): der Wert ist das Startgold 14; Startgold +25 % hebt ihn
/// über 15 (→ verletzt).
fn gold_targets() -> Targets {
    Targets {
        version: 1,
        seeds: vec!["1".into()],
        targets: vec![Target {
            measure: TEST_GOLD_MEASURE.into(),
            name: "Gold Tag 1".into(),
            kind: KIND_MEDIAN.into(),
            players: 1,
            bot: "passive".into(),
            days: 1,
            upper: Some(15.0),
            margin: Some(0.0),
            rule: "Test".into(),
            ..Target::default()
        }],
        ..Targets::default()
    }
}

fn paths(p: &[&str]) -> Vec<String> {
    p.iter().map(ToString::to_string).collect()
}

/// AC-03: ±10 % und ±25 % auf purse.startGold; +25 % kippt das Testziel und der Bericht nennt es mit vorher/nachher.
#[test]
fn sensitivity_nennt_gekippte_ziele() {
    let o = SensitivityOptions {
        paths: paths(&["economy.json:purse.startGold"]),
        ..SensitivityOptions::default()
    };
    let rep = gold_targets().sensitivity(&o).expect("Sensitivität");
    assert_eq!(rep.variations.len(), STEPS.len(), "Variationen");
    let v = &rep.variations[3];
    assert!(
        v.percent == 25 && v.before == 14.0 && v.after == 18.0,
        "Variation {v:?}"
    );
    assert!(
        v.flips.len() == 1
            && v.flips[0].before == "ok"
            && v.flips[0].after == "verletzt"
            && v.flips[0].value_after > 15.0,
        "gekippt: {:?}",
        v.flips
    );
    let md = rep.markdown();
    assert!(
        md.contains("Gold Tag 1 (1 Spieler, passive, Tiefe 0, 1 Tage): ok"),
        "Markdown nennt das gekippte Ziel nicht:\n{md}"
    );
    assert!(
        rep.variations[0].flips.is_empty(),
        "−25 % darf die Obergrenze nicht kippen: {:?}",
        rep.variations[0].flips
    );
}

/// Ein unbekannter Pfad oder kein Zahlenwert ist ein Fehler mit Pfad, noch vor dem ersten Lauf.
#[test]
fn sensitivity_unbekannter_pfad() {
    let ts = gold_targets();
    for p in [
        "economy.json:purse.gibtsNicht",
        "gibtsNicht.json:a",
        "economy.json:purse",
        "buildings.json:wall.levels.9.cost.gold",
        "economy.json",
    ] {
        let o = SensitivityOptions {
            paths: paths(&[p]),
            ..SensitivityOptions::default()
        };
        let err = ts.sensitivity(&o).expect_err(p);
        assert!(err.to_string().contains(p), "{p}: Fehler mit Pfad erwartet, war {err}");
    }
}

/// AC-04: je Grad aus data/difficulty.json eine Kurve über die Nächte; der Grad wirkt (ultra hat mehr Gegner als dev).
#[test]
fn kurven_je_grad() {
    let want = grades().expect("Grade");
    let cs = curves(&["1".to_string()], 1).expect("Kurven");
    let got: Vec<&str> = cs.iter().map(|c| c.grade.as_str()).collect();
    for c in &cs {
        assert!(c.nights.len() == 1 && c.invalid == 0, "{}: {c:?}", c.grade);
    }
    assert!(
        got == want && got[0] == "dev",
        "Grade {got:?}, erwartet {want:?} (dev zuerst)"
    );
    let mut enemies = BTreeMap::new();
    for g in ["dev", "ultra"] {
        let r = run(&Matrix {
            seeds: vec!["1".into()],
            players: vec![2],
            bots: vec!["saver".into()],
            depths: vec![0],
            days: 1,
            grade: g.into(),
            ..Matrix::default()
        })
        .expect("Lauf");
        let m = r.results[0].metrics.as_ref().expect("Kennzahlen");
        enemies.insert(g, m.waves[0].enemies);
    }
    assert!(
        enemies["ultra"] > enemies["dev"],
        "Gegner Welle 1: ultra {}, dev {}",
        enemies["ultra"],
        enemies["dev"]
    );
}

/// Gleiche Eingaben → byte-gleicher Bericht; danach rechnen Sim und Bots wieder mit den Originaldaten.
#[test]
#[ignore = "lang"]
fn sensitivity_byte_gleich_und_zurueckgesetzt() {
    let ts = gold_targets();
    let o = SensitivityOptions {
        paths: paths(&["economy.json:purse.startGold", "buildings.json:wall.cost.gold"]),
        steps: vec![25],
        nights: 1,
        ..SensitivityOptions::default()
    };
    let mut out = Vec::new();
    for _ in 0..2 {
        let rep = ts.sensitivity(&o).expect("Sensitivität");
        out.push(rep.json().expect("JSON") + &rep.markdown());
    }
    assert!(out[0] == out[1], "Bericht nicht byte-gleich");
    let orig = load_prices(&base_shared().expect("Daten").source).expect("Preise");
    let (_, start, _) = varied_source("economy.json:purse.startGold", 0).expect("Pfad");
    let g = start_gold();
    assert!(
        g as f64 == start && prices().wall_gold == orig.wall_gold,
        "nicht zurückgesetzt: Startgold {g}, Mauerpreis {} statt {}",
        prices().wall_gold,
        orig.wall_gold
    );
}

/// BAL6/AC-03 (B-301/AC-01): Ein Pfad in einer Datei, die die Sim nicht neu lädt, ist ein Fehler mit Pfad, bevor der erste
/// Lauf startet; ebenso als Wert-Variante.
#[test]
fn vary_ohne_wirkung() {
    let path = "difficulty.json:grades.hard.waveSize";
    let o = SensitivityOptions {
        paths: paths(&[path]),
        ..SensitivityOptions::default()
    };
    let err = gold_targets().sensitivity(&o).expect_err("Sensitivität");
    let text = err.to_string();
    assert!(
        text.contains(path) && text.contains("difficulty.json wird nicht neu geladen"),
        "Sensitivität: Fehler mit Pfad erwartet, war {text}"
    );
    let err = with_variants(&[format!("{path}=2")], || -> Result<(), _> {
        panic!("Lauf trotz Fehler")
    })
    .expect_err("Variante");
    assert!(
        err.to_string().contains("wird nicht neu geladen"),
        "Variante: Fehler erwartet, war {err}"
    );
}

/// Die Liste `RELOADED` passt zu dem, was `with_data` liest: Eine kaputte Datei daraus ist ein Fehler, und alle stehen in
/// data/. Gos Test prüft zusätzlich, dass `UseData` die übrigen Dateien nicht liest; `load_shared_from` liest alle Dateien,
/// darum gilt diese Richtung in Rust nicht (die Variation in einer anderen Datei lehnt `check_reloaded` trotzdem ab).
#[test]
fn reloaded_wie_use_data() {
    let files = base_shared().expect("Daten").source.files().expect("Dateien");
    for name in RELOADED {
        assert!(files.contains_key(name), "{name} fehlt in data/");
        let mut mem = variant_source(&[]).expect("Quelle");
        if let Source::Mem(m) = &mut mem {
            m.insert(name.to_string(), "{".into());
        }
        assert!(
            load_shared_from(mem).is_err(),
            "{name}: eine kaputte Datei wird nicht gelesen"
        );
    }
}
