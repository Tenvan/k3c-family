//! Werte aus data/ für die Anzeige (Spiegel der Regeln in src/scenes/hud/economyLabels.ts): Namen, Baukosten,
//! Ausbau-Stufen, Bogen-Regal und Plantage der Gebäude, Bogen-Preis, Wurzeln je Biom. Einmal beim ersten Zugriff
//! gelesen; ein Lesefehler lässt die Werte leer (die Anzeige zeigt dann weniger, nie falsch).
//!
//! Die Dateien liest `k3c-data` (`Source::from_env`, wie `data.Files` in Go). Nur `rootsPerNode` je Biom kennt
//! `k3c_data::biome::Biome` nicht; es wird hier aus derselben Quelle gelesen.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use k3c_data::buildings::{BuildingData, Cost};
use k3c_data::{Source, buildings, troops};
use serde::Deserialize;

/// Die gelesenen Anzeige-Werte (Go: die Paket-Variablen `buildings`, `archerCost`, `rootsByBiom`, `depths`).
#[derive(Default)]
pub struct Values {
    pub buildings: BTreeMap<String, BuildingData>,
    /// `name` je Platzart (`k3c_data::buildings::BuildingData` kennt das Feld nicht).
    pub names: BTreeMap<String, String>,
    pub archer_cost: Cost,
    pub roots_by_biom: BTreeMap<String, i64>,
    pub depths: BTreeSet<i64>,
}

/// `RESOURCE_ORDER` ist die feste Reihenfolge der Rohstoffe.
pub const RESOURCE_ORDER: [&str; 5] = ["wood", "stone", "copper", "iron", "crystal"];

/// `resource_name`: deutscher Name eines Rohstoffs (texts.de.ts › res.*).
pub fn resource_name(r: &str) -> Option<&'static str> {
    Some(match r {
        "wood" => "Holz",
        "stone" => "Stein",
        "copper" => "Kupfer",
        "iron" => "Eisen",
        "crystal" => "Kristall",
        _ => return None,
    })
}

/// `values` liefert die Anzeige-Werte, beim ersten Aufruf gelesen.
pub fn values() -> &'static Values {
    static V: OnceLock<Values> = OnceLock::new();
    V.get_or_init(|| load(&Source::from_env()))
}

fn load(src: &Source) -> Values {
    let buildings = read(src, "buildings.json", buildings::load).unwrap_or_default();
    let archer_cost = read(src, "troops.json", troops::load)
        .and_then(|t| t.get("archer").map(|a| a.cost.clone()))
        .unwrap_or_default();
    let (roots_by_biom, depths) = read_biomes(src);
    let names = read(src, "buildings.json", |raw| {
        serde_json::from_str::<BTreeMap<String, NameOnly>>(raw)
            .map_err(|e| k3c_data::Fehler::Json(format!("data/buildings.json: {e}")))
    })
    .map(|m| m.into_iter().map(|(k, v)| (k, v.name)).collect())
    .unwrap_or_default();
    Values {
        buildings,
        names,
        archer_cost,
        roots_by_biom,
        depths,
    }
}

/// Nur der Anzeigename eines Eintrags aus data/buildings.json.
#[derive(Deserialize, Default)]
#[serde(default)]
struct NameOnly {
    #[serde(rename = "name", alias = "Name")]
    name: String,
}

/// `read` liest eine Datei aus data/; Fehler landen im Log, das Ergebnis bleibt dann leer.
fn read<T>(src: &Source, name: &str, parse: fn(&str) -> Result<T, k3c_data::Fehler>) -> Option<T> {
    match src.read(name).and_then(|raw| parse(&raw)) {
        Ok(v) => Some(v),
        Err(e) => {
            k3c_platform::log::error("💥 Anzeige-Werte nicht lesbar", &[("datei", &name), ("err", &e)]);
            None
        }
    }
}

/// Der Teil eines Bioms, den die Anzeige braucht (Go: anonyme Struktur in `readBiomes`).
#[derive(Deserialize, Default)]
#[serde(default)]
struct BiomeRoots {
    #[serde(rename = "id", alias = "ID", alias = "Id")]
    id: String,
    #[serde(rename = "depth", alias = "Depth")]
    depth: i64,
    #[serde(rename = "rootsPerNode", alias = "RootsPerNode")]
    roots_per_node: i64,
}

