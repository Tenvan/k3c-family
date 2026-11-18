//! Tests zu `radar.rs` (Go hat keinen eigenen Test für `radar.go`; Stichproben der reinen Rechnung).

use super::radar::*;
use crate::view::cell::Camera;
use k3c_level::types::Entity;
use k3c_sim::state::{Enemy, Player, Site, World};

#[test]
fn unit_pos_klemmt_und_nullbreite() {
    assert_eq!(unit_pos(50.0, 100.0), 0.5);
    assert_eq!(unit_pos(-5.0, 100.0), 0.0);
    assert_eq!(unit_pos(500.0, 100.0), 1.0);
    assert_eq!(unit_pos(5.0, 0.0), 0.0);
}

#[test]
fn radar_marker_reihenfolge() {
    let mut w = World {
        width_units: 100.0,
        portals: vec![10.0],
        sites: vec![
            Site {
                kind: "stairsUp".to_string(),
                x: 20.0,
                ..Default::default()
            },
            Site {
                kind: "wall".to_string(),
                x: 30.0,
                ..Default::default()
            },
        ],
        enemies: vec![Enemy {
            x: 40.0,
            ..Default::default()
        }],
        players: vec![Player {
            index: 1,
            x: 50.0,
            ..Default::default()
        }],
        ..Default::default()
    };
    w.castle.x = 5.0;
    w.level.entities = vec![Entity {
        kind: "exit".to_string(),
        x: 90.0,
        ..Default::default()
    }];
    let mut out = Vec::new();
    radar_markers(&mut out, &w, 1);
    let kinds: Vec<_> = out.iter().map(|m| m.kind).collect();
    assert_eq!(
        kinds,
        [
            RadarKind::Castle,
            RadarKind::Portal,
            RadarKind::Exit,
            RadarKind::Stairs,
            RadarKind::Enemy,
            RadarKind::Player
        ]
    );
    assert!(out[5].own && !out[5].down && out[5].index == 1 && out[5].pos == 0.5);
    radar_markers(&mut out, &w, -1);
    assert_eq!(out.len(), 6, "Puffer wird geleert");
    assert!(!out[5].own);
}

#[test]
fn radar_view_und_groesse() {
    let k = Camera {
        x: 50.0,
        scale: 10.0,
        center: 100.0,
        half: 100.0,
    };
    let (from, to) = radar_view(&k, 100.0);
    assert_eq!((from, to), (0.4, 0.6));
    assert_eq!(radar_size(800.0, 1.0), (400.0, 20.0));
    assert_eq!(radar_size(4000.0, 2.0), (1280.0, 40.0));
}

fn radar_world() -> World {
    let site = |kind: &str, x: f64| Site {
        kind: kind.to_string(),
        x,
        ..Default::default()
    };
    let entity = |kind: &str, x: f64| Entity {
        kind: kind.to_string(),
        x,
        ..Default::default()
    };
    let mut w = World {
        width_units: 1000.0,
        portals: vec![100.0, 900.0],
        sites: vec![site("stairsUp", 250.0), site("stairsDown", 750.0), site("wall", 300.0)],
        enemies: [0.0, 1000.0, -20.0, 1300.0, 500.0]
            .iter()
            .map(|&x| Enemy {
                x,
                ..Default::default()
            })
            .collect(),
        players: vec![
            Player {
                index: 0,
                x: 500.0,
                ..Default::default()
            },
            Player {
                index: 1,
                x: 250.0,
                crown: Some(Default::default()),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    w.castle.x = 500.0;
    w.level.entities = vec![entity("tree", 40.0), entity("exit", 1000.0)];
    w
}

/// Go: `kampf_test.go › TestRadarMarker` (Radar-Teil von RU20.3).
#[test]
fn radar_marker() {
    let mut ms = Vec::new();
    radar_markers(&mut ms, &radar_world(), 0);
    let pos = |kind: RadarKind| -> Vec<f64> { ms.iter().filter(|m| m.kind == kind).map(|m| m.pos).collect() };
    let checks = [
        (RadarKind::Castle, vec![0.5]),
        (RadarKind::Portal, vec![0.1, 0.9]),
        (RadarKind::Exit, vec![1.0]),
        (RadarKind::Stairs, vec![0.25, 0.75]),
        (RadarKind::Enemy, vec![0.0, 1.0, 0.0, 1.0, 0.5]),
        (RadarKind::Player, vec![0.5, 0.25]),
    ];
    for (kind, want) in checks {
        assert_eq!(pos(kind), want, "Art {kind:?}");
    }
    let last = &ms[ms.len() - 2..];
    assert!(
        last[0].kind == RadarKind::Player && last[0].own && !last[1].own && last[1].down && last[1].index == 1,
        "Monarchen zuletzt, nur der eigene eigen, gefallen markiert: {last:?}"
    );
    let mut w0 = World::default();
    w0.castle.x = 5.0;
    radar_markers(&mut ms, &w0, -1);
    assert_eq!(ms[0].pos, 0.0, "Breite 0 rechnet nicht durch null");
}

/// Go: `kampf_test.go › TestRadarAusschnittUndGroesse`.
#[test]
fn radar_ausschnitt_und_groesse() {
    let k = Camera {
        x: 500.0,
        scale: 10.0,
        half: 500.0,
        ..Default::default()
    };
    assert_eq!(radar_view(&k, 1000.0), (0.45, 0.55), "Ausschnitt");
    let k0 = Camera {
        x: 0.0,
        scale: 10.0,
        half: 500.0,
        ..Default::default()
    };
    assert_eq!(radar_view(&k0, 1000.0), (0.0, 0.05), "am Rand begrenzt");
    assert_eq!(radar_size(1920.0, 1.0), (640.0, 20.0), "1920");
    assert_eq!(radar_size(1000.0, 1.0).0, 500.0, "schmal: halbe Breite");
}
