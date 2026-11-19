//! RU55.6 (B-1047): Wald nach Rodung (`data/look.json` › forest, worlds; Standard Bildaufbau §§ 3 und 5).
//!
//! Die Walddichte ist der Anteil Bäume (nicht gefällt) im Umkreis der Kamera an der Zahl, die den Wald voll dicht
//! macht, geglättet über die Zeit: Fällt ein Baum, wird das Bild über Sekunden heller, nicht in einem Bild. Dichte
//! steuert die Deckkraft der Waldwand (gerodet: Blick zum Horizont), die Tönung (dunkles Oliv) und den Anteil der Fackel am
//! Licht. Das Spiel liest nur den Zustand; Rodungsregeln der Engine bleiben unberührt. Je Bild entsteht nichts Neues.

use std::cell::Cell as Slot;

use k3c_data::look::{MONARCH_UNITS, current};
use k3c_sim::state::World as SimWorld;

use super::World;
use super::tint::{NEUTRAL, mix};
use crate::view::cell::Cell;

/// SLOTS ist die Zahl der Zellen (Spieler), für die eine geglättete Dichte geführt wird.
pub const SLOTS: usize = 8;

/// ForestDensity ist die Dichte 0 … 1: `trees` Bäume im Umkreis, `full` Bäume machen den Wald voll dicht.
pub fn forest_density(trees: f64, full: f64) -> f64 {
    if full <= 0.0 {
        return 0.0;
    }
    (trees / full).clamp(0.0, 1.0)
}

/// CountTrees zählt die stehenden Bäume (nicht gefällt) im Umkreis `radius` Units um `x`; ohne Allokation.
pub fn count_trees(wd: &SimWorld, x: f64, radius: f64) -> usize {
    wd.nodes
        .iter()
        .filter(|n| n.kind == "tree" && !n.gone && (n.x - x).abs() <= radius)
        .count()
}

/// Densities hält je Zelle die geglättete Dichte und den Zeitpunkt ihrer letzten Rechnung.
#[derive(Debug)]
pub struct Densities {
    value: [Slot<f64>; SLOTS],
    clock: [Slot<f64>; SLOTS],
}

impl Default for Densities {
    fn default() -> Self {
        Densities {
            value: std::array::from_fn(|_| Slot::new(0.0)),
            clock: std::array::from_fn(|_| Slot::new(f64::NEG_INFINITY)),
        }
    }
}

impl Densities {
    /// Update führt die Dichte der Zelle `slot` zur Zeit `clock` (Sekunden) dem Ziel nach: exponentiell mit der
    /// Zeitkonstante `tau`; erster Aufruf oder Zeitsprung zurück setzen sie sofort. Mehrfach im selben Bild: unverändert
    /// (das Ziel wird dann nicht gerechnet).
    pub fn update(&self, slot: usize, clock: f64, tau: f64, target: impl FnOnce() -> f64) -> f64 {
        let i = slot.min(SLOTS - 1);
        let (last, v) = (self.clock[i].get(), self.value[i].get());
        if clock == last {
            return v;
        }
        let t = target();
        let dt = clock - last;
        let next = if !dt.is_finite() || dt < 0.0 || tau <= 0.0 {
            t
        } else {
            v + (t - v) * (1.0 - (-dt / tau).exp())
        };
        self.clock[i].set(clock);
        self.value[i].set(next);
        next
    }
}

/// WallAlpha ist die Deckkraft der Waldwand-Schicht `i` bei Dichte `d`: die ferne Schicht bleibt voll, mitte und nah
/// wachsen von `open_alpha` (gerodet) auf 1 (dicht).
pub fn wall_alpha(i: usize, d: f64, open_alpha: f64) -> f64 {
    if i == 0 {
        return 1.0;
    }
    let o = open_alpha.clamp(0.0, 1.0);
    o + (1.0 - o) * d.clamp(0.0, 1.0)
}

/// ForestTint ist der Multiplikator der Lichtkarte für Dichte `d`: Neutral bei 0, die Waldtönung bei 1.
pub fn forest_tint(d: f64) -> [f64; 3] {
    mix(NEUTRAL, current().forest.tint.rgb(), d)
}

/// MotifPlace liefert Lage des Leitmotivs der Welt `level` in der Zelle: Fenster-x der Mitte, Fußlinie (Boden) und
/// Höhe in px; die Größe steht in M (Monarchenhöhen, 3 … 5).
#[allow(clippy::cast_precision_loss)]
pub fn motif_place(c: &Cell, level: &str) -> (f64, f64, f64) {
    let m = current().world(level).motif;
    let x = c.rect.min.x as f64 + c.rect.dx() as f64 * m.x;
    (x, c.ground_y() as f64, m.size * MONARCH_UNITS * c.camera.scale)
}

impl World {
    /// Density ist die geglättete Walddichte der Zelle `c` zur Zeit `clock`; 0 ohne Wald-Wirkung (Profil `go`, Welt
    /// ohne `forest`).
    #[allow(clippy::cast_sign_loss)]
    pub fn density(&self, c: &Cell, wd: &SimWorld, clock: f64, level: &str) -> f64 {
        let look = current();
        let f = &look.forest;
        if !f.enabled || !look.world(level).forest {
            return 0.0;
        }
        let slot = c.player.max(0) as usize;
        self.dens.update(slot, clock, f.smooth, || {
            #[allow(clippy::cast_precision_loss)]
            forest_density(count_trees(wd, c.camera.x, f.radius) as f64, f.full)
        })
    }
}

#[cfg(test)]
#[path = "forest_tests.rs"]
mod forest_tests;
