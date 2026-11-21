//! Elite und Rüstung (Port von `engine/sim/building_upgrades.go`, docs/rules/buerger.md §§ 1–2, B-122, B-014; Q34, Q35,
//! Q37):
//!   - Schmiede: Am Platz zahlt man den Elite-Bogenschützen, am Angebot `eliteWarrior` (dx aus data/buildings.json) den
//!     Elite-Krieger; Gold und Material aus troops.json › cost. Nach `craft_seconds` wird der nächste Bogenschütze bzw.
//!     Krieger zum Gebäude Elite (HP, Schaden, Reichweite aus troops.json); ohne ihn wartet das fertige Upgrade.
//!   - Rüstkammer: Am Platz zahlt man die nächste Rüstungsstufe (buildings.json › armory.armor, ab ihrer Hub-Stufe). Nach
//!     `craft_seconds.armor` bekommen alle Kämpfer des Hubs sofort den Zuschlag auf die Basis-HP auf `max_hp` und `hp`, ohne
//!     Vollheilung; neue Kämpfer starten damit (`make_fighter`). Bauern und Landstreicher bleiben ohne.
//!
//! Gold am Platz liegt in `Site::bow_paid_gold`, die Arbeit in `Site::craft_done` (wie der Bogen der Werkstatt).
//! Abweichung vom Zeiger-Paar `paid`/`done` in Go: `Slot` wählt die Felder des Platzes `si` (Index in `w.sites`), weil
//! `craft_seconds` den Platz in `w.sites` nachschlägt und `w` darum unverändert zugänglich bleiben muss.

use k3c_data::buildings::{ArmorStage, Cost};

use crate::economy::{can_afford, nearest, spend};
use crate::state::island_ref::{IslandRef, cheap};
use crate::state::{Site, World, WorldValues};
use crate::unit::hammer::place_product;
use crate::unit::professions::{OfferTarget, bows_crafting, craft_seconds, find_offer, pay_offer};
use crate::unit::warrior::is_fighter;
use crate::world::values_room::values;

/// Go: `placePayable`: Nimmt der gebaute Platz `s` Münzen für sein Produkt am Platz (Bogen, Elite-Bogenschütze, Rüstung)?
pub fn place_payable(w: &World, v: &WorldValues, isl: Option<&IslandRef>, s: &Site) -> bool {
    match s.kind.as_str() {
        "workshop" | "bowyer" => {
            let p = place_product(v, s);
            p.product.have(s) + bows_crafting(s) < p.rack && s.bow_paid_gold < cheap(isl, &p.cost).gold
        }
        "smithy" if v.native_rules => false, // FG1.2: das Schwert hat ein eigenes Zahlziel
        "armory" if v.native_rules => {
            let p = place_product(v, s);
            p.product.have(s) + bows_crafting(s) < p.rack && s.bow_paid_gold < cheap(isl, &p.cost).gold
        }
        "smithy" => s.bow_paid_gold < cheap(isl, &troop_cost(v, "eliteArcher")).gold,
        "armory" => armor_stage(w, v, isl)
            .is_some_and(|st| s.craft_done == 0.0 && w.hub_level >= st.hub_level && s.bow_paid_gold < st.cost.gold),
        _ => false,
    }
}

/// Go: `w.values().Troops[kind].Cost` (fehlt die Figur: nichts).
fn troop_cost(v: &WorldValues, kind: &str) -> Cost {
    v.troops.get(kind).map(|t| t.cost.clone()).unwrap_or_default()
}

/// Go: `offerAt`: das Berufs-Angebot (`find_offer`), sonst ein bezahlbares Elite-Angebot in Zahl-Reichweite von `x`.
pub fn offer_at<'a>(w: &World, v: &'a WorldValues, isl: Option<&IslandRef>, x: f64) -> Option<OfferTarget<'a>> {
    find_offer(w, v, isl, x).or_else(|| elite_offer_at(w, v, isl, x, true))
}

/// FG1.2: Elite entsteht mit nativen Regeln nur durch Rüstung (`armor_convert`), nicht mehr durch ein Elite-Angebot.
fn no_elite_offers(v: &WorldValues) -> bool {
    v.native_rules
}

