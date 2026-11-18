//! Port von `native/k3c-game/game/voyage_test.go`: Beginn, Ende, Laden, Eingabe und Rückfall der Schiffsfahrt.

use k3c_platform::{InputState, NullCanvas};

use super::*;
use crate::game::Mode;
use crate::game::levels::levels_tests::{AssetsOf, FakeLoader, stages};
use crate::game::play_testutil::press;
use crate::game::testutil::{keys1, pad};
use crate::game::voyage_testutil::{VoyDrawer, harbor, stuck, switched, voy_game, voy_game_with};
use crate::view::frame::Frame;
use crate::view::input::{Action, Command, Input};
use crate::view::roles::AssetSource;

fn raw() -> InputState {
    InputState::new()
}

/// Taktet `n` Mal.
fn advance(g: &mut Game, n: usize) {
    let r = raw();
    for _ in 0..n {
        g.advance(&r);
    }
}

fn strs(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| (*s).to_string()).collect()
}

fn travel_ticks() -> usize {
    (voyage_seconds() * TPS as f64) as usize
}

fn with_art(p: &mut crate::game::Parts) {
    p.art = Some(Box::new(FakeLoader::default()));
    p.assets = vec![Box::new(AssetsOf::of(&[("wald", &["a/b"]), ("mine", &["c/d"])])) as Box<dyn AssetSource>];
}

/// B-977/AC-01: Das Ereignis `islandSwitch` beginnt die Fahrt von der aktuellen Insel (letzte der Abgeschlossenen) zum
/// Ziel des Wechsels; nennt das Ereignis keins, gilt die Wahl, ohne Wahl das erste Ziel der Liste.
#[test]
fn voyage_starts_on_island_switch() {
    let cases = [
        ("Ziel des Ereignisses", "c", "c", "c"),
        ("Wahl ohne Ziel im Ereignis", "c", "", "c"),
        ("ohne Wahl das erste Ziel", "", "", "b"),
        ("ohne Wahl, Ereignis nennt das erste", "", "b", "b"),
    ];
    for (name, choice, target, want) in cases {
        let d = VoyDrawer::new(true);
        let (mut g, _, _) = voy_game(Some(&d), vec![harbor(choice), switched(target)]);
        advance(&mut g, 1);
        assert!(!g.voy.on && d.asked() == 0, "{name}: Fahrt am Hafen ohne Wechsel");
        advance(&mut g, 1);
        let v = d.last();
        assert!(
            g.voy.on
                && v.from == "a"
                && v.to == want
                && v.done == strs(&["z", "a"])
                && v.open == strs(&["b", "c"])
                && v.duration == voyage_seconds(),
            "{name}: Fahrt an {}, {v:?}",
            g.voy.on
        );
    }
}

/// B-977/AC-02, Laden schneller: Die Fahrt endet genau nach der Fahrzeit; solange taktet die Simulation nicht, und die
/// Uhr des Frames steht (Banner laufen nicht hinter der Karte ab); danach läuft sie ohne Sprung zurück weiter.
#[test]
fn voyage_ends_after_travel_time() {
    let d = VoyDrawer::new(true);
    let (mut g, p, _) = voy_game(Some(&d), vec![harbor("b"), switched("b")]);
    advance(&mut g, 2);
    let (steps, clock) = (p.steps.get(), g.frame.clock);
    for i in 1..travel_ticks() {
        advance(&mut g, 1);
        assert!(
            g.voy.on && p.steps.get() == steps && g.frame.clock == clock && g.frame.events.is_empty(),
            "Takt {i}: Fahrt {}, Takte der Simulation {}, Uhr {}, Ereignisse {}",
            g.voy.on,
            p.steps.get() - steps,
            g.frame.clock,
            g.frame.events.len()
        );
    }
    advance(&mut g, 1);
    assert!(
        !g.voy.on && p.steps.get() == steps,
        "nach der Fahrzeit: Fahrt {}",
        g.voy.on
    );
    advance(&mut g, 1);
    assert!(
        p.steps.get() == steps + 1 && g.frame.clock > clock,
        "nach der Fahrt: Takte {}, Uhr {} nach {clock}",
        p.steps.get() - steps,
        g.frame.clock
    );
}

