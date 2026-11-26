//! Was vom Asset-Studio lesend blieb (Entscheidung 005 › 5, Port von `internal/studioview`): Sound-Slots und Quellen für
//! den Bereich Assets und „Im Spiel ansehen“ (`game`). Geändert wird nichts an `data/` oder `public/`: Zuordnen,
//! Recherche und Import bleiben die MCP-Tools `assets_*`.

mod game;

#[cfg(test)]
#[path = "game_tests.rs"]
mod game_tests;
#[cfg(test)]
#[path = "studioview_tests.rs"]
mod tests;

use std::collections::BTreeMap;
use std::io::{self, Read};
use std::path::Path;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use k3c_assetkit as assets;
use serde::Serialize;

pub use game::{Game, default_saves_dir};

/// Ein Sound-Slot aus `data/sounds.json` mit Status (zugeordnet, luecke, fehlt) und seinen Dateien.
#[derive(Debug, Serialize)]
pub struct SoundEintrag {
    pub id: String,
    pub kategorie: String,
    pub beschreibung: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub bus: String,
    pub status: String,
    pub dateien: Vec<String>,
}

/// Die Sound-Slots in der Reihenfolge des Registers.
pub fn sounds(root: &Path) -> Result<Vec<SoundEintrag>, String> {
    let register = assets::load(root).map_err(|e| e.to_string())?;
    Ok(register
        .slots
        .iter()
        .filter(|s| s.kind == assets::KIND_SOUND)
        .map(|s| SoundEintrag {
            id: s.id.clone(),
            kategorie: s.category.clone(),
            beschreibung: s.description.clone(),
            bus: s.bus.clone(),
            status: assets::status(root, s).to_owned(),
            dateien: s.choices().iter().map(|c| c.file.clone()).collect(),
        })
        .collect())
}

/// Der MIME-Typ der Endungen, die [`sound_data`] ausliefert.
fn audio_type(rel: &str) -> Option<&'static str> {
    let name = rel.rsplit('/').next().unwrap_or_default();
    let (_, ext) = name.rsplit_once('.')?;
    match ext.to_lowercase().as_str() {
        "ogg" => Some("audio/ogg"),
        "mp3" => Some("audio/mpeg"),
        "wav" => Some("audio/wav"),
        "m4a" => Some("audio/mp4"),
        _ => None,
    }
}

/// Begrenzt eine Hörprobe (Data-URL über RPC, Base64 macht sie um ein Drittel größer).
const MAX_SOUND_BYTES: u64 = 10 << 20;

/// Nur relative Pfade mit `/`, ohne `..`, nur Audio-Endungen; liefert den MIME-Typ.
fn check_audio_path(rel: &str) -> Result<&'static str, String> {
    if rel.is_empty() || rel.contains('\\') || rel.starts_with('/') || rel.contains(':') {
        return Err(format!("Pfad {rel:?} ungültig: relativ zu public/ mit /"));
    }
    // Nur bereinigte Pfade: keine leeren Teile, kein `.` und kein `..`.
    let unclean = rel != "." && rel.split('/').any(|part| matches!(part, "" | "." | ".."));
    if unclean || rel.starts_with("..") {
        return Err(format!("Pfad {rel:?} ungültig: ohne .."));
    }
    audio_type(rel).ok_or_else(|| format!("Nur Audio (.ogg, .mp3, .wav, .m4a), nicht {rel:?}"))
}

/// Eine Audiodatei unter `public/` als Data-URL (Hörprobe). Gelesen wird nur, was auch nach dem Auflösen von
/// Verknüpfungen unter `public/` liegt (Go `os.Root`).
pub fn sound_data(root: &Path, rel: &str) -> Result<String, String> {
    let mime = check_audio_path(rel)?;
    let public = std::fs::canonicalize(root.join("public")).map_err(|e| format!("public/: {e}"))?;
    let mut path = public.clone();
    path.extend(rel.split('/'));
    let path = match std::fs::canonicalize(&path) {
        Ok(path) => path,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Err(format!("{rel} fehlt unter public/")),
        Err(e) => return Err(format!("{rel}: {e}")),
    };
    if !path.starts_with(&public) {
        return Err(format!("{rel} liegt nicht unter public/"));
    }
    let mut raw = Vec::new();
    std::fs::File::open(path)
        .and_then(|file| file.take(MAX_SOUND_BYTES + 1).read_to_end(&mut raw))
        .map_err(|e| format!("{rel}: {e}"))?;
    if raw.len() as u64 > MAX_SOUND_BYTES {
        return Err(format!("{rel} ist größer als 10 MB"));
    }
    Ok(format!("data:{mime};base64,{}", STANDARD.encode(raw)))
}

/// Der Katalog unter `public/` (mit Lizenz-Kennzeichen) und die Recherche-Treffer je Slot.
#[derive(Debug, Serialize)]
pub struct Quellen {
    pub katalog: Vec<assets::Asset>,
    pub treffer: BTreeMap<String, Vec<assets::Hit>>,
}

/// Liest Katalog und Recherche-Treffer; fehlende Dateien ergeben leere Listen.
pub fn sources(root: &Path) -> Result<Quellen, String> {
    let katalog = match assets::catalog(root) {
        Ok(list) => list,
        Err(assets::Error::Io(e)) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
        Err(e) => return Err(e.to_string()),
    };
    let research = assets::load_research(root).map_err(|e| e.to_string())?;
    Ok(Quellen {
        katalog,
        treffer: research.hits,
    })
}
