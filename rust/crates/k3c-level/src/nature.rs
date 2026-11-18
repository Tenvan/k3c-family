//! Natur-Eigenschaften der Biome (Port von `engine/level/nature.go`, B-891, docs/rules/welten.md § 2.1): Je Stufe gilt
//! ein Wert, fest gesetzt in der Welt-Datei oder aus dem Seed gewürfelt. Die Würfe kommen aus eigenen Strömen
//! `<biom>:<seed>:nature` und `<biom>:<seed>:variant:<gruppe>:<x>`; der Hauptstrom des Generators bleibt unberührt.
//! Geplante Chunks würfeln ihren Inhalt ohne Natur-Faktoren (Chunk-Seed = gleicher Inhalt).
//!
//! `Trait`, `WeightGroup`, `MAX_WEIGHT` und die Datenprüfung kommen aus `k3c_data::biome`. Gos Methoden an `Biome`
//! (`withNature`, `traitOf`, `CheckNatureValues`) sind freie Funktionen, weil `Biome` in `k3c-data` liegt.

use std::fmt::Write;

use k3c_det::golden::go_float;
use k3c_rng::{Rng, Weight};

use crate::biome::{Biome, Trait};
use crate::nature_check::check_nature_values;
use crate::types::{Entity, NatureValues};

impl NatureValues {
    /// Go: `SetTrait`, setzt einen Eigenschafts-Wert.
    pub fn set_trait(&mut self, key: &str, val: i64) {
        self.traits.insert(key.to_string(), val);
    }

    /// Go: `SetWeight`, setzt das Gewicht einer Variante.
    pub fn set_weight(&mut self, group: &str, option: &str, val: i64) {
        self.weights
            .entry(group.to_string())
            .or_default()
            .insert(option.to_string(), val);
    }
}

/// Go: `MergeNature`, legt `over` über `base` (`over` gewinnt je Schlüssel); die Eingaben bleiben unverändert.
pub fn merge_nature(base: &NatureValues, over: &NatureValues) -> NatureValues {
    let mut out = NatureValues {
        traits: base.traits.clone(),
        ..NatureValues::default()
    };
    for v in [base, over] {
        for (g, options) in &v.weights {
            for (k, &w) in options {
                out.set_weight(g, k, w);
            }
        }
    }
    for (k, &val) in &over.traits {
        out.set_trait(k, val);
    }
    out
}

/// Go: `NeutralNature`, die neutralen Werte des Bioms: jede Eigenschaft auf Default, jede Variante auf ihr größtes
/// Gewicht.
pub fn neutral_nature(b: &Biome) -> NatureValues {
    let mut out = NatureValues::default();
    for e in &b.traits.0 {
        out.set_trait(&e.key, e.value.default);
    }
    for g in &b.weights.0 {
        for o in &g.value.options.0 {
            out.set_weight(&g.key, &o.key, o.value.max);
        }
    }
    out
}

/// Go: `ResolveNature`, liefert die geltenden Werte der Stufe: je Eigenschaft und Variante ein Wurf in min..max aus
/// dem Strom `<biom>:<seed>:nature` (immer, auch bei festem Wert, damit ein fester Wert die übrigen nicht verschiebt),
/// dann der feste Wert aus `fixed`. Ergibt eine Gruppe zusammen 0, bekommt ihre erste gewürfelte Variante mit max > 0
/// das Gewicht 1.
pub fn resolve_nature(b: &Biome, seed: &str, fixed: &NatureValues) -> Result<NatureValues, String> {
    check_nature_values(b, fixed, true)?;
    if b.traits.is_empty() && b.weights.is_empty() {
        return Ok(NatureValues::default());
    }
    let mut r = Rng::new(&format!("{}:{seed}:nature", b.id));
    let mut out = NatureValues::default();
    for e in &b.traits.0 {
        let rolled = r.int(e.value.min, e.value.max);
        out.set_trait(&e.key, fixed.traits.get(&e.key).copied().unwrap_or(rolled));
    }
    for g in &b.weights.0 {
        let fixed_group = fixed.weights.get(&g.key);
        let mut sum = 0;
        for o in &g.value.options.0 {
            let rolled = r.int(o.value.min, o.value.max);
            let w = fixed_group.and_then(|m| m.get(&o.key)).copied().unwrap_or(rolled);
            out.set_weight(&g.key, &o.key, w);
            sum += w;
        }
        for o in &g.value.options.0 {
            let is_fixed = fixed_group.is_some_and(|m| m.contains_key(&o.key));
            if sum == 0 && !is_fixed && o.value.max > 0 {
                out.set_weight(&g.key, &o.key, 1);
                break;
            }
        }
    }
    Ok(out)
}

