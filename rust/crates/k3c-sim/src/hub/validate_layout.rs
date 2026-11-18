//! Abstands-Regeln der Prüfung (Port von `engine/sim/hub_validate_layout.go`; W0.1 B-206/AC-05, W11.1 B-409): Plätze und
//! Zahlziele allein aus der Kandidaten-hub.json und data/buildings.json, ohne Welt, in jeder Tiefe. Gos Paket-Variable
//! `biomes` kommt als Parameter.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use k3c_data::biome::Biome;
use k3c_data::buildings::BuildingData;
use k3c_data::hub::{HubData, SitePlacementData};
use k3c_det::golden::go_float;

use super::validate::{HubFinding, RULE_GAP, RULE_STAIRS, STAIRS_GAP_UNITS, all_sites, is_line_kind, site_name};

/// Ein Zahlziel mit Offset zur Hub-Mitte.
#[derive(Debug, Clone, PartialEq)]
pub struct HubTarget {
    pub name: String,
    pub x: f64,
}

fn target(name: impl Into<String>, x: f64) -> HubTarget {
    HubTarget { name: name.into(), x }
}

/// Go: `zoneTargets`: der feste Baugrund der Zone 1 (Mauer und Turm) beider Seiten; die übrigen Plätze der Zonen würfelt
/// der Generator und hält selbst Abstand.
pub fn zone_targets(p: &SitePlacementData) -> Vec<HubTarget> {
    let mut out = Vec::new();
    for dir in [-1.0, 1.0] {
        let k = format!("Z1@{dir:+.0}");
        out.push(target(format!("wall {k}"), dir * p.zone1_wall_units));
        out.push(target(format!("tower {k}"), dir * p.zone1_tower_units));
    }
    out
}

/// Go: `fixedTargets`: Burg, Hub- und Insel-Plätze (ohne Linien), Händler und Angebote, die es in Tiefe `depth` gibt
/// (`deeper`: es gibt eine tiefere Stufe).
pub fn fixed_targets(h: &HubData, bs: &BTreeMap<String, BuildingData>, depth: i64, deeper: bool) -> Vec<HubTarget> {
    let mut out = vec![target("castle", 0.0)];
    if depth == h.merchant.only_depth {
        out.push(target("merchant buy", h.merchant.buy_offset_units));
        out.push(target("merchant sell", h.merchant.sell_offset_units));
    }
    for s in all_sites(h) {
        if is_line_kind(&s.kind) || depth < s.from_depth || (s.needs_deeper && !deeper) {
            continue;
        }
        out.push(target(site_name(s), s.offset_units));
        for o in bs.get(&s.kind).map_or(&[][..], |b| &b.offers) {
            out.push(target(format!("{}:{}", s.kind, o.kind), s.offset_units + o.dx));
        }
    }
    out
}

/// Go: `eachLayout`: ruft `f` für jede Tiefe der Biome (mit und ohne tiefere Stufe) auf. `f` darf die Liste sortieren.
pub fn each_layout(
    h: &HubData,
    bs: &BTreeMap<String, BuildingData>,
    biomes: &[Biome],
    f: &mut dyn FnMut(&str, &mut [HubTarget]),
) {
    let mut depths: Vec<i64> = Vec::new();
    for b in biomes {
        if !depths.contains(&b.depth) {
            depths.push(b.depth);
        }
    }
    for depth in depths {
        for deeper in [true, false] {
            let desc = format!("Tiefe {depth} (tiefer {deeper})");
            let mut all = fixed_targets(h, bs, depth, deeper);
            all.extend(zone_targets(&h.site_placement));
            f(&desc, &mut all);
        }
    }
}

/// Merkt sich die gemeldeten Paare: jedes Paar wird nur einmal gemeldet (erste Lage).
struct Reporter {
    out: Vec<HubFinding>,
    seen: BTreeSet<String>,
}

impl Reporter {
    fn report(&mut self, rule: &str, desc: &str, a: &HubTarget, b: &HubTarget, limit: f64) {
        let key = format!("{rule}|{}|{}", a.name, b.name);
        if !self.seen.insert(key) {
            return;
        }
        self.out.push(HubFinding::new(
            rule,
            format!("{} / {}", a.name, b.name),
            format!(
                "{} und {} nur {} Units auseinander, mindestens {} ({desc})",
                go_float(a.x),
                go_float(b.x),
                go_float((b.x - a.x).abs()),
                go_float(limit)
            ),
        ));
    }
}

/// Go: `checkGaps`: Zwei Zahlziele liegen mindestens 2 × `pay_range` auseinander (kein Zahlziel überdeckt ein anderes),
/// Treppen mindestens `STAIRS_GAP_UNITS` zu jedem Zahlziel derselben Tiefe. Jedes Paar wird nur einmal gemeldet.
pub fn check_gaps(
    h: &HubData,
    bs: &BTreeMap<String, BuildingData>,
    pay_range: f64,
    biomes: &[Biome],
) -> Vec<HubFinding> {
    let min_gap = 2.0 * pay_range;
    let mut rep = Reporter {
        out: Vec::new(),
        seen: BTreeSet::new(),
    };
    each_layout(h, bs, biomes, &mut |desc, all| {
        // stabil, wie `sort.SliceStable`; gleiche Lage behält die Reihenfolge
        all.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal));
        for i in 1..all.len() {
            if all[i].x - all[i - 1].x < min_gap {
                rep.report(RULE_GAP, desc, &all[i - 1], &all[i], min_gap);
            }
        }
        for s in all.iter().filter(|s| s.name.starts_with("stairs")) {
            for o in all.iter() {
                if o.name != s.name && (o.x - s.x).abs() < STAIRS_GAP_UNITS {
                    rep.report(RULE_STAIRS, desc, s, o, STAIRS_GAP_UNITS);
                }
            }
        }
    });
    rep.out
}
