//! Bildvergleich Rust gegen Go für Seekarte und Schiffsfahrt (RU23.6, B-1014), je Szene 1920 × 1080 wie die Aufnahmen
//! von `native/k3c-game/hud/seekarte_shot_test.go` und `voyage_shot_test.go` (gleiche Karte, Sprache, Profile, Zeit).
//!
//! Die Go-Bilder liegen nicht im Repo (je bis 840 KB, Entscheid 🧑 2026-11-18): Aufnehmen im Ordner `native/k3c-game`
//! (Grafik vorher mit `task gen:paper`):
//!
//! ```text
//! K3C_SEA_SHOT=<repo>/testdata/rust/bild/hud/seekarte.go.png go test ./hud/
//! K3C_VOYAGE_SHOT=<repo>/testdata/rust/bild/hud/fahrt.go.png go test ./hud/
//! ```
//!
//! Fehlen Bilder oder die gerenderte Grafik, überspringen die Tests mit Hinweis (`K3C_STILL_REQUIRE=1`: Fehler).
//! `K3C_STILL_OUT=<ordner>` schreibt die Rust-Bilder zum Ansehen. Geschichtszeile: nicht verglichen (Szene fällt aus:
//! `Campaign::world` und die Geschichtstexte kommen mit RU19, `story_*`-Tests sind dort ignoriert; Ticket im Ergebnis).
//! Schwellen wie RU13.6 (mittlere Abweichung je Kanal, Anteil der Pixel über `PIXEL_TOL`), gemessen am ersten Lauf.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use k3c_game::art::{Config, Library};
use k3c_game::hud::Hud;
use k3c_game::hud::fonts::scale_of;
use k3c_game::hud::seekarte::SeaView;
use k3c_game::still::raster::SoftCanvas;
use k3c_game::still::{self, FixedSimView, Image, pngio};
use k3c_game::view::cell::{Cell, new_camera, rect};
use k3c_game::view::frame::{Frame, RouteIsland};
use k3c_game::view::profile::{PROFILE_GAMEPAD, PROFILE_WASD};
use k3c_game::view::roles::AssetSource;
use k3c_game::view::voyage::Voyage;
use k3c_platform::{Canvas, Rect as PRect};
use k3c_sim::state::{SwitchState, SwitchTarget, World};

const W: u32 = 1920;
const H: u32 = 1080;
/// Sekunden seit dem Ablegen im Bild der Fahrt (`voyageShotAt`).
const VOYAGE_AT: f64 = 1.6;

/// Schwellen: mittlere Abweichung je Kanal (0 … 255) und Anteil der Pixel mit einem Kanal über `PIXEL_TOL`.
const PIXEL_TOL: i32 = 24;
const SEA_MEAN: f64 = 1.5;
const SEA_SHARE: f64 = 0.02;
const VOYAGE_MEAN: f64 = 1.0;
const VOYAGE_SHARE: f64 = 0.01;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn go_bild(name: &str) -> PathBuf {
    root().join("testdata/rust/bild/hud").join(format!("{name}.go.png"))
}

/// Karte der Aufnahme: jede Welt-Miniatur, alle Zustände, drei Reihen in der mittleren Etappe (Go: `shotRoute`).
fn shot_route() -> Vec<RouteIsland> {
    let isl = |id: &str, name: &str, etappe, world: &str, stages, chunks| RouteIsland {
        id: id.to_string(),
        name: name.to_string(),
        etappe,
        world: world.to_string(),
        stages,
        chunks,
    };
    vec![
        isl("gruenhain", "Grünhain", 1, "standard", 2, 24),
        isl("kyoto", "Kyoto", 2, "sengoku", 3, 40),
        isl("theben", "Theben", 2, "nil", 5, 100),
        isl("avalon", "Avalon", 2, "britannien", 1, 16),
        isl("nebelkap", "Nebelkap", 3, "standard", 4, 64),
        isl("edo", "Edo", 3, "sengoku", 2, 0),
    ]
}

/// voraussetzungen: Go-Bild und gerenderte Grafik da; sonst Hinweis und `false` (mit `K3C_STILL_REQUIRE=1` Fehler).
fn voraussetzungen(bild: &str) -> bool {
    let grafik = still::config(&root(), None).grafik_dir;
    let ok = go_bild(bild).is_file()
        && std::fs::read_dir(grafik.join("hintergruende"))
            .is_ok_and(|d| d.flatten().any(|e| e.path().join("manifest.json").is_file()));
    if !ok {
        assert!(
            std::env::var("K3C_STILL_REQUIRE").is_err(),
            "Go-Bild {bild} oder gerenderte Grafik fehlt"
        );
        eprintln!("übersprungen: Go-Bild {bild}.go.png oder gerenderte Grafik fehlt (siehe Kopf der Datei)");
    }
    ok
}

/// Setup: Bildfläche, Bibliothek mit den Assets des HUD (samt Seekarte) und das HUD.
fn setup(canvas: &mut SoftCanvas) -> (Rc<RefCell<Library>>, Hud) {
    let cfg = Config {
        rig_dir: PathBuf::new(),
        ..still::config(&root(), None)
    };
    let lib = Rc::new(RefCell::new(Library::new(cfg).unwrap()));
    let h = Hud::new(Some(Rc::clone(&lib)), Box::new(FixedSimView::default()));
    let assets = AssetSource::assets(&h, "forest", &World::default());
    let want: BTreeMap<String, Vec<String>> = [("forest".to_string(), assets)].into();
    lib.borrow_mut().set_levels(&want, canvas).unwrap();
    (lib, h)
}

