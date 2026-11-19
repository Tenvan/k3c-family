//! Tests der reinen Helfer aus `common.rs`. Go hat dafür keine eigene Testdatei (`common.go` wird über die Sim-Golden
//! geprüft); hier die Fälle, die ohne spätere Sprints laufen. Pfade, die `links.rs` rufen, sind `ignore` mit Sprint.

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

/// Go: `TestZerstoerteMauerIstWiederLeer` (rules_test.go) ohne `Step`: Der Bauplatz ist wieder leer und muss neu bezahlt
/// werden; eine Ruine der Planung bleibt Ruine; Türme und Aufträge am Platz werden frei.
#[test]
fn destroy_site_macht_den_platz_wieder_leer() {
    let mut w = World {
        next_id: 1,
        ..World::default()
    };
    let wall_hp = super::links::values(&w).buildings["wall"].hp;
    let mut wall = site(7, "wall", 100.0, "built");
    wall.hp = 5.0;
    wall.paid_gold = 5;
    wall.level = 2;
    wall.upgrade = "waitingMaterial".into();
    w.sites.push(wall);
    w.sites.push(site(8, "wall", 140.0, "built"));
    w.level.ruins.push(k3c_level::types::Ruin {
        kind: "wall".into(),
        x: 140.0,
    });
    w.troops.push(Troop {
        id: 20,
        tower_id: Some(7),
        job: Some(crate::state::Job {
            r#type: "build".into(),
            site_id: 7,
            ..Default::default()
        }),
        ..Troop::default()
    });
    destroy_site(&mut w, 7);
    let s = &w.sites[0];
    assert!(
        s.state == "unpaid" && s.paid_gold == 0 && s.hp == 0.0 && s.max_hp == wall_hp && s.x == 100.0 && s.id == 7,
        "Mauer nach Zerstörung: {s:?}"
    );
    assert!(
        s.level == 0 && s.upgrade.is_empty() && !s.ruin,
        "Stufe und Ausbau gehen verloren: {s:?}"
    );
    assert!(
        w.troops[0].tower_id.is_none() && w.troops[0].job.is_none(),
        "Turm und Bau-Auftrag frei"
    );
    assert_eq!(w.events.len(), 1);
    assert!(w.events[0]["type"] == "destroyed" && w.events[0]["kind"] == "wall");
    destroy_site(&mut w, 8);
    assert!(
        w.sites[1].ruin && w.sites[1].state == "unpaid",
        "eine Ruine bleibt Ruine"
    );
}

/// B-1087: Ein gebauter Platz auf 0 HP panict nicht mehr; ohne Schutz wird er zerstört, in einer geschützten Nacht
/// bleibt er mit 1 HP stehen (Go: `applyDamageBy`, `protectedNight`).
#[test]
fn platz_auf_null_hp_zerstoert_oder_haelt_in_geschuetzter_nacht() {
    use super::island_ref::IslandRef;
    let mut wall = site(7, "wall", 100.0, "built");
    wall.hp = 5.0;
    wall.max_hp = 5.0;
    let mut w = World {
        sites: vec![wall],
        ..Default::default()
    };
    w.cycle.phase = "night".into();
    w.cycle.day = 1;
    let mut isl = IslandRef::default();
    isl.grade.protected_nights = 2;
    apply_damage_by(&mut w, 7, 9.0, "goblin", Some(&isl));
    assert!(
        w.sites[0].state == "built" && w.sites[0].hp == 1.0,
        "geschützt: {:?}",
        w.sites[0]
    );
    apply_damage_by(&mut w, 7, 9.0, "goblin", None);
    assert_eq!(w.sites[0].state, "unpaid", "ungeschützt zerstört: {:?}", w.sites[0]);
}
