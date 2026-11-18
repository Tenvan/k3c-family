//! Tests zu `glyphs.rs` (Port von `glyphs_test.go`; `TestSchriftrollen` steht in `texts_tests.rs`, es prüft `fonts.rs`).

use super::fonts::contrast_ratio;
use super::glyphs::*;
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::profile::{PROFILE_ARROWS as ARROWS, PROFILE_GAMEPAD as GP, PROFILE_WASD as WASD};

const PAD: DeviceID = DeviceID::new(DeviceKind::Gamepad, 3);
const KEYS1: DeviceID = DeviceID::new(DeviceKind::Keyboard, 0);

/// B-894/AC-04: glyph_of: Aktion + Profil → Glyph-Schlüssel (glyphs.test.ts, S6.2 AC-04, B-149 AC-01); jede Aktion hat
/// in jedem Profil ein Icon, nur Xbox-Glyphen am Pad.
#[test]
fn glyph_schluessel() {
    let cases: [(&str, [&str; 9]); 3] = [
        (
            GP,
            [
                "pad:A", "pad:MENU", "pad:RS", "pad:X", "pad:LB", "pad:RB", "pad:LT", "pad:UP", "pad:DOWN",
            ],
        ),
        (
            WASD,
            [
                "key:SPACE",
                "key:ESC",
                "key:F11",
                "key:E",
                "key:Q",
                "key:R",
                "key:T",
                "key:Z",
                "key:K",
            ],
        ),
        (
            ARROWS,
            [
                "key:ENTER",
                "key:ESC",
                "key:F11",
                "key:CONTROLRIGHT",
                "key:INSERT",
                "key:DELETE",
                "key:END",
                "key:PAGEDOWN",
                "key:DOWN",
            ],
        ),
    ];
    for (dev, keys) in cases {
        let mut got = Vec::new();
        for a in GLYPH_ACTIONS {
            let g = glyph_of("de", a, dev);
            assert!(!g.label.is_empty(), "{dev} {a} ohne Beschriftung");
            assert!(!g.icon.is_empty(), "{dev} {a} ohne Icon");
            got.push(g.key);
        }
        assert_eq!(got, keys, "{dev}");
    }
}

#[test]
fn glyph_beschriftung() {
    let label = |lang, a, p| glyph_of(lang, a, p).label;
    assert_eq!(label("de", "confirm", WASD), "Leertaste");
    assert_eq!(label("en", "confirm", WASD), "Space");
    assert_eq!(label("de", "skillMenu", GP), "D-Pad ↓");
    assert_eq!(label("de", "skill2", ARROWS), "Entf");
    assert_eq!(label("en", "skill4", ARROWS), "Page Down");
    assert_eq!(label("de", "fullscreen", GP), "RS");
    assert_eq!(label("de", "dance", GP), "dance");
    assert_eq!(slot_labels("de", WASD), ["E", "Q", "R", "T", "Z", "K"]);
    assert_eq!(slot_labels("de", GP), ["X", "LB", "RB", "LT", "D-Pad ↑", "D-Pad ↓"]);
}

#[test]
fn b_und_view_nie_als_glyph() {
    for d in [GP, WASD, ARROWS] {
        for a in GLYPH_ACTIONS {
            let k = glyph_of("de", a, d).key;
            assert!(k != "pad:B" && k != "pad:VIEW", "{a} auf {d}: {k}");
        }
    }
}

#[test]
fn glyph_farben_kontrast() {
    for (name, c) in glyph_colors() {
        let r = contrast_ratio(c[0], c[1]);
        assert!(r >= 4.5, "{name}: Kontrast {r:.2} < 4,5");
    }
}

#[test]
fn tasten_tabellen() {
    assert_eq!(key_name(0, "attack"), "E");
    assert_eq!(key_name(1, "attack"), "CONTROLRIGHT");
    assert_eq!(key_name(0, "left"), "A");
    assert_eq!(key_name(1, "right"), "RIGHT");
    assert_eq!(key_name(0, "dance"), "");
    assert_eq!(KEY_ACTIONS.len(), 11);
    assert!(
        KEY_ACTIONS
            .iter()
            .all(|a| !key_name(0, a).is_empty() && !key_name(1, a).is_empty())
    );
    assert_eq!(key_label("de", "NUM7"), "Num 7");
    assert_eq!(key_label("de", "ESC"), "Esc");
    assert_eq!(key_label("de", "Q"), "Q");
    assert_eq!(pad_label("de", "skill4"), "D-Pad ↑");
    assert_eq!(pad_label("de", "pause"), "Menu");
    assert_eq!(pad_label("de", "sprint"), "RT");
    assert_eq!(pad_label("de", "attack"), "X");
    assert_eq!(pad_label("de", "dance"), "");
    assert_eq!(icon_asset("pad:A", "A"), "icons/xbox-a");
    assert_eq!(icon_asset("key:E", "E"), CAP_NARROW);
    assert_eq!(icon_asset("key:ESC", "Esc"), CAP_WIDE);
    assert_eq!(icon_asset("", "x"), "");
}

#[test]
fn beitritts_hinweis() {
    let hint = |lang, devs: &[DeviceID]| join_from_key(&new_join_key(lang, devs));
    let pad7 = DeviceID::new(DeviceKind::Gamepad, 7);
    assert_eq!(hint("de", &[PAD]), "A drücken zum Beitreten", "freies Pad");
    assert_eq!(hint("de", &[PAD, pad7]), "A drücken zum Beitreten", "zwei Pads einmal");
    assert_eq!(
        hint("de", &[KEYS1, PAD]),
        "Enter drücken zum Beitreten  ·  A drücken zum Beitreten",
        "Tastatur (Pfeile) und Pad"
    );
    assert_eq!(
        hint("de", &[KEYS1]),
        "Enter drücken zum Beitreten",
        "B-946: Enter statt Leertaste"
    );
    assert_eq!(hint("en", &[KEYS1]), "Press Enter to join", "englisch");
    assert_eq!(hint("de", &[]), "", "alle beigetreten");
}

/// Bedienhinweis des Skill-Menüs mit den Tasten des Profils (Go ruft `menuLine`, Datei von RU23).
#[test]
#[ignore = "wartet auf RU23"]
fn menu_footer_per_device() {
    // Erwartet: wasd "A / D wählen · Leertaste bestätigen · K schließen";
    // pfeile "Pfeil links / Pfeil rechts wählen · Enter bestätigen · Pfeil unten schließen";
    // gamepad "◄ ► wählen · A bestätigen · B schließen". RU23 schaltet den Test mit `menu_line` ein.
    unimplemented!("menu_line kommt mit RU23");
}
