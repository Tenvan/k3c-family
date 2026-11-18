//! Tests des Takts der Session (Go: `local_test.go`, RU27.2): `TestDeterminismLikeRoom`, `TestStepWithoutGameAndPaused`,
//! `TestJoinLeave` mit Frames, `TestPanicInTick`, `TestAutosaveWithoutRun`. Die Simulation selbst (`step_island`) bricht bis
//! RU19 mit `unimplemented!` ab; Tests, die einen Simulationsschritt brauchen, sind darum `ignore`, die übrigen takten in der
//! Pause (kein Schritt) oder über die Naht `before_step`.

use super::session::Session;
use super::testutil::{TWO_DEVICES, TempDir, command_at, reference_worlds, start_test_game, test_session, worlds_json};
use super::watched::lock;
use crate::game::sim::Sim as _;
use crate::view::frame::{CueKind, Frame};
use crate::view::session::Session as _;

const DETERMINISM_TICKS: usize = 300;

/// Taktet `ticks` Takte mit den festen Befehlen und liefert den letzten Frame.
fn run_ticks(s: &mut Session, ticks: usize) -> Frame {
    let mut f = Frame::default();
    for i in 0..ticks {
        for p in 0..2 {
            s.command(p, command_at(p, i)).expect("Befehl");
        }
        f = s.step();
    }
    f
}

fn frame_json(f: &Frame) -> String {
    worlds_json(
        f.stages
            .iter()
            .filter_map(|st| st.world.as_deref().map(|w| (st.index, w))),
    )
}

// AC-01 (RU27.2): 2 Spieler, 300 Takte über local ergeben dieselben Welten wie ein direkt über k3c_room getakteter Raum mit
// denselben Befehlen, und zwei Sitzungen mit gleicher Eingabe ergeben byte-gleiche Zustände. (Go startet über `New`; das
// braucht bis RU19 die Standard-Welt, darum die Insel von Hand.)
#[test]
fn determinism_like_room() {
    let (d1, d2, d3) = (TempDir::new(), TempDir::new(), TempDir::new());
    let (mut a, mut b) = (test_session(&d1), test_session(&d2));
    start_test_game(&mut a, "wald-1", &TWO_DEVICES);
    start_test_game(&mut b, "wald-1", &TWO_DEVICES);
    let f = run_ticks(&mut a, DETERMINISM_TICKS);
    let want = worlds_json(
        reference_worlds(&d3, "wald-1", DETERMINISM_TICKS)
            .iter()
            .map(|(i, w)| (*i, w)),
    );
    assert_eq!(frame_json(&f), want, "Welten wie im Raum direkt");
    assert_eq!(
        frame_json(&f),
        frame_json(&run_ticks(&mut b, DETERMINISM_TICKS)),
        "zwei Sitzungen"
    );
    assert!(
        f.tick == DETERMINISM_TICKS as u64 && f.seats.len() == 2,
        "Frame: Takt {}, {:?}",
        f.tick,
        f.seats
    );
    assert!(f.monarch(0).is_some() && f.monarch(1).is_some());
    assert_eq!(f.seats[1].device, TWO_DEVICES[1]);
}

// Ohne Spiel liefert step einen leeren Frame mit Uhr.
#[test]
fn step_without_game() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    let f = s.step();
    assert!(
        !s.active() && f.stages.is_empty() && f.clock > 0.0,
        "Frame ohne Spiel: {f:?}"
    );
    assert!(f.timescale == 1 && f.campaign.run == 0 && f.route.is_empty() && f.step_ms == 0.0);
    assert!(
        (s.step().clock - 2.0 / 30.0).abs() < 1e-9,
        "die Uhr ist der Takt, nicht die Wanduhr"
    );
}

// In der Pause steht der Takt, der Frame trägt die Stufen aus dem Beitritt und die Uhr läuft weiter. (Go taktet vor der
// Pause einmal; ein Takt rechnet bis RU19 die Simulation nicht.)
#[test]
fn step_in_pause_keeps_tick() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    start_test_game(&mut s, "pause", &TWO_DEVICES[..1]);
    s.pause(true);
    let f = s.step();
    let tick = f.tick;
    assert!(
        f.paused && f.cues.len() == 1 && f.cues[0].kind == CueKind::Pause,
        "Pause-Frame: {f:?}"
    );
    assert!(
        f.seats.len() == 1 && f.stages.len() == 1 && f.monarch(0).is_some(),
        "Stufen und Plätze des Beitritts"
    );
    let mut f = f;
    for _ in 0..10 {
        f = s.step();
    }
    assert!(
        f.tick == tick && !f.stages.is_empty(),
        "Takt {} in der Pause, vorher {tick}",
        f.tick
    );
    assert!((f.clock - 11.0 / 30.0).abs() < 1e-9, "Uhr aus dem Takt: {}", f.clock);
    assert!(f.step_ms == 0.0, "kein Takt, keine Rechenzeit");
}

