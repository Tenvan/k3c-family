//! Port von `native/k3c-game/game/loading.go`: die Ladeanzeige beim Stufenwechsel (B-810). Ändern sich die Level der
//! Frames, lädt die Schleife die Grafik. Damit das Fenster nicht ohne Anzeige einfriert, setzt ein Takt zuerst „lädt“,
//! `draw` zeigt dann die Anzeige, und erst danach beginnt das Laden. Im Aufnahme- und Messmodus lädt die Schleife in
//! einem Zug (das Bild hängt nur vom Takt ab); `--stall` hält dort die Anzeige dauerhaft, um sie aufzunehmen.
//!
//! Aufteilung (Canvas nur im Zeichen-Takt): `sync_levels` (im Takt) entscheidet und plant, `run_pending_load` (Anfang
//! von `draw`) lädt hoch: im Fenster `LOAD_BATCH` Sheets je Bild, sonst alles vor dem ersten Bild.

use std::mem::take;

use k3c_platform::{Canvas, log};

use super::levels::{MENU_LEVEL, wanted_assets};
use super::loadscreen::LoadScreen;
use super::notice::Notice;
use super::{Game, Mode};
use crate::art::{LevelLoad, Levels};
use crate::view::frame::Stage;

/// Zahl der Sheets, die ein Bild beim schrittweisen Laden hochlädt (B-949): dazwischen zeichnet `draw` den Fortschritt.
pub const LOAD_BATCH: usize = 4;

/// Was die Schleife über das Laden der Grafik merkt (Go: `levels`, `menuArt`, `lload`, `stepWant`, `shown`, `loadScr`,
/// `loadMsg`) und was der nächste `draw` noch hochladen soll.
#[derive(Default)]
pub struct LoadState {
    /// geladene Level (sortiert)
    pub levels: Vec<String>,
    /// die Grafik steht auf den Menü-Illustrationen
    pub menu_art: bool,
    /// laufendes schrittweises Laden
    pub lload: Option<LevelLoad>,
    /// Level des laufenden Ladens
    pub step_want: Levels,
    /// `draw` hat die Ladeanzeige gezeichnet
    pub shown: bool,
    pub screen: LoadScreen,
    pub msg: Option<Notice>,
    /// `draw` stellt die Menü-Illustrationen ein
    pub menu: bool,
    /// `draw` lädt diese Level in einem Zug
    pub set: Option<Levels>,
    /// abgebrochene Ladevorgänge: `draw` gibt ihre Sheets frei
    pub dropped: Vec<LevelLoad>,
}

/// Go: `staleLoad`: meldet, ob die Level der Stufen nicht mehr die Level des laufenden Ladens `want` sind.
pub fn stale_load(want: &Levels, stages: &[Stage]) -> bool {
    let mut n = 0;
    for (i, st) in stages.iter().enumerate() {
        if !want.contains_key(&st.level) {
            return true;
        }
        if !stages[..i].iter().any(|o| o.level == st.level) {
            n += 1;
        }
    }
    n != want.len()
}

impl Game {
    /// Go: `deferLoad`: ob die Anzeige dem Laden vorangeht (nur im Fenster).
    pub(super) fn defer_load(&self) -> bool {
        self.args.shots.is_empty() && self.args.measure.seconds == 0.0
    }

    /// Go: `Loading`: die Schleife wartet auf das Laden der Grafik und zeigt die Ladeanzeige.
    pub fn loading(&self) -> bool {
        self.loading
    }

    /// Go: `syncLevels`: bringt die geladene Grafik auf den Stand der Stufen im Frame. Im Fenster: erster Takt setzt
    /// „lädt“, weiter warten, bis `draw` die Anzeige gezeichnet hat, dann planen (das Hochladen macht `draw`).
    pub(super) fn sync_levels(&mut self) {
        let has_art = self.parts.art.is_some();
        let stall = self.args.stall;
        if has_art && self.frame.stages.is_empty() && !stall {
            self.loading = false;
            self.abort_load();
            self.load_levels(); // nur die Menü-Illustrationen, klein und einmal (B-818): ohne Ladeanzeige
            return;
        }
        if !has_art || (!stall && super::levels::same_levels(&self.load.levels, &self.frame.stages)) {
            self.loading = false;
            self.abort_load();
            return;
        }
        let running = self.load.lload.is_some();
        if stall {
            self.start_loading();
        } else if !self.defer_load() {
            self.load_levels();
        } else if !self.loading {
            self.start_loading();
        } else if running && stale_load(&self.load.step_want, &self.frame.stages) {
            self.abort_load(); // die Stufen wechselten während des Ladens: von vorn mit den neuen Leveln
            self.begin_step_load();
        } else if !running && self.load.shown && !self.begin_step_load() {
            self.load_levels();
            self.loading = false;
        }
    }

