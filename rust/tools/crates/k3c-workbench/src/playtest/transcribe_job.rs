//! Auftrag der Auswertung (Port von `transcribe_job.go`): höchstens ein Lauf im Hintergrund, mit Fortschritt und Abbruch;
//! erst das Transkript, dann ein Bild je Marker. Nur auf Knopfdruck, nie während der Aufnahme. Prozesse entstehen über
//! [`Spawn`] und [`Probe`], damit Tests Attrappen einsetzen.

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::capability::check;
use super::devices::Probe;
use super::exec::{Proc, Spawn};
use super::ffmpeg_find::LookPath;
use super::frames::{capture_start, frame_args};
use super::playtest::{Meta, STAGE_TRANSCRIBED, Session};
use super::record::write_meta;
use super::stats::{Transcript, save_stats};
use super::timeline::read_markers;
use super::transcribe::{FILE_RAW_TMP, ModelInfo, frame_name, parse_language, parse_segments, whisper_args};
use super::transcript_txt::write_transcript_text;
use super::{DIR_FRAMES, Error, FILE_META, FILE_TRANSCRIPT, LogFn, Signal, Store, go_json, lock, null_default};

/// Stand der Auswertung für die Seite.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct JobStatus {
    pub running: bool,
    pub id: String,
    /// Transkript, Bilder.
    pub step: String,
    pub done: usize,
    pub total: usize,
    /// Hinweis nach dem Ende (z. B. Bilder entfallen).
    pub message: String,
    pub error: String,
}

/// Wahl von Modell (ID aus der Liste, leer: Standard) und Sprache (de, en, auto; leer: de).
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct TranscribeOptions {
    #[serde(deserialize_with = "null_default")]
    pub model: String,
    #[serde(deserialize_with = "null_default")]
    pub language: String,
}

/// Werkzeuge und Ablage der Auswertung.
pub struct TranscribeConfig {
    pub store: Store,
    pub spawn: Spawn,
    pub look_path: LookPath,
    /// Kurzer Aufruf mit Ausgabe (Prüfung von ffmpeg).
    pub out: Probe,
    /// Standardmodell.
    pub model: Arc<dyn Fn() -> String + Send + Sync>,
    /// Wählbare Modelle; nur daraus darf der Aufruf wählen.
    pub models: Arc<dyn Fn() -> Vec<ModelInfo> + Send + Sync>,
    pub log: Option<LogFn>,
}

#[derive(Default)]
struct State {
    status: JobStatus,
    current: Option<Arc<dyn Proc>>,
    cancel: bool,
    done: Option<Arc<Signal<()>>>,
}

/// Ein Auftrag mit allem, was der Hintergrundfaden braucht.
struct Job {
    session: Session,
    ffmpeg: String,
    model: String,
    language: String,
}

/// Steuert höchstens eine Auswertung.
pub struct Transcriber {
    config: TranscribeConfig,
    state: Mutex<State>,
}

impl Transcriber {
    pub fn new(config: TranscribeConfig) -> Arc<Transcriber> {
        Arc::new(Transcriber {
            config,
            state: Mutex::default(),
        })
    }

    fn log(&self, level: &str, message: &str, fields: &[(&str, Value)]) {
        if let Some(log) = &self.config.log {
            log(level, message, fields);
        }
    }

    pub fn status(&self) -> JobStatus {
        lock(&self.state).status.clone()
    }

    /// Nur Modelle aus der Liste, nie ein Pfad vom Client.
    fn resolve_model(&self, id: &str) -> Result<String, Error> {
        if id.is_empty() {
            return Ok((self.config.model)());
        }
        if let Some(found) = (self.config.models)().into_iter().find(|m| m.id == id) {
            return Ok(found.path);
        }
        let shown = crate::planning::store::quoted(id);
        Err(Error::Other(format!(
            "unbekanntes Modell {shown}: nur Dateien aus models/ oder K3C_WHISPER_MODEL (Liste der Seite)"
        )))
    }

