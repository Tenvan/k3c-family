//! Port von `native/k3c-game/view/crown.go`.
//!
//! Die Kronen-Zustände (sim.CrownFlying …) exportiert die Engine; hier steht nur die Form.

/// CrownParts ist die Form der Krone in Spiel-Pixeln (32 je Unit), gemeinsam für props (am Boden, im Flug) und actors
/// (am Kopf): je Teil Versatz (dx, dy) der Mitte von der Mitte des Bands und Größe (w, h); Band zuerst, dann drei Zacken.
pub const CROWN_PARTS: [[f64; 4]; 4] = [
    [0.0, 0.0, 20.0, 7.0],
    [-7.0, 6.0, 5.0, 7.0],
    [0.0, 7.0, 5.0, 9.0],
    [7.0, 6.0, 5.0, 7.0],
];

#[cfg(test)]
#[path = "crown_tests.rs"]
mod crown_tests;
