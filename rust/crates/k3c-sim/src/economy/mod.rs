//! Monarchen, Münzen und alles, was man bezahlt (Port von `engine/sim/economy.go`, Teil Zahlziel). A halten gibt im Takt
//! Münzen an das nächste bezahlbare Ziel in Reichweite. Ohne Ziel fällt die Münze auf den Boden (so gibt man anderen Gold).
//!
//! Aufteilung: `mod.rs` Zahlziel und Schlüssel, `players.rs` Spielerschritt, Münzen und Truhen, `pay.rs` Zahlen und
//! Erstatten, `sites.rs` Bauplätze und Gold, `dawn.rs` Morgengold, `fountain.rs` Fontäne, `view.rs` Wirtschaftsansicht.

pub mod dawn;
pub mod fountain;
pub mod links;
pub mod pay;
pub mod players;
pub mod sites;
pub mod util;
pub mod view;

#[cfg(test)]
mod coin_scale_tests;
#[cfg(test)]
mod gold_tests;
#[cfg(test)]
mod pay_tests;
#[cfg(test)]
mod players_tests;
#[cfg(test)]
mod view_tests;

pub use dawn::{dawn_gold, pay_dawn_income};
pub use fountain::{fountain_coins, scaled_gold};
pub use pay::{pay_one_coin, pay_site, pay_to_target, refund_pending, refund_site};
pub use players::{collect_coins, collect_pickups, move_player, step_players, to_hub};
pub use sites::{add_stock, give_gold, scatter_coins, step_sites};
pub use util::{can_afford, clear_pending, nearest, spend};
pub use view::{Economy, HubUpgrade, WAITING_NEXT_ISLAND, economy_of, site_gold_left};

use crate::building::ruin::line_before_ruin;
use crate::building::upgrades::{offer_at, place_payable};
use crate::combat::spots::{CombatSpot, SpotKind, combat_target};
use crate::state::island_ref::IslandRef;
use crate::state::links::site_hub_level;
use crate::state::node_mark::node_markable;
use crate::state::{Player, Site, World, WorldValues};
use crate::unit::professions::OfferTarget;
use crate::unit::warrior::sword_at;
use crate::world::lines::line_open;
use crate::world::links::upgrade_payable;
use links::merchant_at;

/// Go: `payTarget`, das Zahlziel am Ort eines Spielers. Go hält Zeiger, Rust die ID (Platz, Landstreicher, Ressource) oder
/// den Index (`Offer`, `Sword`: Platz in `w.sites`); genau eines gilt.
#[derive(Debug, Clone, PartialEq)]
pub enum PayTarget {
    /// Bauplatz (auch der Zahlplatz des Hub-Ausbaus), per ID.
    Site(i64),
    /// Landstreicher, per ID.
    Troop(i64),
    /// Ressource, per ID.
    Node(i64),
    /// Angebot (Beruf, Elite) Nummer `idx` am Platz `site` (Index in `w.sites`).
    Offer { site: usize, idx: usize },
    /// Händler: "buy" oder "sell".
    Merchant(String),
    /// Schwert-Zahlziel der Werkstatt, Platz `site` (Index in `w.sites`).
    Sword(usize),
    /// Schild, Auslöser oder Banner der Kampfvariante samt Zahl-Schlüssel.
    Combat { spot: CombatSpot, key: String },
}

/// Das Angebot `idx` am Platz `site` (Index in `w.sites`) aus dem Wertesatz; `None`, wenn es nicht (mehr) existiert.
pub fn offer_target<'a>(w: &World, v: &'a WorldValues, site: usize, idx: usize) -> Option<OfferTarget<'a>> {
    let s = w.sites.get(site)?;
    let offers = v.buildings.get(&s.kind).map_or(&[][..], |b| b.offers.as_slice());
    (idx < offers.len()).then_some(OfferTarget { site, idx, offers })
}

