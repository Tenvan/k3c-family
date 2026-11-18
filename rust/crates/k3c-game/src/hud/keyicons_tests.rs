//! Port von `native/k3c-game/hud/keyicons_test.go` (RU20.6): Tasten-Icons im HUD (B-911, NA1.5). Jeder Glyph-Schlüssel hat
//! ein Papierschnitt-Icon aus data/paper.json, ohne Icon bleibt die gemalte Glyphe; Kappen wachsen mit der Beschriftung,
//! Bedienhinweise stehen als Zeile mit Icons.
//!
//! `TestGlyphenMitIcons` steht in `keyicons_menu_tests.rs`.

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use k3c_platform::{NullCanvas, Rect as PRect, TexId};
use serde_json::Value;

use super::Hud;
use super::drawglyph::*;
use super::drawstock::Icon;
use super::fonts::WHITE;
use super::glyphs::*;
use super::icons::{HUD_ASSETS, KEY_ASSETS};
use super::keyline::*;
use super::paint::TextSlot;
use super::testutil::{StubView, hud};
use crate::art::{Config, Font, Library, SheetFrame, load_font};
use crate::view::profile::{PROFILE_ARROWS as ARROWS, PROFILE_GAMEPAD as GP, PROFILE_WASD as WASD};
use crate::view::roles::AssetSource;
use k3c_sim::state::World;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// paper_icons liest die Assets der Gruppe icons aus data/paper.json (`icons/<id>`).
fn paper_icons() -> BTreeSet<String> {
    let raw = std::fs::read_to_string(root().join("data/paper.json")).expect("data/paper.json");
    let v: Value = serde_json::from_str(&raw).expect("paper.json ist JSON");
    fn walk(x: &Value, out: &mut BTreeSet<String>) {
        let Some(m) = x.as_object() else {
            return;
        };
        for (k, sub) in m {
            if sub.get("gruppe").and_then(Value::as_str) == Some("icons") {
                out.insert(format!("icons/{k}"));
            }
            walk(sub, out);
        }
    }
    let mut out = BTreeSet::new();
    walk(&v, &mut out);
    out
}

/// all_glyphs liefert (Sprache, Profil, Aktion, Glyph) aus `glyph_of` über alle Aktionen, Eingabe-Profile und Sprachen.
pub(super) fn all_glyphs() -> Vec<(&'static str, &'static str, &'static str, Glyph)> {
    let mut out = Vec::new();
    for lang in ["de", "en"] {
        for p in [WASD, ARROWS, GP] {
            for a in KEY_ACTIONS.iter().chain(GLYPH_ACTIONS.iter()) {
                out.push((lang, p, *a, glyph_of(lang, a, p)));
            }
        }
    }
    out
}

/// AC-05 (B-911/AC-01): Für jeden Glyph-Schlüssel aus `glyph_of` über alle Aktionen, Profile und Sprachen nennt
/// `icon_asset` ein Icon, das in data/paper.json steht und eine Spec in public/grafik/icons hat; das HUD meldet genau
/// diese Icons.
#[test]
fn jeder_glyph_schluessel_hat_ein_icon() {
    let icons = paper_icons();
    let mut used = BTreeSet::new();
    for (lang, p, a, g) in all_glyphs() {
        if g.key.is_empty() {
            assert!(g.icon.is_empty(), "{lang}/{p}/{a}: Icon {:?} ohne Schlüssel", g.icon);
            continue;
        }
        assert!(
            !g.icon.is_empty() && g.icon == icon_asset(&g.key, &g.label) && icons.contains(&g.icon),
            "{lang}/{p}/{a}: Schlüssel {:?}, Icon {:?} nicht in data/paper.json",
            g.key,
            g.icon
        );
        let spec = root().join(format!("public/grafik/{}.paper.json", g.icon));
        assert!(spec.exists(), "{}: Spec fehlt", g.icon);
        used.insert(g.icon.clone());
    }
    for name in PAD_GLYPHS {
        // auch Tasten mit Bild ohne Aktion (Y, RT)
        let a = icon_asset(&format!("pad:{name}"), name);
        assert!(icons.contains(&a), "pad:{name}: Icon {a:?} nicht in data/paper.json");
        used.insert(a);
    }
    for a in &used {
        assert!(
            KEY_ASSETS.contains(&a.as_str()),
            "Icon {a:?} ist nicht als Asset gemeldet (KEY_ASSETS)"
        );
    }
    assert!(
        used.len() == KEY_ASSETS.len() && KEY_ASSETS.is_sorted(),
        "KEY_ASSETS {KEY_ASSETS:?}, gebraucht {}",
        used.len()
    );
}

