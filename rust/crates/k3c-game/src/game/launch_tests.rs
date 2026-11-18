//! Tests von `launch.rs` (RU48.1): Start ohne Fenster, zwei Spieler, Meldung statt Absturz bei fehlender Grafik.

use super::*;
use crate::game::stack::Mode;
use k3c_platform::run_headless;

/// Spielstand-Ordner der Tests (im Workspace-`target/`, wird nur gelesen).
fn saves() -> String {
    format!("{}/../../target/launch-test", env!("CARGO_MANIFEST_DIR"))
}

fn args(extra: &[&str]) -> Args {
    let mut v: Vec<String> = ["--saves".to_string(), saves()].to_vec();
    v.extend(extra.iter().map(|s| (*s).to_string()));
    parse(&v).unwrap()
}

#[test]
fn ohne_schalter_gilt_der_standardordner() {
    let a = parse(&[]).unwrap();
    assert_eq!(a.saves, default_saves_dir());
    assert!(!a.scene.direct(), "ohne Szene zeigt das Spiel das Hauptmenü");
}

#[test]
fn hauptmenue_ohne_taktung_und_erstes_bild() {
    let a = args(&[]);
    let mut g = build_game(&a, open_session(&a), &Libs::default()).unwrap();
    assert_eq!(g.mode(), Mode::Menu);
    assert!(g.screen().is_some(), "Hauptmenü liegt oben");
    let mut c = k3c_platform::NullCanvas::default();
    k3c_platform::App::draw(&mut g, &mut c, 0.0);
}

/// Szene mit zwei Spielern (Tastatur und Pad 0, wie Menü → Neues Spiel mit zwei Geräten): das Spiel steht.
#[test]
fn neues_spiel_mit_zwei_geraeten_steht() {
    let a = args(&["--seed", "smoke", "--players", "2"]);
    let g = build_game(&a, open_session(&a), &Libs::default()).unwrap();
    assert_eq!(g.mode(), Mode::Play);
}

/// Der erste Takt liefert zwei Spieler und zwei Zellen.
#[test]
fn neues_spiel_erster_takt_zwei_spieler() {
    let a = args(&["--seed", "smoke", "--players", "2"]);
    let mut g = build_game(&a, open_session(&a), &Libs::default()).unwrap();
    let raw = k3c_platform::InputState::new();
    k3c_platform::App::step(&mut g, &raw, 1.0 / 30.0);
    assert_eq!(g.frame().seats.len(), 2);
    assert_eq!(g.frame().cells.len(), 2);
}

/// Ohne Grafik zeigt das Programm eine Meldung mit dem Befehl `task gen:paper`, es stürzt nicht ab.
#[test]
fn fehlende_grafik_ist_eine_meldung() {
    let a = args(&["--grafik", "gibt-es-nicht"]);
    let mut app = Launch::new(&a);
    run_headless(&mut app, 3, None);
    let msg = app.fault().unwrap_or_default().to_string();
    assert!(msg.contains("task gen:paper"), "Meldung: {msg}");
    assert!(app.game().is_none());
}

#[test]
fn umbruch_haelt_die_zeilenlaenge() {
    let lines = wrap("aaa bbb ccc\nddd", 7);
    assert_eq!(lines, ["aaa bbb", "ccc", "ddd"]);
}

#[test]
fn schalter_der_dev_teile_werden_abgelehnt() {
    for f in ["--stats", "--measure=5", "--shots=x"] {
        let e = parse(&[f.to_string()]).unwrap_err();
        assert!(e.contains("RU30"), "{f}: {e}");
    }
}