/// `read_biomes` liest rootsPerNode je Biom (data/biomes/*.json, fehlt = 1) und die Tiefen, die es gibt (Ausgang:
/// „Tiefe n“ oder verschüttet).
pub fn read_biomes(src: &Source) -> (BTreeMap<String, i64>, BTreeSet<i64>) {
    let (mut roots, mut deep) = (BTreeMap::new(), BTreeSet::new());
    let ids = match k3c_data::data::biome_ids(src) {
        Ok(ids) => ids,
        Err(e) => {
            k3c_platform::log::error("💥 Biome nicht lesbar", &[("err", &e)]);
            return (roots, deep);
        }
    };
    for id in ids {
        let name = format!("biomes/{id}.json");
        let parsed = src.read(&name).and_then(|raw| {
            serde_json::from_str::<BiomeRoots>(&raw).map_err(|e| k3c_data::Fehler::Json(format!("{name}: {e}")))
        });
        match parsed {
            Ok(b) if !b.id.is_empty() => {
                roots.insert(b.id, b.roots_per_node.max(1));
                deep.insert(b.depth);
            }
            Ok(_) => {}
            Err(e) => k3c_platform::log::error("💥 Anzeige-Werte nicht lesbar", &[("datei", &name), ("err", &e)]),
        }
    }
    (roots, deep)
}

/// `building_level_count` ist die Zahl der `levels` eines Gebäudes in den Daten (0 ohne Eintrag).
pub fn building_level_count(kind: &str) -> usize {
    values().buildings.get(kind).map_or(0, |b| b.levels.len())
}

/// `site_name` ist der Anzeigename einer Platzart (data/buildings.json › name; das Schiff steht dort nicht).
pub fn site_name(kind: &str) -> String {
    if let Some(n) = values().names.get(kind).filter(|n| !n.is_empty()) {
        return n.clone();
    }
    if kind == "ship" {
        return "Schiff".to_string();
    }
    kind.to_string()
}

/// `is_empty_cost`: Go prüft `len(cost) > 0` (Schlüssel vorhanden); die Struktur kennt nur Werte, ein Eintrag mit
/// lauter Nullen zählt wie ein fehlender (die Daten haben keinen solchen Eintrag außer `{}`).
fn is_empty_cost(c: &Cost) -> bool {
    *c == Cost::default()
}

/// `build_cost` sind die Baukosten eines unbezahlten Platzes laut Daten: der Eintrag der Biom-Tiefe in costByDepth
/// (Treppen), fehlt er oder ist er leer, cost der Art (Spiegel von siteCost in engine/sim/hub_level.go).
pub fn build_cost(kind: &str, depth: i64) -> Cost {
    let Some(b) = values().buildings.get(kind) else {
        return Cost::default();
    };
    if depth >= 0 && (depth as usize) < b.cost_by_depth.len() && !is_empty_cost(&b.cost_by_depth[depth as usize]) {
        return b.cost_by_depth[depth as usize].clone();
    }
    b.cost.clone()
}

/// `upgrade_cost` sind die Kosten des Ausbaus eines gebauten Platzes der Stufe level auf level+1 (`None` ohne Stufe).
pub fn upgrade_cost(kind: &str, level: i64) -> Option<Cost> {
    let lv = &values().buildings.get(kind)?.levels;
    let level = level.max(1);
    if level as usize >= lv.len() {
        return None;
    }
    Some(lv[level as usize].cost.clone())
}

/// `cost_of` liest den Betrag eines Rohstoffs oder von Gold aus einer Kostenliste (Go: `c[r]`, fehlt = 0).
pub fn cost_of(c: &Cost, res: &str) -> i64 {
    match res {
        "gold" => c.gold,
        "wood" => c.wood,
        "stone" => c.stone,
        "copper" => c.copper,
        "iron" => c.iron,
        "crystal" => c.crystal,
        _ => 0,
    }
}

/// `materials` nennt die Rohstoffe (ohne Gold) einer Kostenliste in fester Reihenfolge.
pub fn materials(c: &Cost) -> Vec<&'static str> {
    RESOURCE_ORDER.iter().copied().filter(|r| cost_of(c, r) > 0).collect()
}
