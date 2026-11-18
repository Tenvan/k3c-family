//! Bildvergleich Rust gegen Go für die Anzeigen je Hälfte an vier Ständen (RU20.7, B-1014) und Test der Plätze.
//!
//! Die Referenz schreibt `native/k3c-game/hud/rustref_test.go` (`-write`): Zustand `hud-<name>.state.json.gz`, Frame
//! (Sitze, Geräte, Sprache, Profile, Meldungen) `hud-<name>.frame.json`, Sim-Antworten `hud-<name>.simview.json`, Plätze
//! `hud-<name>.layout.json`, Glyph-Schlüssel `hud-<name>.keys.json` und das Go-Bild `hud-<name>.go.png` (nur
//! `drawItems`, ohne Welt-Texte, Seekarte, Menü und Boss-Leiste; Radar und Geschichtszeile zeichnet Rust bis RU23 nicht).
//! Rust zeichnet mit `still::HUD_LAYERS`; Schwellen wie in RU13.6 und RU16.6, unverändert.
//!
//! Die gerenderte Grafik (`public/grafik`, `task gen:paper`) ist nicht eingecheckt: Ohne sie überspringen die
//! Bild- und Platz-Test mit Hinweis (`K3C_STILL_REQUIRE=1`: Fehler); `K3C_GRAFIK_DIR` zeigt auf einen anderen Ordner (die
//! Plätze hängen an den Maßen der Icons des Vorrats). Der Test der Glyph-Schlüssel braucht die Grafik nicht. `K3C_STILL_OUT=<ordner>` schreibt die
//! Rust-Bilder zum Ansehen.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use k3c_game::hud::glyphs::glyph_of;
use k3c_game::hud::layout::{ITEM_BANNER, ITEM_CITIZENS, ITEM_CLOCK, ITEM_EVENTS, ITEM_FIGHT, ITEM_JOIN};
use k3c_game::hud::layout::{ITEM_PLACE, ITEM_PLAYER, ITEM_SHARED, ITEM_SKILLS, ITEM_TRAVEL};
use k3c_game::still::{self, FixedSimView, FrameFile, Image, StillState, pngio};
use serde::Deserialize;

const STAENDE: [&str; 4] = ["tag-allein", "nacht-banner", "pad-tastatur", "englisch-hub"];

/// Schwellen wie RU13.6: mittlere Abweichung je Kanal (0 … 255) und Anteil der Pixel, bei denen ein Kanal um mehr als
/// `PIXEL_TOL` abweicht.
const MAX_MEAN: f64 = 0.5;
const MAX_SHARE: f64 = 0.005;
const PIXEL_TOL: i32 = 24;
/// Obergrenze über das ganze Bild samt Textfeldern (kontrolliert die Maske). RU13.6 und RU16.6 nehmen 1,5; das HUD ist
/// fast nur Text und misst Textzeilen in Rust niedriger (Höhe der Zeile = Schrift statt Zeilenhöhe, Boxen 2,5 px
/// kürzer, Text 2 px höher oder tiefer, B-1063 und Ticket im Ergebnis von RU20.7): gemessen bis 2,6, daher 3,0. Der
/// Nenner ist das ganze Bild (der Grund ist 89 bis 96 %); darum prüfen `box_fill_errors` und `text_bbox` die Flächen und
/// den Text je Element zusätzlich.
const MAX_MEAN_WITH_TEXT: f64 = 3.0;
/// Abweichung der Umrisse von Text (Hellpunkte) zwischen Rust und Go: waagrecht 2 Bildpunkte oder `TEXT_TOL_W` der
/// Breite (der Beitritts-Hinweis in kleiner Schrift ist in Rust 6 % schmaler, Ticket im Ergebnis von RU20.7), die Lage
/// in der Senkrechten wegen B-1063 und der Zeilenhöhe großzügiger.
/// Abweichung von y und Höhe der Textelemente im Platz-Test in px (x und Breite stimmen auf 0,01 genau).
const TEXT_LINE_TOL: f64 = 3.5;
const TEXT_TOL_X: i32 = 2;
const TEXT_TOL_W: f64 = 0.07;
const TEXT_TOL_Y: i32 = 5;

