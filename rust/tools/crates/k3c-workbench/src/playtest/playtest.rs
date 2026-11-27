//! Ablage der Spieltests (Port von `playtest.go`): lesen und aufräumen. Gelöscht wird nur nach Bestätigung, eine
//! laufende Aufnahme ist gesperrt; Kennung und Dateiname sind fest geprüft, nie ein freier Pfad.

use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Error, FILE_FINDINGS, FILE_MARKERS, FILE_META, FILE_TRANSCRIPT, FILE_VIDEO, FILE_VOICE, null_default};

/// Stand einer Session.
pub(super) const STAGE_RUNNING: &str = "läuft";
pub(super) const STAGE_INCOMPLETE: &str = "unvollständig";
pub(super) const STAGE_RECORDED: &str = "aufgenommen";
pub(super) const STAGE_TRANSCRIBED: &str = "transkribiert";
pub(super) const STAGE_EVALUATED: &str = "ausgewertet";

/// Wert von `session.json` › `stand` während der Aufnahme.
pub(super) const STAND_RUNNING: &str = "laeuft";

/// Nur ASCII-Ziffern: nichts, was aus der Ablage herausführt.
static ID: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}-[0-9]+$").ok());

fn valid_id(id: &str) -> bool {
    ID.as_ref().is_some_and(|re| re.is_match(id))
}

/// Aufräum-Aktionen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// `video.mkv` einer Session.
    Video,
    /// Ganze Session.
    Session,
    /// `video.mkv` aller ausgewerteten Sessions.
    Evaluated,
}

impl Action {
    pub fn parse(name: &str) -> Option<Action> {
        match name {
            "video" => Some(Action::Video),
            "session" => Some(Action::Session),
            "ausgewertet" => Some(Action::Evaluated),
            _ => None,
        }
    }
}

/// `session.json` (Schreiber: Aufnahme).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Meta {
    #[serde(deserialize_with = "null_default")]
    pub start_ms: i64,
    /// Start von ffmpeg: Unix-ms bei 0 s von `voice.wav` und `video.mkv`.
    #[serde(skip_serializing_if = "is_zero", deserialize_with = "null_default")]
    pub capture_ms: i64,
    #[serde(deserialize_with = "null_default")]
    pub end_ms: i64,
    #[serde(deserialize_with = "null_default")]
    pub mic: String,
    #[serde(deserialize_with = "null_default")]
    pub monitor: String,
    #[serde(rename = "ffmpeg", deserialize_with = "null_default")]
    pub ffmpeg: String,
    /// `null` bei der Mikro-Session.
    pub game_args: Option<Vec<String>>,
    #[serde(deserialize_with = "null_default")]
    pub stand: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub encoder: String,
    /// Spiel endete mit Fehler, ohne dass Stopp gedrückt wurde.
    #[serde(skip_serializing_if = "std::ops::Not::not", deserialize_with = "null_default")]
    pub crash: bool,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub game_exit: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub error: String,
    /// Nummern der Marker, deren Bild ausgeblendet ist.
    #[serde(skip_serializing_if = "Vec::is_empty", deserialize_with = "null_default")]
    pub skip_frames: Vec<i64>,
}

#[allow(clippy::trivially_copy_pass_by_ref)] // Signatur für `skip_serializing_if`
fn is_zero(value: &i64) -> bool {
    *value == 0
}

impl Meta {
    /// Liest die Datei wie Go: unbekannte Felder zählen nicht, bei doppelten Schlüsseln gilt der letzte.
    pub(super) fn parse(raw: &[u8]) -> Option<Meta> {
        let value: Value = serde_json::from_str(&String::from_utf8_lossy(raw)).ok()?;
        serde_json::from_value::<Option<Meta>>(value)
            .ok()
            .map(Option::unwrap_or_default)
    }
}

/// Datei oder Ordner einer Session mit Größe in Bytes (Ordner: Summe).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct File {
    pub name: String,
    pub size: i64,
}

/// Zeile der Liste.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: String,
    pub date: String,
    pub start_ms: i64,
    pub duration_ms: i64,
    pub markers: usize,
    pub size: i64,
    pub stage: String,
    pub running: bool,
    pub has_video: bool,
    pub has_voice: bool,
    pub files: Vec<File>,
}

