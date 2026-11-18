//! Port von `native/k3c-game/hud/menubox_test.go`: Das Skill-Menü liegt ohne Überlappung im Kasten. Zeichnet nichts,
//! misst nur; überspringt ohne die echte Schrift (`public/schrift/k3c.ttf`).

use std::path::Path;

use super::drawmenu::{MENU_PX_FULL, MENU_PX_HALF, MenuBox, MenuSizes, glyph_px, layout_menu};
use super::fonts::REF_HEIGHT;
use super::layout::Rect;
use super::skillmenu::{menu_entries, slot_views};
use super::skilltext::{MenuView, build_menu, new_menu_key};
use crate::art::{Font, load_font};
use crate::view::profile::{PROFILE_ARROWS, PROFILE_GAMEPAD, PROFILE_WASD};

/// measure_menu misst ein Skill-Menü mit der Schrift `f` wie `Hud::draw_menu`.
fn measure_menu(f: &Font, v: &MenuView, base: f64, k: f64) -> MenuSizes {
    let mut m = MenuSizes {
        n: v.n,
        ..MenuSizes::default()
    };
    (m.w[0], m.h[0]) = f.measure(&v.title, base);
    for i in 0..v.n {
        (m.w[i + 1], m.h[i + 1]) = f.measure(&v.rows[i].text, base);
        if !v.rows[i].glyph.key.is_empty() {
            m.w[i + 1] += glyph_px(base) + 8.0 * k;
            m.h[i + 1] = m.h[i + 1].max(glyph_px(base));
        }
    }
    (m.w[v.n + 1], m.h[v.n + 1]) = f.measure(&v.footer, base);
    m
}

/// Skill-Menü ohne Überlappung (B-813 AC-02): Mit der echten Schrift liegen Kopfzeile, Zeilen und Hinweis samt Kontur
/// (1 px) untereinander ohne Berührung und im Kasten, an Tastatur und Pad, bei 1 und 2 Spielern in 720p und 1080p.
#[test]
fn skill_menu_without_overlap() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/schrift/k3c.ttf");
    let Ok(font) = load_font(&path, true) else {
        eprintln!("übersprungen: Schrift fehlt ({})", path.display());
        return;
    };
    let learned = vec!["taunt".to_string()];
    let es = menu_entries(Vec::new(), &learned, 1, &|id| id == "heal", true);
    let slots = slot_views(&["fireball", "taunt", "", ""].map(str::to_string), &[]);
    for dev in [PROFILE_WASD, PROFILE_ARROWS, PROFILE_GAMEPAD] {
        let v = build_menu(&new_menu_key("de", dev, 0, 1, &es, &slots), &es);
        for win in [(1280.0, 720.0, 2.0), (1920.0, 1080.0, 1.0), (1920.0, 1080.0, 2.0)] {
            let cell = Rect::new(0.0, 0.0, win.0, win.1 / win.2);
            let k = win.1 / REF_HEIGHT;
            let mut base = MENU_PX_HALF * k;
            if cell.w / k >= 1400.0 {
                base = MENU_PX_FULL * k;
            }
            let m = measure_menu(&font, &v, base, k);
            let b = layout_menu(cell, k, base, &m, false);
            check_menu_box(&b, v.n, win);
        }
    }
}

/// check_menu_box prüft, dass die Zeilen (um die Kontur erweitert) einander nicht berühren und im Kasten liegen.
fn check_menu_box(b: &MenuBox, n: usize, win: (f64, f64, f64)) {
    let mut parts = vec![b.title];
    parts.extend_from_slice(&b.rows[..n]);
    parts.push(b.footer);
    let grow = |r: Rect| Rect::new(r.x - 1.0, r.y - 1.0, r.w + 2.0, r.h + 2.0);
    for i in 1..parts.len() {
        assert!(
            !grow(parts[i - 1]).overlaps(grow(parts[i])),
            "{win:?}: Teil {} {:?} überlappt Teil {i} {:?}",
            i - 1,
            parts[i - 1],
            parts[i]
        );
    }
    for (i, r) in parts.iter().enumerate() {
        assert!(r.inside(b.box_), "{win:?}: Teil {i} {r:?} nicht im Kasten {:?}", b.box_);
    }
}
