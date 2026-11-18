//! Tests aus `native/k3c-game/menu/options_test.go`, dazu B und Esc (auch aus dem Pause-Menü).

use super::keys::Key;
use super::options::{
    OPT_BACK, OPT_CONTROLS, OPT_EFFECTS, OPT_FLASHES, OPT_GUIDE, OPT_LANGUAGE, OPT_MUSIC, OPT_WINDOW, apply_option,
    new_options, option_value,
};
use super::testutil::{FakeSession, KEYS1, PAD1, err_fake, has_cue, new_menus, press, step_through};
use crate::view::frame::CueKind;
use crate::view::input::{Action, DeviceInput, Input};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::Session;
use crate::view::settings::{
    LANGUAGE_ENGLISH, Settings, WINDOW_BORDERLESS, WINDOW_FRAMED, WINDOW_FULLSCREEN, WINDOW_MODES, default_settings,
};

fn st(music: f64, effects: f64, window: &str, lang: &str, from: &Settings) -> Settings {
    Settings {
        music,
        effects,
        window: window.into(),
        language: lang.into(),
        // nicht in den Optionen (controls.rs, testing)
        controls: from.controls.clone(),
        stress: from.stress.clone(),
        ..Default::default()
    }
}

#[test]
#[allow(clippy::too_many_lines)] // Tabelle der Fälle
fn apply_option_cases() {
    let d = default_settings();
    let half = Settings {
        music: 0.5,
        effects: 0.05,
        language: LANGUAGE_ENGLISH.into(),
        ..Default::default()
    };
    let full = st(1.0, 1.0, WINDOW_FULLSCREEN, "de", &d);
    let cases: Vec<(&str, &Settings, usize, Key, Settings)> = vec![
        ("Musik leiser", &d, OPT_MUSIC, Key::Left, st(0.9, 1.0, "", "de", &d)),
        ("Musik über 100 bleibt", &d, OPT_MUSIC, Key::Right, d.clone()),
        ("Musik A ändert nichts", &d, OPT_MUSIC, Key::OK, d.clone()),
        (
            "Effekte rundet auf 10 %",
            &half,
            OPT_EFFECTS,
            Key::Right,
            st(0.5, 0.2, "", "en", &half),
        ),
        (
            "Effekte nicht unter 0",
            &half,
            OPT_EFFECTS,
            Key::Left,
            st(0.5, 0.0, "", "en", &half),
        ),
        (
            "Anzeige A: rahmenlos",
            &d,
            OPT_WINDOW,
            Key::OK,
            st(1.0, 1.0, WINDOW_BORDERLESS, "de", &d),
        ),
        (
            "Anzeige rechts: rahmenlos",
            &d,
            OPT_WINDOW,
            Key::Right,
            st(1.0, 1.0, WINDOW_BORDERLESS, "de", &d),
        ),
        ("Anzeige links: Vollbild", &d, OPT_WINDOW, Key::Left, full.clone()),
        (
            "Anzeige nach Vollbild: Fenster",
            &full,
            OPT_WINDOW,
            Key::Right,
            d.clone(),
        ),
        (
            "Sprache englisch",
            &d,
            OPT_LANGUAGE,
            Key::Right,
            st(1.0, 1.0, "", "en", &d),
        ),
        (
            "Sprache zurück",
            &half,
            OPT_LANGUAGE,
            Key::OK,
            st(0.5, 0.05, "", "de", &half),
        ),
        ("Zurück ändert nichts", &d, OPT_BACK, Key::Right, d.clone()),
    ];
    for (name, from, opt, k, want) in cases {
        assert_eq!(apply_option(from, opt, k), want, "{name}");
    }
}

#[test]
fn options_writes_through_session() {
    let mut sc = new_options(&new_menus(), false);
    let mut s = FakeSession::default();
    let (l, d) = (press(PAD1, &[Action::Left]), press(PAD1, &[Action::Down]));
    step_through(&mut sc, &mut s, &[l.clone(), l.clone(), d, l]);
    assert_eq!(s.settings.len(), 3, "SetSettings {}-mal", s.settings.len());
    let last = &s.settings[2];
    assert!(
        last.music == 0.8 && last.effects == 0.9 && Session::settings(&s) == *last,
        "zuletzt {last:?}, Session {:?}",
        Session::settings(&s)
    );
}

