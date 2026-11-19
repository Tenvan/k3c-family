//! Tests der Entwickler-Rolle (Go: `dev_test.go`, RU50.1): Halt und Einzelschritt, Dev-Aktionen, Neu-Laden.

use super::session::Session;
use super::testutil::{TWO_DEVICES, TempDir, test_session};
use crate::game::sim::Sim as _;
use crate::view::devkeys::{DevAction, DevSession as _};
use crate::view::frame::Frame;
use crate::view::session::{NewGame, Session as _};

/// Startet ein Spiel mit zwei Spielern und rechnet einen Takt.
fn dev_game(dir: &TempDir) -> (Session, Frame) {
    let mut s = test_session(dir);
    let g = NewGame {
        name: "dev".into(),
        devices: TWO_DEVICES.to_vec(),
        ..NewGame::default()
    };
    s.new_game(g).expect("neues Spiel");
    let f = s.step();
    (s, f)
}

fn act(action: &str) -> DevAction {
    DevAction {
        action: action.into(),
        ..DevAction::default()
    }
}

/// Münzen und Holz der ersten Stufe im Raum (nicht im Frame: der ist eine Kopie vom Takt).
fn coins_and_wood(s: &mut Session) -> (usize, i64) {
    s.with_room(|r, _| (r.isl.stages[0].coins.len(), r.isl.stages[0].stock.wood))
        .expect("Raum")
}

fn phase(s: &mut Session) -> String {
    s.with_room(|r, _| r.isl.stages[0].cycle.phase.clone()).expect("Raum")
}

// AC-02 (B-888): Halt hält den Takt an, `step_once` rechnet genau einen; ohne Halt wirkt `step_once` nicht.
#[test]
fn halt_step_once() {
    let dir = TempDir::new();
    let (mut s, f) = dev_game(&dir);
    s.step_once();
    let next = s.step();
    assert_eq!(next.tick, f.tick + 1, "step_once ohne Halt");
    s.halt(true);
    let (a, b) = (s.step(), s.step());
    assert!(
        s.halted() && b.paused && b.tick == a.tick,
        "Halt: Takt {} -> {}",
        a.tick,
        b.tick
    );
    s.step_once();
    let (c, d) = (s.step(), s.step());
    assert!(
        c.tick == b.tick + 1 && d.tick == c.tick,
        "Einzelschritt: {} -> {} -> {}",
        b.tick,
        c.tick,
        d.tick
    );
    s.halt(false);
    let e = s.step();
    assert!(
        e.tick == d.tick + 1 && !e.paused,
        "weiter: Takt {}, paused {}",
        e.tick,
        e.paused
    );
}

// AC-03 (B-888): Gold fällt beim Spieler, Material kommt in den Vorrat, Zeitraffer steht im Frame, nächste Tageszeit und
// Nacht wechseln die Phase; ohne Spiel und für fehlende Spieler lehnt `dev` ab.
#[test]
fn dev_actions() {
    let dir = TempDir::new();
    let (mut s, _) = dev_game(&dir);
    let (coins, wood) = coins_and_wood(&mut s);
    s.dev(1, &act_with("gold", 5, "")).expect("gold");
    s.dev(1, &act_with("material", 3, "wood")).expect("material");
    let (c2, w2) = coins_and_wood(&mut s);
    assert!(
        c2 == coins + 5 && w2 > wood,
        "Münzen {coins} -> {c2}, Holz {wood} -> {w2}"
    );
    let ts = DevAction {
        factor: 4,
        ..act("timescale")
    };
    s.dev(0, &ts).expect("timescale");
    assert_eq!(s.step().timescale, 4, "Zeitraffer im Frame");
    let before = phase(&mut s);
    s.dev(0, &act("nextPhase")).expect("nextPhase");
    s.step();
    assert_ne!(phase(&mut s), before, "nächste Tageszeit");
    s.dev(0, &act("night")).expect("night");
    s.step();
    assert_eq!(phase(&mut s), "night", "Nacht");
    let err = s.dev(3, &act_with("gold", 1, "")).expect_err("fehlender Spieler");
    assert_eq!(err, "Spieler 4 spielt nicht");
    let err = s.dev(0, &act("gibt-es-nicht")).expect_err("unbekannte Aktion");
    assert_eq!(err, "Cheat „gibt-es-nicht“ geht gerade nicht");
}

fn act_with(action: &str, amount: i64, resource: &str) -> DevAction {
    DevAction {
        amount,
        resource: resource.into(),
        ..act(action)
    }
}

// Ohne Spiel lehnt `dev` ab, ohne Dev-Mode der Einstellungen geht es trotzdem (das native Spiel läuft lokal).
#[test]
fn dev_without_game() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert_eq!(s.dev(0, &act("gold")).expect_err("kein Spiel"), "Es läuft kein Spiel");
    assert!(!s.settings.dev_mode);
    let (mut s, _) = dev_game(&TempDir::new());
    s.dev(0, &act_with("gold", 1, "")).expect("ohne Dev-Mode");
}

// Reload verwirft den Fortschritt seit dem letzten Speichern und behält die Geräte.
#[test]
fn reload() {
    let dir = TempDir::new();
    let (mut s, _) = dev_game(&dir);
    s.save().expect("speichern");
    let saved = s.step().stages[0].world.as_ref().expect("Welt").time;
    for _ in 0..30 {
        s.step();
    }
    s.reload().expect("Reload");
    let f = s.step();
    let time = f.stages[0].world.as_ref().expect("Welt").time;
    assert!(
        time <= saved + 0.1 && f.seats.len() == 2,
        "nach Reload: Zeit {time} (gespeichert {saved}), Plätze {}",
        f.seats.len()
    );
    s.reset();
    assert!(s.reload().is_err(), "Reload ohne Spiel");
}

// Ohne gespeicherten Stand dieses Spiels meldet Reload es und läßt das Spiel laufen.
#[test]
fn reload_without_save() {
    let dir = TempDir::new();
    let (mut s, _) = dev_game(&dir);
    assert_eq!(
        s.reload().expect_err("nichts gespeichert"),
        "Noch kein ladbarer Spielstand dieses Spiels (F5 speichert)"
    );
    assert!(s.active());
}

// RU52.7: Die Entwickler-Rolle ist über `view::Session` erreichbar (Go: Typ-Zusicherung), `devtools::dev_of` liefert sie,
// und ein Cheat aus dem Menü wirkt im lokalen Spiel mit zwei Spielern.
#[test]
fn dev_role_via_view_session() {
    let (mut s, _) = dev_game(&TempDir::new());
    let before = coins_and_wood(&mut s).1;
    let vs: &mut dyn crate::view::session::Session = &mut s;
    let dev = crate::devtools::dev_of(vs).expect("Entwickler-Rolle");
    dev.dev(1, &act_with("material", 5, "wood")).expect("Cheat");
    assert!(coins_and_wood(&mut s).1 > before, "Holz nicht gestiegen");
}
