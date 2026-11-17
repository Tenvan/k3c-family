//! Hilfen der Tests des Pakets fx (Leitfaden § 6).

use std::rc::Rc;

use serde_json::Value;

use k3c_sim::state::World;

use super::{Fx, new};
use crate::view::frame::{Event, Stage};

/// Ein Drawer ohne Grafik und mit fester (leerer) Aura-Quelle: `aura_of` ist bis RU18 ein Platzhalter.
pub fn new_fx() -> Fx {
    new(None, Rc::new(|_, _| Vec::new()))
}

/// Die Stufe `index` mit einer Kopie der Welt w.
pub fn stage_of(index: i64, w: &World) -> Stage {
    Stage {
        index,
        world: Some(Rc::new(w.clone())),
        ..Stage::default()
    }
}

/// Ein Ereignis der Stufe `stage` mit den Daten v.
pub fn event_of(stage: i64, v: Value) -> Event {
    Event {
        stage,
        data: v.as_object().expect("Objekt").clone(),
        ..Event::default()
    }
}