    /// Prüft alles Nötige und wertet die Session im Hintergrund aus. Blockiert nur für die Prüfung.
    pub fn start(self: &Arc<Self>, id: &str, options: &TranscribeOptions) -> Result<JobStatus, Error> {
        let session = self.config.store.get(id)?;
        if session.running {
            return Err(Error::Running);
        }
        if !session.has_voice {
            return Err(Error::NoVoice);
        }
        let model = self.resolve_model(&options.model)?;
        let language = parse_language(&options.language)?;
        let capability = check(&self.config.look_path, &self.config.out, &model);
        if !capability.ready {
            return Err(Error::Other(capability.message));
        }
        let ffmpeg = (self.config.look_path)("ffmpeg").map_err(super::no_ffmpeg)?;
        let job = Job {
            session,
            ffmpeg,
            model,
            language,
        };
        let mut state = lock(&self.state);
        if state.status.running {
            return Err(Error::TranscribeBusy);
        }
        state.status = JobStatus {
            running: true,
            id: id.to_owned(),
            step: "Transkript".to_owned(),
            ..JobStatus::default()
        };
        let done = Signal::new();
        (state.cancel, state.done) = (false, Some(Arc::clone(&done)));
        self.log("info", "🚀 Auswertung gestartet", &[("id", json!(id))]);
        let this = Arc::clone(self);
        let spawned = std::thread::Builder::new()
            .name("spieltest-auswertung".to_owned())
            .spawn(move || {
                this.run(&job);
                done.set(());
            });
        if let Err(e) = spawned {
            state.status.running = false;
            state.done = None;
            return Err(Error::Other(e.to_string()));
        }
        Ok(state.status.clone())
    }

    /// Bricht die Auswertung ab; ein halbes Transkript bleibt nicht liegen.
    pub fn cancel(&self) -> Result<(), Error> {
        let mut state = lock(&self.state);
        if !state.status.running {
            return Err(Error::NoJob);
        }
        state.cancel = true;
        if let Some(proc) = &state.current {
            proc.kill();
        }
        Ok(())
    }

    /// Wartet aufs Ende der Auswertung (Tests).
    #[cfg(test)]
    pub(crate) fn wait(&self) {
        let done = lock(&self.state).done.clone();
        if let Some(done) = done {
            let ended = done.wait_timeout(std::time::Duration::from_secs(10));
            assert!(ended.is_some(), "Auswertung endet nicht");
        }
    }

    fn set(&self, change: impl FnOnce(&mut JobStatus)) {
        change(&mut lock(&self.state).status);
    }

    /// Führt einen Aufruf aus und wartet; Abbruch und Fehler kommen als Fehler zurück.
    fn exec(&self, args: &[String], dir: &Path) -> Result<(), Error> {
        let proc = {
            let mut state = lock(&self.state);
            if state.cancel {
                return Err(Error::Canceled);
            }
            let proc = (self.config.spawn)(args, dir)?;
            state.current = Some(Arc::clone(&proc));
            proc
        };
        let result = proc.wait();
        let canceled = {
            let mut state = lock(&self.state);
            state.current = None;
            state.cancel
        };
        if canceled {
            return Err(Error::Canceled);
        }
        result.map_err(|e| Error::Other(format!("ffmpeg: {e}: {}", proc.tail())))
    }

    /// Der Hintergrundauftrag.
    fn run(&self, job: &Job) {
        let id = job.session.id.as_str();
        let dir = self.config.store.root.join(id);
        let t0 = SystemTime::now();
        let mut result = self.transcribe(job, &dir);
        let t1 = SystemTime::now();
        let mut note = String::new();
        if result.is_ok() {
            (note, result) = self.frames(job, &dir);
        }
        if result.is_ok()
            && let Err(e) = save_stats(&dir, t0, t1, SystemTime::now(), job.session.duration_ms)
        {
            let message = format!("❌ Statistik nicht gespeichert: {e}");
            self.log("warn", &message, &[("id", json!(id))]);
        }
        self.set(|s| {
            (s.running, s.message) = (false, note);
            if let Err(e) = &result {
                s.error = e.to_string();
            }
        });
        let fields = [("id", json!(id))];
        match result {
            Err(Error::Canceled) => self.log("info", "🛑 Auswertung abgebrochen", &fields),
            Err(e) => self.log("warn", &format!("❌ Auswertung fehlgeschlagen: {e}"), &fields),
            Ok(()) => self.log("info", "✅ Auswertung fertig", &fields),
        }
    }

