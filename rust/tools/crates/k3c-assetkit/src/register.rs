//! Slot-Register der Sounds (`data/sounds.json`, B-298): Laden, Speichern, Zuordnungen prüfen und atomar schreiben.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::value::RawValue;

use crate::error::{Error, Result};
use crate::gojson::{self, null_default};
use crate::target::{Units, check_target};
use crate::{fsutil, pngsize};

/// Slot-Register relativ zur Repo-Wurzel.
pub const REGISTER_PATH: &str = "data/sounds.json";

/// Art eines Asset-Slots: Grafik.
pub const KIND_GRAPHIC: &str = "grafik";
/// Art eines Asset-Slots: Sound.
pub const KIND_SOUND: &str = "sound";

/// Status in der Übersicht: zugeordnet.
pub const STATUS_ASSIGNED: &str = "zugeordnet";
/// Status in der Übersicht: keine Zuordnung.
pub const STATUS_GAP: &str = "luecke";
/// Status in der Übersicht: eine Datei liegt nicht auf der Platte.
pub const STATUS_MISSING: &str = "fehlt";

/// Kategorien je Art, zugleich die Mini-Szenen der Dev-Seite (B-299).
pub const CATEGORIES: &[(&str, &[&str])] = &[
    (KIND_GRAPHIC, &["figuren", "gebaeude", "umgebung", "objekte", "hud"]),
    (KIND_SOUND, &["sfx", "musik", "ambient"]),
];

/// Ausschnitt in Bild-Pixeln.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Rect {
    #[serde(deserialize_with = "null_default")]
    pub x: i64,
    #[serde(deserialize_with = "null_default")]
    pub y: i64,
    #[serde(deserialize_with = "null_default")]
    pub w: i64,
    #[serde(deserialize_with = "null_default")]
    pub h: i64,
}

/// Maß in Pixeln (Frame-Raster).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Size {
    #[serde(deserialize_with = "null_default")]
    pub w: i64,
    #[serde(deserialize_with = "null_default")]
    pub h: i64,
}

/// Zuordnung: Datei relativ zu `public/`, optional Sheet (alte Pixel-Sheets), Ausschnitt, Frame-Raster oder (Sound) der
/// Name im Audio-Atlas.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Choice {
    #[serde(rename = "datei", deserialize_with = "null_default")]
    pub file: String,
    #[serde(deserialize_with = "null_default", skip_serializing_if = "String::is_empty")]
    pub sheet: String,
    #[serde(rename = "ausschnitt", skip_serializing_if = "Option::is_none")]
    pub cut: Option<Rect>,
    #[serde(rename = "raster", skip_serializing_if = "Option::is_none")]
    pub grid: Option<Size>,
    #[serde(deserialize_with = "null_default", skip_serializing_if = "String::is_empty")]
    pub sprite: String,
}

/// Asset-Slot mit Zuordnungen nach Präferenz (Index 0 = Präferenz 1).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Slot {
    #[serde(deserialize_with = "null_default")]
    pub id: String,
    #[serde(rename = "art", deserialize_with = "null_default")]
    pub kind: String,
    #[serde(rename = "kategorie", deserialize_with = "null_default")]
    pub category: String,
    #[serde(rename = "beschreibung", deserialize_with = "null_default")]
    pub description: String,
    #[serde(rename = "ziel", skip_serializing_if = "Option::is_none")]
    pub target: Option<Units>,
    #[serde(deserialize_with = "null_default", skip_serializing_if = "String::is_empty")]
    pub bus: String,
    /// `None` steht wie die nil-Liste in Go als `null` in der Datei; gelesen wird über [`Slot::choices`].
    #[serde(rename = "wahl")]
    pub choices: Option<Vec<Choice>>,
}

impl Slot {
    /// Die Zuordnungen, leer ohne Liste.
    pub fn choices(&self) -> &[Choice] {
        self.choices.as_deref().unwrap_or_default()
    }
}

/// Inhalt von `data/sounds.json`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Register {
    #[serde(rename = "_comment", deserialize_with = "null_default")]
    pub comment: String,
    /// Mischpegel des Spiels (B-935), beim Speichern unverändert.
    #[serde(
        rename = "mischung",
        deserialize_with = "keep_null",
        skip_serializing_if = "Option::is_none"
    )]
    pub mix: Option<Box<RawValue>>,
    #[serde(deserialize_with = "null_default")]
    pub slots: Vec<Slot>,
}

/// Wie Go `json.RawMessage`: auch ein `null` bleibt als Text erhalten.
fn keep_null<'de, D: Deserializer<'de>>(d: D) -> std::result::Result<Option<Box<RawValue>>, D::Error> {
    Box::<RawValue>::deserialize(d).map(Some)
}

/// Reiht Schreibzugriffe von Oberfläche und MCP-Tools hintereinander (ein Prozess).
static MU: Mutex<()> = Mutex::new(());

/// Hält die Schreibsperre des Registers.
pub(crate) fn lock() -> MutexGuard<'static, ()> {
    MU.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Liest das Slot-Register; ein ungültiges Register ist ein Fehler mit Pfad.
pub fn load(root: &Path) -> Result<Register> {
    let bytes = fs::read(root.join(REGISTER_PATH))
        .map_err(|e| Error::Io(io::Error::new(e.kind(), format!("{REGISTER_PATH}: {e}"))))?;
    let invalid = |e: String| Error::Msg(format!("{REGISTER_PATH} ungültig, nichts geschrieben: {e}"));
    let reg = serde_json::from_slice::<Option<Register>>(&bytes)
        .map_err(|e| invalid(e.to_string()))?
        .unwrap_or_default();
    for s in &reg.slots {
        check_target(s.target.as_ref()).map_err(|e| invalid(format!("Slot {}: {e}", s.id)))?;
    }
    Ok(reg)
}

