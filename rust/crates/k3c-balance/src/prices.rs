//! Die Preise aus `data/`, die ein Bot zum Entscheiden braucht (Port von `priceList` und `loadPrices` aus
//! `engine/balance/bots.go`).

use std::collections::BTreeMap;
use std::sync::OnceLock;

use k3c_data::Source;
use k3c_data::buildings::Cost;
use k3c_data::economy::Gatherable;
use k3c_data::{buildings, economy, hub, troops};

use crate::fehler::Fehler;
use crate::vary::current_data;

/// Rohstoff, Menge je Ernte und Markierungs-Preis eines Knotens (Go: `nodePrice`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodePrice {
    pub resource: String,
    pub amount: i64,
    pub mark_cost: i64,
}

impl From<&Gatherable> for NodePrice {
    fn from(g: &Gatherable) -> Self {
        Self {
            resource: g.resource.clone(),
            amount: g.amount,
            mark_cost: g.mark_cost,
        }
    }
}

/// Die Werte aus `data/`, die ein Bot zum Entscheiden braucht (Go: `priceList`; nur gelesen).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PriceList {
    pub wall_gold: i64,
    pub recruit_gold: i64,
    /// Gold je Mauer-Stufe (Eintrag n-1 = Stufe n, buildings.json › wall.levels).
    pub wall_level_gold: Vec<i64>,
    pub pay_range: f64,
    /// Rohstoff-Knoten nach Art (economy.json › gatherables, veins).
    pub nodes: BTreeMap<String, NodePrice>,
    /// Baukosten nach Art (buildings.json › cost).
    pub cost: BTreeMap<String, Cost>,
    /// Gold je Kauf beim Händler (economy.json › merchant.gold).
    pub merchant_gold: i64,
    /// Kaufen-Zahlziel relativ zur Hub-Mitte (hub.json › merchant.buyOffsetUnits).
    pub merchant_buy_offset: f64,
}

/// Liest die Preise aus einer Datenquelle (Go: `loadPrices`).
pub fn load_prices(src: &Source) -> Result<PriceList, Fehler> {
    let read = |name: &str| src.read(name).map_err(|e| Fehler::Daten(format!("balance: {e}")));
    let wrap = |name: &str, e: k3c_data::Fehler| Fehler::Daten(format!("balance: data/{name}: {e}"));
    let b = buildings::load(&read(buildings::FILE)?).map_err(|e| wrap(buildings::FILE, e))?;
    let t = troops::load(&read(troops::FILE)?).map_err(|e| wrap(troops::FILE, e))?;
    let e = economy::load(&read(economy::FILE)?).map_err(|e| wrap(economy::FILE, e))?;
    let h = hub::load(&read(hub::FILE)?).map_err(|e| wrap(hub::FILE, e))?;
    let wall = b.get("wall").cloned().unwrap_or_default();
    let mut nodes: BTreeMap<String, NodePrice> = e.gatherables.iter().map(|(k, g)| (k.clone(), g.into())).collect();
    nodes.extend(e.veins.iter().map(|(k, v)| (k.clone(), (&v.gatherable).into())));
    Ok(PriceList {
        wall_gold: wall.cost.gold,
        recruit_gold: t.get("vagrant").map_or(0, |v| v.recruit_cost.gold).max(1),
        wall_level_gold: wall.levels.iter().map(|l| l.cost.gold).collect(),
        pay_range: e.pay_range_units,
        nodes,
        cost: b.iter().map(|(k, v)| (k.clone(), v.cost.clone())).collect(),
        merchant_gold: e.merchant.gold.max(1),
        merchant_buy_offset: h.merchant.buy_offset_units,
    })
}

static PRICES: OnceLock<PriceList> = OnceLock::new();

/// Die Preise aus `data/` (Go: Paket-Variable `prices`), einmal gelesen. Fehlen die Dateien, bricht Go beim Start ab;
/// Rust rechnet mit leeren Preisen weiter, damit der Produktivcode nicht abstürzt.
pub fn prices() -> &'static PriceList {
    if let Some(ctx) = current_data() {
        return ctx.prices; // innerhalb von `vary::with_data`: die Preise der Variante (Go: `withData` setzt `prices`)
    }
    PRICES.get_or_init(|| load_prices(&Source::from_env()).unwrap_or_default())
}