    /// Lässt whisper laufen und schreibt `transcript.json` erst, wenn alles gelesen ist.
    fn transcribe(&self, job: &Job, dir: &Path) -> Result<(), Error> {
        let part = dir.join(FILE_RAW_TMP);
        let _ = std::fs::remove_file(&part);
        let args = whisper_args(&job.ffmpeg, dir, &job.model, &job.language, FILE_RAW_TMP);
        let result = self.exec(&args, dir).and_then(|()| self.write_transcript(job, dir));
        let _ = std::fs::remove_file(&part);
        result
    }

    fn write_transcript(&self, job: &Job, dir: &Path) -> Result<(), Error> {
        let failed = |e: &dyn std::fmt::Display| Error::Other(e.to_string());
        let raw = std::fs::read(dir.join(FILE_RAW_TMP))
            .map_err(|e| Error::Other(format!("whisper hat kein Ergebnis geschrieben: {e}")))?;
        let mut meta = read_meta(dir);
        let start = capture_start(&meta);
        let model = Path::new(&job.model)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned());
        let transcript = Transcript {
            audio_start_ms: start,
            language: job.language.clone(),
            model: model.unwrap_or_default(),
            segments: Some(parse_segments(&raw, start)?),
            stats: None,
        };
        let out = go_json(&transcript).map_err(|e| failed(&e))?;
        let tmp = dir.join(format!("{FILE_TRANSCRIPT}.tmp"));
        std::fs::write(&tmp, out).map_err(|e| failed(&e))?;
        std::fs::rename(&tmp, dir.join(FILE_TRANSCRIPT)).map_err(|e| failed(&e))?;
        if let Err(e) = write_transcript_text(dir, &transcript) {
            self.log("warn", &format!("❌ transcript.txt nicht geschrieben: {e}"), &[]);
        }
        STAGE_TRANSCRIBED.clone_into(&mut meta.stand);
        write_meta(dir, &meta);
        Ok(())
    }

    /// Zieht je Marker ein Bild; ohne Video entfallen sie mit Hinweis. Ergebnis: Hinweis und Fehler.
    fn frames(&self, job: &Job, dir: &Path) -> (String, Result<(), Error>) {
        let none = "keine Marker, keine Bilder".to_owned();
        let marks = match read_markers(dir) {
            Ok(marks) if !marks.is_empty() => marks,
            Ok(_) => return (none, Ok(())),
            Err(e) => return (none, Err(e)),
        };
        if !job.session.has_video {
            return ("video.mkv fehlt, Bilder entfallen".to_owned(), Ok(()));
        }
        if let Err(e) = std::fs::create_dir_all(dir.join(DIR_FRAMES)) {
            return (String::new(), Err(Error::Other(e.to_string())));
        }
        self.set(|s| (s.step, s.done, s.total) = ("Bilder".to_owned(), 0, marks.len()));
        let start = capture_start(&read_meta(dir));
        let mut missing = Vec::new();
        for (nr, mark) in (1_i64..).zip(&marks) {
            #[allow(clippy::cast_precision_loss)] // Millisekunden einer Aufnahme
            let seconds = mark.ts.saturating_sub(start).max(0) as f64 / 1000.0;
            let name = frame_name(nr).replace('/', std::path::MAIN_SEPARATOR_STR);
            match self.exec(&frame_args(&job.ffmpeg, seconds, &name), dir) {
                Ok(()) => {}
                Err(Error::Canceled) => return (String::new(), Err(Error::Canceled)),
                // Ein Marker nach dem Ende von video.mkv („Nothing was written“) kippt nicht die ganze Auswertung.
                Err(e) => {
                    missing.push(nr.to_string());
                    let message = format!("❌ Marker-Bild nicht erzeugt: {e}");
                    self.log("warn", &message, &[("marker", json!(nr))]);
                }
            }
            self.set(|s| s.done = usize::try_from(nr).unwrap_or_default());
        }
        let note = if missing.is_empty() {
            String::new()
        } else {
            let list = missing.join(", ");
            format!("Bild fehlt bei Marker {list} (liegt er nach dem Ende von video.mkv?)")
        };
        (note, Ok(()))
    }
}

/// Liest `session.json`; fehlt es oder ist es kaputt, ist [`Meta`] leer.
pub(super) fn read_meta(dir: &Path) -> Meta {
    std::fs::read(dir.join(FILE_META))
        .ok()
        .and_then(|raw| Meta::parse(&raw))
        .unwrap_or_default()
}

#[cfg(test)]
#[path = "transcribe_job_tests.rs"]
mod tests;
