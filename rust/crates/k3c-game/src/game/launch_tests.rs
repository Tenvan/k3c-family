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

/// Das Fehlerfenster vor dem Spiel läuft, bis eine Beenden-Taste kommt (Go: `showFault`): Leertaste beendet wie Enter,
/// eine andere Taste nicht.
#[test]
fn fehlerfenster_endet_mit_der_beenden_taste() {
    use k3c_platform::{InputScript, Key, ScriptEntry};
    let press = |step: u64, key: Key| {
        [
            ScriptEntry {
                step,
                keys: vec![key],
                pads: Vec::new(),
            },
            ScriptEntry {
                step: step + 1,
                keys: Vec::new(),
                pads: Vec::new(),
            },
        ]
    };
    let a = args(&["--grafik", "gibt-es-nicht"]);
    let mut app = Launch::new(&a);
    let other = InputScript::new(press(5, Key::A).to_vec());
    run_headless(&mut app, 12, Some(&other));
    assert!(app.fault().is_some() && !app.done(), "eine andere Taste beendet nicht");
    let space = InputScript::new(press(2, Key::Space).to_vec());
    run_headless(&mut app, 12, Some(&space));
    assert!(app.done(), "Leertaste beendet das Fehlerfenster");
    assert!(app.fault().is_some(), "die Meldung bleibt für stderr und Exit 1");
}

/// Seit RU30.4 ist der Messmodus freigegeben: `--measure` mit allen Begleit-Schaltern wird angenommen und gelesen.
#[test]
fn schalter_des_messmodus_werden_angenommen() {
    let v: Vec<String> = [
        "--seed=t",
        "--measure=5",
        "--warmup=2",
        "--report=r.json",
        "--novsync",
        "--measure-frames=b.jsonl",
        "--measure-load=5,160,120",
        "--pacing=x",
    ]
    .iter()
    .map(|s| (*s).to_string())
    .collect();
    let a = parse(&v).unwrap_or_else(|e| unreachable!("{e}"));
    let m = &a.measure;
    assert!(m.seconds == 5.0 && m.warmup == 2.0 && m.no_vsync, "{m:?}");
    assert_eq!(
        (m.report.as_str(), m.frames.as_str(), m.load.as_str()),
        ("r.json", "b.jsonl", "5,160,120")
    );
}

/// Seit RU30.3 sind die Dev-Schalter `--river` und `--stats` freigegeben.
#[test]
fn dev_schalter_river_und_stats_werden_angenommen() {
    for f in ["--stats", "--river=tiles"] {
        let a = parse(&[f.to_string()]);
        assert!(a.is_ok(), "{f}: {:?}", a.err());
    }
}

/// Seit RU30.2 sind Aufnahme, Eingabe-Folge, Ablauf, Ladeanzeige und `--assets` freigegeben.
#[test]
fn schalter_der_aufnahme_werden_angenommen() {
    for f in [
        &["--shots=x"][..],
        &["--shots=x", "--input", "0@0=R"],
        &["--shots=x", "--flow"],
        &["--shots=x", "--stall"],
        &["--assets"],
    ] {
        let v: Vec<String> = f.iter().map(|s| (*s).to_string()).collect();
        assert!(parse(&v).is_ok(), "{f:?}");
    }
}
