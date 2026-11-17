//! Port von `native/k3c-game/actors/draw_test.go`; Go zeichnet in ein `ebiten.Image`, Rust gegen `NullCanvas`.

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use k3c_platform::NullCanvas;
use k3c_rig::rig::matrix::apply;
use k3c_sim::state::{Enemy, Player, Troop, World as SimWorld};

use super::*;
use crate::actors::catalog::UNIT_PX;
use crate::actors::new;
use crate::art::{Config, Library};
use crate::view::cell::{new_camera, rect};
use crate::view::frame::{Seat, Stage};

// Zellen wie im Fenster 1920 × 1080 mit zwei Spielern (game/layout.go: 420/32 Units Welt-Höhe).
const CELL_W: i64 = 1920;
const CELL_H: i64 = 540;
const VIEW_UNITS: f64 = 420.0 / 32.0;

/// libDrawer: Drawer mit den Rigs aus dem Repo (die Sheets der requisite-Figuren fehlen, sie entstehen erst mit
/// `task gen:paper`; Sheet-Figuren zeichnen dann nichts).
fn lib_drawer() -> (Actors, NullCanvas) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut lib = Library::new(Config {
        grafik_dir: root.join("public/grafik"),
        rig_dir: root.join("native/k3c-rig/assets"),
        ..Config::default()
    })
    .unwrap();
    let mut canvas = NullCanvas::default();
    lib.load_rigs(&mut canvas).unwrap();
    let d = new(Some(Rc::new(RefCell::new(lib))));
    assert!(d.painter.is_some(), "kein Rig-Zeichner");
    (d, canvas)
}

/// cells: zwei Zellen übereinander, beide Kameras bei Welt-x x.
fn cells(x: f64) -> [Cell; 2] {
    [0, 1].map(|i| {
        let r = rect(0, i * CELL_H, CELL_W, (i + 1) * CELL_H);
        let mut c = Cell {
            index: i,
            player: i,
            rect: r,
            ..Cell::default()
        };
        #[allow(clippy::cast_precision_loss)]
        let scale = c.world_rect().dy() as f64 / VIEW_UNITS;
        c.camera = new_camera(r, x, scale);
        c
    })
}

fn two_seats(w: &SimWorld) -> Frame {
    Frame {
        stages: vec![Stage {
            index: 0,
            level: "wald".to_string(),
            world: Some(Rc::new(w.clone())),
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

/// Die Rechnung des Zeichenpfads legt nach dem ersten Bild nichts mehr an: Go zählt Allokationen und lässt `advance`
/// (pose.go) laufen; beides gehört nicht hierher (RU13.5, Allokationen gibt es in Rust nicht zu zählen).
#[test]
#[ignore = "wartet auf RU13.5"]
fn pose_without_allocations() {}

/// Fußpunkt und Größe: Die Rig-Figur steht mit dem Rahmen auf der Boden-Linie und ist so hoch wie in data/paper.json.
#[test]
fn rig_foot_and_size() {
    let (mut d, mut canvas) = lib_drawer();
    let w = SimWorld {
        troops: vec![Troop {
            id: 1,
            kind: "warrior".to_string(),
            x: 10.0,
            ..Troop::default()
        }],
        ..SimWorld::default()
    };
    d.listen(&two_seats(&w));
    d.ensure_actors(&mut canvas);
    let k = Key {
        stage: 0,
        class: Class::Troop,
        id: 1,
    };
    let fg = &d.figs[&k];
    assert!(fg.actor.is_some(), "Rig ohne Actor");
    let (size, frame) = rig_size(fg);
    let c = &cells(10.0)[0];
    let m = rig_view(&c.camera, 10.0, c.screen_y(fg.lift), size, frame, false);
    let (top, foot) = (apply(&m, [0.0, frame[1]]), apply(&m, [0.0, 0.0]));
    #[allow(clippy::cast_precision_loss)]
    let ground = c.ground_y() as f64;
    assert!(foot[1] == ground, "Fußpunkt y {}, Boden {ground}", foot[1]);
    let want_px = 102.5 / UNIT_PX * c.camera.scale * (-frame[1] / frame[3]);
    assert!(
        ((foot[1] - top[1]) - want_px).abs() <= 0.01,
        "Höhe bis Rahmen-Oberkante {} px, erwartet {want_px}",
        foot[1] - top[1]
    );
}

/// Die Pose folgt nur dem Takt: Go vergleicht zwei Drawer nach `advance` (pose.go, RU13.5).
#[test]
#[ignore = "wartet auf RU13.5"]
fn pose_independent_of_draws() {}

/// Zusatz: `draw_figure` zeichnet eine Rig-Figur mit Pose in die Zelle und nichts, wenn sie außerhalb der Kamera
/// steht oder noch keine Pose hat; eine Sheet-Figur ohne geladenes Sheet zeichnet nichts, ist aber zeichenbar.
#[test]
fn draw_figure_rig_and_sheet() {
    let (mut d, mut canvas) = lib_drawer();
    let w = SimWorld {
        players: vec![Player {
            id: 1,
            index: 0,
            x: 10.0,
            ..Player::default()
        }],
        troops: vec![Troop {
            id: 2,
            kind: "warrior".to_string(),
            x: 10.0,
            ..Troop::default()
        }],
        enemies: vec![Enemy {
            id: 3,
            kind: "bat".to_string(),
            x: 10.0,
            ..Enemy::default()
        }],
        ..SimWorld::default()
    };
    d.listen(&two_seats(&w));
    d.ensure_actors(&mut canvas);
    let c = cells(10.0)[0];
    let key = |class, id| Key { stage: 0, class, id };
    let (kw, kb) = (key(Class::Troop, 2), key(Class::Enemy, 3));
    assert!(!d.draw_figure(&mut canvas, &c, kw, 10.0), "ohne Pose gezeichnet");
    assert!(d.draw_figure(&mut canvas, &c, kb, 10.0), "Sheet-Figur nicht zeichenbar");
    d.figs.get_mut(&kw).unwrap().posed = true; // die Pose rechnet pose.rs (RU13.5)
    let before = canvas.meshes + canvas.sprites;
    assert!(d.draw_figure(&mut canvas, &c, kw, 10.0), "Rig-Figur nicht zeichenbar");
    assert!(canvas.meshes + canvas.sprites > before, "Rig-Figur zeichnet nichts");
    let before = canvas.meshes + canvas.sprites;
    assert!(
        d.draw_figure(&mut canvas, &c, kw, 5000.0),
        "Figur außerhalb nicht zeichenbar"
    );
    assert_eq!(canvas.meshes + canvas.sprites, before, "Figur außerhalb gezeichnet");
    assert!(
        !d.draw_figure(&mut canvas, &c, key(Class::Troop, 99), 10.0),
        "unbekannte Figur"
    );
}
