//! Stichproben zu `texts.rs`, `fonts.rs` und `data.rs` (Go prüft sie über `labels_test.go`, `values_test.go` und
//! `TestSchriftrollen` in `glyphs_test.go`).

use super::data::{end_boss_phases, line_index, line_name, purse_max_gold, skill_catalog, skill_lines};
use super::fonts::{
    BLACK, FontArt, ROLE_BANNER, ROLE_BOSS, ROLE_CITIZEN, ROLE_FIGHT, ROLE_HINT, ROLE_JOIN, ROLE_PLAYER,
    ROLE_ROOM_INFO, ROLE_SHARED, ROLE_TRAVEL, contrast_ratio, font_px, min_font_px, rgb,
};
use super::texts::{Arg, fill, has, name_of, tf, tr};
use crate::view::settings::{DEFAULT_LANGUAGE, LANGUAGE_ENGLISH};

#[test]
fn tabellen_sind_sortiert_und_gleich_gross() {
    use super::texts_de::TEXTS_DE;
    use super::texts_en::TEXTS_EN;
    assert_eq!(TEXTS_DE.len(), 135);
    assert_eq!(TEXTS_EN.len(), 135);
    for t in [TEXTS_DE, TEXTS_EN] {
        assert!(t.windows(2).all(|w| w[0].0 < w[1].0), "Schlüssel nicht aufsteigend");
    }
}

#[test]
fn tr_deutsch_englisch_und_rueckfall() {
    assert_eq!(tr(DEFAULT_LANGUAGE, "site.wall"), "Mauer");
    assert_eq!(tr(LANGUAGE_ENGLISH, "site.wall"), "Wall");
    // jede andere Sprache: Deutsch
    assert_eq!(tr("fr", "site.wall"), "Mauer");
    // fehlender Schlüssel: der Schlüssel selbst
    assert_eq!(tr(DEFAULT_LANGUAGE, "gibt.es.nicht"), "gibt.es.nicht");
    assert_eq!(tr(LANGUAGE_ENGLISH, "gibt.es.nicht"), "gibt.es.nicht");
    assert_eq!(
        tr(DEFAULT_LANGUAGE, "hud.clockDay"),
        "Tag {day}  ·  Dämmerung in {left}"
    );
}

#[test]
fn has_und_name_of() {
    assert!(has("site.wall"));
    assert!(!has("site.nichts"));
    assert_eq!(name_of(LANGUAGE_ENGLISH, "site", "tower", "x"), "Tower");
    assert_eq!(name_of(DEFAULT_LANGUAGE, "site", "nichts", "Rückfall"), "Rückfall");
}

#[test]
fn tf_ersetzt_platzhalter() {
    let de = tf(DEFAULT_LANGUAGE, "hud.travelUp", &[("depth", Arg::Int(3))]);
    assert_eq!(de, "Aufstieg in Tiefe 3");
    let en = tf(LANGUAGE_ENGLISH, "hud.travelUp", &[("depth", Arg::Int(3))]);
    assert_eq!(en, "Going up to depth 3");
    let kv = [("res", Arg::from("Holz")), ("n", Arg::Int(4)), ("max", Arg::Int(10))];
    assert_eq!(tf(DEFAULT_LANGUAGE, "stock.ofMax", &kv), "Holz 4/10");
    // Umlaut und Mittelpunkt bleiben, `\n` aus der Tabelle ist ein Zeilenumbruch
    let kv = [("p", Arg::Int(2)), ("stage", Arg::from("Stufe 1"))];
    assert_eq!(tf(DEFAULT_LANGUAGE, "hud.place", &kv), "P2 · Stufe 1");
    let kv = [("n", Arg::Int(5))];
    assert_eq!(
        tf(DEFAULT_LANGUAGE, "ev.wave", &kv),
        "Portal öffnet sich!\n5 Gegner kommen"
    );
}

