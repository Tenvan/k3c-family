//! Lebende Gegner im Frieden (RU62.12, docs/rules/kampf.md § 10): Sie brechen ab, greifen nicht mehr an, rauben keine Krone und
//! fliehen zum nächsten Portal, ohne Portal zum nächsten Inselrand; dort verschwinden sie. Getragene Krone, Ausrüstung und
//! geraubtes Gold lassen sie vorher fallen (`portal::drop_loot`), damit der Sieg sauber bleibt (kein Kronenraub, kein Game
//! Over nach dem Sieg). Ersetzt in `world/step.rs` den Gegner-Takt (`step_enemies`), solange `World::peace` gilt; tote Gegner
//! bleiben für `remove_dead_enemies` (Drops und Abrechnung wie sonst).
use crate::portal::drop_loot;
use crate::state::World;
use crate::state::common::sign;

/// Das Ziel der Flucht für einen Gegner bei `x`: das nächste stehende Portal, sonst der nähere Rand der Stufe.
pub fn exit_x(w: &World, x: f64) -> f64 {
    let nearest = w.portals.iter().copied().min_by(|a, b| {
        (a - x)
            .abs()
            .partial_cmp(&(b - x).abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    match nearest {
        Some(p) => p,
        None if x <= w.width_units - x => 0.0,
        None => w.width_units,
    }
}

/// Ein Takt der Gegner im Frieden: alle lebenden fliehen mit dem Tempo `fleeSpeedFactor` (Taifun bremst wie sonst) und
/// verschwinden am Ausgang. Ohne Gegner kehrt der Takt sofort zurück.
pub fn step_flee(w: &mut World, dt: f64) {
    if w.enemies.is_empty() {
        return;
    }
    let Some(rule) = w.festival_rule.as_ref() else {
        return;
    };
    let (factor, margin) = (rule.flee_speed_factor, rule.exit_margin_units);
    for ei in 0..w.enemies.len() {
        let e = &w.enemies[ei];
        if e.hp <= 0.0 {
            continue; // besiegt: `remove_dead_enemies` rechnet ab
        }
        if e.crown.is_some() || e.carried.is_some() || e.carried_gold > 0 {
            drop_loot(w, ei); // Krone, Ausrüstung und Gold fallen vor der Flucht
        }
        let exit = exit_x(w, w.enemies[ei].x);
        let speed = w.enemies[ei].speed() * w.weather_speed * factor;
        let e = &mut w.enemies[ei];
        e.fleeing = true;
        e.home_x = exit;
        let d = exit - e.x;
        e.x += sign(d) * d.abs().min(speed * dt);
    }
    w.enemies.retain(|e| e.hp <= 0.0 || (e.x - e.home_x).abs() >= margin);
}
