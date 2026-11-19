//! RU55.4 (B-1046): Tönung und Dunst nach Look-Profil (`data/look.json` › tint, haze; Standard Bildaufbau § 6).
//!
//! Eine Basisfarbe je Phase des Tages und je Nacht-Event tönt Himmel, Ebenen, Figuren und Spiegelung gleich: Sie geht
//! als Multiplikator in die Lichtkarte (`LightMesh::fill_tinted`), die die ganze Zelle multipliziert; die Assets bleiben
//! neutral. Der Dunst legt Himmelsfarbe über die ferne und die mittlere Kulisse (Luftperspektive). Biome ohne Himmel
//! (`tint.noSky`) bekommen weder Tönung noch Dunst.

use k3c_data::look::{Tint, current};
use k3c_sim::state::{CycleInfo, World as SimWorld};

use super::light::daylight;

/// NEUTRAL ändert nichts (Multiplikator 1).
pub const NEUTRAL: [f64; 3] = [1.0; 3];

/// Mix mischt `a` nach `b` um `t` (0 … 1).
pub(super) fn mix(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    let t = t.clamp(0.0, 1.0);
    [0, 1, 2].map(|i| a[i] + (b[i] - a[i]) * t)
}

/// PhaseTint ist die Tönung der Tageszeit, weich über die Phasen: Dämmerung geht von Tag über Abend nach Nacht, das
/// Morgengrauen von Nacht über Morgen nach Tag (jeweils in zwei Hälften); unbekannte Phasen gelten als Nacht.
pub fn phase_tint(t: &Tint, c: &CycleInfo) -> [f64; 3] {
    let (day, dusk, night, dawn) = (t.day.rgb(), t.dusk.rgb(), t.night.rgb(), t.dawn.rgb());
    let p = c.progress.clamp(0.0, 1.0);
    match c.phase.as_str() {
        "day" => day,
        "dusk" if p < 0.5 => mix(day, dusk, p * 2.0),
        "dusk" => mix(dusk, night, p * 2.0 - 1.0),
        "dawn" if p < 0.5 => mix(night, dawn, p * 2.0),
        "dawn" => mix(dawn, day, p * 2.0 - 1.0),
        _ => night,
    }
}

/// EventTint ist das Produkt der Tönungen der laufenden Nacht-Events, im Maß der Dunkelheit (am Tag keine Wirkung).
pub fn event_tint(t: &Tint, w: &SimWorld) -> [f64; 3] {
    let dark = 1.0 - daylight(&w.cycle);
    let mut out = NEUTRAL;
    for e in &w.night_events {
        if let Some(c) = t.event(&e.id) {
            let m = mix(NEUTRAL, c.rgb(), dark);
            out = [0, 1, 2].map(|i| out[i] * m[i]);
        }
    }
    out
}

/// TintOf ist der Multiplikator der Lichtkarte für die Welt im Level `level`: Phase mal Event; ohne Himmel neutral.
pub fn tint_of(level: &str, w: &SimWorld) -> [f64; 3] {
    let t = &current().tint;
    if t.no_sky.iter().any(|b| b == level) {
        return NEUTRAL;
    }
    let (p, e) = (phase_tint(t, &w.cycle), event_tint(t, w));
    [0, 1, 2].map(|i| p[i] * e[i])
}

/// HazeOf ist die Dunst-Deckkraft nach der Schicht `i` (0 = fern, 1 = mitte, 2 = nah); ohne Himmel 0.
pub fn haze_of(level: &str, i: usize) -> f64 {
    let l = current();
    if l.tint.no_sky.iter().any(|b| b == level) {
        return 0.0;
    }
    l.haze_for(level, i).clamp(0.0, 1.0)
}

/// HasSky meldet, ob das Level einen Himmel hat (Mond, Dunst, Tönung); Höhle und Mine nicht.
pub fn has_sky(level: &str) -> bool {
    !current().tint.no_sky.iter().any(|b| b == level)
}

/// IsNeutral meldet, ob die Tönung nichts ändert (dann genügt der Tag ohne Lichtkarte).
pub fn is_neutral(t: [f64; 3]) -> bool {
    t.iter().all(|c| (c - 1.0).abs() < 1e-9)
}

#[cfg(test)]
#[path = "tint_tests.rs"]
mod tint_tests;
