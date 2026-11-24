//! Port von `native/k3c-game/devtools/devtools.go`.
//!
//! Entwickler-Anzeigen des nativen Spiels (B-888): Tastenübersicht (F1) aus der Tabelle `view::devkeys::DEV_KEYS`,
//! Cheat-Menü je Spieler (F9, `view::roles::Overlay`) und Debug-Overlay (F10). Den Zustand von F1 und F10 setzt game im
//! Frame (`Frame::dev`); das Cheat-Menü führt sein Overlay selbst. Gezeichnet wird alles auf `Layer::OVERLAY` über allen
//! Zellen (je Zelle in deren Rechteck aus `Frame::cells`). Nichts davon erscheint im Aufnahme-Modus.
//!
//! Abweichung von Go: Der Typ `overlay` erfüllt in Go `Drawer` und `Overlay` zugleich. Rust trennt die Rollen
//! (`Drawer::draw(&self)`, `Overlay::update(&mut self)`); beide Hüllen halten denselben Zustand in
//! `Rc<RefCell<DevOverlay>>` (Update und Draw laufen nacheinander im selben Thread, nie gleichzeitig).

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::{Canvas, Rect};

use self::cheatmenu::{CheatState, Toggles};
use self::help::draw_help;
use crate::art::Library;
use crate::view::MAX_PLAYERS;
use crate::view::cell::Cell;
use crate::view::devkeys::{DEBUG_OFF, DevSession};
use crate::view::frame::{Cue, Frame};
use crate::view::input::Input;
use crate::view::layer::{Drawer, Layer};
use crate::view::roles::{Overlay, OverlayStep};
use crate::view::session::Session;

pub mod cheatlist;
pub mod cheatmenu;
pub mod cheats;
pub mod chronicle;
pub mod chronicle_stage;
pub mod debuggeo;
pub mod debugstands;
pub mod drawcheats;
mod drawdebug;
pub mod help;
pub mod report;
pub mod values;

#[cfg(test)]
mod cheats_tests;
#[cfg(test)]
mod chronicle_tests;
#[cfg(test)]
mod debuggeo_tests;
#[cfg(test)]
mod help_tests;
#[cfg(test)]
mod values_tests;

/// Farbe als RGBA (Go: `color.RGBA`, vormultipliziert).
pub type Rgba = [u8; 4];

// Farben der Entwickler-Anzeigen (vormultipliziert).
pub const PANEL_COLOR: Rgba = [10, 12, 18, 220];
pub const TEXT_COLOR: Rgba = [240, 236, 220, 255];
pub const KEY_COLOR: Rgba = [250, 200, 90, 255];
pub const DIM_COLOR: Rgba = [150, 150, 150, 255];
pub const BAD_COLOR: Rgba = [240, 110, 90, 255];
pub const SEL_COLOR: Rgba = [70, 60, 20, 230];

/// DevOverlay ist der gemeinsame Zustand des Pakets: Cheat-Menüs, Schalter und Puffer.
pub struct DevOverlay {
    pub(crate) lib: Option<Rc<RefCell<Library>>>,
    pub(crate) menus: [CheatState; MAX_PLAYERS],
    pub(crate) toggles: Toggles,
    pub(crate) cues: Vec<Cue>,
    /// Puffer der Debug-Formen (`drawdebug`)
    pub(crate) shapes: Vec<debuggeo::Shape>,
    /// Puffer der Werte-Zeilen (`values`)
    pub(crate) lines: Vec<String>,
}

/// DevParts sind die Beiträge des Pakets: die Drawer (`Layer::OVERLAY`) und das Overlay (Cheat-Menü).
pub struct DevParts {
    pub drawers: Vec<Box<dyn Drawer>>,
    pub overlay: Box<dyn Overlay>,
}

/// New liefert die Beiträge des Pakets; `lib` darf `None` sein (Tests), dann ohne Schriftdatei.
pub fn new(lib: Option<Rc<RefCell<Library>>>) -> DevParts {
    let shared = Rc::new(RefCell::new(DevOverlay {
        lib,
        menus: Default::default(),
        toggles: Toggles::new(),
        cues: Vec::new(),
        shapes: Vec::new(),
        lines: Vec::new(),
    }));
    DevParts {
        drawers: vec![Box::new(DevDrawer(Rc::clone(&shared)))],
        overlay: Box::new(DevOverlayRole(shared)),
    }
}

/// DevDrawer ist die Zeichen-Rolle des Pakets (`Layer::OVERLAY`).
pub struct DevDrawer(pub Rc<RefCell<DevOverlay>>);

/// DevOverlayRole ist die Overlay-Rolle des Pakets (Cheat-Menü je Spieler).
pub struct DevOverlayRole(pub Rc<RefCell<DevOverlay>>);

impl Drawer for DevDrawer {
    fn layer(&self) -> Layer {
        Layer::OVERLAY
    }

    /// Zeichnet je Zelle das Debug-Overlay (F10) und das Cheat-Menü, darüber die Tastenübersicht (F1).
    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        if f.recording {
            return;
        }
        let lib = self.0.borrow().lib.clone();
        let mut guard = lib.as_ref().map(|l| l.borrow_mut());
        let mut font = guard.as_mut().map(|g| &mut g.font);
        let o = &mut *self.0.borrow_mut();
        for cell in &f.cells {
            if f.dev.debug != DEBUG_OFF {
                o.draw_debug(dst, font.as_deref_mut(), f, cell);
            }
            let open = usize::try_from(cell.player)
                .ok()
                .and_then(|p| o.menus.get(p))
                .filter(|m| m.open);
            if let Some(m) = open {
                o.draw_cheats(dst, font.as_deref_mut(), cell, m);
            }
        }
        if f.dev.help {
            draw_help(dst, font, c);
        }
    }
}

/// Go: Typ-Zusicherung `s.(view.DevSession)`; `None`, wenn die Session keine Entwickler-Rolle hat (das Cheat-Menü meldet
/// dann „nur im lokalen Spiel“).
pub fn dev_of(s: &mut dyn Session) -> Option<&mut dyn DevSession> {
    s.dev_role()
}

impl Overlay for DevOverlayRole {
    fn update(&mut self, input: &Input, f: &Frame, s: &mut dyn Session) -> OverlayStep {
        let mut dev = dev_of(s);
        self.0.borrow_mut().update_with(input, f, &mut dev)
    }
}

/// straight rechnet eine vormultiplizierte Farbe (Go) in die gerade Farbe der Canvas um.
pub fn straight(c: Rgba) -> [f32; 4] {
    if c[3] == 0 {
        return [0.0; 4];
    }
    let a = f32::from(c[3]);
    let ch = |v: u8| (f32::from(v) / a).min(1.0);
    [ch(c[0]), ch(c[1]), ch(c[2]), a / 255.0]
}

/// fill zeichnet eine Fläche.
#[allow(clippy::cast_possible_truncation)]
pub fn fill(dst: &mut dyn Canvas, x: f64, y: f64, w: f64, h: f64, clr: Rgba) {
    dst.rect(Rect::new(x as f32, y as f32, w as f32, h as f32), straight(clr));
}

/// boxed zeichnet eine dunkle Fläche (Go: `box`).
pub fn boxed(dst: &mut dyn Canvas, x: f64, y: f64, w: f64, h: f64) {
    fill(dst, x, y, w, h, PANEL_COLOR);
}
