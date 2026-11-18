//! Tests aus `native/k3c-game/menu/saves_test.go` (AC-03): Liste, Laden, Löschen mit Rückfrage und nicht ladbare Stände
//! gegen eine Schein-Session; dazu die reine Zeitformatierung (Go rechnet mit `Local()`).

use std::time::{Duration, UNIX_EPOCH};

use super::layout::MAX_ROWS;
use super::saves::{format_saved_at, new_saves, save_row};
use super::testutil::{FakeSession, KEYS1, PAD1, PAD2, err_fake, has_cue, new_menus, press, save, step_through};
use super::texts::TextKey;
use crate::view::frame::CueKind;
use crate::view::input::Action::{self, Back, Confirm, Down, Home, Left, Menu, Right, Up, View};
use crate::view::screen::{Screen, StepKind};
use crate::view::session::SaveInfo;
use crate::view::settings::{DEFAULT_LANGUAGE, LANGUAGE_ENGLISH};

fn three_saves() -> Vec<SaveInfo> {
    vec![
        save("spiel-3", 7, true),
        save("kaputt", 0, false),
        save("spiel-1", 2, true),
    ]
}

fn session(saves: Vec<SaveInfo>) -> FakeSession {
    FakeSession {
        saves,
        ..Default::default()
    }
}

#[test]
fn saves_load() {
    type Case = (
        &'static str,
        Vec<SaveInfo>,
        Vec<Action>,
        StepKind,
        Vec<&'static str>,
        Option<CueKind>,
    );
    let cases: Vec<Case> = vec![
        (
            "neuester zuerst",
            three_saves(),
            vec![],
            StepKind::Play,
            vec!["spiel-3"],
            Some(CueKind::Confirm),
        ),
        (
            "nicht ladbar gesperrt",
            three_saves(),
            vec![Down],
            StepKind::Stay,
            vec![],
            Some(CueKind::Denied),
        ),
        (
            "dritter Stand",
            three_saves(),
            vec![Down, Down, Down],
            StepKind::Play,
            vec!["spiel-1"],
            Some(CueKind::Confirm),
        ),
        ("leere Liste", vec![], vec![Down], StepKind::Stay, vec![], None),
    ];
    for (name, saves, keys, kind, loaded, cue) in cases {
        let mut sc = new_saves(&new_menus());
        let mut s = session(saves);
        for a in keys {
            sc.update(&press(PAD2, &[a]), &mut s);
        }
        let st = sc.update(&press(PAD2, &[Confirm]), &mut s);
        assert!(
            st.kind == kind && s.loads == loaded && cue.is_none_or(|c| has_cue(&st, c)),
            "{name}: Schritt {:?}, geladen {:?}",
            st.kind,
            s.loads
        );
        if !s.loads.is_empty() {
            assert_eq!(s.devs[0], vec![PAD2], "{name}: Geräte");
        }
    }
}

#[test]
fn saves_not_loadable_shows_problem() {
    let mut sc = new_saves(&new_menus());
    let mut s = session(three_saves());
    sc.update(&press(PAD1, &[Down]), &mut s);
    sc.update(&press(PAD1, &[Confirm]), &mut s);
    assert!(
        sc.base.note.raw == "Spielstand nicht ladbar: Version 3" && sc.base.note.bad,
        "Meldung {:?}",
        sc.base.note
    );
    sc.fill(DEFAULT_LANGUAGE);
    let pg = sc.pg.borrow();
    let r = &pg.rows[1];
    assert!(
        r.off && r.value == "Spielstand nicht ladbar: Version 3",
        "Zeile {r:?} nicht markiert"
    );
}

#[test]
fn saves_load_error_stays() {
    let mut sc = new_saves(&new_menus());
    let mut s = FakeSession {
        load_err: Some(err_fake()),
        ..session(three_saves())
    };
    let st = sc.update(&press(PAD1, &[Confirm]), &mut s);
    assert!(
        st.kind == StepKind::Stay && has_cue(&st, CueKind::Denied) && sc.base.note.raw == err_fake().to_string(),
        "Fehler: {:?}, {:?}",
        st.kind,
        sc.base.note
    );
}

type DeleteCase = (&'static str, Vec<Action>, Vec<&'static str>, bool, StepKind);

