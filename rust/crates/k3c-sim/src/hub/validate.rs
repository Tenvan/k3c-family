//! Prüfung einer Kandidaten-hub.json (Port von `engine/sim/hub_validate.go`; WZ15.3, B-462/AC-02): Der Gebäude-Planer prüft
//! eine geänderte hub.json, bevor er sie schreibt oder in der Vorschau baut. Die Prüfung nimmt den Kandidaten als
//! Parameter und ändert keinen globalen Zustand; sie liest nur die eingebetteten Daten, die der Kandidat nicht enthält
//! (Gebäude, `pay_range_units`, Biome). Die Abstands-Regeln stehen in `validate_layout.rs`.
//!
//! Gos Paket-Variablen `buildings`, `economy`, `biomes` und `StandardWorld()` kommen aus den eingebetteten Daten
//! (`world::base`).

use std::collections::BTreeMap;
use std::fmt;
use std::sync::OnceLock;

use k3c_data::biome::Biome;
use k3c_data::buildings::{BuildingData, Cost};
use k3c_data::hub::{HubData, HubSite, SitePlacementData};
use k3c_data::{Fehler, hub};
use k3c_level::rules::Rules;

use super::level::add_cost;
use super::sites::check_site_bounds;
use super::validate_layout::check_gaps;
use crate::island::data::check_planned_chunks_with;
use crate::island::{WorldDef, standard_world};
use crate::world::base::base;
use crate::world::level_sites;

/// Ein Befund: Regel, betroffener Platz oder Feld und was verletzt ist (Wert, Grenze).
#[derive(Debug, Clone, PartialEq)]
pub struct HubFinding {
    /// Regel-Kennung, siehe die `RULE_*`-Konstanten.
    pub rule: String,
    /// Platz (`kind@offset`) oder Feld (`wallLines`, `levels` …).
    pub site: String,
    pub detail: String,
}

impl HubFinding {
    pub fn new(rule: &str, site: impl Into<String>, detail: impl Into<String>) -> Self {
        HubFinding {
            rule: rule.to_string(),
            site: site.into(),
            detail: detail.into(),
        }
    }
}

/// Go: `HubFinding.String`: `regel: platz: detail`.
impl fmt::Display for HubFinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}: {}", self.rule, self.site, self.detail)
    }
}

/// Das Ergebnis der Prüfung: Befunde (leer = gültig) und Hinweise ohne Befund.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HubReport {
    pub findings: Vec<HubFinding>,
    pub notes: Vec<String>,
}

/// Kandidat ist kein gültiges JSON.
pub const RULE_JSON: &str = "json";
/// Hub-Stufe außerhalb 1 … Anzahl levels.
pub const RULE_LEVELS: &str = "stufe";
/// Unbekannte Art oder Art, die nicht in sites gehört.
pub const RULE_KIND: &str = "art";
/// Pflichtplatz fehlt oder steht mehrfach.
pub const RULE_REQUIRED: &str = "pflichtplatz";
pub const RULE_ONE_PER_BAU: &str = "ein-platz-je-bau";
/// Mauerlinien unstimmig oder zu nah am Portal.
pub const RULE_LINES: &str = "linien";
/// Zwei Zahlziele näher als 2 × payRangeUnits bei voller Streuung.
pub const RULE_GAP: &str = "abstand";
/// Treppe näher als `STAIRS_GAP_UNITS` an einem Zahlziel derselben Tiefe.
pub const RULE_STAIRS: &str = "treppe";
/// Material-Tor nicht aus dem Material der eigenen Hub-Stufe (B-631).
pub const RULE_GATE: &str = "material-tor";
/// Eine geplante Chunk-Planung der Welt wäre mit dem Kandidaten ungültig (B-479); der Befund nennt Insel, Tiefe, Platz.
pub const RULE_PLANNED: &str = "geplante-chunks";

/// Treppen brauchen Platz für Preisschild und Wartegrund (W11.1, B-409).
pub const STAIRS_GAP_UNITS: f64 = 8.0;

/// Die Pflichtplätze (WZ15.3): ohne Werkstatt keine Kämpfer, ohne Treppen kein Stufenwechsel.
const REQUIRED_SITE_KINDS: [&str; 3] = ["workshop", "stairsDown", "stairsUp"];

/// Go: `RequiredSiteKinds`: die Pflichtplätze.
pub fn required_site_kinds() -> Vec<&'static str> {
    REQUIRED_SITE_KINDS.to_vec()
}

/// Go: `ParseHub`: liest eine Kandidaten-hub.json.
pub fn parse_hub(raw: &str) -> Result<HubData, Fehler> {
    hub::load(raw)
}

/// Go: `ValidateHub`: prüft eine Kandidaten-hub.json als Text (`parse_hub`, dann `validate_hub_data`).
pub fn validate_hub(raw: &str) -> HubReport {
    match parse_hub(raw) {
        Ok(h) => validate_hub_data(&h),
        Err(e) => HubReport {
            findings: vec![HubFinding::new(RULE_JSON, "hub.json", e.to_string())],
            notes: Vec::new(),
        },
    }
}

