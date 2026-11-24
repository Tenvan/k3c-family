//! Port von `engine/simcli/hub.go` (RU33.2): die Engine-Seite des Gebäude-Planers. Die Prüfung und das Matrix-Modell
//! rechnet das Spiel; es läuft keine Simulation.
//!
//! ```text
//! {"op":"plan","raw":"<Text einer hub.json>"}   Matrix Hub-Stufe × Bauplatz, Befunde und Hinweise, erlaubte Arten
//! ```

use k3c_data::Source;
use k3c_data::buildings::LevelData;
use k3c_data::hub::{HubData, HubSite};
use k3c_room::room::preview_site_key;
use k3c_sim::hub::validate::{HubFinding, RULE_KIND, parse_hub, validate_hub_data};
use serde::Serialize;

use super::RunError;
use super::gojson::to_string;
use super::gotext::quote;
use super::input_world::HubInput;

/// Herkunft einer Zeile: Liste bzw. Block in hub.json.
const SRC_SITES: &str = "sites";
const SRC_ISLAND_SITES: &str = "islandSites";
const SRC_PLACEMENT: &str = "sitePlacement";
const SRC_MERCHANT: &str = "merchant";

/// Eine Zelle der Matrix (Hub-Stufe × Bauplatz): verfügbar ab der Stufe der Zeile, dann mit Platz und Tiefe.
#[derive(Serialize, Clone, Default)]
struct Cell {
    available: bool,
    /// Platz zur Hub-Mitte; bei Mauerlinie und Tor je Seite (±).
    #[serde(rename = "offsetUnits")]
    offset_units: f64,
    #[serde(rename = "fromDepth")]
    from_depth: i64,
    #[serde(rename = "needsDeeper")]
    needs_deeper: bool,
    /// Nur in dieser Tiefe (Händler); -1 = jede Tiefe.
    #[serde(rename = "onlyDepth")]
    only_depth: i64,
}

/// Ein Bauplatz der Matrix: Hub-Platz, Insel-Platz, Baugrund der Zone 1 (mit Turm) oder Händler.
#[derive(Serialize)]
struct Row {
    key: String,
    kind: String,
    source: String,
    index: i64,
    #[serde(rename = "hubLevel")]
    hub_level: i64,
    line: i64,
    #[serde(rename = "bothSides")]
    both_sides: bool,
    #[serde(rename = "towerUnits")]
    tower_units: f64,
    #[serde(rename = "jitterUnits")]
    jitter_units: f64,
    cells: Vec<Cell>,
}

#[derive(Serialize, Default)]
struct Matrix {
    levels: i64,
    rows: Vec<Row>,
}

#[derive(Serialize)]
struct Finding {
    rule: String,
    site: String,
    detail: String,
}

/// Die Antwort von `sim hub`.
#[derive(Serialize, Default)]
struct Report {
    matrix: Matrix,
    findings: Vec<Finding>,
    notes: Vec<String>,
    kinds: Vec<String>,
    #[serde(rename = "parseError")]
    parse_error: String,
}

/// Führt `sim hub` aus und liefert den Bericht als JSON-Text.
pub fn run(src: &Source, input: &HubInput) -> Result<String, RunError> {
    if input.op != "plan" {
        return Err(RunError::Usage(format!(
            "unbekannte op {} (bekannt: plan)",
            quote(&input.op)
        )));
    }
    Ok(to_string(&plan(src, &input.raw)))
}

/// Go: `hubPlan`: prüft den Text einer hub.json und baut die Matrix.
fn plan(src: &Source, raw: &str) -> Report {
    let mut rep = Report {
        kinds: placeable_kinds(src),
        ..Report::default()
    };
    let h = match parse_hub(raw) {
        Ok(h) => h,
        Err(e) => {
            rep.parse_error = e.to_string();
            return rep;
        }
    };
    rep.matrix = matrix_of(&h);
    let report = validate_hub_data(&h);
    rep.findings = report
        .findings
        .into_iter()
        .map(|f: HubFinding| Finding {
            rule: f.rule,
            site: f.site,
            detail: f.detail,
        })
        .collect();
    rep.notes = report.notes;
    rep
}