/// Elemente, die nur aus einem Text bestehen: Rust setzt Text rund 2 px tiefer als Go (B-1063), das verfälscht den
/// Pixelvergleich in diesen Feldern. Ihre Lage prüft `plaetze_stimmen_mit_go_ueberein`; Spielerkarte und Vorrat (mit
/// Icons) bleiben im Vergleich, ihr Text ist nur ein kleiner Teil.
const TEXT_ITEMS: [usize; 9] = [
    ITEM_BANNER,
    ITEM_SKILLS,
    ITEM_CLOCK,
    ITEM_FIGHT,
    ITEM_PLACE,
    ITEM_CITIZENS,
    ITEM_EVENTS,
    ITEM_TRAVEL,
    ITEM_JOIN,
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn bild(name: &str, ext: &str) -> PathBuf {
    root().join("testdata/rust/bild").join(format!("hud-{name}.{ext}"))
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
    let f = FrameFile::load(&bild(name, "frame.json")).unwrap();
    still::load_state(&bild(name, "state.json.gz"))
        .unwrap()
        .with_simview(v)
        .with_frame(f)
        .unwrap()
}

fn draw(s: &StillState) -> Image {
    still::render_with(s, &still::config(&root(), None), still::HUD_LAYERS).unwrap()
}

/// Rechteck (x0, y0, x1, y1) in Bildpunkten, das der Vergleich auslässt.
type Mask = (f64, f64, f64, f64);

#[derive(Deserialize)]
struct LayoutFile {
    cells: Vec<LayoutCell>,
}

#[derive(Deserialize)]
struct LayoutCell {
    index: i64,
    items: HashMap<String, [f64; 4]>,
}

fn layout_go(name: &str) -> LayoutFile {
    serde_json::from_slice(&std::fs::read(bild(name, "layout.json")).unwrap()).unwrap()
}

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

/// Felder der reinen Textelemente aus den Plätzen der Go-Referenz, mit zwei Pixeln Rand.
fn text_masks(name: &str) -> Vec<Mask> {
    let mut out = Vec::new();
    for c in layout_go(name).cells {
        for id in TEXT_ITEMS {
            if let Some(r) = c.items.get(&id.to_string()) {
                out.push((r[0] - 2.0, r[1] - 2.0, r[0] + r[2] + 2.0, r[1] + r[3] + 2.0));
            }
        }
        // Spielerkarte: der Text steht rechts vom Geldsack (Quadrat der Boxhöhe)
        if let Some(r) = c.items.get(&ITEM_PLAYER.to_string()) {
            out.push((r[0] + r[3], r[1] - 2.0, r[0] + r[2] + 2.0, r[1] + r[3] + 2.0));
        }
    }
    out
}

/// Umriss der hellen Punkte (weißer Text) in `r` (mit `TEXT_TOL_Y` Rand): (x0, y0, x1, y1) oder None ohne Text.
fn text_bbox(img: &Image, r: &[f64; 4]) -> Option<(i32, i32, i32, i32)> {
    // nach unten Rand (Rust setzt Text tiefer), nach oben keiner: darüber steht der Text des Nachbarn
    let (x0, y0) = ((r[0] as i32 - 2).max(0), (r[1] as i32).max(0));
    let (x1, y1) = (
        ((r[0] + r[2]) as i32 + 2).min(img.width as i32),
        ((r[1] + r[3]) as i32 + TEXT_TOL_Y - 1).min(img.height as i32),
    );
    let mut bb: Option<(i32, i32, i32, i32)> = None;
    for y in y0..y1 {
        for x in x0..x1 {
            let i = ((y as u32 * img.width + x as u32) * 4) as usize;
            if img.pix[i..i + 3].iter().all(|&v| v > 170) {
                bb = Some(bb.map_or((x, y, x, y), |b| (b.0.min(x), b.1.min(y), b.2.max(x), b.3.max(y))));
            }
        }
    }
    bb
}

/// Felder, deren Text im Umriss verglichen wird: die reinen Textelemente und der Vorrat (Zahlen und Skill-Punkte).
fn text_fields(name: &str) -> Vec<(i64, usize, [f64; 4])> {
    let mut out = Vec::new();
    for c in layout_go(name).cells {
        for id in TEXT_ITEMS.into_iter().chain([ITEM_SHARED]) {
            if let Some(r) = c.items.get(&id.to_string()) {
                out.push((c.index, id, *r));
            }
        }
    }
    out
}

/// Farbe der Boxfläche am linken Rand in halber Höhe jedes Elements (innerhalb des Innenabstands, kein Text): prüft
/// Vorhandensein, Farbe und Deckkraft der Flächen, die der maskierte Vergleich auslässt.
fn box_fill_errors(name: &str, rust: &Image, go: &Image) -> Vec<String> {
    let mut out = Vec::new();
    for c in layout_go(name).cells {
        for (id, r) in &c.items {
            let (x, y) = ((r[0] + 2.0) as u32, (r[1] + r[3] / 2.0) as u32);
            let at = |img: &Image| {
                let i = ((y * img.width + x) * 4) as usize;
                [img.pix[i], img.pix[i + 1], img.pix[i + 2]]
            };
            let (a, b) = (at(rust), at(go));
            if (0..3).any(|k| (i32::from(a[k]) - i32::from(b[k])).abs() > 6) {
                out.push(format!(
                    "{name} Zelle {} Element {id}: Boxfarbe Rust {a:?}, Go {b:?}",
                    c.index
                ));
            }
        }
    }
    out
}

fn dump(name: &str, img: &Image) {
    if let Some(dir) = std::env::var("K3C_STILL_OUT").ok().filter(|d| !d.is_empty()) {
        pngio::write_png(&Path::new(&dir).join(format!("hud-{name}.png")), img).unwrap();
    }
}

/// Ohne Grafik: Zustände, Frames und Sim-Antworten lassen sich laden, Sitze und Sprache stimmen.
#[test]
fn zustaende_lassen_sich_laden() {
    for name in STAENDE {
        let s = state(name);
        assert_eq!(s.seats.len(), s.world.players.len(), "{name}");
        let (frame, cells) = s.frame();
        assert_eq!(cells.len(), s.seats.len(), "{name}");
        assert_eq!(
            frame.settings.language == "en",
            name == "englisch-hub",
            "{name}: Sprache"
        );
        assert_eq!(
            frame.unseated.len(),
            usize::from(name == "pad-tastatur"),
            "{name}: freies Gerät"
        );
        assert_eq!(frame.cues.len(), usize::from(name == "nacht-banner"), "{name}: Meldung");
    }
}

/// AC-01: Rust gegen Go an vier Ständen (Münzen, Vorrat, Uhr, Meldung, Beitritt, Sprache, Hub und Bürger).
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
        let (mean, share) = diff(&rust, &go, &text_masks(name));
        let (all, all_share) = diff(&rust, &go, &[]);
        eprintln!(
            "{name}: mittel {mean:.3}, Anteil > {PIXEL_TOL}: {:.3} % (ohne Textfelder; gesamt: mittel {all:.3}, Anteil {:.3} %)",
            share * 100.0,
            all_share * 100.0
        );
        if mean > MAX_MEAN || share > MAX_SHARE || all > MAX_MEAN_WITH_TEXT {
            failed.push(format!("{name}: mittel {mean:.3}, Anteil {:.3} %", share * 100.0));
        }
        failed.extend(box_fill_errors(name, &rust, &go));
        for (cell, id, r) in text_fields(name) {
            let (a, b) = (text_bbox(&rust, &r), text_bbox(&go, &r));
            let ok = match (a, b) {
                (None, None) => false, // Go hat in jedem Feld weißen Text: fehlt er in beiden, prüft der Test nichts
                (Some(a), Some(b)) => {
                    let tol_x = TEXT_TOL_X.max((f64::from(b.2 - b.0) * TEXT_TOL_W) as i32);
                    (a.0 - b.0).abs() <= TEXT_TOL_X
                        && (a.2 - b.2).abs() <= tol_x
                        && (a.1 - b.1).abs() <= TEXT_TOL_Y
                        && ((a.3 - a.1) - (b.3 - b.1)).abs() <= 2
                }
                _ => false,
            };
            if !ok {
                failed.push(format!(
                    "{name} Zelle {cell} Element {id}: Textumriss Rust {a:?}, Go {b:?}"
                ));
            }
        }
    }
    assert!(failed.is_empty(), "Abweichung über der Schwelle: {failed:?}");
}

