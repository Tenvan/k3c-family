//! Tests von Schaltern und Eingabe-JSON (Port der Eingabe-Teile von `engine/simcli/simcli_test.go`).

use super::input::*;
use super::{EXIT_USAGE, RunError};

fn args(a: &[&str]) -> Vec<String> {
    a.iter().map(ToString::to_string).collect()
}

#[test]
fn schalter_in_beliebiger_folge() {
    let f = parse_flags(&args(&["--input", "a.json", "--headless", "--data=d", "--stop-stdin"])).expect("Schalter");
    assert_eq!(f.input, "a.json");
    assert!(f.headless && f.stop_stdin && !f.window);
    assert_eq!(f.data.as_deref(), Some("d"));
    let g = parse_flags(&args(&["--window", "--speed", "8", "-input=x"])).expect("Schalter");
    assert!(g.window && g.speed == Some(8) && g.input == "x");
}

#[test]
fn schalter_fehler() {
    for a in [
        &["--gibtsnicht"][..],
        &["--input"],
        &["extra"],
        &["--window=vielleicht"],
        &["--speed", "x"],
    ] {
        assert!(matches!(parse_flags(&args(a)), Err(RunError::Usage(_))), "{a:?}");
    }
    assert_eq!(EXIT_USAGE, 2);
}

#[test]
fn run_eingabe_und_unbekannte_felder() {
    let r = read_run_input(r#"{"biome":"cave","seed":"x","ticks":5,"inputs":[{"player":1,"fromTick":2,"toTick":4,"moveX":-1,"sprint":true}]}"#)
        .expect("Eingabe");
    assert_eq!((r.biome.as_str(), r.seed.as_str(), r.ticks), ("cave", "x", 5));
    assert_eq!(r.inputs.len(), 1);
    assert!(r.inputs[0].sprint && r.inputs[0].player == 1 && r.inputs[0].move_x == -1.0);
    // Go vergleicht Feldnamen ohne Groß- und Kleinschreibung
    assert_eq!(read_run_input(r#"{"TICKS":7}"#).expect("Eingabe").ticks, 7);
    let Err(RunError::Usage(m)) = read_run_input(r#"{"tempo":3}"#) else {
        panic!("kein Aufruffehler")
    };
    assert_eq!(m, "Eingabe-JSON: json: unknown field \"tempo\"");
    let Err(RunError::Usage(m)) = read_run_input(r#"{"inputs":[{"fromtick":1,"x":2}]}"#) else {
        panic!("kein Aufruffehler")
    };
    assert!(m.contains("unknown field \"x\""), "{m}");
}

#[test]
fn eingabe_fehler() {
    for raw in [
        "",
        "kein json",
        r#"{"ticks":"zehn"}"#,
        r#"{"ticks":1.5}"#,
        "[1]",
        r#"{} {}"#,
    ] {
        assert!(matches!(read_run_input(raw), Err(RunError::Usage(_))), "{raw:?}");
    }
    assert_eq!(
        read_level_input(r#"{"biome":"mine","seed":"s"}"#)
            .expect("Eingabe")
            .biome,
        "mine"
    );
    assert!(read_level_input(r#"{"ticks":1}"#).is_err());
}
