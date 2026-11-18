//! Port von `native/k3c-game/hud/seekarte_test.go` (ohne `TestSeekarteOhneAllokationen`: `testing.AllocsPerRun` ist in
//! Rust nicht portierbar, das Frame-Budget misst RU39), dazu `sea_back_unbound` für B und Esc.

use super::seekarte::{SeaKind, SeaRow, sea_line};
use super::testutil::{HudSession, has_cue, hud, input, open_switch, sea_world, set_world};
use crate::view::frame::CueKind;
use crate::view::input::Action::{Back, Confirm, Left, Menu, Right};
use crate::view::roles::Overlay;
use crate::view::session::Fehler;
use crate::view::settings::DEFAULT_LANGUAGE;
use k3c_sim::state::{Crown, SwitchTarget};

fn kinds_of(r: &SeaRow) -> Vec<SeaKind> {
    r.nodes.iter().map(|n| n.kind).collect()
}

/// AC-01: Overlay nur bei offenem Wechselpunkt; Ziele, Abgeschlossene und Gesperrte nach Etappen.
#[test]
fn sea_only_with_open_switch() {
    let (mut w, mut f) = sea_world();
    let mut h = hud();
    h.listen(&f);
    assert!(
        !h.seats[0].sea.open && !h.seats[1].sea.open,
        "ohne Wechselpunkt keine Seekarte"
    );
    w.island_switch = Some(open_switch("b"));
    set_world(&mut f, &w);
    h.listen(&f);
    let v = &h.seats[0].sea;
    assert!(
        v.open && h.seats[1].sea.open,
        "offener Wechselpunkt: Seekarte in beiden Zellen"
    );
    assert!(
        v.rows.len() == 3 && v.rows[0].etappe == 1 && v.rows[2].etappe == 3,
        "drei Etappen untereinander: {:?}",
        v.rows
    );
    assert_eq!(kinds_of(&v.rows[0]), [SeaKind::Done]);
    assert_eq!(kinds_of(&v.rows[1]), [SeaKind::Target, SeaKind::Target]);
    assert_eq!(
        kinds_of(&v.rows[2]),
        [SeaKind::Locked],
        "a abgeschlossen, b und c Ziele, d gesperrt"
    );
    let b = &v.rows[1].nodes;
    assert!(
        b[0].chosen && !b[1].chosen && b[0].cursor,
        "Wahl b gilt, Cursor beginnt auf der Wahl"
    );
    w.island_switch = None;
    set_world(&mut f, &w);
    h.listen(&f);
    assert!(
        !h.seats[0].sea.open,
        "Wechselpunkt geschlossen (Schiff verloren): Seekarte weg"
    );
}

#[test]
fn sea_completed_and_one_target() {
    let (mut w, mut f) = sea_world();
    let mut sw = open_switch("d");
    sw.completed = ["a", "b", "c"].map(str::to_string).to_vec();
    sw.targets = vec![SwitchTarget {
        id: "d".to_string(),
        name: "D".to_string(),
        etappe: 3,
    }];
    w.island_switch = Some(sw);
    set_world(&mut f, &w);
    let mut h = hud();
    h.listen(&f);
    for r in &h.seats[0].sea.rows[..2] {
        for n in &r.nodes {
            assert_eq!(n.kind, SeaKind::Done, "{} abgeschlossen", n.name);
        }
    }
    let one = sea_line(DEFAULT_LANGUAGE, &h.seats[0].profile, false).plain;
    let v = &mut h.seats[0].sea;
    assert!(
        v.hint == one && !v.hint.contains('◄'),
        "ein Ziel: Hinweis ohne Pfeile, war {:?}",
        v.hint
    );
    let r = v.step(&[Right, Left]);
    assert!(!r.moved && r.pick.is_empty(), "ein Ziel: Links/Rechts bewegen nichts");
}