/// Die Kappe folgt der Länge der Beschriftung, nicht dem Tastennamen: ein Zeichen schmal, sonst breit, je Sprache.
#[test]
fn kappe_nach_beschriftung() {
    let cases = [
        ("key:E", "E", CAP_NARROW),
        ("key:Z", "Ü", CAP_NARROW),
        ("key:F11", "F11", CAP_WIDE),
        ("key:SPACE", "Leertaste", CAP_WIDE),
        ("key:SPACE", "␣", CAP_NARROW),
        ("key:CONTROLRIGHT", "Strg rechts", CAP_WIDE),
        ("pad:LB", "LB", "icons/xbox-lb"),
        ("pad:UP", "D-Pad ↑", "icons/xbox-up"),
        ("", "dance", ""),
        ("key:", "x", ""),
    ];
    for (key, label, want) in cases {
        assert_eq!(icon_asset(key, label), want, "icon_asset({key:?}, {label:?})");
    }
    let g = glyph_of("de", "skill2", ARROWS);
    assert!(g.icon == CAP_WIDE && g.label == "Entf", "Profil pfeile, Deutsch: {g:?}");
    assert_eq!(glyph_of("de", "skill1", GP).icon, "icons/xbox-lb", "Pad");
}

/// Kappen-Maße: schmal bleibt quadratisch, breit wächst mit der Beschriftung bis `CAP_WIDE_MAX`, danach schrumpft die
/// Schrift; die Beschriftung bleibt immer in der Textfläche (nichts läuft über).
#[test]
fn kappe_waechst_und_schrumpft() {
    const GH: f64 = 40.0;
    for tw in [0.0, 8.0, 20.0, 29.0, 60.0, 100.0, 121.0, 122.0, 200.0, 900.0] {
        check_cap_box(GH, tw, false);
        check_cap_box(GH, tw, true);
    }
    let (w, px) = cap_box(GH, 30.0, true, false);
    assert!(
        w == GH * CAP_WIDE_MIN && px == GH * GLYPH_LABEL_K,
        "kurze Beschriftung: Breite {w}, Schrift {px}"
    );
    let (w, px) = cap_box(GH, 100.0, true, false);
    assert!(
        w == 100.0 + GH * CAP_WIDE_MARGIN && px == GH * GLYPH_LABEL_K,
        "lange Beschriftung streckt die Kappe: Breite {w}, Schrift {px}"
    );
    let (w, px) = cap_box(GH, 900.0, true, true);
    assert!(
        w == 900.0 + GH * CAP_WIDE_MARGIN && px == GH * GLYPH_LABEL_K,
        "Debug-Schrift: Breite {w}, Schrift {px}"
    );
}

/// check_cap_box prüft, dass die Beschriftung der Breite tw in der Textfläche der Kappe bleibt und die Kappe ihre
/// Grenzen hält.
fn check_cap_box(gh: f64, tw: f64, wide: bool) {
    let (w, px) = cap_box(gh, tw, wide, false);
    let (mut text, mut lo, mut hi) = (gh * CAP_NARROW_TEXT, gh, gh);
    if wide {
        (text, lo, hi) = (w - gh * CAP_WIDE_MARGIN, gh * CAP_WIDE_MIN, gh * CAP_WIDE_MAX);
    }
    let drawn = tw * px / (gh * GLYPH_LABEL_K);
    assert!(
        drawn <= text + 1e-9 && px <= gh * GLYPH_LABEL_K && px > 0.0,
        "tw {tw} breit {wide}: Beschriftung {drawn} in Textfläche {text}, Schrift {px}"
    );
    assert!(w >= lo && w <= hi, "tw {tw} breit {wide}: Breite {w}");
}

