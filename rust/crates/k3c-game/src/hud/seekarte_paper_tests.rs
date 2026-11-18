//! Die letzten vier Tests aus `native/k3c-game/hud/seekarte_paper_test.go` (Rückfall ohne Assets, Diorama mit Assets,
//! Lage in der Zelle; der Allokations-Test `TestSeekarteDioramaOhneAllokationen` entfällt: `testing.AllocsPerRun` ist in
//! Rust nicht portierbar, das Frame-Budget misst RU39; ebenso der AllocsPerRun-Block in `sea_diorama_with_assets`). Die Zeichenbefehle selbst misst der Bildvergleich (RU23.6).

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use k3c_platform::NullCanvas;
use k3c_sim::state::World;

use super::Hud;
use super::drawsea::SeaSizes;
use super::drawsea_paper::*;
use super::icons::HUD_ASSETS;
use super::layout::Rect;
use super::seekarte::SeaView;
use super::seekarte_look::SEA_SCALE_MAX;
use super::testutil::{StubView, open_switch, sea_world, set_world};
use crate::art::{Config, Library, STANDARD_WORLD, SeaMap};
use crate::view::cell::{Cell, new_camera, rect};
use crate::view::frame::RouteIsland;
use crate::view::profile::PROFILE_WASD;
use crate::view::roles::AssetSource;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn paper_route() -> Vec<RouteIsland> {
    let isl = |id: &str, etappe, world: &str, stages, chunks| RouteIsland {
        id: id.to_string(),
        name: id.to_uppercase(),
        etappe,
        world: world.to_string(),
        stages,
        chunks,
    };
    vec![
        isl("a", 1, "sengoku", 1, 16),
        isl("b", 2, "sengoku", 3, 40),
        isl("c", 2, "sengoku", 9, 100),
        isl("d", 3, "gibt-es-nicht", 0, 0),
    ]
}

fn assets_of(h: &Hud) -> Vec<String> {
    AssetSource::assets(h, "forest", &World::default())
}

/// AC-03 (B-964/R5): Ohne Grafik, ohne Zuordnung oder mit einem nicht ladbaren Asset bleibt die Karte aus Formen und das
/// HUD meldet nur die Icons.
#[test]
fn sea_fallback_without_assets() {
    let mut v = SeaView::default();
    v.refresh("de", PROFILE_WASD, Some(&open_switch("")), &paper_route());
    let m = SeaMap::from_data().expect("data/paper.json › seekarte nicht lesbar");
    let hud_assets: Vec<String> = HUD_ASSETS.iter().map(|a| (*a).to_string()).collect();
    let mut h = Hud::new(None, Box::new(StubView));
    assert!(h.sea_art_of(&v).is_none(), "Diorama ohne Zuordnung");
    h.use_sea_map(None, &|_| true);
    let ship = m.ship.clone();
    h.use_sea_map(Some(m.clone()), &|a| a != ship);
    assert!(
        h.sea_map.is_none() && assets_of(&h) == hud_assets,
        "Asset nicht ladbar: Zuordnung {}, Assets {:?}",
        h.sea_map.is_some(),
        assets_of(&h)
    );
    h.use_sea_map(Some(m.clone()), &|_| true);
    let got = assets_of(&h);
    assert!(
        h.sea_map.as_ref() == Some(&m)
            && got.is_sorted()
            && got.len() == hud_assets.len() + m.assets().len()
            && got.contains(&m.ship),
        "ladbar: Assets {got:?}"
    );
    assert!(
        h.sea_art_of(&v).is_none(),
        "Zuordnung da, aber keine Bibliothek: Formen"
    );
    assert!(
        Hud::new(None, Box::new(StubView)).sea_map.is_none(),
        "New ohne Grafik nimmt die Seekarten-Assets"
    );
}

/// sea_lib: HUD mit den geladenen Assets aus dem Repo; ohne gerenderte Grafik None (Go: `t.Skip`).
fn sea_lib() -> Option<Hud> {
    let cfg = Config {
        grafik_dir: root().join("public/grafik"),
        font_path: root().join("public/schrift/k3c.ttf"),
        strict_font: true,
        ..Config::default()
    };
    let lib = Rc::new(RefCell::new(Library::new(cfg).ok()?));
    let h = Hud::new(Some(lib.clone()), Box::new(StubView));
    h.sea_map.as_ref()?;
    let want = [("forest".to_string(), assets_of(&h))].into();
    if let Err(e) = lib.borrow_mut().set_levels(&want, &mut NullCanvas::default()) {
        eprintln!("Grafik fehlt (Sheets erst mit `task gen:paper`): {e}");
        return None;
    }
    Some(h)
}

