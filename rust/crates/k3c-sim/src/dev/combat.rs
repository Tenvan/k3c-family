//! Dev-Aktionen für den Kampf (Port von `engine/sim/dev_combat.go`, B-603): Boss, Phasenwechsel des Endbosses, Events und
//! Nacht sofort. Sie nutzen die Abläufe des Spiels (`spawn_boss`, `endboss_phase`, Nachtbeginn, Händler-Überfall) und
//! damit deren rng-Strom, sonst kein rng.

use k3c_data::difficulty::DifficultyFile;

use super::{dev_set_phase, on_stage};
use crate::boss::abilities::arm_boss;
use crate::boss::data::boss_by_id;
use crate::boss::endboss::{EndbossFight, endboss_phase};
use crate::boss::spawn_boss;
use crate::events::merchant::raid_merchant;
use crate::island::Island;
use crate::merchant::merchant_arrive;
use crate::state::pending::values;
use crate::state::{Fehler, World};
use crate::world::hub_of;

/// Go: `DevSpawnBoss`: stellt den Boss `id` (data/bosses.json, Mini- oder Endboss) in der Stufe des Spielers `index` an
/// das nächste Portal; ein Endboss beginnt dort seinen Kampf mit Phase 1. Besiegte Bosse kommen trotzdem.
pub fn dev_spawn_boss(isl: &mut Island, difficulty: &DifficultyFile, index: i64, id: &str) -> Result<(), Fehler> {
    let stage = isl.stage_of(index).ok_or(Fehler::DevInvalid)?;
    let v = values(&isl.stages[stage]);
    let b = boss_by_id(&v.bosses, id).ok_or(Fehler::DevInvalid)?;
    let w = &isl.stages[stage];
    if isl.endboss.is_some() || w.enemies.iter().any(|e| e.boss) {
        return Err(Fehler::DevBossPresent);
    }
    let x = nearest_portal(w, index).ok_or(Fehler::DevInvalid)?;
    on_stage(isl, difficulty, stage, |w, r| {
        let ei = spawn_boss(w, &v, b, x, Some(r));
        if b.kind == "end" {
            if let Some(a) = b.phases.first().and_then(|p| p.ability.as_ref()) {
                arm_boss(&mut w.enemies[ei], a);
            }
            r.endboss = Some(EndbossFight {
                boss_id: w.enemies[ei].id,
                home: stage,
                ..Default::default()
            });
        }
    });
    Ok(())
}

/// Go: `nearestPortal`: das Portal der Stufe am nächsten beim Spieler `index` (bei Gleichstand das erste); `None`: keins.
fn nearest_portal(w: &World, index: i64) -> Option<f64> {
    let p = w.players.iter().find(|p| p.index == index)?;
    let (first, rest) = w.portals.split_first()?;
    let mut best = *first;
    for &x in rest {
        if (x - p.x).abs() < (best - p.x).abs() {
            best = x;
        }
    }
    Some(best)
}

/// Go: `DevBossPhase`: wechselt den laufenden Endboss in seine nächste Phase: Seine HP fallen knapp unter deren Schwelle
/// (höher bleiben sie nie), der Wechsel samt Meldung läuft über `endboss_phase`. Liefert die neue Phase (ab 1); `None`:
/// kein Endboss-Kampf oder schon die letzte Phase.
pub fn dev_boss_phase(isl: &mut Island) -> Option<i64> {
    let mut f = isl.endboss.take()?;
    let out = boss_phase_of(isl, &mut f);
    isl.endboss = Some(f);
    out
}

fn boss_phase_of(isl: &mut Island, f: &mut EndbossFight) -> Option<i64> {
    let w = isl.stages.get_mut(f.home)?;
    let v = values(w);
    let kind = w.enemies.iter().find(|e| e.id == f.boss_id)?.kind.clone();
    let b = boss_by_id(&v.bosses, &kind)?;
    let next = b.phases.get(f.phase + 1)?;
    let e = w.enemies.iter_mut().find(|e| e.id == f.boss_id)?;
    e.hp =
        e.hp.min((next.below_hp_percent * e.max_hp / 100.0).floor() - 1.0)
            .max(1.0);
    endboss_phase(w, &v, f, 0.0);
    Some(f.phase as i64 + 1)
}

/// Go: `DevStartEvent`: startet ein Event aus data/events.json: `fullMoon` und `bloodMoon` laufen in der nächsten Nacht,
/// zu deren Beginn alle Stufen springen (`dev_set_phase`); `merchantRaid` lässt den Händler in seiner Stufe ankommen, wenn
/// er fehlt, und überfällt seinen Besuch. `false`: unbekannte Id, keine Händler-Stufe oder der Besuch wird schon überfallen.
pub fn dev_start_event(isl: &mut Island, difficulty: &DifficultyFile, id: &str) -> bool {
    match id {
        "fullMoon" | "bloodMoon" => {
            if !dev_set_phase(isl, "night") {
                return false;
            }
            let day = isl.stages.first().map_or(0, |w| w.cycle.day);
            isl.dev.nights.insert(id.to_string(), day); // die Nacht gehört zum Tag davor
            true
        }
        "merchantRaid" => dev_merchant_raid(isl, difficulty),
        _ => false,
    }
}

/// Go: `devMerchantRaid`: Händler-Überfall in der Stufe des Händlers (`hub.merchant.onlyDepth`).
fn dev_merchant_raid(isl: &mut Island, difficulty: &DifficultyFile) -> bool {
    let Some(stage) = isl
        .stages
        .iter()
        .position(|w| w.biome.depth == hub_of(w).merchant.only_depth)
    else {
        return false;
    };
    on_stage(isl, difficulty, stage, |w, r| {
        if w.merchant.is_none() {
            let v = values(w);
            merchant_arrive(w, &v, Some(r));
        }
        if w.merchant.as_ref().is_some_and(|m| m.raid) {
            return false;
        }
        raid_merchant(w, r);
        true
    })
}
