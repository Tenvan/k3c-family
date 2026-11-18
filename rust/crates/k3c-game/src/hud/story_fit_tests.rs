//! Port von `native/k3c-game/hud/story_fit_test.go`: Jede Zeile der Geschichten ist in die Zelle darstellbar und die
//! Geschichtszeile liegt im oberen Drittel. Die Zeilen liest der Test aus `data/worlds/*.json` (Go: `sim.LoadWorld`,
//! RU19) und die Grenzen aus `data/story.json` (Go: `sim.StoryLimitsOf`). Die Prüfung „die Schrift hat ein Glyph für
//! jedes Zeichen“ entfällt: Go liest die Zeichentabelle über `reflect` aus Ebitengine, `art::Font` hat dafür keine
//! Schnittstelle (Ergebnis RU23.2). Ohne die Schrift (`public/schrift/k3c.ttf`) überspringen beide Tests.

use std::path::{Path, PathBuf};

use serde_json::Value;

use super::draw::{BOX_PAD_X, fit_px};
use super::drawstory::story_box;
use super::fonts::{REF_HEIGHT, ROLE_STORY, font_px};
use super::layout::*;
use super::layout_tests::sizes_at;
use crate::art::{Font, load_font};

const LINE_START1: &str = "Oru kamae KRONE no sen.";
const LINE_START2: &str = "Shiro nu BRENNT.";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn font() -> Option<Font> {
    let path = root().join("public/schrift/k3c.ttf");
    let f = load_font(&path, true).ok();
    if f.is_none() {
        eprintln!("übersprungen: Schrift fehlt ({})", path.display());
    }
    f
}

/// strings sammelt alle Texte in Listen unterhalb von `v` (Zeilen der Geschichte).
fn strings(v: &Value, out: &mut Vec<String>) {
    match v {
        Value::String(s) => out.push(s.clone()),
        Value::Array(a) => a.iter().for_each(|x| strings(x, out)),
        Value::Object(o) => o.values().for_each(|x| strings(x, out)),
        _ => {}
    }
}

/// world_file_lines liest die Zeilen der Geschichten aller Welt-Dateien: `story` (ohne den Wortschatz) und je Insel
/// `story` (Ankunft, Boss, Abschluss).
fn world_file_lines() -> Vec<String> {
    let dir = root().join("data/worlds");
    let mut out = Vec::new();
    let mut n = 0;
    for e in std::fs::read_dir(&dir).expect("data/worlds").flatten() {
        if e.path().extension().is_none_or(|x| x != "json") {
            continue;
        }
        n += 1;
        let w: Value = serde_json::from_slice(&std::fs::read(e.path()).unwrap()).unwrap();
        let mut story = Vec::new();
        if let Some(s) = w.get("story").and_then(Value::as_object) {
            for (k, v) in s {
                if k != "lexicon" {
                    strings(v, &mut story);
                }
            }
        }
        for isl in w.get("islands").and_then(Value::as_array).into_iter().flatten() {
            if let Some(s) = isl.get("story") {
                strings(s, &mut story);
            }
        }
        out.extend(story);
    }
    assert!(n > 0, "keine Welt-Dateien unter {}", dir.display());
    out
}

fn max_line_chars() -> usize {
    let v: Value = serde_json::from_slice(&std::fs::read(root().join("data/story.json")).unwrap()).unwrap();
    v["maxLineChars"].as_u64().expect("maxLineChars") as usize
}

/// AC-04: Jede Zeile der Geschichten aller Welt-Dateien passt nach dem Verkleinern in eine halbe Bildschirmbreite bei
/// 1280 × 720. Dazu die Zeichen der Zeilenregel (Apostroph, Bindestrich, Gedankenstrich, Umlaute; welten.md § 2.2) und
/// eine Zeile der größten erlaubten Länge.
#[test]
fn story_lines_fit_cell() {
    let Some(font) = font() else { return };
    let longest: String = "KRONE mw "
        .repeat(max_line_chars() / 9 + 1)
        .chars()
        .take(max_line_chars())
        .collect();
    let mut lines = world_file_lines();
    lines.extend([
        "Akh'et sen-hotep KRONE – nu MÄRZ, ÖL über Straße!".to_string(),
        "Oru no FLUSS? Sen: ÜBEL; nu.".to_string(),
        longest,
    ]);
    let (cell_w, win_h) = (640.0, 720.0);
    let k = win_h / REF_HEIGHT;
    for line in &lines {
        let mut px = font_px(ROLE_STORY, 2) * k;
        let (w, _) = font.measure(line, px);
        px = fit_px(px, w + 2.0 * BOX_PAD_X * k, cell_w - 2.0 * GAP * k);
        let (w, h) = font.measure(line, px);
        let (bw, _) = story_box(w, h, 0.0, 0.0, px, k);
        assert!(
            bw <= cell_w - 2.0 * GAP * k,
            "{line:?}: Fläche {bw:.0} px breit bei Schrift {px:.0} px, Platz {:.0} px",
            cell_w - 2.0 * GAP * k
        );
    }
}

/// Die Geschichtszeile (zwei Zeilen) liegt mit der echten Schrift ganz in der Zelle und im oberen Drittel (B-974/R2),
/// allein und zwischen allen anderen Elementen (Boss und Events weichen nach unten aus), bei 1 und 2 Spielern.
#[test]
fn story_in_upper_third() {
    let Some(font) = font() else { return };
    for (w, h, cells) in [(1280, 720, 2), (1920, 1080, 2), (1920, 1080, 1), (1280, 720, 1)] {
        let area = Rect::new(0.0, 0.0, f64::from(w), f64::from(h / cells));
        let k = f64::from(h) / REF_HEIGHT;
        let px = font_px(ROLE_STORY, i64::from(cells)) * k;
        let (wa, ha) = font.measure(LINE_START1, px);
        let (wb, hb) = font.measure(LINE_START2, px);
        for full in [false, true] {
            let mut inp: Items = if full {
                sizes_at(k, area.w)
            } else {
                [Size::default(); ITEM_COUNT]
            };
            (inp[ITEM_STORY].w, inp[ITEM_STORY].h) = story_box(wa, ha, wb, hb, px, k);
            let pl = layout(&inp, area, &[], k);
            let r = pl.at[ITEM_STORY];
            // allein: Mitte im oberen Drittel; zwischen allen Elementen (Meldung, Uhr, Boss zugleich): Beginn im oberen
            // Drittel, ganz in der oberen Hälfte
            let low = if full {
                r.y > area.h / 3.0 || r.y + r.h > area.h / 2.0
            } else {
                r.y + r.h / 2.0 > area.h / 3.0
            };
            assert!(
                pl.ok[ITEM_STORY] && r.inside(area) && !low,
                "{w}x{h}/{cells} alle Elemente {full}: Geschichtszeile {r:?} (gezeigt {}) nicht im oberen Drittel von {area:?}",
                pl.ok[ITEM_STORY]
            );
            if full {
                assert!(
                    pl.ok[ITEM_BANNER]
                        && pl.ok[ITEM_BOSS]
                        && !r.overlaps(pl.at[ITEM_BANNER])
                        && !r.overlaps(pl.at[ITEM_BOSS]),
                    "{w}x{h}/{cells}: Geschichtszeile {r:?} trifft Meldung {:?} oder Boss {:?}",
                    pl.at[ITEM_BANNER],
                    pl.at[ITEM_BOSS]
                );
            }
        }
    }
}
