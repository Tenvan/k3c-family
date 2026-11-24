//! Tests zu `event_veil.rs` (WE8.8): Schleier nur in der Event-Nacht an der Oberfläche, der stärkere gilt, und das Zeichnen
//! setzt genau ein Rechteck.

use std::rc::Rc;

use super::*;
use crate::view::frame::{Frame, Seat, Stage};
use crate::world::testutil::layer_cell;
use k3c_platform::NullCanvas;
use k3c_sim::state::{ActiveEvent, World as SimWorld};

fn night_with(events: &[&str], depth: i64) -> SimWorld {
    let mut s = SimWorld::default();
    s.biome.depth = depth;
    s.cycle.phase = "night".into();
    s.night_events = events
        .iter()
        .map(|id| ActiveEvent {
            id: (*id).into(),
            seconds_left: 60.0,
        })
        .collect();
    s
}

fn frame(w: SimWorld) -> Frame {
    Frame {
        clock: 3.0,
        stages: vec![Stage {
            index: 0,
            level: "forest".to_string(),
            world: Some(Rc::new(w)),
            ..Stage::default()
        }],
        seats: vec![Seat::default()],
        ..Frame::default()
    }
}

#[test]
fn schleier_nur_in_der_event_nacht_an_der_oberflaeche() {
    assert_eq!(veil_of(&night_with(&["sandstorm"], 0)), Some(SANDSTORM_VEIL));
    assert_eq!(veil_of(&night_with(&["fog"], 0)), Some(FOG_VEIL));
    assert_eq!(veil_of(&night_with(&["sandstorm"], 1)), None, "unter Tage kein Schleier");
    assert_eq!(veil_of(&night_with(&["raid"], 0)), None, "Überfall hat keinen Schleier");
    let mut tag = night_with(&["sandstorm"], 0);
    tag.cycle.phase = "day".into();
    assert_eq!(veil_of(&tag), None, "am Tag kein Schleier");
}

#[test]
fn der_staerkere_schleier_gilt_ohne_ueberlagerung() {
    assert_eq!(veil_of(&night_with(&["sandstorm", "fog"], 0)), Some(FOG_VEIL));
}

#[test]
fn zeichnen_setzt_genau_ein_rechteck() {
    let (world, c, f) = (World::new(None), layer_cell(10.0), frame(night_with(&["fog"], 0)));
    let mut canvas = NullCanvas::default();
    world.draw_event_veil(&mut canvas, &f, &c);
    assert_eq!(canvas.rects, 1);
    let mut leer = NullCanvas::default();
    world.draw_event_veil(&mut leer, &frame(night_with(&[], 0)), &c);
    assert_eq!(leer.rects, 0);
}
