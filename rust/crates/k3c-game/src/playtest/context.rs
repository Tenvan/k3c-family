//! Spielkontext eines Spielers aus dem Frame des letzten Takts (nur lesen, B-1220).

use super::marker::Context;
use crate::view::frame::{Frame, Seat};

/// Kontext des Platzes `seat`: Welt und Insel der Kampagne, Stufe des Spielers, Tag/Nacht und Tag-Nummer seiner Stufe.
pub fn of(frame: &Frame, seat: &Seat) -> Context {
    let mut c = Context {
        stage: seat.stage,
        island_id: frame.campaign.island.clone(),
        ..Context::default()
    };
    if let Some(r) = frame.route.iter().find(|r| r.id == c.island_id) {
        c.world.clone_from(&r.world);
        c.island = r.etappe;
    }
    if let Some(w) = frame
        .stages
        .iter()
        .find(|s| s.index == seat.stage)
        .and_then(|s| s.world.as_ref())
    {
        c.phase.clone_from(&w.cycle.phase);
        c.night = w.cycle.phase == "night";
        c.day = w.cycle.day;
    }
    c
}