/// Was eine Aktion löschen würde (vor der Bestätigung) bzw. gelöscht hat.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Plan {
    pub action: String,
    pub id: String,
    pub files: Vec<File>,
    pub total: i64,
}

impl Plan {
    /// Nimmt die Datei auf, wenn die Session sie hat; bei der Sammelaktion mit Session-Ordner im Namen.
    fn add(&mut self, session: &Session, name: &str, prefixed: bool) {
        for file in session.files.iter().filter(|f| f.name == name) {
            let mut file = file.clone();
            if prefixed {
                file.name = format!("{}/{}", session.id, file.name);
            }
            self.total += file.size;
            self.files.push(file);
        }
    }
}

/// Kennung der laufenden Aufnahme dieses Prozesses (leer: keine).
pub type Active = Arc<dyn Fn() -> String + Send + Sync>;

/// Die Ablage `playtests/` im Repo. Ist `active` gesetzt, gilt ein `laeuft` in `session.json` ohne diese Kennung als
/// Überbleibsel eines Absturzes: unvollständig und löschbar.
#[derive(Clone)]
pub struct Store {
    pub root: PathBuf,
    pub active: Option<Active>,
}

impl Store {
    /// Alle Sessions, neueste zuerst; fehlt der Ordner, ist die Liste leer. Andere Namen werden übergangen.
    pub fn list(&self) -> Result<Vec<Session>, Error> {
        let entries = match std::fs::read_dir(&self.root) {
            Ok(entries) => entries,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(Error::Other(format!("{}: {e}", self.root.display()))),
        };
        let mut out: Vec<Session> = entries
            .flatten()
            .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
            .filter_map(|e| e.file_name().into_string().ok())
            .filter(|name| valid_id(name))
            .map(|name| self.read(&name))
            .collect();
        // Neueste zuerst nach Startzeit, ohne session.json nach Kennung.
        out.sort_by(|a, b| b.start_ms.cmp(&a.start_ms).then_with(|| b.id.cmp(&a.id)));
        Ok(out)
    }

    /// Eine Session.
    pub fn get(&self, id: &str) -> Result<Session, Error> {
        if !valid_id(id) {
            return Err(Error::BadId);
        }
        if !self.root.join(id).is_dir() {
            return Err(Error::NotFound);
        }
        Ok(self.read(id))
    }

    /// Zeile einer vorhandenen Session; kaputtes oder fehlendes `session.json` ergibt „unvollständig“.
    fn read(&self, id: &str) -> Session {
        let dir = self.root.join(id);
        let mut session = Session {
            id: id.to_owned(),
            date: id.get(..10).unwrap_or_default().to_owned(),
            ..Session::default()
        };
        for (name, path) in entries(&dir) {
            let size = size_of(&path);
            session.size += size;
            session.has_video |= name == FILE_VIDEO;
            session.has_voice |= name == FILE_VOICE;
            session.files.push(File { name, size });
        }
        session.markers = count_lines(&dir.join(FILE_MARKERS));
        let active = self.active.as_ref().map(|active| active());
        let Some(meta) = std::fs::read(dir.join(FILE_META))
            .ok()
            .and_then(|raw| Meta::parse(&raw))
        else {
            session.stage = STAGE_INCOMPLETE.to_owned();
            // Ordner angelegt, session.json folgt gleich.
            if active.as_deref() == Some(id) {
                (session.stage, session.running) = (STAGE_RUNNING.to_owned(), true);
            }
            return session;
        };
        session.start_ms = meta.start_ms;
        if meta.end_ms > meta.start_ms {
            session.duration_ms = meta.end_ms - meta.start_ms;
        }
        let stale = active.is_some_and(|a| a != id);
        let open = meta.stand == STAND_RUNNING && meta.end_ms == 0;
        let has = |name: &str| session.files.iter().any(|f| f.name == name);
        let stage = if open && stale {
            STAGE_INCOMPLETE
        } else if open {
            STAGE_RUNNING
        } else if has(FILE_FINDINGS) {
            STAGE_EVALUATED
        } else if has(FILE_TRANSCRIPT) {
            STAGE_TRANSCRIBED
        } else {
            STAGE_RECORDED
        };
        session.running = stage == STAGE_RUNNING;
        session.stage = stage.to_owned();
        session
    }

