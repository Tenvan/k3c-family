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
    w0.troops.retain(|t| !t.serving()); // oben soll niemand das Schwert vor dem Besorger holen
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
    w1.troops.retain(|t| t.is_wild());
    peasant(w1);
    isl.sync_stock();
    isl
}

fn peasant(w: &mut World) {
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: "citizen".into(),
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

/// EQ1.5 (B-1209) AC-01: Der Besorger bleibt über den Wechsel als Figur erhalten (Art, Leben) und meldet Verlassen und
/// Erscheinen an der Treppe, damit die Anzeige ein- und ausblendet.
#[test]
fn besorger_bleibt_ueber_den_wechsel_erhalten() {
    let data = test_data();
    let mut isl = island();
    let (hp, max_hp) = {
        let t = isl.stages[1].troops.iter().find(|t| !t.is_wild()).expect("Bürger");
        (t.hp, t.max_hp)
    };
    let stairs_down = super::errand::built_stairs(&isl.stages[0], "stairsDown").unwrap_or(isl.stages[0].hub_x);
    let (mut arrived, mut left, mut shown) = (false, false, false);
    let has = |ev: &[crate::state::Event], t: &str| ev.iter().any(|e| e.get("type") == Some(&t.into()));
    for _ in 0..240 {
        let ev = run_island(&mut isl, &data, &[], 1.0);
        left |= has(&ev[1], "stairsLeave");
        shown |= has(&ev[0], "stairsArrive");
        if let Some(t) = isl.stages[0].troops.iter().find(|t| t.errand_from == 2) {
            arrived = true;
            assert_eq!((t.kind.as_str(), t.hp, t.max_hp), ("citizen", hp, max_hp));
            assert!((t.x - stairs_down).abs() < 40.0, "oben beginnt der Weg an der Treppe");
            break;
        }
    }
    assert!(arrived, "der Besorger erscheint oben");
    assert!(left && shown, "Verlassen und Erscheinen werden gemeldet");
}

/// AC-03: gleiche Eingabe, gleicher Ablauf.
#[test]
fn besorgung_ist_deterministisch() {
    let data = test_data();
    let snap = |isl: &super::Island| -> Vec<(usize, i64, String, f64)> {
        isl.stages
            .iter()
            .enumerate()
            .flat_map(|(i, w)| w.troops.iter().map(move |t| (i, t.errand_from, t.kind.clone(), t.x)))
            .collect()
    };
    let (mut a, mut b) = (island(), island());
    for _ in 0..120 {
        run_island(&mut a, &data, &[], 1.0);
        run_island(&mut b, &data, &[], 1.0);
        assert_eq!(snap(&a), snap(&b));
    }
}

/// Ausnahmefall: Die Treppe ist weg, der Besorger bricht ab und bleibt in seiner Stufe.
#[test]
fn ohne_treppe_bleibt_der_besorger_unten() {
    let data = test_data();
    let mut isl = island();
    isl.stages[1].sites.iter_mut().for_each(|s| {
        if s.kind == "stairsUp" {
            s.state = "unpaid".into();
        }
    });
    run_island(&mut isl, &data, &[], 30.0);
    assert!(
        isl.stages[0].troops.iter().all(|t| t.errand_from == 0),
        "keiner ging hinauf"
    );
}
