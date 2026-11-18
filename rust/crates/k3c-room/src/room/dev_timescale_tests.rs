//! Tests aus `dev_timescale_test.go` (B-178/AC-04, B-231): Zeitraffer und Dev-Pause. Der Zustand wird als JSON-Baum
//! verglichen (Welt je Stufe und Vorrat).
use std::collections::BTreeMap;

use k3c_sim::state::PlayerCommand;

use super::dev_tests::{act, dev, dev_room, isl_player};
use super::testutil::{TestManager, rm, ticks};
use super::{DevAction, MAX_TIMESCALE};
use crate::fehler::Fehler;

fn ts(f: i64) -> DevAction {
    DevAction {
        factor: f,
        ..act("timescale")
    }
}

fn cmd(move_x: f64) -> PlayerCommand {
    PlayerCommand {
        move_x,
        ..PlayerCommand::default()
    }
}

/// Ein Dev-Raum mit festen, verschiedenen Eingaben für beide Slots (Go `scaledRoom`).
fn scaled_room() -> (TestManager, String) {
    let (mut m, code, _i) = dev_room();
    let sprint = PlayerCommand {
        sprint: true,
        ..cmd(1.0)
    };
    let pay = PlayerCommand { pay: true, ..cmd(-1.0) };
    let input = BTreeMap::from([(0, sprint), (1, pay)]);
    assert_eq!(rm(&mut m, &code).input("a", 1, &input), Ok(()));
    (m, code)
}

/// Der Zustand der Insel als JSON-Baum: Welt je Stufe und Vorrat.
fn state(m: &mut TestManager, code: &str) -> serde_json::Value {
    let r = rm(m, code);
    serde_json::json!([
        serde_json::to_value(&r.isl.stages).expect("Stufen"),
        serde_json::to_value(&r.isl.stock).expect("Vorrat")
    ])
}

// B-178/AC-04: ein Tick mit Faktor 4 (2) ergibt denselben Zustand wie 4 (2) normale Ticks bei gleichem Seed und gleicher
// Eingabe; Faktor 1 ist ein normaler Tick. Mehrere Ticks hintereinander, damit Tage und Wellen mitlaufen.
#[test]
fn dev_timescale_gleich_normalen_ticks() {
    for factor in [1, 2, 4] {
        let (mut fast, fc) = scaled_room();
        assert_eq!(dev(&mut fast, &fc, &ts(factor)), Ok(()));
        let (mut slow, sc) = scaled_room();
        for _ in 0..30 {
            ticks(&mut fast, &fc, 1);
            ticks(&mut slow, &sc, factor as usize);
        }
        assert!(
            state(&mut fast, &fc) == state(&mut slow, &sc),
            "Faktor {factor}: Zustand weicht von {} normalen Ticks ab",
            30 * factor
        );
        let (ft, st) = (rm(&mut fast, &fc).tick, rm(&mut slow, &sc).tick);
        assert!(
            ft == 30 && st == 30 * factor,
            "Faktor {factor}: Tick-Zähler {ft} und {st}"
        );
    }
}

// Faktoren außerhalb 1, 2, 4, 8 ergeben bad_request, der Faktor bleibt.
#[test]
fn dev_timescale_ungueltig() {
    let (mut m, code, _i) = dev_room();
    assert_eq!(dev(&mut m, &code, &ts(MAX_TIMESCALE)), Ok(()));
    for f in [3, 0, -1, 9, 16] {
        assert_eq!(dev(&mut m, &code, &ts(f)), Err(Fehler::BadRequest), "Faktor {f}");
    }
    assert_eq!(rm(&mut m, &code).scale(), MAX_TIMESCALE, "Faktor");
}

// Verlässt das letzte Gerät den Raum (pausiert), endet der Zeitraffer, auch nach Verbindungsabbruch; Log nennt ihn.
#[test]
fn dev_timescale_endet_bei_pause() {
    for leave in [true, false] {
        let (mut m, code, _i) = dev_room();
        assert_eq!(dev(&mut m, &code, &ts(4)), Ok(()));
        if leave {
            m.with_room(&code, |r, env| r.leave(env, "a", 1));
        } else {
            m.with_room(&code, |r, env| r.drop_conn(env, "a", 1));
        }
        assert_eq!(rm(&mut m, &code).scale(), 1, "leave={leave}: Faktor nach Pause");
        assert!(
            m.log.lines.iter().any(|l| l.contains("Zeitraffer beendet")),
            "leave={leave}: Log ohne Ende des Zeitraffers:\n{:?}",
            m.log.lines
        );
    }
}

// Zwei Spieler laufen im Zeitraffer beide weiter, jeder nach seiner Eingabe; der Faktor steht im Log.
#[test]
fn dev_timescale_zwei_spieler_und_log() {
    let (mut m, code, _i) = dev_room();
    let input = BTreeMap::from([(0, cmd(1.0)), (1, cmd(-1.0))]);
    assert_eq!(rm(&mut m, &code).input("a", 1, &input), Ok(()));
    assert_eq!(dev(&mut m, &code, &ts(4)), Ok(()));
    let (x0, x1) = (isl_player(rm(&mut m, &code), 0).x, isl_player(rm(&mut m, &code), 1).x);
    ticks(&mut m, &code, 1);
    let r = rm(&mut m, &code);
    let (n0, n1) = (isl_player(r, 0).x, isl_player(r, 1).x);
    assert!(n0 > x0 && n1 < x1, "Spieler laufen nicht: {x0}->{n0}, {x1}->{n1}");
    let log = m.log.lines.join("\n");
    assert!(
        log.contains("Dev-Aktion") && log.contains("aktion=timescale") && log.contains("faktor=4"),
        "Log ohne Faktor:\n{log}"
    );
}

// B-231: In der Dev-Pause tickt der Raum ohne Schritt (Zustand bleibt, Tick-Zähler läuft); danach rechnet er weiter.
// `pause` ohne `paused` ist bad_request.
#[test]
fn dev_pause_haelt_raum_an() {
    let (mut m, code) = scaled_room();
    assert_eq!(
        dev(&mut m, &code, &act("pause")),
        Err(Fehler::BadRequest),
        "ohne paused"
    );
    let pause = |p: bool| DevAction {
        paused: Some(p),
        ..act("pause")
    };
    assert_eq!(dev(&mut m, &code, &pause(true)), Ok(()));
    let before = state(&mut m, &code);
    ticks(&mut m, &code, 10);
    assert!(
        state(&mut m, &code) == before && rm(&mut m, &code).tick == 10,
        "angehalten: Zustand geändert oder Tick {}",
        rm(&mut m, &code).tick
    );
    assert_eq!(dev(&mut m, &code, &pause(false)), Ok(()));
    ticks(&mut m, &code, 1);
    assert!(state(&mut m, &code) != before, "nach der Pause: Zustand unverändert");
}
