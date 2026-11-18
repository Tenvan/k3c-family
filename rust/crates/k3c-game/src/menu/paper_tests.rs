//! Tests aus `native/k3c-game/menu/paper_test.go`: Farben gegen `stil.paper.json` und Karten im Fenster.
//! `TestBackdropInWindow` und `TestBackdropSwitchesFilterAtMostOnce` gehören zu RU9.2 (Hintergrund).

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use super::draw::style_of;
use super::layout::{
    COL_LINE, MAT_BLUE, MAT_CLOTH, MAT_CORAL, MAT_GOLD, MAT_GREEN, MAT_LINEN, MAT_PURPLE, MAT_RED, Material, Rgba,
    geom_of, rgb,
};
use super::pending::{COL_SHADOW, MAIN_COUNT, PAUSE_ROWS};
use crate::view::cell::{Rectangle, rect};

#[derive(Deserialize)]
struct Stil {
    #[serde(default)]
    materialien: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(default)]
    einzeltoene: BTreeMap<String, String>,
    #[serde(default)]
    papier: Papier,
}

#[derive(Deserialize, Default)]
struct Papier {
    #[serde(default)]
    lage: Lage,
}

#[derive(Deserialize, Default)]
struct Lage {
    #[serde(default)]
    farbe: String,
}

fn hex_color(s: &str) -> Rgba {
    assert!(s.len() == 7, "Farbe {s:?}");
    rgb(u32::from_str_radix(&s[1..], 16).unwrap_or_else(|e| panic!("Farbe {s:?}: {e}")))
}

/// Farben aus `stil.paper.json` (B-812 AC-02): Karten-Materialien (Papier, Kante, Licht), Leinen-Schild, Linie der
/// Schrift und Schatten. Ändert die Datei einen Ton, wird der Test rot und die Konstanten werden nachgezogen.
#[test]
fn colors_match_stil_paper() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/grafik/stil.paper.json");
    let Ok(raw) = std::fs::read(&path) else {
        eprintln!("Stil-Datei fehlt: {}", path.display());
        return;
    };
    let st: Stil = serde_json::from_slice(&raw).unwrap();
    let mats: [(&str, Material); 8] = [
        ("gold", MAT_GOLD),
        ("smaragd", MAT_GREEN),
        ("koenigsblau", MAT_BLUE),
        ("koenigsrot", MAT_RED),
        ("purpur", MAT_PURPLE),
        ("kupfer", MAT_CORAL),
        ("stoff", MAT_CLOTH),
        ("leinen", MAT_LINEN),
    ];
    for (name, m) in mats {
        let Some(want) = st.materialien.get(name) else {
            panic!("Material {name} fehlt in stil.paper.json");
        };
        for (tone, c) in [("licht", m.licht), ("basis", m.basis), ("schatten", m.schatten)] {
            let w = hex_color(&want[tone]);
            assert_eq!(c, w, "{name}.{tone}: {c:?}, Datei {w:?}");
        }
    }
    assert_eq!(COL_LINE, hex_color(&st.einzeltoene["linie"]), "Linie");
    assert_eq!(COL_SHADOW, hex_color(&st.papier.lage.farbe), "Schatten");
}

/// Karten von Haupt- und Pause-Menü (B-812 AC-01): Alle liegen im Fenster, keine überlappt eine andere, auch nicht
/// mit dem Auswahl-Rahmen; die gewählte Karte sieht anders aus (heller, Rahmen, goldhelles Schild).
#[test]
fn cards_in_window_without_overlap() {
    for win in [rect(0, 0, 1280, 720), rect(0, 0, 1920, 1080), rect(100, 50, 740, 410)] {
        for n in [MAIN_COUNT, PAUSE_ROWS] {
            check_cards(win, n);
        }
    }
    for m in [MAT_GOLD, MAT_GREEN, MAT_BLUE, MAT_PURPLE, MAT_RED, MAT_CORAL] {
        let (on, off) = (style_of(m, false, true), style_of(m, false, false));
        assert!(
            !(on == off || !on.frame || off.frame || on.fill == off.fill || on.strip == off.strip),
            "Auswahl nicht hervorgehoben: {on:?} gegen {off:?}"
        );
    }
    let g = style_of(MAT_GOLD, true, false);
    assert_eq!(g.fill, MAT_CLOTH.basis, "ausgegraut aus Stoff: {g:?}");
}

/// Prüft `n` Karten im Fenster `win`: im Bild über der Meldung, mit Auswahl-Rahmen ohne Berührung der Nachbarin.
fn check_cards(win: Rectangle, n: i64) {
    let g = geom_of(win);
    let b = g.frame_width();
    for i in 0..n {
        let (x, y, w, h) = g.row_rect(i);
        assert!(
            !(x - b < win.min.x as f64
                || y - b < win.min.y as f64
                || x + w + b > win.max.x as f64
                || y + h + b > g.note_y),
            "{win:?}: Karte {i} von {n} nicht im Bild über der Meldung"
        );
        let (_, py, _, ph) = g.row_rect(i - 1);
        assert!(
            !(i > 0 && (py + ph + b > y || y - b < py + ph)),
            "{win:?}: Karte {i} berührt Karte {} mit Auswahl-Rahmen",
            i - 1
        );
    }
}
