//! Port von `native/k3c-game/hud/radar.go`.
//!
//! Radar (B-090): eine schmale Leiste über das ganze Level mit Burg, Portalen, Ausgang, Treppen, Gegnern und Monarchen
//! sowie dem Kamera-Ausschnitt der Zelle. Sie sitzt an der Unterkante der Zelle im Fluss (layout.go, B-915). Reine
//! Rechnung in einen wiederverwendeten Puffer; gezeichnet wird in drawworld.go (drawRadar).

use super::texts::name_of;
use crate::view::cell::Camera;
use crate::view::frame::Stage;
use k3c_sim::state::World;

/// RadarKind ist die Art eines Markers (Zeichenreihenfolge: Burg, Portale, Ausgang, Treppen, Gegner, Monarchen).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadarKind {
    #[default]
    Castle,
    Portal,
    Exit,
    Stairs,
    Enemy,
    Player,
}

/// RadarMarker ist ein Marker: Lage 0..1 entlang der Levelbreite; nur bei Monarchen Index, eigen und gefallen.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RadarMarker {
    pub kind: RadarKind,
    pub pos: f64,
    pub index: i64,
    pub own: bool,
    pub down: bool,
}

/// Maße der Leiste in px bei 1080 (radar.ts).
pub const RADAR_HEIGHT: f64 = 20.0;
pub const RADAR_MAX_WIDTH: f64 = 640.0;

/// unit_pos ist die Lage x in 0..1 bei Levelbreite width (Breite 0: 0).
pub fn unit_pos(x: f64, width: f64) -> f64 {
    if width <= 0.0 {
        return 0.0;
    }
    (x / width).clamp(0.0, 1.0)
}

/// radar_markers schreibt die Marker der Welt w in out (geleert); self_index ist der Monarch der Zelle (-1 = keiner).
/// Die Monarchen stehen zuletzt (oben).
pub fn radar_markers(out: &mut Vec<RadarMarker>, w: &World, self_index: i64) {
    out.clear();
    let at = |x: f64| unit_pos(x, w.width_units);
    let marker = |kind: RadarKind, x: f64| RadarMarker {
        kind,
        pos: at(x),
        ..Default::default()
    };
    out.push(marker(RadarKind::Castle, w.castle.x));
    for &x in &w.portals {
        out.push(marker(RadarKind::Portal, x));
    }
    for e in &w.level.entities {
        if e.kind == "exit" {
            out.push(marker(RadarKind::Exit, e.x));
        }
    }
    for s in &w.sites {
        if s.kind == "stairsUp" || s.kind == "stairsDown" {
            out.push(marker(RadarKind::Stairs, s.x));
        }
    }
    for e in &w.enemies {
        out.push(marker(RadarKind::Enemy, e.x));
    }
    for p in &w.players {
        out.push(RadarMarker {
            kind: RadarKind::Player,
            pos: at(p.x),
            index: p.index,
            own: p.index == self_index,
            down: p.crown.is_some(),
        });
    }
}

/// radar_view ist der sichtbare Ausschnitt der Kamera in 0..1.
pub fn radar_view(k: &Camera, width: f64) -> (f64, f64) {
    let (l, r) = k.span();
    (unit_pos(l, width), unit_pos(r, width))
}

/// Trenner zwischen den Biomen der Inselzeile.
const BIOME_SEP: &str = " · ";

/// biomes_key ist ein Schlüssel der Stufenliste (Sprache, Tiefe und Biom je Stufe) ohne Allokation; die Zeile wird nur
/// bei Änderung neu gebaut.
pub fn biomes_key(lang: &str, stages: &[Stage]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    lang.hash(&mut h);
    for s in stages {
        (s.depth, &s.level).hash(&mut h);
    }
    h.finish()
}

/// island_biomes ist die Kompass-Zeile im Hub (LA1.3, B-1167): die Biome aller Stufen der Insel in Reihenfolge der
/// Stufen, z. B. „Oberwelt (Wald) · Tiefe 1 (Höhle)“; bei einer Stufe nur ihr Biom (HB1.3: immer am Radar).
pub fn island_biomes(lang: &str, stages: &[Stage]) -> String {
    if stages.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    for s in stages {
        let fallback = s.world.as_deref().map_or("", |w| w.biome.name.as_str());
        if !out.is_empty() {
            out.push_str(BIOME_SEP);
        }
        out.push_str(&name_of(lang, "biome", &s.level, fallback));
    }
    out
}

/// radar_size ist die Größe der Leiste in einer Zelle der Breite cell_w beim Maßstab k: halb so breit wie die Zelle,
/// höchstens 640 px bei 1080.
pub fn radar_size(cell_w: f64, k: f64) -> (f64, f64) {
    ((cell_w / 2.0).min(RADAR_MAX_WIDTH * k), RADAR_HEIGHT * k)
}
