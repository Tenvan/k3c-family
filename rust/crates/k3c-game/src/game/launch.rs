//! Port von `run` und `build` aus `native/k3c-game/main.go` (RU48.1): `k3c-game` ohne Unterbefehl und `k3c-game play` öffnen das
//! Fenster mit dem Hauptmenü. Die Sitzung (`local::Session`) ist die Simulation der Schleife, die Grafik lädt das
//! Programm beim ersten Bild (`LevelLoad` braucht die Zeichenfläche): die Welt (alle Biome) und die Rigs sofort, die
//! übrige Grafik je Stufe mit Ladeanzeige (`LibLoader`, RU30.1).
//!
//! Abweichungen von Go: Die Grafik der Welt aller Biome liegt gemeinsam im Speicher, weil `world` seine Bibliothek nur
//! liest (`Rc<Library>`); je Stufe lädt nur die geteilte Bibliothek (B-1109, Rest siehe Ticket). Ein Fehler vor dem Spiel und ein Panic im Takt stehen als Meldung im
//! Fenster (Go: `showFault`, `fault.go`); Enter oder Esc schließt es, `run` gibt die Meldung danach zurück.

use std::any::Any;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;

use k3c_data::{Source, load_all};
use k3c_platform::{App, Canvas, Config as WindowConfig, InputState, Key, Rect, log, run_window};
use k3c_sim::state::World;

use super::keytable::key_rows;
use super::{Args, Game, LibLoader, Libs, Loader, assemble, packages, parse_args};
use crate::art::{Config as ArtConfig, Levels, Library, default_config};
use crate::local::config::{Config as SessionConfig, default_saves_dir};
use crate::local::session::Session;
use crate::world;

/// Hinweis an jede Grafik-Meldung vor dem Spiel.
const HINT: &str = "Erst `task gen:paper` ausführen (rendert die Grafik nach public/grafik), dann neu starten.";
/// Schlüssel des einen Levels, unter dem alle Sheets liegen.
const LEVEL: &str = "spiel";

/// Liest die Schalter des Spiels (`parse_args` mit dem Standard-Ordner der Spielstände). Schalter, deren Teile erst RU30
/// liefert (Aufnahme, Messmodus, Eingabe-Folge, Dev-Schalter, Stresstest), lehnt es mit einer Meldung ab, statt im Spiel zu
/// scheitern.
pub fn parse(rest: &[String]) -> Result<Args, String> {
    const MISSING: [&str; 8] = ["shots", "measure", "input", "assets", "river", "stats", "flow", "stall"];
    for a in rest {
        let name = a.trim_start_matches('-').split('=').next().unwrap_or("");
        if a.starts_with('-') && MISSING.contains(&name) {
            return Err(format!(
                "Schalter --{name} ist im Rust-Spiel noch nicht verfügbar (RU30)"
            ));
        }
    }
    parse_args(rest, &default_saves_dir())
}

/// Startet das Spiel im Fenster und läuft bis zum Ende; die Meldung eines Fehlers vor dem Spiel oder eines Panics im
/// Takt kommt als `Err` zurück (nach dem Fenster).
pub fn run(args: &Args) -> Result<(), String> {
    let mut app = Launch::new(args);
    let size = (args.w > 0 && args.h > 0).then_some((args.w as u32, args.h as u32));
    let cfg = WindowConfig {
        title: "K3C".to_string(),
        size,
        ..WindowConfig::default()
    };
    run_window(&mut app, cfg)?;
    app.fault.take().map_or(Ok(()), Err)
}

