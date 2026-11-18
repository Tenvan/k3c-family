//! Tests zu `plan_combat.go`. `TestCombatPlan` ruft in Go `plan` ganz; `plan` braucht `decor` (RU16.5), darum ruft der
//! Test hier `combat` (das den Balken und die Krone legt) direkt, mit derselben Welt und `Camera::default()`.

use k3c_sim::state::{Crown, Player, World};

use super::pending::{CombatSpot, SpotState};
use super::plan::{Planner, Shape};
use super::plan_combat::portal_down;
use super::testutil::{FixedSimView, StubSheets};
use crate::view::cell::Camera;

#[test]
fn combat_plan() {
    let w = World {
        portals: vec![10.0, 30.0],
        portal_hp: vec![50.0, 100.0],
        portal_max_hp: 100.0,
        portals_down: vec![50.0],
        players: vec![Player {
            index: 0,
            crown: Some(Crown {
                state: "ground".to_string(),
                x: 12.0,
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    };
    let mut p = Planner::new(None);
    p.combat(&w, None, &Camera::default(), &FixedSimView::default());
    assert!(!p.top.is_empty(), "keine Balken und keine Krone gezeichnet");
    assert!(
        portal_down(&w, 50.0) && !portal_down(&w, 10.0),
        "portal_down erkennt zerstörte Portale falsch"
    );
}

/// Das Gestell (B-942) erscheint erst ab Hub-Stufe 2 und trägt dann je gebautem, nicht abgeholtem Schild ein Bild mehr.
/// Gebaute Schilde setzt nur die Sim (Test dort); die Sicht liefert hier `FixedSimView::shield_rack`.
#[test]
fn shield_rack_stack() {
    let cases = [
        (1, 0, (100.0, 0, false), 0),
        (2, 0, (100.0, 0, true), 1),
        (2, 2, (100.0, 2, true), 3),
    ];
    for (hub_level, shields, rack, want) in cases {
        let w = World {
            hub_level,
            hub_x: 100.0,
            ..Default::default()
        };
        let v = FixedSimView {
            shield_rack: rack,
            ..Default::default()
        };
        let mut p = Planner::new(Some(StubSheets::rc()));
        p.shield_rack(&w, &v);
        assert_eq!(
            p.body.len(),
            want,
            "Stufe {hub_level}, {shields} Schilde: {} Bilder, erwartet {want}",
            p.body.len()
        );
    }
}

/// Zahlstelle: Icon je Art, Preis nur für den Monarchen ohne Krone in Reichweite (Go: `spot`).
#[test]
fn zahlstelle_mit_preis_nur_fuer_den_monarchen() {
    let w = World {
        players: vec![Player::default()],
        ..Default::default()
    };
    let v = FixedSimView {
        spot_near: true,
        spot_state: SpotState {
            price: 3,
            paid: 1,
            payable: true,
            near: true,
        },
        ..Default::default()
    };
    let s = CombatSpot {
        kind: "trigger".to_string(),
        side: 1,
        x: 20.0,
    };
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.spot(&w, &s, &v);
    assert_eq!(p.body.len(), 1, "Horn-Icon");
    assert!(p.top.is_empty(), "ohne Monarch kein Preis");
    p.viewer = Some(0);
    p.spot(&w, &s, &v);
    let rings = p.top.iter().filter(|it| it.shape == Shape::Ring).count();
    let fills = p.top.iter().filter(|it| it.shape == Shape::Circle).count();
    assert_eq!((rings, fills), (3, 1), "3 Münz-Plätze, einer bezahlt");
}
