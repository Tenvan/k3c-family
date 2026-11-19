//! Wirkung des Taifuns (RU62.5, docs/rules/wetter.md § 3.1): Tempo aller Einheiten und Fernkampf-Reichweite aller Schützen
//! beider Seiten sinken in der aktiven Wetternacht, nur an der Oberfläche (Tiefe 0). Die Faktoren stehen je Takt in
//! `World::weather_speed` und `World::weather_range` (1.0 = keine Wirkung); die Kampf- und Bewegungscode multipliziert sie.
use crate::state::World;
use crate::state::weather_view::{WeatherKind, WeatherPhase};

/// Setzt Zustand und Faktoren der Stufe aus ihrem Zyklus (Aufruf je Takt vor der Stufe). Ohne Wetter-Plan bleibt alles
/// unverändert (Zustand leer, Faktoren 1.0).
pub fn update(w: &mut World) {
    let Some(plan) = w.weather_cfg.clone() else {
        return;
    };
    let state = plan.state(w.cycle.day, &w.cycle.phase);
    w.weather = state;
    let taifun = state.is_some_and(|s| s.kind == WeatherKind::Taifun && s.phase == WeatherPhase::Active);
    let surface = w.biome.depth == 0;
    let (speed, range) = if taifun && surface {
        (plan.speed_factor, plan.range_factor)
    } else {
        (1.0, 1.0)
    };
    w.weather_speed = speed;
    w.weather_range = range;
}
