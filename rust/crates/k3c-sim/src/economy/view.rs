//! Wirtschaftsstand einer Stufe für das Protokoll (Port von `engine/sim/economy_view.go`, B-323, W9.1): Lager-Maximum,
//! Hub-Ausbau und Wartegrund „Gefahr“. Alles wird beim Lesen aus den bestehenden Regeln abgelesen (`capacity`,
//! `next_level`, `is_dangerous`), nichts gespeichert; die Welt-JSON und die Golden-Daten bleiben gleich.

use k3c_data::buildings::Cost;
use k3c_data::hub::Stock;
use serde::Serialize;

use super::links::hub_gate;
use super::site_payable;
use crate::building::barracks::{island_fighters, island_limit};
use crate::building::upgrades::armor_stage;
use crate::island::storage::{capacity, purse_max};
use crate::state::common::is_dangerous;
use crate::state::island_ref::{IslandRef, cheap};
use crate::state::links::hub_steps;
use crate::state::links::next_level;
use crate::state::omit::{is_false, is_zero_i64};
use crate::state::{Site, World, WorldValues};
use crate::unit::hammer::place_product;
use crate::world::links::{site_cost, upgrade_payable};

/// Go: `WaitingNextIsland` ist der Wartegrund „Material der nächsten Insel“ (B-592): Gold ist bezahlt, aber das
/// Material-Tor fehlt im Vorrat. Die Simulation wartet wie bei `waitingMaterial`; nur der Zustand nennt den eigenen Grund,
/// weil der Spieler dafür in die nächste Inselstufe muss statt hier abzubauen.
pub const WAITING_NEXT_ISLAND: &str = "waitingNextIsland";

/// Go: `Economy` ist der abgeleitete Wirtschaftsstand einer Stufe.
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Economy {
    /// Maximum je Rohstoff der Insel; fehlt ohne Insel (unbegrenzt, `island/storage.rs`).
    #[serde(rename = "stockMax", skip_serializing_if = "is_zero_i64")]
    pub stock_max: i64,
    /// Gold-Maximum je Spieler; fehlt ohne Dev-Bonus (dann gilt `economy.purse.maxGold` aus data/).
    #[serde(rename = "purseMax", skip_serializing_if = "is_zero_i64")]
    pub purse_max: i64,
    /// Hub-Stufe (1 bis Anzahl der `levels` in data/hub.json).
    #[serde(rename = "hubLevel")]
    pub hub_level: i64,
    /// Ausbauschritt, den `hub_upgrade` nennt (1 bis `steps`, B-592); fehlt mit `hub_upgrade`.
    #[serde(rename = "hubStep", skip_serializing_if = "is_zero_i64")]
    pub hub_step: i64,
    /// Ausbau auf die nächste Hub-Stufe; fehlt auf der letzten Stufe (keine Kosten).
    #[serde(rename = "hubUpgrade", skip_serializing_if = "Option::is_none")]
    pub hub_upgrade: Option<HubUpgrade>,
    /// Nacht oder Gegner da, Bau und Ausbau warten (`waitingWorker` heißt dann Gefahr, nicht „kein Bauer“).
    #[serde(rename = "danger", skip_serializing_if = "is_false")]
    pub danger: bool,
    /// Zahl der Kämpfer der Stufe (B-332, W10.1, W10.2); ohne `omitempty`, 0 Kämpfer ist ein Wert.
    #[serde(rename = "fighters")]
    pub fighters: i64,
    /// Truppen-Limit der Stufe (`building/barracks.rs`).
    #[serde(rename = "troopLimit")]
    pub troop_limit: i64,
}

/// Go: `HubUpgrade` sind Kosten und Stand des laufenden Ausbauschritts auf die nächste Hub-Stufe.
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct HubUpgrade {
    /// Goldkosten des Schritts.
    #[serde(rename = "gold")]
    pub gold: i64,
    /// Materialkosten des Schritts samt Material-Tor.
    #[serde(rename = "material")]
    pub material: Stock,
    /// Schon bezahltes Gold.
    #[serde(rename = "paid")]
    pub paid: i64,
    /// `waitingMaterial`, `waitingNextIsland`, `waitingWorker`; fehlt, solange Gold offen ist.
    #[serde(rename = "state", skip_serializing_if = "String::is_empty")]
    pub state: String,
    /// Zahl der Ausbauschritte auf die nächste Hub-Stufe (B-592).
    #[serde(rename = "steps")]
    pub steps: i64,
}