/// B-977/AC-02, Laden langsamer: Nach der Fahrzeit wartet die Fahrt, bis die Grafik der Zielinsel geladen ist.
#[test]
fn voyage_waits_for_loading() {
    let d = VoyDrawer::new(true);
    let (mut g, p, _) = voy_game_with(Some(&d), vec![harbor("b"), harbor("b"), switched("b")], with_art);
    advance(&mut g, 1); // Hafen: lädt „wald“ nach dem ersten Bild
    g.load.shown = true;
    advance(&mut g, 2); // Inselwechsel: Fahrt beginnt, „mine“ wartet auf ein Bild
    assert!(
        g.voy.on && g.loading(),
        "Wechsel: Fahrt {}, lädt {}",
        g.voy.on,
        g.loading()
    );
    let steps = p.steps.get();
    advance(&mut g, travel_ticks() + 30); // kein Bild gezeichnet: es lädt noch
    assert!(
        g.voy.on && g.loading() && p.steps.get() == steps,
        "lädt noch: Fahrt {}, lädt {}, Takte der Simulation {}",
        g.voy.on,
        g.loading(),
        p.steps.get() - steps
    );
    g.draw_voyage(&mut NullCanvas::default(), 0.0); // das Bild der Fahrt gibt das Laden frei (wie die Ladeanzeige)
    advance(&mut g, 1);
    assert!(
        !g.voy.on && !g.loading() && g.load.levels == strs(&["mine"]) && p.steps.get() == steps,
        "geladen: Fahrt {}, lädt {}, Level {:?}, Takte {}",
        g.voy.on,
        g.loading(),
        g.load.levels,
        p.steps.get() - steps
    );
}

/// B-977/AC-02 (R4): Während der Fahrt löst keine Spiel-Eingabe etwas aus: kein Befehl, keine Pause, kein Beitritt. Nur
/// der Vollbild-Wechsel (kein Spielzug) wirkt weiter.
#[test]
fn voyage_ignores_input() {
    let d = VoyDrawer::new(true);
    let (mut g, p, src) = voy_game(Some(&d), vec![harbor("b"), switched("b")]);
    advance(&mut g, 2);
    g.frame.settings.language = "de".to_string(); // toggle_window wertet nur mit Einstellungen aus
    p.cmds.borrow_mut().clear();
    let go = Command {
        move_: 1.0,
        ..Command::default()
    };
    *src.0.borrow_mut() = Input {
        devices: vec![
            press(
                keys1(),
                go,
                &[Action::Menu, Action::Home, Action::Confirm, Action::Back],
            ),
            press(pad(3), Command::default(), &[Action::Confirm]),
        ],
    };
    advance(&mut g, 10);
    assert!(
        g.voy.on
            && g.mode == Mode::Play
            && g.stack.is_empty()
            && !p.paused.get()
            && p.cmds.borrow().is_empty()
            && p.joins.borrow().is_empty()
            && p.set.borrow().is_empty(),
        "Eingabe wirkt: Fahrt {}, Modus {:?}, Screens {}, Pause {}, Befehle {:?}, Beitritte {:?}, Einstellungen {}",
        g.voy.on,
        g.mode,
        g.stack.len(),
        p.paused.get(),
        p.cmds.borrow(),
        p.joins.borrow(),
        p.set.borrow().len()
    );
    *src.0.borrow_mut() = Input {
        devices: vec![press(keys1(), Command::default(), &[Action::Fullscreen])],
    };
    advance(&mut g, 1);
    assert!(
        p.set.borrow().len() == 1 && g.voy.on,
        "Vollbild-Wechsel in der Fahrt: Einstellungen {}, Fahrt {}",
        p.set.borrow().len(),
        g.voy.on
    );
}

/// Die Tabelle von `TestVoyageFallsBackToLoadScreen`: ohne Zeichner, ohne Assets, neues Spiel, nicht getauscht, ohne
/// Wechselpunkt davor.
fn fallback_cases() -> Vec<(&'static str, Option<VoyDrawer>, Vec<Frame>)> {
    let wald = || Frame {
        stages: stages(&["wald"]),
        ..Frame::default()
    };
    vec![
        ("ohne Zeichner", None, vec![harbor("b"), harbor("b"), switched("b")]),
        (
            "ohne Assets",
            Some(VoyDrawer::new(false)),
            vec![harbor("b"), harbor("b"), switched("b")],
        ),
        (
            "neues Spiel oder Spielstand",
            Some(VoyDrawer::new(true)),
            vec![
                Frame::default(),
                Frame::default(),
                Frame {
                    stages: stages(&["mine"]),
                    ..Frame::default()
                },
            ],
        ),
        (
            "Insel nicht getauscht (Fehler im Raum)",
            Some(VoyDrawer::new(true)),
            vec![harbor("b"), harbor("b"), stuck("b")],
        ),
        (
            "ohne Wechselpunkt davor",
            Some(VoyDrawer::new(true)),
            vec![wald(), wald(), switched("b")],
        ),
    ]
}