#[test]
fn fill_unbekannter_platzhalter_bleibt_stehen() {
    assert_eq!(fill("a {x} b {y} c", &[("x", Arg::Int(1))]), "a 1 b {y} c");
    assert_eq!(fill("ohne", &[]), "ohne");
    assert_eq!(fill("offen {x", &[("x", Arg::Int(1))]), "offen {x");
    assert_eq!(fill("zu } {x}", &[("x", Arg::Int(1))]), "zu } 1");
}

#[test]
fn fill_zahlenformen_wie_go() {
    let f = |a: Arg| fill("{v}", &[("v", a)]);
    assert_eq!(f(Arg::Int(-7)), "-7");
    assert_eq!(f(Arg::Float(3.0)), "3");
    assert_eq!(f(Arg::Float(2.5)), "2.5");
    assert_eq!(f(Arg::Float(0.1)), "0.1");
    assert_eq!(f(Arg::Float(1e21)), "1000000000000000000000");
    assert_eq!(f(Arg::Float(f64::INFINITY)), "+Inf");
    assert_eq!(f(Arg::Str("Text".into())), "Text");
}

/// Schriftrollen (fontRules.ts, Q03): Pflicht-Info mindestens 28 px, Nebeninfo 24 px bei 1–2 Spielern; Kontrast der
/// Farben zur schwarzen Kontur ≥ 4,5 (Go: `TestSchriftrollen`).
#[test]
fn schriftrollen() {
    assert!(
        min_font_px(2, true, FontArt::Info) == 28.0
            && min_font_px(2, false, FontArt::Info) == 24.0
            && min_font_px(2, true, FontArt::Side) == 24.0
            && min_font_px(4, false, FontArt::Side) == 20.0,
        "Mindestgrößen wie bedienung.md § 2"
    );
    let roles = [
        ROLE_PLAYER,
        ROLE_SHARED,
        ROLE_FIGHT,
        ROLE_BANNER,
        ROLE_BOSS,
        ROLE_TRAVEL,
        ROLE_JOIN,
        ROLE_HINT,
        ROLE_ROOM_INFO,
        ROLE_CITIZEN,
    ];
    for r in roles {
        assert!(
            font_px(r, 2) >= min_font_px(2, true, r.art),
            "Rolle {r:?} unter der Mindestgröße"
        );
        assert!(
            contrast_ratio(r.clr, BLACK) >= 4.5,
            "Rolle {r:?}: Kontrast zur Kontur zu klein"
        );
    }
    assert!(
        ROLE_HINT.art == FontArt::Side && ROLE_HINT.px == 24.0,
        "Aktions-Hinweis ist Nebeninfo in 24 px"
    );
    assert_eq!(font_px(ROLE_JOIN, 2), 26.0);
    assert_eq!(font_px(ROLE_CITIZEN, 2), 32.0);
}

#[test]
fn rgb_vormultipliziert() {
    assert_eq!(rgb(0xff8000, 1.0), [255, 128, 0, 255]);
    assert_eq!(rgb(0xff8000, 0.5), [127, 64, 0, 127]);
    assert_eq!(rgb(0xffffff, 2.0), [255, 255, 255, 255]);
    assert_eq!(rgb(0xffffff, -1.0), [0, 0, 0, 0]);
}

/// Werte aus data/ (Reihenfolge der Dateien wie `Object.entries`).
#[test]
fn werte_aus_den_daten() {
    assert!(purse_max_gold() > 0);
    let lines = skill_lines();
    let skills = skill_catalog();
    assert!(!lines.is_empty() && !skills.is_empty());
    for s in skills {
        assert!(!s.id.is_empty());
        assert!(
            line_index(&s.line) < lines.len(),
            "Linie {} von {} unbekannt",
            s.line,
            s.id
        );
    }
    assert_eq!(line_index("gibt-es-nicht"), lines.len());
    assert_eq!(line_name("gibt-es-nicht"), "gibt-es-nicht");
    let l0 = &lines[0];
    let want = if l0.name.is_empty() { &l0.id } else { &l0.name };
    assert_eq!(&line_name(&l0.id), want);
    assert!(!end_boss_phases().is_empty());
}
