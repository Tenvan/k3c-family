//! Port von `native/k3c-game/game/blend_test.go`. Der Frame ist selbst gebaut; `Stage::world` ist ein `Rc<World>`:
//! `unique_world_stays_in_place` läuft mit dem eindeutigen Besitzer (kein Klon), `shared_world_is_cloned_once` mit einer
//! zweiten Referenz (`Rc::make_mut` klont einmal, die zweite Referenz bleibt unberührt).

use std::rc::Rc;

use k3c_sim::state::{Enemy, Player};

use super::*;
use crate::view::cell::Camera;
use crate::view::frame::Stage;

fn blend_frame(x: f64, cam: f64) -> (Frame, Vec<Cell>) {
    let w = World {
        players: vec![Player {
            id: 1,
            x,
            ..Player::default()
        }],
        enemies: vec![Enemy {
            id: 7,
            x: x * 2.0,
            ..Enemy::default()
        }],
        ..World::default()
    };
    let cells = vec![Cell {
        index: 0,
        camera: Camera {
            x: cam,
            ..Camera::default()
        },
        ..Cell::default()
    }];
    let f = Frame {
        clock: x,
        stages: vec![Stage {
            index: 0,
            world: Some(Rc::new(w)),
            ..Stage::default()
        }],
        cells: cells.clone(),
        ..Frame::default()
    };
    (f, cells)
}

fn player_x(f: &Frame) -> f64 {
    f.stages[0].world.as_ref().map_or(f64::NAN, |w| w.players[0].x)
}

