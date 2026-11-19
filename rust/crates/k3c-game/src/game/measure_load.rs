//! Port von `native/k3c-game/game/measure_load.go` (RU0.1, B-1019): Last-Szene des Messmodus. `--measure-load
//! <ausbau>,<truppen>,<gegner>` baut vor der Messung die Welt aus und füllt sie über die Dev-Aktionen des Raums (wie der
//! Stresstest): Ausbaustufe für alle Bauplätze, eigene Truppen als Mischung aus Kämpfern, Schützen und Bürgern, Gegner
//! als Mischung zweier Sorten. Die Spieler reiten über `--input` durch die Landschaft. Die Simulation selbst bleibt
//! unverändert.

use k3c_platform::log;

use super::Game;
use super::measure::world_state;
use crate::view::devkeys::DevAction;

/// Mengen der Last-Szene; alle 0 = keine (Go: `MeasureLoad`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MeasureLoad {
    /// Ausbaustufe aller Bauplätze (Dev-Aktion `buildAll`)
    pub build: i64,
    /// eigene Einheiten insgesamt
    pub troops: i64,
    /// Gegner insgesamt
    pub enemies: i64,
}

/// Verteilung der Truppen auf Sorten (Anteile in Prozent, Go: `loadTroops`).
pub const LOAD_TROOPS: [(&str, i64); 4] = [("warrior", 35), ("archer", 25), ("peasant", 25), ("vagrant", 15)];
/// Verteilung der Gegner auf Sorten (Anteile in Prozent, Go: `loadEnemies`).
pub const LOAD_ENEMIES: [(&str, i64); 2] = [("goblin", 60), ("skeleton", 40)];

/// Abstand in Takten, in dem die Last-Szene die Gegner prüft und auffüllt (5 Sekunden).
pub const LOAD_REFILL_TICKS: u64 = 150;

/// Go: `errNoDev` (`stress.go`, RU30.5 nimmt sie von hier): Die Session hat keine Entwickler-Rolle.
pub const ERR_NO_DEV: &str = "ohne Entwickler-Rolle der Session kein Stresstest";

/// Liest „ausbau,truppen,gegner“ (drei Zahlen ab 0); leer = keine Last-Szene.
pub fn parse_measure_load(s: &str) -> Result<MeasureLoad, String> {
    let mut l = MeasureLoad::default();
    if s.is_empty() {
        return Ok(l);
    }
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return Err(format!("🚫 --measure-load {s:?}: drei Zahlen ausbau,truppen,gegner"));
    }
    let mut n = [0i64; 3];
    for (dst, p) in n.iter_mut().zip(&parts) {
        *dst = match p.parse::<i64>() {
            Ok(v) if v >= 0 => v,
            _ => return Err(format!("🚫 --measure-load {s:?}: drei Zahlen ab 0")),
        };
    }
    (l.build, l.troops, l.enemies) = (n[0], n[1], n[2]);
    Ok(l)
}

/// Eine Aufstellung: `Amount` je Sorte nach ihrem Anteil an `total`; Sorten mit Menge 0 entfallen.
fn spawns(kind: &str, action: &str, mix: &[(&str, i64)], total: i64) -> Vec<DevAction> {
    mix.iter()
        .filter_map(|&(unit, percent)| {
            let n = total * percent / 100;
            (n > 0).then(|| DevAction {
                action: action.to_string(),
                kind: kind.to_string(),
                unit: unit.to_string(),
                amount: n,
                ..DevAction::default()
            })
        })
        .collect()
}

impl Game {
    /// Hält die Last: Lebt weniger als die Hälfte der Gegner, kommt die Hälfte neu dazu, an der Burg (beim Spieler
    /// entstanden, rauben sie sofort die Krone und das Spiel endet). Go: `refillMeasureLoad`.
    pub(super) fn refill_measure_load(&mut self) {
        let l = self.mload;
        if l.enemies == 0 || !self.frame.tick.is_multiple_of(LOAD_REFILL_TICKS) || self.frame.seats.is_empty() {
            return;
        }
        if world_state(&self.frame).1 as i64 >= l.enemies / 2 {
            return;
        }
        let player = self.frame.seats[0].player;
        let acts = spawns("enemy", "spawnCastle", &LOAD_ENEMIES, l.enemies / 2);
        let Some(ds) = self.sim.dev_session() else {
            return;
        };
        for a in &acts {
            if let Err(e) = ds.dev(player, a) {
                log::warn("🚫 Last-Szene: Gegner nicht aufgefüllt", &[("err", &e)]);
            }
        }
    }

    /// Liest die Angabe `spec` und führt die Last-Szene aus (braucht `--tick` ab 1). Go: `measureLoad`.
    pub(super) fn measure_load(&mut self, spec: &str) -> Result<(), String> {
        let l = parse_measure_load(spec)?;
        self.apply_measure_load(l)
    }

    /// Führt die Last-Szene aus (nach dem ersten Takt, wenn die Plätze der Spieler stehen). Go: `applyMeasureLoad`.
    fn apply_measure_load(&mut self, l: MeasureLoad) -> Result<(), String> {
        if l == MeasureLoad::default() {
            return Ok(());
        }
        let Some(seat) = self.frame.seats.first() else {
            return Err(ERR_NO_DEV.to_string());
        };
        let player = seat.player;
        let mut acts = Vec::new();
        let act = |name: &str, level: i64| DevAction {
            action: name.to_string(),
            level,
            ..DevAction::default()
        };
        if l.build > 0 {
            acts.push(act("buildAll", l.build));
        }
        if l.enemies > 0 {
            acts.push(act("night", 0)); // Gegner halten sich nur nachts
        }
        acts.extend(spawns("troop", "spawn", &LOAD_TROOPS, l.troops));
        acts.extend(spawns("enemy", "spawnCastle", &LOAD_ENEMIES, l.enemies));
        let Some(ds) = self.sim.dev_session() else {
            return Err(ERR_NO_DEV.to_string());
        };
        for a in &acts {
            if let Err(e) = ds.dev(player, a) {
                log::warn(
                    "🚫 Last-Szene: Dev-Aktion abgelehnt",
                    &[("aktion", &a.action), ("einheit", &a.unit), ("err", &e)],
                );
            }
        }
        self.mload = l;
        log::info(
            "🐛 Last-Szene",
            &[("ausbau", &l.build), ("truppen", &l.troops), ("gegner", &l.enemies)],
        );
        Ok(())
    }
}

#[cfg(test)]
#[path = "measure_load_tests.rs"]
mod tests;
