//! Tests zu `bottle/mod.rs` (GS4.6 R1 bis R8, B-1261): Rhythmus, Zielpool über alle Stufen, Erscheinen, Aufheben,
//! Markierung über Stufen, Ziel erreicht und verloren.

use std::sync::Arc;

use super::*;
use crate::state::{Pickup, Player};

fn cfg() -> Arc<BottleData> {
    Arc::new(BottleData {
        first_day: 3,
        every_days: 4,
        edge_margin_min_units: 2.0,
        edge_margin_max_units: 6.0,
        pickup_range_units: 1.5,
        target_range_units: 1.5,
    })
}

fn truhe(x: f64) -> Pickup {
    Pickup {
        id: 1,
        kind: "chest".into(),
        x,
    }
}

/// Stufe der Tiefe 0 mit Konfiguration, Tag 3, und einer Truhe bei x = 50 als Ziel.
fn welt() -> World {
    let mut w = World {
        seed: "insel-1".into(),
        width_units: 100.0,
        bottle_cfg: Some(cfg()),
        ..Default::default()
    };
    w.biome.depth = 0;
    w.cycle.day = 3;
    w.pickups.push(truhe(50.0));
    w
}

/// Insel aus den gegebenen Stufen, Zielpool vorbereitet wie im Takt.
fn insel(stages: Vec<World>) -> Island {
    let mut isl = Island {
        stages,
        ..Default::default()
    };
    prepare(&mut isl);
    isl
}

fn monarch_at(x: f64) -> Player {
    Player {
        index: 0,
        x,
        free: false,
        ..Default::default()
    }
}

fn names(w: &World) -> Vec<String> {
    w.events
        .iter()
        .filter_map(|e| e.get("type").and_then(|v| v.as_str()).map(str::to_string))
        .collect()
}

#[test]
fn erste_flasche_am_tag_drei_mit_ziel() {
    let mut isl = insel(vec![welt()]);
    at_dawn(&mut isl.stages[0]);
    let w = &isl.stages[0];
    let x = w.bottle.ashore.expect("Flasche liegt");
    assert!(x <= 6.0 || x >= 94.0, "liegt am Inselende: {x}");
    assert_eq!(w.bottle.last_day, Some(3));
    assert!(names(w).contains(&"bottleAshore".to_string()));
}

#[test]
fn ohne_ziel_keine_flasche() {
    let mut w = welt();
    w.pickups.clear();
    let mut isl = insel(vec![w]);
    at_dawn(&mut isl.stages[0]);
    assert!(isl.stages[0].bottle.ashore.is_none(), "R3: ohne Ziel kein Erscheinen");
}

#[test]
fn ziel_auf_anderer_stufe_zaehlt() {
    // B-1261: Die einzige Truhe liegt auf Stufe 2; die Flasche erscheint trotzdem auf Stufe 1.
    let mut tief = World {
        width_units: 100.0,
        ..Default::default()
    };
    tief.biome.depth = 1;
    tief.pickups.push(truhe(30.0));
    let mut w = welt();
    w.pickups.clear();
    let mut isl = insel(vec![w, tief]);
    at_dawn(&mut isl.stages[0]);
    assert!(isl.stages[0].bottle.ashore.is_some(), "Zielpool über alle Stufen");
}

#[test]
fn rhythmus_wartet_everydays() {
    let mut isl = insel(vec![welt()]);
    isl.stages[0].bottle.last_day = Some(3);
    isl.stages[0].cycle.day = 5;
    at_dawn(&mut isl.stages[0]);
    assert!(
        isl.stages[0].bottle.ashore.is_none(),
        "vier Tage Abstand, Tag 5 zu früh"
    );
    isl.stages[0].cycle.day = 7;
    at_dawn(&mut isl.stages[0]);
    assert!(isl.stages[0].bottle.ashore.is_some(), "Tag 7 erlaubt");
}

#[test]
fn ort_ist_deterministisch() {
    let (mut a, mut b) = (insel(vec![welt()]), insel(vec![welt()]));
    at_dawn(&mut a.stages[0]);
    at_dawn(&mut b.stages[0]);
    assert_eq!(a.stages[0].bottle.ashore, b.stages[0].bottle.ashore);
}

#[test]
fn frieden_und_unter_tage_kein_erscheinen() {
    let mut w = welt();
    w.peace = true;
    let mut isl = insel(vec![w]);
    at_dawn(&mut isl.stages[0]);
    assert!(isl.stages[0].bottle.ashore.is_none(), "im Frieden keine Flasche");
    let mut t = welt();
    t.biome.depth = 1;
    t.bottle_cfg = None;
    at_dawn(&mut t);
    assert!(t.bottle.ashore.is_none(), "unter Tage keine Konfiguration");
}

