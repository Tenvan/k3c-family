//! Tests zu `listen.go`. `TestListenJeStufeUndPreisschild` baut seine Welt mit `sim.CreateIsland` (RU10) und rechnet mit
//! der echten Wirtschaft (RU18): `#[ignore = "wartet auf RU10"]`.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use std::rc::Rc;

use k3c_sim::state::{Pickup, World};

use super::listen::{Infos, Opening, gone, open_alpha};
use super::sites::Candidate;
use super::testutil::FixedSimView;
use crate::view::frame::{Frame, Stage};

/// Go: `island(t)`; die Insel kommt mit RU10.
fn island() -> World {
    unimplemented!("RU10: create_island")
}

fn cand(id: i64, x: f64) -> Candidate {
    Candidate { id, x }
}

fn chest(id: i64, x: f64) -> Pickup {
    Pickup {
        id,
        kind: "chest".to_string(),
        x,
    }
}

fn frame(clock: f64, w: Option<&World>) -> Frame {
    Frame {
        clock,
        stages: w
            .map(|w| {
                vec![Stage {
                    index: 0,
                    level: "forest".to_string(),
                    world: Some(Rc::new(w.clone())),
                    ..Default::default()
                }]
            })
            .unwrap_or_default(),
        ..Default::default()
    }
}

#[test]
fn truhe_oeffnet_sich_kurz() {
    let prev = [cand(1, 10.0), cand(2, 20.0)];
    let now = [cand(2, 20.0)];
    let open = gone(Vec::new(), &prev, &now, 5.0);
    assert_eq!(open, vec![Opening { x: 10.0, start: 5.0 }], "aufgehobene Truhe");
    assert!(
        open_alpha(&open[0], 5.1) == 1.0 && open_alpha(&open[0], 5.55) == 0.5 && open_alpha(&open[0], 5.9) == 0.0,
        "offen 0,3 s, dann bis 0,8 s ausgeblendet"
    );
    let open = gone(open, &now, &now, 5.5);
    assert_eq!(open.len(), 1, "läuft noch");
    let open = gone(open, &now, &now, 6.0);
    assert!(open.is_empty(), "nach 0,8 s weg: {open:?}");
}

#[test]
#[ignore = "wartet auf RU10"]
fn listen_je_stufe_und_preisschild() {
    // Ablauf wie Go: Listen legt je Stufe `flags` an, `info(1)` kennt keine Stufe, ein Frame ohne Spieler löscht die Stufe.
    let w = island();
    let v = FixedSimView::default();
    let mut d = Infos::default();
    d.listen(&frame(0.0, Some(&w)), &v);
    let info = d.info(0).expect("Zustand der Stufe fehlt");
    assert_eq!(info.flags.len(), w.sites.len());
    assert!(d.info(1).is_none(), "unbekannte Stufe");
    d.listen(&frame(0.0, None), &v);
    assert!(d.info(0).is_none(), "Stufe ohne Spieler fällt aus dem Frame");
}

/// Kommt eine Stufe nach einer Lücke wieder in den Frame, öffnet sich keine Truhe, die inzwischen verschwunden ist.
#[test]
fn wiedereintritt_ohne_offene_truhe() {
    let v = FixedSimView::default();
    let mut w = World {
        pickups: vec![chest(1, 10.0)],
        ..Default::default()
    };
    let mut d = Infos::default();
    d.listen(&frame(1.0, Some(&w)), &v);
    d.listen(&frame(2.0, None), &v);
    w.pickups.clear();
    d.listen(&frame(3.0, Some(&w)), &v);
    assert_eq!(
        d.info(0).expect("Stufe 0").openings.len(),
        0,
        "Truhen öffnen sich nach dem Wiedereintritt"
    );
    w.pickups = vec![chest(2, 12.0)];
    d.listen(&frame(3.0, Some(&w)), &v);
    w.pickups.clear();
    d.listen(&frame(3.0, Some(&w)), &v);
    assert_eq!(
        d.info(0).expect("Stufe 0").openings.len(),
        1,
        "aufgehobene Truhe im laufenden Frame: erwartet 1"
    );
}

#[test]
fn preisschild_am_naechsten_platz_und_flag_ohne_welt() {
    use super::listen::{info_flag, info_tagged};
    use super::testutil::wall_at;
    let v = FixedSimView {
        payable: true,
        gold_left: (5, true),
        ..Default::default()
    };
    // paid_gold > 0: `site_visible` ist sofort wahr (die Hub-Stufe des Platzes rechnet erst RU18).
    let mut site = wall_at(7, 10.0, 0, "unpaid");
    site.paid_gold = 1;
    let w = World {
        sites: vec![site],
        players: vec![k3c_sim::state::Player {
            x: 10.5,
            ..Default::default()
        }],
        hub_level: 1,
        ..Default::default()
    };
    let mut d = Infos::default();
    d.listen(&frame(0.0, Some(&w)), &v);
    let info = d.info(0).expect("Stufe 0");
    assert_eq!(info.flags.len(), 1);
    assert!(info.flags[0].payable && info.flags[0].gold_left == 5);
    assert!(
        info_tagged(Some(info), 7),
        "Preisschild am sichtbaren, bezahlbaren Platz"
    );
    assert!(info_flag(None, 3).visible, "ohne Zustand sichtbar");
    assert!(!info_tagged(None, 7));
}
