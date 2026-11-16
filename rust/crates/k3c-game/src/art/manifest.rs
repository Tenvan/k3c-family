//! Papier-Manifeste und Gruppen-Indizes lesen (Entscheidung 005); Vorlage `native/k3c-game/art/manifest.go`.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use super::ArtError;

/// Animation im Manifest; `rahmen`, `frame_w` und `frame_h` überschreiben optional die Werte des Assets.
#[derive(Debug, Clone, Deserialize)]
pub struct Anim {
    pub frames: u32,
    #[serde(default)]
    pub fps: f64,
    pub datei: String,
    #[serde(default)]
    pub rahmen: Option<[f64; 4]>,
    #[serde(default, rename = "frameW")]
    pub frame_w: Option<u32>,
    #[serde(default, rename = "frameH")]
    pub frame_h: Option<u32>,
}

/// Asset-Manifest: Rahmen (x, y, Breite, Höhe in Einheiten; Fußpunkt = -x/Breite, -y/Höhe) und Animationen.
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub rahmen: [f64; 4],
    #[serde(rename = "frameW")]
    pub frame_w: u32,
    #[serde(rename = "frameH")]
    pub frame_h: u32,
    #[serde(default)]
    pub animationen: BTreeMap<String, Anim>,
}

impl Manifest {
    /// Frame-Größe und Rahmen einer Animation (Animation vor Asset).
    pub fn resolve(&self, a: &Anim) -> (u32, u32, [f64; 4]) {
        (
            a.frame_w.unwrap_or(self.frame_w),
            a.frame_h.unwrap_or(self.frame_h),
            a.rahmen.unwrap_or(self.rahmen),
        )
    }
}

#[derive(Deserialize)]
struct IndexEntry {
    id: String,
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path, was: &str) -> Result<T, ArtError> {
    let text = std::fs::read_to_string(path).map_err(|e| {
        ArtError::Missing(format!(
            "{was} {} ({e}); erst `task gen:paper` ausführen",
            path.display()
        ))
    })?;
    serde_json::from_str(&text).map_err(|e| ArtError::Broken(format!("{}: {was} kaputt ({e})", path.display())))
}

/// Liest das Manifest von Asset `gruppe/id` unter dem Grafik-Ordner.
pub fn read_manifest(dir: &Path, asset: &str) -> Result<Manifest, ArtError> {
    read_json(&dir.join(asset).join("manifest.json"), "Manifest")
}

/// Nennt die Assets einer Gruppe (`gruppe/id`) aus `<dir>/<gruppe>/index.paper.json`.
pub fn group_assets(dir: &Path, group: &str) -> Result<Vec<String>, ArtError> {
    let entries: Vec<IndexEntry> = read_json(&dir.join(group).join("index.paper.json"), "Gruppen-Index")?;
    Ok(entries.into_iter().map(|e| format!("{group}/{}", e.id)).collect())
}
