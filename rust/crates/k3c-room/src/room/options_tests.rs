//! Tests aus `options_test.go`. Die Optionen-Logik (`island_options`, `set_options`, `live_grade`) läuft direkt; `create`
//! mit neuer Insel prüft `anlegen_prueft_optionen_ueber_create`, Speichern und Laden läuft über den Manager.
use k3c_sim::island::set_options;

use super::Options;
use super::save_testutil::create_with;
use super::testutil::{island, manager, peer, rm};
use crate::fehler::Fehler;

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
fn anlegen_prueft_optionen_ueber_create() {
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
        let (p, _inbox) = peer(1);
        let res = m.create("x", p, "optionen", true, 0, &[0], opts);
        if grade.is_empty() {
            assert_eq!(res, Err(Fehler::BadRequest), "{name}: erwartet bad_request");
            assert!(m.rooms().is_empty(), "{name}: Raum trotz Fehler angelegt");
            continue;
        }
        let code = res.expect(name);
        let got = &rm(&mut m, &code).isl.options;
        assert_eq!(
            (got.grade.as_str(), got.goal.as_str(), got.defeat.as_str()),
            (grade, goal, defeat),
            "{name}: Optionen"
        );
        assert_eq!(m.rooms()[0].grade, grade, "{name}: Grad in der Raumliste");
    }
}

// create, close, create aus dem Stand: die Optionen aus dem Stand gelten, die aus `create` werden ignoriert.
#[test]
fn optionen_ueberleben_speichern_und_laden() {
    let (mut f, _) = manager();
    create_with(&mut f, "x", 1, "stand", true, o("hard", "days", "stage"));
    f.close();
    let (mut g, _) = manager();
    g.store = std::mem::take(&mut f.store);
    let (code, _xi) = create_with(&mut g, "x", 2, "stand", false, o("easy", "gold", ""));
    let opts = &rm(&mut g, &code).isl.options;
    assert_eq!(
        (opts.grade.as_str(), opts.goal.as_str(), opts.defeat.as_str()),
        ("hard", "days", "stage"),
        "Optionen nach dem Laden"
    );
}
