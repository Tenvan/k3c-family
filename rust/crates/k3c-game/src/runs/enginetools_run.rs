//! Port von `engine/enginetools/run.go` (RU33.1): rechnet Ticks mit einem oder mehreren Monarchen und fasst den Zustand
//! am Ende zusammen. Gleiche Eingabe ergibt denselben Text. Der Lauf selbst steckt im `Stepper` (RU33.3), den auch das
//! Fenster nutzt.

use k3c_data::Source;

use super::RunError;
use super::stepper::Stepper;
use super::stop::Stop;

/// Die Obergrenze für `sim run` (Entscheidung 🧑 2026-10-01): 30 Ticks/s, gut drei Tag/Nacht-Zyklen.
pub const MAX_TICKS: i64 = 100_000;
/// Takt der Sim je Sekunde (wie der Raum: `step(w, commands, 1/TICK_HZ)`).
pub const TICK_HZ: i64 = 30;
/// Monarchen je Lauf.
pub const MAX_PLAYERS: i64 = 4;
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

/// Go: `Run`. Vor jedem Takt prüft der Lauf `stop` (Abbruch über `--stop-stdin`).
pub fn run(
    src: &Source,
    biome_id: &str,
    seed: &str,
    ticks: i64,
    inputs: &[Segment],
    stop: &Stop,
) -> Result<String, RunError> {
    let mut s = Stepper::new(src, biome_id, seed, ticks, inputs)?;
    while !s.done() {
        if stop.is_set() {
            return Err(RunError::Aborted);
        }
        s.step();
    }
    Ok(s.finish())
}