/// Die Tabelle von `TestSavesDeleteWithQuestion`: Tasten, gelöschte Stände, Rückfrage offen, Schritt.
fn delete_cases() -> Vec<DeleteCase> {
    vec![
        ("View fragt", vec![View], vec![], true, StepKind::Stay),
        ("A auf Abbrechen", vec![View, Confirm], vec![], false, StepKind::Stay),
        (
            "Löschen wählen und bestätigen",
            vec![View, Right, Confirm],
            vec!["spiel-3"],
            false,
            StepKind::Stay,
        ),
        (
            "zweimal wechseln ist Abbrechen",
            vec![View, Down, Up, Confirm],
            vec![],
            false,
            StepKind::Stay,
        ),
        (
            "B bricht ab und bleibt in der Liste",
            vec![View, Right, Back],
            vec![],
            false,
            StepKind::Stay,
        ),
        (
            "View noch einmal bricht ab",
            vec![View, View],
            vec![],
            false,
            StepKind::Stay,
        ),
        (
            "Home verlässt die Liste",
            vec![View, Home],
            vec![],
            false,
            StepKind::Pop,
        ),
        (
            "kaputten Stand löschen",
            vec![Down, View, Left, Confirm],
            vec!["kaputt"],
            false,
            StepKind::Stay,
        ),
    ]
}

#[test]
fn saves_delete_with_question() {
    for (name, keys, deleted, asking, kind) in delete_cases() {
        let mut sc = new_saves(&new_menus());
        let mut s = session(three_saves());
        let mut st = crate::view::screen::Step::default();
        for a in keys {
            st = sc.update(&press(PAD1, &[a]), &mut s);
        }
        assert!(
            s.deleted == deleted && sc.asking == asking && st.kind == kind,
            "{name}: gelöscht {:?}, Rückfrage {}, Schritt {:?}",
            s.deleted,
            sc.asking,
            st.kind
        );
    }
}

#[test]
fn saves_esc_cancels_question_once() {
    let mut sc = new_saves(&new_menus());
    let mut s = session(three_saves());
    sc.update(&press(KEYS1, &[View]), &mut s);
    // Esc liefert Back und Menu im selben Update: das zählt einmal (Rückfrage zu, Liste bleibt)
    let st = sc.update(&press(KEYS1, &[Back, Menu]), &mut s);
    assert!(
        st.kind == StepKind::Stay && !sc.asking,
        "Esc: {:?}, Rückfrage {}",
        st.kind,
        sc.asking
    );
    let st = sc.update(&press(KEYS1, &[Back, Menu]), &mut s);
    assert_eq!(st.kind, StepKind::Pop, "zweites Esc verlässt die Liste nicht");
}

#[test]
fn saves_delete_refreshes_and_keeps_selection_in_list() {
    let mut sc = new_saves(&new_menus());
    let mut s = session(three_saves());
    let ins: Vec<_> = [Down, Down, View, Right, Confirm]
        .iter()
        .map(|a| press(PAD1, &[*a]))
        .collect();
    step_through(&mut sc, &mut s, &ins);
    assert!(
        sc.list.len() == 2 && sc.sel == 1 && sc.base.note.key == TextKey::Deleted && sc.base.note.arg == "spiel-1",
        "Liste {}, Auswahl {}, Meldung {:?}",
        sc.list.len(),
        sc.sel,
        sc.base.note
    );
}

#[test]
fn saves_delete_error() {
    let mut sc = new_saves(&new_menus());
    let mut s = FakeSession {
        del_err: Some(err_fake()),
        ..session(three_saves())
    };
    let ins: Vec<_> = [View, Right, Confirm].iter().map(|a| press(PAD1, &[*a])).collect();
    let st = step_through(&mut sc, &mut s, &ins);
    assert!(
        has_cue(&st, CueKind::Denied) && sc.base.note.raw == err_fake().to_string() && sc.list.len() == 3,
        "Fehler: {:?}",
        sc.base.note
    );
}

#[test]
fn saves_empty_list_and_back() {
    let mut sc = new_saves(&new_menus());
    let mut s = FakeSession::default();
    let st = sc.update(&press(PAD1, &[View]), &mut s);
    assert!(
        st.kind == StepKind::Stay && !sc.asking,
        "Löschen in leerer Liste: {:?}",
        st.kind
    );
    sc.fill(DEFAULT_LANGUAGE);
    {
        let pg = sc.pg.borrow();
        assert!(
            pg.sub == "Keine Spielstände" && pg.n == 0,
            "leere Liste: {:?}, {} Zeilen",
            pg.sub,
            pg.n
        );
    }
    let st = sc.update(&press(PAD1, &[Back]), &mut s);
    assert!(
        st.kind == StepKind::Pop && has_cue(&st, CueKind::Back),
        "B: {:?}",
        st.kind
    );
}

