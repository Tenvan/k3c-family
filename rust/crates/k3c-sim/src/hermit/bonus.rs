//! Die Boni des Einsiedlers (RU62.10): Faktor und Heilanteil stehen je Takt in `World::hermit` (`publish`), nur solange das
//! Bauwerk der Insel steht; ist es zerstört oder noch im Bau, gelten der Grundwert (Faktor 1, keine Heilung).
use serde_json::json;

use crate::events::emit;
use crate::state::common::event;
use crate::state::{Site, World};

/// Schadensfaktor der Krieger und Schützen dieser Stufe (Dojo); 1 ohne Bonus.
pub fn damage_factor(w: &World) -> f64 {
    w.hermit
        .as_ref()
        .map_or(1.0, |h| if h.damage_factor > 1.0 { h.damage_factor } else { 1.0 })
}

/// Tagesanbruch: Jede stehende Mauer der Stufe heilt um `wall_heal` ihrer vollen HP, höchstens bis voll (Schrein).
pub fn heal_walls(w: &mut World) {
    let fraction = w.hermit.as_ref().map_or(0.0, |h| h.wall_heal);
    if fraction <= 0.0 {
        return;
    }
    let mut healed = 0;
    for s in &mut w.sites {
        if s.kind == "wall" && s.state == "built" && s.hp > 0.0 && s.hp < s.max_hp {
            s.hp = s.max_hp.min(s.hp + s.max_hp * fraction);
            healed += 1;
        }
    }
    if healed > 0 {
        emit(w, "wallsHealed", event(json!({"count": healed})));
    }
}

/// Bauzeit des Bauwerks für die Reparatur (`building/repair.rs`: der Platz steht in keiner buildings.json); 0 für jeden anderen
/// Platz.
pub fn hall_seconds(w: &World, s: &Site) -> f64 {
    let (Some(cfg), Some(h)) = (w.hermit_cfg.as_ref(), w.hermit.as_ref()) else {
        return 0.0;
    };
    if cfg.hub_site.id != s.kind {
        return 0.0;
    }
    cfg.entry(&h.hermit).map_or(0.0, |e| e.build_seconds)
}

/// Der Platz `s` ist der Hub-Platz des Bauwerks (nimmt keine Münzen an, sondern wird über die Hütte bezahlt).
pub fn is_hall(w: &World, s: &Site) -> bool {
    w.hermit_cfg.as_ref().is_some_and(|c| c.hub_site.id == s.kind)
}
