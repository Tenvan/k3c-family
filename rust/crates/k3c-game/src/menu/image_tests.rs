//! Bildvergleich der Menü-Seiten gegen das Go-Spiel (RU9.6, B-1014): Hauptmenü, Lobby, Optionen, Tasten
//! mit Tastatur als Gerät. Die Lizenz-Seite hat seit RU58.6 (B-1139) eine eigene Rust-Baseline `licenses.png` (Text ändert sich absichtlich, Ralf 2026-11-19). Die Go-Bilder stehen in `testdata/rust/bild/menu/<seite>.go.png` (Aufnahme-Modus,
//! `--menu <seite> --size 1280x720`, Version `v0.24.1-164-g694bffa1`); die Anzeige „T 0“ oben rechts zeichnet game
//! selbst und ist ausgenommen. Ohne gerenderte Grafik (`task gen:paper`) überspringt der Test.
//! Mit `K3C_STILL_OUT=<ordner>` schreibt er die Rust-Bilder zum Ansehen.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use super::testutil::{FakeSession, KEYS1};
use super::version::set_version;
use super::{MenuSet, art, new_set};
use crate::art::{Levels, Library};
use crate::still::raster::{Image, SoftCanvas};
use crate::still::{self, pngio};
use crate::view::cell::{Cell, new_camera, rect};
use crate::view::frame::Frame;
use crate::view::keys::{KEY_ACTIONS, KeyRow, KeyTable};
use crate::view::screen::{MenuPages, Screen};
use crate::view::session::Session;
use crate::view::settings::default_settings;

const W: u32 = 1280;
const H: u32 = 720;
/// Schwellen je Seite (wie `tests/still.rs`): mittlere Abweichung je Kanal, Anteil abweichender Pixel. Gemessen:
/// mittel 2,6 … 3,0, abweichend 2,6 … 2,9 %; die Ursache ist Text, der in Rust durchgehend etwa 2 % schmaler läuft
/// als in Go (Hauptmenü-Hinweis 459 statt 468 px, Version 350 statt 358 px): Befund für den Hauptrechner.
const MAX_MEAN: f64 = 3.5;
const MAX_SHARE: f64 = 0.035;
const PIXEL_TOL: i32 = 24;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// Die Tasten der Standardbelegung wie `game.KeyRows` in deutscher Sprache.
fn key_table() -> KeyTable {
    const ROWS: [[&str; 3]; 11] = [
        ["A / D", "Pfeil links / Pfeil rechts", "Stick / Steuerkreuz"],
        ["Shift links", "Shift rechts", "RT"],
        ["Leertaste", "Enter", "A"],
        ["E", "Strg rechts", "X"],
        ["Q", "Einfg", "LB"],
        ["R", "Entf", "RB"],
        ["T", "Ende", "LT"],
        ["Z", "Bild ab", "D-Pad ↑"],
        ["K", "Pfeil unten", "D-Pad ↓"],
        ["Esc", "Esc", "Menu"],
        ["F11", "F11", "RS"],
    ];
    Box::new(|_| {
        KEY_ACTIONS
            .iter()
            .zip(ROWS)
            .map(|(a, r)| KeyRow {
                action: (*a).to_string(),
                left: r[0].to_string(),
                right: r[1].to_string(),
                pad: r[2].to_string(),
            })
            .collect()
    })
}

fn library(canvas: &mut SoftCanvas) -> Option<Rc<RefCell<Library>>> {
    let cfg = still::config(&root(), None);
    let probe = std::fs::read_dir(cfg.grafik_dir.join("hintergruende"))
        .is_ok_and(|d| d.flatten().any(|e| e.path().join("manifest.json").is_file()));
    if !probe {
        // K3C_STILL_REQUIRE=1 (Abnahme): fehlende Grafik ist ein Fehler statt eines Überspringens
        assert!(
            std::env::var("K3C_STILL_REQUIRE").is_err(),
            "gerenderte Grafik fehlt (task gen:paper)"
        );
        eprintln!("übersprungen: gerenderte Grafik fehlt (task gen:paper oder K3C_GRAFIK_DIR)");
        return None;
    }
    let mut lib = Library::new(crate::art::Config {
        rig_dir: PathBuf::new(),
        ..cfg
    })
    .ok()?;
    let assets = art::art(Some(&|a| lib.usable(a)));
    let mut want = Levels::new();
    want.insert("menu".to_string(), assets);
    lib.set_levels(&want, canvas).ok()?;
    Some(Rc::new(RefCell::new(lib)))
}

fn cell() -> Cell {
    let win = rect(0, 0, i64::from(W), i64::from(H));
    Cell {
        index: -1,
        player: -1,
        rect: win,
        camera: new_camera(win, 0.0, 1.0),
    }
}