/// Baut die Schleife aus Sitzung, Grafik und Schaltern; `libs` trägt die Grafik (`Libs::default()` ohne).
pub fn build_game(args: &Args, session: Session, libs: &Libs) -> Result<Game, String> {
    let mut parts = assemble(packages(libs, Some(Box::new(key_rows))))?;
    // Die Grafik der übrigen Module lädt die Schleife je Stufe (`LibLoader`, RU30.1, B-1109)
    parts.art = libs
        .shared
        .as_ref()
        .map(|l| Box::new(LibLoader::new(Rc::clone(l), libs.skip.clone())) as Box<dyn Loader>);
    log::info(
        "🚀 K3C natives Spiel",
        &[
            ("pakete", &parts.names.join(",")),
            ("drawer", &parts.drawers.len()),
            ("spielstaende", &args.saves),
        ],
    );
    Game::new(parts, Box::new(session), args.clone())
}

/// Öffnet die Sitzung mit dem Ordner der Spielstände aus den Schaltern.
pub fn open_session(args: &Args) -> Session {
    Session::open(SessionConfig {
        dir: args.saves.clone(),
        ..SessionConfig::default()
    })
}

/// Das Programm als `App`: baut das Spiel beim ersten Bild, fängt Panics und zeigt Fehler als Meldung.
pub struct Launch {
    args: Args,
    session: Option<Session>,
    game: Option<Game>,
    /// Meldung im Fenster; gesetzt: die Schleife zeigt nur sie (`run` gibt sie am Ende zurück)
    fault: Option<String>,
    quit: bool,
}

impl Launch {
    /// Das Programm mit offener Sitzung; Spiel und Grafik entstehen beim ersten Bild.
    pub fn new(args: &Args) -> Launch {
        Launch {
            args: args.clone(),
            session: Some(open_session(args)),
            game: None,
            fault: None,
            quit: false,
        }
    }

    /// Das Spiel, sobald es steht (nach dem ersten Bild).
    pub fn game(&self) -> Option<&Game> {
        self.game.as_ref()
    }

    /// Die Meldung, wenn Start oder Takt gescheitert sind.
    pub fn fault(&self) -> Option<&str> {
        self.fault.as_deref()
    }

    /// Hält die Meldung fest (Fenster, stderr, Rückgabe) und verwirft das Spiel.
    fn fail(&mut self, msg: String) {
        log::error("💥 Fehler im Spiel, Meldung im Fenster", &[("err", &msg)]);
        self.fault = Some(msg);
        self.game = None;
    }

    /// Baut das Spiel mit der Grafik auf `c`.
    fn build(&mut self, c: &mut dyn Canvas) -> Result<Game, String> {
        let session = self.session.take().ok_or("Sitzung fehlt")?;
        let libs = load_libs(&self.args, c)?;
        build_game(&self.args, session, &libs)
    }

    fn draw_fault(&self, c: &mut dyn Canvas) {
        let (w, h) = c.size();
        c.rect(Rect::new(0.0, 0.0, w, h), [0.12, 0.02, 0.02, 1.0]);
        c.layer(30);
        let msg = self.fault.as_deref().unwrap_or("");
        c.text("K3C: Fehler beim Start", 60.0, 60.0, 36.0, [1.0, 0.8, 0.3, 1.0]);
        for (i, line) in wrap(msg, 90).iter().enumerate() {
            c.text(line, 60.0, 120.0 + 34.0 * i as f32, 26.0, [1.0; 4]);
        }
        c.text("Enter oder Esc beendet", 60.0, h - 60.0, 24.0, [0.8; 4]);
    }
}

impl App for Launch {
    fn step(&mut self, input: &InputState, dt: f64) {
        if self.fault.is_some() {
            self.quit |= input.just_pressed(Key::Enter) || input.just_pressed(Key::Escape);
            return;
        }
        let Some(g) = self.game.as_mut() else {
            return;
        };
        if let Err(p) = catch_unwind(AssertUnwindSafe(|| g.step(input, dt))) {
            self.fail(format!("Panic im Takt: {}", panic_text(p.as_ref())));
        }
    }

    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        if self.fault.is_none() && self.game.is_none() {
            match catch_unwind(AssertUnwindSafe(|| self.build(c))) {
                Ok(Ok(g)) => self.game = Some(g),
                Ok(Err(e)) => self.fail(e),
                Err(p) => self.fail(format!("Panic beim Start: {}", panic_text(p.as_ref()))),
            }
        }
        if let Some(g) = self.game.as_mut()
            && let Err(p) = catch_unwind(AssertUnwindSafe(|| g.draw(c, alpha)))
        {
            self.fail(format!("Panic beim Zeichnen: {}", panic_text(p.as_ref())));
        }
        if self.fault.is_some() {
            self.draw_fault(c);
        }
    }

    fn done(&self) -> bool {
        self.quit || self.game.as_ref().is_some_and(Game::done)
    }

    fn fullscreen(&self) -> Option<bool> {
        self.game.as_ref().and_then(Game::fullscreen)
    }
}