fn enemy_x(f: &Frame) -> f64 {
    f.stages[0].world.as_ref().map_or(f64::NAN, |w| w.enemies[0].x)
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

fn take_time(f: &mut Frame, b: &mut Blend) {
    f.clock = 10.0 + 1.0 / 30.0;
    b.clock = 10.0;
}

#[test]
fn blend_glides_and_restores() {
    let mut b = Blend::new(true);
    let (f, cells) = blend_frame(10.0, 5.0);
    b.capture(&f, &cells);
    let (mut f, mut cells) = blend_frame(10.2, 5.1);
    take_time(&mut f, &mut b);
    b.apply(&mut f, &mut cells, 0.5);
    assert!(near(player_x(&f), 10.1), "Spieler x = {}, erwartet 10.1", player_x(&f));
    assert!(
        near(cells[0].camera.x, 5.05),
        "Kamera x = {}, erwartet 5.05",
        cells[0].camera.x
    );
    assert!(near(f.clock, 10.0 + 0.5 / 30.0), "Uhr {}", f.clock);
    b.restore(&mut f, &mut cells);
    assert_eq!(player_x(&f), 10.2);
    assert_eq!(cells[0].camera.x, 5.1);
    assert_eq!(f.clock, 10.0 + 1.0 / 30.0);
}

#[test]
fn blend_jump_and_new_units() {
    let mut b = Blend::new(true);
    let (f, cells) = blend_frame(10.0, 5.0);
    b.capture(&f, &cells);
    let (mut f, mut cells) = blend_frame(40.0, 5.0); // weiter als JUMP_UNITS: springt
    if let Some(w) = f.stages[0].world.as_mut().and_then(Rc::get_mut) {
        w.enemies[0].id = 8; // neue Einheit: nicht im Schlüssel
    }
    b.apply(&mut f, &mut cells, 0.5);
    assert_eq!(player_x(&f), 40.0, "Sprung darf nicht gleiten");
    assert_eq!(enemy_x(&f), 80.0, "neue Einheit darf nicht gleiten");
    b.restore(&mut f, &mut cells);
}

#[test]
fn blend_off_and_full_alpha() {
    for on in [false, true] {
        let mut b = Blend::new(on);
        let (f, cells) = blend_frame(10.0, 5.0);
        b.capture(&f, &cells);
        let (mut f, mut cells) = blend_frame(10.2, 5.1);
        b.apply(&mut f, &mut cells, 1.0);
        assert_eq!(player_x(&f), 10.2, "alpha 1 oder aus (on = {on}): keine Verschiebung");
        b.restore(&mut f, &mut cells);
    }
    let mut b = Blend::new(false);
    let (f, cells) = blend_frame(10.0, 5.0);
    b.capture(&f, &cells);
    let (mut f, mut cells) = blend_frame(10.2, 5.1);
    b.apply(&mut f, &mut cells, 0.5);
    assert_eq!(player_x(&f), 10.2, "aus: keine Verschiebung bei alpha 0.5");
    assert!(!b.is_on());
}

/// Zwei Stufen: gleiche ID in verschiedenen Stufen hat verschiedene Schlüssel.
#[test]
fn blend_keys_per_stage() {
    let mut b = Blend::new(true);
    let (mut f, cells) = blend_frame(10.0, 5.0);
    let second = f.stages[0].clone();
    f.stages.push(Stage { index: 3, ..second });
    b.capture(&f, &cells);
    let (mut f, mut cells) = blend_frame(10.2, 5.1);
    let mut w2 = f.stages[0].world.as_deref().cloned().unwrap_or_default();
    w2.players[0].x = 12.0; // Stufe 3: vorher 10, jetzt 12
    f.stages.push(Stage {
        index: 3,
        world: Some(Rc::new(w2)),
        ..Stage::default()
    });
    b.apply(&mut f, &mut cells, 0.5);
    let x3 = f.stages[1].world.as_ref().map_or(f64::NAN, |w| w.players[0].x);
    assert!(near(player_x(&f), 10.1) && near(x3, 11.0), "{} {x3}", player_x(&f));
    b.restore(&mut f, &mut cells);
    let x3 = f.stages[1].world.as_ref().map_or(f64::NAN, |w| w.players[0].x);
    assert_eq!((player_x(&f), x3), (10.2, 12.0));
}

/// Eindeutiger Besitzer: dieselbe Welt (`Rc::ptr_eq`) wird verschoben und zurückgestellt, kein Klon.
#[test]
fn unique_world_stays_in_place() {
    let mut b = Blend::new(true);
    let (f, cells) = blend_frame(10.0, 5.0);
    b.capture(&f, &cells);
    let (mut f, mut cells) = blend_frame(10.2, 5.1);
    let before = f.stages[0].world.as_ref().map(Rc::as_ptr);
    b.apply(&mut f, &mut cells, 0.5);
    assert_eq!(f.stages[0].world.as_ref().map(Rc::as_ptr), before, "Welt geklont");
    b.restore(&mut f, &mut cells);
    assert_eq!(f.stages[0].world.as_ref().map(Rc::as_ptr), before);
}

/// Geteilte Welt (die Sitzung hält eine zweite Referenz): `Rc::make_mut` klont einmal, die Sitzung sieht die
/// Verschiebung nie.
#[test]
fn shared_world_is_cloned_once() {
    let mut b = Blend::new(true);
    let (f, cells) = blend_frame(10.0, 5.0);
    b.capture(&f, &cells);
    let (mut f, mut cells) = blend_frame(10.2, 5.1);
    let session = f.stages[0].world.clone();
    b.apply(&mut f, &mut cells, 0.5);
    assert!(near(player_x(&f), 10.1));
    let held = session.as_ref().map_or(f64::NAN, |w| w.players[0].x);
    assert_eq!(held, 10.2, "die Welt der Sitzung bleibt unverändert");
    b.restore(&mut f, &mut cells);
    assert_eq!(player_x(&f), 10.2);
    let after = f.stages[0].world.as_ref().map(Rc::strong_count);
    assert_eq!(after, Some(1), "der Frame besitzt seine Kopie allein");
}

/// Keine Allokation je Bild: die Puffer behalten Kapazität und Zeiger.
#[test]
fn blend_no_alloc() {
    let mut b = Blend::new(true);
    let (f, cells) = blend_frame(10.0, 5.0);
    b.capture(&f, &cells);
    let (mut f, mut cells) = blend_frame(10.2, 5.1);
    b.apply(&mut f, &mut cells, 0.5);
    b.restore(&mut f, &mut cells);
    let (cap_s, ptr_s) = (b.saved.capacity(), b.saved.as_ptr());
    let (cap_p, ptr_p) = (b.prev.capacity(), b.prev.as_ptr());
    let world = f.stages[0].world.as_ref().map(Rc::as_ptr);
    for _ in 0..50 {
        b.apply(&mut f, &mut cells, 0.5);
        b.restore(&mut f, &mut cells);
    }
    assert!(b.saved.capacity() == cap_s && b.saved.as_ptr() == ptr_s, "saved wächst");
    assert!(b.prev.capacity() == cap_p && b.prev.as_ptr() == ptr_p, "prev wächst");
    assert_eq!(
        f.stages[0].world.as_ref().map(Rc::as_ptr),
        world,
        "Welt je Bild geklont"
    );
}
