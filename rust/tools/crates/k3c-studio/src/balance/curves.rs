//! Grade (B-158) für die Prüfung der Ziele; die Grad-Kurven selbst rechnet `k3c-game sim balance --curves`.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use super::datafiles;
use super::report_types::read_go;

#[derive(Default, Deserialize)]
#[serde(default)]
struct Difficulty {
    grades: BTreeMap<String, Grade>,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Grade {
    wave_size: f64,
}

/// Die Grade aus `data/difficulty.json`, nach Wellengröße sortiert (dev, easy, normal, hard, ultra).
pub fn grades(root: &Path) -> Result<Vec<String>, String> {
    let d: Difficulty = datafiles::read(root, "difficulty.json")
        .and_then(|raw| read_go(&raw))
        .map_err(|e| format!("data/difficulty.json: {e}"))?;
    let mut list: Vec<(String, f64)> = d.grades.into_iter().map(|(name, g)| (name, g.wave_size)).collect();
    list.sort_by(|a, b| a.1.total_cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    Ok(list.into_iter().map(|(name, _)| name).collect())
}
