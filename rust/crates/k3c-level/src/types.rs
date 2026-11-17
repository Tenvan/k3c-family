//! Level-Typen als reine Daten (Port der Typ-Definitionen aus `engine/level`); den Generator liefert RU6.
//!
//! JSON-Namen wie die Go-Tags. Felder mit `json:"-"` sind `#[serde(skip)]`. `Biome`, `Cycle`, `Trait` und
//! `WeightGroup` kommen aus `k3c-data` (RU3) und werden hier nur wiedergegeben.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use k3c_data::biome::{Biome, Cycle, Trait, WeightGroup};

/// Go: `level.Chunk`, ein Abschnitt fester Breite.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Chunk {
    #[serde(rename = "index")]
    pub index: i64,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "startUnits")]
    pub start_units: f64,
}

/// Go: `level.Entity`, ein Objekt im Level (Burg, Portal, Baum, Truhe, Skill-Punkt …).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Entity {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "x")]
    pub x: f64,
    /// Variante aus einer Gewichts-Gruppe des Bioms (Baumart, Kristallfarbe); leer ohne Gruppe.
    #[serde(skip)]
    pub variant: String,
}

/// Go: `level.WallSite`, ein verteilter Mauer- oder Turm-Platz einer Seite (ohne JSON-Tags in Go; im Layout `json:"-"`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WallSite {
    /// -1 links, +1 rechts.
    #[serde(rename = "Side")]
    pub side: i64,
    /// "wall" oder "tower".
    #[serde(rename = "Kind")]
    pub kind: String,
    /// Absolute Lage in Units.
    #[serde(rename = "X")]
    pub x: f64,
    /// Zone n von innen, ab 1.
    #[serde(rename = "Zone")]
    pub zone: i64,
}

/// Go: `level.Ruin`, eine Ruine im Level: Art und X-Position in Units.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ruin {
    #[serde(rename = "Kind")]
    pub kind: String,
    #[serde(rename = "X")]
    pub x: f64,
}

/// Go: `level.Guard`, eine Wache im Level: Gegnerart und X-Position in Units.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Guard {
    #[serde(rename = "Kind")]
    pub kind: String,
    #[serde(rename = "X")]
    pub x: f64,
}

/// Go: `level.NatureValues`, Werte der Natur-Eigenschaften (Traits je Schlüssel, Weights je Gruppe und Variante).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct NatureValues {
    #[serde(rename = "traits", skip_serializing_if = "BTreeMap::is_empty")]
    pub traits: BTreeMap<String, i64>,
    #[serde(rename = "weights", skip_serializing_if = "BTreeMap::is_empty")]
    pub weights: BTreeMap<String, BTreeMap<String, i64>>,
}

impl NatureValues {
    /// Go: `Empty`, kein Wert gesetzt.
    pub fn is_empty(&self) -> bool {
        self.traits.is_empty() && self.weights.is_empty()
    }
}

/// Go: `level.PlannedObject`, ein festes Objekt eines geplanten Chunks; `at` zählt ab Chunk-Anfang.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PlannedObject {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "at")]
    pub at: f64,
}

/// Go: `level.PlannedBuilding`, die Ruine eines geplanten Chunks.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PlannedBuilding {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "at")]
    pub at: f64,
}

/// Go: `level.PlannedEnemy`, eine Wache eines Gegnerlagers.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PlannedEnemy {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "at")]
    pub at: f64,
}

/// Go: `level.PlannedChunk`; `slot` zählt relativ zu den Hub-Chunks (−n links, +n rechts, nie 0).
/// Go liest streng (`UnmarshalJSON`): ein unbekanntes Feld ist ein Fehler.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PlannedChunk {
    #[serde(rename = "slot")]
    pub slot: i64,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "seed", skip_serializing_if = "String::is_empty")]
    pub seed: String,
    #[serde(rename = "objects", skip_serializing_if = "Vec::is_empty")]
    pub objects: Vec<PlannedObject>,
    #[serde(rename = "buildings", skip_serializing_if = "Vec::is_empty")]
    pub buildings: Vec<PlannedBuilding>,
    #[serde(rename = "enemies", skip_serializing_if = "Vec::is_empty")]
    pub enemies: Vec<PlannedEnemy>,
}

/// Go: `level.Layout`, ein fertiges Level (gleiche JSON-Form wie `LevelLayout` in TS).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Layout {
    #[serde(rename = "seed")]
    pub seed: String,
    #[serde(rename = "biomeId")]
    pub biome_id: String,
    #[serde(rename = "widthUnits")]
    pub width_units: f64,
    #[serde(rename = "chunkWidthUnits")]
    pub chunk_width_units: f64,
    #[serde(rename = "hubCenterUnits")]
    pub hub_center_units: f64,
    #[serde(rename = "chunks")]
    pub chunks: Vec<Chunk>,
    #[serde(rename = "entities")]
    pub entities: Vec<Entity>,
    /// Verteilte Mauer- und Turm-Plätze je Seite und Zone (nicht Teil der level-Nachricht).
    #[serde(skip)]
    pub sites: Vec<WallSite>,
    /// Orte (X) der Plätze auf freiem Baugrund.
    #[serde(skip)]
    pub free_ground: Vec<f64>,
    /// Ruinen der geplanten Chunks.
    #[serde(skip)]
    pub ruins: Vec<Ruin>,
    /// Wachen der Gegnerlager geplanter Chunks.
    #[serde(skip)]
    pub guards: Vec<Guard>,
    /// Hintergrund-Variante je Kachel; ohne Varianten im Biom fehlt das Feld.
    #[serde(rename = "backdrops", skip_serializing_if = "Vec::is_empty")]
    pub backdrops: Vec<i64>,
    /// Geltende Natur-Werte der Stufe (nicht Teil der level-Nachricht).
    #[serde(skip)]
    pub nature: NatureValues,
}

#[cfg(test)]
mod tests {
    use k3c_det::golden;

    use super::*;

    /// Jedes Level der Level-Golden lädt als `Layout` und wird als derselbe JSON-Baum geschrieben.
    #[test]
    fn layout_rundlauf() {
        let mut n = 0;
        for name in ["cave", "crystal", "forest", "ironhold", "mine", "planned"] {
            let g = golden::read(golden::testdata().join(format!("golden/level-{name}.json")));
            for (i, v) in g["levels"].as_array().unwrap().iter().enumerate() {
                let l: Layout = serde_json::from_value(v.clone()).unwrap();
                let d = golden::diff(&format!("level-{name}[{i}]"), v, &golden::tree(&l));
                assert!(d.is_empty(), "{d}");
                n += 1;
            }
        }
        assert!(n > 0);
    }

    #[test]
    fn planned_chunk_liest_streng() {
        assert!(serde_json::from_str::<PlannedChunk>(r#"{"slot":1,"kind":"x","object":[]}"#).is_err());
        let c: PlannedChunk = serde_json::from_str(r#"{"slot":-1,"kind":"x"}"#).unwrap();
        assert_eq!(serde_json::to_string(&c).unwrap(), r#"{"slot":-1,"kind":"x"}"#);
    }

    #[test]
    fn nature_values_leer() {
        let v = NatureValues::default();
        assert!(v.is_empty());
        assert_eq!(serde_json::to_string(&v).unwrap(), "{}");
    }
}
