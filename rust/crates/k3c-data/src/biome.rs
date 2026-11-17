//! data/biomes/*.json: Go `level.Biome` (engine/level/biome.go) mit `LoadBiome` und den Prüfungen aus
//! nature_check.go; `check_pool` wie `checkPool` in engine/sim/data.go.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::enemies::EnemyData;
use crate::ordered::Ordered;
use crate::{Fehler, decode};

/// Go: `level.Biome`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Biome {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "depth")]
    pub depth: i64,
    #[serde(rename = "lengthUnits")]
    pub length_units: Range,
    #[serde(rename = "chunkWidthUnits")]
    pub chunk_width_units: f64,
    #[serde(rename = "hubWidthUnits")]
    pub hub_width_units: f64,
    #[serde(rename = "exitSide")]
    pub exit_side: String,
    #[serde(rename = "portals")]
    pub portals: BiomePortals,
    #[serde(rename = "chunkWeights")]
    pub chunk_weights: Ordered<f64>,
    #[serde(rename = "eventChunks")]
    pub event_chunks: BTreeMap<String, Range>,
    #[serde(rename = "skillPoints")]
    pub skill_points: Range,
    #[serde(rename = "resourcesPerChunk")]
    pub resources_per_chunk: Ordered<Ordered<[i64; 2]>>,
    #[serde(rename = "cycle")]
    pub cycle: Cycle,
    #[serde(rename = "primaryResource")]
    pub primary_resource: String,
    #[serde(rename = "veins")]
    pub veins: BiomeVeins,
    #[serde(rename = "backdropVariants")]
    pub backdrop_variants: i64,
    #[serde(rename = "lair")]
    pub lair: bool,
    #[serde(rename = "lava")]
    pub lava: BiomeLava,
    #[serde(rename = "traits")]
    pub traits: Ordered<Trait>,
    #[serde(rename = "weights")]
    pub weights: Ordered<WeightGroup>,
    #[serde(rename = "enemies")]
    pub enemies: BiomeEnemies,
}

/// Go: `level.Range`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Range {
    #[serde(rename = "min")]
    pub min: i64,
    #[serde(rename = "max")]
    pub max: i64,
}

/// Go: `struct { Count int "json:\"count\""; MinDistanceFromHubUnits float64 "json:\"minDistanceFromHubUnits\""; HP float64 "json:\"hp\""; Defense struct { IntervalSeconds float64 "json:\"intervalSeconds\""; Size int "json:\"size\"" } "json:\"defense\""; Provisional bool "json:\"provisional\"" }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BiomePortals {
    #[serde(rename = "count")]
    pub count: i64,
    #[serde(rename = "minDistanceFromHubUnits")]
    pub min_distance_from_hub_units: f64,
    #[serde(rename = "hp")]
    pub hp: f64,
    #[serde(rename = "defense")]
    pub defense: BiomePortalsDefense,
    #[serde(rename = "provisional")]
    pub provisional: bool,
}

/// Go: `struct { IntervalSeconds float64 "json:\"intervalSeconds\""; Size int "json:\"size\"" }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BiomePortalsDefense {
    #[serde(rename = "intervalSeconds")]
    pub interval_seconds: f64,
    #[serde(rename = "size")]
    pub size: i64,
}

/// Go: `level.Cycle`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Cycle {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "dayMinutes")]
    pub day_minutes: f64,
    #[serde(rename = "twilightMinutes")]
    pub twilight_minutes: f64,
    #[serde(rename = "nightMinutes")]
    pub night_minutes: f64,
    #[serde(rename = "dawnMinutes")]
    pub dawn_minutes: f64,
    #[serde(rename = "percentPerMinute")]
    pub percent_per_minute: f64,
    #[serde(rename = "percentPerGather")]
    pub percent_per_gather: f64,
    #[serde(rename = "percentPerKill")]
    pub percent_per_kill: f64,
}

/// Go: `struct { Count int "json:\"count\""; Kind string "json:\"kind\"" }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BiomeVeins {
    #[serde(rename = "count")]
    pub count: i64,
    #[serde(rename = "kind")]
    pub kind: String,
}

/// Go: `struct { DamagePerSecond float64 "json:\"damagePerSecond\""; WidthUnits float64 "json:\"widthUnits\"" }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BiomeLava {
    #[serde(rename = "damagePerSecond")]
    pub damage_per_second: f64,
    #[serde(rename = "widthUnits")]
    pub width_units: f64,
}