/// kinds beschreibt die Teile einer Zeile: `<Schlüssel>` für Glyphen, `[Text]` für Text.
fn kinds(l: &KeyLine) -> String {
    l.segs
        .iter()
        .take(l.n)
        .map(|s| {
            if s.g.key.is_empty() {
                format!("[{}]", s.g.label)
            } else {
                format!("<{}>", s.g.key)
            }
        })
        .collect()
}

/// Bedienhinweis als Zeile: Text und Glyphen im Wechsel, der Text (plain) wie vor den Icons.
#[test]
fn bedienhinweis_als_zeile() {
    let ok = arg("ok", vec![new_glyph("key:E", "E")]);
    let l = new_key_line("{x} und {ok}", &[ok]);
    assert_eq!(l.plain, "{x} und E");
    assert_eq!(kinds(&l), "[{x} und]<key:E>");
    // Text nach Text wächst zusammen, Ränder fallen weg, der Text bleibt unberührt
    let l = new_key_line(
        "{a}{b} wählen · {c}",
        &[
            arg("a", vec![new_glyph("key:A", "A"), text_part(" / ")]),
            arg("b", vec![new_glyph("key:D", "D")]),
            arg("c", vec![new_glyph("pad:A", "A"), text_part(" bestätigen")]),
        ],
    );
    assert_eq!(l.plain, "A / D wählen · A bestätigen");
    assert_eq!(kinds(&l), "<key:A>[/]<key:D>[wählen ·]<pad:A>[bestätigen]");
    let mut parts = Vec::new();
    for _ in 0..LINE_SEGS {
        parts.push(new_glyph("key:E", "E"));
        parts.push(text_part(" x "));
    }
    let l = new_key_line("{k}", &[arg("k", parts)]);
    assert!(
        l.n == 0 && l.plain.starts_with("E x E x"),
        "zu viele Teile: n {}, Text {:?}",
        l.n,
        l.plain
    );
}

/// Die Zeilen des Skill-Menüs und eine Vorlage mit unbekanntem Platzhalter (Go: Fälle 1, 2 und 5 von `TestKeyLine`).
#[test]
fn bedienhinweis_menu() {
    let cases = [
        (
            super::skilltext::menu_line("de", WASD),
            "A / D wählen · Leertaste bestätigen · K schließen",
            "<key:A>[/]<key:D>[wählen ·]<key:SPACE>[bestätigen ·]<key:K>[schließen]",
        ),
        (
            super::skilltext::menu_line("de", GP),
            "◄ ► wählen · A bestätigen · B schließen",
            "[◄ ► wählen ·]<pad:A>[bestätigen · B schließen]",
        ),
        (
            super::seekarte::sea_line("en", ARROWS, true),
            "Left arrow / Right arrow choose destination · Enter confirm",
            "<key:LEFT>[/]<key:RIGHT>[choose destination ·]<key:ENTER>[confirm]",
        ),
        (
            super::seekarte::sea_line("de", GP, false),
            "A bestätigen",
            "<pad:A>[bestätigen]",
        ),
        (
            new_key_line("{x} und {ok}", &[arg("ok", vec![new_glyph("key:E", "E")])]),
            "{x} und E",
            "[{x} und]<key:E>",
        ),
    ];
    for (line, plain, segs) in cases {
        assert!(
            line.plain == plain && kinds(&line) == segs,
            "Zeile {:?}: {}, erwartet {plain:?}: {segs}",
            line.plain,
            kinds(&line)
        );
    }
}

fn font() -> Font {
    load_font(&root().join("public/schrift/k3c.ttf"), false).expect("Schrift")
}

/// painted_width ist die Breite der gemalten Glyph g bei Höhe gh wie vor den Icons: Kappe um die Beschriftung (Tastatur,
/// Schultertasten), sonst ein Quadrat.
fn painted_width(f: &Font, g: &Glyph, gh: f64) -> f64 {
    let pad = g.key.split_once(':').map_or("", |(_, n)| n);
    let label = if is_key_glyph(g) {
        g.label.as_str()
    } else if is_shoulder(pad) {
        pad
    } else {
        return gh;
    };
    let (tw, _) = f.measure(label, gh * 0.5);
    f64::max(gh, tw + gh * 0.5)
}

