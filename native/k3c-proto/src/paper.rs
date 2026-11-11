//! Papier-Sheets lesen (Entscheidung 005): `public/grafik/<gruppe>/<asset>/manifest.json` und die PNG-Streifen daneben.
//! Jede Animation ist ein einzeiliger Streifen aus `frames` Bildern der Größe `frameW` × `frameH`.

use bevy::asset::RenderAssetUsages;
use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Clone)]
pub struct Anim {
    pub frames: u32,
    pub fps: f32,
    pub datei: String,
    pub rahmen: Option<[f32; 4]>,
    #[serde(rename = "frameW")]
    pub frame_w: Option<u32>,
    #[serde(rename = "frameH")]
    pub frame_h: Option<u32>,
}

#[derive(Deserialize, Clone)]
pub struct Manifest {
    pub rahmen: [f32; 4],
    #[serde(rename = "frameW")]
    pub frame_w: u32,
    #[serde(rename = "frameH")]
    pub frame_h: u32,
    pub animationen: HashMap<String, Anim>,
}

/// Ein geladenes Sheet: Bild, Raster und Fußpunkt.
#[derive(Clone)]
pub struct Sheet {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub frames: u32,
    pub fps: f32,
    /// Anker für Bevy (Mitte = 0/0, y nach oben), aus dem Fußpunkt im Rahmen
    pub anchor: Vec2,
    /// Rahmen-Einheiten je Bild-Pixel (für den Maßstab: Spielhöhe / Rahmenhöhe des Assets)
    pub units_per_px: f32,
    pub bytes_rgba: u64,
}

/// Alle Manifeste und die schon angeforderten Sheets.
#[derive(Resource)]
pub struct Paper {
    pub root: PathBuf,
    manifests: HashMap<String, Manifest>,
    sheets: HashMap<String, Sheet>,
    pub handles: Vec<Handle<Image>>,
}

impl Paper {
    pub fn new(root: PathBuf) -> Self {
        Self { root, manifests: HashMap::new(), sheets: HashMap::new(), handles: Vec::new() }
    }

    /// Manifest eines Assets `gruppe/id`; liest es beim ersten Zugriff.
    pub fn manifest(&mut self, asset: &str) -> Manifest {
        if let Some(m) = self.manifests.get(asset) {
            return m.clone();
        }
        let path = self.root.join(asset).join("manifest.json");
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("💥 Manifest fehlt: {} ({e}). Erst `task gen:paper` ausführen.", path.display()));
        let m: Manifest = serde_json::from_str(&text).unwrap_or_else(|e| panic!("💥 Manifest kaputt: {} ({e})", path.display()));
        self.manifests.insert(asset.to_string(), m.clone());
        m
    }

    /// Sheet `gruppe/id` + Animation; lädt das Bild nur für die GPU (keine Kopie im Hauptspeicher).
    pub fn sheet(&mut self, asset: &str, anim: &str, server: &AssetServer, layouts: &mut Assets<TextureAtlasLayout>) -> Sheet {
        let key = format!("{asset}:{anim}");
        if let Some(s) = self.sheets.get(&key) {
            return s.clone();
        }
        let m = self.manifest(asset);
        let a = m.animationen.get(anim).unwrap_or_else(|| panic!("💥 Animation {anim} fehlt in {asset}")).clone();
        let (fw, fh) = (a.frame_w.unwrap_or(m.frame_w), a.frame_h.unwrap_or(m.frame_h));
        let r = a.rahmen.unwrap_or(m.rahmen);
        let rel = format!("{asset}/{}", a.datei);
        let image = load_gpu_only(server, rel.clone());
        self.handles.push(image.clone());
        let layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::new(fw, fh), a.frames, 1, None, None));
        let (ox, oy) = (-r[0] / r[2], -r[1] / r[3]);
        let sheet = Sheet {
            image,
            layout,
            frames: a.frames,
            fps: a.fps,
            anchor: Vec2::new(ox - 0.5, 0.5 - oy),
            units_per_px: r[3] / fh as f32,
            bytes_rgba: png_bytes(&self.root.join(&rel)),
        };
        self.sheets.insert(key, sheet.clone());
        sheet
    }

    pub fn texture_bytes(&self) -> u64 {
        self.sheets.values().map(|s| s.bytes_rgba).sum()
    }

    pub fn sheet_count(&self) -> usize {
        self.sheets.len()
    }

    /// Alle Assets aller Gruppen (für den Lastfall `alle`), sortiert für eine feste Reihenfolge.
    pub fn all_assets(&self) -> Vec<String> {
        let mut out = Vec::new();
        for group in sorted_dirs(&self.root) {
            for asset in sorted_dirs(&self.root.join(&group)) {
                if self.root.join(&group).join(&asset).join("manifest.json").exists() {
                    out.push(format!("{group}/{asset}"));
                }
            }
        }
        out
    }
}

fn load_gpu_only(server: &AssetServer, rel: String) -> Handle<Image> {
    server
        .load_builder()
        .with_settings(|s: &mut ImageLoaderSettings| {
            s.asset_usage = RenderAssetUsages::RENDER_WORLD;
        })
        .load(rel)
}

fn sorted_dirs(dir: &Path) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(dir)
        .map(|it| it.filter_map(|e| e.ok()).filter(|e| e.path().is_dir()).map(|e| e.file_name().to_string_lossy().into_owned()).collect())
        .unwrap_or_default();
    v.sort();
    v
}

/// Speicher eines Bildes als RGBA8 aus dem PNG-Kopf (Breite × Höhe × 4).
fn png_bytes(path: &Path) -> u64 {
    use std::io::Read;
    let mut b = [0u8; 24];
    let Ok(mut f) = std::fs::File::open(path) else { return 0 };
    if f.read_exact(&mut b).is_err() {
        return 0;
    }
    let w = u32::from_be_bytes([b[16], b[17], b[18], b[19]]) as u64;
    let h = u32::from_be_bytes([b[20], b[21], b[22], b[23]]) as u64;
    w * h * 4
}
