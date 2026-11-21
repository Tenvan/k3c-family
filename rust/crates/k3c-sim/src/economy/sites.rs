//! Gold und Material, Bauplätze (Port von `engine/sim/economy.go`: `giveGold`, `scatterCoins`, `addStock`, `stepSites`).

use super::links::step_upgrade;
use super::{can_afford, spend};
use crate::building::upgrades::step_crafts;
use crate::island::storage::add_stock_capped;
use crate::state::island_ref::IslandRef;
use crate::state::purse::purse_max_of;
use crate::state::{Coin, World, WorldValues};
use crate::unit::professions::{step_bow_craft, step_offers};
use crate::unit::warrior::step_sword_craft;
use crate::world::links::site_cost;
use crate::world::vals;

/// Go: `giveGold` füllt den Beutel des Spielers `pi`, was nicht mehr passt, fällt bei `x` als Münzen auf den Boden.
pub fn give_gold(w: &mut World, isl: Option<&IslandRef>, pi: usize, amount: i64, x: f64) {
    // nach Respec kann der Beutel über der Grenze liegen: nichts abziehen
    let fits = amount
        .min(purse_max_of(w, &w.players[pi], isl) - w.players[pi].gold)
        .max(0);
    w.players[pi].gold += fits;
    scatter_coins(w, x, amount - fits);
}

/// Go: `scatterCoins`: `count` Münzen um `x` auf den Boden.
pub fn scatter_coins(w: &mut World, x: f64, count: i64) {
    for _ in 0..count {
        let cx = (x + (w.rng.next() - 0.5) * 3.0).max(0.0).min(w.width_units);
        let id = w.new_id();
        w.coins.push(Coin {
            id,
            x: cx,
            ..Coin::default()
        });
    }
}

/// Go: `addStock` legt Baumaterial in den Vorrat (in einer Insel höchstens bis zur Kapazität); `false`, wenn `resource`
/// kein Baumaterial ist.
pub fn add_stock(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, resource: &str, amount: i64) -> bool {
    add_stock_capped(w, v, isl, resource, amount).1
}

/// Go: `stepSites`: Bezahlte Bauplätze ziehen das Material aus dem Hub-Vorrat, sobald genug da ist. Werkstatt fertigt Bögen
/// (mit Herstellungszeit), voll bezahlte Angebote bilden Bauern aus (`unit/professions.rs`), Schmiede und Rüstkammer werten
/// auf (`building/upgrades.rs`).
pub fn step_sites(w: &mut World, isl: Option<&mut IslandRef>) {
    let v = vals(w);
    let isl: Option<&IslandRef> = isl.map(|i| &*i);
    for si in 0..w.sites.len() {
        if w.sites[si].state == "waitingMaterial" {
            let cost = site_cost(w, isl, &w.sites[si]);
            if can_afford(&w.stock, &cost) {
                spend(&mut w.stock, &cost);
                w.sites[si].state = "waitingWorker".into();
            }
        }
        let (kind, built) = (w.sites[si].kind.clone(), w.sites[si].state == "built");
        if (kind == "workshop" || kind == "bowyer") && built {
            step_bow_craft(w, &v, isl, si);
            step_sword_craft(w, &v, isl, si);
        }
        if v.native_rules && built {
            match kind.as_str() {
                "smithy" => step_sword_craft(w, &v, isl, si), // FG1.2: Waffenschmied
                "armory" => step_bow_craft(w, &v, isl, si),   // FG1.2: Rüstungsschmied (Produkt Rüstung)
                _ => {}
            }
        }
    }
    // Go: `eachSite`: alle Plätze der Stufe, danach der Zahlplatz des Hubs
    for si in 0..w.sites.len() {
        let id = w.sites[si].id;
        step_upgrade(w, isl, id);
    }
    if let Some(id) = w.hub_site.as_ref().map(|h| h.id) {
        step_upgrade(w, isl, id);
    }
    step_offers(w, &v, isl);
    step_crafts(w, &v, isl);
}
