//! Smoke-Test des Spiels ohne Fenster (RU48.2): ein neues Spiel mit zwei Spielern läuft 6000 Takte (200 Sekunden, mehr
//! als ein Tag-Nacht-Zyklus) durch dieselbe Verdrahtung wie `k3c-game` (`game::launch`), ohne Grafik. Spieler 1 (Tastatur)
//! läuft nach rechts und zahlt, Spieler 2 (Pad 0) steht. Geprüft: kein Panic, die Zeit der Welt wächst, in der Nacht
//! erscheinen Gegner, Zeichnen läuft in jedem Bild.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use k3c_game::game::keytable::keys_for;
use k3c_game::game::launch::{build_game, open_session, parse};
use k3c_game::game::{Game, Libs};
use k3c_game::view::keys::{KEY_MOVE, KEY_PAY};
use k3c_platform::{App, Canvas, InputScript, InputState, ScriptEntry, run_headless};

const TICKS: u32 = 6000;

/// Spielstand-Ordner (unter `target/`, nur gelesen).
fn saves() -> String {
    format!("{}/target/run-smoke", env!("CARGO_MANIFEST_DIR"))
}

/// Eingabe: Spieler 1 hält rechts, tippt alle 30 Takte „zahlen“.
fn script() -> InputScript {
    let right = *keys_for(0, KEY_MOVE).get(1).expect("Taste rechts");
    let pay = *keys_for(0, KEY_PAY).first().expect("Taste zahlen");
    let entries = (0..u64::from(TICKS))
        .step_by(15)
        .enumerate()
        .map(|(i, step)| ScriptEntry {
            step,
            keys: if i % 2 == 0 { vec![right, pay] } else { vec![right] },
            pads: Vec::new(),
        })
        .collect();
    InputScript::new(entries)
}

/// Schleife mit Messung: Zeit der Welt, größte Zahl Gegner, gezeichnete Bilder.
struct Probe {
    g: Game,
    ticks: u64,
    time: f64,
    max_enemies: usize,
    frames: u64,
}

impl App for Probe {
    fn step(&mut self, input: &InputState, dt: f64) {
        self.g.step(input, dt);
        self.ticks += 1;
        let world = self.g.frame().stages.first().and_then(|s| s.world.as_deref());
        if let Some(w) = world {
            self.time = w.time;
            self.max_enemies = self.max_enemies.max(w.enemies.len());
        }
    }

    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        self.g.draw(c, alpha);
        self.frames += 1;
    }

    fn done(&self) -> bool {
        self.g.done()
    }
}

#[test]
#[ignore = "wartet auf RU48.3: Platzhalter im Spielpfad (props/simview.rs economy_of u. a.)"]
fn zwei_spieler_6000_takte_ohne_panic() {
    let args = parse(&[
        "--saves".into(),
        saves(),
        "--seed".into(),
        "smoke".into(),
        "--players".into(),
        "2".into(),
    ])
    .expect("Schalter");
    let g = build_game(&args, open_session(&args), &Libs::default()).expect("Spiel");
    let mut p = Probe {
        g,
        ticks: 0,
        time: 0.0,
        max_enemies: 0,
        frames: 0,
    };
    run_headless(&mut p, TICKS, Some(&script()));
    assert_eq!(p.ticks, u64::from(TICKS), "alle Takte gerechnet");
    assert_eq!(p.frames, u64::from(TICKS), "jedes Bild gezeichnet");
    assert!(p.time > 100.0, "Zeit der Welt wächst: {}", p.time);
    assert!(p.max_enemies > 0, "in der Nacht erscheinen Gegner");
    assert_eq!(p.g.frame().seats.len(), 2, "zwei Spieler");
}

/// Ohne Takte: Menü → Frame ist lieferbar, die Sitzung hat keine Welt.
#[test]
fn menue_liefert_frame_ohne_spiel() {
    let args = parse(&["--saves".into(), saves()]).expect("Schalter");
    let mut g = build_game(&args, open_session(&args), &Libs::default()).expect("Spiel");
    assert!(g.screen().is_some());
    let mut c = k3c_platform::NullCanvas::default();
    g.draw(&mut c, 0.0);
    assert!(g.frame().stages.is_empty() && g.frame().seats.is_empty());
}
