//! Tests von `RunApp` (RU33.3, AC-03): Das Ergebnis hängt weder vom Modus noch vom Zeitraffer ab. Der echte Fensterlauf
//! läuft nicht in `cargo test`; hier zeichnet die App auf den `NullCanvas` der Plattform.

use k3c_data::Source;
use k3c_platform::run_headless;

use super::enginetools_run::Segment;
use super::stepper::Stepper;
use super::stop::Stop;
use super::testutil::{call, data_dir, root};
use super::window::{Outcome, RunApp};
use super::{EXIT_USAGE, enginetools_run};

fn source() -> Source {
    Source::Dir(data_dir().into())
}

fn inputs() -> Vec<Segment> {
    vec![
        Segment {
            from_tick: 0,
            to_tick: 600,
            move_x: 1.0,
            pay: true,
            ..Segment::default()
        },
        Segment {
            player: 1,
            from_tick: 30,
            to_tick: 900,
            move_x: -1.0,
            ..Segment::default()
        },
    ]
}

fn stepper(ticks: i64) -> Stepper {
    Stepper::new(&source(), "forest", "k3c-dev", ticks, &inputs()).expect("Lauf")
}

/// Der Text der Zeile `result` des Falls `run-zwei-spieler` (von Go geschrieben).
fn fixture_text() -> String {
    let raw = std::fs::read_to_string(root().join("testdata/rust/runs/run-zwei-spieler.json")).expect("Fixture");
    let v: serde_json::Value = serde_json::from_str(&raw).expect("JSON");
    let line: serde_json::Value = serde_json::from_str(v["out"][0].as_str().expect("Zeile")).expect("JSON");
    line["text"].as_str().expect("text").to_string()
}

#[test]
fn headless_gleich_fixture() {
    let mut app = RunApp::headless(stepper(900), Stop::never());
    run_headless(&mut app, u32::MAX, None);
    assert_eq!(app.outcome(), Outcome::Finished(fixture_text()));
}

/// AC-03: Zeitraffer 1, 7 und 64 mit Zeichnen ergeben dieselbe Zeile `result` wie headless.
#[test]
fn zeitraffer_aendert_das_ergebnis_nicht() {
    let want = enginetools_run::run(&source(), "forest", "k3c-dev", 900, &inputs(), &Stop::never()).expect("Lauf");
    assert_eq!(want, fixture_text());
    for speed in [1, 7, 64] {
        let mut app = RunApp::windowed(stepper(900), Stop::never(), speed, None);
        run_headless(&mut app, u32::MAX, None);
        assert_eq!(app.outcome(), Outcome::Finished(want.clone()), "speed {speed}");
        assert!(app.frames > 0, "speed {speed}: nichts gezeichnet");
    }
}

/// Im Fenster bleibt die App nach dem letzten Takt 60 Bilder stehen.
#[test]
fn fenster_bleibt_nach_dem_ende_stehen() {
    let mut app = RunApp::windowed(stepper(30), Stop::never(), 64, None);
    run_headless(&mut app, u32::MAX, None);
    assert!(app.frames >= 60, "{} Bilder", app.frames);
}

#[test]
fn abbruch_und_fenster_zu_frueh_zu() {
    let stop = Stop::never();
    let mut app = RunApp::windowed(stepper(900), stop.clone(), 1, None);
    run_headless(&mut app, 10, None);
    assert_eq!(app.ticks_done(), 10);
    assert_eq!(app.outcome(), Outcome::Aborted, "Fenster vor dem Ende zu");
    stop.trigger();
    run_headless(&mut app, 5, None);
    assert_eq!(app.ticks_done(), 10, "nach dem Abbruch kein Takt mehr");
    assert_eq!(app.outcome(), Outcome::Aborted);
}

#[test]
fn schalter_von_window_und_speed() {
    let input = r#"{"seed":"1","ticks":1}"#;
    for args in [
        vec!["run", "--speed", "8"],
        vec!["run", "--window", "--speed", "0"],
        vec!["run", "--window", "--speed", "65"],
        vec!["run", "--window", "--headless"],
        vec!["level", "--window"],
    ] {
        let (code, out, _) = call(input, &args);
        assert!(
            code == EXIT_USAGE && out.contains("\"event\":\"error\""),
            "{args:?}: Exit {code} {out}"
        );
    }
}

/// Ein Stop in den Haltebildern beendet das Warten, das Ergebnis gilt (Exit 0 statt 130).
#[test]
fn stop_in_den_haltebildern_behaelt_das_ergebnis() {
    let stop = Stop::never();
    let mut app = RunApp::windowed(stepper(30), stop.clone(), 64, None);
    run_headless(&mut app, 3, None);
    stop.trigger();
    run_headless(&mut app, 3, None);
    assert!(matches!(app.outcome(), Outcome::Finished(_)));
}
