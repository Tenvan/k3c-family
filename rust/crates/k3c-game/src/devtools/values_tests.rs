//! Port von `native/k3c-game/devtools/values_test.go`.

use std::rc::Rc;

use k3c_platform::NullCanvas;
use k3c_sim::state::shared::CycleInfo;
use k3c_sim::state::{Coin, Job, Player};

use super::debuggeo_tests::{geo_cell, geo_world};
use super::new;
use super::values::{pick_x, select_at, value_lines};
use crate::view::cell::{rect, window_cell};
use crate::view::devkeys::{DEBUG_VALUES, DevView, Pick};
use crate::view::frame::{Frame, Seat, Stage};

/// value_frame ist ein Frame mit zwei Spielern in derselben Stufe der Welt `geo_world` (Monarch 1 bei 50, Monarch 2
/// bei 20).
fn value_frame() -> Frame {
    let mut w = geo_world();
    w.seed = "wald".to_string();
    w.wave = 3;
    w.stock.wood = 7;
    w.stock.iron = 2;
    w.cycle = CycleInfo {
        phase: "day".to_string(),
        day: 4,
        ..CycleInfo::default()
    };
    w.players[0].gold = 12;
    w.players.push(Player {
        index: 1,
        x: 20.0,
        gold: 5,
        ..Player::default()
    });
    w.sites[0].state = "built".to_string();
    w.coins = vec![Coin {
        id: 9,
        x: 80.0,
        ..Coin::default()
    }];
    Frame {
        tick: 120,
        timescale: 2,
        step_ms: 1.25,
        dev: DevView {
            debug: DEBUG_VALUES,
            tps: 30.0,
            ..DevView::default()
        },
        seats: vec![
            Seat {
                player: 0,
                monarch: 0,
                stage: 0,
                ..Seat::default()
            },
            Seat {
                player: 1,
                monarch: 1,
                stage: 0,
                ..Seat::default()
            },
        ],
        stages: vec![Stage {
            index: 0,
            depth: 0,
            world: Some(Rc::new(w)),
            ..Stage::default()
        }],
        ..Frame::default()
    }
}

/// AC-05 (B-889): Die Seite zeigt Tageszeit mit Tag und Welle, je Spieler Gold, Vorrat, Position und Chunk, die
/// Objektanzahl, den Takt und das Objekt am Monarchen (Gegner bei 52, näher als der Bauplatz bei 46).
#[test]
fn test_value_lines() {
    let mut lines = Vec::new();
    value_lines(&mut lines, &value_frame(), &geo_cell());
    let text = lines.join("\n");
    for want in [
        "Tag 4 · Tag · Welle 3",
        "Spieler 1: Gold 12 · Holz 7",
        "Eisen 2",
        "x 50.0 · Chunk 2",
        "Spieler 2: Gold 5",
        "Chunk 1",
        "Truppen 1 · Gegner 1 · Bauten 1 · Münzen am Boden 1",
        "Takt 120 · 30/s · 1.25 ms · Zeitraffer 2× · Seed wald",
        "Auswahl (am Monarchen): Monarch 1",
    ] {
        assert!(text.contains(want), "fehlt {want:?} in\n{text}");
    }
}

fn select(w: &k3c_sim::state::World, x: f64, how: &str) -> Vec<String> {
    let mut out = Vec::new();
    select_at(&mut out, w, x, how);
    out
}

/// AC-05: Ein Klick wählt das nächste Objekt an der Stelle (Gegner mit Zustand, Ziel, LP und Entscheidung, Truppe mit
/// Auftrag); weit weg von allem bleibt die Auswahl leer.
#[test]
fn test_select_at() {
    let mut w = geo_world();
    w.enemies[0].hp = 3.0;
    w.enemies[0].max_hp = 10.0;
    w.enemies[0].fleeing = true;
    w.enemies[0].home_x = 99.0;
    let got = select(&w, 52.5, "Klick").join(" | ");
    for want in [
        "Auswahl (Klick): Gegner #2",
        "LP 3/10",
        "Zustand flieht",
        "Ziel Portal x 99.0",
        "KI: zurück zum Portal",
    ] {
        assert!(got.contains(want), "fehlt {want:?} in {got:?}");
    }
    w.troops[0].job = Some(Job {
        r#type: "gather".to_string(),
        resource: "wood".to_string(),
        ..Job::default()
    });
    let got = select(&w, 95.0, "").join(" ");
    assert!(got.contains("KI: gather wood"), "Truppe: {got:?}");
    assert_eq!(select(&w, 30.0, "Klick"), vec!["Auswahl: –".to_string()], "leer");
    let mut f = value_frame();
    f.dev.pick = Pick {
        picked: true,
        player: 0,
        x: 46.0,
    };
    assert_eq!(pick_x(&f, &geo_cell()), (46.0, "Klick"), "Maus-Wahl");
}

/// Die Seite Werte zeichnet ohne Schrift ohne Absturz.
#[test]
fn test_values_draw() {
    let parts = new(None);
    let mut dst = NullCanvas::default();
    let mut f = value_frame();
    f.cells = vec![geo_cell()];
    let win = window_cell(rect(0, 0, 640, 360));
    for d in &parts.drawers {
        d.draw(&mut dst, &f, &win);
    }
    assert!(dst.shapes > 0, "Formen bleiben auf der Seite Werte stehen");
}
