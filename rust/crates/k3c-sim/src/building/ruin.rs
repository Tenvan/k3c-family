//! Ruinen als Plätze (Port von `engine/sim/building_ruin.go`, docs/rules/geplante-chunks.md § 8, B-468): Jede Ruine der
//! Planung (`Layout.ruins`) ist ein zusätzlicher Platz der Stufe, gebaut mit dem HP-Anteil ihrer Art im Wertesatz der Welt
//! (`ruin_hp`), und wirkt wie ein gebautes Gebäude dieser Art. Sie ist Hub-neutral (Hub-Stufe und Mauerlinien zählen sie
//! nicht; für „alles gebaut“ zählt sie wie jeder Platz), wird wie jedes Gebäude repariert und zerstört, ist danach erst ab
//! der Hub-Stufe ihrer Art bezahlbar und kehrt beim Burgfall (`castle_losses`) in den Ausgangszustand zurück. Die Herkunft
//! kommt immer aus dem Level (kind@x wie `site_line`), damit `empty_site` und `destroy_site` die Kennung `Site::ruin` neu
//! setzen; der Spielstand ordnet den Platz über kind@x zu.

use super::pending::site_payable;
use crate::state::island_ref::IslandRef;
use crate::state::{Site, World, WorldValues};
use crate::unit::econ::nearest;
use crate::world::base::base;
use crate::world::lines::level_site;
use crate::world::{empty_site, hub_of, vals};

/// Go: `isRuin`: Ist `kind`@`x` eine Ruine der Planung dieser Stufe?
pub fn is_ruin(w: &World, kind: &str, x: f64) -> bool {
    w.level.ruins.iter().any(|r| r.kind == kind && r.x == x)
}

/// Go: `ruinSite` ist der Ausgangszustand eines Ruinen-Platzes: gebaut, Stufe 1, Start-HP der Ruine. Zieht wie
/// `empty_site` eine ID.
pub fn ruin_site(w: &mut World, kind: &str, x: f64) -> Site {
    let mut s = empty_site(w, kind, x);
    let hp = vals(w).buildings.get(kind).map_or(0.0, |b| b.hp);
    s.state = "built".into();
    s.build_progress = 1.0;
    // Ohne lesbare eingebettete Daten (nicht zu erwarten) gelten die vollen HP.
    s.hp = base().map_or(hp, |b| k3c_level::ruins::ruin_hp(&b.rules, hp));
    s
}

/// Go: `addRuinSites` legt die Ruinen der Stufe als Plätze an; ohne Planung gibt es keine (Goldens bleiben gleich).
pub fn add_ruin_sites(w: &mut World) {
    for i in 0..w.level.ruins.len() {
        let (kind, x) = (w.level.ruins[i].kind.clone(), w.level.ruins[i].x);
        let s = ruin_site(w, &kind, x);
        w.sites.push(s);
    }
}

/// Go: `kindHubLevel` ist die Hub-Stufe der Art `kind`: die kleinste der Hub-Plätze dieser Art in hub.json (`sites`,
/// `island_sites`); 1, wenn die Art dort keinen Platz hat.
pub fn kind_hub_level(w: &World, kind: &str) -> i64 {
    let h = hub_of(w);
    let mut best = 0;
    for s in h.sites.iter().chain(h.island_sites.iter()) {
        if s.kind == kind && (best == 0 || s.hub_level < best) {
            best = s.hub_level;
        }
    }
    best.max(1)
}

/// Go: `lineBeforeRuin`: Ist das Zahlziel `s` (Index in `w.sites`) eine Ruine und steht ein bezahlbarer, unbezahlter
/// Bauplatz der Mauerlinien in Zahl-Reichweite von `x`, geht die Münze an ihn (der nächste), nicht an Wiederaufbau,
/// Produkt oder Ausbau der Ruine.
pub fn line_before_ruin(
    w: &World,
    v: &WorldValues,
    isl: Option<&IslandRef>,
    s: Option<usize>,
    x: f64,
) -> Option<usize> {
    let si = s?;
    if !w.sites[si].ruin {
        return s;
    }
    let line = |o: &Site| level_site(w, o).is_some() && o.state == "unpaid" && site_payable(w, v, isl, o);
    nearest(&w.sites, |o| o.x, x, v.economy.pay_range_units, line).or(s)
}
