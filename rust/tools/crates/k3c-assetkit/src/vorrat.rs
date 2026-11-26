//! Vorrat der Vorschlagsliste (B-601, WZ29.6): Cache unter `.work/vorrat/` (nie in Git) mit Index; Status je Eintrag.
//! Download in `vorrat_download.rs`, Übernahme und Aufräumen in `vorrat_adopt.rs`.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::fsutil;
use crate::gojson::{self, null_default};
use crate::register::{load, write_atomic};
use crate::research::from_object;
use crate::suggestions::{Suggestion, suggestion_license_allowed};

/// Ordner des Vorrats, je Art und Quelle ein Unterordner; erst die Übernahme kopiert nach `public/`.
pub const VORRAT_DIR: &str = ".work/vorrat";

/// Merkt je geladenem Eintrag Quelle, Lizenz, Größe, Prüfsumme und Ladezeit.
const VORRAT_INDEX: &str = "index.json";

const VORRAT_COMMENT: &str = "Vorrat der Vorschlagsliste (B-601): geladene Dateien mit Quelle, Lizenz, Größe, SHA-256 \
     und Ladezeit. Nur lokal, nie in Git.";

/// Status eines Eintrags aus Liste und Index: noch nicht geladen.
pub const VORRAT_OPEN: &str = "offen";
/// Index und Datei sind vorhanden.
pub const VORRAT_LOADED: &str = "geladen";
/// Lizenz nicht erlaubt oder in der Liste gesperrt.
pub const VORRAT_BLOCKED: &str = "gesperrt";
/// In der Liste verworfen.
pub const VORRAT_DROPPED: &str = "verworfen";

/// Eine geladene Datei des Vorrats.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct VorratEntry {
    #[serde(deserialize_with = "null_default")]
    pub id: String,
    #[serde(rename = "quelle", deserialize_with = "null_default")]
    pub source: String,
    #[serde(rename = "lizenz", deserialize_with = "null_default")]
    pub license: String,
    #[serde(rename = "groesse", deserialize_with = "null_default")]
    pub size: i64,
    #[serde(rename = "pruefsumme", deserialize_with = "null_default")]
    pub sha256: String,
    #[serde(rename = "geladen", deserialize_with = "null_default")]
    pub loaded: String,
    /// Relativ zu `.work/vorrat/`.
    #[serde(rename = "datei", deserialize_with = "null_default")]
    pub file: String,
    /// Nach der Übernahme: Datei relativ zu `public/`.
    #[serde(deserialize_with = "null_default", skip_serializing_if = "String::is_empty")]
    pub public: String,
}

/// Inhalt von `.work/vorrat/index.json`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct VorratIndex {
    #[serde(rename = "_comment", deserialize_with = "null_default")]
    pub comment: String,
    /// `None` steht wie in Go (nil) als `null` in der Datei.
    #[serde(rename = "eintraege")]
    pub entries: Option<Vec<VorratEntry>>,
}

/// Reiht Lesen und Schreiben des Index hintereinander (Downloads, Übernahme, Aufräumen).
static VORRAT_MU: Mutex<()> = Mutex::new(());

pub(crate) fn lock_vorrat() -> MutexGuard<'static, ()> {
    VORRAT_MU.lock().unwrap_or_else(PoisonError::into_inner)
}

pub(crate) fn vorrat_path(root: &Path, rel: &str) -> PathBuf {
    fsutil::join_below(&fsutil::join_below(root, VORRAT_DIR), rel)
}

/// Liest den Index; fehlt er, ist der Vorrat leer.
pub fn load_vorrat(root: &Path) -> Result<VorratIndex> {
    let bytes = match fs::read(vorrat_path(root, VORRAT_INDEX)) {
        Ok(b) => b,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Ok(VorratIndex {
                comment: VORRAT_COMMENT.to_string(),
                entries: None,
            });
        }
        Err(e) => return Err(Error::Io(e)),
    };
    from_object(&bytes).map_err(|e| Error::Msg(format!("{VORRAT_DIR}/{VORRAT_INDEX}: {e}")))
}

