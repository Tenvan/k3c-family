//! Wirkung der Flut (RU62.6, docs/rules/wetter.md § 3.2): In der aktiven Wetternacht stehen die beiden Inselränder
//! `floodUnits` Units unter Wasser, nur an der Oberfläche (Tiefe 0). Die Zonen sind für alle Bodeneinheiten gesperrt
//! (Monarchen, Truppen samt Bürgern, Gegner ohne `ignoresWalls`); wer beim Beginn darin steht, läuft mit `runUnits` zur
//! inneren Kante heraus. Portale in der Zone halten ihre Gegner (`in_zone`, genutzt von `step_spawns`). Die Zonen werden
//! je Takt aus dem Plan berechnet (`World::flood_zones`), nichts davon steht im Spielstand; Gebäude nehmen keinen Schaden.
use crate::state::World;
use crate::state::weather_view::{FloodZone, WeatherKind, WeatherPhase};
use crate::world::vals;

/// Trait der Gegner, die Wände und damit die Flut ignorieren (Flieger).
const AIR_TRAIT: &str = "ignoresWalls";

/// Setzt `World::flood_zones` aus Zustand und Plan (Aufruf je Takt nach der Zustandsrechnung). Die Zone endet vor dem
/// Burg-Radius, damit die Burg auf schmalen Inseln erreichbar bleibt; fällt sie dabei weg, entsteht keine.
pub fn update(w: &mut World) {
    w.flood_zones.clear();
    let Some(plan) = w.weather_cfg.clone() else {
        return;
    };
    let active = w
        .weather
        .is_some_and(|s| s.kind == WeatherKind::Flut && s.phase == WeatherPhase::Active);
    if !active || w.biome.depth != 0 {
        return;
    }
    let radius = vals(w).hub.castle_radius_units;
    let left_to = plan.flood_units.min(w.hub_x - radius);
    let right_from = (w.width_units - plan.flood_units).max(w.hub_x + radius);
    if left_to > 0.0 {
        w.flood_zones.push(FloodZone { from: 0.0, to: left_to });
    }
    if right_from < w.width_units {
        w.flood_zones.push(FloodZone {
            from: right_from,
            to: w.width_units,
        });
    }
}

/// Liegt `x` in einer gesperrten Zone (die Ränder zählen mit)? Portale in der Zone halten ihre Gegner.
pub fn in_zone(zones: &[FloodZone], x: f64) -> bool {
    zones.iter().any(|z| x >= z.from && x <= z.to)
}

/// Schiebt `x` aus der Zone zur inneren Kante, höchstens `step` Units; wahr, wenn sich `x` bewegt hat.
fn push_out(x: &mut f64, zones: &[Option<FloodZone>; 2], step: f64) -> bool {
    for z in zones.iter().flatten() {
        if *x < z.from || *x > z.to {
            continue;
        }
        let edge = if z.from <= 0.0 { z.to } else { z.from }; // die Zone am linken Rand beginnt bei 0
        let d = edge - *x;
        if d == 0.0 {
            return false; // steht auf der Kante: bleibt
        }
        *x = if d.abs() <= step { edge } else { *x + d.signum() * step };
        return true;
    }
    false
}

/// Nach dem Tick einer Stufe: Bodeneinheiten in den Zonen laufen heraus; wer hineinlaufen wollte, steht wieder an der
/// Kante (das Herauslaufen ist schneller als jedes Bodentempo). Ohne Zonen geschieht nichts.
pub fn apply(w: &mut World, dt: f64) {
    if w.flood_zones.is_empty() {
        return;
    }
    let Some(plan) = w.weather_cfg.clone() else {
        return;
    };
    let step = plan.run_units * dt;
    let zones = [w.flood_zones.first().copied(), w.flood_zones.get(1).copied()];
    for p in &mut w.players {
        if push_out(&mut p.x, &zones, step) {
            p.vx = 0.0;
        }
    }
    for t in &mut w.troops {
        push_out(&mut t.x, &zones, step);
    }
    for e in &mut w.enemies {
        if !e.traits.iter().any(|t| t == AIR_TRAIT) {
            push_out(&mut e.x, &zones, step);
        }
    }
}
