//! Tests aus `native/k3c-game/menu/lobby_test.go`, dazu B und Esc in der Lobby.

use super::lobby::{LOBBY_CANCEL, LOBBY_GRADE, LOBBY_ROWS, LOBBY_SEED, LOBBY_START, LOBBY_WORLD, MAX_SEED, new_lobby};
use super::testutil::{FakeSession, KEYS1, PAD1, PAD2, PAD3, err_fake, has_cue, new_menus, press, step_through};
use crate::view::frame::CueKind;
use crate::view::input::{Action, DeviceID, DeviceInput, Input};
use crate::view::screen::{Screen, StepKind};
use crate::view::session::NewGame;

fn lobby(d: DeviceID) -> super::lobby::LobbyScreen {
    new_lobby(&new_menus(), d)
}

#[test]
fn lobby_starts_with_one_player() {
    let mut sc = lobby(PAD1);
    let mut s = FakeSession::default();
    sc.row = LOBBY_START;
    let st = sc.update(&press(PAD1, &[Action::Confirm]), &mut s);
    assert!(st.kind == StepKind::Play && s.news.len() == 1, "Start: {:?}", st.kind);
    let g = &s.news[0];
    assert!(
        g.grade == "normal"
            && g.goal == "endboss"
            && g.defeat == "resources"
            && g.world == "standard"
            && g.seed.is_empty()
            && g.devices == [PAD1]
            && g.name.is_empty(),
        "NewGame {g:?}"
    );
}

/// Ein Fall von `lobby_join_and_leave`.
struct Case {
    name: &'static str,
    ins: Vec<Input>,
    devs: Vec<DeviceID>,
    kind: StepKind,
    cue: Option<CueKind>,
    noted: bool,
}
fn join_cases() -> Vec<Case> {
    use Action::{Back, Confirm, Down};
    let c = |name, ins, devs, kind, cue, noted| Case {
        name,
        ins,
        devs,
        kind,
        cue,
        noted,
    };
    vec![
        c(
            "zweites Pad tritt mit A bei",
            vec![press(PAD2, &[Confirm])],
            vec![PAD1, PAD2],
            StepKind::Stay,
            Some(CueKind::Joined),
            false,
        ),
        c(
            "Tastatur tritt bei",
            vec![press(KEYS1, &[Confirm])],
            vec![PAD1, KEYS1],
            StepKind::Stay,
            Some(CueKind::Joined),
            false,
        ),
        c(
            "drittes Gerät abgelehnt",
            vec![press(PAD2, &[Confirm]), press(PAD3, &[Confirm])],
            vec![PAD1, PAD2],
            StepKind::Stay,
            Some(CueKind::Denied),
            true,
        ),
        c(
            "Spieler 2 geht mit B",
            vec![press(PAD2, &[Confirm]), press(PAD2, &[Back])],
            vec![PAD1],
            StepKind::Stay,
            Some(CueKind::Left),
            false,
        ),
        c(
            "B von Spieler 1 bricht ab",
            vec![press(PAD2, &[Confirm]), press(PAD1, &[Back])],
            vec![PAD1, PAD2],
            StepKind::Pop,
            Some(CueKind::Back),
            false,
        ),
        c(
            "fremdes Gerät bedient nicht",
            vec![press(PAD2, &[Down]), press(PAD2, &[Back])],
            vec![PAD1],
            StepKind::Stay,
            None,
            false,
        ),
    ]
}

#[test]
fn lobby_join_and_leave() {
    for case in join_cases() {
        let mut sc = lobby(PAD1);
        let st = step_through(&mut sc, &mut FakeSession::default(), &case.ins);
        assert_eq!(&sc.devs[..sc.n], case.devs.as_slice(), "{}: Spieler", case.name);
        assert!(
            st.kind == case.kind && case.cue.is_none_or(|k| has_cue(&st, k)) && sc.base.note.set == case.noted,
            "{}: Schritt {:?}, Meldung {:?}",
            case.name,
            st.kind,
            sc.base.note
        );
        assert_eq!(sc.row, 0, "{}: Zeile bewegt", case.name);
    }
}

#[test]
fn lobby_starts_with_two_players_in_join_order() {
    let mut sc = lobby(PAD2);
    let mut s = FakeSession::default();
    let input = Input {
        devices: vec![
            DeviceInput {
                device: KEYS1,
                actions: vec![Action::Confirm],
                ..Default::default()
            },
            DeviceInput {
                device: PAD2,
                actions: vec![Action::Down],
                ..Default::default()
            },
        ],
    };
    sc.update(&input, &mut s);
    sc.row = LOBBY_START;
    let st = sc.update(&press(KEYS1, &[Action::Confirm]), &mut s);
    assert_eq!(st.kind, StepKind::Play, "Spieler 2 startet nicht");
    assert_eq!(s.news[0].devices, [PAD2, KEYS1], "Geräte");
}

