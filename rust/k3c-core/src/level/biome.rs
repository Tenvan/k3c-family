//! Biom und Regelwerte des Level-Generators (Port von `engine/level/biome.go`, `lines.go` und den Ladefunktionen in
//! `sites.go`, `guards.go`, `ruins.go`). Rust liest `data/` nicht selbst: Go liefert Biom und Regeln als JSON
//! (`engine/core/level.go`, `LevelInput`).

use serde::Deserialize;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;

/// Bereich min..max (beide inklusive).
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct Range {
    pub min: i64,
    pub max: i64,
}

/// JSON-Objekt in Datei-Reihenfolge. Liest die Datei-Form (`{"a": 1}`) und die Form von `json.Marshal` des
/// Go-Typs `Ordered[V]` (`[{"Key": "a", "Value": 1}]`); `null` ist leer.
#[derive(Clone, Debug, Default)]
pub struct Ordered<V>(pub Vec<(String, V)>);

impl<V> Ordered<V> {
    /// Wert zu `key`.
    pub fn get(&self, key: &str) -> Option<&V> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

#[derive(Deserialize)]
struct Entry<V> {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: V,
}

struct OrderedVisitor<V>(PhantomData<V>);

impl<'de, V: Deserialize<'de>> Visitor<'de> for OrderedVisitor<V> {
    type Value = Ordered<V>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("JSON-Objekt oder Liste von {Key, Value}")
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(Ordered(Vec::new()))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut out = Vec::new();
        while let Some((k, v)) = map.next_entry::<String, V>()? {
            out.push((k, v));
        }
        Ok(Ordered(out))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut out = Vec::new();
        while let Some(e) = seq.next_element::<Entry<V>>()? {
            out.push((e.key, e.value));
        }
        Ok(Ordered(out))
    }
}

impl<'de, V: Deserialize<'de>> Deserialize<'de> for Ordered<V> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(OrderedVisitor(PhantomData))
    }
}

/// `null` wie in Go als Nullwert lesen (Go schreibt leere Listen als `null`).
pub fn nullable<'de, D: Deserializer<'de>, T: Deserialize<'de> + Default>(
    d: D,
) -> Result<T, D::Error> {
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Portals {
    pub count: i64,
    pub min_distance_from_hub_units: f64,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Veins {
    pub count: i64,
    #[serde(deserialize_with = "nullable")]
    pub kind: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Enemies {
    #[serde(deserialize_with = "nullable")]
    pub portal: Vec<String>,
}

/// Die Felder eines Bioms, die der Generator braucht (Go: `level.Biome`; Zyklus und Lava braucht nur die Sim).
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Biome {
    pub id: String,
    pub depth: i64,
    pub length_units: Range,
    pub chunk_width_units: f64,
    pub hub_width_units: f64,
    pub exit_side: String,
    pub portals: Portals,
    pub chunk_weights: Ordered<f64>,
    #[serde(deserialize_with = "nullable")]
    pub event_chunks: BTreeMap<String, Range>,
    pub skill_points: Range,
    pub resources_per_chunk: Ordered<Ordered<[i64; 2]>>,
    pub veins: Veins,
    pub backdrop_variants: i64,
    pub lair: bool,
    pub enemies: Enemies,
}

impl Biome {
    /// Bereich einer Ereignis-Art (fehlt sie: 0..0 wie die Go-map).
    pub fn event(&self, kind: &str) -> Range {
        self.event_chunks.get(kind).copied().unwrap_or_default()
    }
}

/// `data/hub.json › sitePlacement` (Go: `SitePlacement`).
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SitePlacement {
    pub zone_start_units: f64,
    pub zone1_wall_units: f64,
    pub zone1_tower_units: f64,
    pub zones: i64,
    pub per_zone_min: i64,
    pub per_zone_max: i64,
    pub tower_near_share: f64,
    pub tower_near_min_units: f64,
    pub tower_near_max_units: f64,
    pub clear_radius_units: f64,
    pub edge_margin_units: f64,
    pub outside_units: f64,
    pub nursery_min_units: f64,
}

/// Hub- oder Insel-Platz (`data/hub.json › sites, islandSites`): Art und Abstand zur Hub-Mitte, links negativ.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct HubSite {
    pub kind: String,
    pub offset_units: f64,
}

/// `data/guards.json` (Go: `GuardRules`).
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct GuardRules {
    pub aggro_radius_units: f64,
    pub max_per_chunk: i64,
    pub min_gap_units: f64,
}

/// Regelwerte aus `data/` für Plätze und Prüfung der Planung. `ruin_edges`: Art → Breite des Plantagen-Streifens
/// (`buildings.json › plantation.stripUnits`), nur für Arten mit Plantage.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Rules {
    pub pay_range_units: f64,
    pub site_placement: SitePlacement,
    #[serde(deserialize_with = "nullable")]
    pub hub_sites: Vec<HubSite>,
    pub guards: GuardRules,
    #[serde(deserialize_with = "nullable")]
    pub ruin_kinds: Vec<String>,
    #[serde(deserialize_with = "nullable")]
    pub ruin_edges: BTreeMap<String, f64>,
}

impl Rules {
    /// Mindestabstand zweier Zahlziele: 2 × payRangeUnits (Go: `payGapUnits`).
    pub fn pay_gap(&self) -> f64 {
        2.0 * self.pay_range_units
    }
}

/// Eingabe des Generators: Biom und Regeln (`{"biome": …, "rules": …}`).
#[derive(Clone, Debug, Deserialize)]
pub struct Input {
    pub biome: Biome,
    pub rules: Rules,
}