/// AC-01: Die Elemente jeder Hälfte stehen an denselben Koordinaten wie im Go-Spiel (`layout` je Zelle).
#[test]
fn plaetze_stimmen_mit_go_ueberein() {
    if !grafik_da() {
        return;
    }
    let mut checked = 0;
    for name in STAENDE {
        let s = state(name);
        let rust = still::hud_layout(&s, &still::config(&root(), None)).unwrap();
        let go = layout_go(name);
        assert_eq!(rust.len(), go.cells.len(), "{name}: Zellen");
        for ((index, items), g) in rust.iter().zip(&go.cells) {
            assert_eq!(*index, g.index, "{name}");
            let mine: HashMap<String, [f64; 4]> = items
                .iter()
                .map(|(i, r)| (i.to_string(), [r.x, r.y, r.w, r.h]))
                .collect();
            let (mut a, mut b): (Vec<_>, Vec<_>) = (mine.keys().collect(), g.items.keys().collect());
            a.sort();
            b.sort();
            assert_eq!(a, b, "{name} Zelle {index}: gezeigte Elemente");
            for (id, r) in &g.items {
                let text = TEXT_ITEMS.iter().any(|t| t.to_string() == *id);
                for (k, (x, y)) in mine[id].iter().zip(r).enumerate() {
                    // Zeilenhöhe der Textelemente (Rust misst die Schrift, Go die Zeile): y und Höhe um bis 3,5 px
                    let tol = if text && matches!(k, 1 | 3) {
                        TEXT_LINE_TOL
                    } else {
                        0.01
                    };
                    assert!(
                        (x - y).abs() < tol,
                        "{name} Zelle {index} Element {id}: Rust {:?}, Go {r:?}",
                        mine[id]
                    );
                }
                checked += 1;
            }
            for id in [ITEM_PLAYER, ITEM_CLOCK, ITEM_SHARED] {
                assert!(
                    g.items.contains_key(&id.to_string()),
                    "{name} Zelle {index}: Element {id} fehlt"
                );
            }
        }
    }
    assert!(checked >= 30, "zu wenige Plätze geprüft: {checked}");
}

