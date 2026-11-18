//! Tests aus `options_test.go`. Die Optionen-Logik (`island_options`, `set_options`, `live_grade`) läuft direkt; `create`
//! mit neuer Insel und das Laden brauchen `create_in` und `load_in` (RU24).
use k3c_sim::island::set_options;

use super::Options;
use super::testutil::{island, manager};

// Name, Dev-Mode, Optionen, erwartet Grad, Ziel, Niederlage (Grad leer = bad_request).
type Case = (&'static str, bool, Options, &'static str, &'static str, &'static str);

fn o(grade: &str, goal: &str, defeat: &str) -> Options {
    Options {
        grade: grade.into(),
        goal: goal.into(),
        defeat: defeat.into(),
        ..Options::default()
    }
}

// Prüft die Optionen: Standards je Modus, gültige Werte, ungültige und dev ohne Dev-Mode -> bad_request.
#[test]
fn anlegen_prueft_optionen() {
    let cases: [Case; 9] = [
        (
            "live-standard",
            false,
            Options::default(),
            "normal",
            "endboss",
            "resources",
        ),
        ("dev-standard", true, Options::default(), "dev", "endboss", "resources"),
        ("gueltig", false, o("hard", "gold", "stage"), "hard", "gold", "stage"),
        (
            "grad-bestimmt-niederlage",
            false,
            o("ultra", "", ""),
            "ultra",
            "endboss",
            "stage",
        ),
        ("dev-im-devmode", true, o("dev", "", ""), "dev", "endboss", "resources"),
        ("dev-ohne-devmode", false, o("dev", "", ""), "", "", ""),
        ("unbekannter-grad", true, o("mittel", "", ""), "", "", ""),
        ("unbekanntes-ziel", true, o("", "frieden", ""), "", "", ""),
        ("unbekannte-niederlage", true, o("", "", "egal"), "", "", ""),
    ];
    for (name, dev, opts, grade, goal, defeat) in cases {
        let (mut m, _) = manager();
        m.dev = dev;
        let mut isl = island("optionen", 0);
        let res = set_options(&mut isl, m.island_options(&opts), m.dev);
        if grade.is_empty() {
            assert!(res.is_err(), "{name}: erwartet bad_request");
            continue;
        }
        assert!(res.is_ok(), "{name}: {res:?}");
        let got = &isl.options;
        assert_eq!(
            (got.grade.as_str(), got.goal.as_str(), got.defeat.as_str()),
            (grade, goal, defeat),
            "{name}"
        );
    }
}

// Ein Stand mit Grad dev wird im Live-Modus auf normal gesetzt und geloggt; im Dev-Mode bleibt er dev.
#[test]
fn dev_stand_im_live_modus() {
    let (mut m, _) = manager();
    let mut isl = island("dev-stand", 0);
    isl.options.grade = "dev".into();
    m.live_grade(&mut isl, "dev-stand");
    assert_eq!(isl.options.grade, "normal");
    assert!(
        m.log.lines.iter().any(|l| l.contains("🐛") && l.contains("dev-stand")),
        "{:?}",
        m.log.lines
    );
    let (mut m, _) = manager();
    m.dev = true;
    let mut isl = island("dev-stand", 0);
    isl.options.grade = "dev".into();
    m.live_grade(&mut isl, "dev-stand");
    assert_eq!(isl.options.grade, "dev");
    assert!(m.log.lines.is_empty());
}

// Manager::create mit neuer Insel: Raum trotz Fehler nicht angelegt, Grad in der Raumliste.
#[test]
#[ignore = "wartet auf RU24"]
fn anlegen_prueft_optionen_ueber_create() {}

// create, close, create aus dem Stand: die Optionen aus dem Stand gelten.
#[test]
#[ignore = "wartet auf RU24"]
fn optionen_ueberleben_speichern_und_laden() {}
