//! RU39.1: Die Wiedergabe zeichnet mit den Zeichnern des Spiels (Welt, Bauten, Figuren, Effekte, HUD) und läuft die
//! eingecheckte Aufzeichnung ohne Panik durch.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use k3c_platform::{App, InputState, NullCanvas, STEP};

use super::app::ReplayApp;
use super::run::run_headless_with;
use super::trace::{load, read_lines};
use crate::art::Config;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn file() -> PathBuf {
    root().join("testdata/rust/trace/nacht-300.jsonl.gz")
}

/// Grafik des Repos (`K3C_GRAFIK_DIR` gilt vor); ohne gerenderte Grafik `None` mit Hinweis (wie `tests/still.rs`).
fn grafik() -> Option<Config> {
    let cfg = crate::still::config(&root(), None);
    let probe = std::fs::read_dir(cfg.grafik_dir.join("hintergruende"))
        .is_ok_and(|d| d.flatten().any(|e| e.path().join("manifest.json").is_file()));
    if !probe {
        assert!(
            std::env::var("K3C_STILL_REQUIRE").is_err(),
            "gerenderte Grafik fehlt (task gen:paper)"
        );
        eprintln!("übersprungen: gerenderte Grafik fehlt (task gen:paper oder K3C_GRAFIK_DIR)");
    }
    probe.then_some(cfg)
}

/// Zeichnet ein Bild Drawer für Drawer und zählt je Modul (Sprites, alle Zeichenbefehle).
fn count(app: &ReplayApp) -> BTreeMap<&'static str, (u64, u64)> {
    let mut c = NullCanvas::default();
    let mut out = BTreeMap::new();
    let (f, cells) = app.frame();
    for cell in cells {
        for (name, d) in &app.scene().unwrap().drawers {
            let (s0, a0) = (c.sprites, c.sprites + c.rects + c.texts + c.meshes + c.shapes);
            d.draw(&mut c, f, cell);
            let e = out.entry(*name).or_insert((0, 0));
            e.0 += c.sprites - s0;
            e.1 += c.sprites + c.rects + c.texts + c.meshes + c.shapes - a0;
        }
    }
    out
}

#[test]
fn zeichner_des_spiels_in_zwei_zellen() {
    let Some(cfg) = grafik() else { return };
    let mut app = ReplayApp::with_config(load(file()).unwrap(), Some(cfg));
    let mut c = NullCanvas::default();
    app.draw(&mut c, 0.0);
    for _ in 0..200 {
        app.step(&InputState::default(), STEP);
    }
    app.draw(&mut c, 0.5);
    let (_, cells) = app.frame();
    assert_eq!(cells.len(), 2, "zwei Spieler, zwei Zellen");
    assert!(cells[0].rect.max.y <= cells[1].rect.min.y, "übereinander");
    let n = count(&app);
    eprintln!("je Modul (Sprites, Befehle): {n:?}");
    for m in ["world", "props", "actors", "hud"] {
        assert!(n.get(m).is_some_and(|v| v.0 > 0), "{m} zeichnet keine Sprites: {n:?}");
    }
    assert!(n.contains_key("fx"), "fx angemeldet");
}

#[test]
fn reihenfolge_der_ebenen_wie_go() {
    let mut app = ReplayApp::new(load(file()).unwrap());
    app.draw(&mut NullCanvas::default(), 0.0);
    let s = app.scene().unwrap();
    let layers: Vec<_> = s.drawers.iter().map(|(_, d)| d.layer()).collect();
    assert!(layers.is_sorted(), "{layers:?}");
    let names: Vec<_> = s.drawers.iter().map(|(n, _)| *n).collect();
    for m in ["world", "props", "actors", "fx", "hud"] {
        assert!(names.contains(&m), "{m} fehlt: {names:?}");
    }
}

#[test]
fn wiedergabe_ohne_panik_bis_zum_ende() {
    let lines = read_lines(file()).unwrap();
    let hl = run_headless_with(&lines, u64::MAX - 60, grafik()).unwrap();
    assert_eq!(hl.steps, 299);
    assert!(hl.report.app.playback().finished());
}
