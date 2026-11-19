//! Friedliche Insel und Fest (RU62.12, B-973, docs/rules/kampf.md § 10, bosse.md § 1 und § 2; Entscheidung 013: neue
//! Mechanik nur in Rust, ohne Go-Orakel): Ab dem Sieg über den Endboss der Insel (`Island::endboss_defeated`) enden alle
//! Angriffe in allen Stufen, und der Zustand nennt `festival` bis zur Abfahrt (der Inselwechsel baut eine neue, nicht
//! friedliche Insel); das Ereignis `festivalStart` kommt genau einmal je Sieg. Daten: data/festival.json
//! (`k3c-data/festival.rs`), die Anzeige RU62.13 liest dieselbe Datei.
//!
//! Der Frieden ist abgeleitet, nicht gespeichert (kein neues Feld im Spielstand, `ISLAND_SAVE_VERSION` bleibt): Nach dem
//! Laden gilt `endboss_defeated` wieder, die Regel kommt aus der Welt-ID. Er gilt nur in Welten aus `festival.json › worlds`
//! (Schalter, damit die Golden-Läufe unverändert bleiben) und nur beim Ziel „Endboss“ (`options.goal`); sonst Verhalten wie
//! heute.
//!
//! - `mod.rs`: Einschalten der Insel, Zustand je Takt, `festivalStart`.
//! - `flee.rs`: lebende Gegner im Frieden (fliehen, lassen Beute und Krone fallen, verschwinden).
//!
//! Wo der Frieden wirkt (jeweils `World::peace`): `world/step.rs` (keine Wachen, Flucht statt Gegner-Takt), `world/cycle.rs`
//! (keine Wellen, kein Aggressionspool, keine Nacht-Events), `portal/mod.rs` (keine Verteidiger-Wellen),
//! `events/merchant.rs` (Besuch ohne Überfall). Tag und Nacht, Händler, Wetter, Bauern, Handwerker, Schiffsbau und Beladen
//! laufen unverändert weiter; Lager der Vagabunden (`camps`) schicken keine Gegner und bleiben.
use std::sync::Arc;

use k3c_data::Data;
use serde_json::json;

use crate::events::emit;
use crate::island::Island;
use crate::state::common::event;
use crate::state::festival_view::{FestivalRule, FestivalState};
use crate::state::{IslandRef, World};

pub mod flee;

#[cfg(test)]
mod flee_tests;
#[cfg(test)]
mod peace_tests;
#[cfg(test)]
mod save_tests;
#[cfg(test)]
mod step_tests;
#[cfg(test)]
mod testutil;
#[cfg(test)]
mod work_tests;

/// Das Ziel, bei dem Frieden und Fest gelten (`options.goal`): der Endboss.
const GOAL_ENDBOSS: &str = "endboss";

/// Das Fest einer Insel (alle Stufen teilen es).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Festival {
    /// Die Regel; `None` = Insel ohne Fest (Welt ohne Eintrag in festival.json).
    pub rule: Option<Arc<FestivalRule>>,
    /// Zustand im letzten Takt (`None` vor dem ersten Takt). Der erste Takt setzt ihn nur, ohne `festivalStart`: Eine
    /// geladene Insel mit besiegtem Endboss feiert weiter, ohne das Ereignis noch einmal zu melden. Nicht gespeichert.
    pub seen: Option<bool>,
}

/// Herrscht auf der Stufe `w` Frieden? Die Regel der Welt gilt, das Ziel ist der Endboss und die Insel hat ihn besiegt
/// (`isl`; ohne Insel nie, einzelne Welten haben keinen Endboss-Sieg).
pub fn is_peace(w: &World, isl: Option<&IslandRef>) -> bool {
    w.festival_rule.is_some() && w.festival_goal && isl.is_some_and(|i| i.endboss_defeated)
}

impl Island {
    /// Schaltet das Fest der Insel ein (Aufruf beim Bauen der Insel): Die Regel kommt in alle Stufen, wenn die Welt in
    /// festival.json steht. Ohne Eintrag geschieht nichts.
    pub fn enable_festival(&mut self, data: &Data) {
        let Some(effect) = data.festival.effect_of(self.world_id()) else {
            return;
        };
        let rule = Arc::new(FestivalRule {
            effect: effect.to_string(),
            flee_speed_factor: data.festival.flee_speed_factor,
            exit_margin_units: data.festival.exit_margin_units,
        });
        for w in &mut self.stages {
            w.festival_rule = Some(Arc::clone(&rule));
        }
        self.festival = Festival {
            rule: Some(rule),
            seen: None,
        };
    }

    /// Je Takt vor den Stufen: Gilt das Ziel „Endboss“? Ohne Fest geschieht nichts, ohne Allokation.
    pub fn prepare_festival(&mut self) {
        if self.festival.rule.is_none() {
            return;
        }
        let goal = self.options.goal == GOAL_ENDBOSS;
        for w in &mut self.stages {
            w.festival_goal = goal;
        }
    }

    /// Das Fest läuft: Fest-Regel, Ziel Endboss und Endboss besiegt.
    pub fn festival_active(&self) -> bool {
        self.festival.rule.is_some() && self.options.goal == GOAL_ENDBOSS && self.endboss_defeated
    }
}

/// Ein Takt des Festes einer Insel, nach den Stufen (`step_island`): schreibt den Zustand `festival` in alle Stufen und
/// meldet `festivalStart` in Stufe 0, genau einmal beim Wechsel vom Kampf zum Sieg. Der Wechsel räumt auch, was die Stufen
/// vor dem Sieg dieses Takts noch erzeugt haben (Warteschlange, Geschosse und Bodenflächen der Gegner). Ohne Fest kehrt der
/// Takt sofort zurück, ohne zu allozieren.
pub fn step_festival(isl: &mut Island) {
    if isl.festival.rule.is_none() {
        return;
    }
    let active = isl.festival_active();
    let was = isl.festival.seen.replace(active);
    let Some(rule) = isl.festival.rule.as_ref() else {
        return;
    };
    for w in &mut isl.stages {
        if !active {
            w.festival = None;
        } else if w.festival.is_none() {
            w.festival = Some(FestivalState {
                effect: rule.effect.clone(),
            });
        }
    }
    if !active || was != Some(false) {
        return;
    }
    for w in &mut isl.stages {
        w.spawn_queue.clear();
        w.projectiles.retain(|p| p.team != "enemy");
        w.hazards.clear();
    }
    let Some(first) = isl.stages.first_mut() else {
        return;
    };
    let day = first.cycle.day;
    emit(
        first,
        "festivalStart",
        event(json!({"effect": rule.effect, "day": day})),
    );
}