/// Go: `findPayTarget`: Was würde eine Münze gerade bezahlen? Bauplatz vor Landstreicher vor Ressource.
pub fn find_pay_target(w: &World, v: &WorldValues, isl: Option<&IslandRef>, p: &Player) -> Option<PayTarget> {
    let r = v.economy.pay_range_units;
    let combat = combat_target(w, v, p);
    if let Some(c) = combat.filter(|c| c.kind != SpotKind::Shield) {
        // Auslöser und Banner gehen Bauplätzen in Reichweite vor
        return Some(PayTarget::Combat { key: c.key(p), spot: c });
    }
    let near = nearest(&w.sites, |s| s.x, p.x, r, |s| site_payable(w, v, isl, s));
    let near = line_before_ruin(w, v, isl, near, p.x); // Linien-Platz vor Ruine (building/ruin.rs)
    let hub = w.hub_site.as_ref().filter(|h| {
        (h.x - p.x).abs() <= r
            && site_payable(w, v, isl, h)
            && near.is_none_or(|si| (h.x - p.x).abs() < (w.sites[si].x - p.x).abs())
    });
    if let Some(h) = hub {
        return Some(PayTarget::Site(h.id));
    }
    if let Some(si) = near {
        return Some(PayTarget::Site(w.sites[si].id));
    }
    find_other_target(w, v, isl, p, combat, r)
}

/// Der Rest von `find_pay_target`, wenn kein Bauplatz in Reichweite zahlt: Angebot, Schwert, Kampf-Schild, Händler,
/// Landstreicher, Ressource.
fn find_other_target(
    w: &World,
    v: &WorldValues,
    isl: Option<&IslandRef>,
    p: &Player,
    combat: Option<CombatSpot>,
    r: f64,
) -> Option<PayTarget> {
    if let Some(o) = offer_at(w, v, isl, p.x) {
        return Some(PayTarget::Offer {
            site: o.site,
            idx: o.idx,
        });
    }
    if let Some(si) = sword_at(w, v, isl, p.x, true) {
        return Some(PayTarget::Sword(si));
    }
    if let Some(c) = combat {
        return Some(PayTarget::Combat { key: c.key(p), spot: c });
    }
    if let Some(side) = merchant_at(w, v, p.x) {
        return Some(PayTarget::Merchant(side.to_string()));
    }
    if let Some(i) = nearest(&w.troops, |t| t.x, p.x, r, |t| t.is_wild()) {
        return Some(PayTarget::Troop(w.troops[i].id));
    }
    nearest(&w.nodes, |n| n.x, p.x, r, |n| node_markable(w, n)).map(|i| PayTarget::Node(w.nodes[i].id))
}

/// Go: `sitePayable`: Nimmt der Platz gerade Münzen? Unbezahlte Plätze erst ab ihrer Hub-Stufe, Linien-Plätze nach der
/// Linien-Regel (`world/lines.rs`); gebaute für den Ausbau auf die nächste Stufe (RU18.3) oder ihr Produkt
/// (`building/upgrades.rs`).
pub fn site_payable(w: &World, v: &WorldValues, isl: Option<&IslandRef>, s: &Site) -> bool {
    if s.state == "unpaid" {
        // RU62.10: der Hub-Platz des Einsiedler-Bauwerks nimmt keine Münzen, nur die Hütte bezahlt ihn
        return w.hub_level >= site_hub_level(w, s) && line_open(w, s) && !crate::hermit::bonus::is_hall(w, s);
    }
    if upgrade_payable(w, s) {
        return true;
    }
    s.state == "built" && place_payable(w, v, isl, s)
}

/// Go: `keyOf`: der Schlüssel eines Ziels ("site:3"), leer für kein Ziel.
pub fn key_of(w: &World, t: Option<&PayTarget>) -> String {
    match t {
        None => String::new(),
        Some(PayTarget::Offer { site, idx }) => format!("offer:{}.{}", w.sites[*site].id, idx),
        Some(PayTarget::Merchant(side)) => format!("merchant:{side}"),
        Some(PayTarget::Sword(si)) => format!("sword:{}", w.sites[*si].id),
        Some(PayTarget::Combat { key, .. }) => format!("combat:{key}"),
        Some(PayTarget::Site(id)) => format!("site:{id}"),
        Some(PayTarget::Troop(id)) => format!("vagrant:{id}"),
        Some(PayTarget::Node(id)) => format!("node:{id}"),
    }
}
