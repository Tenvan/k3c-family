//! Port von `native/k3c-game/props/` (Paketkopf `props.go`): Zeichnen von Bauten, Natur und Objekten aus dem
//! Spielzustand (Sprint NG4, B-797): Burg mit Wachstumsreihe, Bauplätze mit Stufe, Schaden und Ruine, Hintergrundmauer,
//! Baustellen mit Gerüst, Bauschild, Preis-Anzeige und Wartegrund, Bäume, Felsen, Büsche, Adern mit Abbau-Schild,
//! Plantage, Portal, Ausgang, Hafen, Lager, Truhen, Sterne, Münzen, wie der Browser; Fackeln zeichnet world (Licht).
//! Alles liegt auf `Layer::BUILDINGS`, Schilder, Balken und Texte wie im Browser hinter den Figuren.
//!
//! Aufbau: `listen` rechnet je Update den abgeleiteten Zustand (`Infos`), `plan*` baut je Zelle die Zeichenliste (rein,
//! getestet), `paint` zeichnet sie; `buildings`, `sites` und `nature` sind die Auswahl-Regeln.
//!
//! Abweichungen von Go: Der Drawer hat nur `&self` (Trait `view::Drawer`); Planer, Zeichner und Zustand liegen in einer
//! `RefCell`. Die Bibliothek ist `Rc<RefCell<Library>>` wie bei `actors` (Schrift und Zeichnen brauchen sie
//! veränderlich). Die Sim-Sicht (`SimView`) bekommt der Drawer im Konstruktor; `new_pending` nimmt `LiveSimView`
//! bis RU15 und RU18 die Sim liefern.

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::Canvas;
use k3c_sim::state::World;

use self::listen::Infos;
use self::paint::Painter;
use self::pending::LiveSimView;
use self::plan::{Planner, Sheets};
use self::simview::SimView;
use crate::art::Library;
use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::view::layer::{Drawer, Layer, Listener};
use crate::view::roles::AssetSource;

pub mod assets;
pub mod buildings;
pub mod coin_fountain;
pub mod data;
pub mod decor;
pub mod decor_glow;
pub mod listen;
pub mod nature;
pub mod paint;
pub mod pending;
pub mod plan;
pub mod plan_castle;
pub mod plan_combat;
pub mod plan_cost;
pub mod plan_entities;
pub mod plan_hermit;
pub mod plan_objects;
pub mod plan_sites;
pub mod plan_sites_cargo;
pub mod plan_sites_gate;
pub mod plan_sites_marks;
pub mod simview;
pub mod sites;

#[cfg(test)]
mod assets_tests;
#[cfg(test)]
mod buildings_tests;
#[cfg(test)]
mod coin_fountain_tests;
#[cfg(test)]
mod decor_tests;
#[cfg(test)]
mod felling_tests;
#[cfg(test)]
mod listen_tests;
#[cfg(test)]
mod mod_tests;
#[cfg(test)]
mod nature_tests;
#[cfg(test)]
mod plan_chest_tests;
#[cfg(test)]
mod plan_combat_tests;
#[cfg(test)]
mod plan_cost_tests;
#[cfg(test)]
mod plan_hermit_tests;
#[cfg(test)]
mod plan_objects_stage_tests;
#[cfg(test)]
mod plan_sites_cargo_tests;
#[cfg(test)]
mod plan_sites_gate_tests;
#[cfg(test)]
mod plan_sites_tests;
#[cfg(test)]
mod plan_tests;
#[cfg(test)]
mod simview_tests;
#[cfg(test)]
mod sites_tests;
#[cfg(test)]
mod sizes_sites_tests;
#[cfg(test)]
mod testutil;
#[cfg(test)]
mod walls_tests;

/// `State` ist der Zustand des Drawers über die Bilder: Planer (Puffer bleiben erhalten), Zeichner und Stufen-Zustand.
struct State {
    plan: Planner,
    paint: Painter,
    infos: Infos,
}

/// `Props` ist der Beitrag des Pakets: Drawer (`Layer::BUILDINGS`), Listener und AssetSource (Go: `drawer`).
pub struct Props {
    lib: Option<Rc<RefCell<Library>>>,
    view: Box<dyn SimView>,
    state: RefCell<State>,
}

/// `new` liefert den Drawer der Bauten; `lib` (Grafik) darf `None` sein, dann zeichnen die Platzhalter-Formen.
pub fn new(lib: Option<Rc<RefCell<Library>>>, view: Box<dyn SimView>) -> Props {
    let sheets = lib.clone().map(|l| l as Rc<dyn Sheets>);
    Props {
        lib,
        view,
        state: RefCell::new(State {
            plan: Planner::new(sheets),
            paint: Painter::default(),
            infos: Infos::default(),
        }),
    }
}

/// `new_pending` wie `new` mit `LiveSimView` (die Sim-Funktionen fehlen bis RU15 und RU18).
pub fn new_pending(lib: Option<Rc<RefCell<Library>>>) -> Props {
    new(lib, Box::new(LiveSimView))
}

impl Drawer for Props {
    fn layer(&self) -> Layer {
        Layer::BUILDINGS
    }

    /// `draw` zeichnet die Bauten der Welt, in der der Spieler der Zelle steht.
    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let Some(seat) = f.seat(c.player) else {
            return;
        };
        let Some(stage) = f.stage(seat.stage) else {
            return;
        };
        let Some(w) = stage.world.as_deref() else {
            return;
        };
        let mut st = self.state.borrow_mut();
        let State { plan, paint, infos } = &mut *st;
        plan.viewer = viewer_of(w, seat.monarch);
        plan.plan(Some(w), infos.info(stage.index), &c.camera, f.clock, self.view.as_ref());
        let mut guard = self.lib.as_ref().map(|l| l.borrow_mut());
        paint.paint(dst, &plan.body, guard.as_deref_mut(), c, f.clock);
        paint.paint(dst, &plan.top, guard.as_deref_mut(), c, f.clock);
    }
}

/// `viewer_of` ist der Index des Monarchen `monarch` in `w.players` (`None`: keiner).
fn viewer_of(w: &World, monarch: i64) -> Option<usize> {
    w.players.iter().position(|m| m.index == monarch)
}

impl Listener for Props {
    /// `listen` rechnet den abgeleiteten Zustand aller Stufen des Frames (einmal je Update).
    fn listen(&mut self, f: &Frame) {
        self.state.get_mut().infos.listen(f, self.view.as_ref());
    }
}

impl AssetSource for Props {
    /// `assets` nennt alle Papier-Assets, die props im Level zeichnen kann.
    fn assets(&self, level: &str, _w: &World) -> Vec<String> {
        assets::level_assets(level)
    }
}
