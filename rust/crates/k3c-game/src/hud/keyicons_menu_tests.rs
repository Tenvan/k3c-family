//! `TestGlyphenMitIcons` aus `native/k3c-game/hud/keyicons_test.go` (RU20.6, RU23.5): Mit geladenen Icons zeichnet das HUD
//! sie. Nicht portiert: der Allokations-Teil von `checkLines` (`testing.AllocsPerRun`, das Frame-Budget misst RU39).

use super::Hud;
use super::drawglyph::CAP_WIDE_MIN;
use super::glyphs::{CAP_WIDE, glyph_of};
use super::keyicons_tests::all_glyphs;
use super::paint::TextSlot;
use super::skillmenu::{menu_entries, slot_views};
use super::skilltext::{build_menu, new_menu_key};
use crate::view::profile::{PROFILE_ARROWS as ARROWS, PROFILE_GAMEPAD as GP, PROFILE_WASD as WASD};

/// Mit geladenen Icons: jede Glyph hat ihr Sheet, zwei Spieler mit verschiedenen Profilen je ihre Glyphen, die lange
/// Beschriftung liegt auf der gestreckten Kappe (Go: `TestGlyphenMitIcons`; `checkLines` ohne den Allokations-Teil).
#[test]
fn glyphen_mit_icons() {
    let Some(mut h) = super::seekarte_paper_tests::sea_lib() else {
        eprintln!("übersprungen: Grafik fehlt");
        return;
    };
    let lib = h.lib.clone().expect("Bibliothek");
    let lib = lib.borrow();
    let f = &lib.font;
    assert!(h.key_icons, "Tasten-Icons nicht gemeldet");
    for (lang, p, a, g) in all_glyphs() {
        assert!(
            g.key.is_empty() || h.glyph_icon(&g).is_some(),
            "{lang}/{p}/{a}: Icon {:?} nicht geladen",
            g.icon
        );
    }
    let es = menu_entries(Vec::new(), &["taunt".to_string()], 1, &|id| id == "heal", true);
    let slots = slot_views(&["fireball".into(), "taunt".into(), String::new(), String::new()], &[]);
    h.seats[0].menu_v = build_menu(&new_menu_key("de", ARROWS, 0, 1, &es, &slots), &es);
    h.seats[1].menu_v = build_menu(&new_menu_key("de", GP, 0, 1, &es, &slots), &es);
    let icon = |h: &Hud, p: usize| -> String {
        h.seats[p]
            .menu_v
            .rows
            .iter()
            .find(|r| !r.glyph.key.is_empty())
            .map(|r| r.glyph.icon.clone())
            .unwrap_or_default()
    };
    let (i0, i1) = (icon(&h, 0), icon(&h, 1));
    assert!(
        i0 == CAP_WIDE && i1.starts_with("icons/xbox-"),
        "zwei Spieler: Icons {i0:?} und {i1:?}"
    );
    const GH: f64 = 36.0;
    let long = glyph_of("de", "attack", ARROWS); // „Strg rechts“
    let w = h.glyph_width(Some(f), 0, TextSlot::menu_cap(0), &long, GH);
    let short = h.glyph_width(Some(f), 0, TextSlot::menu_cap(0), &glyph_of("de", "attack", WASD), GH);
    assert!(
        w >= GH * CAP_WIDE_MIN && short == GH,
        "Kappen-Breite lang {w}, kurz {short}"
    );
    for p in 0..2 {
        let mut l = h.seats[p].menu_v.line.clone();
        let (w, ok) = h.line_width(Some(f), &mut l, 24.0);
        assert!(ok && w > 0.0, "Spieler {p}: Zeile {w} breit ({ok})");
    }
}
