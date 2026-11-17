//! Port von `native/k3c-game/world/camera_test.go`.

use std::rc::Rc;

use k3c_sim::state::{Castle, Player, World};

use super::*;
use crate::view::cell::{new_camera, rect};
use crate::view::frame::{Seat, Stage};

/// testWorld ist eine Stufe der Breite 100 Units mit Burg bei 50 und Monarchen 0 bei x.
fn test_world(x: f64) -> World {
    let mut w = World {
        width_units: 100.0,
        castle: Castle {
            x: 50.0,
            ..Castle::default()
        },
        players: vec![Player {
            index: 0,
            x,
            ..Player::default()
        }],
        seed: "a".into(),
        ..World::default()
    };
    w.biome.id = "forest".into();
    w
}

/// testFrame stellt Spieler 0 in die Welt w (Stufe 0).
fn test_frame(w: World) -> Frame {
    Frame {
        stages: vec![Stage {
            index: 0,
            level: "forest".into(),
            world: Some(Rc::new(w)),
            ..Stage::default()
        }],
        seats: vec![Seat {
            player: 0,
            monarch: 0,
            stage: 0,
            ..Seat::default()
        }],
        ..Frame::default()
    }
}

/// Setzt den Monarchen 0 der Stufe 0 auf x (Go ändert `World.Players[0].X` im geteilten Zeiger).
fn set_monarch_x(f: &mut Frame, x: f64) {
    let w = Rc::get_mut(f.stages[0].world.as_mut().unwrap()).unwrap();
    w.players[0].x = x;
}