impl VorratIndex {
    /// Die Einträge; ohne Index leer.
    pub fn entries(&self) -> &[VorratEntry] {
        self.entries.as_deref().unwrap_or_default()
    }

    fn save(&self, root: &Path) -> Result<()> {
        let file = vorrat_path(root, VORRAT_INDEX);
        if let Some(dir) = file.parent() {
            fs::create_dir_all(dir)?;
        }
        let mut bytes = gojson::marshal_indent(self, true).map_err(|e| Error::Msg(e.to_string()))?;
        bytes.push(b'\n');
        write_atomic(&file, &bytes)
    }

    /// Der Eintrag mit der ID.
    pub fn find(&self, id: &str) -> Option<&VorratEntry> {
        self.entries().iter().find(|e| e.id == id)
    }

    pub(crate) fn find_mut(&mut self, id: &str) -> Option<&mut VorratEntry> {
        self.entries.as_mut()?.iter_mut().find(|e| e.id == id)
    }

    /// Ersetzt den Eintrag mit derselben ID oder hängt ihn an.
    pub(crate) fn put(&mut self, entry: VorratEntry) {
        match self.find_mut(&entry.id) {
            Some(old) => *old = entry,
            None => self.entries.get_or_insert_default().push(entry),
        }
    }
}

/// Liest den Index unter Sperre, ändert ihn und speichert.
pub(crate) fn change_vorrat(root: &Path, change: impl FnOnce(&mut VorratIndex) -> Result<()>) -> Result<()> {
    let _guard = lock_vorrat();
    let mut index = load_vorrat(root)?;
    change(&mut index)?;
    index.save(root)
}

/// Status eines Eintrags der Liste: `entry` nur bei „geladen“, `used` = die übernommene Datei ist einem Slot zugeordnet.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VorratState {
    pub status: &'static str,
    pub reason: String,
    pub entry: Option<VorratEntry>,
    pub used: bool,
}

/// Status je ID der Liste: gesperrt (wird nie geladen), verworfen, geladen (Index und Datei vorhanden) oder offen.
pub fn vorrat_states(root: &Path, list: &[Suggestion]) -> Result<BTreeMap<String, VorratState>> {
    let index = load_vorrat(root)?;
    let used = used_files(root)?;
    Ok(list
        .iter()
        .map(|s| (s.id.clone(), state_of(root, s, &index, &used)))
        .collect())
}

fn state_of(root: &Path, s: &Suggestion, index: &VorratIndex, used: &HashSet<String>) -> VorratState {
    let state = |status: &'static str, reason: String| VorratState {
        status,
        reason,
        ..VorratState::default()
    };
    if !suggestion_license_allowed(&s.license) {
        return state(VORRAT_BLOCKED, format!("Lizenz {} nicht erlaubt (Q70)", s.license));
    }
    if s.status == VORRAT_BLOCKED {
        let reason = "in der Vorschlagsliste gesperrt (Lizenz der Quelle weicht ab)";
        return state(VORRAT_BLOCKED, reason.to_string());
    }
    if s.status == VORRAT_DROPPED {
        return state(VORRAT_DROPPED, String::new());
    }
    match index.find(&s.id) {
        Some(e) if file_exists(&vorrat_path(root, &e.file)) => VorratState {
            status: VORRAT_LOADED,
            reason: String::new(),
            used: !e.public.is_empty() && used.contains(&e.public),
            entry: Some(e.clone()),
        },
        _ => state(VORRAT_OPEN, String::new()),
    }
}

/// Alle Dateien, die das Slot-Register einem Slot zuordnet.
pub(crate) fn used_files(root: &Path) -> Result<HashSet<String>> {
    let register = load(root)?;
    Ok(register
        .slots
        .iter()
        .flat_map(|s| s.choices())
        .map(|c| c.file.clone())
        .collect())
}

pub(crate) fn file_exists(path: &Path) -> bool {
    fs::metadata(path).is_ok_and(|m| m.is_file())
}
