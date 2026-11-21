//! Verteilte Mauer- und Turm-Plätze in der Sim (Port von `engine/sim/world_lines.go`, B-652, BS12.2): Mauer und Turm kommen
//! aus `Layout.sites` der Stufe (Zone n von innen je Seite), ein Tor-Platz gibt es nicht mehr: Die äußerste gebaute Mauer
//! je Seite ist das Tor (`outer_wall`) und wird wie jede Mauer ausgebaut. Seite und Zone eines Platzes ergeben sich aus Art
//! und Position im Layout (`level_site`), nicht aus Feldern am Site: `destroy_site` und `castle_fallen` setzen den Platz
//! neu auf und würden sie sonst verlieren.
//!
//! Abweichung von Go: Gos Zwischenspeicher `wallLookup` am Platz fällt weg (Zustandsmodell, Entscheidung 4); `level_site`
//! sucht je Aufruf in `w.level.sites` nach Art und X. Das Ergebnis ist gleich, weil Art und X eines Platzes sich nie ändern.

use k3c_level::types::WallSite;

use super::base::base;
use super::hub_of;
use crate::state::data::has_depth;
use crate::state::{Site, World};

/// Go: `siteSpec`: ein anzulegender Platz.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SiteSpec {
    pub kind: String,
    pub x: f64,
}

/// Go: `worldSiteSpecs`: die Hub-Plätze dieser Tiefe und alle Mauern und Türme der Stufe, fest nach X sortiert (jeder Platz
/// zieht beim Anlegen eine ID). `has_depth` liest die Biom-Liste der eingebetteten Daten (Go: Paket-Variable `biomes`).
pub fn world_site_specs(w: &World) -> Result<Vec<SiteSpec>, String> {
    let biomes = &base()?.data.biomes;
    let mut out: Vec<SiteSpec> = Vec::new();
    let native = w.vals.as_ref().is_some_and(|v| v.native_rules);
    for s in &hub_of(w).sites {
        if w.biome.depth < s.from_depth || (s.needs_deeper && !has_depth(biomes, w.biome.depth + 1)) {
            continue;
        }
        if native && s.top_only && w.biome.depth > 0 {
            continue; // ECO1.6: Produktion nur ganz oben, unten bauen nur Kampfgebäude
        }
        out.push(SiteSpec {
            kind: s.kind.clone(),
            x: w.hub_x + s.offset_units,
        });
    }
    for l in &w.level.sites {
        out.push(SiteSpec {
            kind: l.kind.clone(),
            x: l.x,
        });
    }
    // Vec::sort_by ist stabil wie sort.SliceStable
    out.sort_by(|a, b| a.x.total_cmp(&b.x));
    Ok(out)
}

/// Go: `levelSite`: der verteilte Platz zu `s`; `None` für Plätze, die nicht aus `Layout.sites` stammen.
pub fn level_site(w: &World, s: &Site) -> Option<WallSite> {
    w.level.sites.iter().find(|l| l.kind == s.kind && l.x == s.x).cloned()
}

/// Go: `lineOpen`: Darf ein unbezahlter Mauer- oder Turm-Platz bezahlt werden? Zone n ab Hub-Stufe n, und im Freiraum
/// (`sitePlacement.clearRadiusUnits`) steht kein Baum und kein Fels mehr (B-652, 🧑 2026-10-28). Innere Mauern müssen nicht
/// gebaut sein.
pub fn line_open(w: &World, s: &Site) -> bool {
    match level_site(w, s) {
        None => true,
        Some(l) => w.hub_level >= l.zone && site_cleared(w, s.x),
    }
}

/// Der Freiraum-Radius um einen Mauer- oder Turm-Platz (Go: `level.DefaultSitePlacement().ClearRadiusUnits`, die
/// eingebetteten Werte). Ohne lesbare Daten 0: dann sperrt nichts.
fn clear_radius() -> f64 {
    base().map_or(0.0, |b| b.rules.site_placement.clear_radius_units)
}

/// Ein Baum oder Fels, der den Freiraum um `x` sperrt (abgebaute Knoten sind aus `nodes` entfernt oder `gone`).
fn blocks(n: &crate::state::ResourceNode, x: f64, radius: f64) -> bool {
    (n.kind == "tree" || n.kind == "rock") && !n.gone && (n.x - x).abs() < radius
}

/// Go: `siteCleared`: Im Freiraum um `x` steht kein Baum und kein Fels.
pub fn site_cleared(w: &World, x: f64) -> bool {
    let radius = clear_radius();
    !w.nodes.iter().any(|n| blocks(n, x, radius))
}

/// Go: `blockersAt`: die Bäume und Felsen im Freiraum um `x`, als Indizes in `w.nodes`.
pub fn blockers_at(w: &World, x: f64) -> Vec<usize> {
    let radius = clear_radius();
    w.nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| blocks(n, x, radius))
        .map(|(i, _)| i)
        .collect()
}

/// Go: `SiteBlockers`: die Bäume und Felsen (Indizes in `w.nodes`), die einen Mauer- oder Turm-Platz noch sperren (leer für
/// alle anderen Plätze); Bots markieren sie zum Abbau (B-652).
pub fn site_blockers(w: &World, s: &Site) -> Vec<usize> {
    if level_site(w, s).is_none() {
        return Vec::new();
    }
    blockers_at(w, s.x)
}

/// Go: `SiteNeedsClearing`: Wird ein Platz erst nach dem Roden bezahlbar (Baumschule, Mauer, Turm)? Seine Hub-Stufe ist
/// erreicht, aber im Freiraum stehen noch Bäume oder Felsen. Das Protokoll meldet es als `clearing`, der Client zeigt „erst
/// roden“ (B-636, B-652).
pub fn site_needs_clearing(w: &World, s: &Site) -> bool {
    level_site(w, s).is_some_and(|l| s.state == "unpaid" && w.hub_level >= l.zone && !site_cleared(w, s.x))
}
