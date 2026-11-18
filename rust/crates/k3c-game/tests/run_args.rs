//! Startparameter des Spiels (RU30.6, AC-03): Jeder Schalter aus `USAGE` wird angenommen, ungültige Schalter lehnt das
//! Spiel mit `🚫 ungültige Schalter` ab (`k3c-game` endet dann mit `💥 …` und Exit-Code 1, siehe `game::run`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use k3c_game::game::launch::parse;
use k3c_game::game::run::{finish, start};
use k3c_game::game::scene::USAGE;

fn strs(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| (*s).to_string()).collect()
}

/// Gültige Aufrufe: je Schalter aus `USAGE` mindestens einer, mit einem Wert, der zum Muster passt.
const GOOD: &[&[&str]] = &[
    &["--saves", "ordner"],
    &["--seed", "s"],
    &["--load", "stand"],
    &["--flow"],
    &["--depth", "2"],
    &["--players", "1"],
    &["--players", "2"],
    &["--cam", "10"],
    &["--cam", "10,20"],
    &["--tick", "5"],
    &["--input", "0@0=R,1@5=a:confirm"],
    &["--menu", "main"],
    &["--menu", "saves"],
    &["--menu", "options"],
    &["--menu", "lobby"],
    &["--menu", "keys"],
    &["--menu", "licenses"],
    &["--size", "800x600"],
    &["--shots", "bilder", "--seed", "s"],
    &["--shots", "bilder", "--seed", "s", "--count", "3"],
    &["--shots", "bilder", "--seed", "s", "--every", "2"],
    &["--shots", "bilder", "--seed", "s", "--at", "1,5,9"],
    &["--measure", "5", "--seed", "s"],
    &["--measure", "5", "--seed", "s", "--warmup", "2"],
    &["--measure", "5", "--seed", "s", "--report", "bericht.json"],
    &["--measure", "5", "--seed", "s", "--novsync"],
    &["--assets"],
    &["--grafik", "ordner"],
    &["--river", "shader"],
    &["--river", "tiles"],
    &["--stats"],
    &["--stall", "--shots", "bilder", "--seed", "s"],
];

/// Ungültige Aufrufe: unbekannter Schalter, fehlender Wert, übrige Wörter.
const BAD: &[&[&str]] = &[
    &["--gibt-es-nicht"],
    &["--saves"],
    &["--players", "x"],
    &["--depth", "zwei"],
    &["uebrig"],
    &["--seed", "s", "uebrig"],
];

/// Jeder Schalter aus `USAGE` kommt in der Tabelle vor (die Tabelle wächst mit dem Text).
#[test]
fn tabelle_deckt_alle_schalter_aus_usage() {
    let names: Vec<&str> = USAGE
        .split("--")
        .skip(1)
        .map(|s| {
            s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
                .next()
                .unwrap_or("")
        })
        .collect();
    assert!(names.len() >= 20, "USAGE gelesen: {names:?}");
    for n in names {
        let flag = format!("--{n}");
        assert!(GOOD.iter().any(|g| g.contains(&flag.as_str())), "{flag} fehlt in GOOD");
    }
}

#[test]
fn gueltige_schalter_werden_angenommen() {
    for g in GOOD {
        // --saves ohne Ordner-Anlegen: `parse` liest nur, der Ordner entsteht erst beim Öffnen der Sitzung
        let r = parse(&strs(g));
        assert!(r.is_ok(), "{g:?}: {:?}", r.err());
    }
}

#[test]
fn ungueltige_schalter_sind_exit_1_mit_meldung() {
    for b in BAD {
        let (res, quiet) = start(&strs(b));
        let msg = res.clone().err().unwrap_or_default();
        assert!(msg.starts_with("🚫 ungültige Schalter"), "{b:?}: {msg:?}");
        assert!(!quiet, "{b:?}");
        assert_eq!(finish(res, quiet), 1, "{b:?}");
    }
}
