//! Port von `native/k3c-game/game/fault_test.go` (`TestMissingGraphicsIsFault`, `TestFailMeasureKeepsExit`,
//! `TestFaultTitleAndHint`) und der Beenden-Tasten.

use std::rc::Rc;

use k3c_platform::{InputState, Key, NullCanvas, PadButton};

use super::*;
use crate::art::ArtError;
use crate::game::levels::levels_tests::{AssetsOf, FakeLoader, game_with, stages};
use crate::game::testutil::{FakeSim, parts};
use crate::game::{Args, Mode};
use crate::view::roles::AssetSource;
use crate::view::screen::StepKind;

fn plain_game(args: Args) -> Game {
    let (sim, _) = FakeSim::new(2);
    let p = parts(StepKind::Stay, &Rc::default(), Vec::new());
    Game::new(p, Box::new(sim), args).unwrap_or_else(|e| unreachable!("{e}"))
}

// AC-01 (B-810): Fehlende Grafik liefert einen Fehlerzustand mit Text und Pfad, kein stilles Ende: `err` bleibt leer,
// `update` beendet nicht, `fault()` nennt den Fehler; Enter beendet mit dem Fehler (Exit 1 in `main`).
#[test]
fn missing_graphics_is_fault() {
    let ld = FakeLoader::default();
    ld.fail.set(true);
    let src: Vec<Box<dyn AssetSource>> = vec![Box::new(AssetsOf::of(&[("wald", &["natur/baum"])]))];
    let mut g = game_with(Box::new(ld), src, Args::default());
    g.frame.stages = stages(&["wald"]);
    g.load.menu_art = false;
    g.load_levels();
    g.run_pending_load(&mut NullCanvas::default());
    assert!(g.err.is_none(), "err gesetzt: {:?}", g.err);
    let msg = g.fault().map(str::to_string).unwrap_or_default();
    assert!(msg.contains("fehlende Grafik"), "Fehlerzustand fehlt: {msg:?}");
    let lines = g.fault.as_ref().map(|f| f.lines().to_vec()).unwrap_or_default();
    assert!(
        lines.first().is_some_and(|l| l.contains("Grafik fehlt")),
        "Meldung ohne Titel: {lines:?}"
    );
    g.update(&InputState::new()); // im Fehlerzustand ohne Taste: kein Ende
    assert_ne!(g.mode(), Mode::Quit, "Fehlerzustand beendet ohne Taste");
    let mut raw = InputState::new();
    raw.keys.press(Key::Enter);
    g.update(&raw);
    assert_eq!(g.mode(), Mode::Quit, "Enter beendet nicht");
    assert!(g.fault().is_some(), "Fehler nach dem Beenden weg");
}

// Das Fehlerfenster zeigt die Meldung mit dem Pfad und dem Hinweis, Enter, Esc oder A beendet (Test gegen `NullCanvas`).
#[test]
fn fault_shows_path_and_exits() {
    let path = "public/grafik/natur/baum/manifest.json";
    let f = Fault::missing(format!("fehlende Grafik: Manifest {path}"));
    let all = f.lines().join("").replace(' ', "");
    assert!(all.contains(path), "Pfad fehlt: {:?}", f.lines());
    assert!(f.lines().last().is_some_and(|l| l.contains("Log")), "Hinweis fehlt");
    f.draw(&mut NullCanvas::default());
    let mut none = InputState::new();
    assert!(!f.update(&none));
    for k in [Key::Enter, Key::NumpadEnter, Key::Escape, Key::Space] {
        let mut raw = InputState::new();
        raw.keys.press(k);
        assert!(f.update(&raw), "{k:?} beendet nicht");
    }
    for b in [PadButton::A, PadButton::B] {
        let mut raw = InputState::new();
        raw.pad_mut(1).set_button(b, true);
        assert!(f.update(&raw), "{b:?} beendet nicht");
    }
    none.pad_mut(1).set_button(PadButton::X, true);
    assert!(!f.update(&none), "X beendet");
}

// Im Messmodus beendet der Fehler das Programm über `update` (kein Fenster für Fehler); ein erster Fehler bleibt stehen.
#[test]
fn fail_measure_keeps_exit() {
    let mut g = plain_game(Args::default());
    g.args.measure.seconds = 5.0;
    g.fail(&ArtError::Broken("a".to_string()));
    g.fail(&ArtError::Broken("b".to_string()));
    assert!(
        g.err.as_deref() == Some("a") && g.fault.is_none(),
        "Messmodus: err {:?}",
        g.err
    );
    assert_eq!(g.fault(), Some("a"), "main bekommt den Fehler");
    assert!(g.err_exit() && g.mode() == Mode::Quit, "Messmodus beendet nicht");
    let mut f = plain_game(Args::default());
    f.fail(&ArtError::Broken("erster".to_string()));
    f.fail(&ArtError::Broken("zweiter".to_string()));
    assert_eq!(f.fault(), Some("erster"), "erster Fehler bleibt");
}

#[test]
fn fault_title_and_hint() {
    let other = Fault::new("Fenster zu klein");
    assert!(other.lines()[0].starts_with("Fehler"), "Titel: {:?}", other.lines()[0]);
    let all = other.lines().join("\n");
    assert!(
        all.contains("Fenster zu klein") && all.contains("beendet das Spiel"),
        "Text: {all:?}"
    );
}

// Ein zusammengesetzter Fehler zeigt die erste Meldung und die Zahl der weiteren; mehr als 12 Zeilen kürzt „...“.
#[test]
fn fault_joined_and_capped() {
    let f = Fault::new("eins\nzwei\n\ndrei");
    let all = f.lines().join("\n");
    assert!(all.contains("eins") && !all.contains("zwei"), "erste Meldung: {all:?}");
    assert!(
        all.contains("(und 2 weitere, alle im Log)"),
        "Zahl der weiteren: {all:?}"
    );
    let long = Fault::new("wort ".repeat(400));
    assert!(
        long.lines().iter().any(|l| l == "..."),
        "nicht gekürzt: {:?}",
        long.lines().len()
    );
    assert!(long.lines().len() <= FAULT_MAX_LINES + 3, "zu viele Zeilen");
}
