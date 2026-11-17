//! Papier-Manifeste lesen (Entscheidung 005): <grafik>/<gruppe>/<asset>/manifest.json, daneben je Animation ein
//! einzeiliger PNG-Streifen aus `frames` Bildern der Größe frameW × frameH. Reine Datei-Logik, keine GPU.
//! Vorlage: `native/k3c-game/art/manifest.go`.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use super::ArtError;

/// Animation im Manifest; Rahmen, frameW und frameH überschreiben optional die Werte des Assets.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Anim {
    #[serde(rename = "frames")]
    pub frames: i64,
    #[serde(rename = "fps")]
    pub fps: f64,
    #[serde(rename = "datei")]
    pub file: String,
    #[serde(rename = "rahmen")]
    pub frame: Option<[f64; 4]>,
    #[serde(rename = "frameW")]
    pub frame_w: Option<i64>,
    #[serde(rename = "frameH")]
    pub frame_h: Option<i64>,
}

/// Manifest beschreibt ein Asset: Rahmen (x, y, Breite, Höhe in Einheiten, Fußpunkt = -x/Breite, -y/Höhe) und Animationen.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Manifest {
    #[serde(rename = "rahmen")]
    pub frame: [f64; 4],
    #[serde(rename = "frameW")]
    pub frame_w: i64,
    #[serde(rename = "frameH")]
    pub frame_h: i64,
    #[serde(rename = "animationen")]
    pub anims: BTreeMap<String, Anim>,
}

/// Liest das Manifest von Asset `gruppe/id` unter dem Grafik-Ordner dir.
pub fn read_manifest(dir: &Path, asset: &str) -> Result<Manifest, ArtError> {
    let path = dir.join(asset).join("manifest.json");
    let text = std::fs::read(&path).map_err(|e| {
        ArtError::Missing(format!(
            "Manifest {} ({e}); erst `task gen:paper` ausführen",
            path.display()
        ))
    })?;
    serde_json::from_slice(&text).map_err(|e| ArtError::Broken(format!("{}: Manifest kaputt ({e})", path.display())))
}

impl Manifest {
    /// Liefert die Namen der Animationen sortiert (feste Reihenfolge, damit der Atlas bei jedem Start gleich liegt).
    pub fn anim_names(&self) -> Vec<String> {
        // BTreeMap liefert die Schlüssel bereits sortiert wie `slices.Sort`.
        self.anims.keys().cloned().collect()
    }

    /// Rechnet Frame-Größe und Rahmen einer Animation aus (Animation vor Asset).
    pub fn resolve(&self, a: &Anim) -> (i64, i64, [f64; 4]) {
        (
            a.frame_w.unwrap_or(self.frame_w),
            a.frame_h.unwrap_or(self.frame_h),
            a.frame.unwrap_or(self.frame),
        )
    }
}

#[derive(Deserialize)]
struct IndexEntry {
    #[serde(default, rename = "id")]
    id: String,
}

/// Nennt die Assets einer Gruppe (`gruppe/id`) aus ihrem Index <grafik>/<gruppe>/index.paper.json.
pub fn group_assets(dir: &Path, group: &str) -> Result<Vec<String>, ArtError> {
    let path = dir.join(group).join("index.paper.json");
    let text = std::fs::read(&path).map_err(|e| {
        ArtError::Missing(format!(
            "Gruppen-Index {} ({e}); erst `task gen:paper` ausführen",
            path.display()
        ))
    })?;
    let entries: Vec<IndexEntry> = serde_json::from_slice(&text)
        .map_err(|e| ArtError::Broken(format!("{}: Gruppen-Index kaputt ({e})", path.display())))?;
    Ok(entries.into_iter().map(|e| format!("{group}/{}", e.id)).collect())
}