/// Go: `placeableKinds`: die Arten aus data/buildings.json, die als Hub- oder Insel-Platz stehen dürfen. Die Prüfung
/// entscheidet (Probe mit einem Platz je Art), damit die Workbench die Regel nicht nachbaut.
fn placeable_kinds(src: &Source) -> Vec<String> {
    let Ok(raw) = src.read("buildings.json") else {
        return Vec::new();
    };
    let Ok(serde_json::Value::Object(buildings)) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return Vec::new();
    };
    let mut kinds: Vec<&String> = buildings.keys().collect();
    kinds.sort();
    kinds
        .into_iter()
        .filter(|kind| {
            let probe = HubData {
                levels: vec![LevelData::default()],
                sites: vec![HubSite {
                    kind: (*kind).clone(),
                    offset_units: 1.0,
                    hub_level: 1,
                    ..HubSite::default()
                }],
                ..HubData::default()
            };
            let prefix = format!("{kind}@");
            !validate_hub_data(&probe)
                .findings
                .iter()
                .any(|f| f.rule == RULE_KIND && f.site.starts_with(&prefix))
        })
        .cloned()
        .collect()
}

/// Go: `hubMatrixOf`: das Matrix-Modell aus einer gelesenen hub.json.
fn matrix_of(h: &HubData) -> Matrix {
    let levels = h.levels.len() as i64;
    let mut rows = Vec::new();
    for (i, s) in h.sites.iter().enumerate() {
        rows.push(site_row(s, i, SRC_SITES, levels));
    }
    for (i, s) in h.island_sites.iter().enumerate() {
        rows.push(site_row(s, i, SRC_ISLAND_SITES, levels));
    }
    let wall = Cell {
        offset_units: h.site_placement.zone1_wall_units,
        only_depth: -1,
        ..Cell::default()
    };
    rows.push(Row {
        key: "zone1".into(),
        kind: "wall".into(),
        source: SRC_PLACEMENT.into(),
        index: 0,
        hub_level: 1,
        line: 1,
        both_sides: true,
        tower_units: h.site_placement.zone1_tower_units,
        jitter_units: 0.0,
        cells: cells(levels, 1, &wall),
    });
    for (kind, offset) in [
        ("merchantBuy", h.merchant.buy_offset_units),
        ("merchantSell", h.merchant.sell_offset_units),
    ] {
        let place = Cell {
            offset_units: offset,
            only_depth: h.merchant.only_depth,
            ..Cell::default()
        };
        rows.push(Row {
            key: preview_site_key(kind, offset),
            kind: kind.into(),
            source: SRC_MERCHANT.into(),
            index: 0,
            hub_level: 1,
            line: 0,
            both_sides: false,
            tower_units: 0.0,
            jitter_units: 0.0,
            cells: cells(levels, 1, &place),
        });
    }
    Matrix { levels, rows }
}

/// Go: `hubCells`: füllt die Zellen 1 … levels: ab der Stufe der Zeile verfügbar, dann mit `place`.
fn cells(levels: i64, from: i64, place: &Cell) -> Vec<Cell> {
    (0..levels)
        .map(|i| {
            if i + 1 >= from {
                Cell {
                    available: true,
                    ..place.clone()
                }
            } else {
                Cell::default()
            }
        })
        .collect()
}

/// Go: `hubSiteRow`: ein fehlender Wert (0) bei hubLevel heißt Stufe 1.
fn site_row(s: &HubSite, index: usize, source: &str, levels: i64) -> Row {
    let from = s.hub_level.max(1);
    let place = Cell {
        offset_units: s.offset_units,
        from_depth: s.from_depth,
        needs_deeper: s.needs_deeper,
        only_depth: -1,
        ..Cell::default()
    };
    Row {
        key: preview_site_key(&s.kind, s.offset_units),
        kind: s.kind.clone(),
        source: source.into(),
        index: index as i64,
        hub_level: from,
        line: 0,
        both_sides: false,
        tower_units: 0.0,
        jitter_units: 0.0,
        cells: cells(levels, from, &place),
    }
}
