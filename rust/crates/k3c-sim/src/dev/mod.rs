//! Dev-Aktionen der Insel (Port von `engine/sim/dev.go`, B-178): nur für den Dev-Mode des Raums, keine Spielregel.
//!
//! Alle `Dev*(isl *Island, …)` sind freie Funktionen `dev_*(isl: &mut Island, …)`. Wo Go `w.island` liest, rechnet die
//! Stufe wie in `step_island` mit einem `IslandRef` und dem getauschten Vorrat (`on_stage`); `difficulty` ist dafür die
//! Grad-Tabelle wie bei `step_island`. `pure` liest den Spiegel `World::pure` (von `dev_set_pure` gesetzt).

mod build;
pub mod combat;
pub mod spawn;

#[cfg(test)]
mod cheat_tests;
#[cfg(test)]
mod combat_tests;
#[cfg(test)]
mod mod_tests;
#[cfg(test)]
mod spawn_tests;
#[cfg(test)]
mod stress_tests;
#[cfg(test)]
mod testutil;

use std::collections::BTreeMap;

use k3c_data::difficulty::DifficultyFile;

pub use build::{dev_build_all, dev_build_site};

use crate::enemies::waves::start_wave;
use crate::island::Island;
use crate::island::storage::{add_stock_capped, capacity, stock_field};
use crate::state::links::values;
use crate::state::purse::purse_max_of;
use crate::state::{Coin, IslandRef, World};
use crate::world::base::base;
use crate::world::castle_losses;
use crate::world::cycle::{cycle_at, cycle_phases};

/// Go: `DevFlags`: die Dev-Schalter einer Insel (nur im Speicher, nie im Spielstand).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DevFlags {
    /// Alle Bau-, Ausbau- und Angebotskosten höchstens 1 je Posten.
    pub cheap_build: bool,
    /// Reiner Stressmodus (`dev_set_pure`): keine Wellen, Lager, Wachen, Händler, Schänke.
    pub pure: bool,
    /// Zusätzliches Lager-Maximum je Rohstoff und Gold-Maximum je Spieler (`devMaxStock`).
    pub stock_bonus: i64,
    /// Event-Id → Nacht, in der das Event zusätzlich zum Rhythmus läuft (`dev_start_event`, `combat.rs`).
    pub nights: BTreeMap<String, i64>,
}

/// Go: `DevStockStep`: Schritt des zusätzlichen Lager-Maximums.
pub const DEV_STOCK_STEP: i64 = 100;
/// Go: `DevStockBonusMax`: Obergrenze des zusätzlichen Lager-Maximums.
pub const DEV_STOCK_BONUS_MAX: i64 = 10000;

/// Go: `World.pure`: Die Insel der Stufe läuft im reinen Stressmodus (`dev_set_pure`).
pub fn pure(w: &World) -> bool {
    w.pure
}

/// Ruft `f` auf der Stufe `i` wie `step_island` (Entscheidung 4 und 5): der Vorrat der Insel liegt per `swap` in der Stufe,
/// der `IslandRef` kommt aus der Insel; danach geht beides zurück und der Vorrat wird in alle Stufen gespiegelt.
pub(crate) fn on_stage<R>(
    isl: &mut Island,
    difficulty: &DifficultyFile,
    i: usize,
    f: impl FnOnce(&mut World, &mut IslandRef) -> R,
) -> R {
    let mut r = isl.stage_ref(i, difficulty);
    std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock);
    let out = f(&mut isl.stages[i], &mut r);
    std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock);
    isl.put_stage_ref(r);
    isl.sync_stock();
    out
}

/// Go: `DevDropGold`: lässt `amount` Münzen beim Spieler `index` in seiner Stufe fallen. Alle liegen bei seinem X, ohne
/// rng-Aufruf, damit der Zufallsstrom unberührt bleibt. `false`: kein Spieler.
pub fn dev_drop_gold(isl: &mut Island, index: i64, amount: i64) -> bool {
    let Some(stage) = isl.stage_of(index) else {
        return false;
    };
    let w = &mut isl.stages[stage];
    for pi in 0..w.players.len() {
        if w.players[pi].index != index {
            continue;
        }
        let x = w.players[pi].x;
        for _ in 0..amount {
            let id = w.new_id();
            w.coins.push(Coin {
                id,
                x,
                ..Default::default()
            });
        }
    }
    true
}

/// Go: `DevAddStock`: legt Material in den Vorrat der Insel des Spielers `index`, höchstens bis zum Lager-Maximum; der Rest
/// wird verworfen. `ok` false: unbekannter Rohstoff oder Spieler.
pub fn dev_add_stock(
    isl: &mut Island,
    difficulty: &DifficultyFile,
    index: i64,
    resource: &str,
    amount: i64,
) -> (i64, bool) {
    let Some(stage) = isl.stage_of(index) else {
        return (0, false);
    };
    on_stage(isl, difficulty, stage, |w, r| {
        let v = values(w);
        add_stock_capped(w, &v, Some(r), resource, amount)
    })
}

