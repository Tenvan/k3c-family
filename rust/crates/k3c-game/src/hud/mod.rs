//! Port von `native/k3c-game/hud/` (RU20): HUD je Spielerhälfte (Sprint NG6, B-799), wie der Browser (HudScene.ts, hud/,
//! hints/, kampfAnzeige.ts): Spielerkarte mit Leben und Gold, Uhr, Kampf, Vorrat, Hub und Bürger, Ort, Meldungen,
//! Boss-Leiste und Nacht-Events, Radar, Reise, Beitritts-Hinweis, Skill-Leiste, Aktions-Hinweise und die geführte erste
//! Nacht über der Welt, Berufe und Händler als Labels, dazu das Skill-Menü je Spieler.
//!
//! Aufbau: `state.rs` schreibt je Frame den Zustand fort (Listener), `layout.rs`, `values.rs`, `banner.rs`, `radar.rs`,
//! `hints.rs` und `glyphs.rs` rechnen rein und sind getestet, `draw*.rs` und `paint.rs` zeichnen (RU20.5 und später).
//! Draw ändert keinen Zustand außer gemessenen Texten und dem Layout-Cache.

use std::cell::{Ref, RefCell};
use std::rc::Rc;

use k3c_platform::Canvas;

use self::draw::LayoutCache;
use self::glyphs::JoinKey;
use self::hints::GuideSeen;
use self::paint::Painter;
use self::pending::{LabelSpot, TaleState, VoyageView};
use self::state::SeatState;
use self::values::Memo;
use crate::art::{Font, Library, SeaMap};
use crate::props::pending::PendingSimView;
use crate::props::simview::SimView;
use crate::view::MAX_PLAYERS;
use crate::view::cell::Cell;
use crate::view::frame::{Cue, Frame};
use crate::view::layer::{Drawer, Layer, Listener};
use crate::view::roles::AssetSource;
use crate::view::settings::DEFAULT_LANGUAGE;

pub mod banner;
pub mod data;
pub mod draw;
pub mod drawglyph;
pub mod drawstock;
pub mod fonts;
pub mod glyphs;
pub mod hints;
pub mod icons;
pub mod keyline;
pub mod layout;
pub mod paint;
pub mod pending;
pub mod purse;
pub mod radar;
pub mod state;
mod state_labels;
pub mod texts;
mod texts_de;
mod texts_en;
pub mod values;
mod values_texts;

#[cfg(test)]
mod banner_fit_tests;
#[cfg(test)]
mod banner_tests;
#[cfg(test)]
mod draw_tests;
#[cfg(test)]
mod glyphs_tests;
#[cfg(test)]
mod hints_tests;
#[cfg(test)]
mod icons_tests;
#[cfg(test)]
mod keyicons_tests;
#[cfg(test)]
mod labels_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod profile_tests;
#[cfg(test)]
mod purse_tests;
#[cfg(test)]
mod radar_tests;
#[cfg(test)]
mod state_tests;
#[cfg(test)]
mod testutil;
#[cfg(test)]
mod texts_tests;
#[cfg(test)]
mod values_tests;

/// Hud ist der Beitrag des Pakets: Zustand je Spieler (Listener) und, hinter `HudDrawer`, der Drawer (`Layer::HUD`)
/// und das Overlay (Skill-Menü, RU23).
#[allow(dead_code)] // cue_buf, paint, lay, spots, voy, tale liest der Zeichner (RU20.5, RU23)
pub struct Hud {
    lib: Option<Rc<RefCell<Library>>>,
    view: Box<dyn SimView>,
    pub seats: [SeatState; MAX_PLAYERS],
    /// gesehene Hinweise der ersten Nacht: `Settings::guide_seen` und `pending`
    seen: GuideSeen,
    /// Anleitung an (nicht `Settings::guide_off`)
    guide: bool,
    /// neu gesehen, noch nicht in `Settings::guide_seen` (Overlay.Update speichert sie)
    pending: GuideSeen,
    lang: String,
    cells: usize,
    join_s: String,
    m_join: Memo<JoinKey>,
    cue_buf: Vec<Cue>,
    paint: Painter,
    lay: [LayoutCache; MAX_PLAYERS],
    /// Puffer der sichtbaren Welt-Labels in draw
    spots: Vec<LabelSpot>,
    /// Assets der Seekarte (None: Karte aus Formen)
    sea_map: Option<SeaMap>,
    /// gemeldete Assets (None: nur die Icons)
    assets: Option<Vec<String>>,
    /// Tasten-Icons gemeldet (`use_key_icons`, B-911)
    key_icons: bool,
    /// Schiffsfahrt im Vollbild (voyage.go, B-977)
    voy: VoyageView,
    /// Geschichtszeilen an den Wendepunkten (story.go, B-974)
    tale: TaleState,
}