#[test]
#[allow(clippy::too_many_lines)] // Tabelle der Fälle
fn lobby_options() {
    use Action::{Confirm, Down, Left, Right, Up};
    let g = |grade: &str, goal: &str, defeat: &str, seed: &str| NewGame {
        grade: grade.into(),
        goal: goal.into(),
        defeat: defeat.into(),
        seed: seed.into(),
        ..Default::default()
    };
    let cases = [
        (
            "Grad rechts bringt Standard-Niederlage",
            vec![Right],
            g("hard", "endboss", "stage", ""),
        ),
        (
            "Grad Ultra: Stufenverlust",
            vec![Right, Right],
            g("ultra", "endboss", "stage", ""),
        ),
        (
            "Grad links: Leicht",
            vec![Left, Left],
            g("easy", "endboss", "resources", ""),
        ),
        (
            "A im Kreis",
            vec![Confirm, Confirm, Confirm],
            g("easy", "endboss", "resources", ""),
        ),
        (
            "gewählte Niederlage bleibt beim Grad",
            vec![Down, Down, Left, Up, Up, Right],
            g("hard", "endboss", "resources", ""),
        ),
        ("Ziel", vec![Down, Right, Right], g("normal", "days", "resources", "")),
        (
            "Welt bleibt Standard",
            vec![Down, Down, Down, Right],
            g("normal", "endboss", "resources", ""),
        ),
        (
            "Seed 2",
            vec![Down, Down, Down, Down, Right, Right],
            g("normal", "endboss", "resources", "2"),
        ),
        (
            "Seed links bleibt Name",
            vec![Down, Down, Down, Down, Left],
            g("normal", "endboss", "resources", ""),
        ),
    ];
    for (name, keys, want) in cases {
        let mut sc = lobby(PAD1);
        let mut s = FakeSession::default();
        for a in keys {
            sc.update(&press(PAD1, &[a]), &mut s);
        }
        let got = sc.new_game();
        assert!(
            got.grade == want.grade
                && got.goal == want.goal
                && got.defeat == want.defeat
                && got.seed == want.seed
                && got.world == "standard",
            "{name}: NewGame {got:?}, erwartet {want:?}"
        );
    }
}

#[test]
fn lobby_seed_wraps_with_confirm() {
    let mut sc = lobby(PAD1);
    sc.row = LOBBY_SEED;
    sc.seed = MAX_SEED;
    sc.update(&press(PAD1, &[Action::Confirm]), &mut FakeSession::default());
    assert!(
        sc.seed == 0 && sc.new_game().seed.is_empty(),
        "Seed {} nach A am Ende",
        sc.seed
    );
}

#[test]
fn lobby_rows_and_cancel() {
    let mut sc = lobby(PAD1);
    let mut s = FakeSession::default();
    for _ in 0..LOBBY_ROWS + 2 {
        sc.update(&press(PAD1, &[Action::Down]), &mut s);
    }
    assert_eq!(sc.row, LOBBY_CANCEL, "Zeile am unteren Rand");
    assert_eq!(
        sc.update(&press(PAD1, &[Action::Right]), &mut s).kind,
        StepKind::Stay,
        "Rechts auf Abbrechen"
    );
    let st = sc.update(&press(PAD1, &[Action::Confirm]), &mut s);
    assert!(
        st.kind == StepKind::Pop && s.news.is_empty(),
        "Abbrechen: {:?}",
        st.kind
    );
    let st = lobby(PAD1).update(&press(PAD1, &[Action::Home]), &mut s);
    assert_eq!(st.kind, StepKind::Pop, "Home");
}

/// B am Pad und Esc (Back und Menu zugleich) an der Tastatur liefern denselben Schritt: Spieler 1 geht zurück
/// (`Pop`), Spieler 2 tritt aus und bleibt in der Lobby.
#[test]
fn lobby_back_and_esc_do_the_same() {
    let esc = |d: DeviceID| Input {
        devices: vec![DeviceInput {
            device: d,
            actions: vec![Action::Back, Action::Menu],
            ..Default::default()
        }],
    };
    for (d, input) in [(PAD1, press(PAD1, &[Action::Back])), (KEYS1, esc(KEYS1))] {
        let mut sc = lobby(d);
        let st = sc.update(&input, &mut FakeSession::default());
        assert!(
            st.kind == StepKind::Pop && has_cue(&st, CueKind::Back),
            "Spieler 1 {d}: {:?}",
            st.kind
        );
    }
    for (d, input) in [(PAD2, press(PAD2, &[Action::Back])), (KEYS1, esc(KEYS1))] {
        let mut sc = lobby(PAD1);
        let mut s = FakeSession::default();
        sc.update(&press(d, &[Action::Confirm]), &mut s);
        let st = sc.update(&input, &mut s);
        assert!(
            st.kind == StepKind::Stay && has_cue(&st, CueKind::Left) && sc.n == 1,
            "Spieler 2 {d}: {:?}",
            st.kind
        );
    }
}

#[test]
fn lobby_start_error_stays() {
    let mut sc = lobby(PAD1);
    let mut s = FakeSession {
        new_err: Some(err_fake()),
        ..Default::default()
    };
    sc.row = LOBBY_START;
    let st = sc.update(&press(PAD1, &[Action::Confirm]), &mut s);
    assert!(
        st.kind == StepKind::Stay && has_cue(&st, CueKind::Denied) && sc.base.note.raw == err_fake().to_string(),
        "Fehler: {:?}, {:?}",
        st.kind,
        sc.base.note
    );
}

#[test]
fn lobby_texts_show_players() {
    let mut sc = lobby(PAD1);
    sc.update(&press(KEYS1, &[Action::Confirm]), &mut FakeSession::default());
    sc.fill("de");
    let want = "Spieler 1: Pad 1   ·   Spieler 2: Tastatur";
    assert_eq!(sc.pg.borrow().sub, want, "Spieler-Zeile");
    assert_eq!(sc.pg.borrow().rows[LOBBY_GRADE].value, "< Normal >", "Grad");
    sc.fill("en");
    assert_eq!(
        sc.pg.borrow().rows[LOBBY_SEED].value,
        "< Name of the save >",
        "Seed englisch"
    );
    assert_eq!(sc.pg.borrow().rows[LOBBY_WORLD].value, "< standard >");
}