/// testCell ist eine Zelle 320 × 300 px mit 10 px je Unit: sichtbar sind 32 Units (halb 16).
fn test_cell(player: i64, x: f64) -> Cell {
    let r = rect(0, 0, 320, 300);
    Cell {
        index: player,
        player,
        rect: r,
        camera: new_camera(r, x, 10.0),
    }
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

/// Erstes Bild (prev ohne Maßstab): Die Kamera springt auf den Monarchen und behält Lage und Maßstab der Zelle.
#[test]
fn follow_jumps_first() {
    let mut s = Cameras::default();
    let c = test_cell(0, 0.0);
    let k = s.follow(&test_frame(test_world(40.0)), &c, Camera::default());
    assert!(
        k.x == 40.0 && k.scale == 10.0 && k.center == 160.0,
        "erstes Bild: {k:?}"
    );
}

/// Totzone: Ein kleiner Schritt bewegt die Kamera nicht, ein großer holt followRate des Abstands zum Rand auf.
#[test]
fn follow_dead_zone() {
    let mut s = Cameras::default();
    let mut f = test_frame(test_world(40.0));
    let c = test_cell(0, 40.0);
    let prev = s.follow(&f, &c, Camera::default());
    set_monarch_x(&mut f, 40.0 + DEAD_ZONE / 2.0);
    let k = s.follow(&f, &c, prev);
    assert!(k.x == 40.0, "in der Totzone bewegt: {}", k.x);
    set_monarch_x(&mut f, 50.0);
    let want = 40.0 + (10.0 - DEAD_ZONE) * FOLLOW_RATE;
    let k = s.follow(&f, &c, prev);
    assert!(near(k.x, want), "außerhalb der Totzone {}, erwartet {want}", k.x);
}

/// Die Kamera holt den stehenden Monarchen bis auf die Totzone ein und schwingt nicht über.
#[test]
fn follow_converges() {
    let mut s = Cameras::default();
    let mut f = test_frame(test_world(40.0));
    let c = test_cell(0, 40.0);
    let mut k = s.follow(&f, &c, Camera::default());
    set_monarch_x(&mut f, 60.0);
    for _ in 0..200 {
        k = s.follow(&f, &c, k);
        assert!(k.x <= 60.0, "übergeschwungen: {}", k.x);
    }
    assert!((60.0 - k.x).abs() <= DEAD_ZONE + 1e-6, "nicht eingeholt: {}", k.x);
}

/// Grenzen der Stufe: Am Rand bleibt die Kamera so stehen, dass die Zelle nicht über die Stufe hinausschaut.
#[test]
fn follow_bounds() {
    let cases: [(&str, f64, f64); 3] = [("links", 2.0, 16.0), ("rechts", 99.0, 84.0), ("Mitte", 50.0, 50.0)];
    for (name, monarch, want) in cases {
        let mut s = Cameras::default();
        let k = s.follow(&test_frame(test_world(monarch)), &test_cell(0, 0.0), Camera::default());
        assert!(k.x == want, "{name}: {}, erwartet {want}", k.x);
    }
    let x = clamp_x(3.0, 16.0, 20.0);
    assert!(x == 10.0, "schmale Stufe: {x}, erwartet Mitte 10");
}

/// Ohne Monarchen folgt die Kamera der Burg; ohne Welt bleibt sie stehen (cameraFollow.ts: Ziel verschwindet).
#[test]
fn follow_fallbacks() {
    let mut s = Cameras::default();
    let mut f = test_frame(test_world(10.0));
    f.seats[0].monarch = 7;
    let k = s.follow(&f, &test_cell(0, 0.0), Camera::default());
    assert!(k.x == 50.0, "ohne Monarchen {}, erwartet Burg 50", k.x);
    let prev = Camera {
        x: 33.0,
        scale: 10.0,
        ..Camera::default()
    };
    let k = s.follow(&Frame::default(), &test_cell(0, 0.0), prev);
    assert!(k.x == 33.0, "ohne Welt {}, erwartet 33", k.x);
}

/// Zwei Zellen folgen ihren eigenen Monarchen unabhängig, auch auf verschiedenen Stufen.
#[test]
fn follow_two_cells() {
    let mut s = Cameras::default();
    let a = test_world(30.0);
    let mut b = test_world(70.0);
    b.players[0].index = 1;
    let f = Frame {
        stages: vec![
            Stage {
                index: 0,
                level: "forest".into(),
                world: Some(Rc::new(a)),
                ..Stage::default()
            },
            Stage {
                index: 1,
                depth: 1,
                level: "cave".into(),
                world: Some(Rc::new(b)),
            },
        ],
        seats: vec![
            Seat {
                player: 0,
                monarch: 0,
                stage: 0,
                ..Seat::default()
            },
            Seat {
                player: 1,
                monarch: 1,
                stage: 1,
                ..Seat::default()
            },
        ],
        ..Frame::default()
    };
    let k0 = s.follow(&f, &test_cell(0, 0.0), Camera::default());
    let k1 = s.follow(&f, &test_cell(1, 0.0), Camera::default());
    assert!(
        k0.x == 30.0 && k1.x == 70.0,
        "Zellen {} / {}, erwartet 30 / 70",
        k0.x,
        k1.x
    );
}

/// Stufenwechsel: Die Kamera springt ans neue Ziel statt hinzugleiten, der Übergang beginnt zur Zeit des Wechsels.
#[test]
fn follow_stage_switch() {
    let mut s = Cameras::default();
    let mut f = test_frame(test_world(20.0));
    let c = test_cell(0, 0.0);
    let mut k = s.follow(&f, &c, Camera::default());
    f.clock = 10.0;
    let a = s.fade(0, f.clock);
    assert!(a == 0.0, "Übergang ohne Wechsel: {a}");
    let mut other = test_world(80.0);
    other.seed = "b".into();
    f.stages = vec![Stage {
        index: 1,
        depth: 1,
        level: "cave".into(),
        world: Some(Rc::new(other)),
    }];
    f.seats[0].stage = 1;
    k = s.follow(&f, &c, k);
    assert!(k.x == 80.0, "nach dem Wechsel {}, erwartet Sprung auf 80", k.x);
    let a = s.fade(0, 10.0);
    assert!(a == 1.0, "Übergang am Anfang {a}, erwartet 1");
    let a = s.fade(0, 10.0 + FADE_SECONDS / 2.0);
    assert!(near(a, 0.5), "Übergang in der Mitte {a}");
    let a = s.fade(0, 10.0 + FADE_SECONDS + 0.01);
    assert!(a == 0.0, "Übergang am Ende {a}");
}
