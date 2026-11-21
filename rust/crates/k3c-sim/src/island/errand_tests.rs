//! ECO1.7 (B-1197): Ausrüstung von oben. Unten ohne Werkstatt geht ein freier Bürger zur Treppe, holt oben das Schwert
//! und kehrt als Krieger zurück; Stufe 0 und Läufe ohne native Regeln bleiben unberührt.

use super::errand::gear_above;
use super::testutil::*;
use crate::state::{Troop, World};

/// Eine Insel mit Stufe 0 und 1: oben eine gebaute Werkstatt mit einem Schwert im Regal, unten eine gebaute Treppe nach
/// oben, ein freier Bürger und kein Gegner.
fn island() -> super::Island {
    let data = test_data();
    let mut isl = must_island(&data, "besorgung", &[0, 1]);
    let w0 = &mut isl.stages[0];
    let si = w0
        .sites
        .iter()
        .position(|s| s.kind == "workshop")
        .expect("Werkstatt oben");
    w0.sites[si].state = "built".into();
    w0.sites[si].swords = 1;
    w0.troops.retain(|t| t.kind != "peasant"); // oben soll niemand das Schwert vor dem Besorger holen
    let w1 = &mut isl.stages[1];
    assert!(
        !w1.sites.iter().any(|s| s.kind == "workshop"),
        "unten keine Werkstatt (ECO1.6)"
    );
    let up = w1
        .sites
        .iter()
        .position(|s| s.kind == "stairsUp")
        .expect("Treppe nach oben");
    w1.sites[up].state = "built".into();
    w1.troops.retain(|t| t.kind == "vagrant");
    peasant(w1);
    isl.sync_stock();
    isl
}

fn peasant(w: &mut World) {
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: "peasant".into(),
        x: w.hub_x,
        hp: 10.0,
        max_hp: 10.0,
        anchor_x: w.hub_x,
        target_x: w.hub_x,
        ..Troop::default()
    });
}

fn kinds(w: &World, kind: &str) -> usize {
    w.troops.iter().filter(|t| t.kind == kind).count()
}

#[test]
fn unten_ohne_werkstatt_wartet_ausruestung_oben() {
    let isl = island();
    assert!(gear_above(&isl, 1), "unten: oben liegt ein Schwert");
    assert!(!gear_above(&isl, 0), "Stufe 0 holt selbst");
}

#[test]
fn ohne_schwert_oben_geht_keiner() {
    let mut isl = island();
    isl.stages[0].sites.iter_mut().for_each(|s| s.swords = 0);
    assert!(!gear_above(&isl, 1), "nichts im Regal");
}

#[test]
fn buerger_holt_das_schwert_oben_und_kehrt_als_krieger_zurueck() {
    let data = test_data();
    let mut isl = island();
    let (mut went_up, mut back) = (false, false);
    for _ in 0..240 {
        run_island(&mut isl, &data, &[], 1.0);
        went_up |= isl.stages[0].troops.iter().any(|t| t.errand_from == 2);
        if kinds(&isl.stages[1], "warrior") == 1 {
            back = true;
            break;
        }
    }
    assert!(went_up, "der Bürger war oben");
    assert!(back, "ein Krieger steht unten");
    assert_eq!(kinds(&isl.stages[0], "warrior"), 0, "oben bleibt keiner hängen");
    assert!(
        isl.stages[0].troops.iter().all(|t| t.errand_from == 0),
        "keine Besorgung offen"
    );
    let swords: i64 = isl.stages[0].sites.iter().map(|s| s.swords).sum();
    assert_eq!(swords, 0, "das Schwert ist aus dem Regal");
}

#[test]
fn ohne_native_regeln_gibt_es_keine_besorgung() {
    let mut isl = island();
    let mut v = (*isl.values()).clone();
    v.native_rules = false;
    let v = std::sync::Arc::new(v);
    isl.stages
        .iter_mut()
        .for_each(|w| w.vals = Some(std::sync::Arc::clone(&v)));
    assert!(!gear_above(&isl, 1), "Go-Lauf");
}

/// Findet der Besorger oben nichts (hier: Gegner am Regal), pendelt er nicht, sondern wartet unten `FUTILE_WAIT` ab.
#[test]
fn ohne_fund_oben_kein_pendeln() {
    let data = test_data();
    let mut isl = island();
    isl.stages[0].sites.iter_mut().for_each(|s| s.hammers = 0);
    let ups = |isl: &super::Island| isl.stages[0].troops.iter().filter(|t| t.errand_from == 2).count();
    // oben nimmt ein anderer Bürger das Schwert (Auftrag fetchSword), sobald der Besorger oben ankommt
    let mut trips = 0;
    let mut was_up = false;
    for _ in 0..300 {
        if ups(&isl) == 1 && !was_up {
            was_up = true;
            trips += 1;
            let w0 = &mut isl.stages[0];
            w0.sites.iter_mut().for_each(|s| s.swords = 0); // das Regal ist leer, der Besuch war umsonst
        } else if ups(&isl) == 0 {
            was_up = false;
        }
        run_island(&mut isl, &data, &[], 1.0 / 30.0);
    }
    assert_eq!(trips, 1, "ein Weg, danach Wartezeit statt Pendeln");
}
