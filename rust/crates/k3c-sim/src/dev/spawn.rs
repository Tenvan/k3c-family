//! Dev-Aktion spawn (Port von `engine/sim/dev_spawn.go`, B-766, RX1): setzt Gegner oder Truppen in die Stufe eines
//! Spielers, für den Stresstest des echten Clients. Nur für den Dev-Mode des Raums, keine Spielregel. Die Figuren stehen
//! gleichmäßig verteilt um den Spieler (kein rng, damit der Strom des Spiels unberührt bleibt).

use k3c_data::difficulty::DifficultyFile;

use super::on_stage;
use crate::enemies::spawn_enemy;
use crate::island::Island;
use crate::state::links::values;
use crate::state::{Fehler, Troop};

/// Go: `DevMaxSpawn`: die größte Zahl je Aufruf; Aufrufe lassen sich wiederholen, die Obergrenze schützt das Tick-Budget.
pub const DEV_MAX_SPAWN: i64 = 200;

/// Go: `devSpawnSpan`: Breite (Units), auf die sich die gesetzten Figuren verteilen, um den Spieler zentriert.
const DEV_SPAWN_SPAN: f64 = 40.0;

/// Go: `DevSpawnUnits`: stellt `count` Figuren der Art `kind` ("enemy" oder "troop") und der Sorte `unit` (Schlüssel aus
/// data/enemies.json bzw. data/troops.json) in die Stufe des Spielers `index`. Liefert die Zahl der Figuren; Fehler:
/// unbekannter Spieler, Art oder Sorte oder `count` außerhalb von 1…`DEV_MAX_SPAWN`.
pub fn dev_spawn_units(
    isl: &mut Island,
    difficulty: &DifficultyFile,
    index: i64,
    kind: &str,
    unit: &str,
    count: i64,
) -> Result<i64, Fehler> {
    dev_spawn(isl, difficulty, index, kind, unit, count, false)
}

/// Go: `DevSpawnAtCastle`: wie `dev_spawn_units`, aber um die Burg der Stufe statt um den Spieler, auf die Breite des
/// Burgradius (Stressmodus, B-921).
pub fn dev_spawn_at_castle(
    isl: &mut Island,
    difficulty: &DifficultyFile,
    index: i64,
    kind: &str,
    unit: &str,
    count: i64,
) -> Result<i64, Fehler> {
    dev_spawn(isl, difficulty, index, kind, unit, count, true)
}

fn dev_spawn(
    isl: &mut Island,
    difficulty: &DifficultyFile,
    index: i64,
    kind: &str,
    unit: &str,
    count: i64,
    at_castle: bool,
) -> Result<i64, Fehler> {
    let stage = isl.stage_of(index).ok_or(Fehler::DevInvalid)?;
    if !(1..=DEV_MAX_SPAWN).contains(&count) {
        return Err(Fehler::DevInvalid);
    }
    let w = &isl.stages[stage];
    let pi = w
        .players
        .iter()
        .position(|p| p.index == index)
        .ok_or(Fehler::DevInvalid)?;
    let v = values(w);
    match kind {
        "enemy" if v.enemies.contains_key(unit) => {}
        "troop" if v.troops.contains_key(unit) => {}
        _ => return Err(Fehler::DevInvalid),
    }
    let (center, span) = if at_castle {
        (w.hub_x, 2.0 * v.hub.castle_radius_units)
    } else {
        (w.players[pi].x, DEV_SPAWN_SPAN)
    };
    on_stage(isl, difficulty, stage, |w, r| {
        for n in 0..count {
            let x = dev_spawn_x(center, span, n, count).max(0.0).min(w.width_units);
            if kind == "enemy" {
                spawn_enemy(w, unit, x, Some(r));
                continue;
            }
            let hp = v.troops.get(unit).map_or(0.0, |t| t.hp);
            let id = w.new_id();
            let anchor_x = w.hub_x;
            w.troops.push(Troop {
                id,
                kind: unit.to_string(),
                x,
                hp,
                max_hp: hp,
                anchor_x,
                target_x: x,
                ..Default::default()
            });
        }
    });
    Ok(count)
}

/// Go: `devSpawnX`: verteilt Figur `n` von `count` gleichmäßig auf `span` Units um `center`.
pub fn dev_spawn_x(center: f64, span: f64, n: i64, count: i64) -> f64 {
    if count == 1 {
        return center;
    }
    center + (n as f64 / (count - 1) as f64 - 0.5) * span
}
