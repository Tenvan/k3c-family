//! Freiraum (Port von `engine/balance/bot_clear.go`, B-652, B-636): Ein Mauer-, Turm- oder Baumschul-Platz wird erst
//! bezahlbar, wenn im Radius keine Bäume und Felsen mehr stehen. Die Profile markieren die Blocker des nächsten gesperrten
//! Platzes zum Abbau (Bauern holen sie), bevor sie dort bauen.

use std::sync::OnceLock;

use k3c_sim::state::site_view::site_hub_level_of;
use k3c_sim::state::site_view::site_zone;
use k3c_sim::state::{Player, ResourceNode, Site, World};
use k3c_sim::world::lines::site_blockers;

use crate::bot_boss::ENDBOSS_WANT;
use crate::bots::{closer, economy_view};
use crate::prices::prices;

/// Der Abstand zum Bau des Endbosses, den der Bot beim Markieren und Sammeln hält: Wer dort hinläuft, weckt den Endboss
/// (lair.triggerUnits 15) und seine Beschwörung räumt das Heer ab (B-652: Kristall-Knoten am Bau, Tag 27).
const LAIR_SAFE_UNITS: f64 = 60.0;

/// Der nächste Baum oder Fels, der einen schon freigeschalteten Platz sperrt und noch nicht markiert ist; `None`, wenn
/// nichts zu räumen ist oder das Gold für die Markierung fehlt. Gewählt wird der Platz mit den wenigsten Blockern (der
/// billigste Freiraum), Mauern vor Türmen, bei Gleichstand der innere.
pub fn clear_node<'a>(w: &'a World, p: &Player) -> Option<&'a ResourceNode> {
    let site = blocked_site(w)?;
    let mut best: Option<&ResourceNode> = None;
    for i in site_blockers(w, site) {
        let Some(n) = w.nodes.get(i) else {
            continue;
        };
        let Some(np) = prices().nodes.get(&n.kind) else {
            continue;
        };
        if n.marked || p.gold < np.mark_cost - n.paid_gold {
            continue;
        }
        if best.is_none_or(|b| closer(p.x, n.x, b.x)) {
            best = Some(n);
        }
    }
    best
}

/// Der Mauer-, Turm- oder Baumschul-Platz mit freigeschalteter Hub-Stufe, den noch Bäume oder Felsen sperren, mit dem
/// niedrigsten `rank`; `None`, wenn keiner gesperrt ist.
pub fn blocked_site(w: &World) -> Option<&Site> {
    let mut site: Option<&Site> = None;
    let want = WantSet::new(w);
    for s in &w.sites {
        if !matches!(s.kind.as_str(), "wall" | "tower" | "nursery")
            || s.state != "unpaid"
            || site_hub_level_of(w, s) > w.hub_level
        {
            continue;
        }
        if !site_blockers(w, s).is_empty() && want.wanted(s) && site.is_none_or(|b| rank(w, s) < rank(w, b)) {
            site = Some(s);
        }
    }
    site
}

/// Sortiert Plätze: Mauern vor Türmen vor Baumschulen, dann nach der Zahl der Blocker, dann nach Abstand zur Burg.
pub fn rank(w: &World, s: &Site) -> f64 {
    let mut d = (s.x - w.hub_x).abs();
    d += 1000.0 * site_blockers(w, s).len() as f64;
    match s.kind.as_str() {
        "tower" => d += 1e6,
        "nursery" => d += 2e6,
        _ => {}
    }
    d
}

/// Art, Seite und Zone eines Platzes, der nicht mehr unbezahlt ist (Go: `takenZone`).
#[derive(Debug)]
struct TakenZone<'a> {
    site: &'a Site,
    kind: &'a str,
    left: bool,
    zone: i64,
}

/// Der Bot baut je Seite, Zone und Art (Mauer, Turm) nur einen Platz, den billigsten (innersten); weitere Plätze der Zone
/// lässt er stehen, sonst verbrennt er das Startgold an verstreuten Mauern statt an Bauern und Bögen (B-652). Plätze, die
/// nicht aus den Zonen stammen, sind immer gewollt. `WantSet` sammelt die belegten Zonen einmal je Durchlauf über die
/// Plätze, statt je Platz erneut alle Plätze zu prüfen (B-654); das Ergebnis ist dasselbe. Die Liste entsteht erst beim
/// ersten Platz mit Zone (Go: `done`).
#[derive(Debug)]
pub struct WantSet<'a> {
    w: &'a World,
    taken: OnceLock<Vec<TakenZone<'a>>>,
}

impl<'a> WantSet<'a> {
    pub fn new(w: &'a World) -> Self {
        Self {
            w,
            taken: OnceLock::new(),
        }
    }

    /// Go: `wantSet.wanted`.
    pub fn wanted(&self, s: &Site) -> bool {
        let Some(z) = site_zone(self.w, s) else {
            return true;
        };
        let taken = self.taken.get_or_init(|| taken_zones(self.w));
        let left = s.x < self.w.hub_x;
        !taken
            .iter()
            .any(|t| !std::ptr::eq(t.site, s) && t.kind == s.kind && t.left == left && t.zone == z)
    }
}

fn taken_zones(w: &World) -> Vec<TakenZone<'_>> {
    w.sites
        .iter()
        .filter(|o| o.state != "unpaid")
        .filter_map(|o| {
            site_zone(w, o).map(|zone| TakenZone {
                site: o,
                kind: &o.kind,
                left: o.x < w.hub_x,
                zone,
            })
        })
        .collect()
}

/// `x` liegt nahe am Bau des Endbosses dieser Stufe, solange das Heer ihn nicht tragen kann (< `ENDBOSS_WANT` Kämpfer).
pub fn near_lair(w: &World, x: f64) -> bool {
    if economy_view(w).fighters >= ENDBOSS_WANT {
        return false;
    }
    w.level
        .entities
        .iter()
        .any(|e| e.kind == "lair" && (e.x - x).abs() < LAIR_SAFE_UNITS)
}