#[derive(Deserialize)]
struct KeyGlyph {
    player: i64,
    profile: String,
    action: String,
    key: String,
    label: String,
    icon: String,
}

/// AC-02: Auf Stand (c) zeigen die Hinweise je Sitz nur Xbox-Symbole (Pad) und Tastenkappen (Tastatur): Die
/// Glyph-Schlüssel stimmen mit Go überein, das Profil folgt dem Gerät des Sitzes.
#[test]
fn hinweise_zeigen_nur_xbox_symbole_und_tastenkappen() {
    let s = state("pad-tastatur");
    let (frame, _) = s.frame();
    let go: Vec<KeyGlyph> = serde_json::from_slice(&std::fs::read(bild("pad-tastatur", "keys.json")).unwrap()).unwrap();
    assert!(go.len() >= 18, "Schlüssel beider Sitze erwartet");
    let (mut pads, mut keys) = (0, 0);
    for g in &go {
        let seat = s.seats.iter().find(|x| x.player == g.player).unwrap();
        let profile = frame.settings.profile_of(seat.device);
        assert_eq!(profile, g.profile, "Profil von Spieler {}", g.player);
        let mine = glyph_of(&frame.settings.language, &g.action, profile);
        assert_eq!(
            (mine.key.as_str(), mine.label.as_str(), mine.icon.as_str()),
            (g.key.as_str(), g.label.as_str(), g.icon.as_str()),
            "{} {}",
            g.player,
            g.action
        );
        if profile == "gamepad" {
            pads += 1;
            assert!(
                mine.key.starts_with("pad:") && mine.icon.starts_with("icons/xbox-"),
                "{}: {mine:?}",
                g.action
            );
        } else {
            keys += 1;
            assert!(
                mine.key.starts_with("key:") && mine.icon.starts_with("icons/taste-kappe"),
                "{}: {mine:?}",
                g.action
            );
        }
    }
    assert!(
        pads >= 9 && keys >= 9,
        "beide Arten erwartet: Pad {pads}, Tastatur {keys}"
    );
}
