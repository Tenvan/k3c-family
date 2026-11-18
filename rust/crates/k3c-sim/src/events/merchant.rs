//! Händler-Überfall (Port von `engine/sim/events_merchant.go`, B-131, docs/rules/bosse.md § 2): Jeder `everyVisits`-te Besuch
//! des Händlers auf einer Insel (`Island::merchant_visits`, K3.2a) bringt bei seiner Ankunft eine zusätzliche Welle der Stufe
//! (Größe und Pool wie eine normale Welle, der Wellenzähler bleibt); deren und alle übrigen Gegner greifen den Händler zuerst
//! an. Lebt er bei der Abreise, kommen `rewardMaterial` seines Materials in den Vorrat (Lager-Maximum gilt); flieht er, gibt es
//! nichts.

use serde_json::json;

use super::emit;
use super::values::night;
use crate::enemies::waves::{plan_wave, queue_wave};
use crate::island::storage::add_stock_capped;
use crate::state::common::event;
use crate::state::pending::values;
use crate::state::{IslandRef, World};

/// Go: `merchantRaidStart`: nach der Ankunft (`merchant_dawn`); ohne Insel oder außerhalb des Rhythmus kein Überfall.
pub fn merchant_raid_start(w: &mut World, isl: Option<&IslandRef>) {
    let (Some(raid), Some(isl)) = (night(w, "merchantRaid"), isl) else {
        return;
    };
    // every_visits 0 = aus
    if w.merchant.is_none() || raid.every_visits <= 0 || isl.merchant_visits % raid.every_visits != 0 {
        return;
    }
    raid_merchant(w, isl);
}

/// Go: `raidMerchant`: Der Besuch wird überfallen, eine Welle der Stufe kommt dazu (auch Dev-Aktion event, `dev/combat.rs`).
pub fn raid_merchant(w: &mut World, isl: &IslandRef) {
    let (Some(raid), Some(m)) = (night(w, "merchantRaid"), w.merchant.as_mut()) else {
        return;
    };
    m.raid = true;
    let (size, hp, damage) = isl.wave_factors();
    let v = values(w);
    let night_phase = w.cycle.phase == "night";
    let plan = plan_wave(&v, &w.biome, w.wave.max(1), &mut w.rng, &w.portals, night_phase, size);
    queue_wave(w, &plan, hp, damage);
    let ev = event(json!({"event": raid.id, "day": w.cycle.day, "visit": isl.merchant_visits}));
    emit(w, "eventStarted", ev);
}

/// Go: `merchantRaidEnd`: bei Abreise (`protected`) oder Flucht des Händlers (`merchant_leaves`). `isl` begrenzt die
/// Belohnung auf das Lager-Maximum der Insel.
pub fn merchant_raid_end(w: &mut World, isl: Option<&IslandRef>, protected: bool) {
    let (Some(raid), Some(m)) = (night(w, "merchantRaid"), w.merchant.as_ref()) else {
        return;
    };
    if !m.raid {
        return;
    }
    let resource = m.resource.clone();
    let mut reward = 0;
    if protected {
        let v = values(w);
        reward = add_stock_capped(w, &v, isl, &resource, raid.reward_material).0;
    }
    let ev = event(json!({
        "event": raid.id, "day": w.cycle.day, "protected": protected, "resource": resource, "amount": reward
    }));
    emit(w, "eventEnded", ev);
}
