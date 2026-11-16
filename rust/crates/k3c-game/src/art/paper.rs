//! Assets aus `data/paper.json` einsammeln: jede Zuordnung `gruppe/id[@variante]`, Reittiere, Objekte, Gebäude-Gruppe.

use std::collections::BTreeSet;
use std::path::Path;

use serde_json::Value;

use super::{ArtError, group_assets};

/// Nennt alle Assets, die die Zuordnung verwendet (sortiert, ohne Doppelte).
pub fn paper_assets(dir: &Path, paper: &Value) -> Result<Vec<String>, ArtError> {
    let mut out = BTreeSet::new();
    walk(paper, dir, &mut out);
    if let Some(m) = paper.get("reittiere").and_then(Value::as_object) {
        out.extend(m.values().filter_map(Value::as_str).map(|id| format!("reittiere/{id}")));
    }
    if let Some(m) = paper.get("objekte").and_then(Value::as_object) {
        for (id, v) in m {
            let group = v.get("gruppe").and_then(Value::as_str).unwrap_or("objekte");
            out.insert(format!("{group}/{id}"));
        }
    }
    if let Some(group) = paper.get("gebaeude").and_then(Value::as_str) {
        out.extend(group_assets(dir, group)?);
    }
    Ok(out.into_iter().collect())
}

/// Strings der Form `gruppe/id[@variante]`, deren Gruppe ein Ordner mit Index ist.
fn walk(v: &Value, dir: &Path, out: &mut BTreeSet<String>) {
    match v {
        Value::String(s) => {
            let name = s.split('@').next().unwrap_or(s);
            if let Some((group, id)) = name.split_once('/')
                && !id.contains('/')
                && dir.join(group).join("index.paper.json").is_file()
            {
                out.insert(name.to_string());
            }
        }
        Value::Array(a) => a.iter().for_each(|x| walk(x, dir, out)),
        Value::Object(o) => o.values().for_each(|x| walk(x, dir, out)),
        _ => {}
    }
}
