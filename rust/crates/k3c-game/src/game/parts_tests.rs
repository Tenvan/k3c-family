//! Port von `native/k3c-game/game/parts_test.go`. Abweichungen: Rollen stehen als Felder des `Part` (kein Typ-Test);
//! der Go-Fall „Drawer nil“ entfällt (ein Trait-Objekt kann nicht fehlen); `Part` ist nicht `Clone`, jeder Fall baut
//! `packages()` neu.

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use k3c_platform::{Canvas, NullCanvas};
use k3c_sim::state::World;

use super::*;
use crate::game::imports_tests::FACHMODULE;
use crate::game::{Libs, packages};
use crate::view::cell::{Camera, Cell};
use crate::view::frame::Frame;
use crate::view::input::Input;
use crate::view::layer::{Drawer, Layer, Listener, layers};
use crate::view::roles::{AssetSource, CameraRule, Overlay, OverlayStep};
use crate::view::session::Session;
use crate::view::voyage::{Voyage, VoyageDrawer};

fn base() -> Vec<Part> {
    packages(&Libs::default(), None)
}

fn src_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Jedes Fachmodul hat genau einen Eintrag und seinen Modulordner.
#[test]
fn every_package_registered() {
    let ps = base();
    let mut names: Vec<&str> = ps.iter().map(|p| p.name.as_str()).collect();
    names.sort_unstable();
    assert_eq!(names, FACHMODULE, "Pakete");
    for name in FACHMODULE {
        assert!(src_dir().join(name).is_dir(), "{name}: Modulordner fehlt");
    }
}

#[test]
fn assemble_base() {
    let p = assemble(base()).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(p.names.len(), FACHMODULE.len());
    assert!(!p.listeners.is_empty(), "Listener fehlen");
    assert!(p.camera.is_some(), "Kamera-Regel fehlt");
    assert!(p.voyage.is_some(), "Schiffsfahrt fehlt");
    for w in p.drawers.windows(2) {
        assert!(
            w[0].layer() <= w[1].layer(),
            "Drawer nicht nach Ebene sortiert: {} vor {}",
            w[0].layer(),
            w[1].layer()
        );
    }
    // Die Module decken alle Zellen-Ebenen ab (Overlay gehört der Schleife: FPS)
    for l in layers() {
        assert!(
            !l.per_cell() || p.drawers.iter().any(|d| d.layer() == l),
            "keine Zeichner auf Ebene {l}"
        );
    }
}

struct FakeDrawer(Layer, &'static str, Rc<RefCell<Vec<&'static str>>>);

impl Drawer for FakeDrawer {
    fn layer(&self) -> Layer {
        self.0
    }
    fn draw(&self, _: &mut dyn Canvas, _: &Frame, _: &Cell) {
        self.2.borrow_mut().push(self.1);
    }
}

struct Tag(&'static str, Rc<RefCell<Vec<&'static str>>>);

impl Listener for Tag {
    fn listen(&mut self, _: &Frame) {
        self.1.borrow_mut().push(self.0);
    }
}

/// Die Reihenfolge ist die von Go: Drawer nach Ebene (stabil), Listener je Beitrag erst die der Drawer, dann der
/// eigene.
#[test]
fn assemble_order() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let d = |l, n| -> Box<dyn Drawer> { Box::new(FakeDrawer(l, n, Rc::clone(&log))) };
    let t = |n| -> Box<dyn Listener> { Box::new(Tag(n, Rc::clone(&log))) };
    let ps = vec![
        Part {
            drawers: vec![d(Layer::HUD, "hud1"), d(Layer::FAR, "far1")],
            drawer_listeners: vec![t("a-drawer")],
            listener: Some(t("a-own")),
            ..Part::named("a")
        },
        Part {
            drawers: vec![d(Layer::FAR, "far2")],
            listener: Some(t("b-own")),
            ..Part::named("b")
        },
    ];
    let mut base = base();
    base.retain(|p| p.source.is_some() || p.menus.is_some());
    let mut all = ps;
    all.extend(base);
    let mut p = assemble(all).unwrap_or_else(|e| panic!("{e}"));
    let (cell, f, mut canvas) = (Cell::default(), Frame::default(), NullCanvas::default());
    for dr in &p.drawers {
        dr.draw(&mut canvas, &f, &cell);
    }
    assert_eq!(*log.borrow(), ["far1", "far2", "hud1"], "Ebenen-Reihenfolge");
    log.borrow_mut().clear();
    for l in &mut p.listeners {
        l.listen(&f);
    }
    assert_eq!(*log.borrow(), ["a-drawer", "a-own", "b-own"], "Listener-Reihenfolge");
}

struct RolePlayer;

impl Listener for RolePlayer {
    fn listen(&mut self, _: &Frame) {}
}

impl CameraRule for RolePlayer {
    fn follow(&mut self, _: &Frame, c: &Cell, _: Camera) -> Camera {
        c.camera
    }
}

impl AssetSource for RolePlayer {
    fn assets(&self, _: &str, _: &World) -> Vec<String> {
        Vec::new()
    }
}

