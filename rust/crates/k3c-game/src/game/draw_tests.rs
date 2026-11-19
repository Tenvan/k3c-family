//! Tests der Zeichen-Reihenfolge im Fenster (B-1127): Mit der echten Plattform-`Frame` (sie sortiert nach Ebene,
//! Ausschnitt und Aufruf) liegt der oberste Screen nach `finish` hinter allen Zellen- und HUD-Aufrufen, also oben.

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::{Canvas, Frame as Batch, Rect, run_headless};

use super::testutil::{FakeSim, Log, parts};
use super::*;
use crate::hud::paint::Z_CONTENT;
use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::view::input::Input;
use crate::view::layer::{Drawer, Layer};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::Session;

const WORLD: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
const HUD: [f32; 4] = [0.4, 0.5, 0.6, 1.0];
const MENU: [f32; 4] = [0.7, 0.8, 0.9, 1.0];

/// Zellen-Drawer wie die Welt (Ebene 0, vom Aufrufer auf die Zelle beschnitten) bzw. wie das HUD (Ebene `Z_CONTENT`,
/// danach zurück auf 0).
struct Fill {
    layer: Layer,
    color: [f32; 4],
    z: Option<u32>,
}

impl Drawer for Fill {
    fn layer(&self) -> Layer {
        self.layer
    }

    fn draw(&self, dst: &mut dyn Canvas, _f: &Frame, c: &Cell) {
        if let Some(z) = self.z {
            dst.layer(z);
        }
        dst.rect(Rect::new(c.rect.min.x as f32, 0.0, 10.0, 10.0), self.color);
        if self.z.is_some() {
            dst.layer(0);
        }
    }
}

/// Der oberste Screen (Pause): ein Rechteck ohne Ausschnitt.
struct Cover;

impl Screen for Cover {
    fn update(&mut self, _input: &Input, _s: &mut dyn Session) -> Step {
        Step::default()
    }

    fn draw(&self, dst: &mut dyn Canvas, _f: &Frame, _c: &Cell) {
        dst.rect(Rect::new(0.0, 0.0, 1920.0, 1080.0), MENU);
    }
}

fn game_with_world_and_hud() -> Game {
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let drawers: Vec<Box<dyn Drawer>> = vec![
        Box::new(Fill {
            layer: Layer::GROUND,
            color: WORLD,
            z: None,
        }),
        Box::new(Fill {
            layer: Layer::HUD,
            color: HUD,
            z: Some(Z_CONTENT),
        }),
    ];
    let (sim, _probe) = FakeSim::new(2);
    let args = Args {
        scene: Scene {
            seed: "t".to_string(),
            players: 2,
            ..Scene::default()
        },
        ..Args::default()
    };
    let mut g =
        Game::new(parts(StepKind::Stay, &log, drawers), Box::new(sim), args).unwrap_or_else(|e| unreachable!("{e}"));
    run_headless(&mut g, 2, None);
    g
}

/// B-1127: Pause über zwei Zellen mit Welt und HUD: der letzte Aufruf nach der Sortierung der Plattform gehört dem
/// Screen. Ohne die Fenster-Ebene sortierte er (Ausschnitt 0) vor die Zellen und unter das HUD.
#[test]
fn screen_liegt_ueber_zellen_und_hud() {
    let mut g = game_with_world_and_hud();
    let mut batch = Batch::new();
    batch.begin();
    g.draw_game(&mut batch, 0.0);
    batch.finish();
    let colors: Vec<[f32; 4]> = batch.instances().map(|i| i.c).collect();
    assert!(
        colors.contains(&WORLD) && colors.contains(&HUD),
        "Welt und HUD gezeichnet: {colors:?}"
    );
    assert_eq!(colors.iter().filter(|c| **c == MENU).count(), 0, "kein Screen offen");

    g.stack.push(Box::new(Cover));
    g.mode = Mode::Menu;
    batch.begin();
    g.draw_game(&mut batch, 0.0);
    batch.finish();
    let colors: Vec<[f32; 4]> = batch.instances().map(|i| i.c).collect();
    let menu = colors.iter().position(|c| *c == MENU);
    let last_other = colors.iter().rposition(|c| *c == WORLD || *c == HUD);
    assert!(
        menu.is_some() && menu > last_other,
        "Screen nach Welt und HUD: {colors:?}"
    );
}