/// Gos `StandardWorld()`: die Standard-Welt der eingebetteten Daten, einmal gebaut.
fn standard() -> Result<&'static WorldDef, String> {
    static STANDARD: OnceLock<Result<WorldDef, String>> = OnceLock::new();
    let b = base()?;
    STANDARD
        .get_or_init(|| standard_world(&b.data, &b.rules))
        .as_ref()
        .map_err(Clone::clone)
}

/// Go: `ValidateHubData`: prüft einen Kandidaten mit allen Platz-Regeln; `h` wird nicht verändert. Die Abstands-Regeln
/// laufen nur bei stimmigem Baugrund der Zone 1, weil sie dessen Lage voraussetzen. Fehlen die eingebetteten Daten (nicht zu
/// erwarten), ist das ein Befund der Regel `json`.
pub fn validate_hub_data(h: &HubData) -> HubReport {
    match standard() {
        Ok(world) => validate_hub_data_in(h, world),
        Err(e) => HubReport {
            findings: vec![HubFinding::new(RULE_JSON, "standardWorld", e)],
            notes: Vec::new(),
        },
    }
}

/// Go: `ValidateHubDataIn`: wie `validate_hub_data`, die geplanten Chunks werden aber in der Welt `world` geprüft (B-479).
pub fn validate_hub_data_in(h: &HubData, world: &WorldDef) -> HubReport {
    let b = match base() {
        Ok(b) => b,
        Err(e) => {
            return HubReport {
                findings: vec![HubFinding::new(RULE_JSON, "base", e)],
                notes: Vec::new(),
            };
        }
    };
    let biomes = &b.data.biomes;
    let mut out = check_hub_levels(h);
    out.extend(check_gates(h));
    // aufgelöste Welt: Offers und payRangeUnits aus ihrem Wertesatz (wie `check_world_hub_sites`)
    let (bs, pay_range) = match &world.values {
        Some(v) => (&v.buildings, v.economy.pay_range_units),
        None => (&b.data.buildings, b.data.economy.pay_range_units),
    };
    out.extend(check_site_kinds(h, bs));
    out.extend(check_site_bounds(h, biomes));
    let placement = check_site_placement(&h.site_placement, biomes);
    let ok = placement.is_empty();
    out.extend(placement);
    if ok {
        out.extend(check_gaps(h, bs, pay_range, biomes));
        out.extend(validate_planned(world, h, biomes, &b.rules));
    }
    HubReport {
        findings: out,
        notes: Vec::new(),
    }
}

/// Go: `validatePlanned`: prüft die geplanten Chunks aller Inseln der Welt (B-479): Eine Änderung, die eine Planung
/// ungültig macht, würde das Laden der Welt scheitern lassen und wird abgelehnt.
pub fn validate_planned(world: &WorldDef, h: &HubData, biomes: &[Biome], rules: &Rules) -> Vec<HubFinding> {
    let sites = level_sites(h); // die Plätze des Kandidaten, wie `create_world` sie dem Generator gibt (B-983)
    let mut out = Vec::new();
    for d in &world.islands {
        if let Err(e) = check_planned_chunks_with(d, Some(&sites), biomes, rules) {
            out.push(HubFinding::new(RULE_PLANNED, format!("Insel {}", d.id), e));
        }
    }
    out
}

/// Go: `allSites`: Hub- und Insel-Plätze in Dateireihenfolge.
pub fn all_sites(h: &HubData) -> impl Iterator<Item = &HubSite> {
    h.sites.iter().chain(&h.island_sites)
}

/// Go: `siteName`: `kind@offset` mit Vorzeichen, ohne Nachkommastellen.
pub fn site_name(s: &HubSite) -> String {
    format!("{}@{:+.0}", s.kind, s.offset_units)
}

/// Go: `checkHubLevels`: Es gibt Hub-Stufen, und jede Stufe eines Platzes liegt in 1 … Anzahl levels.
pub fn check_hub_levels(h: &HubData) -> Vec<HubFinding> {
    let n = h.levels.len() as i64;
    if n == 0 {
        return vec![HubFinding::new(
            RULE_LEVELS,
            "levels",
            "mindestens eine Hub-Stufe erwartet",
        )];
    }
    all_sites(h)
        .filter(|s| s.hub_level < 1 || s.hub_level > n)
        .map(|s| {
            HubFinding::new(
                RULE_LEVELS,
                site_name(s),
                format!("hubLevel {} außerhalb 1…{n}", s.hub_level),
            )
        })
        .collect()
}

/// Go: `fmt.Sprintf("%+v", cost)`: `{Gold:0 Wood:1 …}`.
fn go_cost(c: &Cost) -> String {
    format!(
        "{{Gold:{} Wood:{} Stone:{} Copper:{} Iron:{} Crystal:{}}}",
        c.gold, c.wood, c.stone, c.copper, c.iron, c.crystal
    )
}

