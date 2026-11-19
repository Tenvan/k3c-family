//! Port von `native/k3c-game/game/devvalues_test.go`.

use super::*;
use crate::game::play_testutil::play_game;
use crate::view::cell::{new_camera, rect};
use crate::view::devkeys::DEBUG_GEOMETRY;

// Takte je Sekunde: erst nach einer vollen Sekunde ein Wert; ein Neustart des Takts (neues Spiel) beginnt neu.
#[test]
fn tps_meter() {
    let mut m = TpsMeter::default();
    assert!(m.update(10.0, 100).is_none(), "erster Aufruf mit Wert");
    assert!(m.update(10.5, 115).is_none(), "nach einer halben Sekunde mit Wert");
    let tps = m.update(12.0, 160);
    assert!(
        tps.is_some_and(|t| (t - 30.0).abs() < 1e-9),
        "nach zwei Sekunden: {tps:?}"
    );
    assert!(m.update(13.0, 5).is_none(), "Takt zurückgesetzt, trotzdem ein Wert");
}

// Ein Klick wählt Spieler und Welt-x der Zelle unter der Maus (untere Zelle: Spieler 2).
#[test]
fn pick_at_cells() {
    let (top, bottom) = (rect(0, 0, 640, 180), rect(0, 180, 640, 360));
    let cells = [
        Cell {
            player: 0,
            rect: top,
            camera: new_camera(top, 50.0, 32.0),
            ..Cell::default()
        },
        Cell {
            index: 1,
            player: 1,
            rect: bottom,
            camera: new_camera(bottom, 10.0, 32.0),
        },
    ];
    let p = pick_at(&cells, Point { x: 320 + 64, y: 200 });
    assert!(
        p.is_some_and(|p| p.picked && p.player == 1 && (p.x - 12.0).abs() < 1e-9),
        "unten: {p:?}"
    );
    assert!(pick_at(&cells, Point { x: 700, y: 10 }).is_none(), "außerhalb gewählt");
}

// Die Takte je Sekunde zählt nur die Seite Werte; mit anderer Seite oder im Aufnahme-Modus bleibt der Zähler leer.
#[test]
fn dev_values_only_on_values_page() {
    let (mut g, _) = play_game(Vec::new());
    g.dev.debug = DEBUG_GEOMETRY;
    g.tps.update(10.0, 1);
    g.dev_values();
    assert!(g.tps.update(10.0, 1).is_none(), "Zähler nicht zurückgesetzt");
    g.dev.debug = DEBUG_VALUES;
    g.dev_values();
    g.args.shots = "x".to_string();
    g.dev_values();
    assert!(g.dev.tps.abs() < f64::EPSILON, "Wert im Aufnahme-Modus");
}
