//! Port von `native/k3c-game/game/loading_test.go`. Das Hochladen läuft in Rust im Zeichen-Takt: wo Go nach dem
//! Takt `calls` zählt, ruft der Test `run_pending_load` mit `NullCanvas` (Entscheidung im Takt, Laden in `draw`).

use std::cell::{Cell as Counter, RefCell};
use std::rc::Rc;

use k3c_platform::NullCanvas;

use super::*;
use crate::art::ArtError;
use crate::game::Args;
use crate::game::levels::Loader;
use crate::game::levels::levels_tests::{AssetsOf, FakeLoader, game_with, stages};
use crate::view::roles::AssetSource;

fn sources() -> Vec<Box<dyn AssetSource>> {
    vec![Box::new(AssetsOf::of(&[("wald", &["a/b"]), ("mine", &["c/d"])]))]
}

/// Eine Schleife im Fenster mit einem Loader, der die Aufrufe zählt.
fn load_game() -> (Game, FakeLoader) {
    let ld = FakeLoader::default();
    let g = game_with(Box::new(ld.clone()), sources(), Args::default());
    ld.calls.borrow_mut().clear();
    (g, ld)
}

fn draw_pending(g: &mut Game) {
    g.run_pending_load(&mut NullCanvas::default());
}

/// AC-02 (B-810): Ein Stufenwechsel setzt zuerst „lädt“ und lädt nicht; erst nach einem gezeichneten Bild mit der
/// Anzeige lädt das folgende Bild, danach ist „lädt“ vorbei.
#[test]
fn loading_before_load() {
    let (mut g, ld) = load_game();
    g.frame.stages = stages(&["wald"]);
    g.sync_levels();
    draw_pending(&mut g);
    assert!(g.loading() && ld.calls.borrow().is_empty(), "1. Takt");
    g.sync_levels(); // draw hat noch nichts gezeichnet: weiter warten
    draw_pending(&mut g);
    assert!(g.loading() && ld.calls.borrow().is_empty(), "ohne Bild geladen");
    let msg = g.load.msg.as_ref().map(|m| m.lines.clone()).unwrap_or_default();
    assert!(msg.contains(&"Stufe: wald".to_string()), "Text der Anzeige: {msg:?}");
    g.load.shown = true; // draw hat die Anzeige gezeichnet (draw_loading)
    g.sync_levels();
    draw_pending(&mut g);
    assert!(!g.loading() && ld.calls.borrow().len() == 1, "nach dem Bild");
    assert_eq!(g.load.levels, vec!["wald".to_string()]);
    g.sync_levels(); // nichts mehr zu tun
    draw_pending(&mut g);
    assert!(!g.loading() && ld.calls.borrow().len() == 1, "ohne Wechsel");
}

/// Ein weiterer Stufenwechsel (andere Level) durchläuft die Anzeige erneut.
#[test]
fn loading_again_on_next_change() {
    let (mut g, ld) = load_game();
    g.frame.stages = stages(&["wald"]);
    g.sync_levels();
    g.load.shown = true;
    g.sync_levels();
    draw_pending(&mut g);
    g.frame.stages = stages(&["mine"]);
    g.sync_levels();
    draw_pending(&mut g);
    assert!(
        g.loading() && !g.load.shown && ld.calls.borrow().len() == 1,
        "zweiter Wechsel"
    );
}

/// Aufnahme- und Messmodus laden sofort (bildgleich wie vor B-810); `--stall` hält die Anzeige dauerhaft.
#[test]
fn loading_recording_synchronous() {
    // `shots` erst nach `new` setzen: `Game::new` baut mit ihm den Aufnahme-Modus (RU30.2)
    let ld = FakeLoader::default();
    let mut g = game_with(Box::new(ld.clone()), sources(), Args::default());
    g.args.shots = "x".to_string();
    ld.calls.borrow_mut().clear();
    g.frame.stages = stages(&["wald"]);
    g.sync_levels();
    draw_pending(&mut g);
    assert!(!g.loading() && ld.calls.borrow().len() == 1, "Aufnahme");

    let sl = FakeLoader::default();
    let mut s = game_with(Box::new(sl.clone()), sources(), Args::default());
    (s.args.shots, s.args.stall) = ("x".to_string(), true);
    sl.calls.borrow_mut().clear();
    s.frame.stages = stages(&["wald"]);
    s.sync_levels();
    s.sync_levels();
    draw_pending(&mut s);
    assert!(s.loading() && sl.calls.borrow().is_empty(), "--stall");

    let ml = FakeLoader::default();
    let mut m = game_with(Box::new(ml.clone()), sources(), Args::default());
    m.args.measure.seconds = 5.0;
    ml.calls.borrow_mut().clear();
    m.frame.stages = stages(&["wald"]);
    m.sync_levels();
    draw_pending(&mut m);
    assert!(!m.loading() && ml.calls.borrow().len() == 1, "Messmodus");
}

