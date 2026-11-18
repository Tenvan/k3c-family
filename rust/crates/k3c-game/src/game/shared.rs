//! Geteilter Zustand eines Fachmoduls hinter mehreren Rollen (Go: ein Zeiger erfüllt Drawer, Listener, AssetSource und
//! Overlay; Rust: ein Trait-Objekt je Rolle über denselben `Rc<RefCell<T>>`, Leitfaden § 4 gilt nur für die Sim-Crates).

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::Canvas;
use k3c_sim::state::World;

use crate::hud::HudDrawer;
use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::view::input::{DeviceID, Input, KeyboardSource, Source};
use crate::view::layer::{Drawer, Layer, Listener};
use crate::view::roles::{AssetSource, Overlay, OverlayStep};
use crate::view::screen::{MenuPages, Menus, Screen};
use crate::view::session::Session;
use crate::view::story::EndHolder;
use crate::view::voyage::{Voyage, VoyageDrawer};

/// Eine Rolle über dem geteilten Zustand `T`.
pub struct Shared<T>(pub Rc<RefCell<T>>);

impl<T> Shared<T> {
    /// Neue Rolle über demselben Zustand.
    pub fn share(&self) -> Shared<T> {
        Shared(Rc::clone(&self.0))
    }
}

impl<T: Drawer> Drawer for Shared<T> {
    fn layer(&self) -> Layer {
        self.0.borrow().layer()
    }

    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        self.0.borrow().draw(dst, f, c);
    }
}

impl<T: Listener> Listener for Shared<T> {
    fn listen(&mut self, f: &Frame) {
        self.0.borrow_mut().listen(f);
    }
}

impl<T: Source> Source for Shared<T> {
    fn poll(&mut self, raw: &k3c_platform::InputState) -> Input {
        self.0.borrow_mut().poll(raw)
    }
}

impl<T: KeyboardSource> KeyboardSource for Shared<T> {
    fn set_keyboard(&mut self, profile: &str) {
        self.0.borrow_mut().set_keyboard(profile);
    }

    fn set_menu(&mut self, open: bool) {
        self.0.borrow_mut().set_menu(open);
    }
}

impl<T: AssetSource> AssetSource for Shared<T> {
    fn assets(&self, level: &str, w: &World) -> Vec<String> {
        self.0.borrow().assets(level, w)
    }
}

/// Skill-Menü des `Hud` als Overlay.
pub struct HudOverlay(pub Shared<HudDrawer>);

impl Overlay for HudOverlay {
    fn update(&mut self, input: &crate::view::input::Input, f: &Frame, s: &mut dyn Session) -> OverlayStep {
        let d = self.0.0.borrow();
        d.hud().borrow_mut().update(input, f, s)
    }
}

/// Schiffsfahrt des `Hud` im Vollbild.
pub struct HudVoyage(pub Shared<HudDrawer>);

impl VoyageDrawer for HudVoyage {
    fn voyage_ready(&self, f: &Frame, v: &Voyage) -> bool {
        self.0.0.borrow().voyage_ready(f, v)
    }

    fn draw_voyage(&self, dst: &mut dyn Canvas, f: &Frame, v: &Voyage) -> bool {
        self.0.0.borrow().draw_voyage(dst, f, v)
    }
}

impl<T: Menus> Menus for Shared<T> {
    fn main(&mut self) -> Box<dyn Screen> {
        self.0.borrow_mut().main()
    }

    fn pause(&mut self) -> Box<dyn Screen> {
        self.0.borrow_mut().pause()
    }
}

impl<T: MenuPages> MenuPages for Shared<T> {
    fn page(&mut self, name: &str, in_game: bool, by: DeviceID) -> Option<Box<dyn Screen>> {
        self.0.borrow_mut().page(name, in_game, by)
    }
}

/// Geschichte des `Hud` zum Spielende (`EndHolder`).
pub struct HudEnd(pub Shared<HudDrawer>);

impl EndHolder for HudEnd {
    fn end_holding(&self) -> bool {
        let d = self.0.0.borrow();
        d.hud().borrow().end_holding()
    }
}
