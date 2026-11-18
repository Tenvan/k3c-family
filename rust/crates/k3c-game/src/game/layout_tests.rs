//! Port von `native/k3c-game/game/layout_test.go`.

use std::cell::Cell as Counter;
use std::rc::Rc;

use k3c_sim::state::{Castle, Player, World};

use super::*;
use crate::view::frame::{Seat, Stage};

struct Case {
    w: i64,
    h: i64,
    n: usize,
    rects: Vec<Rectangle>,
    world: i64,
    river: i64,
}

fn case(w: i64, h: i64, n: usize, rects: Vec<Rectangle>, world: i64, river: i64) -> Case {
    Case {
        w,
        h,
        n,
        rects,
        world,
        river,
    }
}

/// Lage der Zellen für 1 und 2 Spieler bei 1920×1080 und 1280×720: Welt zwei Drittel, Fluss ein Drittel je Zelle.
#[test]
fn cell_layout() {
    let cases = [
        case(1920, 1080, 1, vec![rect(0, 0, 1920, 1080)], 720, 360),
        case(
            1920,
            1080,
            2,
            vec![rect(0, 0, 1920, 540), rect(0, 540, 1920, 1080)],
            360,
            180,
        ),
        case(1280, 720, 1, vec![rect(0, 0, 1280, 720)], 480, 240),
        case(
            1280,
            720,
            2,
            vec![rect(0, 0, 1280, 360), rect(0, 360, 1280, 720)],
            240,
            120,
        ),
    ];
    for tc in &cases {
        let win = rect(0, 0, tc.w, tc.h);
        for (i, want) in tc.rects.iter().enumerate() {
            let r = cell_rect(win, tc.n, i);
            assert_eq!(r, *want, "{}x{}, {} Spieler, Zelle {i}", tc.w, tc.h, tc.n);
            let c = Cell {
                rect: r,
                ..Cell::default()
            };
            assert_eq!(c.world_rect().dy(), tc.world, "Welt, Zelle {i}");
            assert_eq!(c.river_rect().dy(), tc.river, "Fluss, Zelle {i}");
            #[allow(clippy::cast_precision_loss)]
            let want_scale = tc.world as f64 / VIEW_UNITS;
            assert!((scale(&c) - want_scale).abs() < 1e-9, "Maßstab {}", scale(&c));
        }
    }
}

/// Ungerade Höhe: die Zellen schließen lückenlos an.
#[test]
fn cell_layout_odd_height() {
    let win = rect(10, 5, 110, 106);
    let (a, b) = (cell_rect(win, 2, 0), cell_rect(win, 2, 1));
    assert!(
        a.min == win.min && b.max == win.max && a.max.y == b.min.y,
        "Zellen {a:?} {b:?} im Fenster {win:?}"
    );
}

fn player(index: i64, x: f64) -> Player {
    Player {
        index,
        x,
        ..Player::default()
    }
}

fn cell_frame() -> Frame {
    let w = World {
        castle: Castle {
            x: 50.0,
            ..Castle::default()
        },
        players: vec![player(0, 10.0), player(1, 90.0)],
        ..World::default()
    };
    Frame {
        stages: vec![Stage {
            index: 0,
            world: Some(Rc::new(w)),
            ..Stage::default()
        }],
        seats: vec![
            Seat {
                player: 0,
                monarch: 0,
                ..Seat::default()
            },
            Seat {
                player: 1,
                monarch: 1,
                ..Seat::default()
            },
        ],
        ..Frame::default()
    }
}

/// Baut die Zellen und setzt die Kameras wie ein Update.
fn cells_follow(dst: &mut Vec<Cell>, win: Rectangle, f: &Frame, k: &mut Cameras) {
    cells(dst, win, f);
    k.follow(dst, f);
}

fn set_player_x(f: &mut Frame, i: usize, x: f64) {
    if let Some(w) = f.stages[0].world.as_mut().and_then(Rc::get_mut) {
        w.players[i].x = x;
    }
}

