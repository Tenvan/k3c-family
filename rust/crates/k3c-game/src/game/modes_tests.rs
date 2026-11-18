//! Port von `native/k3c-game/game/modes_test.go` (ohne `TestAssetDirs` und `TestPercentile`, die zu RU30 gehören).

use super::super::scene::parse_args;
use super::*;

fn argv(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

/// --flow startet im Hauptmenü ohne Szenen-Spiel und ohne Standard-Folge; --at setzt Vorlauf und Zahl der Bilder.
/// Der `shooter`-Teil des Go-Tests (`shots.go`) und die Folge `--input` kommen mit RU30.
#[test]
fn parse_args_flow_at() {
    let a = parse_args(&argv(&["--flow", "--shots", "o", "--at", "1,20,300"]), "s").unwrap();
    assert!(!a.scene.direct() && a.flow && a.script, "Ablauf: {:?}", a.scene);
    assert_eq!(a.at, vec![1, 20, 300]);
    assert_eq!((a.scene.tick, a.count), (1, 3));
    let a = parse_args(&argv(&["--flow", "--shots", "o"]), "s").unwrap();
    assert!(
        a.scene.seed.is_empty() && a.scene.script.is_empty(),
        "--flow ohne Folge: {:?}",
        a.scene
    );
}

/// Wie Go `TestParseArgsFlowAt` mit `--input` (braucht `parse_script`).
#[test]
#[ignore = "wartet auf RU30"]
fn parse_args_flow_at_input() {
    let a = parse_args(
        &argv(&["--flow", "--shots", "o", "--at", "1,20,300", "--input", "0@2=a:confirm"]),
        "s",
    )
    .unwrap();
    assert!(
        !a.scene.direct() && a.flow && a.script && a.scene.script.len() == 1,
        "Ablauf: {:?}",
        a.scene
    );
}

#[test]
fn parse_args_measure() {
    let a = parse_args(
        &argv(&[
            "--seed",
            "k3c",
            "--measure",
            "20",
            "--warmup",
            "5",
            "--report",
            "r.json",
            "--novsync",
        ]),
        "s",
    )
    .unwrap();
    let want = Measure {
        seconds: 20.0,
        warmup: 5.0,
        report: "r.json".into(),
        no_vsync: true,
        ..Measure::default()
    };
    assert!(a.measure == want && a.shots.is_empty(), "Messmodus: {:?}", a.measure);
}

/// --pacing wird angenommen (alte Aufrufe) und gemerkt, wirkt aber nicht.
#[test]
fn parse_args_pacing_wird_angenommen() {
    for v in ["frame", "sync", "irgendwas"] {
        let a = parse_args(&argv(&["--seed", "k3c", "--pacing", v]), "s").unwrap();
        assert_eq!(a.measure.pacing, v);
    }
}

#[test]
fn parse_args_modes_reject() {
    let bad: &[&[&str]] = &[
        &["--flow", "--seed", "x"],
        &["--flow", "--menu", "main"],
        &["--at", "1,2"],
        &["--measure", "-1"],
        &["--river", "x"],
        &["--stall"],
    ];
    for v in bad {
        assert!(parse_args(&argv(v), "s").is_err(), "angenommen: {}", v.join(" "));
    }
}

/// Die Fälle mit `--shots` ohne `--menu` laden die Standard-Folge und brauchen `parse_script` (RU30.2).
#[test]
#[ignore = "wartet auf RU30"]
fn parse_args_modes_reject_shots() {
    let bad: &[&[&str]] = &[
        &["--shots", "o", "--at", "5,5"],
        &["--shots", "o", "--at", "9,3"],
        &["--shots", "o", "--at", "a"],
        &["--measure", "5", "--shots", "o"],
    ];
    for v in bad {
        assert!(parse_args(&argv(v), "s").is_err(), "angenommen: {}", v.join(" "));
    }
}

/// `--at` mit Menü-Aufnahme (ohne Standard-Folge): streng aufsteigend wird ohne `parse_script` geprüft.
#[test]
fn parse_args_at_streng_aufsteigend() {
    for bad in ["5,5", "9,3", "a", "-1", "432001"] {
        let r = parse_args(&argv(&["--shots", "o", "--menu", "main", "--at", bad]), "s");
        assert!(r.is_err(), "angenommen: --at {bad}");
    }
    let a = parse_args(&argv(&["--shots", "o", "--menu", "main", "--at", "0,432000"]), "s").unwrap();
    assert_eq!(a.at, vec![0, 432000]);
}
