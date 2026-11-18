//! Port von `native/k3c-game/hud/banner_fit_test.go`.
//!
//! Die zwei Bild-Tests messen mit der echten Schrift (`public/schrift/k3c.ttf`, fehlt sie: übersprungen).

use std::path::Path;

use serde_json::json;

use super::banner::{BANNER_MAX_CHARS, fit_banner};
use super::draw::{BOX_PAD_X, box_size, fit_px};
use super::fonts::{REF_HEIGHT, ROLE_BANNER, font_px};
use super::layout::*;
use super::testutil::{hud, two_players};
use super::texts::{tf, tr};
use crate::art::{DEFAULT_FONT_PATH, Font, load_font};
use crate::view::frame::Event;
use k3c_sim::state::Event as SimEvent;

fn event(v: serde_json::Value) -> Event {
    let data: SimEvent = v.as_object().cloned().unwrap_or_default();
    Event {
        stage: 0,
        tick: 0,
        data,
    }
}

/// Burgfall-Banner (B-813 AC-01): Im Modus „lost“ (castleFallen und gameOver im selben Takt) nennt die Meldung die
/// Verluste dieses Modus, sonst die des Stufenverlusts; beide ganz, ohne „…“. Braucht `hud.Listen` (RU20.4).
#[test]
fn burgfall_banner_verluste() {
    let cases = [
        (true, "Die Insel ist komplett verloren", "Hälfte der Vorräte"),
        (false, "Hälfte der Vorräte sind verloren", "komplett verloren"),
    ];
    for (over, want, miss) in cases {
        let (_, mut f) = two_players(0);
        f.events = vec![event(json!({"type": "castleFallen"}))];
        if over {
            f.events.push(event(json!({"type": "gameOver"})));
        }
        let mut h = hud();
        h.listen(&f);
        let got = &h.seats[0].banner.showing;
        assert!(
            got.contains(want) && !got.contains(miss) && !got.contains('…'),
            "gameOver {over}: {got:?}"
        );
    }
}

/// font lädt die echte Schrift; fehlt sie, überspringt der Test (Go: `t.Skipf`).
fn font() -> Option<Font> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(DEFAULT_FONT_PATH);
    load_font(&path, true).ok()
}

/// banner_in_zelle: Das Banner mit dem Text liegt bei 2 Spielern in einem Fenster win ganz in der Zelle (fitPx
/// verkleinert die Schrift, wenn nötig); false, wenn nicht.
fn banner_in_zelle(font: &Font, text: &str, win: (f64, f64)) -> bool {
    let area = Rect::new(0.0, 0.0, win.0, win.1 / 2.0);
    let k = area.h * 2.0 / REF_HEIGHT;
    let mut px = font_px(ROLE_BANNER, 2) * k;
    let (w, _) = font.measure(text, px);
    px = fit_px(px, w + 2.0 * BOX_PAD_X * k, area.w - 2.0 * GAP * k);
    let (w, h) = font.measure(text, px);
    let mut inp = Items::default();
    (inp[ITEM_BANNER].w, inp[ITEM_BANNER].h) = box_size(w, h, k);
    let pl = layout(&inp, area, &[], k);
    pl.ok[ITEM_BANNER] && pl.at[ITEM_BANNER].inside(area)
}

const WINDOWS: [(f64, f64); 3] = [(1280.0, 720.0), (1920.0, 1080.0), (640.0, 720.0)];

/// Das Banner liegt mit der echten Schrift ganz in der Zelle (B-813 AC-01): 2 Spieler bei 1280 × 720 und 1920 × 1080,
/// dazu ein schmales Fenster, in dem fitPx die Schrift verkleinert.
#[test]
fn burgfall_banner_im_bild() {
    let Some(font) = font() else { return };
    let text = fit_banner(tr("de", "ev.castleFallen"), BANNER_MAX_CHARS);
    for win in WINDOWS {
        assert!(banner_in_zelle(&font, &text, win), "{win:?}: Banner nicht in der Zelle");
    }
    assert!(
        fit_px(28.0, 100.0, 200.0) == 28.0 && fit_px(28.0, 400.0, 200.0) == 14.0,
        "fit_px: passt bleibt, zu breit im Verhältnis kleiner"
    );
}

/// Die Kampf-Meldungen (KS5, B-934) liegen mit der echten Schrift ganz in der Zelle, auch Kronenraub (längste) mit P4.
#[test]
fn kampf_banner_im_bild() {
    let Some(font) = font() else { return };
    for lang in ["de", "en"] {
        for key in ["ev.crownStolen", "ev.stageSafe", "ev.crownTaken", "ev.crownPickedUp"] {
            let text = fit_banner(&tf(lang, key, &[("p", 4.into())]), BANNER_MAX_CHARS);
            assert!(!text.contains('…'), "{lang} {key} gekürzt: {text:?}");
            for win in WINDOWS {
                assert!(
                    banner_in_zelle(&font, &text, win),
                    "{lang} {key} {win:?}: Banner nicht in der Zelle"
                );
            }
        }
    }
}
