//! Bildvergleich Rust gegen Go für Bauten und Objekte an fünf Ständen (RU16.6, B-1014) und Test der Positionen.
//!
//! Die Referenz schreibt `native/k3c-game/props/rustref_test.go` (`-write`): Zustand `props-<name>.state.json.gz`, das
//! Go-Bild `props-<name>.go.png` (Boden der Welt ohne Fluss und Licht plus props) und die Antworten der Sim-Funktionen
//! `props-<name>.simview.json` (Orakel für `FixedSimView`, bis RU15 und RU18 die Sim liefern). Rust zeichnet mit
//! `still::PROPS_LAYERS`; Schwellen wie in RU13.6 (`tests/still.rs`), unverändert.
//!
//! Die gerenderte Grafik (`public/grafik`, `task gen:paper`) ist nicht eingecheckt: Ohne sie überspringen die
//! Bildtests mit Hinweis (`K3C_STILL_REQUIRE=1`: Fehler); `K3C_GRAFIK_DIR` zeigt auf einen anderen Ordner. Der Test der
//! Positionen braucht keine Grafik. Mit `K3C_STILL_OUT=<ordner>` schreiben die Tests die Rust-Bilder zum Ansehen.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use std::path::{Path, PathBuf};

use k3c_game::props::plan::{Item, Planner, Shape, visible};
use k3c_game::props::simview::SimView;
use k3c_game::still::{self, FixedSimView, Image, StillState, pngio};

/// Die fünf Stände einer Basis und zwei Stände für die Deckkraft von Glut und Kristall-Schein (Wald mit den Eigenschaften glow und brightness).
const STAENDE: [&str; 7] = [
    "basis-tag",
    "baustelle",
    "mauer-ruine",
    "wald-plantage",
    "kampf",
    "glut",
    "kristall",
];

/// Schwellen wie RU13.6: mittlere Abweichung je Kanal (0 … 255) und Anteil der Pixel, bei denen ein Kanal um mehr als
/// `PIXEL_TOL` abweicht.
const MAX_MEAN: f64 = 0.5;
const MAX_SHARE: f64 = 0.005;
const PIXEL_TOL: i32 = 24;
/// Obergrenze über das ganze Bild samt Textfeldern (kontrolliert die Maske; Text steht 2 px tiefer, B-1063).
const MAX_MEAN_WITH_TEXT: f64 = 1.5;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn bild(name: &str, ext: &str) -> PathBuf {
    root().join("testdata/rust/bild").join(format!("props-{name}.{ext}"))
}

fn grafik_da() -> bool {
    let dir = still::config(&root(), None).grafik_dir;
    let probe = std::fs::read_dir(dir.join("hintergruende"))
        .is_ok_and(|d| d.flatten().any(|e| e.path().join("manifest.json").is_file()));
    if !probe {
        assert!(
            std::env::var("K3C_STILL_REQUIRE").is_err(),
            "gerenderte Grafik fehlt (task gen:paper)"
        );
        eprintln!("übersprungen: gerenderte Grafik fehlt (task gen:paper oder K3C_GRAFIK_DIR)");
    }
    probe
}

fn state(name: &str) -> StillState {
    let v = FixedSimView::load(&bild(name, "simview.json")).unwrap();
    still::load_state(&bild(name, "state.json.gz")).unwrap().with_simview(v)
}

fn draw(s: &StillState) -> Image {
    still::render_with(s, &still::config(&root(), None), still::PROPS_LAYERS).unwrap()
}

/// Rechteck (x0, y0, x1, y1) in Bildpunkten, das der Vergleich auslässt.
type Mask = (f64, f64, f64, f64);

/// (mittlere Abweichung je Kanal, Anteil abweichender Pixel) über die Bildpunkte außerhalb von `mask`.
fn diff(a: &Image, b: &Image, mask: &[Mask]) -> (f64, f64) {
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
        let (x, y) = ((i as u32 % a.width) as f64, (i as u32 / a.width) as f64);
        if mask.iter().any(|m| x >= m.0 && x < m.2 && y >= m.1 && y < m.3) {
            continue;
        }
        let d: Vec<i32> = (0..3).map(|k| (i32::from(p[k]) - i32::from(q[k])).abs()).collect();
        sum += d.iter().map(|&v| v as u64).sum::<u64>();
        bad += u64::from(d.iter().any(|&v| v > PIXEL_TOL));
        n += 1;
    }
    (sum as f64 / (3 * n) as f64, bad as f64 / n as f64)
}

/// Felder der Texte (Namen, Preise, Wartegründe): Rust setzt Text rund 2 Pixel tiefer als Go (B-1063, Schrift der
/// Plattform), das verfälscht den Pixelvergleich dort. Die Lage der Texte prüft `bauten_stehen_an_den_positionen`.
fn text_masks(s: &StillState) -> Vec<Mask> {
    let items = still::props_items(s, &still::config(&root(), None)).unwrap();
    let (_, cells) = s.frame();
    let mut out = Vec::new();
    for (c, list) in cells.iter().zip(&items) {
        for it in list.iter().filter(|i| i.shape == Shape::Text) {
            let size = (it.size * c.camera.scale).max(8.0);
            let half = it.text.chars().count() as f64 * size * 0.35 + 2.0 * (size / 16.0).max(1.0) + 3.0;
            let (x, y) = (c.camera.screen_x(it.x), c.screen_y(it.y) - size * 1.2);
            out.push((x - half, y - 4.0, x + half, y + size * 1.3 + 4.0));
        }
    }
    out
}

