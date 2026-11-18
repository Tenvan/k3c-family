//! Tests aus `native/k3c-game/menu/table_test.go`. `main_shows_version` und `main_first_draw_after_leave` stehen seit RU9.4 in `main_screen_tests.rs`.

use super::layout::geom_of;
use super::licenses::{license_text, licenses};
use super::main_screen::MAIN_LICENSES;
use super::table::{TableKind, new_keys, new_licenses, table_of};
use super::testutil::{FakeSession, KEYS1, PAD1, has_cue, new_menus, press, step_through, version_guard};
use super::texts::{TextKey, tr};
use super::{new_set, new_shared};
use crate::view::cell::rect;
use crate::view::frame::CueKind;
use crate::view::input::{Action, DeviceInput, Input};
use crate::view::keys::{KEY_ACTIONS, KeyRow};
use crate::view::screen::{Screen, StepKind};

/// fake_keys ist eine Tastenbelegung mit einer Zeile je `KEY_ACTIONS` (wie `game.KeyRows`).
fn fake_keys(lang: &str) -> Vec<KeyRow> {
    KEY_ACTIONS
        .iter()
        .map(|a| KeyRow {
            action: (*a).to_string(),
            left: format!("L-{a}"),
            right: format!("R-{a}"),
            pad: format!("P-{lang}"),
        })
        .collect()
}

/// Seite Tasten (B-807 AC-01): eine Zeile je Aktion der Belegung mit beschrifteter Aktion; B, Esc (Back und Menu
/// zugleich) und Home führen zurück. Den Weg Pause › Tasten prüft RU12 (Pause-Menü), hier öffnet der Test die Seite direkt.
#[test]
fn keys_page() {
    let m = new_shared(None, Some(Box::new(fake_keys)));
    let keys = new_keys(&m);
    assert_eq!(keys.kind, TableKind::Keys);
    keys.fill("de");
    {
        let tb = keys.tb.borrow();
        assert_eq!(tb.rows.len(), KEY_ACTIONS.len(), "eine Zeile je Aktion");
        for (i, a) in KEY_ACTIONS.iter().enumerate() {
            let r = &tb.rows[i];
            assert!(
                !(r[0] == *a
                    || r[0].is_empty()
                    || r[1] != format!("L-{a}")
                    || r[2] != format!("R-{a}")
                    || r[3] != "P-de"),
                "Zeile {i} ({a}): {r:?}"
            );
        }
    }
    let mut s = FakeSession::default();
    let esc = Input {
        devices: vec![DeviceInput {
            device: KEYS1,
            actions: vec![Action::Back, Action::Menu],
            ..Default::default()
        }],
    };
    let mut inputs = vec![esc];
    for d in [PAD1, KEYS1] {
        for a in [Action::Back, Action::Home, Action::Confirm] {
            inputs.push(press(d, &[a]));
        }
    }
    for input in inputs {
        let st = new_keys(&m).update(&input, &mut s);
        assert!(
            st.kind == StepKind::Pop && has_cue(&st, CueKind::Back),
            "zurück ins Pause-Menü: {:?}",
            st.kind
        );
    }
    let st = new_keys(&m).update(&press(PAD1, &[Action::Down]), &mut s);
    assert_eq!(st.kind, StepKind::Stay, "Hoch/Runter bleibt");
}

/// Lizenz-Seite (B-808 AC-01): nennt Ebitengine, die Schrift (Go Regular) und Audio; die Lizenzdatei kommt aus derselben Liste.
#[test]
fn licenses_page() {
    let _g = version_guard();
    let m = new_menus();
    for lang in ["de", "en"] {
        let sc = new_licenses(&m);
        sc.fill(lang);
        let tb = sc.tb.borrow();
        let text: String = tb.rows.iter().map(|r| r.join(" ") + "\n").collect();
        for want in [
            "Ebitengine",
            "Go Regular",
            &tr(lang, TextKey::LicFont),
            &tr(lang, TextKey::LicAudio),
            "go-mp3",
            "CC0",
        ] {
            assert!(text.contains(want), "{lang}: Lizenz-Seite ohne {want:?}:\n{text}");
        }
    }
    let file = license_text();
    for l in licenses() {
        let line = format!("{} – {} – {}", l.work, l.holder, l.license);
        assert!(file.contains(&line), "Lizenzdatei ohne {}", l.work);
    }
    // Hauptmenü › Lizenzen (über Testing, Entwickler-Build)
    let mut set = new_set(None, None);
    let mut main = set.main_screen();
    let down = press(PAD1, &[Action::Down]);
    let ins = [
        down.clone(),
        down.clone(),
        down.clone(),
        down,
        press(PAD1, &[Action::Confirm]),
    ];
    let st = step_through(&mut main, &mut FakeSession::default(), &ins);
    assert!(
        st.kind == StepKind::Push && st.next.is_some() && main.sel == MAIN_LICENSES,
        "Hauptmenü › Lizenzen: {:?}",
        st.kind
    );
}

/// Die Tafel liegt im Fenster unter dem Titel und über der Meldung; die Zeilen überlappen nicht.
#[test]
fn table_geometry() {
    for win in [rect(0, 0, 1280, 720), rect(0, 0, 1920, 1080), rect(0, 0, 640, 360)] {
        let g = geom_of(win);
        let tg = table_of(&g, KEY_ACTIONS.len());
        let mut prev_bottom = -1.0;
        for i in 0..=KEY_ACTIONS.len() {
            let (x, y, w, h) = tg.row(i);
            assert!(
                !(x < 0.0
                    || x + w > win.dx() as f64
                    || y < g.title_y + g.title_px
                    || y + h > g.note_y + 1e-9
                    || y < prev_bottom - 1e-9
                    || tg.px > h),
                "{win:?}: Zeile {i} bei {x}/{y} {w}x{h} (Schrift {})",
                tg.px
            );
            prev_bottom = y + h;
        }
    }
}