impl Register {
    /// Der Asset-Slot mit der ID.
    pub fn find(&self, id: &str) -> Option<&Slot> {
        self.slots.iter().find(|s| s.id == id)
    }

    /// Wie [`Register::find`], zum Ändern.
    pub fn find_mut(&mut self, id: &str) -> Option<&mut Slot> {
        self.slots.iter_mut().find(|s| s.id == id)
    }

    /// Schreibt das Register atomar: zwei Leerzeichen Einrückung, Umlaute im Klartext, Zeilenende `\n`.
    pub fn save(&self, root: &Path) -> Result<()> {
        let mut bytes = gojson::marshal_indent(self, false).map_err(|e| Error::Msg(e.to_string()))?;
        bytes.push(b'\n');
        write_atomic(&root.join(REGISTER_PATH), &bytes)
    }
}

/// Ersetzt die Zuordnungen eines Asset-Slots (Reihenfolge = Präferenz) nach Prüfung und speichert.
pub fn set_choices(root: &Path, id: &str, choices: &[Choice]) -> Result<Slot> {
    let _guard = lock();
    set_choices_locked(root, id, choices)
}

/// [`set_choices`] ohne Sperre; der Aufrufer hält [`lock`].
pub(crate) fn set_choices_locked(root: &Path, id: &str, choices: &[Choice]) -> Result<Slot> {
    let mut reg = load(root)?;
    let slot = reg
        .find_mut(id)
        .ok_or_else(|| Error::Msg(format!("unbekannter Asset-Slot {id:?}")))?;
    for (i, c) in choices.iter().enumerate() {
        check_choice(root, &slot.kind, c).map_err(|e| Error::Msg(format!("präferenz {}: {e}", i + 1)))?;
    }
    slot.choices = Some(choices.to_vec());
    let slot = slot.clone();
    reg.save(root)?;
    Ok(slot)
}

/// Endungen eines Sound-Slots, alles andere gilt als Grafik.
pub(crate) const SOUND_EXT: &[&str] = &["ogg", "mp3", "wav", "m4a"];

pub(crate) fn check_choice(root: &Path, kind: &str, c: &Choice) -> Result<()> {
    let full = public_file(root, &c.file)?;
    let name = c.file.rsplit('/').next().unwrap_or_default();
    let sound = name
        .rsplit_once('.')
        .is_some_and(|(_, ext)| SOUND_EXT.contains(&ext.to_lowercase().as_str()));
    if sound != (kind == KIND_SOUND) {
        return Err(Error::Msg(format!("{} passt nicht zur Art {kind}", c.file)));
    }
    let Some(r) = c.cut else { return Ok(()) };
    let (width, height) =
        pngsize::size(&full).map_err(|e| Error::Msg(format!("{}: Maße nicht lesbar: {e}", c.file)))?;
    // wrapping_add rechnet bei Überlauf wie Go, statt abzubrechen.
    if r.x < 0 || r.y < 0 || r.w <= 0 || r.h <= 0 || r.x.wrapping_add(r.w) > width || r.y.wrapping_add(r.h) > height {
        return Err(Error::Msg(format!(
            "ausschnitt {},{},{},{} außerhalb von {} ({width}×{height})",
            r.x, r.y, r.w, r.h, c.file
        )));
    }
    Ok(())
}

/// Prüft, dass `rel` eine vorhandene Datei unter `public/` ist, und liefert den vollen Pfad.
fn public_file(root: &Path, rel: &str) -> Result<PathBuf> {
    let clean = fsutil::path_clean(rel);
    if rel.is_empty() || clean.starts_with('/') || clean == ".." || clean.starts_with("../") || rel.contains('\\') {
        return Err(Error::Msg(format!("{rel:?} liegt nicht unter public/")));
    }
    let full = fsutil::join_below(&root.join("public"), &clean);
    match fs::metadata(&full) {
        Ok(meta) if !meta.is_dir() => Ok(full),
        _ => Err(Error::Msg(format!("{rel} fehlt unter public/"))),
    }
}

/// `zugeordnet`, `luecke` (keine Zuordnung) oder `fehlt` (eine Datei liegt nicht auf der Platte).
pub fn status(root: &Path, s: &Slot) -> &'static str {
    if s.choices().is_empty() {
        return STATUS_GAP;
    }
    if s.choices().iter().any(|c| public_file(root, &c.file).is_err()) {
        return STATUS_MISSING;
    }
    STATUS_ASSIGNED
}

/// Schreibt `data` atomar nach `path` (Schreibregel B-459/AC-07): über eine Temp-Datei im selben Ordner mit Sync vor dem
/// Umbenennen. Die Rechte einer vorhandenen Datei bleiben; scheitert ein Schritt, bleibt die alte Datei unverändert.
pub fn write_atomic(path: &Path, data: &[u8]) -> Result<()> {
    let dir = match path.parent() {
        Some(p) if !p.as_os_str().is_empty() => p,
        _ => Path::new("."),
    };
    let (mut file, tmp) = fsutil::create_temp(dir, ".assets-")?;
    let written = file.write_all(data).and_then(|()| file.sync_all());
    drop(file);
    if let Err(e) = written {
        let _ = fsutil::remove(&tmp);
        return Err(Error::Msg(format!("schreiben: {e} <nil>")));
    }
    if let Ok(meta) = fs::metadata(path)
        && meta.is_file()
    {
        let _ = fs::set_permissions(&tmp, meta.permissions());
    }
    if let Err(e) = fs::rename(&tmp, path) {
        let _ = fsutil::remove(&tmp);
        return Err(Error::Io(e));
    }
    Ok(())
}

#[cfg(test)]
#[path = "register_tests.rs"]
mod tests;
