//! Tests der reinen Helfer aus `common.rs`. Go hat dafür keine eigene Testdatei (`common.go` wird über die Sim-Golden
//! geprüft); hier die Fälle, die ohne spätere Sprints laufen. Pfade, die `pending.rs` rufen, sind `ignore` mit Sprint.

use super::common::*;
use super::{Player, ResourceNode, Site, Troop, World};

fn site(id: i64, kind: &str, x: f64, state: &str) -> Site {
    Site {
        id,
        kind: kind.into(),
        x,
        state: state.into(),
        ..Default::default()
    }
}

#[test]
fn approach_und_sign() {
    let cases = [
        (0.0, 10.0, 2.0, 1.0, 2.0),
        (9.5, 10.0, 2.0, 1.0, 10.0),
        (10.0, 0.0, 3.0, 1.0, 7.0),
        (5.0, 5.0, 1.0, 1.0, 5.0),
    ];
    for (x, target, speed, dt, want) in cases {
        assert_eq!(approach(x, target, speed, dt), want, "approach({x}, {target})");
    }
    assert_eq!((sign(2.5), sign(-0.1), sign(0.0)), (1.0, -1.0, 0.0));
}

#[test]
fn shielded_aus_den_drei_gruenden() {
    assert!(!shielded(&Player::default()));
    assert!(shielded(&Player {
        protected: true,
        ..Default::default()
    }));
    assert!(shielded(&Player {
        free: true,
        ..Default::default()
    }));
    assert!(shielded(&Player {
        stealth_for: 0.5,
        ..Default::default()
    }));
}

#[test]
fn outer_wall_nimmt_die_aeusserste_gebaute() {
    let w = World {
        hub_x: 100.0,
        sites: vec![
            site(1, "wall", 90.0, "built"),
            site(2, "wall", 80.0, "built"),
            site(3, "wall", 70.0, "unpaid"),
            site(4, "tower", 60.0, "built"),
            site(5, "wall", 120.0, "built"),
        ],
        ..Default::default()
    };
    assert_eq!(outer_wall(&w, -1.0).map(|s| s.id), Some(2));
    assert_eq!(outer_wall(&w, 1.0).map(|s| s.id), Some(5));
    assert!(outer_wall(&World::default(), 1.0).is_none());
}

#[test]
fn nachschlagen_per_id_inklusive_hub_platz() {
    let mut w = World {
        sites: vec![site(1, "wall", 1.0, "built")],
        hub_site: Some(site(9, "castle", 0.0, "built")),
        troops: vec![Troop {
            id: 4,
            ..Default::default()
        }],
        nodes: vec![ResourceNode {
            id: 6,
            ..Default::default()
        }],
        ..Default::default()
    };
    assert_eq!(site_by_id(&w, 1).map(|s| s.kind.as_str()), Some("wall"));
    assert_eq!(site_by_id(&w, 9).map(|s| s.kind.as_str()), Some("castle"));
    assert!(site_by_id(&w, 2).is_none());
    assert!(troop_by_id(&w, 4).is_some() && troop_by_id(&w, 5).is_none());
    assert!(node_by_id(&w, 6).is_some() && node_by_id(&w, 7).is_none());
    w.site_mut(9).unwrap().hp = 5.0;
    assert_eq!(w.hub_site.as_ref().unwrap().hp, 5.0);
    w.troop_mut(4).unwrap().hp = 2.0;
    assert_eq!(w.troops[0].hp, 2.0);
}

#[test]
fn is_worker_lebt_die_truppe() {
    let w = World {
        troops: vec![Troop {
            id: 4,
            ..Default::default()
        }],
        ..Default::default()
    };
    assert!(is_worker(&w, Some(4)));
    assert!(!is_worker(&w, Some(5)));
    assert!(!is_worker(&w, None));
    assert_eq!(int_ptr(3), Some(3));
}

#[test]
fn absorb_by_shield_zieht_den_schaden_ab() {
    let mut p = Player {
        shield: 5.0,
        shield_for: 9.0,
        ..Default::default()
    };
    assert!(absorb_by_shield(&mut p, 2.0));
    assert_eq!((p.shield, p.shield_for), (3.0, 9.0));
    assert!(absorb_by_shield(&mut p, 0.2)); // mindestens 1 Punkt
    assert_eq!(p.shield, 2.0);
    assert!(absorb_by_shield(&mut p, 50.0)); // Überschuss wird mit geschluckt
    assert_eq!((p.shield, p.shield_for), (0.0, 0.0));
    assert!(!absorb_by_shield(&mut p, 1.0));
}

#[test]
fn cause_or_other_ersetzt_leer() {
    assert_eq!(cause_or_other(""), CAUSE_OTHER);
    assert_eq!(cause_or_other("goblin"), "goblin");
}

#[test]
fn event_baut_ein_objekt() {
    let e = event(serde_json::json!({"type": "destroyed", "kind": "wall"}));
    assert_eq!(e["type"], "destroyed");
}

#[test]
#[ignore = "wartet auf RU18"]
fn destroy_site_macht_den_platz_wieder_leer() {
    // Ruft values, is_ruin (pending.rs); die Fälle der Go-Tests laufen mit RU18.
}
