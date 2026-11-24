//! Port von `engine/enginetools/run.go` (RU33.1): rechnet Ticks mit einem oder mehreren Monarchen und fasst den Zustand
//! am Ende zusammen. Gleiche Eingabe ergibt denselben Text.

use std::collections::BTreeMap;

use k3c_data::Source;
use k3c_sim::state::{PlayerCommand, World};
use k3c_sim::world::step::step;
use k3c_sim::world::{Options, add_player, create_world};

use super::RunError;
use super::enginetools_level::load_biome;
use super::gotext::quote;
use super::stop::Stop;

/// Die Obergrenze für `sim run` (Entscheidung 🧑 2026-10-01): 30 Ticks/s, gut drei Tag/Nacht-Zyklen.
pub const MAX_TICKS: i64 = 100_000;
/// Takt der Sim je Sekunde (wie der Raum: `step(w, commands, 1/TICK_HZ)`).
pub const TICK_HZ: i64 = 30;
const MAX_PLAYERS: i64 = 4;
/// Begrenzt `inputs`: jeder Tick prüft alle Segmente.
pub const MAX_SEGMENTS: usize = 100;

/// Hält die Eingabe eines Monarchen von `from_tick` (einschließlich) bis `to_tick` (ausschließlich) gedrückt.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Segment {
    pub player: i64,
    pub from_tick: i64,
    pub to_tick: i64,
    pub move_x: f64,
    pub sprint: bool,
    pub pay: bool,
}

fn failed(msg: String) -> RunError {
    RunError::Failed(msg)
}

/// Go: `Run`. Vor jedem Takt prüft der Lauf `stop` (Abbruch über `--stop-stdin`).
pub fn run(
    src: &Source,
    biome_id: &str,
    seed: &str,
    ticks: i64,
    inputs: &[Segment],
    stop: &Stop,
) -> Result<String, RunError> {
    if !(1..=MAX_TICKS).contains(&ticks) {
        return Err(failed(format!(
            "ticks muss zwischen 1 und {MAX_TICKS} liegen (30 Ticks pro Sekunde), war {ticks}"
        )));
    }
    let players = player_count(inputs)?;
    let b = load_biome(src, biome_id)?;
    let mut w =
        create_world(&b, seed, &Options::default()).map_err(|e| failed(format!("welt nicht erzeugbar: {e}")))?;
    for _ in 0..players {
        add_player(&mut w);
    }
    let mut events: BTreeMap<String, i64> = BTreeMap::new();
    let mut first_fall = 0;
    for tick in 0..ticks {
        if stop.is_set() {
            return Err(RunError::Aborted);
        }
        step(&mut w, &commands_at(inputs, players, tick), None, 1.0 / TICK_HZ as f64);
        for e in &w.events {
            if let Some(kind) = e.get("type").and_then(|t| t.as_str()).filter(|k| !k.is_empty()) {
                *events.entry(kind.to_string()).or_insert(0) += 1;
                if kind == "castleFallen" && first_fall == 0 {
                    first_fall = tick + 1;
                }
            }
        }
    }
    Ok(summary(&w, &b.id, seed, ticks, first_fall, &events))
}

/// Go: `playerCount`: die Zahl der Monarchen, geprüft an den Segmenten.
fn player_count(inputs: &[Segment]) -> Result<i64, RunError> {
    if inputs.len() > MAX_SEGMENTS {
        return Err(failed(format!(
            "inputs hat {} Segmente, höchstens {MAX_SEGMENTS} erlaubt",
            inputs.len()
        )));
    }
    let mut players = 1;
    for (i, s) in inputs.iter().enumerate() {
        if s.player < 0 || s.player >= MAX_PLAYERS {
            return Err(failed(format!(
                "inputs[{i}]: player muss zwischen 0 und {} liegen",
                MAX_PLAYERS - 1
            )));
        }
        if s.to_tick < s.from_tick || s.from_tick < 0 {
            return Err(failed(format!(
                "inputs[{i}]: fromTick..toTick ungültig ({}..{})",
                s.from_tick, s.to_tick
            )));
        }
        players = players.max(s.player + 1);
    }
    Ok(players)
}

/// Go: `commandsAt`: die Eingaben aller Segmente, die im Tick gelten; spätere überschreiben frühere.
fn commands_at(inputs: &[Segment], players: i64, tick: i64) -> Vec<PlayerCommand> {
    let mut cmds = vec![PlayerCommand::default(); players as usize];
    for s in inputs {
        if tick >= s.from_tick && tick < s.to_tick {
            cmds[s.player as usize] = PlayerCommand {
                move_x: s.move_x,
                sprint: s.sprint,
                pay: s.pay,
                ..PlayerCommand::default()
            };
        }
    }
    cmds
}

fn summary(w: &World, biome: &str, seed: &str, ticks: i64, first_fall: i64, events: &BTreeMap<String, i64>) -> String {
    let n = |k: &str| events.get(k).copied().unwrap_or(0);
    let gold: Vec<String> = w.players.iter().map(|p| p.gold.to_string()).collect();
    let mut troops: BTreeMap<String, i64> = BTreeMap::new();
    for t in &w.troops {
        *troops.entry(t.data_kind().to_string()).or_insert(0) += 1;
    }
    let end = if first_fall > 0 {
        format!(
            "Ticks erreicht, Burg {}× gefallen (erstmals bei Tick {first_fall})",
            n("castleFallen")
        )
    } else {
        "Ticks erreicht, Burg nie gefallen".to_string()
    };
    [
        format!(
            "sim_run {biome} · Seed {} · {ticks} Ticks ({:.1} s Spielzeit)",
            quote(seed),
            ticks as f64 / TICK_HZ as f64
        ),
        format!(
            "Tag {} ({}) · Welle {} · Gegner {} · Burg {:.0} HP",
            w.cycle.day,
            w.cycle.phase,
            w.wave,
            w.enemies.len(),
            w.castle.hp
        ),
        format!("Gold je Monarch: {}", gold.join(", ")),
        format!("Truppen: {}", counts(&troops)),
        format!(
            "Verluste: {} Gebäude zerstört, {} Monarchen am Boden · Gold gestohlen {}×",
            n("destroyed"),
            n("crownFallen"),
            n("goldStolen")
        ),
        format!("Ende: {end}"),
    ]
    .join("\n")
}

/// Go: `counts`: eine Zählung sortiert („archer 1, peasant 3“), damit der Text deterministisch bleibt.
fn counts(m: &BTreeMap<String, i64>) -> String {
    if m.is_empty() {
        return "keine".to_string();
    }
    m.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", ")
}