#[test]
fn aufheben_setzt_markierung_auf_ein_ziel() {
    let mut isl = insel(vec![welt()]);
    isl.stages[0].bottle.ashore = Some(10.0);
    isl.stages[0].players.push(monarch_at(10.5));
    step_bottle(&mut isl);
    let w = &isl.stages[0];
    assert!(w.bottle.ashore.is_none());
    assert_eq!(w.bottle.found, 1);
    assert_eq!(
        w.bottle.target,
        Some(BottleTarget { stage: 0, x: 50.0 }),
        "einzige Truhe ist das Ziel"
    );
    assert!(names(w).contains(&"bottleFound".to_string()));
}

#[test]
fn einzelner_spieler_findet_flasche() {
    // R4 mit einem Spieler: die Flasche erscheint und wird aufgehoben, ohne zweiten Monarchen
    let mut isl = insel(vec![welt()]);
    isl.stages[0].bottle.ashore = Some(10.0);
    isl.stages[0].players.push(monarch_at(10.0));
    step_bottle(&mut isl);
    assert_eq!(isl.stages[0].bottle.found, 1);
}

#[test]
fn markierung_bleibt_ohne_monarch() {
    // Der Zielpool wird je Takt neu gelesen; eine laufende Markierung darf dadurch nicht verschwinden.
    let mut isl = insel(vec![welt()]);
    isl.stages[0].bottle.target = Some(BottleTarget { stage: 0, x: 50.0 });
    for _ in 0..3 {
        prepare(&mut isl);
        step_bottle(&mut isl);
    }
    assert_eq!(isl.stages[0].bottle.target, Some(BottleTarget { stage: 0, x: 50.0 }));
    assert!(!names(&isl.stages[0]).contains(&"bottleTargetReached".to_string()));
}

#[test]
fn abend_verliert_liegende_flasche() {
    let mut w = welt();
    w.bottle.ashore = Some(10.0);
    at_dusk(&mut w);
    assert!(w.bottle.ashore.is_none());
    assert!(names(&w).contains(&"bottleLost".to_string()));
}

#[test]
fn ziel_erreicht_loescht_markierung() {
    let mut isl = insel(vec![welt()]);
    isl.stages[0].bottle.target = Some(BottleTarget { stage: 0, x: 50.0 });
    isl.stages[0].players.push(monarch_at(50.2));
    step_bottle(&mut isl);
    assert!(isl.stages[0].bottle.target.is_none());
    assert!(names(&isl.stages[0]).contains(&"bottleTargetReached".to_string()));
}

#[test]
fn ziel_auf_anderer_stufe_wird_dort_erreicht() {
    let mut tief = World {
        width_units: 100.0,
        ..Default::default()
    };
    tief.biome.depth = 1;
    tief.pickups.push(truhe(30.0));
    tief.players.push(monarch_at(30.2));
    let mut isl = insel(vec![welt(), tief]);
    isl.stages[0].bottle.target = Some(BottleTarget { stage: 1, x: 30.0 });
    step_bottle(&mut isl);
    assert!(
        isl.stages[0].bottle.target.is_none(),
        "Monarch auf Stufe 2 erreicht das Ziel"
    );
}

#[test]
fn verschwundenes_ziel_endet_mit_ereignis() {
    let mut isl = insel(vec![welt()]);
    isl.stages[0].bottle.target = Some(BottleTarget { stage: 0, x: 70.0 });
    step_bottle(&mut isl);
    assert!(isl.stages[0].bottle.target.is_none(), "R6: Ziel besteht nicht mehr");
    assert!(names(&isl.stages[0]).contains(&"bottleTargetReached".to_string()));
}

#[test]
fn spielstand_rundlauf_erhaelt_flasche_und_markierung() {
    let mut w = welt();
    w.bottle = BottleState {
        ashore: Some(8.0),
        target: Some(BottleTarget { stage: 1, x: 50.0 }),
        last_day: Some(3),
        found: 2,
        lines: 0,
        pool_len: 0,
    };
    let s = w.bottle.to_save();
    assert_eq!(BottleState::from_save(&s), w.bottle);
    let json = serde_json::to_string(&s).expect("json");
    let back: BottleSave = serde_json::from_str(&json).expect("parse");
    assert_eq!(back, s);
}

#[test]
fn leerer_zustand_fehlt_im_spielstand() {
    assert!(BottleSave::default().is_empty());
    assert!(
        !BottleSave {
            found: 1,
            ..Default::default()
        }
        .is_empty()
    );
}

#[test]
fn geschichtszeile_wechselt_je_aufgehobene_flasche() {
    let mut w = welt();
    w.bottle.lines = 3;
    let mut isl = insel(vec![w]);
    for n in 1..=4 {
        isl.stages[0].bottle.ashore = Some(10.0);
        isl.stages[0].players = vec![monarch_at(10.0)];
        step_bottle(&mut isl);
        let line = isl.stages[0]
            .events
            .last()
            .and_then(|e| e.get("line"))
            .and_then(|v| v.as_i64())
            .expect("line");
        assert_eq!(line, (n - 1) % 3, "Flasche {n}");
        isl.stages[0].bottle.target = None;
    }
}
