//! Flaschenpost-Markierung aus dem Frame (GS4.7): Ziel und sein Ort je Stufe, für Radar, Pfeil (hud) und Leuchten (props).

use k3c_sim::bottle::BottleTarget;

use super::frame::Frame;

/// Das markierte Ziel aus der Stufe der Tiefe 0 des Frames (`None` = keine Markierung oder Stufe nicht im Frame).
pub fn target_of(f: &Frame) -> Option<BottleTarget> {
    f.stages
        .iter()
        .find(|s| s.depth == 0)
        .and_then(|s| s.world.as_deref())
        .and_then(|w| w.bottle.target)
}

/// Der Ort des Ziels, wenn es in der Stufe `stage` liegt.
pub fn goal_in(f: &Frame, stage: i64) -> Option<f64> {
    target_of(f).filter(|t| t.stage as i64 == stage).map(|t| t.x)
}
