//! Tests des Zeichners (RU20.5): `Drawer::draw` für zwei Spieler mit `NullCanvas`, mit und ohne Schrift, und auf
//! Welten der Aufzeichnung `testdata/rust/trace/nacht-300.jsonl.gz` (Absturzfreiheit mit echten Welten, RU39).

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use k3c_platform::NullCanvas;

use super::testutil::{StubView, hud, set_world, two_players};
use super::{HudDrawer, new};
use crate::art::{Config, Library};
use crate::replay::trace::load;
use crate::view::cell::{Cell, new_camera, rect};
use crate::view::frame::Frame;
use crate::view::layer::{Drawer, Listener};
use crate::view::settings::default_settings;

fn cells() -> [Cell; 2] {
    std::array::from_fn(|i| {
        let r = rect(0, 360 * i as i64, 1280, 360 * (i as i64 + 1));
        Cell {
            index: i as i64,
            player: i as i64,
            rect: r,
            camera: new_camera(r, 0.0, 1.0),
        }
    })
}

fn root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// library lädt die Schrift ohne Sheets (None, wenn die Schrift fehlt).
fn library() -> Option<Rc<RefCell<Library>>> {
    let cfg = Config {
        font_path: root().join("public/schrift/k3c.ttf"),
        strict_font: true,
        ..Config::default()
    };
    Library::new(cfg).ok().map(|l| Rc::new(RefCell::new(l)))
}

fn draw_both(d: &mut HudDrawer, f: &Frame, c: &mut NullCanvas) {
    d.listen(f);
    for cell in &cells() {
        d.draw(c, f, cell);
    }
}

/// Ohne Grafik zeichnet das HUD nur Flächen: Karte (Geldsack-Platz und Gold) je Hälfte, ohne Text und ohne Absturz.
#[test]
fn draw_zwei_spieler_ohne_grafik() {
    let (_, mut f) = two_players(0);
    f.settings = default_settings();
    let mut d = new(None, Box::new(StubView));
    let mut c = NullCanvas::default();
    draw_both(&mut d, &f, &mut c);
    assert!(c.rects >= 2, "je Hälfte mindestens die Karte: {}", c.rects);
    assert_eq!(c.texts, 0, "ohne Schrift kein Text");
    let (rects, textures) = (c.rects, c.textures);
    draw_both(&mut d, &f, &mut c);
    assert_eq!(c.textures, textures, "Vorlagen nur einmal hochgeladen");
    assert_eq!(c.rects, rects * 2, "gleiches Bild, gleiche Zeichenbefehle");
}

/// Mit Schrift stehen Gold, Uhr und Ort als Text je Hälfte an den Plätzen des Layouts.
#[test]
fn draw_zwei_spieler_mit_schrift() {
    let Some(lib) = library() else { return };
    let (_, mut f) = two_players(0);
    f.settings = default_settings();
    let mut d = new(Some(lib), Box::new(StubView));
    let mut c = NullCanvas::default();
    draw_both(&mut d, &f, &mut c);
    assert!(c.texts >= 2 * 2, "Gold und Uhr je Hälfte: {}", c.texts);
    assert!(c.rects >= 2 * 3, "Flächen je Textzeile: {}", c.rects);
}

/// Ein Spieler ohne Zustand (nicht aktiv) zeichnet nichts.
#[test]
fn draw_ohne_spieler_zeichnet_nichts() {
    let d = new(None, Box::new(StubView));
    let mut c = NullCanvas::default();
    d.draw(&mut c, &Frame::default(), &cells()[0]);
    assert_eq!((c.rects, c.textures), (0, 0));
    let _ = hud();
}

/// RU18-Shims (`purse_max_of`, `train_at`, `trade_at`): `listen` und Draw laufen auf Welten der Aufzeichnung ohne Panik
/// durch, auch mit Schrift.
#[test]
fn listen_und_draw_auf_welten_der_aufzeichnung() {
    let t = load(k3c_det::golden::testdata().join("rust/trace/nacht-300.jsonl.gz")).expect("Aufzeichnung");
    let (_, mut f) = two_players(0);
    f.settings = default_settings();
    let mut d = new(library(), Box::new(StubView));
    let mut c = NullCanvas::default();
    for i in [0, 1, 60, 120, 180, 240, t.rows.len() - 1] {
        set_world(&mut f, &t.rows[i].zustand);
        f.clock = i as f64 / 30.0;
        draw_both(&mut d, &f, &mut c);
    }
    assert!(c.rects > 0);
}
