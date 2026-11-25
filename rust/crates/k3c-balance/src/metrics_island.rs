//! Kennzahlen eines Insel-Laufs mit Welt (Port von `engine/balance/metrics_island.go`, BS11, B-642): je Tiefe der Insel
//! Ankunft und Burgfall, besiegte Bosse mit Tag, Endboss und Sieg. Ohne Welt (eine Stufe) bleiben alle Felder leer und
//! stehen nicht im JSON.

use k3c_sim::island::Island;
use k3c_sim::state::{Event, World};
use serde::{Deserialize, Serialize};

use crate::bots::economy_view;
use crate::metrics::{Collector, first};
use crate::pending::{load_ship, ship_needs};

/// Eine Stufe der Insel (Reihenfolge = Stufenindex). `arrival_*` ist der erste Tick, an dem ein Monarch auf der Stufe
/// stand (Stufe 0: Tick 0); `null`: nie erreicht.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct StageMetrics {
    pub depth: i64,
    pub arrival_day: Option<i64>,
    pub arrival_tick: Option<i64>,
    pub castle_fall_tick: Option<i64>,
    /// Ereignisse `crownFallen` auf dieser Stufe (ein Monarch fiel).
    #[serde(skip_serializing_if = "is_zero")]
    pub downs: i64,
    /// Hub-Stufe am Ende des Laufs.
    #[serde(skip_serializing_if = "is_zero")]
    pub hub_level: i64,
    /// Kämpfer (Truppen außer Landstreichern und Bauern) am Ende.
    #[serde(skip_serializing_if = "is_zero")]
    pub fighters: i64,
}

/// Ein besiegter Boss (Ereignis `bossDefeated`) mit Tag und Tick.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BossMetrics {
    pub boss: String,
    pub day: i64,
    pub tick: i64,
}

fn is_zero(v: &i64) -> bool {
    *v == 0
}

impl Collector {
    fn multi(isl: &Island) -> bool {
        isl.stages.len() > 1
    }

    /// Legt die Zeilen der Stufen an (nur Insel-Lauf).
    pub(crate) fn init_stages(&mut self, isl: &Island) {
        if !Self::multi(isl) {
            return;
        }
        for w in &isl.stages {
            self.m.stages.push(StageMetrics {
                depth: w.biome.depth,
                ..StageMetrics::default()
            });
        }
        self.observe_island(isl, 0);
    }

    /// Merkt Ankünfte und den Endboss-Sieg nach Tick `tick`.
    pub(crate) fn observe_island(&mut self, isl: &Island, tick: i64) {
        if !Self::multi(isl) {
            return;
        }
        for (s, w) in self.m.stages.iter_mut().zip(&isl.stages) {
            if s.arrival_tick.is_none() && !w.players.is_empty() {
                (s.arrival_tick, s.arrival_day) = (Some(tick), Some(w.cycle.day));
            }
        }
        if isl.endboss_defeated && self.m.endboss_tick.is_none() {
            self.m.endboss_tick = Some(tick);
            self.m.endboss_day = Some(isl.stages[0].cycle.day);
        }
    }

    /// Verbucht Ereignisse der Stufe `w`, die nur der Insel-Lauf misst.
    pub(crate) fn island_event(&mut self, isl: &Island, w: &World, ev: &Event, tick: i64) {
        if !Self::multi(isl) {
            return;
        }
        let boss = || BossMetrics {
            boss: ev.get("boss").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            day: w.cycle.day,
            tick,
        };
        let stage = isl.stages.iter().position(|s| std::ptr::eq(s, w));
        match ev.get("type").and_then(|v| v.as_str()).unwrap_or("") {
            "bossDefeated" => self.m.bosses.push(boss()),
            "bossSpawned" => self.m.boss_spawns.push(boss()),
            "crownFallen" => {
                if let Some(i) = stage {
                    self.m.stages[i].downs += 1;
                }
            }
            "castleFallen" => {
                if let Some(i) = stage {
                    first(&mut self.m.stages[i].castle_fall_tick, tick);
                }
            }
            "victory" => first(&mut self.m.victory_tick, tick),
            _ => {}
        }
    }

    /// Rechnet am Ende des Insel-Laufs den Bedarf gegen den Bestand (ship_need.go).
    pub(crate) fn ship_need(&mut self, isl: &Island) {
        if !Self::multi(isl) {
            return;
        }
        for (s, w) in self.m.stages.iter_mut().zip(&isl.stages) {
            let e = economy_view(w);
            (s.hub_level, s.fighters) = (e.hub_level, e.fighters);
        }
        if let Ok(stages) = load_ship() {
            self.m.ship = ship_needs(&stages, &self.m);
        }
    }
}