    /// Go: `startLoading`: setzt „lädt“ und baut den Text der Anzeige (einmal je Wechsel).
    fn start_loading(&mut self) {
        if self.loading {
            return;
        }
        self.loading = true;
        self.load.shown = false;
        let names: Vec<String> = self.frame.stages.iter().map(|s| s.level.clone()).collect();
        let text = format!("Stufe: {}", names.join(", "));
        self.abort_load();
        self.load.screen.label.clear();
        self.load.msg = Some(Notice::new(&["Lädt …", "", &text]));
        log::info("⏳ Grafik wird geladen", &[("level", &names.join(","))]);
    }

    /// Go: `drawLoading`: zeichnet die Ladeanzeige und merkt, dass sie zu sehen war (nur dieses Flag ändert `draw`:
    /// ohne gezeichnete Anzeige beginnt das Laden nicht).
    pub(super) fn draw_loading(&mut self, c: &mut dyn Canvas) {
        let p = self.load.lload.as_ref().map_or(0.0, LevelLoad::progress);
        let lib = self.parts.art.as_ref().and_then(|a| a.library());
        let drawn = match lib {
            Some(l) => self.load.screen.draw(c, &mut l.borrow_mut(), p),
            None => false,
        };
        if !drawn && let Some(m) = &self.load.msg {
            m.draw(c);
        }
        self.load.shown = true;
    }

    /// Go: `abortLoad`: bricht ein laufendes schrittweises Laden ab; `draw` gibt dessen Sheets frei.
    fn abort_load(&mut self) {
        if let Some(ld) = self.load.lload.take() {
            self.load.dropped.push(ld);
        }
    }

    /// Go: `beginStepLoad`: plant das schrittweise Laden der Level aus den Stufen; `false`, wenn der Loader nur in einem
    /// Zug lädt.
    fn begin_step_load(&mut self) -> bool {
        let want = wanted_assets(&self.frame.stages, &self.parts.assets);
        let begun = self.parts.art.as_ref().and_then(|a| a.begin_levels(&want));
        match begun {
            None => false,
            Some(Err(e)) => {
                self.loading = false;
                self.fail(e);
                true
            }
            Some(Ok(ld)) => {
                self.load.step_want = want;
                self.load.lload = Some(ld);
                true
            }
        }
    }

    /// Anfang von `draw`: lädt hoch, was der Takt geplant hat (Go lud in `Update`): freigeben, Menü-Illustrationen,
    /// Level in einem Zug, den nächsten Schritt des schrittweisen Ladens.
    pub(super) fn run_pending_load(&mut self, c: &mut dyn Canvas) {
        for mut ld in take(&mut self.load.dropped) {
            ld.abort(c);
        }
        let Some(art) = self.parts.art.as_mut() else {
            return;
        };
        let mut failed = None;
        if take(&mut self.load.menu) {
            let names = {
                let a = &**art;
                crate::menu::art::art(Some(&|s: &str| a.usable(s)))
            };
            let want = Levels::from([(MENU_LEVEL.to_string(), names)]);
            if let Err(e) = art.set_levels(&want, c) {
                log::warn("🚫 Menü-Illustrationen fehlen, Karten nur mit Text", &[("err", &e)]);
            }
        }
        if let Some(want) = self.load.set.take()
            && let Err(e) = art.set_levels(&want, c)
        {
            failed = Some(e);
        }
        let mut done = false;
        if failed.is_none()
            && let Some(ld) = self.load.lload.as_mut()
        {
            match art.step(ld, LOAD_BATCH, c) {
                Ok(d) => done = d,
                Err(e) => failed = Some(e),
            }
        }
        if let Some(e) = failed {
            self.load.lload = None;
            self.load.levels.clear();
            self.loading = false;
            self.fail(e);
        } else if done {
            self.load.lload = None;
            self.loading = false;
            let want = take(&mut self.load.step_want);
            self.set_levels(&want);
        }
    }

    /// Fehler beim Laden der Grafik (Go: `fail`). Das Fehlerfenster liefert RU30.3; bis dahin Log und Ende.
    pub(super) fn fail(&mut self, err: impl std::fmt::Display) {
        log::error("💥 Grafik fehlt, Spiel endet", &[("err", &err.to_string())]);
        self.mode = Mode::Quit;
    }
}

#[cfg(test)]
#[path = "loading_tests.rs"]
mod loading_tests;
