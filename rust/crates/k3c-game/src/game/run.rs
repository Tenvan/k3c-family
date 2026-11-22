//! Port von `main` aus `native/k3c-game/main.go` (RU30.6): Schalter lesen, den Wegwerf-Ordner der Messung anlegen und
//! aufräumen, das Spiel laufen lassen und das Ende melden. `build`, das Fenster und das Fehlerfenster liegen in
//! `launch.rs` (`Launch` zeigt einen Fehler vor dem Spiel mit demselben `Fault` wie einen Fehler im Spiel, Go: `showFault`),
//! die Aufnahme in `shots_run.rs` (eigener Wegwerf-Ordner), `--assets` in `launch::run`.
//!
//! Ein Fehler endet mit `💥 <Meldung>` auf stderr und Exit-Code 1 (Go: `fatal`), das normale Ende meldet
//! `🛑 K3C natives Spiel beendet` auf stdout; `--assets` endet still (Go: `return` vor der Meldung).

use std::process::ExitCode;

use super::Args;
use super::launch::{self, Launch};
use super::shots_run::{remove_temp, shot_saves};
use crate::hud::rec;
use crate::playtest::{self, Playtest};
use k3c_platform::{App, Canvas, Config as WindowConfig, InputState, run_window};
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Meldung nach dem normalen Ende.
pub const END_MESSAGE: &str = "🛑 K3C natives Spiel beendet";

/// Läuft mit den gelesenen Schaltern bis zum Ende. Die Messung (`--measure`) arbeitet in einem Wegwerf-Ordner neben den
/// Spielständen (Autosave verändert `--saves` nicht), der danach verschwindet, auch bei einem Fehler (Go: `removeTemp`
/// vor `fatal`). Aufnahme (`--shots`) und `--assets` kommen ohne Umweg an `launch::run`.
pub fn run(args: &Args) -> Result<(), String> {
    if args.assets || args.measure.seconds <= 0.0 {
        return launch::run(args);
    }
    let tmp = shot_saves(args)?;
    let mut a = args.clone();
    a.saves = tmp.display().to_string();
    let res = launch::run(&a);
    remove_temp(&tmp);
    res
}

/// Liest die Schalter und startet das Spiel; zweiter Wert: `--assets` (endet ohne `🛑`-Meldung).
///
/// `--playtest <ordner>` (WZ66, B-1220) wird vorab herausgelöst; nur im normalen Fenster startet es REC und Marker,
/// sonst (`--assets`, `--shots`, `--measure`) bleibt es ohne Wirkung. Ohne den Schalter läuft alles wie bisher.
pub fn start(rest: &[String]) -> (Result<(), String>, bool) {
    let (dir, rest) = match playtest::flag::extract(rest) {
        Ok(v) => v,
        Err(e) => return (Err(e), false),
    };
    match launch::parse(&rest) {
        Ok(a) => {
            let res = match dir.filter(|_| !a.assets && a.shots.is_empty() && a.measure.seconds <= 0.0) {
                Some(d) => run_playtest(&a, &d),
                None => run(&a),
            };
            (res, a.assets)
        }
        Err(e) => (Err(e), false),
    }
}

/// Meldet das Ergebnis wie Go (`fatal` bzw. `🛑`-Zeile) und liefert den Exit-Code.
pub fn finish(res: Result<(), String>, quiet: bool) -> u8 {
    match res {
        Ok(()) => {
            if !quiet {
                println!("{END_MESSAGE}");
            }
            0
        }
        Err(e) => {
            eprintln!("💥 {e}");
            1
        }
    }
}

/// Der Aufruf `k3c-game [play] [Schalter]`: Exit-Code 0 oder 1.
pub fn play(rest: &[String]) -> ExitCode {
    let (res, quiet) = start(rest);
    ExitCode::from(finish(res, quiet))
}

/// Das Spiel samt Spieltest (umhüllt `Launch` mit REC und Markern; ohne `--playtest` wird es nie gebaut).
pub struct PlaytestApp {
    inner: Launch,
    pt: Playtest,
    last_draw: Option<Instant>,
}

impl PlaytestApp {
    pub fn new(inner: Launch, pt: Playtest) -> PlaytestApp {
        PlaytestApp {
            inner,
            pt,
            last_draw: None,
        }
    }
}

/// Unix-ms der Systemuhr.
fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
}

impl App for PlaytestApp {
    fn step(&mut self, input: &InputState, dt: f64) {
        let frame = self.inner.game().map(|g| g.frame());
        self.pt.step(input, frame, now_ms());
        self.inner.step(&playtest::detect::mask(input), dt);
    }

    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        self.inner.draw(c, alpha);
        let now = Instant::now();
        if let Some(t) = self.last_draw.replace(now) {
            self.pt
                .frame_done(now_ms(), now.duration_since(t).as_secs_f64() * 1000.0);
        }
        c.layer(31);
        c.clip(None);
        rec::draw_rec(c, self.pt.failed());
        if let Some(g) = self.inner.game() {
            for cell in g.frame().cells.iter().filter(|cell| self.pt.marked(cell.player)) {
                rec::draw_marked(c, &cell.rect);
            }
        }
    }

    fn done(&self) -> bool {
        self.inner.done()
    }

    fn fullscreen(&self) -> Option<bool> {
        self.inner.fullscreen()
    }
}

/// Startet das Spiel im Fenster mit Spieltest in `dir`; Fehler wie `game::launch::run`.
pub fn run_playtest(args: &Args, dir: &Path) -> Result<(), String> {
    let mut app = PlaytestApp::new(Launch::new(args), Playtest::open(dir));
    let size = (args.w > 0 && args.h > 0).then_some((args.w as u32, args.h as u32));
    let cfg = WindowConfig {
        title: "K3C".to_string(),
        size,
        novsync: args.measure.no_vsync,
        ..WindowConfig::default()
    };
    run_window(&mut app, cfg)?;
    app.inner.fault().map(str::to_string).map_or(Ok(()), Err)
}

#[cfg(test)]
#[path = "run_tests.rs"]
mod run_tests;
