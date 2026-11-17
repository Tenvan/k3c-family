use super::*;
use k3c_sim::state::{Coin, Player};

fn world(x: f64, time: f64) -> World {
    let mut w = World {
        time,
        ..World::default()
    };
    w.players.push(Player {
        id: 7,
        x,
        ..Player::default()
    });
    w.coins.push(Coin {
        id: 9,
        x: x + 1.0,
        ..Coin::default()
    });
    w
}

/// Zwei Takte: vorher x = 10, nachher x = 12; die Kamera läuft von 0 auf 4.
fn zwei_takte(nachher: f64) -> (Blend, World) {
    let mut b = Blend::new(0);
    b.capture(&world(10.0, 1.0), &[0.0]);
    (b, world(nachher, 1.0 + 1.0 / 30.0))
}

#[test]
fn alpha_null_halb_eins() {
    let (mut b, mut w) = zwei_takte(12.0);
    let mut cams = [4.0];
    for (a, x, cam) in [(0.0, 10.0, 0.0), (0.5, 11.0, 2.0)] {
        b.apply(&mut w, &mut cams, a);
        assert_eq!(w.players[0].x, x, "alpha {a}");
        assert_eq!(w.coins[0].x, x + 1.0, "Münze, alpha {a}");
        assert_eq!(cams[0], cam, "Kamera, alpha {a}");
        b.restore(&mut w, &mut cams);
        assert_eq!((w.players[0].x, cams[0]), (12.0, 4.0), "restore nach alpha {a}");
    }
    b.apply(&mut w, &mut cams, 1.0);
    assert_eq!((w.players[0].x, cams[0]), (12.0, 4.0), "alpha 1 ändert nichts");
}

#[test]
fn uhr_gleitet_und_kommt_zurueck() {
    let (mut b, mut w) = zwei_takte(12.0);
    let end = w.time;
    b.apply(&mut w, &mut [0.0], 0.5);
    assert!((w.time - (1.0 + 0.5 / 30.0)).abs() < 1e-12);
    b.restore(&mut w, &mut [0.0]);
    assert_eq!(w.time, end);
}

#[test]
fn sprung_ueber_jump_units_gleitet_nicht() {
    let (mut b, mut w) = zwei_takte(10.0 + JUMP_UNITS + 0.5);
    b.apply(&mut w, &mut [0.0], 0.0);
    assert_eq!(w.players[0].x, 10.0 + JUMP_UNITS + 0.5);
    assert_eq!(
        glide(0.0, JUMP_UNITS, 0.5),
        JUMP_UNITS / 2.0,
        "genau jumpUnits gleitet noch"
    );
}

#[test]
fn unbekannte_einheit_bleibt() {
    let mut b = Blend::new(0);
    b.capture(&World::default(), &[]);
    let mut w = world(50.0, 2.0);
    b.apply(&mut w, &mut [], 0.0);
    assert_eq!(w.players[0].x, 50.0);
}
