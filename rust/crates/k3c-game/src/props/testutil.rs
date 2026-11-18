//! Hilfen der props-Tests: Welten und Plätze als Literal (die Go-Tests bauen `&sim.World{…}` ebenso) und eine
//! `FixedSimView` mit festen Werten für die Tests der Zeichner (RU16.2 bis RU16.5, hud).

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

use k3c_sim::state::{Player, Site, World};

use super::pending::{CombatSpot, Economy, SpotState};
use super::plan::{Sheets, sheet_size};
use super::simview::SimView;
use crate::actors::catalog::UNIT_PX;
use crate::art::{Sheet, read_manifest};
use crate::view::cell::{Camera, new_camera, rect};

/// `wall_at` ist eine Mauer an x (Go: `wallAt` in buildings_test.go).
pub fn wall_at(id: i64, x: f64, level: i64, state: &str) -> Site {
    Site {
        id,
        kind: "wall".to_string(),
        x,
        level,
        state: state.to_string(),
        ..Default::default()
    }
}

/// `FixedSimView` liefert in jedem Aufruf dieselben, vom Test gesetzten Werte.
#[derive(Debug, Clone, Default)]
pub struct FixedSimView {
    pub economy: Economy,
    pub gold_left: (i64, bool),
    pub payable: bool,
    pub needs_clearing: bool,
    pub shield_rack: (f64, i64, bool),
    pub combat_spots: Vec<CombatSpot>,
    pub at_lair: bool,
    pub spot_near: bool,
    pub spot_state: SpotState,
}

impl SimView for FixedSimView {
    fn economy_of(&self, _w: &World) -> Economy {
        self.economy.clone()
    }
    fn site_gold_left(&self, _w: &World, _s: &Site) -> (i64, bool) {
        self.gold_left
    }
    fn site_payable(&self, _w: &World, _s: &Site) -> bool {
        self.payable
    }
    fn site_needs_clearing(&self, _w: &World, _s: &Site) -> bool {
        self.needs_clearing
    }
    fn shield_rack(&self, _w: &World) -> (f64, i64, bool) {
        self.shield_rack
    }
    fn combat_spots(&self, _w: &World) -> Vec<CombatSpot> {
        self.combat_spots.clone()
    }
    fn at_lair(&self, _w: &World, _p: &Player) -> bool {
        self.at_lair
    }
    fn spot_near(&self, _w: &World, _s: &CombatSpot, _p: &Player) -> bool {
        self.spot_near
    }
    fn spot_state(&self, _w: &World, _s: &CombatSpot, _p: &Player) -> SpotState {
        self.spot_state
    }
}

/// `grafik_dir`: gerenderte Papier-Grafik (`task gen:paper`; PNG und Manifeste sind nicht eingecheckt, B-737).
pub fn grafik_dir() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../public/grafik"))
}

/// `MapSheets` liefert für jedes Asset ein Sheet mit den Maßen aus dem Manifest unter `public/grafik` (ohne GPU;
/// Go: `fakeSheets` in plan_test.go).
#[derive(Debug, Default)]
pub struct MapSheets(BTreeMap<(String, String), Sheet>);

impl Sheets for MapSheets {
    fn size_of(&self, asset: &str, anim: &str) -> Option<(f64, f64)> {
        self.0.get(&(asset.to_string(), anim.to_string())).map(sheet_size)
    }
}

/// `fake_sheets` liest alle Manifeste; `None` (Test überspringen), wenn die gerenderte Grafik fehlt (Go: `needGrafik`).
pub fn fake_sheets() -> Option<Rc<dyn Sheets>> {
    let dir = grafik_dir();
    if !dir.join("objekte").join("tanne").join("manifest.json").exists() {
        return None;
    }
    let mut out = MapSheets::default();
    for group in std::fs::read_dir(&dir).ok()?.flatten() {
        for asset in std::fs::read_dir(group.path()).ok()?.flatten() {
            let name = format!(
                "{}/{}",
                group.file_name().to_string_lossy(),
                asset.file_name().to_string_lossy()
            );
            let Ok(m) = read_manifest(&dir, &name) else {
                continue;
            };
            let r = m.frame;
            for (anim, a) in &m.anims {
                let mut s = Sheet::default();
                s.asset.clone_from(&name);
                s.anim.clone_from(anim);
                s.fw = m.frame_w;
                s.fh = m.frame_h;
                s.fps = a.fps;
                s.origin_x = -r[0] / r[2];
                s.origin_y = -r[1] / r[3];
                s.units_per_px = r[3] / m.frame_h as f64;
                out.0.insert((name.clone(), anim.clone()), s);
            }
        }
    }
    Some(Rc::new(out))
}

/// `StubSheets` liefert für jedes Bild ein 32x32-Sheet ohne Pixel (nur die Größe zählt; Go: `stubSheets`).
#[derive(Debug)]
pub struct StubSheets(Sheet);

impl StubSheets {
    pub fn rc() -> Rc<dyn Sheets> {
        let mut s = Sheet::default();
        s.fw = 32;
        s.fh = 32;
        s.origin_y = 1.0;
        s.units_per_px = UNIT_PX;
        Rc::new(StubSheets(s))
    }
}

impl Sheets for StubSheets {
    fn size_of(&self, _asset: &str, _anim: &str) -> Option<(f64, f64)> {
        Some(sheet_size(&self.0))
    }
}

/// `camera`: Zelle 1280 × 360 um x, 27 Pixel je Unit.
pub fn camera(x: f64) -> Camera {
    new_camera(rect(0, 0, 1280, 360), x, 27.0)
}

#[test]
fn fixed_sim_view_liefert_feste_werte() {
    let v = FixedSimView {
        payable: true,
        shield_rack: (3.0, 2, true),
        ..Default::default()
    };
    let w = World::default();
    assert!(v.site_payable(&w, &Site::default()));
    assert_eq!(v.shield_rack(&w), (3.0, 2, true));
    assert!(!v.at_lair(&w, &Player::default()));
}