#[test]
fn step_resumes_after_pause() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    start_test_game(&mut s, "pause", &TWO_DEVICES[..1]);
    let tick = s.step().tick;
    s.pause(true);
    assert_eq!(s.step().tick, tick);
    s.pause(false);
    let f = s.step();
    assert!(
        f.tick == tick + 1 && !f.paused,
        "nach der Pause Takt {}, erwartet {}",
        f.tick,
        tick + 1
    );
}

// Halt (Entwickler-Taste, RU50.1 setzt `halt` und `single`): nur der Einzelschritt rechnet, nie auf Vorrat.
#[test]
fn running_halt_and_single() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert!(!s.running(), "ohne Spiel");
    start_test_game(&mut s, "halt", &TWO_DEVICES[..1]);
    assert!(s.running());
    s.halt = true;
    assert!(!s.running() && s.step().paused, "im Halt steht der Takt");
    s.single = true;
    assert!(s.running(), "Einzelschritt");
    assert!(!s.running(), "kein Einzelschritt auf Vorrat");
    s.single = true;
    s.paused = true;
    assert!(!s.running() && !s.single, "Pause verwirft den Einzelschritt");
}

// Spieler kommen und gehen, die Frames danach zeigen es (in der Pause, siehe `step_in_pause_keeps_tick`).
#[test]
fn join_leave_frames() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    start_test_game(&mut s, "couch", &TWO_DEVICES[..1]);
    s.pause(true);
    s.step();
    s.join(TWO_DEVICES[1]).expect("Join");
    let f = s.step();
    assert!(
        f.seats.len() == 2 && f.cues[0].kind == CueKind::Joined && f.monarch(1).is_some(),
        "nach Join: {:?}",
        f.seats
    );
    assert_eq!(f.seats[1].device, TWO_DEVICES[1]);
    s.leave(1).expect("Leave");
    let f = s.step();
    assert!(
        f.seats.len() == 1 && f.cues[0].kind == CueKind::Left,
        "nach Leave: {:?}",
        f.seats
    );
    let err = s.command(1, command_at(1, 0)).expect_err("gegangener Spieler");
    assert_eq!(err.to_string(), "Spieler 2 spielt nicht");
    s.command(0, command_at(0, 0)).expect("Befehl des ersten Spielers");
}

#[test]
fn command_without_game_does_nothing() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert!(s.command(0, command_at(0, 0)).is_ok());
    assert!(s.command(5, command_at(0, 0)).is_ok());
}

// Ein Panic im Takt schließt das Spiel ohne Speichern und meldet CueError mit dem Panic (`Manager::safe_tick`, B-803); der
// Raum ist danach weg. Die Naht `before_step` löst den Panic vor der Simulation aus. (Go prüft zusätzlich, dass der
// Spielstand unverändert bleibt und sich laden lässt: das braucht Speichern, RU18, und Laden, RU19.)
#[test]
fn panic_in_tick() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    start_test_game(&mut s, "absturz", &TWO_DEVICES[..1]);
    s.with_room(|r, _| r.before_step = Some(Box::new(|| panic!("kaputt"))));
    let f = s.step();
    let crashed = |c: &crate::view::frame::Cue| c.kind == CueKind::Error && c.text.ends_with("kaputt");
    assert!(
        !s.active() && f.cues.iter().any(crashed),
        "nach Panic aktiv {}, Cues {:?}",
        s.active(),
        f.cues
    );
    assert!(
        f.stages.is_empty() && f.seats.is_empty(),
        "kein Spiel im Frame nach dem Absturz"
    );
    assert!(!dir.path().join("absturz.json").exists(), "ohne Speichern");
    assert_eq!(s.step().tick, 0, "danach läuft der Takt ohne Spiel weiter");
}

// Ein Panic im Zustand des Geräts (Test-Naht `on_state`) im Takt wird ebenso gefangen.
#[test]
fn panic_in_state() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    start_test_game(&mut s, "absturz", &TWO_DEVICES[..1]);
    s.step();
    lock(s.peer.as_ref().expect("Peer")).on_state = Some(Box::new(|| panic!("kaputt")));
    let f = s.step();
    assert!(
        !s.active()
            && f.cues
                .iter()
                .any(|c| c.kind == CueKind::Error && c.text.ends_with("kaputt"))
    );
}

// Autosave greift im Takt (Raum ohne `Manager::run`): nach `SAVE_TICKS` Takten liegt der Spielstand im Ordner und ein Frame
// meldet CueSaved. Die Dev-Pause des Raums lässt die Takte ohne Simulationsschritt laufen (`step_island` bricht bis RU19
// ab); der Takt, das Speichern und die Cues laufen echt.
#[test]
fn autosave_without_run() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    start_test_game(&mut s, "auto", &TWO_DEVICES[..1]);
    s.with_room(|r, _| r.paused = true);
    let mut saved = false;
    for _ in 0..k3c_room::room::SAVE_TICKS {
        saved |= s.step().cues.iter().any(|c| c.kind == CueKind::Saved);
    }
    assert!(saved, "kein Autosave ohne Manager::run");
    assert!(dir.path().join("auto.json").is_file());
}