/// Text eines Panics (`&str` oder `String`).
fn panic_text(p: &(dyn Any + Send)) -> String {
    p.downcast_ref::<&str>()
        .map(|s| (*s).to_string())
        .or_else(|| p.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "unbekannt".to_string())
}

/// Bricht `s` in Zeilen von höchstens `n` Zeichen (an Leerzeichen).
fn wrap(s: &str, n: usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for para in s.lines() {
        let mut cur = String::new();
        for word in para.split_whitespace() {
            if !cur.is_empty() && cur.chars().count() + 1 + word.chars().count() > n {
                out.push(std::mem::take(&mut cur));
            }
            if !cur.is_empty() {
                cur.push(' ');
            }
            cur.push_str(word);
        }
        out.push(cur);
    }
    out
}

/// Grafik-Pfade: Standard im Repo, `--grafik` ersetzt den Grafik-Ordner.
fn art_config(args: &Args) -> ArtConfig {
    let mut cfg = default_config();
    if !args.grafik.is_empty() {
        cfg.grafik_dir = args.grafik.clone().into();
    }
    cfg
}

/// Die Assets der Welt in allen Biomen (eigene Bibliothek, `Rc<Library>`, liegt wegen des Nur-Lesens von `world`
/// gemeinsam im Speicher; die übrigen Module laden je Stufe, `LibLoader`).
fn world_assets() -> Result<Vec<String>, String> {
    let data = load_all(&Source::from_env()).map_err(|e| format!("Daten: {e}"))?;
    let world_src = world::new(None).asset_source.ok_or("world ohne Asset-Quelle")?;
    let mut out = BTreeSet::new();
    for b in &data.biomes {
        let w = World {
            biome: b.clone(),
            ..World::default()
        };
        out.extend(world_src.assets(&b.id, &w));
    }
    Ok(out.into_iter().collect())
}

fn levels(assets: Vec<String>) -> Levels {
    Levels::from([(LEVEL.to_string(), assets)])
}

/// Lädt die Grafik auf `c`: Welt-Bibliothek ohne Rigs, geteilte Bibliothek mit Rigs und den Illustrationen des Menüs
/// (fehlende Menü-Bilder lässt das Menü weg, `usable`).
fn load_libs(args: &Args, c: &mut dyn Canvas) -> Result<Libs, String> {
    let cfg = art_config(args);
    let fail = |what: &str, e: &dyn std::fmt::Display| format!("{what}: {e}\n{HINT}");
    let for_world = world_assets()?;
    let skip: BTreeSet<String> = for_world.iter().cloned().collect();
    let mut wlib = Library::new(ArtConfig {
        rig_dir: std::path::PathBuf::new(),
        ..cfg.clone()
    })
    .map_err(|e| fail("Grafik", &e))?;
    wlib.set_levels(&levels(for_world), c).map_err(|e| fail("Grafik", &e))?;
    let mut slib = Library::new(cfg).map_err(|e| fail("Grafik", &e))?;
    slib.load_rigs(c).map_err(|e| fail("Rigs", &e))?;
    Ok(Libs {
        world: Some(Rc::new(wlib)),
        shared: Some(Rc::new(RefCell::new(slib))),
        skip,
    })
}

#[cfg(test)]
#[path = "launch_tests.rs"]
mod launch_tests;
