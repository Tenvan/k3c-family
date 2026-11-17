//! Port von `native/k3c-game/actors/actors.go`.
//!
//! Zeichnet die Figuren aus dem Spielzustand (NG3, B-796): Monarchen auf ihrem Reittier, Truppen je Beruf, Gegner und
//! Bosse. Figuren mit Teile-Baum laufen als Rig (art::Actor, art::Painter), `requisite`-Figuren als Sheet. Zuordnung
//! Rolle → Asset steht in catalog.rs (RU13.4), Zustand → Animation als reine Funktionen in anim.rs.
//!
//! Ablauf: Listen (einmal je Frame) gleicht die Figuren über stabile IDs mit der Welt ab, wählt die Pose und schreibt
//! Animation und Umhang aller Rig-Figuren im Takt fort; Draw zeichnet nur (figures.rs, pose.rs, draw.rs; RU13.4/5).
//!
//! Abweichung von Go: Die Rig-Actors brauchen eine Zeichenfläche (Textur-Upload) und die Bibliothek veränderlich;
//! `lib` ist darum `Rc<RefCell<Library>>`, `check_rigs` nimmt die Zeichenfläche, und `ActorsDrawer` (draw.rs) meldet
//! den Zustand als Drawer und Listener an. Die Rig-Bilder lädt der Aufrufer vorher (`Library::load_rigs`).

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use k3c_data::Source;
use k3c_platform::Canvas;
use k3c_sim::state::World as SimWorld;

use crate::art::{Library, Painter};
use crate::view::roles::AssetSource;

pub mod anim;
pub mod catalog;
pub mod combatmarks;
pub mod draw;
pub mod figures;
pub mod figures_pose;
pub mod levels;
pub mod markdraw;
pub mod marks;
pub mod pose;

use catalog::{Catalog, load_catalog};
use combatmarks::MARK_ASSETS;
use figures::{Figure, Key, WarnKey};
use markdraw::Ink;

/// Actors ist Drawer (LayerActors), Listener und AssetSource der Figuren (Go: `drawer`). Die `DrawImageOptions` von
/// Go entfallen.
pub struct Actors {
    pub lib: Option<Rc<RefCell<Library>>>,
    pub cat: Option<Catalog>,
    pub painter: Option<Painter>,
    /// Vorlagen der Anzeigen, beim ersten Zeichnen angelegt (markdraw.rs)
    pub ink: Option<Ink>,
    pub figs: BTreeMap<Key, Figure>,
    pub generation: u64,
    /// Darstellungszeit in Sekunden (aus Frame.Clock, steht in der Pause)
    pub now: f64,
    /// Frame.Clock des letzten Listen
    pub last: f64,
    pub started: bool,
    pub warned: BTreeMap<WarnKey, bool>,
    /// Assets, die als Sheet gezeichnet werden (AssetSource)
    pub sheets: Vec<String>,
}

/// New liefert den Drawer der Figuren. lib darf `None` sein (Tests): Dann rechnet er die Posen, zeichnet aber nichts.
/// Fehlt der Katalog, steht das beim Start im Log; die Rigs prüft `check_rigs` (braucht die Zeichenfläche).
pub fn new(lib: Option<Rc<RefCell<Library>>>) -> Actors {
    let mut d = Actors {
        lib: lib.clone(),
        cat: None,
        painter: None,
        ink: None,
        figs: BTreeMap::new(),
        generation: 0,
        now: 0.0,
        last: 0.0,
        started: false,
        warned: BTreeMap::new(),
        sheets: Vec::new(),
    };
    let cat = match load_catalog(&Source::from_env()) {
        Ok(c) => c,
        Err(e) => {
            k3c_platform::log::error(
                "❌ Figuren-Katalog nicht lesbar, es erscheinen keine Figuren",
                &[("err", &e)],
            );
            return d;
        }
    };
    d.sheets = cat.sheet_assets();
    d.cat = Some(cat);
    let Some(lib) = lib else {
        return d;
    };
    match lib.borrow().new_painter() {
        Ok(p) => d.painter = Some(p),
        Err(e) => {
            k3c_platform::log::error("❌ Rig-Zeichner fehlt, es erscheinen keine Rig-Figuren", &[("err", &e)]);
            return d;
        }
    }
    d
}

impl Actors {
    /// checkRigs legt je Rig-Look einmal einen Actor an, damit eine fehlende Rig-Datei beim Start im Log steht und
    /// nicht erst, wenn die Figur auftaucht. In fester Reihenfolge (sortierte Arten, `BTreeMap`): Die Teil-Bilder
    /// landen so bei jedem Start an derselben Stelle im Atlas, sonst weicht das lineare Filtern um ±1 ab (Aufnahme
    /// nicht bildgleich).
    pub fn check_rigs(&mut self, canvas: &mut dyn Canvas) {
        let Some(cat) = self.cat.clone() else {
            return;
        };
        let mut n = 0;
        for m in [&cat.troops, &cat.enemies, &cat.bosses] {
            for l in m.values() {
                if !l.sheet {
                    let (_, broken) = self.new_actor(canvas, l, false);
                    if !broken {
                        n += 1;
                    }
                }
            }
        }
        let (_, broken) = self.new_actor(canvas, &cat.player(0), true);
        if !broken {
            n += 1;
        }
        k3c_platform::log::info("✅ Figuren geladen", &[("rigs", &n), ("sheets", &self.sheets.len())]);
    }
}

impl AssetSource for Actors {
    /// Assets nennt die Sheets der `requisite`-Figuren, die im Level vorkommen können (Biom und Tiefe der Welt,
    /// levels.rs).
    fn assets(&self, _level: &str, w: &SimWorld) -> Vec<String> {
        let Some(cat) = &self.cat else {
            return Vec::new();
        };
        let mut out = cat.level_sheets(w);
        // Banner, Krone und Schild: Papier-Bilder der Anzeigen (B-940)
        out.extend(MARK_ASSETS.iter().map(|s| (*s).to_string()));
        out.sort();
        out.dedup();
        out
    }
}
