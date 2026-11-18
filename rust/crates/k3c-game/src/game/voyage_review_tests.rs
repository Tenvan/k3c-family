//! Port von `native/k3c-game/game/voyage_review_test.go`: Fahrt aus tieferer Stufe, Ladefehler und verlorenes Pad.

use k3c_platform::{InputState, NullCanvas};

use super::*;
use crate::game::levels::levels_tests::{AssetsOf, FakeLoader};
use crate::game::testutil::{keys1, pad};
use crate::game::voyage_testutil::{VoyDrawer, deep_harbor, harbor, switched, voy_game, voy_game_with};
use crate::view::frame::{Cue, CueKind, Frame, Seat};
use crate::view::input::{Action, DeviceID, DeviceInput, Input};
use crate::view::roles::AssetSource;

fn advance(g: &mut Game, n: usize) {
    let r = InputState::new();
    for _ in 0..n {
        g.advance(&r);
    }
}

fn travel_ticks() -> usize {
    (voyage_seconds() * TPS as f64) as usize
}

/// GS2.4: Liegt der Hafen nicht in Stufe 0, trägt der Frame des Wechsels neben der neuen Insel (Spieler in Stufe 0) noch
/// den letzten Zustand der verlassenen Stufe 2 der alten Insel samt Wechselpunkt (der Raum schickt erst die alte, dann
/// die neue Insel). Die Fahrt beginnt trotzdem; nur ein Spieler, der weiter am Wechselpunkt steht, heißt „nicht
/// getauscht“.
#[test]
fn voyage_starts_from_deeper_stage() {
    let mut sw = switched("b");
    sw.stages.push(deep_harbor().stages[1].clone()); // veraltete Stufe 2 der alten Insel
    let d = VoyDrawer::new(true);
    let (mut g, _, _) = voy_game(Some(&d), vec![deep_harbor(), sw]);
    advance(&mut g, 2);
    let v = d.last();
    assert!(
        g.voy.on && v.from == "a" && v.to == "b",
        "Wechsel aus Stufe 2: Fahrt {} von {:?} nach {:?}",
        g.voy.on,
        v.from,
        v.to
    );
    let stay = Frame {
        events: switched("b").events,
        ..deep_harbor() // Fehler im Raum: der Spieler steht weiter in Stufe 2 am Wechselpunkt
    };
    let (mut g, _, _) = voy_game(Some(&VoyDrawer::new(true)), vec![deep_harbor(), stay]);
    advance(&mut g, 2);
    assert!(!g.voy.on, "Insel nicht getauscht (Spieler in Stufe 2): Fahrt beginnt");
}

/// Spiel in der Fahrt zur Insel `mine`, der Loader hat die Insel noch nicht geladen; liefert Spiel, Loader und Probe.
fn loading_voyage() -> (Game, FakeLoader) {
    let ld = FakeLoader::default();
    let l2 = ld.clone();
    let d = VoyDrawer::new(true);
    let (mut g, _, _) = voy_game_with(Some(&d), vec![harbor("b"), harbor("b"), switched("b")], move |p| {
        p.art = Some(Box::new(l2));
        p.assets = vec![Box::new(AssetsOf::of(&[("wald", &["a/b"]), ("mine", &["c/d"])])) as Box<dyn AssetSource>];
    });
    advance(&mut g, 1);
    g.load.shown = true;
    advance(&mut g, 2);
    assert!(
        g.voy.on && g.loading(),
        "Wechsel: Fahrt {}, lädt {}",
        g.voy.on,
        g.loading()
    );
    (g, ld)
}