    /// Was die Aktion löschen würde, ohne etwas anzufassen. `id` bleibt bei `ausgewertet` leer.
    pub fn plan(&self, action: &str, id: &str) -> Result<Plan, Error> {
        let mut plan = Plan {
            action: action.to_owned(),
            id: id.to_owned(),
            ..Plan::default()
        };
        match Action::parse(action).ok_or(Error::BadAction)? {
            kind @ (Action::Video | Action::Session) => {
                let session = self.get(id)?;
                if session.running {
                    return Err(Error::Running);
                }
                if kind == Action::Session {
                    plan.files.push(File {
                        name: format!("{id}/"),
                        size: session.size,
                    });
                    plan.total = session.size;
                } else {
                    plan.add(&session, FILE_VIDEO, false);
                }
            }
            Action::Evaluated => {
                plan.id = String::new();
                for session in self.list()?.iter().filter(|s| s.stage == STAGE_EVALUATED) {
                    plan.add(session, FILE_VIDEO, true);
                }
            }
        }
        Ok(plan)
    }

    /// Führt die Aktion aus, aber nur mit `confirm`; ohne bleibt alles stehen. Ergebnis ist der ausgeführte Plan.
    pub fn delete(&self, action: &str, id: &str, confirm: bool) -> Result<Plan, Error> {
        let plan = self.plan(action, id)?;
        if !confirm {
            return Err(Error::NotConfirmed);
        }
        // `plan` hat Aktion und Kennung geprüft; die Sammelaktion nennt nur Kennungen aus `list`.
        match Action::parse(action).ok_or(Error::BadAction)? {
            Action::Session => ignore_missing(std::fs::remove_dir_all(self.root.join(id)))?,
            Action::Video => {
                for file in &plan.files {
                    ignore_missing(remove(&self.root.join(id).join(&file.name)))?;
                }
            }
            Action::Evaluated => {
                for file in &plan.files {
                    let path = file
                        .name
                        .split('/')
                        .fold(self.root.clone(), |path, part| path.join(part));
                    ignore_missing(remove(&path))?;
                }
            }
        }
        Ok(plan)
    }

    /// Pfad einer abspielbaren Datei; nur `voice.wav` und `video.mkv` sind erlaubt.
    pub fn media_path(&self, id: &str, name: &str) -> Result<PathBuf, Error> {
        if !valid_id(id) {
            return Err(Error::BadId);
        }
        if name != FILE_VOICE && name != FILE_VIDEO {
            return Err(Error::NotFound);
        }
        let path = self.root.join(id).join(name);
        if std::fs::metadata(&path).is_ok_and(|m| !m.is_dir()) {
            Ok(path)
        } else {
            Err(Error::NotFound)
        }
    }
}

/// Wie Go `os.Remove`: Datei, sonst leerer Ordner.
fn remove(path: &Path) -> std::io::Result<()> {
    std::fs::remove_file(path).or_else(|e| {
        if path.is_dir() {
            std::fs::remove_dir(path)
        } else {
            Err(e)
        }
    })
}

fn ignore_missing(result: std::io::Result<()>) -> Result<(), Error> {
    match result {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(Error::Other(e.to_string())),
        _ => Ok(()),
    }
}

/// Einträge eines Ordners nach Namen sortiert wie Go `os.ReadDir`.
fn entries(dir: &Path) -> Vec<(String, PathBuf)> {
    let Ok(read) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<(String, PathBuf)> = read
        .flatten()
        .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path()))
        .collect();
    out.sort();
    out
}

/// Größe einer Datei oder Summe eines Ordners; Verknüpfungen wird nicht gefolgt.
fn size_of(path: &Path) -> i64 {
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return 0;
    };
    if meta.is_dir() {
        return entries(path).iter().map(|(_, p)| size_of(p)).sum();
    }
    i64::try_from(meta.len()).unwrap_or(i64::MAX)
}

/// Nichtleere Zeilen einer Datei.
fn count_lines(path: &Path) -> usize {
    std::fs::read(path).map_or(0, |raw| {
        String::from_utf8_lossy(&raw)
            .split('\n')
            .filter(|l| !l.trim().is_empty())
            .count()
    })
}

#[cfg(test)]
#[path = "playtest_tests.rs"]
mod tests;