/// Zeichnet die Seite `name` (zwei Takte, damit Cues und Seitenaufbau durch sind) und liefert das Bild.
fn render(name: &str) -> Option<Image> {
    let mut canvas = SoftCanvas::new(W, H);
    let lib = library(&mut canvas)?;
    let mut set: MenuSet = new_set(Some(lib), Some(key_table()));
    let mut s = FakeSession::default();
    let mut sc: Box<dyn Screen> = if name == "main" {
        Box::new(set.main_screen())
    } else {
        set.page(name, false, KEYS1)?
    };
    sc.update(&crate::view::input::Input::default(), &mut s);
    let f = Frame {
        settings: Session::settings(&s),
        ..Frame::default()
    };
    debug_assert_eq!(f.settings, default_settings());
    sc.draw(&mut canvas, &f, &cell());
    Some(canvas.finish())
}

/// (mittlere Abweichung je Kanal, Anteil abweichender Pixel) ohne die Anzeige oben rechts.
fn diff(a: &Image, b: &Image) -> (f64, f64) {
    assert_eq!((a.width, a.height), (b.width, b.height), "Bildgröße");
    let (mut sum, mut bad, mut n) = (0u64, 0u64, 0u64);
    for (i, (p, q)) in a
        .pix
        .as_chunks::<4>()
        .0
        .iter()
        .zip(b.pix.as_chunks::<4>().0)
        .enumerate()
    {
        let (x, y) = (i as u32 % a.width, i as u32 / a.width);
        if x >= 1100 && y < 80 {
            continue;
        }
        let d: Vec<i32> = (0..3).map(|k| (i32::from(p[k]) - i32::from(q[k])).abs()).collect();
        sum += d.iter().map(|&v| v as u64).sum::<u64>();
        bad += u64::from(d.iter().any(|&v| v > PIXEL_TOL));
        n += 1;
    }
    (sum as f64 / (3 * n) as f64, bad as f64 / n as f64)
}

#[test]
fn menu_pages_match_go_images() {
    let _g = super::testutil::version_guard();
    struct Restore(String);
    impl Drop for Restore {
        fn drop(&mut self) {
            set_version(&self.0);
        }
    }
    let _restore = Restore(super::version::version());
    set_version("v0.24.1-164-g694bffa1");
    let mut report = String::new();
    let mut fails = Vec::new();
    for name in ["main", "lobby", "options", "keys"] {
        let Some(img) = render(name) else {
            return;
        };
        if let Some(dir) = std::env::var("K3C_STILL_OUT").ok().filter(|d| !d.is_empty()) {
            pngio::write_png(&Path::new(&dir).join(format!("menu-{name}.png")), &img).unwrap();
        }
        let go = pngio::read_png(&root().join(format!("testdata/rust/bild/menu/{name}.go.png"))).unwrap();
        let (mean, share) = diff(&img, &go);
        report += &format!("{name}: mittel {mean:.3}, abweichend {share:.4}\n");
        if mean > MAX_MEAN || share > MAX_SHARE {
            fails.push(format!("{name}: mittel {mean:.3}, abweichend {share:.4}"));
        }
    }
    eprintln!("{report}");
    assert!(fails.is_empty(), "Abweichungen über den Schwellen: {fails:?}");
}

/// Lizenz-Seite gegen die eigene Baseline `licenses.png` (RU58.6): Stabilität, keine Go-Vorlage; Abweichung null je Pixel.
/// Neu aufnehmen: `K3C_STILL_OUT=<ordner>` schreibt `menu-licenses.png`, nach `testdata/rust/bild/menu/licenses.png` kopieren.
#[test]
fn licenses_page_matches_own_baseline() {
    let _g = super::testutil::version_guard();
    struct Restore(String);
    impl Drop for Restore {
        fn drop(&mut self) {
            set_version(&self.0);
        }
    }
    let _restore = Restore(super::version::version());
    set_version("v0.24.1-164-g694bffa1");
    let Some(img) = render("licenses") else {
        return;
    };
    if let Some(dir) = std::env::var("K3C_STILL_OUT").ok().filter(|d| !d.is_empty()) {
        pngio::write_png(&Path::new(&dir).join("menu-licenses.png"), &img).unwrap();
    }
    let base = pngio::read_png(&root().join("testdata/rust/bild/menu/licenses.png")).unwrap();
    let (mean, share) = diff(&img, &base);
    assert!(
        mean == 0.0 && share == 0.0,
        "licenses: mittel {mean:.3}, abweichend {share:.4}"
    );
}