/// GS2.4: Schlägt das Laden der Zielinsel während der Fahrt fehl, meldet die Schleife den Fehler im Fenster (Draw zeigt
/// ihn vor der Fahrt); nichts rechnet weiter, die Fahrt hängt nicht in der Ladeanzeige.
#[test]
fn voyage_load_failure_is_fault() {
    let (mut g, ld) = loading_voyage();
    ld.fail.set(true);
    g.draw_voyage(&mut NullCanvas::default(), 0.0); // ein Bild der Fahrt gibt das Laden frei
    advance(&mut g, 1);
    g.run_pending_load(&mut NullCanvas::default()); // das Hochladen scheitert im Zeichen-Takt
    assert!(
        g.fault.is_some() && g.fault().is_some() && !g.loading(),
        "Ladefehler in der Fahrt: lädt {}",
        g.loading()
    );
    let steps = ld.calls.borrow().len();
    advance(&mut g, travel_ticks() + 5);
    assert_eq!(
        ld.calls.borrow().len(),
        steps,
        "nach dem Fehler wird nicht weiter geladen"
    );
}

/// Eingabe aller Geräte `ds`; die in `confirm` drücken Bestätigen.
fn devices_in(ds: &[DeviceID], confirm: &[DeviceID]) -> Input {
    Input {
        devices: ds
            .iter()
            .map(|&d| DeviceInput {
                device: d,
                actions: if confirm.contains(&d) {
                    vec![Action::Confirm]
                } else {
                    Vec::new()
                },
                ..DeviceInput::default()
            })
            .collect(),
    }
}

/// Zählt die Meldungen „Pad 3 von Spieler 1 getrennt“.
fn lost_cues(cues: &[Cue], pad3: DeviceID) -> usize {
    cues.iter()
        .filter(|c| c.kind == CueKind::DeviceLost && c.player == 0 && c.device == pad3)
        .count()
}

/// GS2.4: Verliert ein Spieler während der Fahrt sein Pad, meldet die Schleife es einmal (Cue, `Seat::lost`), die Fahrt
/// läuft ungestört zu Ende, und kein Gerät übernimmt den Platz, solange sie läuft (auch nicht mit Bestätigen). Erst der
/// erste Takt auf der Zielinsel ordnet neu zu.
#[test]
fn voyage_device_lost() {
    let pad3 = pad(3);
    let (mut a, mut b) = (harbor("b"), switched("b"));
    let seat = Seat {
        player: 0,
        device: pad3,
        ..Seat::default()
    };
    a.seats = vec![seat];
    b.seats = vec![seat];
    let d = VoyDrawer::new(true);
    let (mut g, p, src) = voy_game(Some(&d), vec![a, b]);
    *src.0.borrow_mut() = devices_in(&[keys1(), pad3], &[]);
    advance(&mut g, 1);
    advance(&mut g, 1);
    assert!(g.voy.on, "Fahrt beginnt nicht");
    let steps = p.steps.get();
    *src.0.borrow_mut() = devices_in(&[keys1()], &[keys1()]); // Pad weg, die Tastatur drückt Bestätigen
    advance(&mut g, 1);
    assert!(
        lost_cues(&g.frame.cues, pad3) == 1 && g.frame.seats[0].lost && g.voy.on,
        "Pad getrennt: Cues {:?}, Platz {:?}, Fahrt {}",
        g.frame.cues,
        g.frame.seats[0],
        g.voy.on
    );
    for i in 2..travel_ticks() {
        advance(&mut g, 1);
        assert!(
            g.voy.on && g.frame.cues.is_empty() && g.frame.seats[0].lost,
            "Takt {i}: Fahrt {}, Cues {:?}, Platz {:?}",
            g.voy.on,
            g.frame.cues,
            g.frame.seats[0]
        );
    }
    advance(&mut g, 1);
    assert!(
        !g.voy.on && p.steps.get() == steps && p.rebinds.borrow().is_empty() && p.joins.borrow().is_empty(),
        "Ende der Fahrt: Fahrt {}, Takte {}, Neuzuordnungen {:?}, Beitritte {:?}",
        g.voy.on,
        p.steps.get() - steps,
        p.rebinds.borrow(),
        p.joins.borrow()
    );
    advance(&mut g, 1);
    assert!(
        p.steps.get() == steps + 1 && *p.rebinds.borrow() == vec![(0, keys1())],
        "erster Takt am Ziel: Takte {}, Neuzuordnungen {:?}",
        p.steps.get() - steps,
        p.rebinds.borrow()
    );
}