impl Overlay for RolePlayer {
    fn update(&mut self, _: &Input, _: &Frame, _: &mut dyn Session) -> OverlayStep {
        OverlayStep::default()
    }
}

impl VoyageDrawer for RolePlayer {
    fn voyage_ready(&self, _: &Frame, _: &Voyage) -> bool {
        true
    }
    fn draw_voyage(&self, _: &mut dyn Canvas, _: &Frame, _: &Voyage) -> bool {
        true
    }
}

struct BadDrawer;

impl Drawer for BadDrawer {
    fn layer(&self) -> Layer {
        Layer(77)
    }
    fn draw(&self, _: &mut dyn Canvas, _: &Frame, _: &Cell) {}
}

fn with(mut ps: Vec<Part>, extra: Part) -> Vec<Part> {
    ps.push(extra);
    ps
}

fn clone_role(ps: &mut Vec<Part>, pick: fn(&mut Part) -> bool) {
    // Go hängt eine Kopie des Beitrags an; Rust baut dasselbe Modul ein zweites Mal
    let mut again = base();
    again.retain_mut(pick);
    ps.extend(again);
}

/// `assemble` lehnt ab: ungültige Ebene, zweite Eingabe, zweites Menü, fehlende Eingabe oder fehlendes Menü, zweite
/// Kamera-Regel, zweite Schiffsfahrt. Ein Drawer mit Listener hört mit (genau ein Listener mehr).
#[test]
fn assemble_rejects() {
    let err = |ps: Vec<Part>| assemble(ps).err().unwrap_or_default();
    let bad = Part {
        drawers: vec![Box::new(BadDrawer)],
        ..Part::named("kaputt")
    };
    assert_eq!(err(with(base(), bad)), "❌ kaputt: Drawer ohne gültige Ebene");
    let mut second_menu = base();
    clone_role(&mut second_menu, |p| p.menus.is_some());
    assert!(err(second_menu).contains("zweites Menü"), "zweites Menü angenommen");
    let mut second_input = base();
    clone_role(&mut second_input, |p| p.source.is_some());
    assert!(
        err(second_input).contains("zweite Eingabe"),
        "zweite Eingabe angenommen"
    );
    let mut no_menu = base();
    no_menu.retain(|p| p.menus.is_none());
    assert_eq!(err(no_menu), "❌ Eingabe-Quelle oder Menü fehlt");
    let mut no_input = base();
    no_input.retain(|p| p.source.is_none());
    assert_eq!(err(no_input), "❌ Eingabe-Quelle oder Menü fehlt");
    let camera = Part {
        camera: Some(Box::new(RolePlayer)),
        ..Part::named("welt2")
    };
    assert_eq!(err(with(base(), camera)), "❌ welt2: zweite Kamera-Regel");
    let voyage = Part {
        voyage: Some(Box::new(RolePlayer)),
        ..Part::named("hud2")
    };
    assert_eq!(err(with(base(), voyage)), "❌ hud2: zweite Schiffsfahrt");

    let p0 = assemble(base()).map(|p| p.listeners.len()).unwrap_or(0);
    let log = Rc::new(RefCell::new(Vec::new()));
    let fx2 = Part {
        drawers: vec![Box::new(FakeDrawer(Layer::EFFECTS, "fx2", Rc::clone(&log)))],
        drawer_listeners: vec![Box::new(Tag("fx2", log))],
        ..Part::named("fx2")
    };
    let p = assemble(with(base(), fx2)).map(|p| p.listeners.len());
    assert_eq!(p, Ok(p0 + 1), "Drawer mit Listener hört nicht mit");
}

/// Genau ein Beitrag der echten Module führt die Kamera (world); jede Rolle wird angemeldet. Ohne die Kamera-Regel von
/// world übernimmt die eines anderen Beitrags.
#[test]
fn assemble_roles() {
    let mut ps = base();
    assert_eq!(
        ps.iter().filter(|p| p.camera.is_some()).count(),
        1,
        "Kamera-Regeln in den Modulen"
    );
    let assets: usize = ps.iter().map(|p| p.assets.len()).sum();
    let overlays: usize = ps.iter().map(|p| p.overlays.len()).sum();
    assert!(assets >= 5 && overlays == 2, "Assets {assets}, Overlays {overlays}");
    let p = assemble(ps).unwrap_or_else(|e| panic!("{e}"));
    assert!(p.camera.is_some());
    assert_eq!((p.assets.len(), p.overlays.len()), (assets, overlays));

    ps = base();
    for part in &mut ps {
        part.camera = None;
    }
    let extra = Part {
        listener: Some(Box::new(RolePlayer)),
        camera: Some(Box::new(RolePlayer)),
        assets: vec![Box::new(RolePlayer)],
        overlays: vec![Box::new(RolePlayer)],
        ..Part::named("hud2")
    };
    let p = assemble(with(ps, extra)).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(
        (p.assets.len(), p.overlays.len()),
        (assets + 1, overlays + 1),
        "Rollen mit RolePlayer"
    );
    assert!(p.camera.is_some(), "Kamera-Regel des RolePlayer");
}
