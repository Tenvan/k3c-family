//! Zustand des Standbilds: Dateiformat `<name>.state.json.gz` (geschrieben von `native/k3c-game/world/rustref_test.go`)
//! und der Frame mit den Zellen, den die Drawer bekommen.
//!
//! Die Datei trägt die Sim-Welt wie im Spielstand (`world`), dazu Biom (`data/biomes/<level>.json`) und Level
//! (`layout`, Form des Level-Golden): beides ist in der Welt ohne JSON (`serde(skip)`), die Zeichner brauchen es für
//! Boden und Gegnerarten.

use std::path::Path;
use std::rc::Rc;

use k3c_data::biome::Biome;
use k3c_level::types::Layout;
use k3c_sim::state::World;
use serde::Deserialize;

use super::inflate::gunzip;
use crate::view::cell::{Cell, new_camera, rect};
use crate::view::frame::{Frame, Seat, Stage};
use crate::view::settings::{RIVER_TILES, Settings};

/// Welt-Höhe der Zelle in Units (game/layout.go: 420/32).
pub const VIEW_UNITS: f64 = 420.0 / 32.0;

#[derive(Deserialize)]
struct SeatFile {
    player: i64,
    monarch: i64,
    stage: i64,
}

#[derive(Deserialize)]
struct StateFile {
    name: String,
    level: String,
    size: [u32; 2],
    clock: f64,
    cams: Vec<f64>,
    seats: Vec<SeatFile>,
    biome: serde_json::Value,
    layout: serde_json::Value,
    world: serde_json::Value,
}

/// Ein Stand: Welt, Plätze, Kamera je Zelle und Bildgröße.
#[derive(Clone)]
pub struct StillState {
    pub name: String,
    pub level: String,
    pub size: (u32, u32),
    /// Frame.Clock in Sekunden
    pub clock: f64,
    /// Kamera-x je Zelle in Units
    pub cams: Vec<f64>,
    pub seats: Vec<Seat>,
    pub world: World,
}

/// Liest einen Zustand aus gzip oder aus JSON (erkannt an der gzip-Kennung).
pub fn parse_state(raw: &[u8]) -> Result<StillState, String> {
    let plain = if raw.starts_with(&[0x1f, 0x8b]) {
        gunzip(raw)?
    } else {
        raw.to_vec()
    };
    let f: StateFile = serde_json::from_slice(&plain).map_err(|e| format!("Zustand: {e}"))?;
    let mut world: World = serde_json::from_value(f.world).map_err(|e| format!("Zustand › world: {e}"))?;
    world.biome = serde_json::from_value::<Biome>(f.biome).map_err(|e| format!("Zustand › biome: {e}"))?;
    world.level = serde_json::from_value::<Layout>(f.layout).map_err(|e| format!("Zustand › layout: {e}"))?;
    if f.cams.len() != f.seats.len() || f.seats.is_empty() {
        return Err("Zustand: je Platz eine Kamera erwartet".to_string());
    }
    Ok(StillState {
        name: f.name,
        level: f.level,
        size: (f.size[0], f.size[1]),
        clock: f.clock,
        cams: f.cams,
        seats: f
            .seats
            .iter()
            .map(|s| Seat {
                player: s.player,
                monarch: s.monarch,
                stage: s.stage,
                ..Seat::default()
            })
            .collect(),
        world,
    })
}

/// Liest die Datei path.
pub fn load_state(path: &Path) -> Result<StillState, String> {
    let raw = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    parse_state(&raw).map_err(|e| format!("{}: {e}", path.display()))
}

impl StillState {
    /// Frame und Zellen wie die Go-Referenz: Zellen übereinander, Kamera fest auf `cams`, Fluss als Kacheln.
    pub fn frame(&self) -> (Frame, Vec<Cell>) {
        let (w, h) = (i64::from(self.size.0), i64::from(self.size.1));
        let n = self.seats.len() as i64;
        let mut cells = Vec::new();
        for (i, x) in self.cams.iter().enumerate() {
            let i = i as i64;
            let r = rect(0, i * h / n, w, (i + 1) * h / n);
            let mut c = Cell {
                index: i,
                player: self.seats[i as usize].player,
                rect: r,
                ..Cell::default()
            };
            #[allow(clippy::cast_precision_loss)]
            let scale = c.world_rect().dy() as f64 / VIEW_UNITS;
            c.camera = new_camera(r, *x, scale);
            cells.push(c);
        }
        let f = Frame {
            clock: self.clock,
            stages: vec![Stage {
                index: 0,
                level: self.level.clone(),
                world: Some(Rc::new(self.world.clone())),
                ..Stage::default()
            }],
            seats: self.seats.clone(),
            cells: cells.clone(),
            settings: Settings {
                river: RIVER_TILES.to_string(),
                ..Settings::default()
            },
            ..Frame::default()
        };
        (f, cells)
    }
}