fn dump(name: &str, img: &Image) {
    if let Some(dir) = std::env::var("K3C_STILL_OUT").ok().filter(|d| !d.is_empty()) {
        pngio::write_png(&Path::new(&dir).join(format!("props-{name}.png")), img).unwrap();
    }
}

/// Ohne Grafik: Die fünf Zustände und ihre Sim-Antworten sind lesbar, Hub-Stufe und Varianten stehen in der Welt.
#[test]
fn zustaende_lassen_sich_laden() {
    for name in STAENDE {
        let s = state(name);
        assert_eq!(s.world.players.len(), 2, "{name}");
        assert_eq!(s.world.biome.id, "forest", "{name}");
        assert!(s.world.hub_level >= 1, "{name}: Hub-Stufe fehlt");
        assert!(!s.world.sites.is_empty(), "{name}");
        assert!(
            s.world.nodes.iter().any(|n| !n.variant.is_empty()),
            "{name}: Knoten ohne Variante"
        );
        assert!(s.simview.is_some(), "{name}");
    }
}

/// AC-01: Rust gegen Go an fünf Ständen einer Basis: Bauten, Baustellen, Schaden, Ruine, Natur, Zahlstellen.
#[test]
fn bilder_stimmen_mit_go_ueberein() {
    if !grafik_da() {
        return;
    }
    let mut failed = Vec::new();
    for name in STAENDE {
        let rust = draw(&state(name));
        dump(name, &rust);
        let go = pngio::read_png(&bild(name, "go.png")).unwrap();
        let s = state(name);
        let (mean, share) = diff(&rust, &go, &text_masks(&s));
        let (all, _) = diff(&rust, &go, &[]);
        eprintln!(
            "{name}: mittel {mean:.3}, Anteil > {PIXEL_TOL}: {:.3} % (ohne Textfelder; mit: mittel {all:.3})",
            share * 100.0
        );
        if mean > MAX_MEAN || share > MAX_SHARE || all > MAX_MEAN_WITH_TEXT {
            failed.push(format!("{name}: mittel {mean:.3}, Anteil {:.3} %", share * 100.0));
        }
    }
    assert!(failed.is_empty(), "Abweichung über der Schwelle: {failed:?}");
}

fn has_item(list: &[Item], x: f64) -> bool {
    list.iter().any(|i| (i.x - x).abs() < 1e-9)
}

/// AC-02: Burg, Plätze, Natur und Objekte stehen an den Positionen des Zustands. Der Planer (ohne Grafik, Platzhalter-
/// Formen) legt für jedes sichtbare Ding des Zustands einen Eintrag der Zeichenliste an dessen x.
#[test]
fn bauten_stehen_an_den_positionen_des_zustands() {
    let mut checked = 0;
    for name in STAENDE {
        let s = state(name);
        let view = s.simview.clone().unwrap();
        let (frame, cells) = s.frame();
        let mut infos = k3c_game::props::listen::Infos::default();
        infos.listen(&frame, &view);
        let w = &s.world;
        for (i, c) in cells.iter().enumerate() {
            let mut p = Planner::new(None);
            p.viewer = w.players.iter().position(|m| m.index == s.seats[i].monarch);
            p.plan(Some(w), infos.info(0), &c.camera, s.clock, &view);
            let all: Vec<Item> = p.body.iter().chain(&p.top).cloned().collect();
            let k = &c.camera;
            assert!(
                has_item(&all, w.castle.x) || !visible(k, w.castle.x, 8.0),
                "{name}: Burg bei {}",
                w.castle.x
            );
            // unbezahlte Plätze ohne Schild und Preis zeichnen nichts, alle anderen sichtbaren einen Eintrag
            let drawn = |x: &&k3c_sim::state::Site| x.state != "unpaid" && view.site_visible(w, x);
            for site in w.sites.iter().filter(|x| drawn(x) && visible(k, x.x, 5.0)) {
                assert!(
                    has_item(&all, site.x),
                    "{name}: Platz {} ({}) bei x={}",
                    site.id,
                    site.kind,
                    site.x
                );
                checked += 1;
            }
            let nurseries: Vec<f64> = w
                .sites
                .iter()
                .filter(|x| x.kind == "nursery" && x.state == "built")
                .map(|x| x.x)
                .collect();
            for n in w.nodes.iter().filter(|n| visible(k, n.x, 3.0)) {
                if n.kind == "tree" && nurseries.iter().any(|x| (x - n.x).abs() <= 12.0) {
                    continue; // Plantage-Baum: die Plantage zeichnet ihn nach Plätzen
                }
                assert!(has_item(&all, n.x), "{name}: {} bei x={}", n.kind, n.x);
                checked += 1;
            }
        }
    }
    assert!(checked >= 100, "zu wenige Positionen geprüft: {checked}");
}
