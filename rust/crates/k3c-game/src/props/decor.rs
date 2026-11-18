//! Port von `native/k3c-game/props/decor.go`: Darstellung der Natur-Eigenschaften (Sprint BI2, B-902 bis B-906).
//! Eigenschaften mit `display: true` aus data/biomes zeigen sich als Formen der Zeichenliste. Der Wert gilt je Stufe
//! (`World::level.nature`, B-891); fehlt er, gilt der Default des Bioms, ein Biom ohne die Eigenschaft zeigt nichts
//! davon. Orte und Formen kommen aus der Position (`mix`), kein Zufall. `decor` läuft vor allen anderen props, liegt
//! also hinter ihnen.

use k3c_sim::state::World;

use super::nature::mix;
use super::plan::{Item, Planner, Shape};
use crate::view::cell::Camera;

/// `VIEW_TOP` ist die sichtbare Welthöhe über dem Boden in Units (game.ViewUnits: 420 px bei 32 px je Unit).
pub const VIEW_TOP: f64 = 420.0 / 32.0;

/// Raster der Decken-Zacken und der Tropfsteine in Units.
pub const CEILING_STEP: f64 = 1.5;
pub const STAL_STEP: f64 = 2.0;

pub const CEILING_ROCK: [u8; 4] = [36, 36, 46, 255];
pub const STAL_COLOR: [u8; 4] = [74, 72, 86, 255];

/// `trait_of` liefert den geltenden Wert der Eigenschaft `key`, begrenzt auf ihren Bereich; `None`, wenn das Biom sie
/// nicht hat (Go: `trait`).
pub fn trait_of(w: &World, key: &str) -> Option<i64> {
    let t = w.biome.traits.get(key)?;
    let v = w.level.nature.traits.get(key).copied().unwrap_or(t.default);
    Some(v.max(t.min).min(t.max))
}

/// `span` ist der sichtbare Bereich der Kamera `k` innerhalb der Welt, um `margin` erweitert, Anfang auf das Raster
/// `step` gelegt; `None`, wenn nichts davon in der Welt liegt. Kamera ohne Maßstab = ganze Welt (wie `visible`).
pub fn span(w: &World, k: &Camera, step: f64, margin: f64) -> Option<(f64, f64)> {
    let (mut l, mut r) = (0.0, w.width_units);
    if k.scale != 0.0 {
        (l, r) = k.span();
    }
    let (l, r) = ((l - margin).max(0.0), (r + margin).min(w.width_units));
    if l >= r {
        return None;
    }
    Some(((l / step).floor() * step, r))
}

/// `cell_hash` ist die Mischung des Rasterplatzes bei `x` (Raster `step`) mit `salt`: gleicher Platz, gleicher Wert.
pub fn cell_hash(x: f64, step: f64, salt: i64) -> u32 {
    mix((x / step).round() * 7.0 + salt as f64)
}

/// `ceiling_bottom` ist die Unterkante der Decke in Units über dem Boden zum Wert `v` der Deckenhöhe (1 tief, 3 hoch):
/// Dicke 3,5 Units bei 1, 2,15 bei 2, 0,8 bei 3.
pub fn ceiling_bottom(v: i64) -> f64 {
    VIEW_TOP - (3.5 - (v - 1) as f64 * 1.35)
}

impl Planner {
    /// `decor` plant die Darstellung der Natur-Eigenschaften der Welt `w` für Kamera `k` zur Uhr `clock` (Pulsieren).
    pub fn decor(&mut self, w: &World, k: &Camera, clock: f64) {
        self.ceiling(w, k);
        self.stalactites(w, k);
        self.decor_glow(w, k, clock);
    }

    /// `ceiling`: Felsdecke über die sichtbare Breite bis zur Unterkante der Deckenhöhe, mit Zacken nach unten (B-902).
    pub fn ceiling(&mut self, w: &World, k: &Camera) {
        let Some(v) = trait_of(w, "ceilingHeight") else {
            return;
        };
        let Some((from, to)) = span(w, k, CEILING_STEP, 2.0) else {
            return;
        };
        let y = ceiling_bottom(v);
        Planner::rect(
            &mut self.body,
            (from + to) / 2.0,
            y,
            to - from,
            VIEW_TOP + 2.0 - y,
            CEILING_ROCK,
        );
        let mut x = from;
        while x < to {
            let h = cell_hash(x, CEILING_STEP, 101);
            let depth = 0.3 + f64::from(h % 6) / 10.0;
            let width = CEILING_STEP * (1.1 + f64::from((h >> 8) % 4) / 10.0);
            self.body.push(Item {
                shape: Shape::Tri,
                x: x + CEILING_STEP / 2.0,
                y: y + 0.05,
                w: width,
                h: -depth,
                clr: CEILING_ROCK,
                ..Default::default()
            });
            x += CEILING_STEP;
        }
    }

    /// `stalactites`: Tropfsteine hängen von der Decke, an jedem Rasterplatz mit `mix % 6 < Wert` (B-903); so bleiben
    /// die Plätze eines kleineren Werts bei einem größeren erhalten. Ohne Deckenhöhe hängen sie an der Decke mit Wert 2.
    pub fn stalactites(&mut self, w: &World, k: &Camera) {
        let Some(v) = trait_of(w, "stalactites").filter(|&v| v > 0) else {
            return;
        };
        let c = trait_of(w, "ceilingHeight").unwrap_or(2);
        let Some((from, to)) = span(w, k, STAL_STEP, 1.0) else {
            return;
        };
        let y = ceiling_bottom(c) + 0.1;
        let mut x = from;
        while x < to {
            let h = cell_hash(x, STAL_STEP, 211);
            if i64::from(h % 6) < v {
                let length = 0.6 + f64::from((h >> 4) % 13) / 10.0;
                let width = 0.4 + f64::from((h >> 8) % 5) / 10.0;
                let off = f64::from((h >> 12) % 7) / 10.0 - 0.3;
                self.body.push(Item {
                    shape: Shape::Tri,
                    x: x + STAL_STEP / 2.0 + off,
                    y,
                    w: width,
                    h: -length,
                    clr: STAL_COLOR,
                    ..Default::default()
                });
            }
            x += STAL_STEP;
        }
    }
}