/// Go: `level.Trait`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Trait {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "min")]
    pub min: i64,
    #[serde(rename = "max")]
    pub max: i64,
    #[serde(rename = "default")]
    pub default: i64,
    #[serde(rename = "scales", skip_serializing_if = "Vec::is_empty")]
    pub scales: Vec<String>,
    #[serde(rename = "chunks", skip_serializing_if = "Vec::is_empty")]
    pub chunks: Vec<String>,
    #[serde(rename = "display", skip_serializing_if = "crate::omit::is_false")]
    pub display: bool,
}

/// Go: `level.WeightGroup`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WeightGroup {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "options")]
    pub options: Ordered<Range>,
}

/// Go: `struct { Portal []string "json:\"portal\""; Night []string "json:\"night\"" }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BiomeEnemies {
    #[serde(rename = "portal")]
    pub portal: Vec<String>,
    #[serde(rename = "night")]
    pub night: Vec<String>,
}

/// Größtes Gewicht einer Variante (engine/level `MaxWeight`).
pub const MAX_WEIGHT: i64 = 5;

/// `load` liest ein Biom wie `LoadBiome` (id ohne `.json`) und prüft Zyklus, Natur und Portale.
pub fn load(id: &str, raw: &str) -> Result<Biome, Fehler> {
    let prefix = format!("biomes/{id}.json");
    let b: Biome = decode(&prefix, raw)?;
    let checks = [b.cycle.check(), b.check_nature(), b.check_portals()];
    for r in checks {
        r.map_err(|e| Fehler::Pruefung(format!("{prefix}: {e}")))?;
    }
    Ok(b)
}

impl Cycle {
    /// `check` lehnt einen Tag/Nacht-Zyklus ab, dem eine Dauer fehlt oder dessen Dauer nicht positiv ist.
    pub fn check(&self) -> Result<(), String> {
        if self.r#type != "dayNight" {
            return Ok(());
        }
        for (name, minutes) in [
            ("dayMinutes", self.day_minutes),
            ("twilightMinutes", self.twilight_minutes),
            ("nightMinutes", self.night_minutes),
            ("dawnMinutes", self.dawn_minutes),
        ] {
            #[allow(clippy::neg_cmp_op_on_partial_ord)] // wie Go: NaN fällt durch
            if !(minutes > 0.0) {
                return Err(format!(
                    "cycle.{name} fehlt oder ist nicht positiv ({})",
                    k3c_det::golden::go_float(minutes)
                ));
            }
        }
        Ok(())
    }
}

impl Biome {
    /// `check_portals` verlangt Portal-HP und Verteidiger-Takt größer 0.
    fn check_portals(&self) -> Result<(), String> {
        if self.portals.hp <= 0.0 || self.portals.defense.interval_seconds <= 0.0 {
            return Err("portals.hp und portals.defense.intervalSeconds müssen größer 0 sein".into());
        }
        Ok(())
    }

    fn check_nature(&self) -> Result<(), String> {
        let mut used: Vec<(String, &str)> = Vec::new();
        for (i, e) in self.traits.0.iter().enumerate() {
            if e.key.is_empty() || self.traits.0[..i].iter().any(|o| o.key == e.key) {
                return Err(format!("traits: Schlüssel {:?} leer oder doppelt", e.key));
            }
            self.check_trait(&e.value)
                .map_err(|err| format!("traits.{}: {err}", e.key))?;
            let res = e.value.scales.iter().map(|k| format!("res:{k}"));
            let chunks = e.value.chunks.iter().map(|k| format!("chunk:{k}"));
            for k in res.chain(chunks) {
                if let Some((_, other)) = used.iter().find(|(u, _)| *u == k) {
                    return Err(format!("traits.{}: {k} wirkt schon über traits.{other}", e.key));
                }
                used.push((k, e.key.as_str()));
            }
        }
        self.check_weights()
    }

    fn check_trait(&self, t: &Trait) -> Result<(), String> {
        let effects = [!t.scales.is_empty(), !t.chunks.is_empty(), t.display]
            .iter()
            .filter(|h| **h)
            .count();
        if t.name.is_empty() {
            return Err("name fehlt".into());
        }
        if !(t.min >= 0 && t.min <= t.default && t.default <= t.max && t.default > 0) {
            return Err(format!(
                "braucht 0 ≤ min ≤ default ≤ max und default > 0 ({}, {}, {})",
                t.min, t.default, t.max
            ));
        }
        if effects != 1 {
            return Err("braucht genau eine Wirkung: scales, chunks oder display".into());
        }
        self.check_targets(t)
    }

