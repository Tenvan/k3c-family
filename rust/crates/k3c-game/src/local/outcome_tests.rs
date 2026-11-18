//! Tests des Ausgangs und der Plätze (Go: `outcome_test.go`).

use std::rc::Rc;

use k3c_sim::state::World;

use super::config::Config;
use super::session::Session;
use super::testutil::{TWO_DEVICES, TempDir, fixed_now, start_test_game, test_session};
use crate::game::sim::Sim as _;
use crate::view::frame::{CueKind, Event, Outcome};
use crate::view::session::Session as _;
use crate::view::settings::default_settings;

fn event(typ: &str) -> Event {
    Event {
        data: serde_json::json!({ "type": typ })
            .as_object()
            .cloned()
            .unwrap_or_default(),
        ..Event::default()
    }
}

fn with_time(t: f64) -> Rc<World> {
    Rc::new(World {
        time: t,
        ..World::default()
    })
}

// Aufnahme-Modus: CueSaved liegt im Frame des Takts, in dem der Raum das Speichern beginnt, unabhängig davon, wie lange das
// Schreiben dauert; zwei Läufe melden es im selben Takt. (`k3c-room` speichert synchron im Takt, das Warten von Go entfällt.)
#[test]
fn recording_saved_cue_follows_tick() {
    let run = || {
        let dir = TempDir::new();
        let mut s = Session::open(Config {
            dir: dir.dir(),
            now: Some(Box::new(fixed_now)),
            recording: true,
            ..Config::default()
        });
        start_test_game(&mut s, "aufnahme", &TWO_DEVICES[..1]);
        s.with_room(|r, _| r.paused = true); // Takte ohne Simulationsschritt (`step_island` bricht bis RU19 ab)
        let mut at = Vec::new();
        for _ in 0..k3c_room::room::SAVE_TICKS + 5 {
            let f = s.step();
            if f.cues.iter().any(|c| c.kind == CueKind::Saved) {
                at.push(f.tick);
            }
        }
        at
    };
    let (a, b) = (run(), run());
    assert!(!a.is_empty() && a == b, "CueSaved in Takt {a:?} bzw. {b:?}");
    let want = k3c_room::room::SAVE_TICKS as u64;
    assert!(
        a.contains(&want),
        "Autosave nach SAVE_EVERY nicht im Takt {want}: {a:?}"
    );
}

// Ausgang: victory bzw. gameOver unter den Ereignissen, sonst verloren, sobald die Spielzeit einer Stufe trotz Takt steht
// (Insel ruht nach gameOver in einer Stufe ohne lokalen Spieler). Der erste Ausgang bleibt.
#[test]
fn judge_outcome() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    s.stages.worlds.insert(0, with_time(1.0));
    s.judge();
    s.stages.worlds.insert(0, with_time(1.5));
    s.judge();
    assert_eq!(s.ended, Outcome::None, "laufende Insel");
    s.judge();
    assert_eq!(s.ended, Outcome::Lost, "stehende Insel");

    let mut s = test_session(&dir);
    s.stages.worlds.insert(1, with_time(0.0));
    s.stages.events = vec![event("victory"), event("gameOver")];
    s.judge();
    assert_eq!(s.ended, Outcome::Won, "Sieg");
    s.stages.events.clear();
    s.judge();
    assert_eq!(s.ended, Outcome::Won, "Ausgang bleibt");
    // neue Welt derselben Stufe (Inselwechsel, das Level kam neu): kein Stillstand
    s.stages.worlds.insert(1, with_time(0.0));
    s.stages.replaced.push(1);
    assert!(!s.stages.stalled(), "neue Welt gilt als stehend");
    s.stages.replaced.clear();
    assert!(s.stages.stalled(), "dieselbe Welt mit gleicher Zeit steht");
}

#[test]
fn judge_game_over_event_loses() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    s.stages.worlds.insert(0, with_time(0.0));
    s.stages.events = vec![event("night"), event("gameOver")];
    s.judge();
    assert_eq!(s.ended, Outcome::Lost);
}

// seats liefert eine eigene Liste: Frame::seats (mit `lost` von game) bleibt unberührt.
#[test]
fn seats_own_buffer() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert!(s.seats().is_empty() && s.settings() == default_settings(), "ohne Spiel");
    start_test_game(&mut s, "plaetze", &TWO_DEVICES);
    s.pause(true);
    let mut f = s.step();
    f.seats[0].lost = true;
    let got = s.seats();
    assert!(
        got.len() == 2 && got[1].device == TWO_DEVICES[1] && !got[0].lost && f.seats[0].lost,
        "seats {got:?}, Frame {:?}",
        f.seats
    );
}