impl Hud {
    /// Go: `New` (ohne den Drawer): lib (Grafik) darf `None` sein, dann zeichnet es nur Flächen ohne Text.
    pub fn new(lib: Option<Rc<RefCell<Library>>>, view: Box<dyn SimView>) -> Hud {
        let mut h = Hud {
            lib: lib.clone(),
            view,
            seats: Default::default(),
            seen: GuideSeen::default(),
            guide: false,
            pending: GuideSeen::default(),
            lang: DEFAULT_LANGUAGE.to_string(),
            cells: 0,
            join_s: String::new(),
            m_join: Memo::new(),
            cue_buf: Vec::new(),
            paint: Painter::default(),
            lay: Default::default(),
            spots: Vec::new(),
            sea_map: None,
            assets: None,
            key_icons: false,
            voy: VoyageView::default(),
            tale: TaleState::default(),
        };
        if let Some(l) = lib {
            // ohne Grafik bleiben die Tasten gemalt und die Seekarte aus Formen
            let usable = |a: &str| l.borrow().usable(a);
            h.use_key_icons(&usable);
            h.use_sea_map(SeaMap::from_data(), &usable);
        }
        h
    }

    /// font liefert die Schrift (None ohne Grafik-Bibliothek).
    pub fn font(&self) -> Option<Ref<'_, Font>> {
        self.lib.as_ref().map(|l| Ref::map(l.borrow(), |l| &l.font))
    }

    /// seat liefert den Zustand des Spielers p (None, wenn er keinen hat).
    pub fn seat(&self, p: i64) -> Option<&SeatState> {
        let st = self.seats.get(usize::try_from(p).ok()?)?;
        st.active.then_some(st)
    }
}

/// HudDrawer ist der Drawer des HUD (`Layer::HUD`): `Hud` liegt hinter `RefCell`, weil `Drawer::draw` nur `&self` hat.
pub struct HudDrawer {
    hud: RefCell<Hud>,
}

/// new liefert den Drawer des HUD; lib (Grafik) darf `None` sein, dann zeichnet es nur Flächen ohne Text.
pub fn new(lib: Option<Rc<RefCell<Library>>>, view: Box<dyn SimView>) -> HudDrawer {
    HudDrawer {
        hud: RefCell::new(Hud::new(lib, view)),
    }
}

/// new_pending: wie `new`, mit der Sicht auf Sim-Werte, die RU15 und RU18 liefern.
pub fn new_pending(lib: Option<Rc<RefCell<Library>>>) -> HudDrawer {
    new(lib, Box::new(PendingSimView))
}

impl HudDrawer {
    /// hud gibt den Zustand zum Lesen frei (Tests, Overlay).
    pub fn hud(&self) -> &RefCell<Hud> {
        &self.hud
    }
}

impl Drawer for HudDrawer {
    fn layer(&self) -> Layer {
        Layer::HUD
    }

    /// Zeichnet die Anzeigen des Spielers der Zelle c (`Hud::draw_cell`, draw.rs).
    fn draw(&self, dst: &mut dyn Canvas, _f: &Frame, c: &Cell) {
        self.hud.borrow_mut().draw_cell(dst, c);
    }
}
impl Listener for HudDrawer {
    fn listen(&mut self, f: &Frame) {
        self.hud.get_mut().listen(f);
    }
}

impl AssetSource for HudDrawer {
    fn assets(&self, level: &str, w: &k3c_sim::state::World) -> Vec<String> {
        AssetSource::assets(&*self.hud.borrow(), level, w)
    }
}
