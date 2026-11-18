//! Port von `native/k3c-game/game/stack_test.go`.

use super::*;
use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::view::input::Input;
use crate::view::session::Session;
use k3c_platform::Canvas;

/// Dummy-Screen, der seine Nummer meldet (Go: `named`).
struct Named(#[allow(dead_code)] &'static str);

impl Screen for Named {
    fn update(&mut self, _input: &Input, _s: &mut dyn Session) -> Step {
        Step::default()
    }
    fn draw(&self, _dst: &mut dyn Canvas, _f: &Frame, _c: &Cell) {}
}

/// Legt einen Dummy-Screen an; der Test erkennt ihn über `top_id` (`top` liefert nur den Trait, nicht den Namen).
fn mk(name: &'static str) -> Box<dyn Screen> {
    Box::new(Named(name))
}

fn step(kind: StepKind, next: Option<Box<dyn Screen>>) -> Step {
    Step {
        kind,
        next,
        cues: Vec::new(),
    }
}

/// Zeigerwert (Daten-Adresse) des obersten Screens als Identität.
fn top_id(s: &Stack) -> Option<usize> {
    s.top().map(|t| t as *const dyn Screen as *const () as usize)
}

#[test]
fn stack_apply() {
    let mut s = Stack::new();
    let mut main = || mk("main");
    s.push(mk("main"));
    let main_id = top_id(&s);
    let m = s.apply(step(StepKind::Push, Some(mk("laden"))), false, &mut main);
    let laden_id = top_id(&s);
    assert!(m == Mode::Menu && s.len() == 2 && laden_id != main_id, "Push: {m:?}");
    let m = s.apply(step(StepKind::Replace, Some(mk("löschen"))), false, &mut main);
    let loeschen_id = top_id(&s);
    assert!(
        m == Mode::Menu && s.len() == 2 && loeschen_id != laden_id,
        "Replace: {m:?}"
    );
    let m = s.apply(step(StepKind::Pop, None), false, &mut main);
    assert!(m == Mode::Menu && s.len() == 1 && top_id(&s) == main_id, "Pop: {m:?}");
    let m = s.apply(step(StepKind::Play, None), true, &mut main);
    assert!(m == Mode::Play && s.is_empty(), "Play: {m:?} {}", s.len());
    assert_eq!(s.apply(step(StepKind::Quit, None), true, &mut main), Mode::Quit);
}

/// Ohne laufendes Spiel bleibt nie ein leerer Stapel: Pop des letzten Screens öffnet das Hauptmenü.
#[test]
fn stack_never_empty_without_game() {
    let mut s = Stack::new();
    let mut opened = 0;
    let mut main = || {
        opened += 1;
        mk("main")
    };
    s.push(mk("pause"));
    let m = s.apply(step(StepKind::Pop, None), false, &mut main);
    assert!(
        m == Mode::Menu && s.len() == 1 && s.top().is_some(),
        "leerer Stapel ohne Spiel: {m:?}"
    );
    let m = s.apply(step(StepKind::Push, None), false, &mut main);
    assert!(m == Mode::Menu && s.len() == 1, "Push ohne Next: {m:?} {}", s.len());
    assert_eq!(opened, 1);
}

/// Replace ohne Next oder bei leerem Stapel ändert nichts; Stay lässt den Stapel stehen.
#[test]
fn stack_replace_und_stay_ohne_wirkung() {
    let mut s = Stack::new();
    let mut main = || mk("main");
    s.push(mk("a"));
    let id = top_id(&s);
    assert_eq!(s.apply(step(StepKind::Replace, None), false, &mut main), Mode::Menu);
    assert_eq!(s.apply(step(StepKind::Stay, None), false, &mut main), Mode::Menu);
    assert!(s.len() == 1 && top_id(&s) == id);
    assert!(s.top_mut().is_some());
    // Leerer Stapel mit Spiel: Replace tut nichts, Play bleibt.
    let mut e = Stack::new();
    assert_eq!(
        e.apply(step(StepKind::Replace, Some(mk("x"))), true, &mut main),
        Mode::Play
    );
    assert!(e.is_empty());
}
