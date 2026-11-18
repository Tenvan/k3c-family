//! Port von `native/k3c-game/game/scene_test.go` (ohne `TestCommandsAt` und `TestScriptedSource`, die zu RU30.2 gehören).

use super::*;
use crate::view::input::DeviceKind;

fn argv(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

/// Szenen-Schalter des Aufnahme-Modus (AC-11): Standardwerte, gesetzte Werte und Ablehnung.
#[test]
fn parse_args_defaults() {
    let a = parse_args(&[], "saves-dir").unwrap();
    assert!(
        a.saves == "saves-dir" && a.shots.is_empty() && !a.scene.direct() && !a.script && a.w == 1280 && a.h == 720,
        "ohne Schalter: {a:?}"
    );
}

/// Zweiter Teil von `TestParseArgsDefaults`: die Aufnahme lädt die Standard-Folge (`parse_script`).
#[test]
#[ignore = "wartet auf RU30"]
fn parse_args_defaults_aufnahme() {
    let a = parse_args(&argv(&["--shots", "out"]), "s").unwrap();
    let ok = a.scene.seed == "k3c" && a.count == 8 && a.every == 3 && a.scene.players == 2;
    assert!(ok && a.script && a.scene.script.len() == 2, "Aufnahme-Standard: {a:?}");
}

#[test]
#[ignore = "wartet auf RU30"]
fn parse_args_scene() {
    let v = [
        "--shots",
        "o",
        "--load",
        "wald-1",
        "--players",
        "1",
        "--cam",
        "120.5",
        "--tick",
        "900",
        "--count",
        "4",
        "--every",
        "6",
        "--size",
        "1920x1080",
        "--input",
        "0@0=RS,0@30=-",
        "--saves",
        "x",
    ];
    let a = parse_args(&argv(&v), "s").unwrap();
    let s = &a.scene;
    assert!(
        s.load == "wald-1" && s.seed.is_empty() && s.players == 1 && s.cams == [120.5] && s.tick == 900,
        "{s:?}"
    );
    assert!(a.count == 4 && a.every == 6 && a.w == 1920 && a.h == 1080 && a.saves == "x" && s.script.len() == 2);
}

/// Ohne `--input` und ohne `--shots` läuft die Szene mit allen Schaltern, die kein `parse_script` brauchen.
#[test]
fn parse_args_scene_ohne_folge() {
    let v = [
        "--load",
        "wald-1",
        "--players",
        "1",
        "--cam",
        "120.5",
        "--tick",
        "900",
        "--count",
        "4",
        "--every",
        "6",
        "--size",
        "1920x1080",
        "--saves",
        "x",
    ];
    let a = parse_args(&argv(&v), "s").unwrap();
    let s = &a.scene;
    assert!(
        s.load == "wald-1" && s.seed.is_empty() && s.players == 1 && s.cams == [120.5] && s.tick == 900,
        "{s:?}"
    );
    assert!(a.count == 4 && a.every == 6 && a.w == 1920 && a.h == 1080 && a.saves == "x" && s.script.is_empty());
    assert!(!a.script);
}

#[test]
fn parse_args_cam_and_devices() {
    let d = Scene {
        players: 1,
        ..Scene::default()
    }
    .devices();
    assert_eq!(d, vec![DeviceID::new(DeviceKind::Keyboard, 0)]);
    let d = Scene {
        players: 2,
        ..Scene::default()
    }
    .devices();
    assert_eq!(d, vec![DeviceID::default(), DeviceID::new(DeviceKind::Gamepad, 0)]);
    let a = parse_args(&argv(&["--seed", "x", "--depth", "1", "--cam", "-,40"]), "s").unwrap();
    let cams = &a.scene.cams;
    assert!(
        a.scene.depth == 1 && cams[0].is_nan() && cams[1] == 40.0 && !a.script,
        "Kamera folgt / fest: {:?}",
        a.scene
    );
}

#[test]
fn scene_in_game_und_direct() {
    let mk = |seed: &str, menu: &str| Scene {
        seed: seed.into(),
        menu: menu.into(),
        ..Scene::default()
    };
    assert!(mk("", "pause").in_game() && mk("x", "options").in_game() && mk("x", "keys").in_game());
    assert!(!mk("", "options").in_game() && !mk("x", "").in_game() && !mk("x", "main").in_game());
    assert!(mk("x", "").direct() && !mk("", "").direct());
}

#[test]
fn parse_args_rejects() {
    let bad: &[&[&str]] = &[
        &["--players", "3"],
        &["--seed", "a", "--load", "b"],
        &["--load", "../boese"],
        &["--tick", "-1"],
        &["--depth", "-2"],
        &["--cam", "1,2,3"],
        &["--cam", "abc"],
        &["--count", "0"],
        &["--every", "0"],
        &["--size", "100x50"],
        &["--size", "groß"],
        &["überzählig"],
        &["--unbekannt"],
        &["--menu", "x"],
    ];
    for v in bad {
        assert!(parse_args(&argv(v), "s").is_err(), "angenommen: {}", v.join(" "));
    }
}

/// Die Fälle mit `--input` brauchen `parse_script` (RU30.2).
#[test]
#[ignore = "wartet auf RU30"]
fn parse_args_rejects_input() {
    let bad: &[&[&str]] = &[
        &["--players", "1", "--input", "1@0=R"],
        &["--input", "0@0=X"],
        &["--input", "0=R"],
        &["--input", "0@0="],
    ];
    for v in bad {
        assert!(parse_args(&argv(v), "s").is_err(), "angenommen: {}", v.join(" "));
    }
}

#[test]
fn fehlermeldung_nennt_grund_und_usage() {
    let e = parse_args(&argv(&["--unbekannt"]), "s").unwrap_err();
    assert!(e.starts_with("🚫 ungültige Schalter (") && e.ends_with(USAGE), "{e}");
    let e = parse_args(&argv(&["ueber"]), "s").unwrap_err();
    assert!(e.contains("(<nil> [ueber])"), "{e}");
}