/// Ohne Regel folgt die Kamera dem Monarchen, außer der Aufnahme-Modus setzt sie fest; ohne Monarchen steht sie an der
/// Burg.
#[test]
fn cells_camera() {
    let mut f = cell_frame();
    let win = rect(0, 0, 1280, 720);
    let mut k = Cameras::default();
    let mut cs = Vec::with_capacity(MAX_PLAYERS);
    cells_follow(&mut cs, win, &f, &mut k);
    assert!(
        cs.len() == 2 && cs[0].camera.x == 10.0 && cs[1].camera.x == 90.0 && cs[1].player == 1,
        "{cs:?}"
    );
    assert_eq!(cs[1].rect, rect(0, 360, 1280, 720));
    assert!(
        (cs[1].camera.scale - 240.0 / VIEW_UNITS).abs() < 1e-9,
        "Maßstab {}",
        cs[1].camera.scale
    );

    k.fixed = vec![f64::NAN, 200.0];
    cells_follow(&mut cs, win, &f, &mut k);
    assert_eq!((cs[0].camera.x, cs[1].camera.x), (10.0, 200.0), "feste Kamera");

    k.fixed.clear();
    f.seats[0].monarch = 7;
    cells_follow(&mut cs, win, &f, &mut k);
    assert_eq!(cs[0].camera.x, 50.0, "ohne Monarchen: Burg");

    f.seats.truncate(1);
    cells_follow(&mut cs, win, &f, &mut k);
    assert!(cs.len() == 1 && cs[0].rect == win, "ein Spieler: {cs:?}");

    // Kapazität MAX_PLAYERS: Cells und Follow legen nichts an (Gegenstück zu AllocsPerRun)
    let (cap, ptr) = (cs.capacity(), cs.as_ptr());
    for _ in 0..10 {
        cells_follow(&mut cs, win, &f, &mut k);
    }
    assert!(cs.capacity() == cap && cs.as_ptr() == ptr, "Cells legt neu an");
    assert_eq!(cap, MAX_PLAYERS);
}

/// Mehr Plätze als `MAX_PLAYERS` ergeben höchstens `MAX_PLAYERS` Zellen.
#[test]
fn cells_capped() {
    let mut f = cell_frame();
    f.seats.push(Seat {
        player: 2,
        ..Seat::default()
    });
    let mut cs = Vec::new();
    cells(&mut cs, rect(0, 0, 100, 100), &f);
    assert_eq!(cs.len(), MAX_PLAYERS);
}

/// Regel mit Zustand: rückt je Update höchstens 5 Units an den Monarchen heran, ohne Vorgänger springt sie; sie zählt
/// ihre Aufrufe.
struct LagRule {
    calls: Rc<Counter<usize>>,
}

impl CameraRule for LagRule {
    fn follow(&mut self, f: &Frame, c: &Cell, prev: Camera) -> Camera {
        self.calls.set(self.calls.get() + 1);
        let mut k = c.camera;
        k.x = match f.monarch(c.player) {
            None => f64::NAN,
            Some(m) if prev.scale == 0.0 => m.x,
            Some(m) => prev.x + (m.x - prev.x).clamp(-5.0, 5.0),
        };
        k
    }
}

/// Die Regel bekommt je Spieler die Kamera des vorigen Updates, ein NaN-Ergebnis gilt nicht, `--cam` setzt danach
/// fest; geht ein Spieler, vergisst er seine Kamera.
#[test]
fn cameras_rule_state() {
    let mut f = cell_frame();
    let win = rect(0, 0, 1280, 720);
    let calls = Rc::new(Counter::new(0));
    let mut k = Cameras {
        rule: Some(Box::new(LagRule {
            calls: Rc::clone(&calls),
        })),
        ..Cameras::default()
    };
    let mut cs = Vec::new();
    cells_follow(&mut cs, win, &f, &mut k);
    assert!(
        cs[0].camera.x == 10.0 && cs[1].camera.x == 90.0 && calls.get() == 2,
        "erstes Update springt"
    );

    set_player_x(&mut f, 0, 30.0);
    cells_follow(&mut cs, win, &f, &mut k);
    assert_eq!(cs[0].camera.x, 15.0, "Totzone mit Zustand");

    f.seats[0].monarch = 7;
    cells_follow(&mut cs, win, &f, &mut k);
    assert_eq!(cs[0].camera.x, 15.0, "NaN der Regel: alte Kamera");

    k.fixed = vec![-3.0];
    cells_follow(&mut cs, win, &f, &mut k);
    assert_eq!((cs[0].camera.x, cs[1].camera.x), (-3.0, 90.0), "--cam");

    k.fixed.clear();
    f.seats[0].monarch = 0;
    let all = f.seats.clone();
    f.seats.remove(0); // Spieler 0 geht, Spieler 1 steht allein in Zelle 0
    cells_follow(&mut cs, win, &f, &mut k);
    f.seats = all;
    cells_follow(&mut cs, win, &f, &mut k);
    assert_eq!(cs[0].camera.x, 30.0, "zurückgekehrter Spieler springt");
}

/// `place` legt die Zellen bei neuer Fenstergröße neu und behält das Welt-x; ohne Änderung bleibt alles.
#[test]
fn place_keeps_camera_x() {
    let f = cell_frame();
    let mut cs = Vec::new();
    cells_follow(&mut cs, rect(0, 0, 1280, 720), &f, &mut Cameras::default());
    let before = cs[1];
    place(&mut cs, rect(0, 0, 1280, 720));
    assert_eq!(cs[1], before, "ohne Änderung verändert");
    place(&mut cs, rect(0, 0, 1920, 1080));
    assert!(
        cs[1].rect == rect(0, 540, 1920, 1080) && cs[1].camera.x == 90.0 && cs[1].camera.center == 960.0,
        "neue Größe: {:?}",
        cs[1]
    );
}
