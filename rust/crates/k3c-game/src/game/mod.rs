//! Port von `native/k3c-game/game/`: die Spielschleife des nativen Spiels. RU26.1 legt Schalter, Modi, Menü-Stapel und
//! die Konstante `TPS` an, RU26.2 bis RU26.3 Zellen, Zusammenbau, Zwischenbilder, Geräte und Anzeigen; `game.go` (die
//! Nabe) liefert RU26.4: `Game` ist eine `k3c_platform::App` (`mod.rs`: Aufbau und Takt, `advance.rs`: ein Takt,
//! `draw.rs`: ein Bild). Die Zeitsammlung `pacing.go` entfällt: Die Plattform (`k3c_platform::Clock`, 30 Hz, höchstens
//! 4 Takte je Bild, `alpha`) ersetzt sie; `deps.go` (Go-Modul-Pins) entfällt.

mod advance;
pub mod assets;
pub mod blend;
pub mod clock;
pub mod devices;
pub mod devkeys;
pub mod devshot;
pub mod devvalues;
mod draw;
pub mod fault;
pub mod flags;
pub mod keytable;
pub mod launch;
pub mod layout;
pub mod levels;
pub mod loading;
pub mod loadscreen;
#[cfg(test)]
mod look_tests;
pub mod measure;
pub mod measure_frames;
pub mod measure_load;
mod menu;
pub mod modes;
pub mod notice;
pub mod overlay;
pub mod packages;
pub mod parts;
mod play;
#[cfg(test)]
pub mod play_testutil;
pub mod run;
pub mod scene;
pub mod script;
pub mod shared;
pub mod shots;
mod shots_canvas;
mod shots_image;
pub mod shots_run;
pub mod sim;
pub mod stack;
pub mod stress;
pub mod stressreport;
#[cfg(test)]
pub mod testutil;
pub mod voyage;
#[cfg(test)]
pub mod voyage_testutil;
pub mod window;

#[cfg(test)]
#[path = "draw_tests.rs"]
mod draw_tests;

#[cfg(test)]
#[path = "ending_tests.rs"]
mod ending_tests;

#[cfg(test)]
#[path = "imports_tests.rs"]
mod imports_tests;

#[cfg(test)]
#[path = "loop_tests.rs"]
mod loop_tests;

#[cfg(test)]
#[path = "pacing_tests.rs"]
mod pacing_tests;

#[cfg(test)]
#[path = "play_tests.rs"]
mod play_tests;

#[cfg(test)]
#[path = "story_tests.rs"]
mod story_tests;

#[cfg(test)]
#[path = "window_tests.rs"]
mod window_tests;

use k3c_platform::{App, Canvas, InputState, log};

use crate::view::MAX_PLAYERS;
use crate::view::cell::{Cell, Rectangle, rect};
use crate::view::devkeys::DevView;
use crate::view::frame::{Cue, Frame};
use crate::view::input::{DeviceInput, Input, Source};
use crate::view::screen::Screen;
use crate::view::session::NewGame;

pub use blend::Blend;
pub use devvalues::TpsMeter;
pub use fault::Fault;
pub use layout::{Cameras, VIEW_UNITS, cell_rect, cells, place, scale};
pub use levels::{LibLoader, Loader};
pub use measure::{Meter, Report};
pub use measure_load::MeasureLoad;
pub use modes::Measure;
pub use packages::{Libs, packages};
pub use parts::{Part, Parts, assemble};
pub use scene::{Args, Hold, Scene, USAGE, parse_args};
pub use script::parse_script;
pub use shots::Shooter;
pub use sim::Sim;
pub use stack::{Mode, Stack};
pub use stress::StarterLink;
pub use stressreport::{StressReport, StressRun};
pub use voyage::Voyage;

use devices::Devices;
use overlay::StatsOverlay;
use stress::StressEnv;

/// Takte der Simulation je Sekunde.
pub const TPS: i64 = 30;

