//! Port von `native/k3c-game/game/levels.go`: welche Grafik zu den Leveln der Stufen gehört und wann sie geladen wird.
//!
//! Abweichung von Go (Canvas nur im Zeichen-Takt): Go lädt in `Update`, weil `SetLevels` keine Zeichenfläche braucht. In
//! Rust laden Texturen nur über die `Canvas`, und `step` hat keine. Darum entscheidet der Takt hier nur, was geladen
//! werden soll (`load_levels` legt die Arbeit in `LoadState` ab), und der Anfang von `draw` führt sie aus
//! (`run_pending_load`, `loading.rs`). Die Level (`LoadState::levels`) gelten ab der Entscheidung, damit `same_levels`
//! im nächsten Takt nicht noch einmal anstößt.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use k3c_platform::Canvas;
use k3c_sim::state::World;

use super::Game;
use crate::art::{ArtError, LevelLoad, Levels, Library};
use crate::view::frame::Stage;
use crate::view::roles::AssetSource;

/// Schlüssel der Menü-Illustrationen ohne Level (Hauptmenü, B-818); kein Biom heißt so.
pub const MENU_LEVEL: &str = "_menu";

/// Go: `Loader` samt `stepLoader`: lädt die Grafik der Level (`Library::set_levels`): genau die Level in `want`,
/// Level → Assets `gruppe/id`. Nur der Zeichen-Takt ruft ihn auf (`Canvas`).
pub trait Loader {
    /// Stellt die Grafik in einem Zug auf `want`.
    fn set_levels(&mut self, want: &Levels, c: &mut dyn Canvas) -> Result<(), ArtError>;

    /// Go: `stepLoader.BeginLevels`: plant das Laden in Schritten; `None` = dieser Loader lädt nur in einem Zug.
    fn begin_levels(&self, _want: &Levels) -> Option<Result<LevelLoad, ArtError>> {
        None
    }

    /// Lädt bis zu `n` weitere Sheets eines mit `begin_levels` geplanten Ladens; `true` = fertig.
    fn step(&mut self, _ld: &mut LevelLoad, _n: usize, _c: &mut dyn Canvas) -> Result<bool, ArtError> {
        Ok(true)
    }

    /// Go: `interface{ Usable(string) bool }`: ob ein Asset vollständig ladbar ist (Weltbilder laden weich, B-937).
    fn usable(&self, _asset: &str) -> bool {
        false
    }

    /// Die Bibliothek für das Ladebild (Schrift, Figur); `None` = keine (dann zeichnet die Schleife die Meldung).
    fn library(&self) -> Option<Rc<RefCell<Library>>> {
        None
    }
}

/// Der Loader des Spiels: die geteilte Bibliothek (`Libs::shared`). Die Assets in `skip` hält die Welt in ihrer eigenen
/// Bibliothek (`world` liest nur, `Rc`); sie lädt dieser Loader nicht noch einmal. Die Menü-Illustrationen laden immer.
pub struct LibLoader {
    lib: Rc<RefCell<Library>>,
    skip: BTreeSet<String>,
}

impl LibLoader {
    pub fn new(lib: Rc<RefCell<Library>>, skip: BTreeSet<String>) -> LibLoader {
        LibLoader { lib, skip }
    }

    /// `want` ohne die Assets der Welt (Menü-Level unverändert).
    fn mine(&self, want: &Levels) -> Levels {
        want.iter()
            .map(|(lv, assets)| {
                let keep = if lv == MENU_LEVEL {
                    assets.clone()
                } else {
                    assets.iter().filter(|a| !self.skip.contains(*a)).cloned().collect()
                };
                (lv.clone(), keep)
            })
            .collect()
    }
}

impl Loader for LibLoader {
    fn set_levels(&mut self, want: &Levels, c: &mut dyn Canvas) -> Result<(), ArtError> {
        let want = self.mine(want);
        self.lib.borrow_mut().set_levels(&want, c)
    }

    fn begin_levels(&self, want: &Levels) -> Option<Result<LevelLoad, ArtError>> {
        Some(self.lib.borrow().begin_levels(&self.mine(want)))
    }

    fn step(&mut self, ld: &mut LevelLoad, n: usize, c: &mut dyn Canvas) -> Result<bool, ArtError> {
        ld.step(&mut self.lib.borrow_mut(), n, c)
    }

    fn usable(&self, asset: &str) -> bool {
        self.lib.borrow().usable(asset)
    }

    fn library(&self) -> Option<Rc<RefCell<Library>>> {
        Some(Rc::clone(&self.lib))
    }
}

/// Go: `SameLevels`: meldet, ob die Level der Stufen genau die Menge `have` (ohne Doppelte) sind; ohne Allokation.
pub fn same_levels(have: &[String], stages: &[Stage]) -> bool {
    let mut n = 0;
    for (i, st) in stages.iter().enumerate() {
        if !have.contains(&st.level) {
            return false;
        }
        if !stages[..i].iter().any(|o| o.level == st.level) {
            n += 1;
        }
    }
    n == have.len()
}

/// Go: `WantedAssets`: sammelt je Level der Stufen die Assets aller Quellen, je Level sortiert und ohne Doppelte; die
/// Welt eines Levels ist die seiner ersten Stufe.
pub fn wanted_assets(stages: &[Stage], sources: &[Box<dyn AssetSource>]) -> Levels {
    let mut want: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let none = World::default();
    for st in stages {
        if want.contains_key(&st.level) {
            continue;
        }
        let w = st.world.as_deref().unwrap_or(&none);
        let mut assets: Vec<String> = sources.iter().flat_map(|s| s.assets(&st.level, w)).collect();
        assets.sort();
        assets.dedup();
        want.insert(st.level.clone(), assets);
    }
    want
}

impl Game {
    /// Go: `loadLevels`: stellt die Grafik auf die Level in `frame.stages` ein, sobald sie sich ändern (hier: legt das
    /// Laden für den Anfang von `draw` ab). Ohne Stufen (Hauptmenü) nur die Illustrationen des Menüs.
    pub(super) fn load_levels(&mut self) {
        if self.parts.art.is_none() {
            return;
        }
        if self.frame.stages.is_empty() {
            self.load_menu_art();
            return;
        }
        if same_levels(&self.load.levels, &self.frame.stages) {
            return;
        }
        let want = wanted_assets(&self.frame.stages, &self.parts.assets);
        self.set_levels(&want);
        self.load.menu = false;
        self.load.set = Some(want);
    }

    /// Go: `setLevels`: merkt die Level (sortiert, `Levels` ist sortiert).
    pub(super) fn set_levels(&mut self, want: &Levels) {
        self.load.menu_art = false;
        self.load.levels.clear();
        self.load.levels.extend(want.keys().cloned());
    }

    /// Go: `loadMenuArt`: stellt die Grafik einmal auf die Menü-Illustrationen (gibt die Sheets der Level frei).
    fn load_menu_art(&mut self) {
        if self.load.menu_art {
            return;
        }
        self.load.menu_art = true;
        self.load.levels.clear();
        self.load.set = None;
        self.load.menu = true;
    }
}

#[cfg(test)]
#[path = "levels_tests.rs"]
pub mod levels_tests;