#[test]
fn saves_list_error_is_shown() {
    let mut sc = new_saves(&new_menus());
    let mut s = FakeSession {
        list_err: Some(err_fake()),
        ..Default::default()
    };
    sc.update(&crate::view::input::Input::default(), &mut s);
    assert_eq!(sc.base.note.raw, err_fake().to_string(), "Meldung {:?}", sc.base.note);
}

#[test]
fn saves_rows_scroll_around_selection() {
    let mut sc = new_saves(&new_menus());
    let mut s = FakeSession::default();
    for i in 0..12u8 {
        s.saves
            .push(save(&format!("s{}", char::from(b'a' + i)), i64::from(i), true));
    }
    for _ in 0..9 {
        sc.update(&press(PAD1, &[Down]), &mut s);
    }
    sc.fill(DEFAULT_LANGUAGE);
    let pg = sc.pg.borrow();
    let label = &pg.rows[pg.sel as usize].label;
    assert!(
        pg.n == MAX_ROWS && label == "sj",
        "{} Zeilen, Auswahl {} = {label:?}",
        pg.n,
        pg.sel
    );
}

#[test]
fn save_row_text() {
    let e = SaveInfo {
        name: "spiel-1".to_string(),
        day: 4,
        depths: vec![0, 2],
        loadable: true,
        ..Default::default()
    };
    let r = save_row(DEFAULT_LANGUAGE, &e, 0);
    assert!(r.value == "Tag 4 · Stufe 0,2" && !r.off, "Zeile {r:?}");
    let r = save_row(LANGUAGE_ENGLISH, &e, 0);
    assert_eq!(r.value, "Day 4 · Level 0,2", "englisch");
}

/// Die Zeit der Zeile: feste Zeiten, Versatz 0 und +60 (Go: `Local().Format("02.01. 15:04")`).
#[test]
fn save_row_time_with_offset() {
    let t = UNIX_EPOCH + Duration::from_secs(1_735_826_640); // 02.01.2025 14:04 UTC
    assert_eq!(format_saved_at(t, 0), "02.01. 14:04");
    assert_eq!(format_saved_at(t, 60), "02.01. 15:04");
    assert_eq!(format_saved_at(t, -15), "02.01. 13:49");
    // Tageswechsel durch den Versatz, Schaltjahr und Jahresende
    let leap = UNIX_EPOCH + Duration::from_secs(1_709_164_800 + 23 * 3600 + 30 * 60); // 29.02.2024 23:30 UTC
    assert_eq!(format_saved_at(leap, 0), "29.02. 23:30");
    assert_eq!(format_saved_at(leap, 60), "01.03. 00:30");
    let eve = UNIX_EPOCH + Duration::from_secs(1_735_689_599); // 31.12.2024 23:59:59 UTC
    assert_eq!(format_saved_at(eve, 0), "31.12. 23:59");
    assert_eq!(format_saved_at(UNIX_EPOCH, 0), "01.01. 00:00");
    assert_eq!(format_saved_at(UNIX_EPOCH - Duration::from_secs(1), 0), "31.12. 23:59");
    let mut e = save("a", 1, true);
    e.saved_at = Some(t);
    assert_eq!(
        save_row(DEFAULT_LANGUAGE, &e, 60).value,
        "Tag 1 · Stufe 0 · 02.01. 15:04"
    );
}

/// Zwei Geräte: B und Esc gehen vom zweiten Gerät aus zurück; die Rückfrage fängt das erste B ab.
#[test]
fn saves_back_with_two_devices() {
    let mut sc = new_saves(&new_menus());
    let mut s = session(three_saves());
    let st = sc.update(&press(PAD2, &[Back]), &mut s);
    assert!(
        st.kind == StepKind::Pop && has_cue(&st, CueKind::Back),
        "B (Pad 2): {:?}",
        st.kind
    );
    let mut sc = new_saves(&new_menus());
    sc.update(&press(PAD1, &[View]), &mut s);
    assert!(sc.asking, "Rückfrage fehlt");
    let st = sc.update(&press(KEYS1, &[Back, Menu]), &mut s);
    assert!(
        st.kind == StepKind::Stay && !sc.asking,
        "Esc (Tastatur) schließt die Rückfrage"
    );
    let st = sc.update(&press(KEYS1, &[Back, Menu]), &mut s);
    assert_eq!(st.kind, StepKind::Pop, "Esc (Tastatur) verlässt die Liste");
}