/// Hintergrund hinter allen Zellen (Go: `clearColor`).
const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// Die Schleife: Eingabe, Menü-Stapel, Takt der Simulation, Kameras, Grafik der Level und Zeichnen der Zellen in
/// Ebenen-Reihenfolge. Alle Zeit kommt aus dem Takt (`Frame::clock`), nie aus der Wanduhr.
///
pub struct Game {
    parts: Parts,
    sim: Box<dyn Sim>,
    args: Args,
    /// feste Eingabe-Folge der Szene (`--input`), sonst die Quelle aus `parts`
    scripted: Option<Box<dyn Source>>,
    stack: Stack,
    mode: Mode,
    /// angewendeter Anzeige-Modus (`Settings::window`, `window.rs`)
    window: String,
    /// Takte im Spiel seit dem Spielende (`play.rs`)
    end_in: i64,
    /// Puffer der Eingabe ohne verbrauchte Geräte (`play.rs`)
    in_buf: Vec<DeviceInput>,
    frame: Frame,
    cues: Vec<Cue>,
    /// Puffer der Zellen zwischen zwei Takten (Kapazität `MAX_PLAYERS`); nach `advance` liegen sie in `frame.cells`
    cells: Vec<Cell>,
    /// Zeichenfläche (logische Fläche der Plattform, `draw` setzt sie aus `Canvas::size`)
    win: Rectangle,
    cams: Cameras,
    devs: Devices,
    dev: DevView,
    /// Eingabe des Takts (Go: `pressed`), Takte je Sekunde der Seite Werte, Fehler im Messmodus (beendet das Programm)
    raw_input: InputState,
    tps: TpsMeter,
    err: Option<String>,
    blend: Blend,
    stats: StatsOverlay,
    /// gemessene Bildrate und Zeit des letzten Bilds für die Leistungsanzeige (nur mit `StatsOverlay::timing`)
    fps: f64,
    last_draw: f64,
    /// Stufenwechsel wartet auf das Laden (Ladeanzeige statt Spiel)
    loading: bool,
    /// Zustand des Ladens der Grafik (`loading.rs`)
    load: loading::LoadState,
    /// Fehler im Fenster; gesetzt: die Schleife zeigt nur die Meldung (RU30.3)
    fault: Option<Fault>,
    /// Schiffsfahrt beim Inselwechsel (RU30.2)
    voy: Voyage,
    /// F12: das nächste Bild nach reports/ schreiben (RU30.3)
    shot_next: bool,
    shots: Option<Shooter>,
    /// Messmodus (`--measure`), sonst `None` (RU30.4)
    meter: Option<Meter>,
    /// Last-Szene des Messmodus (`--measure-load`, Go: `load`)
    mload: MeasureLoad,
    /// laufender Stresstest, sonst `None` (RU30.5)
    stress: Option<StressRun>,
    /// Uhr, Bildrate, Berichtsordner und Starter des Stresstests (`stress.rs`)
    stress_env: StressEnv,
}

impl Game {
    /// Baut die Schleife. Mit Szene (Seed oder Spielstand) startet das Spiel sofort, sonst liegt das Hauptmenü oben.
    /// Die Schleife läuft `Scene::tick` Takte vor (wie im Spiel: Eingabe, Overlays, Listener; nur ohne Zeichnen) und
    /// legt danach das Menü der Szene auf.
    pub fn new(mut p: Parts, s: Box<dyn Sim>, a: Args) -> Result<Game, String> {
        let mut cams = Cameras::default();
        cams.rule = p.camera.take();
        cams.fixed.clone_from(&a.scene.cams);
        let blend_on = a.shots.is_empty() && (!a.script || a.measure.seconds > 0.0);
        let mut stats = StatsOverlay::new(a.shots.is_empty(), None);
        stats.measuring = a.measure.seconds > 0.0;
        let mut g = Game {
            parts: p,
            sim: s,
            scripted: a.script.then(|| script::scripted(&a.scene)),
            stack: Stack::new(),
            mode: Mode::Menu,
            window: String::new(),
            end_in: 0,
            in_buf: Vec::new(),
            frame: Frame::default(),
            cues: Vec::new(),
            cells: Vec::with_capacity(MAX_PLAYERS),
            win: rect(0, 0, 1920, 1080),
            cams,
            devs: Devices::default(),
            dev: DevView::default(),
            raw_input: InputState::new(),
            tps: TpsMeter::default(),
            err: None,
            blend: Blend::new(blend_on),
            stats,
            fps: 0.0,
            last_draw: 0.0,
            loading: false,
            load: loading::LoadState::default(),
            fault: None,
            voy: Voyage::default(),
            shot_next: false,
            shots: None,
            meter: None,
            mload: MeasureLoad::default(),
            stress: None,
            stress_env: StressEnv::default(),
            args: a,
        };
        g.start()?;
        Ok(g)
    }

    /// Zweiter Teil von `new`: Dev-Schalter, Aufnahme, Messmodus, Szene bzw. Hauptmenü, Vorlauf, Menü der Szene.
    fn start(&mut self) -> Result<(), String> {
        self.apply_dev_args();
        if !self.args.shots.is_empty() {
            self.shots = Some(Shooter::new(&self.args)?);
        }
        if self.args.measure.seconds > 0.0 {
            self.meter = Some(Meter::new(&self.args.measure));
        }
        if self.args.scene.direct() {
            self.start_scene()?;
        } else {
            let main = self.parts.menus.main();
            self.stack.push(main);
        }
        self.prerun()?;
        self.open_menu()?;
        if !self.args.scene.direct() && self.parts.art.is_some() {
            self.sync_levels(); // Menü-Illustrationen vor dem ersten Bild
        }
        Ok(())
    }

