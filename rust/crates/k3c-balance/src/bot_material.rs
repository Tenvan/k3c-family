//! Material des Wirtschafts-Bots (Port des zweiten Teils von `engine/balance/bot_wirtschaft.go`): Knoten markieren, den
//! Vorrat gegen Kosten prüfen und das Schiff des Hafens bezahlen (BS9, B-637).

use k3c_data::buildings::Cost;
use k3c_data::hub::Stock;
use k3c_sim::island::Island;
use k3c_sim::state::{Player, ResourceNode, World};
use k3c_sim::world::ship::ship_next_cost;

use crate::bot_clear::near_lair;
use crate::bot_wirtschaft::{FIGHTER_WANT, waiting_material};
use crate::bots::{closer, economy_view, gold_left};
use crate::prices::prices;

/// Der nächste unmarkierte Knoten eines Rohstoffs, der für `need` fehlt (Vorrat plus markierte Knoten reichen nicht), mit
/// bezahlbarem Rest und ohne zahlbaren Platz in Reichweite (der ginge beim Zahlen vor). B-654: Ein Knoten, der nicht näher
/// als der beste liegt, kostet keinen Preis-Zugriff; die teuren Prüfungen (zahlbare Plätze, Bau des Endbosses) laufen nur
/// für Kandidaten, die zahlbaren Plätze höchstens einmal.
pub fn node_for<'a>(isl: &Island, w: &'a World, p: &Player, need: &Cost) -> Option<&'a ResourceNode> {
    let prices = prices();
    let mut pending = Cost::default(); // markierte Knoten je Rohstoff
    for n in w.nodes.iter().filter(|n| n.marked) {
        if let Some(np) = prices.nodes.get(&n.kind) {
            add_res(&mut pending, &np.resource, np.amount);
        }
    }
    let mut best: Option<&ResourceNode> = None;
    let mut blocked: Option<Vec<f64>> = None;
    for n in &w.nodes {
        if n.marked || best.is_some_and(|b| !closer(p.x, n.x, b.x)) {
            continue;
        }
        let Some(np) = prices.nodes.get(&n.kind) else {
            continue;
        };
        if p.gold < np.mark_cost - n.paid_gold
            || amount_of(&isl.stock, &np.resource) + cost_of(&pending, &np.resource) >= cost_of(need, &np.resource)
        {
            continue;
        }
        let xs = blocked.get_or_insert_with(|| payable_sites_x(w));
        if !near_any(n.x, xs) && !near_lair(w, n.x) {
            best = Some(n);
        }
    }
    best
}

/// Zählt `amount` zum Rohstoff `r` (Name wie in economy.json) in `c`.
pub fn add_res(c: &mut Cost, r: &str, amount: i64) {
    match r {
        "wood" => c.wood += amount,
        "stone" => c.stone += amount,
        "copper" => c.copper += amount,
        "iron" => c.iron += amount,
        "crystal" => c.crystal += amount,
        _ => {}
    }
}

/// Orte aller Plätze, die gerade Münzen nehmen.
fn payable_sites_x(w: &World) -> Vec<f64> {
    w.sites.iter().filter(|s| gold_left(w, s).1).map(|s| s.x).collect()
}

/// `x` liegt in Zahl-Reichweite eines der Orte.
fn near_any(x: f64, xs: &[f64]) -> bool {
    let range = prices().pay_range;
    xs.iter().any(|o| (o - x).abs() <= range)
}

/// Der Vorrat deckt das Material der Kosten (Gold zählt nicht).
pub fn covers(have: &Stock, c: &Cost) -> bool {
    have.wood >= c.wood
        && have.stone >= c.stone
        && have.copper >= c.copper
        && have.iron >= c.iron
        && have.crystal >= c.crystal
}

/// Die Menge des Rohstoffs `r` (Name wie in economy.json) im Vorrat.
pub fn amount_of(s: &Stock, r: &str) -> i64 {
    match r {
        "wood" => s.wood,
        "stone" => s.stone,
        "copper" => s.copper,
        "iron" => s.iron,
        "crystal" => s.crystal,
        _ => 0,
    }
}

/// Die Menge des Rohstoffs `r` in den Kosten.
pub fn cost_of(c: &Cost, r: &str) -> i64 {
    match r {
        "wood" => c.wood,
        "stone" => c.stone,
        "copper" => c.copper,
        "iron" => c.iron,
        "crystal" => c.crystal,
        _ => 0,
    }
}

/// Der Schiffs-Platz des Hafens, wenn das Gold für den nächsten Abschnitt reicht und das Material des Abschnitts im Vorrat
/// liegt (BS9, B-637); `None` sonst. Das Material holen die Bots unter Tage und für den Hub-Ausbau, der Halter am Hub zahlt,
/// sobald es liegt.
pub fn ship_x(isl: &Island, w: &World, p: &Player, reserve: i64) -> Option<f64> {
    if !ship_ready(w) || waiting_material(w) {
        return None;
    }
    let iref = w.island_ref_view();
    for s in &w.sites {
        let Some(cost) = ship_next_cost(w, iref.as_ref(), s) else {
            continue;
        };
        if !covers(&isl.stock, &cost) {
            continue;
        }
        let (left, ok) = gold_left(w, s);
        if ok && left > 0 && p.gold >= left + reserve {
            return Some(s.x);
        }
    }
    None
}

/// Erst Heer und Hub, das Schiff ist der Schluss der Insel.
pub fn ship_ready(w: &World) -> bool {
    w.hub_level >= 2 && economy_view(w).fighters >= FIGHTER_WANT
}

/// Der nächste Knoten, der Material für den nächsten Schiffsabschnitt liefert, solange der Vorrat ihn nicht deckt; `None`,
/// wenn kein Schiff in dieser Stufe wartet oder nichts zu holen ist.
pub fn ship_node<'a>(isl: &Island, w: &'a World, p: &Player) -> Option<&'a ResourceNode> {
    if !ship_ready(w) {
        return None;
    }
    let iref = w.island_ref_view();
    for s in &w.sites {
        if let Some(cost) = ship_next_cost(w, iref.as_ref(), s)
            && !covers(&isl.stock, &cost)
        {
            return node_for(isl, w, p, &cost);
        }
    }
    None
}

/// Zahlen, sobald der Abschnitt bezahlbar ist, sonst (ohne Rücklage) Material dafür sammeln (BS9).
pub fn ship_step(isl: &Island, w: &World, p: &Player, reserve: i64) -> Option<f64> {
    if let Some(x) = ship_x(isl, w, p, reserve) {
        return Some(x);
    }
    if reserve == 0 {
        return ship_node(isl, w, p).map(|n| n.x);
    }
    None
}