    /// `check_targets`: Jede skalierte Art steht in resourcesPerChunk oder ist die Adern-Art, jede Chunk-Art in
    /// chunkWeights.
    fn check_targets(&self, t: &Trait) -> Result<(), String> {
        for kind in &t.scales {
            if !self.has_resource(kind) && (self.veins.count == 0 || *kind != self.veins.kind) {
                return Err(format!(
                    "{kind:?} steht in keinem resourcesPerChunk und ist nicht die Adern-Art"
                ));
            }
        }
        for kind in &t.chunks {
            if self.chunk_weights.get(kind).is_none() {
                return Err(format!("chunk-Art {kind:?} steht nicht in chunkWeights"));
            }
        }
        Ok(())
    }

    /// `check_weights` prüft die Gewichts-Gruppen: Name, Art, je Art höchstens eine Gruppe, Gewichte 0..MaxWeight
    /// und mindestens eine Variante mit Gewicht > 0.
    fn check_weights(&self) -> Result<(), String> {
        let w = &self.weights.0;
        for (i, g) in w.iter().enumerate() {
            if g.key.is_empty() || w[..i].iter().any(|o| o.key == g.key) {
                return Err(format!("weights: Gruppe {:?} leer oder doppelt", g.key));
            }
            if g.value.name.is_empty() || g.value.kind.is_empty() {
                return Err(format!("weights.{}: name oder kind fehlt", g.key));
            }
            if w[..i].iter().any(|o| o.value.kind == g.value.kind) {
                return Err(format!(
                    "weights.{}: Art {:?} hat schon eine Gruppe",
                    g.key, g.value.kind
                ));
            }
            if !self.has_resource(&g.value.kind) && g.value.kind != self.veins.kind {
                return Err(format!(
                    "weights.{}: Art {:?} kommt im Biom nicht vor",
                    g.key, g.value.kind
                ));
            }
            check_options(&g.value.options).map_err(|e| format!("weights.{}: {e}", g.key))?;
        }
        Ok(())
    }

    fn has_resource(&self, kind: &str) -> bool {
        self.resources_per_chunk.0.iter().any(|c| c.value.get(kind).is_some())
    }
}

/// `check_options` prüft die Varianten einer Gruppe: vorhanden, eindeutig, Gewichte 0..MaxWeight, eine mit max > 0.
fn check_options(opts: &Ordered<Range>) -> Result<(), String> {
    if opts.is_empty() {
        return Err("ohne Varianten".into());
    }
    let mut positive = false;
    for (j, o) in opts.0.iter().enumerate() {
        let r = &o.value;
        if o.key.is_empty() || opts.0[..j].iter().any(|p| p.key == o.key) {
            return Err(format!("variante {:?} leer oder doppelt", o.key));
        }
        if r.min < 0 || r.min > r.max || r.max > MAX_WEIGHT {
            return Err(format!(
                "{}: braucht 0 ≤ min ≤ max ≤ {MAX_WEIGHT} ({}, {})",
                o.key, r.min, r.max
            ));
        }
        positive = positive || r.max > 0;
    }
    if !positive {
        return Err("alle Gewichte 0, mindestens eine Variante braucht Gewicht > 0".into());
    }
    Ok(())
}

/// `check_pool` wie `checkPool` in engine/sim: Jeder Gegner in `enemies.portal` und `enemies.night` steht in
/// enemies.json, der Portal-Pool hat mindestens einen Standard- und einen Elite-Gegner.
pub fn check_pool(b: &Biome, enemies: &BTreeMap<String, EnemyData>) -> Result<(), Fehler> {
    let fail = |s: String| Err(Fehler::Pruefung(format!("data/biomes/{}.json: {s}", b.id)));
    let mut tiers: BTreeMap<&str, bool> = BTreeMap::new();
    let portal = b.enemies.portal.len();
    for (i, k) in b.enemies.portal.iter().chain(&b.enemies.night).enumerate() {
        let Some(d) = enemies.get(k) else {
            return fail(format!("unbekannter Gegner {k:?}"));
        };
        let t = tiers.entry(d.tier.as_str()).or_default();
        *t = *t || i < portal;
    }
    if !tiers.get("standard").copied().unwrap_or(false) || !tiers.get("elite").copied().unwrap_or(false) {
        return fail("Portal-Pool braucht Standard- und Elite-Gegner".into());
    }
    Ok(())
}
