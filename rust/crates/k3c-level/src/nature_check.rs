//! Prüfung fest gesetzter Natur-Werte (Port von `engine/level/nature_check.go`, ab `CheckNatureValues`). Die
//! Datenprüfung des Bioms (`checkNature`, `checkTrait`, `checkWeights`, `checkOptions`, `prefixed`) liegt seit RU3 in
//! `k3c_data::biome` und läuft beim Laden (`load_biome`); sie wird hier nicht nachgebaut.

use std::collections::BTreeMap;

use crate::biome::{Biome, WeightGroup};
use crate::types::NatureValues;

/// Go: `CheckNatureValues`, prüft fest gesetzte Werte gegen das Biom: Wert im Bereich, unbekannte Schlüssel nur mit
/// `strict` als Fehler (die Vorgabe einer Insel gilt nur in den Biomen, die die Eigenschaft haben), und keine Gruppe mit
/// allen Varianten fest auf 0.
pub fn check_nature_values(b: &Biome, v: &NatureValues, strict: bool) -> Result<(), String> {
    for (k, &val) in &v.traits {
        match b.traits.get(k) {
            None if strict => return Err(format!("biom {} hat keine Eigenschaft {k:?}", b.id)),
            Some(t) if val < t.min || val > t.max => {
                return Err(format!(
                    "{k} = {val} liegt außerhalb {}–{} (Biom {})",
                    t.min, t.max, b.id
                ));
            }
            _ => {}
        }
    }
    for (gk, fixed) in &v.weights {
        match b.weights.get(gk) {
            None if strict => return Err(format!("biom {} hat keine Gewichts-Gruppe {gk:?}", b.id)),
            None => {}
            Some(g) => check_group_values(&b.id, gk, g, fixed, strict)?,
        }
    }
    Ok(())
}

fn check_group_values(
    biome: &str,
    gk: &str,
    g: &WeightGroup,
    fixed: &BTreeMap<String, i64>,
    strict: bool,
) -> Result<(), String> {
    for (k, &w) in fixed {
        match g.options.get(k) {
            None if strict => return Err(format!("biom {biome} hat in {gk} keine Variante {k:?}")),
            Some(r) if w < r.min || w > r.max => {
                return Err(format!(
                    "{gk}.{k} = {w} liegt außerhalb {}–{} (Biom {biome})",
                    r.min, r.max
                ));
            }
            _ => {}
        }
    }
    let any_weight = g.options.0.iter().any(|e| {
        let w = fixed.get(&e.key);
        (w.is_none() && e.value.max > 0) || w.is_some_and(|&w| w > 0)
    });
    if !any_weight {
        return Err(format!("{gk}: alle Varianten fest auf Gewicht 0 (Biom {biome})"));
    }
    Ok(())
}
