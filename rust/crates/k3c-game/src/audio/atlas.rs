//! Port von `native/k3c-game/audio/atlas.go`: Ton-Atlas (public/audio/atlas.json, `task gen:soundatlas`).

use std::collections::BTreeMap;

use serde::Deserialize;

/// Ausschnitt des Ton-Atlas in Sekunden.
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Sprite {
    pub start: f64,
    pub duration: f64,
}

/// Der Ton-Atlas: eine Datei je Format und Start und Dauer je Effekt-Slot (src/audio/atlas.ts).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Atlas {
    /// Format (ogg, mp3, wav) → Dateiname neben atlas.json
    pub files: BTreeMap<String, String>,
    /// Effekt-Slot → Ausschnitt
    pub sprites: BTreeMap<String, Sprite>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct AtlasFile {
    files: BTreeMap<String, String>,
    sprites: Option<BTreeMap<String, Sprite>>,
}

/// Reihenfolge der Formate wie im Browser (atlas.ts › FORMAT_ORDER).
pub const FORMAT_ORDER: [&str; 3] = ["ogg", "mp3", "wav"];

/// Größe eines Abtastwerts im dekodierten Ton: 16 Bit, Stereo.
pub const PCM_FRAME: usize = 4;

/// Prüft die JSON-Beschreibung wie atlas.ts › parseAtlas: ungültige Sprites (negativer Start, Dauer ≤ 0, keine Zahl)
/// fallen weg; ohne Dateien oder Sprites ist es ein Fehler.
pub fn parse_atlas(b: &[u8]) -> Result<Atlas, String> {
    let raw: AtlasFile = serde_json::from_slice(b).map_err(|e| format!("❌ Ton-Atlas: {e}"))?;
    let sprites = match raw.sprites {
        Some(s) if !raw.files.is_empty() => s,
        _ => return Err("❌ Ton-Atlas ohne Dateien oder Sprites".to_string()),
    };
    let sprites = sprites
        .into_iter()
        .filter(|(_, s)| finite(s.start) && finite(s.duration) && s.start >= 0.0 && s.duration > 0.0)
        .collect();
    Ok(Atlas {
        files: raw.files,
        sprites,
    })
}

fn finite(v: f64) -> bool {
    v.is_finite()
}

impl Atlas {
    /// Die Datei des ersten Formats aus `FORMAT_ORDER`, die der Atlas nennt und die es gibt (`exists`).
    pub fn pick(&self, exists: impl Fn(&str) -> bool) -> Option<String> {
        FORMAT_ORDER
            .iter()
            .filter_map(|f| self.files.get(*f))
            .find(|name| !name.is_empty() && exists(name))
            .cloned()
    }
}

/// Byte-Bereich [lo, hi) eines Sprites in dekodiertem Ton mit `rate` Hz und `total` Bytes, auf ganze Abtastwerte
/// gerundet und auf den Ton begrenzt. Ein leerer Bereich (lo == hi) heißt: liegt außerhalb.
pub fn sprite_range(s: &Sprite, rate: usize, total: usize) -> (usize, usize) {
    let frames = (total / PCM_FRAME) as i64;
    let at = |sec: f64| (sec * rate as f64).round() as i64;
    let from = frames.min(at(s.start).max(0));
    let to = frames.min(from.max(at(s.start + s.duration)));
    (from as usize * PCM_FRAME, to as usize * PCM_FRAME)
}
