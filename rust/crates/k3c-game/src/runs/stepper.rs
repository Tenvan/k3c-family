//! Der gemeinsame Lauf von `sim run` (RU33.3): Welt, Takt, Eingabe-Segmente und Zähler. Headless (`run`) und Fenster
//! (`RunApp`) rechnen mit demselben `Stepper`, darum hängt das Ergebnis nicht vom Modus und nicht vom Zeitraffer ab.

use std::collections::BTreeMap;

use k3c_data::Source;
use k3c_sim::state::{PlayerCommand, World};
use k3c_sim::world::step::step;
use k3c_sim::world::{Options, add_player, create_world};

use super::RunError;
use super::enginetools_level::load_biome;
use super::enginetools_run::{MAX_PLAYERS, MAX_SEGMENTS, MAX_TICKS, Segment, TICK_HZ};
use super::gotext::quote;

fn failed(msg: String) -> RunError {
    RunError::Failed(msg)
}

/// Ein laufender `sim run`: rechnet je `step` genau einen Sim-Takt mit `dt = 1/30` (Gos `TickHz`).
pub struct Stepper {
    w: World,
    inputs: Vec<Segment>,
    players: i64,
    ticks: i64,
    tick: i64,
    events: BTreeMap<String, i64>,
    first_fall: i64,
    biome: String,
    seed: String,
}

impl Stepper {
    /// Prüft die Eingabe wie Gos `Run` und legt die Welt an (Biom, Seed, Monarchen).
    pub fn new(src: &Source, biome_id: &str, seed: &str, ticks: i64, inputs: &[Segment]) -> Result<Stepper, RunError> {
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
        Ok(Stepper {
            w,
            inputs: inputs.to_vec(),
            players,
            ticks,
            tick: 0,
            events: BTreeMap::new(),
            first_fall: 0,
            biome: b.id,
            seed: seed.to_string(),
        })
    }

    /// Sind alle Takte gerechnet?
    pub fn done(&self) -> bool {
        self.tick >= self.ticks
    }

    /// Die Welt nach dem letzten gerechneten Takt.
    pub fn world(&self) -> &World {
        &self.w
    }

    /// Zahl der gerechneten Takte.
    pub fn tick(&self) -> i64 {
        self.tick
    }

    /// Zahl der Monarchen.
    pub fn players(&self) -> i64 {
        self.players
    }

    /// Rechnet einen Takt und zählt seine Ereignisse; nach dem Ende ohne Wirkung.
    pub fn step(&mut self) {
        if self.done() {
            return;
        }
        let cmds = commands_at(&self.inputs, self.players, self.tick);
        step(&mut self.w, &cmds, None, 1.0 / TICK_HZ as f64);
        for e in &self.w.events {
            if let Some(kind) = e.get("type").and_then(|t| t.as_str()).filter(|k| !k.is_empty()) {
                *self.events.entry(kind.to_string()).or_insert(0) += 1;
                if kind == "castleFallen" && self.first_fall == 0 {
                    self.first_fall = self.tick + 1;
                }
            }
        }
        self.tick += 1;
    }

    /// Der Text der Zeile `result` (Go: `summary`).
    pub fn finish(&self) -> String {
        let n = |k: &str| self.events.get(k).copied().unwrap_or(0);
        let w = &self.w;
        let gold: Vec<String> = w.players.iter().map(|p| p.gold.to_string()).collect();
        let mut troops: BTreeMap<String, i64> = BTreeMap::new();
        for t in &w.troops {
            *troops.entry(t.data_kind().to_string()).or_insert(0) += 1;
        }
        let end = if self.first_fall > 0 {
            format!(
                "Ticks erreicht, Burg {}× gefallen (erstmals bei Tick {})",
                n("castleFallen"),
                self.first_fall
            )
        } else {
            "Ticks erreicht, Burg nie gefallen".to_string()
        };
        [
            format!(
                "sim_run {} · Seed {} · {} Ticks ({:.1} s Spielzeit)",
                self.biome,
                quote(&self.seed),
                self.ticks,
                self.ticks as f64 / TICK_HZ as f64
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

/// Go: `counts`: eine Zählung sortiert („archer 1, peasant 3“), damit der Text deterministisch bleibt.
fn counts(m: &BTreeMap<String, i64>) -> String {
    if m.is_empty() {
        return "keine".to_string();
    }
    m.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", ")
}