    /// Lässt `Scene::tick` Takte vorlaufen; im Messmodus baut es nach dem ersten Takt die Last-Szene auf (die Plätze
    /// der Spieler stehen dann).
    fn prerun(&mut self) -> Result<(), String> {
        let raw = InputState::new();
        for i in 0..self.args.scene.tick {
            self.advance(&raw);
            if i == 0 && self.meter.is_some() {
                let load = self.args.measure.load.clone();
                self.measure_load(&load)?;
            }
        }
        Ok(())
    }

    /// Startet das Spiel der Szene ohne Menü.
    fn start_scene(&mut self) -> Result<(), String> {
        let sc = &self.args.scene;
        let devices = sc.devices();
        let started = if sc.load.is_empty() {
            let g = NewGame {
                seed: sc.seed.clone(),
                depth: sc.depth,
                devices,
                ..NewGame::default()
            };
            self.sim.new_game(g)
        } else {
            self.sim.load(&sc.load, &devices)
        };
        started.map_err(|e| e.to_string())?;
        self.mode = Mode::Play;
        let sc = &self.args.scene;
        log::info(
            "🚀 Szene gestartet",
            &[
                ("seed", &sc.seed),
                ("load", &sc.load),
                ("tiefe", &sc.depth),
                ("spieler", &sc.players),
                ("vorlauf", &sc.tick),
            ],
        );
        Ok(())
    }

    /// Rechnet einen Takt: Eingabe, Menü bzw. Spiel, Simulation, Listener. Im Aufnahme-Modus taktet der `Shooter`; im
    /// Messmodus endet das Programm, sobald der Bericht geschrieben ist. Das Ende meldet `done()`.
    fn update(&mut self, raw: &InputState) {
        if self.mode == Mode::Quit {
            return;
        }
        if let Some(mut sh) = self.shots.take() {
            sh.update(self, raw);
            self.shots = Some(sh);
            return;
        }
        if self.fault.is_some() {
            self.fault_update(raw);
            return;
        }
        if self.err_exit() {
            return;
        }
        if self.meter.as_ref().is_some_and(Meter::done) {
            self.mode = Mode::Quit;
            self.close_sim();
            return;
        }
        self.advance_timed(raw);
        if self.mode == Mode::Quit {
            self.finish();
            self.close_sim();
        }
    }

    /// Schließt die Sitzung beim Ende (Go gab den Fehler aus `Update` zurück, das beendete das Programm).
    fn close_sim(&mut self) {
        if let Err(e) = self.sim.close() {
            log::error("💥 Spiel schließen fehlgeschlagen", &[("fehler", &e)]);
        }
    }

    /// Meldet allen Listenern mit `Finisher` das Programmende (Sitzungsbericht).
    fn finish(&mut self) {
        for f in &mut self.parts.finishers {
            f.finish();
        }
    }

    /// Der Frame des letzten Takts (nur lesen, gilt bis zum nächsten Takt; Ende-zu-Ende-Test).
    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    /// Der oberste Screen des Menü-Stapels (`None` im laufenden Spiel; Ende-zu-Ende-Test).
    pub fn screen(&self) -> Option<&dyn Screen> {
        self.stack.top()
    }

    /// Der Zustand der Schleife.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Liest die Eingabe: die feste Folge der Szene oder die Quelle der Teile.
    fn poll(&mut self, raw: &InputState) -> Input {
        match self.scripted.as_mut() {
            Some(s) => s.poll(raw),
            None => self.parts.input.poll(raw),
        }
    }
}

impl App for Game {
    /// Go: `Update` (ein Takt; `dt` ist `STEP`, die Zeit kommt aus dem Takt).
    fn step(&mut self, input: &InputState, _dt: f64) {
        self.update(input);
    }

    /// Go: `Draw`; `alpha` kommt aus der Zeitsammlung der Plattform.
    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        self.draw_frame(c, alpha);
    }

    /// Go: `ebiten.Termination`.
    fn done(&self) -> bool {
        self.mode == Mode::Quit
    }

    /// Vollbild nach den Einstellungen (`window.rs`).
    fn fullscreen(&self) -> Option<bool> {
        self.fullscreen_wish()
    }
}