/// Go: `traitValue`, der Wert der Eigenschaft in `v`, ohne Eintrag ihr Default.
fn trait_value(v: &NatureValues, key: &str, t: &Trait) -> i64 {
    v.traits.get(key).copied().unwrap_or(t.default)
}

/// Go: `Biome.traitOf`, die Eigenschaft, die `kind` in der Liste `of` nennt.
fn trait_of<'a>(b: &'a Biome, kind: &str, of: fn(&Trait) -> &[String]) -> Option<(&'a str, &'a Trait)> {
    b.traits
        .0
        .iter()
        .find(|e| of(&e.value).iter().any(|k| k == kind))
        .map(|e| (e.key.as_str(), &e.value))
}

fn scales(t: &Trait) -> &[String] {
    &t.scales
}

fn chunks(t: &Trait) -> &[String] {
    &t.chunks
}

/// Go: `Biome.withNature`, das Biom nach den Werten `v` (Kopie): Bereiche in resourcesPerChunk und die Zahl der Adern
/// skaliert, Chunk-Gewichte skaliert. Default ergibt dieselben Zahlen und damit dieselben Würfe wie ohne Eigenschaft.
/// Würden alle Chunk-Gewichte 0, bleiben die des Bioms.
pub fn with_nature(b: &Biome, v: &NatureValues) -> Biome {
    let mut out = b.clone();
    if b.traits.is_empty() {
        return out;
    }
    for c in &mut out.resources_per_chunk.0 {
        for k in &mut c.value.0 {
            if let Some((key, t)) = trait_of(b, &k.key, scales) {
                let val = trait_value(v, key, t);
                k.value = [
                    scale_count(k.value[0], val, t.default),
                    scale_count(k.value[1], val, t.default),
                ];
            }
        }
    }
    if b.veins.count > 0
        && let Some((key, t)) = trait_of(b, &b.veins.kind, scales)
    {
        out.veins.count = scale_count(b.veins.count, trait_value(v, key, t), t.default);
    }
    let mut weights = b.chunk_weights.clone();
    let mut sum = 0.0;
    for c in &mut weights.0 {
        if let Some((key, t)) = trait_of(b, &c.key, chunks) {
            let val = trait_value(v, key, t);
            if val != t.default {
                c.value = c.value * val as f64 / t.default as f64;
            }
        }
        sum += c.value;
    }
    if sum > 0.0 {
        out.chunk_weights = weights;
    }
    out
}

/// Go: `scaleCount`, n × v / def, kaufmännisch gerundet, in ganzen Zahlen (kein Gleitkomma, deterministisch).
pub(crate) fn scale_count(n: i64, v: i64, def: i64) -> i64 {
    (2 * n * v + def) / (2 * def)
}

/// Go: `assignVariants`, gibt jeder Entität der Art einer Gewichts-Gruppe eine Variante nach den Gewichten aus `v`,
/// gewürfelt je Entität im Strom `<biom>:<seed>:variant:<gruppe>:<x>`: Die Variante hängt nur vom Ort ab, ein Baum
/// mehr oder weniger verschiebt keine andere.
pub fn assign_variants(b: &Biome, seed: &str, v: &NatureValues, es: &mut [Entity]) {
    let mut stream = String::new();
    for g in &b.weights.0 {
        let w: Vec<Weight> = g
            .value
            .options
            .0
            .iter()
            .map(|o| Weight {
                key: o.key.clone(),
                w: v.weights.get(&g.key).and_then(|m| m.get(&o.key)).copied().unwrap_or(0) as f64,
            })
            .collect();
        for e in es.iter_mut().filter(|e| e.kind == g.value.kind) {
            stream.clear();
            // PERF(B-LVL): `go_float` allokiert einen Text je Entität; läuft einmal je Level, nicht je Bild.
            // Der Strom-Name wird in einem wiederverwendeten Puffer gebaut.
            let _ = write!(stream, "{}:{seed}:variant:{}:{}", b.id, g.key, go_float(e.x));
            e.variant = Rng::new(&stream).weighted(&w);
        }
    }
}

#[cfg(test)]
#[path = "nature_tests.rs"]
mod nature_tests;
