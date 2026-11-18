//! Natur-Werte der Insel (Port von `engine/sim/island_nature.go`, B-891, docs/rules/welten.md § 2.1): `nature` ist die
//! Vorgabe der Insel für alle Stufen, deren Biom die Eigenschaft hat; `natureByDepth` setzt je Tiefe fest und gewinnt. Was
//! keine Stufe festsetzt, würfelt der Level-Generator aus dem Seed (`resolve_nature`).
//!
//! Maps laufen wie Go in Schlüsselreihenfolge (`BTreeMap` ist sortiert).

use k3c_data::biome::Biome;
use k3c_level::nature::merge_nature;
use k3c_level::nature_check::check_nature_values;
use k3c_level::types::NatureValues;

use super::create::CreateOptions;
use super::data::{IslandDef, biome_for_depth, depth_key};

impl IslandDef {
    /// Go: `NatureAt` liefert die fest gesetzten Natur-Werte der Tiefe: Vorgabe der Insel ohne die Schlüssel, die das Biom
    /// nicht kennt, darüber die Werte der Tiefe. Eine unbekannte Tiefe ergibt leere Werte (Go bricht mit `panic` ab; die
    /// Prüfung der Insel lehnt sie vorher ab).
    pub fn nature_at(&self, biomes: &[Biome], depth: i64) -> NatureValues {
        let Some(b) = biome_for_depth(biomes, depth) else {
            return NatureValues::default();
        };
        let mut base = NatureValues::default();
        for (k, &v) in &self.nature.traits {
            if b.traits.get(k).is_some() {
                base.set_trait(k, v);
            }
        }
        for (gk, options) in &self.nature.weights {
            let group = b.weights.get(gk);
            for (k, &w) in options {
                if group.is_some_and(|g| g.options.get(k).is_some()) {
                    base.set_weight(gk, k, w);
                }
            }
        }
        let empty = NatureValues::default();
        merge_nature(&base, self.nature_by_depth.get(&depth.to_string()).unwrap_or(&empty))
    }
}

/// Go: `checkNature` prüft die Natur-Werte der Insel: jeder Schlüssel der Vorgabe gilt in mindestens einem Biom der Insel
/// und liegt dort im Bereich; je Tiefe nur bekannte Tiefen und Schlüssel ihres Bioms; die zusammengelegten Werte setzen
/// nicht alle Baumarten auf 0.
pub fn check_nature(d: &IslandDef, biomes: &[Biome]) -> Result<(), String> {
    let insel_biome = |depth: &i64| biome_for_depth(biomes, *depth);
    for k in d.nature.traits.keys() {
        if !d
            .depths
            .iter()
            .any(|depth| insel_biome(depth).is_some_and(|b| b.traits.get(k).is_some()))
        {
            return Err(format!("nature: kein Biom der Insel hat die Eigenschaft {k:?}"));
        }
    }
    for (gk, options) in &d.nature.weights {
        for k in options.keys() {
            let known = d.depths.iter().any(|depth| {
                insel_biome(depth)
                    .and_then(|b| b.weights.get(gk))
                    .is_some_and(|g| g.options.get(k).is_some())
            });
            if !known {
                return Err(format!("nature: kein Biom der Insel hat die Variante {gk}.{k}"));
            }
        }
    }
    for (key, values) in &d.nature_by_depth {
        let Some(depth) = depth_key(key, &d.depths) else {
            return Err(format!("natureByDepth: Tiefe {key:?} ist keine Tiefe der Insel"));
        };
        let Some(b) = biome_for_depth(biomes, depth) else {
            return Err(format!("unbekannte Tiefe {depth}"));
        };
        check_nature_values(b, values, true).map_err(|e| format!("natureByDepth Tiefe {depth}: {e}"))?;
    }
    for &depth in &d.depths {
        let Some(b) = biome_for_depth(biomes, depth) else {
            return Err(format!("unbekannte Tiefe {depth}"));
        };
        check_nature_values(b, &d.nature_at(biomes, depth), false).map_err(|e| format!("nature Tiefe {depth}: {e}"))?;
    }
    Ok(())
}

impl CreateOptions {
    /// Go: `natureFor` liefert die fest gesetzten Natur-Werte der Tiefe; ohne `nature` die der Insel `def`.
    pub fn nature_for(&self, depth: i64, def: &IslandDef, biomes: &[Biome]) -> NatureValues {
        match &self.nature {
            Some(m) => m.get(&depth).cloned().unwrap_or_default(),
            None => def.nature_at(biomes, depth),
        }
    }
}