/// Go: `EconomyOf` liest den Wirtschaftsstand der Stufe `w` ab.
pub fn economy_of(w: &World, v: &WorldValues, isl: Option<&IslandRef>) -> Economy {
    let mut e = Economy {
        hub_level: w.hub_level,
        danger: is_dangerous(w),
        fighters: island_fighters(w),
        troop_limit: island_limit(w),
        ..Economy::default()
    };
    if let Some(limit) = capacity(w, v, isl) {
        e.stock_max = limit;
    }
    if isl.is_some_and(|i| i.stock_bonus > 0) {
        e.purse_max = purse_max(v, isl);
    }
    if let Some(h) = w.hub_site.as_ref()
        && let Some(next) = next_level(w, isl, h)
    {
        e.hub_step = w.hub_step.max(1);
        e.hub_upgrade = Some(HubUpgrade {
            gold: next.cost.gold,
            material: material_of(&next.cost),
            paid: h.upgrade_paid,
            state: hub_wait_state(w, isl, h, &next.cost),
            steps: hub_steps(w),
        });
    }
    e
}

/// Go: `hubWaitState` ist der Wartegrund des Hub-Ausbaus: `waitingNextIsland`, solange mit bezahltem Gold von einem Material
/// des Material-Tors nur der Tor-Anteil im Vorrat fehlt (`need` sind die Kosten des Schritts samt Tor; das Tor nutzt das
/// Material der Stufe selbst, B-631), sonst der Stand des Platzes.
pub fn hub_wait_state(w: &World, isl: Option<&IslandRef>, h: &Site, need: &Cost) -> String {
    if h.upgrade != "waitingMaterial" {
        return h.upgrade.clone();
    }
    let (gate, have) = (hub_gate(w, isl), &w.stock);
    // je Material: Tor, Vorrat, Bedarf
    let materials = [
        (gate.wood, have.wood, need.wood),
        (gate.stone, have.stone, need.stone),
        (gate.copper, have.copper, need.copper),
        (gate.iron, have.iron, need.iron),
        (gate.crystal, have.crystal, need.crystal),
    ];
    for (g, h_, n) in materials {
        if g > 0 && h_ < n && h_ >= n - g {
            return WAITING_NEXT_ISLAND.to_string();
        }
    }
    h.upgrade.clone()
}

/// Go: `materialOf` sind die Materialkosten ohne Gold.
pub fn material_of(c: &Cost) -> Stock {
    Stock {
        wood: c.wood,
        stone: c.stone,
        copper: c.copper,
        iron: c.iron,
        crystal: c.crystal,
    }
}

/// Go: `SiteGoldLeft` ist das Gold, das der Platz bis zum vollen Preis noch nimmt (Bau, Ausbau oder Produkt wie in
/// `pay_site`), und ob er gerade Münzen nimmt (`site_payable`). Nur gelesen, für die Bots des Balancing-Testers (BAL6).
pub fn site_gold_left(w: &World, v: &WorldValues, isl: Option<&IslandRef>, s: &Site) -> (i64, bool) {
    if !site_payable(w, v, isl, s) {
        return (0, false);
    }
    (gold_left(w, v, isl, s), true)
}

/// Der Teil von `site_gold_left` für einen zahlenden Platz: Bau, Ausbau oder Produkt am Platz.
fn gold_left(w: &World, v: &WorldValues, isl: Option<&IslandRef>, s: &Site) -> i64 {
    if s.state == "unpaid" {
        return site_cost(w, isl, s).gold - s.paid_gold;
    }
    if upgrade_payable(w, s) {
        return next_level(w, isl, s).map_or(0, |l| l.cost.gold) - s.upgrade_paid;
    }
    let price = match s.kind.as_str() {
        "workshop" | "bowyer" => cheap(isl, &place_product(v, s).cost).gold,
        "smithy" if v.native_rules => 0, // Schwert: eigenes Zahlziel (`sword_payable`)
        "armory" if v.native_rules => cheap(isl, &place_product(v, s).cost).gold,
        "smithy" => v.troops.get("eliteArcher").map_or(0, |t| cheap(isl, &t.cost).gold),
        "armory" => armor_stage(w, v, isl).map_or(0, |st| st.cost.gold),
        _ => 0,
    };
    price - s.bow_paid_gold
}