/// check_painted prüft, dass h jede Glyph malt (kein Icon, Breite wie vor den Icons) und den Bedienhinweis als Text lässt.
fn check_painted(name: &str, h: &mut Hud) {
    const GH: f64 = 36.0;
    let mut dst = NullCanvas::default();
    let mut f = font();
    h.paint.stencils(&mut dst);
    for (lang, p, a, g) in all_glyphs() {
        if g.key.is_empty() {
            continue;
        }
        assert!(h.glyph_icon(&g).is_none(), "{name}: {} hat ein Icon", g.key);
        let want = painted_width(&f, &g, GH);
        let w = h.draw_glyph(&mut dst, Some(&mut f), 0, TextSlot::menu_glyph(0), &g, 0.0, 32.0, GH);
        let m = h.glyph_width(Some(&f), 0, TextSlot::menu_cap(0), &g, GH);
        assert!(
            w == want && m == want,
            "{name}: {lang}/{p}/{a} gemalt {w} breit, erwartet {want}"
        );
    }
    let mut l = new_key_line("{ok} bestätigen", &[arg("ok", vec![new_glyph("key:ENTER", "Enter")])]);
    let (_, ok) = h.line_width(Some(&f), &mut l, 24.0);
    assert!(
        !ok && !h.line_icons(&l) && !h.draw_line(&mut dst, Some(&mut f), &mut l, 0.0, 0.0, 24.0, WHITE),
        "{name}: Bedienhinweis mit Icons"
    );
    h.draw_hint(&mut dst, Some(&mut f), &mut l, "Enter bestätigen", 0.0, 0.0, 24.0);
    assert!(dst.texts > 0 || dst.sprites > 0, "{name}: draw_hint zeichnet den Text");
}

/// AC-06 (B-911/AC-02): Ohne Grafik, mit nicht ladbaren oder nicht geladenen Icons zeichnet das HUD die gemalte Glyphe
/// (Breite wie vor den Icons) und den Bedienhinweis als Text; nicht ladbare Icons meldet es nicht als Asset.
#[test]
fn ohne_icon_gemalte_glyphe() {
    check_painted("ohne Grafik", &mut hud());
    let mut soft = hud();
    soft.use_key_icons(&|a| a != "icons/xbox-rs");
    let w = World::default();
    let hud_assets: Vec<String> = HUD_ASSETS.iter().map(|a| (*a).to_string()).collect();
    assert!(
        !soft.key_icons && AssetSource::assets(&soft, "forest", &w) == hud_assets,
        "nicht ladbares Icon: gemeldet {:?}",
        AssetSource::assets(&soft, "forest", &w)
    );
    check_painted("Icon nicht ladbar", &mut soft);
    soft.use_key_icons(&|_| true);
    let got = AssetSource::assets(&soft, "forest", &w);
    assert!(
        got.is_sorted() && got.len() == HUD_ASSETS.len() + KEY_ASSETS.len() && got.iter().any(|a| a == CAP_WIDE),
        "ladbar: Assets {got:?}"
    );
    // mit Grafik-Bibliothek, aber ohne geladene Tasten-Icons
    let cfg = Config {
        font_path: root().join("public/schrift/k3c.ttf"),
        strict_font: true,
        ..Config::default()
    };
    let lib = Library::new(cfg).expect("Bibliothek");
    let mut h = Hud::new(Some(Rc::new(RefCell::new(lib))), Box::new(StubView));
    h.use_key_icons(&|_| true);
    assert!(h.key_icons);
    check_painted("Icons nicht geladen", &mut h);
}

/// Mit geladenen Icons zeichnet das HUD die Kappe: in ihrer Breite ein Bild, breiter in drei Streifen (Ecken
/// unverzerrt), die Xbox-Taste als ein Bild.
#[test]
fn kappe_in_drei_streifen() {
    let ic = Icon {
        frame: SheetFrame {
            tex: TexId(1),
            src: PRect::new(0.0, 0.0, 48.0, 32.0),
        },
        fw: 48,
        fh: 32,
    };
    let sprites = |w: f64| {
        let mut h = hud();
        let mut c = NullCanvas::default();
        h.draw_cap(&mut c, &ic, 0.0, 0.0, w, 32.0);
        c.sprites
    };
    assert_eq!(sprites(48.0), 1, "eigene Breite: ein Bild");
    assert_eq!(sprites(100.0), 3, "breiter: drei Streifen");
    assert_eq!(sprites(30.0), 1, "schmaler: ein Bild");
}