/// AC-02: Links/Rechts/A erzeugen die Wahl-Aktion, mit 2 Spielern; die Wahl eines Spielers ist für beide sichtbar.
#[test]
fn sea_input_two_players() {
    let (mut w, mut f) = sea_world();
    w.island_switch = Some(open_switch("b"));
    set_world(&mut f, &w);
    let mut h = hud();
    h.listen(&f);
    let mut s = HudSession::default();
    let st = h.update(&input(&[Right, Confirm], &[Confirm]), &f, &mut s);
    assert_eq!(
        s.picks,
        ["0:c", "1:b"],
        "Spieler 1 wählt c, Spieler 2 (eigener Cursor) b"
    );
    assert!(
        st.consumed[0]
            && st.consumed[1]
            && has_cue(&st.cues, CueKind::Move, 0)
            && has_cue(&st.cues, CueKind::Confirm, 1),
        "A wird verbraucht (kein Bezahlen), Cues: {:?} {:?}",
        st.consumed,
        st.cues
    );
    w.island_switch = Some(open_switch("c")); // die Simulation hat die Wahl übernommen: beide sehen c
    set_world(&mut f, &w);
    h.listen(&f);
    for p in 0..2 {
        assert!(
            h.seats[p].sea.rows[1].nodes[1].chosen && h.seats[p].sea.cursor == 1,
            "Spieler {} sieht Wahl c",
            p + 1
        );
    }
}

#[test]
fn sea_no_input_without_crown_or_at_harbor() {
    let (mut w, mut f) = sea_world();
    w.island_switch = Some(open_switch("b"));
    w.players[0].crown = Some(Crown::default()); // ohne Krone: keine Wahl
    w.players[1].x = 80.0; // fern vom Steg: Links/Rechts bewegen nur den Monarchen
    set_world(&mut f, &w);
    let mut h = hud();
    h.listen(&f);
    let mut s = HudSession::default();
    h.update(&input(&[Right, Confirm], &[Right, Confirm]), &f, &mut s);
    assert!(
        s.picks.is_empty() && h.seats[1].sea.cursor == 0,
        "keine Wahl, Cursor fest: {:?}",
        s.picks
    );
    w.players[1].x = 50.0;
    set_world(&mut f, &w);
    h.update(&input(&[], &[Back]), &f, &mut s);
    let st = h.update(&input(&[], &[Back]), &f, &mut s);
    assert!(!st.consumed[1] && s.picks.is_empty(), "B ist im Spiel unbelegt");
    s.choose_err = Some(Fehler::Session("insel: ohne Krone keine Wahl".to_string()));
    let st = h.update(&input(&[], &[Confirm]), &f, &mut s);
    assert!(has_cue(&st.cues, CueKind::Denied, 1), "abgelehnte Wahl: Cue");
}

/// Nur das Bestätigen wird verbraucht; Blättern und B lassen den Monarchen laufen.
#[test]
fn sea_consumes_only_confirm() {
    let (mut w, mut f) = sea_world();
    w.island_switch = Some(open_switch("b"));
    set_world(&mut f, &w);
    let mut h = hud();
    h.listen(&f);
    let mut s = HudSession::default();
    let st = h.update(&input(&[Right], &[Back]), &f, &mut s);
    assert!(!st.consumed[0] && !st.consumed[1], "Blättern und B verbrauchen nichts");
    s.choose_err = Some(Fehler::Session("abgelehnt".to_string()));
    let st = h.update(&input(&[Confirm], &[]), &f, &mut s);
    assert!(st.consumed[0], "auch abgelehntes Bestätigen verbraucht (kein Bezahlen)");
    w.players[0].x = 80.0;
    set_world(&mut f, &w);
    let st = h.update(&input(&[Confirm], &[]), &f, &mut s);
    assert!(!st.consumed[0], "fern vom Steg bleibt A Bezahlen");
}

/// B und Esc (Back und Menu) bleiben auch am Steg unbelegt: weder Cursor noch Eingabe noch Cue (Projektregel).
#[test]
fn sea_back_unbound() {
    let (mut w, mut f) = sea_world();
    w.island_switch = Some(open_switch("b"));
    set_world(&mut f, &w);
    let mut h = hud();
    h.listen(&f);
    let mut s = HudSession::default();
    let before = (h.seats[0].sea.cursor, h.seats[1].sea.cursor);
    let st = h.update(&input(&[Back, Menu], &[Back]), &f, &mut s);
    assert!(!st.consumed[0] && !st.consumed[1], "B und Esc verbrauchen nichts");
    assert!(
        st.cues.is_empty() && s.picks.is_empty(),
        "keine Cues, keine Wahl: {:?}",
        st.cues
    );
    assert_eq!((h.seats[0].sea.cursor, h.seats[1].sea.cursor), before, "Cursor fest");
    assert!(h.seats[0].sea.open, "die Karte bleibt offen");
}
