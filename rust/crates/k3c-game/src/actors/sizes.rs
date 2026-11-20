//! Größenstufen für Figuren (LA1.1): Höhe der Truppen, Gegner und Bosse aus `data/sizes.json`, damit ihre sichtbare
//! Höhe Stufe × Monarchenhöhe ist. Gebäude und Objekte skaliert die Library (`art/scales.rs`). Der Monarch bleibt der
//! Bezugswert und behält seine Höhe.

use k3c_data::sizes::Sizes;

use super::catalog::{Catalog, Look, UNIT_PX};

/// Setzt die Höhe jedes Looks mit Stufe; Bosse nur, wenn sie ein eigenes Boss-Asset haben (Endbosse ohne eigenes Asset
/// zeichnen die Basisart in Boss-Höhe).
pub fn apply(c: &mut Catalog, sizes: &Sizes) {
    for l in c.troops.values_mut().chain(c.enemies.values_mut()) {
        set(l, sizes);
    }
    for l in c.bosses.values_mut().filter(|l| l.asset.starts_with("bosse/")) {
        set(l, sizes);
    }
}

fn set(l: &mut Look, sizes: &Sizes) {
    if let Some(px) = sizes.frame_px(&l.asset) {
        l.height = px / UNIT_PX;
    }
}

#[cfg(test)]
#[path = "sizes_tests.rs"]
mod tests;