/// Go: `checkGates`: Das Material-Tor eines Ausbauschritts nutzt nur Material, das die Schritte derselben Hub-Stufe kosten
/// (B-631: Material der eigenen Stufe, damit der Ausbau ohne Material tieferer Stufen läuft).
pub fn check_gates(h: &HubData) -> Vec<HubFinding> {
    let mut out = Vec::new();
    for (n, lv) in h.levels.iter().enumerate() {
        let own = lv.steps.iter().fold(Cost::default(), |a, st| add_cost(&a, &st.cost));
        for (i, st) in lv.steps.iter().enumerate() {
            let g = &st.gate;
            let pairs = [
                (g.wood, own.wood),
                (g.stone, own.stone),
                (g.copper, own.copper),
                (g.iron, own.iron),
                (g.crystal, own.crystal),
            ];
            if pairs.iter().any(|&(gate, have)| gate > 0 && have == 0) {
                out.push(HubFinding::new(
                    RULE_GATE,
                    format!("levels[{n}].steps[{i}].gate"),
                    format!("{} nicht aus dem Material der Stufe {}", go_cost(g), n + 1),
                ));
            }
        }
    }
    out
}

/// Go: `placeableKind`: Arten aus data/buildings.json, die auf einem Hub- oder Insel-Platz stehen dürfen (Burg und Mauer
/// und Turm kommen aus Hub-Mitte und sitePlacement).
pub fn placeable_kind(bs: &BTreeMap<String, BuildingData>, kind: &str) -> bool {
    bs.contains_key(kind) && kind != "castle" && !is_line_kind(kind)
}

/// Go: `isLineKind`: Mauer und Turm gehören zu den verteilten Plätzen der Zonen, nicht zu den Hub-Plätzen.
pub fn is_line_kind(kind: &str) -> bool {
    kind == "wall" || kind == "tower"
}

/// Go: `sideOf`: die Seite eines Offsets, -1 links, +1 rechts.
fn side_of(x: f64) -> usize {
    if x < 0.0 { 0 } else { 1 }
}

/// Go: `checkSiteKinds`: nur bekannte Arten; Pflichtplätze genau einmal, optionale höchstens einmal.
pub fn check_site_kinds(h: &HubData, bs: &BTreeMap<String, BuildingData>) -> Vec<HubFinding> {
    let mut out = Vec::new();
    let mut count: BTreeMap<&str, [i64; 2]> = BTreeMap::new(); // je Art: links, rechts
    for s in all_sites(h) {
        if s.kind == "castle" || is_line_kind(&s.kind) {
            out.push(HubFinding::new(
                RULE_KIND,
                site_name(s),
                "gehört nicht in sites (Hub-Mitte bzw. sitePlacement)",
            ));
        } else if !placeable_kind(bs, &s.kind) {
            out.push(HubFinding::new(
                RULE_KIND,
                site_name(s),
                format!("unbekannte Art {:?}", s.kind),
            ));
        }
        count.entry(&s.kind).or_default()[side_of(s.offset_units)] += 1;
    }
    for kind in REQUIRED_SITE_KINDS {
        let n = count.get(kind).map_or(0, |c| c[0] + c[1]);
        if n != 1 {
            out.push(HubFinding::new(
                RULE_REQUIRED,
                kind,
                format!("Pflichtplatz: genau einer erwartet, gefunden {n}"),
            ));
        }
    }
    for (kind, c) in &count {
        let n = c[0] + c[1];
        if placeable_kind(bs, kind) && !REQUIRED_SITE_KINDS.contains(kind) && n > 1 {
            out.push(HubFinding::new(
                RULE_ONE_PER_BAU,
                *kind,
                format!("höchstens ein Platz erwartet, gefunden {n}"),
            ));
        }
    }
    out
}

/// Go: `checkSitePlacement`: Der feste Baugrund der Zone 1 hat positive Lagen, der Turm steht innen vor der Mauer, und die
/// Verteidigungszone endet unter dem Portal-Mindestabstand jedes Bioms.
pub fn check_site_placement(p: &SitePlacementData, biomes: &[Biome]) -> Vec<HubFinding> {
    use k3c_det::golden::go_float as f;
    const SITE: &str = "sitePlacement";
    let mut out = Vec::new();
    if p.zone1_tower_units <= 0.0 || p.zone1_wall_units <= p.zone1_tower_units {
        out.push(HubFinding::new(
            RULE_LINES,
            SITE,
            format!(
                "Zone-1-Baugrund: Turm {} muss positiv und innen vor der Mauer {} liegen",
                f(p.zone1_tower_units),
                f(p.zone1_wall_units)
            ),
        ));
    }
    for b in biomes {
        let min = b.portals.min_distance_from_hub_units;
        if p.outside_units >= min {
            out.push(HubFinding::new(
                RULE_LINES,
                SITE,
                format!(
                    "{}: outsideUnits {} nicht unter dem Portal-Mindestabstand {}",
                    b.id,
                    f(p.outside_units),
                    f(min)
                ),
            ));
        }
    }
    out
}