fn fill(canvas: &mut SoftCanvas, rgb: [u8; 3]) {
    let [r, g, b] = rgb.map(|v| f32::from(v) / 255.0);
    canvas.rect(PRect::new(0.0, 0.0, W as f32, H as f32), [r, g, b, 1.0]);
}

fn cell(i: i64) -> Cell {
    let r = rect(0, i * i64::from(H) / 2, i64::from(W), (i + 1) * i64::from(H) / 2);
    Cell {
        index: i,
        player: i,
        rect: r,
        camera: new_camera(r, 0.0, 1.0),
    }
}

/// Seekarte: zwei Zellen wie bei zwei Spielern, Spieler 1 hat geblättert (Cursor und Wahl auf verschiedenen Inseln).
fn render_sea() -> Image {
    let mut canvas = SoftCanvas::new(W, H);
    let (lib, mut h) = setup(&mut canvas);
    fill(&mut canvas, [0x3c, 0x6e, 0x47]); // Wiese statt Spielwelt, damit die Tafel durchscheint
    let target = |id: &str, name: &str| SwitchTarget {
        id: id.to_string(),
        name: name.to_string(),
        etappe: 2,
    };
    let sw = SwitchState {
        open: true,
        choice: "theben".to_string(),
        completed: vec!["gruenhain".to_string()],
        targets: vec![
            target("kyoto", "Kyoto"),
            target("theben", "Theben"),
            target("avalon", "Avalon"),
        ],
        ..SwitchState::default()
    };
    let route = shot_route();
    for (i, profile) in [PROFILE_WASD, PROFILE_GAMEPAD].into_iter().enumerate() {
        let mut v = SeaView::default();
        v.refresh("de", profile, Some(&sw), &route);
        if i == 1 {
            v.step(&[k3c_game::view::input::Action::Left]); // Spieler 1 blättert
            v.refresh("de", profile, Some(&sw), &route);
        }
        let c = cell(i as i64);
        let art = h.sea_art_of(&v);
        let mut guard = lib.borrow_mut();
        h.draw_sea(&mut canvas, &mut v, &c, scale_of(&c, 2), Some(&mut guard.font), art);
    }
    canvas.finish()
}

/// Fahrt von Grünhain nach Theben zur Zeit `VOYAGE_AT`, im Vollbild.
fn render_voyage() -> Image {
    let mut canvas = SoftCanvas::new(W, H);
    let (_lib, mut h) = setup(&mut canvas);
    let f = Frame {
        route: shot_route(),
        ..Frame::default()
    };
    let v = Voyage {
        from: "gruenhain".to_string(),
        to: "theben".to_string(),
        done: vec!["gruenhain".to_string()],
        open: ["kyoto", "theben", "avalon"].map(String::from).to_vec(),
        elapsed: VOYAGE_AT,
        duration: 3.0,
    };
    assert!(
        h.voyage_ready(&f, &v),
        "Fahrt nicht bereit, obwohl alle Assets geladen sind"
    );
    assert!(h.draw_voyage(&mut canvas, &f, &v), "Fahrt nicht gezeichnet");
    canvas.finish()
}

/// (mittlere Abweichung je Kanal, Anteil abweichender Pixel).
fn diff(a: &Image, b: &Image) -> (f64, f64) {
    assert_eq!((a.width, a.height), (b.width, b.height), "Bildgröße");
    let (mut sum, mut bad, mut n) = (0u64, 0u64, 0u64);
    for (p, q) in a.pix.as_chunks::<4>().0.iter().zip(b.pix.as_chunks::<4>().0) {
        let d: Vec<i32> = (0..3).map(|k| (i32::from(p[k]) - i32::from(q[k])).abs()).collect();
        sum += d.iter().map(|&v| v as u64).sum::<u64>();
        bad += u64::from(d.iter().any(|&v| v > PIXEL_TOL));
        n += 1;
    }
    (sum as f64 / (3 * n) as f64, bad as f64 / n as f64)
}

fn vergleich(name: &str, rust: &Image, max_mean: f64, max_share: f64) {
    if let Ok(dir) = std::env::var("K3C_STILL_OUT") {
        let out = Path::new(&dir).join(format!("hud-{name}.rust.png"));
        pngio::write_png(&out, rust).unwrap();
    }
    let go = pngio::read_png(&go_bild(name)).unwrap();
    let (mean, share) = diff(rust, &go);
    eprintln!("{name}: mittlere Abweichung {mean:.3}, Anteil abweichender Pixel {share:.4}");
    assert!(
        mean <= max_mean && share <= max_share,
        "{name}: mittlere Abweichung {mean:.3} (Schwelle {max_mean}), Anteil {share:.4} (Schwelle {max_share})"
    );
}

/// AC-02: Seekarte in zwei Zellen sieht aus wie im Go-Spiel.
#[test]
fn seekarte_wie_go() {
    if !voraussetzungen("seekarte") {
        return;
    }
    vergleich("seekarte", &render_sea(), SEA_MEAN, SEA_SHARE);
}

/// AC-02: Schiffsfahrt im Vollbild sieht aus wie im Go-Spiel.
#[test]
fn fahrt_wie_go() {
    if !voraussetzungen("fahrt") {
        return;
    }
    vergleich("fahrt", &render_voyage(), VOYAGE_MEAN, VOYAGE_SHARE);
}