/// Ohne Grafik-Bibliothek (Tests, Platzhalter) gibt es nichts zu laden und keine Anzeige.
#[test]
fn loading_without_art() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let (sim, _) = crate::game::testutil::FakeSim::new(2);
    let p = crate::game::testutil::parts(crate::view::screen::StepKind::Stay, &log, Vec::new());
    let mut g = Game::new(p, Box::new(sim), Args::default()).unwrap_or_else(|e| unreachable!("{e}"));
    g.frame.stages = stages(&["wald"]);
    g.sync_levels();
    assert!(!g.loading(), "lädt ohne Bibliothek");
}

/// Schritt-Loader: lädt in Schritten zu `LOAD_BATCH` Sheets je Bild, erst nach dem gezeichneten Ladebild; bricht ab,
/// wenn die Stufen währenddessen wechseln.
struct StepLoader {
    steps: Rc<Counter<usize>>,
    sizes: Rc<RefCell<Vec<usize>>>,
    begun: Rc<Counter<usize>>,
}

impl Loader for StepLoader {
    fn set_levels(&mut self, _want: &Levels, _c: &mut dyn Canvas) -> Result<(), ArtError> {
        Ok(())
    }

    fn begin_levels(&self, _want: &Levels) -> Option<Result<LevelLoad, ArtError>> {
        self.begun.set(self.begun.get() + 1);
        Some(Ok(LevelLoad::default()))
    }

    fn step(&mut self, _ld: &mut LevelLoad, n: usize, _c: &mut dyn Canvas) -> Result<bool, ArtError> {
        self.steps.set(self.steps.get() + 1);
        self.sizes.borrow_mut().push(n);
        Ok(self.steps.get() >= 2)
    }
}

#[test]
fn loading_in_steps() {
    let (steps, sizes, begun) = (
        Rc::new(Counter::new(0)),
        Rc::new(RefCell::new(Vec::new())),
        Rc::new(Counter::new(0)),
    );
    let ld = StepLoader {
        steps: Rc::clone(&steps),
        sizes: Rc::clone(&sizes),
        begun: Rc::clone(&begun),
    };
    let mut g = game_with(Box::new(ld), sources(), Args::default());
    g.frame.stages = stages(&["wald"]);
    g.sync_levels();
    draw_pending(&mut g);
    assert!(g.loading() && steps.get() == 0, "Anzeige zuerst");
    g.load.shown = true;
    g.sync_levels();
    assert!(g.load.lload.is_some() && begun.get() == 1, "geplant");
    draw_pending(&mut g);
    assert!(g.loading() && steps.get() == 1, "erster Schritt");
    g.frame.stages = stages(&["mine"]);
    g.sync_levels(); // veraltet: von vorn mit den neuen Leveln
    assert_eq!(begun.get(), 2);
    assert_eq!(g.load.dropped.len(), 1);
    draw_pending(&mut g);
    draw_pending(&mut g);
    assert!(!g.loading() && g.load.lload.is_none(), "fertig");
    assert_eq!(g.load.levels, vec!["mine".to_string()]);
    assert!(sizes.borrow().iter().all(|n| *n == LOAD_BATCH));
}

/// Review FB4: Wechseln die Stufen während des schrittweisen Ladens (drittes Level), gilt das laufende Laden als veraltet.
#[test]
fn stale_load_faelle() {
    let want: Levels = Levels::from([("wald".to_string(), Vec::new())]);
    let cases = [
        ("gleich", stages(&["wald"]), false),
        ("drittes Level", stages(&["mine"]), true),
        ("zusätzliches Level", stages(&["wald", "mine"]), true),
        ("keine Stufen", stages(&[]), true),
    ];
    for (name, st, expect) in cases {
        assert_eq!(stale_load(&want, &st), expect, "{name}");
    }
}
