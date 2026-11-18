//! Tests von Laden und Puffern des Takts (Go: `load_test.go`).

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use k3c_room::room::{NullLog, Options};
use k3c_room::store::Saves;
use k3c_sim::state::World;

use super::session::{CONN, Session};
use super::testutil::{
    RefPeer, TWO_DEVICES, TempDir, command_at, fixed_now, start_test_game, test_session, worlds_json,
};
use super::worlds::game_data;
use crate::game::sim::Sim as _;
use crate::view::frame::Frame;
use crate::view::session::{NewGame, Session as _};

const LOAD_TICKS: usize = 120;

/// Lädt den Spielstand direkt über `k3c_room` und taktet ihn `n` Mal mit `command_at` (Go: `roomLoadTicked`).
fn room_load_ticked(dir: &TempDir, name: &str, n: usize) -> BTreeMap<i64, World> {
    let mut m = k3c_room::room::Manager::new(
        Saves::new(dir.path().to_path_buf(), fixed_now),
        NullLog,
        fixed_now,
        "ref",
        game_data().expect("Spieldaten"),
    );
    let states = Rc::new(RefCell::new(BTreeMap::new()));
    m.create(
        "ref",
        Box::new(RefPeer(Rc::clone(&states))),
        name,
        false,
        0,
        &[0, 1],
        Options::default(),
    )
    .expect("Raum laden");
    for i in 0..n {
        let input: BTreeMap<i64, _> = (0..2).map(|p| (p, command_at(p, i).sim())).collect();
        let code = m.rooms.keys().next().cloned().expect("Raum");
        let res = m.with_room(&code, |r, _| r.input("ref", CONN, &input));
        assert!(matches!(res, Some(Ok(()))), "Eingabe: {res:?}");
        assert!(m.safe_tick(&code), "Raum läuft");
    }
    states.take()
}

// Ein geladener Stand läuft über local mit denselben Befehlen genauso weiter wie ein direkt über k3c_room geladener Raum
// (Welten nach 120 Takten).
#[test]
fn load_determinism_like_room() {
    let dir = TempDir::new();
    let mut s: Session = test_session(&dir);
    s.new_game(NewGame {
        name: "spiel-1".into(),
        devices: TWO_DEVICES.to_vec(),
        ..NewGame::default()
    })
    .expect("neues Spiel");
    s.save().expect("speichern");
    s.close().expect("beenden");
    s.load("spiel-1", &TWO_DEVICES).expect("laden");
    let mut f = Frame::default();
    for i in 0..LOAD_TICKS {
        for p in 0..2 {
            s.command(p, command_at(p, i)).expect("Befehl");
        }
        f = s.step();
    }
    let got = worlds_json(
        f.stages
            .iter()
            .filter_map(|st| st.world.as_deref().map(|w| (st.index, w))),
    );
    let want = room_load_ticked(&dir, "spiel-1", LOAD_TICKS);
    assert_eq!(
        got,
        worlds_json(want.iter().map(|(i, w)| (*i, w))),
        "Welten nach Laden und {LOAD_TICKS} Takten"
    );
}

// Die Puffer von step (Plätze, Stufen, Ereignisse) behalten ihre Kapazität: Es entsteht keine neue Liste je Takt. Eine
// Zuteilung durch die Welt selbst und die Listen des Frames (er besitzt sie) sind davon ausgenommen. Ohne Takt (Pause) wird
// die Welt nicht kopiert: Der Frame teilt dieselbe `Rc`.
#[test]
fn step_buffers_do_not_allocate() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    start_test_game(&mut s, "puffer", &TWO_DEVICES);
    s.pause(true);
    let first = s.step();
    s.step();
    let (seat_cap, stage_cap) = (s.seat_buf.capacity(), s.stages.stage_capacity());
    assert!(seat_cap >= 2 && stage_cap >= 1, "Puffer sind gefüllt");
    let mut last = first.clone();
    for _ in 0..20 {
        s.command(0, command_at(0, 0)).expect("Befehl");
        last = s.step();
    }
    assert_eq!(s.seat_buf.capacity(), seat_cap, "seat_buf wiederverwendet");
    assert_eq!(s.stages.stage_capacity(), stage_cap, "Stufen-Puffer wiederverwendet");
    let (a, b) = (first.stages[0].world.as_ref(), last.stages[0].world.as_ref());
    assert!(
        matches!((a, b), (Some(a), Some(b)) if Rc::ptr_eq(a, b)),
        "ohne Takt keine Kopie der Welt"
    );
}

// Die Ereignisse eines Takts liegen im Frame genau einmal; der Puffer der Session behält seine Kapazität.
#[test]
fn events_buffer_keeps_capacity() {
    let mut st = super::peer::StageState::default();
    let mut w = World::default();
    w.events.push(
        serde_json::json!({"type": "night"})
            .as_object()
            .cloned()
            .unwrap_or_default(),
    );
    w.events.push(
        serde_json::json!({"type": "saved"})
            .as_object()
            .cloned()
            .unwrap_or_default(),
    );
    st.absorb(7, 0, &w);
    assert_eq!(st.events.len(), 1, "saved ist kein Ereignis der Simulation");
    assert!(st.events[0].stage == 0 && st.events[0].tick == 7 && st.events[0].type_() == "night");
    let cap = st.events.capacity();
    let mut out = Vec::new();
    out.append(&mut st.events);
    assert_eq!(out.len(), 1);
    assert_eq!(st.events.capacity(), cap);
    st.begin();
    assert!(st.worlds.is_empty(), "begin leert die Stufen");
}
