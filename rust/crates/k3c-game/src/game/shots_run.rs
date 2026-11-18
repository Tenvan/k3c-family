//! Der Lauf des Aufnahme-Modus (Go: `main.go` › `shotSaves`, `removeTemp`, `build` mit `Recording`, `run` mit Ebiten):
//! Der Lauf treibt `Game` selbst, ohne Fenster: Takt (`Game::update` mit dem `Shooter`), Bild (`Game::draw` gegen den
//! Software-Rasterer, `alpha = 0`), Pixel nach `Shooter::store`. Alles hängt nur vom Takt ab (feste Uhr der
//! Spielstände, kein Ton, kein Vollbild).

use std::path::{Path, PathBuf};

use k3c_platform::{App, InputState, log};
use k3c_room::store::Stamp;

use super::launch::{build_game, load_libs};
use super::shots_canvas::ShotCanvas;
use super::{Args, Game};
use crate::local::config::Config as SessionConfig;
use crate::local::session::Session;

/// Die Uhr der Spielstände im Aufnahme-Modus (das Bild hängt nicht von der Wanduhr ab; Go: `shotTime`).
fn shot_time() -> Stamp {
    Stamp::from_civil(2026, 1, 1, 12, 0, 0, 0)
}

/// Legt den Wegwerf-Ordner der Spielstände einer Aufnahme an; mit `--load` liegt der Spielstand als Kopie darin, damit
/// Autosave den Stand im Ordner `--saves` nicht verändert (Go: `shotSaves`). Der Ordner liegt neben den Spielständen.
pub fn shot_saves(a: &Args) -> Result<PathBuf, String> {
    let saves = Path::new(&a.saves);
    std::fs::create_dir_all(saves).map_err(|e| format!("{}: {e}", saves.display()))?;
    let tmp = (0..1000)
        .map(|i| saves.join(format!(".k3c-shots-{}-{i}", std::process::id())))
        .find(|p| std::fs::create_dir(p).is_ok())
        .ok_or_else(|| format!("{}: kein Wegwerf-Ordner anlegbar", saves.display()))?;
    let name = &a.scene.load;
    if !name.is_empty() {
        let file = format!("{name}.json");
        if let Err(e) = std::fs::copy(saves.join(&file), tmp.join(&file)) {
            remove_temp(&tmp);
            return Err(format!("{name}: Spielstand nicht kopiert: {e}"));
        }
    }
    Ok(tmp)
}

/// Räumt den Wegwerf-Ordner weg; ein Fehler steht nur im Log.
pub fn remove_temp(dir: &Path) {
    if let Err(e) = std::fs::remove_dir_all(dir) {
        log::warn(
            "🧹 Wegwerf-Ordner nicht entfernt",
            &[("ordner", &dir.display().to_string()), ("err", &e)],
        );
    }
}

/// Öffnet die Sitzung für die Aufnahme: Speichern im Takt, feste Uhr (Zeitstempel in der Spielstand-Liste, Insel-ID).
pub fn open_recording(a: &Args) -> Session {
    Session::open(SessionConfig {
        dir: a.saves.clone(),
        now: Some(Box::new(shot_time)),
        recording: true,
        ..SessionConfig::default()
    })
}

/// Der Aufnahme-Modus von `--shots`: Grafik laden, Spiel bauen, Bilder aufnehmen, Wegwerf-Ordner löschen.
pub fn run(args: &Args) -> Result<(), String> {
    let tmp = shot_saves(args)?;
    let mut a = args.clone();
    a.saves = tmp.display().to_string();
    let res = run_in(&a);
    remove_temp(&tmp);
    res
}

fn run_in(a: &Args) -> Result<(), String> {
    let (w, h) = (u32::try_from(a.w).unwrap_or(1), u32::try_from(a.h).unwrap_or(1));
    let mut canvas = ShotCanvas::new(w, h);
    let libs = load_libs(a, &mut canvas)?;
    let mut g = build_game(a, open_recording(a), &libs)?;
    record(&mut g, &mut canvas)
}

/// Nimmt auf, bis der `Shooter` den Streifen geschrieben hat: Takt, Bild, Datei. Endet das Spiel vorher (Fehler beim
/// Laden, Ende über das Menü), ist das ein Fehler (Go beendete dann ohne Streifen und ohne Fehler).
pub fn record(g: &mut Game, canvas: &mut ShotCanvas) -> Result<(), String> {
    let raw = InputState::new();
    while !App::done(g) {
        g.update(&raw);
        if let Some(e) = g.shots.as_ref().and_then(|s| s.error()) {
            return Err(e.to_string());
        }
        if App::done(g) {
            break;
        }
        g.draw_frame(canvas, 0.0);
        let img = canvas.next_image();
        let tick = g.frame.tick;
        if let Some(s) = g.shots.as_mut() {
            s.store(img, tick);
        }
        if let Some(e) = g.shots.as_ref().and_then(|s| s.error()) {
            return Err(e.to_string());
        }
    }
    match g.shots.as_ref() {
        Some(s) if !s.finished() => Err(format!(
            "💥 Aufnahme: das Spiel endete nach {} von {} Bildern",
            s.taken(),
            s.count()
        )),
        _ => Ok(()),
    }
}