/// B-977/AC-03: Ohne Zeichner, ohne Assets (Zeichner nicht bereit), im Aufnahme- und im Messmodus bleibt beim
/// Inselwechsel der Ladebildschirm; neues Spiel und Laden eines Spielstands (Stufen ohne `islandSwitch`) zeigen ihn
/// ohnehin.
#[test]
fn voyage_falls_back_to_load_screen() {
    for (name, d, frames) in fallback_cases() {
        let stay = frames[2].stages[0].level == "wald";
        let (mut g, _, _) = voy_game_with(d.as_ref(), frames, with_art);
        advance(&mut g, 1);
        g.load.shown = true;
        advance(&mut g, 2);
        assert!(
            !g.voy.on && g.loading() != stay,
            "{name}: Fahrt {}, Ladeanzeige {}",
            g.voy.on,
            g.loading()
        );
    }
    for mode in ["Aufnahme", "Messmodus"] {
        let d = VoyDrawer::new(true);
        let (mut g, _, _) = voy_game(Some(&d), vec![harbor("b"), switched("b")]);
        if mode == "Aufnahme" {
            g.args.shots = "x".to_string();
        } else {
            g.args.measure.seconds = 5.0;
        }
        advance(&mut g, 2);
        assert!(!g.voy.on, "{mode}: Fahrt beginnt");
    }
}

/// Die Fahrt läuft mit dem `alpha` aus `draw`, nicht aus der Wanduhr: `elapsed` ist Takte seit dem Ablegen minus eins
/// plus `alpha`, durch `TPS`, und nie negativ.
#[test]
fn voyage_draws_with_alpha_of_draw() {
    let d = VoyDrawer::new(true);
    let (mut g, _, _) = voy_game(Some(&d), vec![harbor("b"), switched("b")]);
    advance(&mut g, 2);
    assert_eq!(g.voy.ticks, 0);
    let mut c = NullCanvas::default();
    g.draw_voyage(&mut c, 0.5);
    assert_eq!(d.last().elapsed, 0.0, "vor dem ersten Takt der Fahrt nie negativ");
    advance(&mut g, 31);
    for alpha in [0.0, 0.25, 0.75] {
        g.draw_voyage(&mut c, alpha);
        let want = (g.voy.ticks as f64 - 1.0 + alpha) / TPS as f64;
        assert_eq!(d.last().elapsed, want, "alpha {alpha}");
    }
    assert!(d.seen.borrow().drawn >= 4, "der Zeichner wurde gerufen");
    assert!(g.load.shown, "ein Bild war zu sehen");
}

/// Zeichnet der Zeichner nichts, erscheint die leere Fläche bzw. der Ladebildschirm (kein Absturz ohne Zeichner).
#[test]
fn voyage_without_drawer_clears() {
    let (mut g, _, _) = voy_game(None, vec![harbor("b"), switched("b")]);
    advance(&mut g, 2);
    g.voy.on = true;
    let c = &mut NullCanvas::default();
    g.draw_voyage(c, 0.0);
    assert!(g.load.shown);
}

/// B-977/AC-04: Ein Takt der Fahrt legt nichts an. Die Zahl der Allokationen misst `tests/alloc_je_bild.rs` (eigener
/// Zähler); hier die Puffer: Ereignis- und Cue-Liste behalten ihre Kapazität, die Fahrt bleibt an.
#[test]
fn voyage_tick_keeps_buffers() {
    let d = VoyDrawer::new(true);
    let (mut g, _, _) = voy_game(Some(&d), vec![harbor("b"), switched("b")]);
    advance(&mut g, 2);
    g.voy.v.duration = 1e9; // die Messung bleibt in der Fahrt
    advance(&mut g, 5);
    let (ev, cues) = (g.frame.events.capacity(), g.frame.cues.capacity());
    let r = Input::default();
    for _ in 0..50 {
        g.sail(&r);
        g.arrive();
    }
    assert!(g.voy.on, "Fahrt bleibt");
    assert!(
        g.frame.events.capacity() == ev && g.frame.cues.capacity() == cues,
        "Puffer wachsen"
    );
}
