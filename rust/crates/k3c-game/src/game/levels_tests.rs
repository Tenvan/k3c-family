//! Port von `native/k3c-game/game/levels_test.go`; die Hilfen (`stages`, `AssetsOf`, `FakeLoader`, `load_game`) teilt
//! `loading_tests.rs`. Der Loader lädt in Rust im Zeichen-Takt: `run_pending_load` mit `NullCanvas` ersetzt den Aufruf
//! von `loadLevels` im Takt (Go lud in `Update`).

use std::cell::{Cell as Counter, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use k3c_platform::NullCanvas;
use k3c_sim::state::World;

use super::*;
use crate::art::ArtError;
use crate::game::testutil::{FakeSim, parts};
use crate::game::{Args, Mode};
use crate::menu;
use crate::view::screen::StepKind;

/// Stufen mit den gegebenen Leveln (Go: `stages`).
pub fn stages(levels: &[&str]) -> Vec<Stage> {
    levels
        .iter()
        .enumerate()
        .map(|(i, lv)| Stage {
            index: i as i64,
            level: (*lv).to_string(),
            world: Some(Rc::new(World::default())),
            ..Stage::default()
        })
        .collect()
}

/// Asset-Quelle: je Level feste Assets (Go: `assetsOf`).
pub struct AssetsOf(pub BTreeMap<String, Vec<String>>);

impl AssetsOf {
    pub fn of(list: &[(&str, &[&str])]) -> AssetsOf {
        AssetsOf(
            list.iter()
                .map(|(lv, a)| ((*lv).to_string(), a.iter().map(|s| (*s).to_string()).collect()))
                .collect(),
        )
    }
}

impl AssetSource for AssetsOf {
    fn assets(&self, level: &str, _w: &World) -> Vec<String> {
        self.0.get(level).cloned().unwrap_or_default()
    }
}

/// Loader, der die Aufrufe von `set_levels` merkt; `fail` lässt ihn scheitern (Go: `loader`).
#[derive(Clone, Default)]
pub struct FakeLoader {
    pub calls: Rc<RefCell<Vec<Levels>>>,
    pub fail: Rc<Counter<bool>>,
}

impl Loader for FakeLoader {
    fn set_levels(&mut self, want: &Levels, _c: &mut dyn Canvas) -> Result<(), ArtError> {
        self.calls.borrow_mut().push(want.clone());
        if self.fail.get() {
            return Err(ArtError::Missing("fehlende Grafik".to_string()));
        }
        Ok(())
    }

    fn usable(&self, _asset: &str) -> bool {
        false
    }
}

/// Eine Schleife mit dem Loader und den Quellen; das Laden der Menü-Illustrationen aus `Game::new` ist schon erledigt.
pub fn game_with(loader: Box<dyn Loader>, sources: Vec<Box<dyn AssetSource>>, args: Args) -> Game {
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut p = parts(StepKind::Stay, &log, Vec::new());
    p.art = Some(loader);
    p.assets = sources;
    let (sim, _) = FakeSim::new(2);
    let mut g = Game::new(p, Box::new(sim), args).unwrap_or_else(|e| unreachable!("{e}"));
    g.run_pending_load(&mut NullCanvas::default());
    g
}

#[test]
fn same_levels_faelle() {
    let have = |l: &[&str]| l.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
    let cases = [
        (have(&[]), stages(&[]), true),
        (have(&[]), stages(&["wald"]), false),
        (have(&["wald"]), stages(&["wald", "wald"]), true),
        (have(&["wald"]), stages(&["wald", "mine"]), false),
        (have(&["mine", "wald"]), stages(&["wald"]), false),
        (have(&["mine", "wald"]), stages(&["wald", "mine"]), true),
    ];
    for (i, (h, st, want)) in cases.iter().enumerate() {
        assert_eq!(same_levels(h, st), *want, "Fall {i}");
    }
}

/// Die Vereinigung aller Quellen je Level, sortiert und ohne Doppelte; zwei Spieler in zwei Leveln laden beide.
#[test]
fn wanted_assets_vereinigt() {
    let srcs: Vec<Box<dyn AssetSource>> = vec![
        Box::new(AssetsOf::of(&[
            ("wald", &["natur/baum", "figuren/zwerg"]),
            ("mine", &["natur/stein"]),
        ])),
        Box::new(AssetsOf::of(&[("wald", &["figuren/zwerg", "gebaeude/burg"])])),
    ];
    let got = wanted_assets(&stages(&["wald", "mine", "wald"]), &srcs);
    let want: Levels = BTreeMap::from([
        (
            "wald".to_string(),
            vec![
                "figuren/zwerg".to_string(),
                "gebaeude/burg".to_string(),
                "natur/baum".to_string(),
            ],
        ),
        ("mine".to_string(), vec!["natur/stein".to_string()]),
    ]);
    assert_eq!(got, want);
}

/// Die Schleife lädt nur, wenn sich die Level ändern; ein Fehler (fehlende Grafik) beendet hier das Spiel (RU30.3: Fenster, B-810).
#[test]
fn load_levels_nur_bei_aenderung() {
    let ld = FakeLoader::default();
    let src: Vec<Box<dyn AssetSource>> = vec![Box::new(AssetsOf::of(&[("wald", &["a/b"])]))];
    let mut g = game_with(Box::new(ld.clone()), src, Args::default());
    g.load.menu_art = false;
    g.frame.stages = stages(&["wald"]);
    ld.calls.borrow_mut().clear();
    g.load_levels();
    g.load_levels();
    g.run_pending_load(&mut NullCanvas::default());
    assert_eq!(ld.calls.borrow().len(), 1);
    assert_eq!(g.load.levels, vec!["wald".to_string()]);

    g.frame.stages = Vec::new();
    g.load_levels();
    g.load_levels();
    g.run_pending_load(&mut NullCanvas::default());
    let calls = ld.calls.borrow().clone();
    let menu_want = Levels::from([(MENU_LEVEL.to_string(), menu::art::art(None))]);
    assert_eq!(calls.len(), 2, "ins Menü: nur Illustrationen, einmal (B-818)");
    assert_eq!(calls[1], menu_want);
    assert!(g.load.levels.is_empty());

    ld.fail.set(true);
    g.load.menu_art = false;
    g.load_levels();
    g.run_pending_load(&mut NullCanvas::default());
    assert_eq!(ld.calls.borrow().len(), 3);
    assert_eq!(
        g.mode(),
        Mode::Menu,
        "fehlende Menü-Illustration beendet das Menü nicht"
    );

    g.frame.stages = stages(&["mine"]);
    g.load_levels();
    g.run_pending_load(&mut NullCanvas::default());
    assert_eq!(g.mode(), Mode::Quit, "Fehler nicht gemeldet");
    assert!(g.load.levels.is_empty());
}
