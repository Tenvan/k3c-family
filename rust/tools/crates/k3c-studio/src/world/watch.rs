//! Polling-Wächter für `world:changed` (Port von `watch.go`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::{Duration, UNIX_EPOCH};

use serde::Serialize;

use super::WORLDS_DIR;

/// Nutzlast des Ereignisses `world:changed` (B-1216): IDs der geänderten Welten. Ohne `ids` hat sich eine Basis-Datei
/// unter `data/` geändert oder die Lage ist unklar: die Oberfläche lädt dann jede Welt neu.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Changed {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Vec<String>,
}

/// Schlüssel eines Snapshots für Dateien außerhalb von `data/worlds/`.
const BASE: char = '*';

/// Stempel (Größe und Änderungszeit) je Datei: `<id>` für `data/worlds/<id>.json`, sonst `*<pfad>` für die
/// Basis-Dateien `data/*.json` und `data/biomes/*.json`. Gelesen werden nur Metadaten.
pub fn snapshot(root: &Path) -> BTreeMap<String, String> {
    let mut snap = BTreeMap::new();
    for dir in [WORLDS_DIR, "data", "data/biomes"] {
        let Ok(entries) = std::fs::read_dir(root.join(dir)) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !name.ends_with(".json") {
                continue;
            }
            let Ok(meta) = entry.metadata() else { continue };
            if meta.is_dir() {
                continue;
            }
            let modified = meta.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok());
            let stamp = format!("{}|{}", meta.len(), modified.unwrap_or_default().as_nanos());
            let key = if dir == WORLDS_DIR {
                name.trim_end_matches(".json").to_owned()
            } else {
                format!("{BASE}{dir}/{name}")
            };
            snap.insert(key, stamp);
        }
    }
    snap
}

/// Die Welt-IDs, deren Stempel sich geändert hat (neu, gelöscht, anders), sortiert. Hat sich eine Basis-Datei geändert,
/// ist die Liste leer und das zweite Ergebnis wahr.
pub fn changed_ids(old: &BTreeMap<String, String>, current: &BTreeMap<String, String>) -> (Vec<String>, bool) {
    let mut seen = BTreeSet::new();
    let mut all = false;
    for (a, b) in [(current, old), (old, current)] {
        for (key, stamp) in a {
            if b.get(key) == Some(stamp) {
                continue;
            }
            if key.starts_with(BASE) {
                all = true;
            } else {
                seen.insert(key.clone());
            }
        }
    }
    if all {
        return (Vec::new(), true);
    }
    (seen.into_iter().collect(), false)
}

/// Laufender Wächter; er endet, wenn der Wert fallen gelassen wird.
#[must_use = "der Wächter endet beim Fallenlassen"]
pub struct Watcher {
    _stop: mpsc::Sender<()>,
}

/// Fragt alle `every` die Dateien ab und ruft `on_change` mit den geänderten IDs (leer: alle). Polling statt
/// Dateisystem-Ereignissen (keine neue Abhängigkeit, ein paar Stat-Aufrufe).
pub fn watch(root: impl Into<PathBuf>, every: Duration, on_change: impl Fn(Vec<String>) + Send + 'static) -> Watcher {
    let root = root.into();
    let (stop, stopped) = mpsc::channel::<()>();
    let mut last = snapshot(&root);
    std::thread::spawn(move || {
        while stopped.recv_timeout(every) == Err(RecvTimeoutError::Timeout) {
            let current = snapshot(&root);
            let (ids, all) = changed_ids(&last, &current);
            if all || !ids.is_empty() {
                last = current;
                on_change(ids);
            }
        }
    });
    Watcher { _stop: stop }
}

#[cfg(test)]
#[path = "watch_tests.rs"]
mod tests;