/// Go: `DevStartWave`: startet sofort eine Welle in der Stufe des Spielers `index`, wie bei Nachtbeginn (B-232).
/// `false`: kein Spieler.
pub fn dev_start_wave(isl: &mut Island, difficulty: &DifficultyFile, index: i64) -> bool {
    let Some(stage) = isl.stage_of(index) else {
        return false;
    };
    on_stage(isl, difficulty, stage, |w, r| start_wave(w, Some(r)));
    true
}

/// Go: `DevCastleFall`: löst in der Stufe des Spielers `index` den Stufenverlust aus (`castle_losses`), unabhängig vom
/// Niederlage-Modus. `false`: kein Spieler.
pub fn dev_castle_fall(isl: &mut Island, difficulty: &DifficultyFile, index: i64) -> bool {
    let Some(stage) = isl.stage_of(index) else {
        return false;
    };
    on_stage(isl, difficulty, stage, |w, _| castle_losses(w));
    true
}

/// Go: `DevSetPhase`: lässt die Zeit aller Stufen vorwärts zum Beginn der nächsten Phase (day, dusk, night, dawn)
/// springen (B-232). Der Zyklus steht danach kurz vor dem Wechsel, damit der nächste Schritt ihn samt Ereignissen wie
/// sonst auslöst, auch wenn die Stufe schon in dieser Phase war. `false`: unbekannte Phase.
pub fn dev_set_phase(isl: &mut Island, phase: &str) -> bool {
    let Ok(b) = base() else {
        return false;
    };
    let (phases, length) = cycle_phases(&b.day_night);
    let Some(p) = phases.iter().find(|p| p.name == phase) else {
        return false;
    };
    for w in &mut isl.stages {
        let now = w.time * w.cycle_speed;
        let mut target = (now / length).floor() * length + p.start;
        if target <= now {
            target += length;
        }
        w.time = target / w.cycle_speed;
        w.cycle = cycle_at(&b.day_night, target - 0.001);
    }
    true
}

/// Go: `DevRaiseMaxStock`: erhöht das Lager-Maximum der Insel und das Gold-Maximum aller Spieler um `DEV_STOCK_STEP`;
/// `false` an der Obergrenze.
pub fn dev_raise_max_stock(isl: &mut Island) -> bool {
    if isl.dev.stock_bonus + DEV_STOCK_STEP > DEV_STOCK_BONUS_MAX {
        return false;
    }
    isl.dev.stock_bonus += DEV_STOCK_STEP;
    true
}

/// Go: `DevFillStock`: füllt jeden Rohstoff der Insel bis zum Lager-Maximum und das Gold aller Spieler bis zum
/// Gold-Maximum (ein größerer Vorrat bleibt).
pub fn dev_fill_stock(isl: &mut Island, difficulty: &DifficultyFile) {
    for i in 0..isl.stages.len() {
        on_stage(isl, difficulty, i, |w, r| {
            for pi in 0..w.players.len() {
                let max = purse_max_of(w, &w.players[pi], Some(r));
                w.players[pi].gold = w.players[pi].gold.max(max);
            }
        });
    }
    if isl.stages.is_empty() {
        return;
    }
    let Some(limit) = on_stage(isl, difficulty, 0, |w, r| capacity(w, &values(w), Some(r))) else {
        return;
    };
    for res in ["wood", "stone", "copper", "iron", "crystal"] {
        if let Some(f) = stock_field(&mut isl.stock, res).filter(|f| **f < limit) {
            *f = limit;
        }
    }
    isl.sync_stock();
}

/// Go: `DevSetPure`: schaltet den reinen Stressmodus (B-921) ein oder aus. Ein: natürliche Wellen (`start_wave`),
/// Rekrutenlager, Händler und Schänke ruhen, vorhandene Landstreicher, Wachen und wartende Gegner verschwinden; Gegner und
/// Truppen kommen nur noch über die Dev-Aktion spawn bzw. spawnCastle.
pub fn dev_set_pure(isl: &mut Island, on: bool) {
    isl.dev.pure = on;
    for w in &mut isl.stages {
        w.pure = on;
        if !on {
            continue;
        }
        w.troops.retain(|t| t.kind != "vagrant");
        w.enemies.retain(|e| !e.guard);
        w.spawn_queue.clear();
    }
}

/// Go: `DevSetCheapBuild`: schaltet die Baukosten 1 ein oder aus (Preise stehen nicht im Zustand; Bezahltes bleibt bezahlt).
pub fn dev_set_cheap_build(isl: &mut Island, on: bool) {
    isl.dev.cheap_build = on;
}

/// Go: `DevNextPhase`: springt zum Beginn der nächsten Tageszeit (day → dusk → night → dawn → day) und liefert ihren Namen.
pub fn dev_next_phase(isl: &mut Island) -> &'static str {
    let phase = isl.stages.first().map_or("", |w| w.cycle.phase.as_str());
    let next = match phase {
        "day" => "dusk",
        "dusk" => "night",
        "night" => "dawn",
        "dawn" => "day",
        _ => "dusk",
    };
    dev_set_phase(isl, next);
    next
}