/// Go: `eliteOfferAt` ist das Elite-Angebot eines gebauten Gebäudes in Zahl-Reichweite von `x`; mit `payable` nur, solange
/// sein Preis nicht voll bezahlt ist.
pub fn elite_offer_at<'a>(
    w: &World,
    v: &'a WorldValues,
    isl: Option<&IslandRef>,
    x: f64,
    payable: bool,
) -> Option<OfferTarget<'a>> {
    if no_elite_offers(v) {
        return None;
    }
    for (si, s) in w.sites.iter().enumerate() {
        let offers = v.buildings.get(&s.kind).map_or(&[][..], |b| b.offers.as_slice());
        for (idx, of) in offers.iter().enumerate() {
            let o = OfferTarget { site: si, idx, offers };
            let near = is_fighter(&v.troops, &of.kind)
                && s.state == "built"
                && (o.x(w) - x).abs() <= v.economy.pay_range_units;
            if near && (!payable || o.paid(w) < cheap(isl, &troop_cost(v, &of.kind)).gold) {
                return Some(o);
            }
        }
    }
    None
}

/// Go: `payAnyOffer` zahlt eine Münze an ein Angebot. `pay_offer` schließt die Zahlung an einem Elite-Angebot nach jeder
/// Münze ab (es ist kein Beruf); bis der Preis voll ist, bleibt sie offen, damit Loslassen erstattet.
pub fn pay_any_offer(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, pi: usize, o: &OfferTarget) {
    let (key, amount) = (w.players[pi].pay_key.clone(), w.players[pi].pay_amount);
    pay_offer(w, v, isl, pi, o);
    if is_fighter(&v.troops, o.kind()) && o.paid(w) < cheap(isl, &troop_cost(v, o.kind())).gold {
        w.players[pi].pay_key = key;
        w.players[pi].pay_amount = amount;
    }
}

/// Die Felder des Platzes, in denen eine Herstellung Gold und Arbeit führt (Go: das Zeiger-Paar `paid`, `done`).
#[derive(Debug, Clone, Copy)]
enum Slot {
    /// `bow_paid_gold` und `craft_done` (Produkt am Platz, Elite-Bogenschütze, Rüstung).
    Main,
    /// `offer_paid[i]` und `offer_done` (Elite-Krieger der Schmiede).
    Offer(usize),
}

impl Slot {
    fn get(self, s: &Site) -> (i64, f64) {
        match self {
            Self::Main => (s.bow_paid_gold, s.craft_done),
            Self::Offer(i) => (s.offer_paid.get(i).copied().unwrap_or(0), s.offer_done),
        }
    }

    fn set(self, s: &mut Site, paid: i64, done: f64) {
        match self {
            Self::Main => {
                s.bow_paid_gold = paid;
                s.craft_done = done;
            }
            Self::Offer(i) => {
                if let Some(p) = s.offer_paid.get_mut(i) {
                    *p = paid;
                }
                s.offer_done = done;
            }
        }
    }
}

/// Go: `stepCrafts`: Elite-Upgrades der Schmiede und Rüstungsstufen der Rüstkammer herstellen und anwenden.
pub fn step_crafts(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>) {
    for si in 0..w.sites.len() {
        if w.sites[si].state != "built" {
            continue;
        }
        match w.sites[si].kind.as_str() {
            "smithy" if v.native_rules => {} // FG1.2: der Waffenschmied macht Schwerter (`step_sword_craft`)
            "armory" if v.native_rules => crate::unit::armor::assign(w, si), // EQ1.1: Kämpfer holen die Rüstung ab
            "smithy" => {
                step_elite(w, v, isl, si, Slot::Main, "eliteArcher");
                let offers = v.buildings.get("smithy").map_or(&[][..], |b| b.offers.as_slice());
                for (i, o) in offers.iter().enumerate() {
                    if is_fighter(&v.troops, &o.kind) && i < w.sites[si].offer_paid.len() {
                        step_elite(w, v, isl, si, Slot::Offer(i), &o.kind);
                    }
                }
            }
            "armory" => step_armor(w, v, isl, si),
            _ => {}
        }
    }
}

