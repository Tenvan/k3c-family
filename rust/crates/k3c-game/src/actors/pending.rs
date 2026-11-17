//! Platzhalter für Teile späterer Sessions (Leitfaden § 7); RU13.5 löscht sie (levels, markdraw, marks, pose).

use std::collections::BTreeMap;

use k3c_platform::Canvas;
use k3c_sim::state::{Enemy, Player, Troop, World as SimWorld};

use super::Actors;
use super::catalog::{BossEntry, Catalog};
use super::figures::Figure;
use crate::view::cell::Cell;
use crate::view::frame::Seat;

/// RU13.5: marks.go `loadHeight`, `loadSide` (Werte aus Go; combatmarks.rs braucht sie).
pub const LOAD_HEIGHT: f64 = 0.55;
pub const LOAD_SIDE: f64 = 14.0;

/// RU13.5: levels.go `byDepth` ordnet die Gegner-Arten (Rohdaten aus enemies.json, Schlüssel ohne depth wie "notes"
/// fallen weg) und die Bosse samt Beschwörungen ihrer Tiefe zu. Der Körper ist schon echt, weil `load_catalog` ihn
/// braucht; RU13.5 verschiebt die Funktion nach `levels.rs`.
#[allow(clippy::type_complexity)]
pub fn by_depth(
    enemies: &BTreeMap<String, serde_json::Value>,
    bosses: &[BossEntry],
) -> (BTreeMap<i64, Vec<String>>, BTreeMap<i64, Vec<String>>) {
    let (mut kinds, mut ids): (BTreeMap<i64, Vec<String>>, BTreeMap<i64, Vec<String>>) = Default::default();
    for (kind, raw) in enemies {
        if let Some(d) = raw
            .as_object()
            .and_then(|o| o.get("depth"))
            .and_then(serde_json::Value::as_i64)
        {
            kinds.entry(d).or_default().push(kind.clone());
        }
    }
    for b in bosses {
        ids.entry(b.depth).or_default().push(b.id.clone());
        let abilities = std::iter::once(&b.ability).chain(b.phases.iter().map(|p| &p.ability));
        for a in abilities.flatten() {
            if !a.enemy.is_empty() {
                kinds.entry(b.depth).or_default().push(a.enemy.clone());
            }
        }
    }
    (kinds, ids)
}

impl Catalog {
    /// RU13.5: levels.go `Catalog.LevelSheets`.
    pub fn level_sheets(&self, _w: &SimWorld) -> Vec<String> {
        unimplemented!("RU13.5: catalog_level_sheets")
    }
}

/// RU13.5: markdraw.go `ink` (Vorlagen der Anzeigen).
#[derive(Clone, Debug, Default)]
pub struct Ink;

/// RU13.5: markdraw.go `newInk`.
pub fn new_ink() -> Ink {
    unimplemented!("RU13.5: new_ink")
}

/// RU13.5: markdraw.go `anchor`: Fußpunkt (sx, sy) in Fenster-px, s Fenster-px je Spiel-Pixel.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Anchor {
    pub sx: f64,
    pub sy: f64,
    pub s: f64,
}

/// RU13.5: marks.go `box`: Rechteck um die Mitte (cx, cy) in px, y nach oben.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoxPx {
    pub cx: f64,
    pub cy: f64,
    pub w: f64,
    pub h: f64,
}

impl Ink {
    /// RU13.5: markdraw.go `ink.oval`.
    pub fn oval(&self, _dst: &mut dyn Canvas, _a: Anchor, _b: BoxPx, _clr: [f32; 4]) {
        unimplemented!("RU13.5: ink_oval")
    }
}

/// RU13.5: markdraw.go `tint` (0xRRGGBB mit Deckkraft, vormultipliziert).
pub fn tint(_hex: u32, _a: f64) -> [f32; 4] {
    unimplemented!("RU13.5: tint")
}

/// RU13.5: marks.go `PlayerColor`.
pub fn player_color(_i: i64) -> u32 {
    unimplemented!("RU13.5: player_color")
}

/// RU13.5: marks.go `LoadOffset`.
pub fn load_offset(_left: bool) -> f64 {
    unimplemented!("RU13.5: load_offset")
}

/// RU13.5: marks.go `Seated`.
pub fn seated(_seats: &[Seat], _monarch: i64) -> bool {
    unimplemented!("RU13.5: seated")
}

/// RU13.5: markdraw.go `marksAnchor`.
pub fn marks_anchor(_c: &Cell, _fg: &Figure, _x: f64) -> Option<Anchor> {
    unimplemented!("RU13.5: marks_anchor")
}

impl Actors {
    /// RU13.5: markdraw.go `topPx`.
    pub fn top_px(&self, _fg: &Figure) -> f64 {
        unimplemented!("RU13.5: actors_top_px")
    }

    /// RU13.5: markdraw.go `troopMarks`.
    pub fn troop_marks(&self, _dst: &mut dyn Canvas, _c: &Cell, _fg: &Figure, _t: &Troop) {
        unimplemented!("RU13.5: actors_troop_marks")
    }

    /// RU13.5: markdraw.go `enemyMarks`.
    pub fn enemy_marks(&self, _dst: &mut dyn Canvas, _c: &Cell, _fg: &Figure, _e: &Enemy) {
        unimplemented!("RU13.5: actors_enemy_marks")
    }

    /// RU13.5: markdraw.go `playerMarker`.
    pub fn player_marker(&self, _dst: &mut dyn Canvas, _c: &Cell, _fg: &Figure, _p: &Player) {
        unimplemented!("RU13.5: actors_player_marker")
    }

    /// RU13.5: pose.go `advanceAll`: schreibt die Pose aller Rig-Figuren fort. Ohne Rig-Figur (Tests ohne Grafik)
    /// gibt es nichts zu tun.
    pub fn advance_all(&mut self) {
        if self.figs.values().any(|f| f.actor.is_some() && !f.broken) {
            unimplemented!("RU13.5: actors_advance_all");
        }
    }
}
