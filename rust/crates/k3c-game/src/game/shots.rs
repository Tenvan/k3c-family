//! Port von `native/k3c-game/game/shots.go`: der Aufnahme-Modus (`--shots <ordner>`, B-792/AC-05, Vorbild
//! `native/k3c-ebiten/shots.go`). Die Szene läuft in `Game::new` bis `Scene::tick` vor (mit `--menu` liegt danach das
//! Menü oben), dann schreibt der `Shooter` `count` Bilder im Abstand von `every` Takten (mit `--at` genau nach den
//! genannten Takten) als `shot-01.png` … und zusätzlich verkleinert nebeneinander als `strip.png`, und beendet das
//! Programm. Die Zeit folgt dem Takt: `update` rechnet die Takte bis zum nächsten Bild und wartet, bis `store` es
//! geschrieben hat; dieselben Schalter ergeben dieselben Bilder.
//!
//! Abweichung von Go: Die Plattform kann keine Pixel auslesen. `capture` (Ende von `Game::draw`) meldet darum nur, dass
//! das Bild fertig gezeichnet ist; die Pixel liefert der Software-Rasterer (`shots_canvas`), das Schreiben macht
//! `store` (aufgerufen von `shots_run::record`). Den Streifen schreibt `update` wie in Go.

use std::path::{Path, PathBuf};

use k3c_platform::{Canvas, InputState, log};

use super::shots_image::{shrink, strip};
use super::{Args, Game, Mode};
use crate::still::Image;
use crate::still::pngio::write_png;

/// Höchste Breite eines Bilds im Streifen in px.
const THUMB_WIDTH: usize = 320;
/// Abstand der Bilder im Streifen in px.
const STRIP_GAP: usize = 4;
const STRIP_NAME: &str = "strip.png";

/// Go: `shooter`.
pub struct Shooter {
    dir: PathBuf,
    count: i64,
    every: i64,
    /// --at: Takte der Bilder; gesetzt, ersetzt `every`
    at: Vec<i64>,
    /// --flow: auch ohne laufendes Spiel weiter aufnehmen
    flow: bool,
    /// ein Bild ist fällig, `Game::draw` zeichnet es, `store` schreibt es
    pending: bool,
    /// `capture` hat das fällige Bild fertig gezeichnet gemeldet
    drawn: bool,
    taken: i64,
    thumbs: Vec<Image>,
    err: Option<String>,
    /// Streifen geschrieben, Programm beendet
    finished: bool,
}

impl Shooter {
    /// Legt den Ordner an und räumt Bilder eines früheren Laufs weg (Go: `newShooter`).
    pub fn new(a: &Args) -> Result<Shooter, String> {
        let dir = PathBuf::from(&a.shots);
        let fail = |p: &Path, e: &dyn std::fmt::Display| format!("{}: {e}", p.display());
        std::fs::create_dir_all(&dir).map_err(|e| fail(&dir, &e))?;
        let old = std::fs::read_dir(&dir).map_err(|e| fail(&dir, &e))?;
        let mut gone: Vec<PathBuf> = old
            .filter_map(Result::ok)
            .filter(|e| {
                let n = e.file_name().to_string_lossy().into_owned();
                n.starts_with("shot-") && n.ends_with(".png")
            })
            .map(|e| e.path())
            .collect();
        gone.push(dir.join(STRIP_NAME));
        for p in gone {
            match std::fs::remove_file(&p) {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(fail(&p, &e)),
                _ => {}
            }
        }
        Ok(Shooter {
            dir,
            count: a.count,
            every: a.every,
            at: a.at.clone(),
            flow: a.flow,
            pending: true,
            drawn: false,
            taken: 0,
            thumbs: Vec::new(),
            err: None,
            finished: false,
        })
    }

    /// Die Takte bis zum nächsten Bild (nach dem ersten: `taken` ≥ 1).
    fn gap(&self) -> i64 {
        let taken = usize::try_from(self.taken).unwrap_or(0);
        if self.at.len() > taken && taken > 0 {
            return self.at[taken] - self.at[taken - 1];
        }
        self.every
    }

    /// Rechnet die Takte bis zum nächsten Bild; nach dem letzten Bild schreibt es den Streifen und beendet.
    pub fn update(&mut self, g: &mut Game, raw: &InputState) {
        if self.err.is_some() || self.pending {
            return;
        }
        if self.taken == self.count {
            self.finish(g);
            return;
        }
        for _ in 0..self.gap() {
            g.advance(raw);
            if g.mode == Mode::Quit {
                return; // die Schleife endet; `record` meldet das vorzeitige Ende
            }
        }
        if g.args.scene.menu.is_empty() && !self.flow && !g.sim.active() {
            self.err = Some("💥 Aufnahme: Spiel nicht mehr aktiv (Absturz oder Spielende)".to_string());
            return;
        }
        self.pending = true;
    }

    /// Ende von `Game::draw`: das fällige Bild ist gezeichnet (Go: `capture` las hier die Pixel).
    pub fn capture(&mut self, _c: &mut dyn Canvas) {
        if self.pending && self.err.is_none() {
            self.drawn = true;
        }
    }

    /// Schreibt das fällige Bild (die Pixel des Rasterers) als `shot-NN.png` und merkt seine Vorschau.
    pub fn store(&mut self, img: Image, tick: u64) {
        if !self.drawn || self.err.is_some() {
            return;
        }
        self.drawn = false;
        let path = self.dir.join(format!("shot-{:02}.png", self.taken + 1));
        if let Err(e) = write_png(&path, &img) {
            self.err = Some(e);
            return;
        }
        log::info(
            "💾 Bild geschrieben",
            &[("datei", &path.display().to_string()), ("takt", &tick)],
        );
        self.thumbs
            .push(shrink(&img, (img.width as usize).div_ceil(THUMB_WIDTH)));
        self.taken += 1;
        self.pending = false;
    }

    /// Schreibt den Streifen, beendet das Spiel und das Programm.
    fn finish(&mut self, g: &mut Game) {
        let path = self.dir.join(STRIP_NAME);
        if let Err(e) = write_png(&path, &strip(&self.thumbs, STRIP_GAP)) {
            self.err = Some(e);
            return;
        }
        g.close_sim();
        log::info(
            "✅ Aufnahme fertig",
            &[("bilder", &self.taken), ("streifen", &path.display().to_string())],
        );
        g.mode = Mode::Quit;
        self.finished = true;
    }

    /// Der Fehler, der die Aufnahme beendet hat.
    pub fn error(&self) -> Option<&str> {
        self.err.as_deref()
    }

    /// Meldet, ob der Streifen geschrieben und das Programm beendet ist.
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Zahl der geschriebenen Bilder.
    pub fn taken(&self) -> i64 {
        self.taken
    }

    /// Zahl der geforderten Bilder.
    pub fn count(&self) -> i64 {
        self.count
    }
}

#[cfg(test)]
#[path = "shots_tests.rs"]
mod shots_tests;