/// Go: `craft`: Voll bezahltes Gold geht mit dem Material aus dem Vorrat in Arbeit, wenn gerade nichts entsteht; `true`,
/// sobald das Stück fertig ist (`done` bleibt gesetzt, bis der Aufrufer es anwendet und auf 0 setzt).
fn craft(w: &mut World, v: &WorldValues, si: usize, slot: Slot, product: &str, cost: &Cost) -> bool {
    let (paid, done) = slot.get(&w.sites[si]);
    if done == 0.0 && paid >= cost.gold && can_afford(&w.stock, cost) {
        spend(&mut w.stock, cost);
        let ready = w.time + craft_seconds(w, v, si, product);
        slot.set(&mut w.sites[si], 0, ready);
    }
    let done = slot.get(&w.sites[si]).1;
    done > 0.0 && w.time >= done
}

/// Go: `stepElite` wertet nach der Herstellung den nächsten Kämpfer der Ausgangsfigur (troops.json › upgradeFrom) zum
/// Platz zur Figur `kind` auf: volle HP der neuen Figur mit Rüstung, Schaden und Reichweite liest sein Verhalten aus `kind`.
fn step_elite(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, si: usize, slot: Slot, kind: &str) {
    let cost = cheap(isl, &troop_cost(v, kind));
    if !craft(w, v, si, slot, kind, &cost) {
        return;
    }
    let from = v.troops.get(kind).map_or("", |t| t.upgrade_from.as_str());
    let sx = w.sites[si].x;
    let Some(ti) = nearest(&w.troops, |t| t.x, sx, f64::INFINITY, |t| t.kind == from) else {
        return;
    };
    let hp = armored_hp(w, kind);
    let t = &mut w.troops[ti];
    t.kind = kind.to_string();
    t.hp = hp;
    t.max_hp = hp;
    let (paid, _) = slot.get(&w.sites[si]);
    slot.set(&mut w.sites[si], paid, 0.0);
}

/// Go: `stepArmor`: Nach der Herstellung steigt die Rüstungsstufe des Hubs; alle Kämpfer bekommen den Zuschlag auf
/// `max_hp` und `hp` (keine Vollheilung).
fn step_armor(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, si: usize) {
    let Some(stage) = armor_stage(w, v, isl) else {
        return;
    };
    if !craft(w, v, si, Slot::Main, "armor", &stage.cost) {
        return;
    }
    w.sites[si].craft_done = 0.0;
    let before = armor_bonus(w, v);
    w.armor_level += 1;
    let after = armor_bonus(w, v);
    for t in &mut w.troops {
        if is_fighter(&v.troops, &t.kind) {
            let gain = v.troops.get(t.data_kind()).map_or(0.0, |d| d.hp) * (after - before);
            t.max_hp += gain;
            t.hp += gain;
        }
    }
}

/// Go: `armorStage` ist die nächste Rüstungsstufe; `None`, wenn alle erreicht sind.
pub fn armor_stage(w: &World, v: &WorldValues, isl: Option<&IslandRef>) -> Option<ArmorStage> {
    if v.native_rules {
        return None; // FG1.2: keine Rüstungsstufen und kein Hub-HP-Bonus mehr
    }
    let stages = &v.buildings.get("armory")?.armor;
    let mut stage = stages.get(usize::try_from(w.armor_level).ok()?)?.clone();
    stage.cost = cheap(isl, &stage.cost);
    Some(stage)
}

/// Go: `armorBonus` ist der Zuschlag der erreichten Rüstungsstufe als Anteil der Basis-HP (0 ohne Rüstung).
pub fn armor_bonus(w: &World, v: &WorldValues) -> f64 {
    if w.armor_level == 0 || v.native_rules {
        return 0.0;
    }
    let idx = usize::try_from(w.armor_level - 1).unwrap_or(0);
    v.buildings
        .get("armory")
        .and_then(|b| b.armor.get(idx))
        .map_or(0.0, |s| s.hp_bonus)
}

/// Go: `armoredHP` sind die vollen HP der Kämpfer-Figur `kind` mit der Rüstung des Hubs.
pub fn armored_hp(w: &World, kind: &str) -> f64 {
    let v = values(w);
    v.troops.get(kind).map_or(0.0, |t| t.hp) * (1.0 + armor_bonus(w, &v))
}