#[test]
fn options_unchanged_value_is_not_written() {
    let mut s = FakeSession::default();
    let mut sc = new_options(&new_menus(), false);
    step_through(
        &mut sc,
        &mut s,
        &[press(PAD1, &[Action::Right]), press(PAD1, &[Action::Confirm])],
    );
    assert!(
        s.settings.is_empty(),
        "unveränderte Werte gespeichert: {:?}",
        s.settings
    );
}

#[test]
fn options_start_from_known_settings() {
    let mut s = FakeSession {
        cur: Some(Settings {
            music: 0.3,
            effects: 0.4,
            window: WINDOW_FULLSCREEN.into(),
            language: LANGUAGE_ENGLISH.into(),
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut sc = new_options(&new_menus(), true);
    sc.update(&press(PAD1, &[Action::Right]), &mut s);
    let got = &s.settings[0];
    assert!(
        got.music == 0.4 && got.effects == 0.4 && got.window == WINDOW_FULLSCREEN && got.language == LANGUAGE_ENGLISH,
        "Basis nicht von der Session: {got:?}"
    );
}

#[test]
fn options_save_error_keeps_values() {
    let mut sc = new_options(&new_menus(), false);
    let mut s = FakeSession {
        set_err: Some(err_fake()),
        ..Default::default()
    };
    sc.update(&press(PAD1, &[Action::Left]), &mut s);
    assert!(
        sc.cur.music == 0.9 && sc.base.note.raw == err_fake().to_string() && sc.base.note.bad,
        "Wert {}, Meldung {:?}",
        sc.cur.music,
        sc.base.note
    );
}

#[test]
fn options_navigation() {
    use Action::{Back, Confirm, Down, Home, Menu, Right, Up};
    let cases: Vec<(&str, Vec<Action>, usize, StepKind)> = vec![
        ("hoch springt ans Ende", vec![Up], OPT_BACK, StepKind::Stay),
        ("runter im Kreis", vec![Up, Down], OPT_MUSIC, StepKind::Stay),
        ("Zurück mit A", vec![Up, Confirm], OPT_BACK, StepKind::Pop),
        ("Zurück mit rechts bleibt", vec![Up, Right], OPT_BACK, StepKind::Stay),
        ("B", vec![Back], OPT_MUSIC, StepKind::Pop),
        ("Menu", vec![Menu], OPT_MUSIC, StepKind::Pop),
        ("Home", vec![Home], OPT_MUSIC, StepKind::Pop),
    ];
    for (name, keys, sel, kind) in cases {
        let mut sc = new_options(&new_menus(), false);
        let mut last = Step::default();
        for a in keys {
            last = sc.update(&press(PAD1, &[a]), &mut FakeSession::default());
        }
        assert!(
            sc.sel == sel && last.kind == kind,
            "{name}: Auswahl {}, Schritt {:?}",
            sc.sel,
            last.kind
        );
    }
}

/// B am Pad und Esc (Back und Menu zugleich) an der Tastatur schließen die Optionen, auch aus dem Pause-Menü (dim).
#[test]
fn options_back_and_esc_close_also_from_pause() {
    let esc = Input {
        devices: vec![DeviceInput {
            device: KEYS1,
            actions: vec![Action::Back, Action::Menu],
            ..Default::default()
        }],
    };
    for dim in [false, true] {
        for input in [press(PAD1, &[Action::Back]), esc.clone()] {
            let step = new_options(&new_menus(), dim).update(&input, &mut FakeSession::default());
            assert!(
                step.kind == StepKind::Pop && has_cue(&step, CueKind::Back),
                "dim {dim}: {:?}",
                step.kind
            );
        }
    }
}

#[test]
fn option_value_texts() {
    let s = Settings {
        music: 0.7,
        effects: 0.0,
        window: WINDOW_FULLSCREEN.into(),
        language: LANGUAGE_ENGLISH.into(),
        ..Default::default()
    };
    let want = [
        "< 70 % >",
        "< 0 % >",
        "< Vollbild >",
        "< English >",
        "an",
        "an",
        "aus",
        "",
        "",
    ];
    for (i, w) in want.iter().enumerate() {
        assert_eq!(option_value("de", &s, i), *w, "Zeile {i}");
    }
}

/// B-826 AC-01: Die Zeile Anzeige nennt jeden Modus in beiden Sprachen; Rechts läuft zyklisch durch alle drei.
#[test]
fn option_window_modes() {
    let want = [
        (WINDOW_FRAMED, ["< Fenster >", "< Windowed >"]),
        (WINDOW_BORDERLESS, ["< Rahmenlos >", "< Borderless >"]),
        (WINDOW_FULLSCREEN, ["< Vollbild >", "< Fullscreen >"]),
    ];
    let mut s = default_settings();
    for _ in WINDOW_MODES {
        let w = want.iter().find(|(m, _)| *m == s.window).unwrap().1;
        let (de, en) = (option_value("de", &s, OPT_WINDOW), option_value("en", &s, OPT_WINDOW));
        assert!(
            de == w[0] && en == w[1],
            "Modus {:?}: {de:?}/{en:?}, erwartet {w:?}",
            s.window
        );
        s = apply_option(&s, OPT_WINDOW, Key::Right);
    }
    assert_eq!(s.window, WINDOW_FRAMED, "nach drei Schritten Fenster");
}

/// Blitze und Anleitung (B-809 AC-07): A legt die Schalter um, die Session bekommt den neuen Stand; Anleitung wieder an
/// setzt „Anleitung gesehen“ zurück.
#[test]
fn options_flashes_and_guide() {
    let mut s = FakeSession {
        cur: Some(Settings {
            music: 1.0,
            effects: 1.0,
            guide_seen: 0b101,
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut sc = new_options(&new_menus(), false);
    sc.sel = OPT_FLASHES;
    sc.update(&press(PAD1, &[Action::Confirm]), &mut s);
    let cur = s.cur.clone().unwrap();
    assert!(
        cur.flashes_off && option_value("de", &cur, OPT_FLASHES) == "aus",
        "Blitze aus: {cur:?}"
    );
    sc.sel = OPT_GUIDE;
    sc.update(&press(PAD1, &[Action::Confirm]), &mut s);
    let cur = s.cur.clone().unwrap();
    assert!(
        cur.guide_off && cur.guide_seen == 0b101,
        "Anleitung aus, gesehen bleibt: {cur:?}"
    );
    sc.update(&press(PAD1, &[Action::Right]), &mut s);
    let cur = s.cur.clone().unwrap();
    assert!(
        !cur.guide_off && cur.guide_seen == 0 && cur.flashes_off,
        "Anleitung wieder an: {cur:?}"
    );
}

/// F2/F3 bei offenen Optionen (Befund Welle 4a): Ändert sich `stats` oder `river` in der Session, nachdem das Menü
/// geöffnet ist, schreibt der nächste Optionswechsel nicht den alten Stand zurück.
#[test]
fn options_keep_stats_and_river() {
    let mut s = FakeSession {
        cur: Some(default_settings()),
        ..Default::default()
    };
    let mut sc = new_options(&new_menus(), true);
    sc.update(&Input::default(), &mut s);
    if let Some(c) = s.cur.as_mut() {
        c.stats = true;
        c.river = crate::view::settings::RIVER_TILES.into();
    }
    sc.update(&press(PAD1, &[Action::Left]), &mut s);
    let got = s.settings.last().unwrap();
    assert!(
        got.stats && got.river == crate::view::settings::RIVER_TILES && got.music == 0.9,
        "F2/F3 verloren: {got:?}"
    );
}

/// Steuerung öffnet mit A die Unterseite; Links/Rechts auf der Zeile ändert nichts.
#[test]
fn options_open_controls() {
    let mut sc = new_options(&new_menus(), false);
    let mut s = FakeSession::default();
    sc.sel = OPT_CONTROLS;
    let step = sc.update(&press(PAD1, &[Action::Right]), &mut s);
    assert!(
        step.kind == StepKind::Stay && s.settings.is_empty(),
        "rechts auf Steuerung: {:?}",
        step.kind
    );
    let step = sc.update(&press(PAD1, &[Action::Confirm]), &mut s);
    assert!(
        step.kind == StepKind::Push && step.next.is_some(),
        "A auf Steuerung: {:?}",
        step.kind
    );
}