/// AC-02, AC-03 (B-964/R5, R6): Mit geladenen Assets gilt das Diorama, auch für eine Welt ohne eigene Miniatur; fehlt ein
/// Sheet (anderes Level), gelten die Formen. Ohne gerenderte Grafik läuft der Test nicht (wie `t.Skip`).
#[test]
fn sea_diorama_with_assets() {
    let Some(mut h) = sea_lib() else {
        eprintln!("übersprungen: Grafik fehlt");
        return;
    };
    let mut v = SeaView::default();
    v.refresh("de", PROFILE_WASD, Some(&open_switch("")), &paper_route());
    let a = h.sea_art_of(&v).expect("Diorama nicht gewählt");
    assert!(
        a.ship.is_some() && a.flag.is_some() && a.fog.is_some(),
        "Sheets der Karte"
    );
    let (b, m, f) = (a.back.unwrap(), a.mid.unwrap(), a.front.unwrap());
    let tex = |s: &SeaSheet| (s.frame.tex, s.frame.src.w.to_bits());
    assert!(
        tex(&b) != tex(&m) || tex(&m) != tex(&f),
        "Papierschichten des Meers verschieden"
    );
    assert_eq!(a.isl.len(), 4, "Miniatur und Aufsatz je Insel");
    h.sea_isl = a.isl;
    let m = h.sea_map.clone().expect("Zuordnung");
    let lib = h.lib.clone().expect("Bibliothek");
    let (std, own) = (m.island("gibt-es-nicht"), m.island("sengoku"));
    assert!(
        sea_sheet(&lib.borrow(), std).is_some() && std != own && std == m.island(STANDARD_WORLD),
        "Welt ohne Eintrag nutzt nicht die Miniatur von standard"
    );
    let want = [(
        "forest".to_string(),
        HUD_ASSETS.iter().map(|a| (*a).to_string()).collect(),
    )]
    .into();
    lib.borrow_mut()
        .set_levels(&want, &mut NullCanvas::default())
        .expect("Level");
    assert!(h.sea_art_of(&v).is_none(), "Diorama ohne geladene Sheets");
}

/// AC-02 (Split-Screen): Das Diorama bleibt in jeder Zelle, alle Inseln und Texte liegen in der Tafel, Etappen stehen
/// von links nach rechts, Reihen von hinten nach vorn.
#[test]
fn sea_diorama_position() {
    let m = SeaSizes {
        lab_w: 120.0,
        name_w: 200.0,
        title_w: 180.0,
        hint_w: 520.0,
        max_nodes: 4,
        hint_icons: false,
    };
    let cells = [
        (Rect::new(0.0, 0.0, 1920.0, 1080.0), 1.0),
        (Rect::new(0.0, 0.0, 1920.0, 540.0), 1.0),
        (Rect::new(0.0, 540.0, 1920.0, 540.0), 1.0),
        (Rect::new(0.0, 360.0, 1280.0, 360.0), 2.0 / 3.0),
        (Rect::new(0.0, 0.0, 640.0, 360.0), 1.0 / 3.0),
    ];
    for (cell, k) in cells {
        let p = sea_paper_plan(&m, 3, cell, k);
        assert!(
            p.box_.inside(cell) && p.u > 0.0 && p.u <= k,
            "Zelle {cell:?}: Tafel {:?}, u {}",
            p.box_,
            p.u
        );
        assert!(
            p.front + SEA_LAYER_H * p.u <= p.box_.y + p.box_.h + 0.01
                && p.water - SEA_TOP_H * p.u >= p.box_.y + 2.5 * p.b,
            "Zelle {cell:?}: Meer ragt aus der Tafel ({} … {} in {:?})",
            p.water,
            p.front,
            p.box_
        );
        for col in 0..3 {
            for lane in 0..4 {
                let (x, half) = (p.node_x(col, lane, 4), 120.0 * SEA_SCALE_MAX * p.u);
                assert!(
                    x - half >= p.box_.x && x + half <= p.box_.x + p.box_.w,
                    "Zelle {cell:?}: Insel {col}/{lane} bei {x} außerhalb"
                );
                assert!(
                    col == 0 || p.node_x(col, lane, 4) > p.node_x(col - 1, lane, 4),
                    "Etappen nicht von links nach rechts"
                );
            }
        }
        assert!(
            p.water_of(1) > p.water_of(0) && p.front > p.water_of(3),
            "Reihen nicht von hinten nach vorn"
        );
    }
}

fn sea_cell() -> Cell {
    let r = rect(0, 0, 1280, 360);
    Cell {
        index: 0,
        player: 0,
        rect: r,
        camera: new_camera(r, 0.0, 1.0),
    }
}

fn font_lib() -> Option<Rc<RefCell<Library>>> {
    let cfg = Config {
        font_path: root().join("public/schrift/k3c.ttf"),
        strict_font: true,
        ..Config::default()
    };
    Library::new(cfg).ok().map(|l| Rc::new(RefCell::new(l)))
}

/// Ohne Sheets zeichnet die offene Seekarte die Formen (Tafel, Knoten, Marken, Rahmen) mit Text; zweimal gleich.
#[test]
fn sea_draws_shapes_without_assets() {
    let Some(lib) = font_lib() else { return };
    let mut h = Hud::new(Some(lib), Box::new(StubView));
    let (mut w, mut f) = sea_world();
    w.island_switch = Some(open_switch("b"));
    set_world(&mut f, &w);
    h.listen(&f);
    let mut c = NullCanvas::default();
    h.draw_cell(&mut c, &sea_cell());
    assert!(c.texts > 0 && c.rects > 0 && c.shapes > 0, "Formen: {c:?}");
    let (texts, rects) = (c.texts, c.rects);
    h.draw_cell(&mut c, &sea_cell());
    assert_eq!(
        (c.texts, c.rects),
        (texts * 2, rects * 2),
        "gleiches Bild, gleiche Zeichenbefehle"
    );
}

/// Mit geladenen Sheets zeichnet die Karte das Diorama (Sprites für Meer, Inseln, Schiff); ohne Grafik übersprungen.
#[test]
fn sea_draws_diorama_with_assets() {
    let Some(mut h) = sea_lib() else { return };
    let (mut w, mut f) = sea_world();
    w.island_switch = Some(open_switch("b"));
    f.route = paper_route();
    set_world(&mut f, &w);
    h.listen(&f);
    let mut c = NullCanvas::default();
    h.draw_cell(&mut c, &sea_cell());
    assert!(c.sprites > 10 && c.texts > 0, "Diorama: {c:?}");
}
