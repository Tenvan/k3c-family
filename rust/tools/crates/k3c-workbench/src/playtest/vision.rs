//! Bildbeschreibung (Port von `vision.go`, B-1241): ein lokales Vision-Modell (llama.cpp, `llama-mtmd-cli`) beschreibt
//! die ausgewählten Marker-Bilder vorab, damit der Chat sie nicht ansehen muss. Modell und Laufzeit liegen in
//! `models/vision/` (ignoriert, nie automatisch geladen); das Ergebnis steht in `descriptions.json` der Session.

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::Instant;

use regex::Regex;
use serde::Serialize;
use serde_json::{Value, json};

use super::timeline::{FILE_DESCRIPTIONS, read_descriptions};
use super::transcribe::frame_name;
use super::vision_exec::VisionRun;
use super::{Error, FrameDesc, LogFn, Signal, Store, go_json, lock};
use crate::services::Cancel;

#[cfg(test)]
#[path = "vision_tests.rs"]
mod tests;

const VISION_DIR: &str = "models/vision";
/// Ordner mit `llama-mtmd-cli`; sonst `models/vision/llama`.
pub const VISION_ENV: &str = "K3C_LLAMA_DIR";
const VISION_EXE: &str = "llama-mtmd-cli.exe";
/// Kurz, deutsch, ohne Erfindungen.
const VISION_PROMPT: &str = "Beschreibe diesen Screenshot aus dem Side-Scroller-Spiel Family Three Crowns in zwei bis \
    vier Sätzen auf Deutsch: sichtbare Texte, Menüs mit Werten, Gebäude, Figuren, Ressourcen und Auffälligkeiten. \
    Erfinde nichts, was nicht zu sehen ist.";

const BUSY: &str = "Bildbeschreibung läuft schon";
const NOT_RUNNING: &str = "keine Bildbeschreibung aktiv";
const NO_RUNTIME: &str = "llama-mtmd-cli.exe fehlt: llama.cpp (Vulkan-Build) nach models/vision/llama/ entpacken oder \
    K3C_LLAMA_DIR setzen";
const NO_MODEL: &str = "kein Bildmodell in models/vision/ (Hauptdatei und mmproj-Datei, z. B. Qwen2.5-VL-3B)";

static QUANT: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"(?i)-(q[0-9].*|f16|f32|bf16)\.gguf$").ok());
/// Denkblöcke, die Qwen3.5 leer voranstellt.
static THINK: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"(?s)<think>.*?</think>").ok());

/// Ein Bildmodell (Hauptdatei samt mmproj); `id` ist der Pfad relativ zur Repo-Wurzel.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct VisionModel {
    pub id: String,
    pub name: String,
    #[serde(rename = "sizeMb")]
    pub size_mb: u64,
    #[serde(skip)]
    pub path: String,
    #[serde(skip)]
    pub mmproj: String,
}

/// Stand der Bildbeschreibung für die Seite.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct VisionStatus {
    pub running: bool,
    pub id: String,
    pub done: usize,
    pub total: usize,
    pub error: String,
}

/// Welche Bildmodelle es gibt und ob die Laufzeit bereitsteht.
#[derive(Clone, Debug, Default, Serialize)]
pub struct VisionInfo {
    pub ready: bool,
    pub models: Vec<VisionModel>,
    pub message: String,
}

fn under(root: &Path, relative: &str) -> PathBuf {
    relative
        .split('/')
        .fold(root.to_path_buf(), |path, part| path.join(part))
}

/// Dateinamen eines Ordners, sortiert wie Go `Glob`; ohne Ordner leer.
fn file_names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    names.sort();
    names
}

/// Die Dateien `models/vision/*.gguf` (ohne `mmproj-*`), zu denen es eine passende mmproj-Datei gibt.
pub fn list_vision_models(root: &Path) -> Vec<VisionModel> {
    let dir = under(root, VISION_DIR);
    let names = file_names(&dir);
    let gguf: Vec<&String> = names.iter().filter(|n| n.ends_with(".gguf")).collect();
    let mut out = Vec::new();
    for name in &gguf {
        if name.starts_with("mmproj") {
            continue;
        }
        let base = QUANT
            .as_ref()
            .map_or_else(|| name.as_str().into(), |re| re.replace_all(name, ""));
        let prefix = format!("mmproj-{base}");
        let projector = gguf
            .iter()
            .find(|n| n.strip_prefix(&prefix).is_some_and(|rest| rest.ends_with(".gguf")));
        let Some(projector) = projector else { continue };
        let path = dir.join(name);
        let Ok(meta) = std::fs::metadata(&path) else { continue };
        out.push(VisionModel {
            id: format!("{VISION_DIR}/{name}"),
            name: (*name).clone(),
            size_mb: meta.len() >> 20,
            path: path.to_string_lossy().into_owned(),
            mmproj: dir.join(projector).to_string_lossy().into_owned(),
        });
    }
    out
}

/// `llama-mtmd-cli.exe` im Ordner `env` (Wert von [`VISION_ENV`]) oder `models/vision/llama`; leer, wenn sie fehlt.
pub fn vision_runtime(root: &Path, env: &str) -> String {
    let dir = match env.trim() {
        "" => under(root, VISION_DIR).join("llama"),
        dir => PathBuf::from(dir),
    };
    let path = dir.join(VISION_EXE);
    if path.is_file() {
        path.to_string_lossy().into_owned()
    } else {
        String::new()
    }
}

/// Der Aufruf für ein Bild (absolute Pfade); `-ngl 99` nimmt die Grafikkarte, ohne sie rechnet die Laufzeit auf der CPU.
pub fn vision_args(exe: &str, model: &VisionModel, image: &str) -> Vec<String> {
    [
        exe,
        "-m",
        &model.path,
        "--mmproj",
        &model.mmproj,
        "--image",
        image,
        "-p",
        VISION_PROMPT,
        "-n",
        "300",
        "--temp",
        "0.1",
        "-ngl",
        "99",
    ]
    .map(str::to_owned)
    .to_vec()
}

/// Schneidet die Ausgabe der Laufzeit auf den Text.
fn clean_vision(out: &str) -> String {
    let text = out.replace("\r\n", "\n");
    let text = THINK
        .as_ref()
        .map_or_else(|| text.as_str().into(), |re| re.replace_all(&text, ""));
    text.trim().to_owned()
}

/// Wie Go `writeDescriptions`: über eine Nebendatei, Schlüssel als Text.
fn write_descriptions(dir: &Path, descriptions: BTreeMap<i64, FrameDesc>) -> Result<(), String> {
    let map: BTreeMap<String, FrameDesc> = descriptions.into_iter().map(|(n, d)| (n.to_string(), d)).collect();
    let raw = go_json(&map).map_err(|e| e.to_string())?;
    let tmp = dir.join(format!("{FILE_DESCRIPTIONS}.tmp"));
    std::fs::write(&tmp, raw).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, dir.join(FILE_DESCRIPTIONS)).map_err(|e| e.to_string())
}

/// Werkzeuge und Ablage der Bildbeschreibung.
pub struct VisionConfig {
    pub store: Store,
    pub models: Arc<dyn Fn() -> Vec<VisionModel> + Send + Sync>,
    /// Pfad der Laufzeit (leer: fehlt).
    pub run_exe: Arc<dyn Fn() -> String + Send + Sync>,
    pub run: VisionRun,
    pub log: Option<LogFn>,
}

#[derive(Default)]
struct State {
    status: VisionStatus,
    cancel: Option<Cancel>,
    done: Option<Arc<Signal<()>>>,
}

/// Ein Auftrag mit allem, was der Hintergrundfaden braucht.
struct Job {
    id: String,
    exe: String,
    model: VisionModel,
    todo: Vec<i64>,
    cancel: Cancel,
}

/// Steuert höchstens eine Bildbeschreibung.
pub struct Describer {
    config: VisionConfig,
    state: Mutex<State>,
}

impl Describer {
    pub fn new(config: VisionConfig) -> Arc<Describer> {
        Arc::new(Describer {
            config,
            state: Mutex::default(),
        })
    }

    fn log(&self, level: &str, message: &str, fields: &[(&str, Value)]) {
        if let Some(log) = &self.config.log {
            log(level, message, fields);
        }
    }

    /// Prüft Laufzeit und Modelle.
    pub fn info(&self) -> VisionInfo {
        let models = (self.config.models)();
        let message = if (self.config.run_exe)().is_empty() {
            NO_RUNTIME
        } else if models.is_empty() {
            NO_MODEL
        } else {
            ""
        };
        VisionInfo {
            ready: message.is_empty(),
            models,
            message: message.to_owned(),
        }
    }

    pub fn status(&self) -> VisionStatus {
        lock(&self.state).status.clone()
    }

    /// Bricht die Beschreibung ab; schon fertige Beschreibungen bleiben.
    pub fn cancel(&self) -> Result<(), Error> {
        let state = lock(&self.state);
        match &state.cancel {
            Some(cancel) if state.status.running => {
                cancel.cancel();
                Ok(())
            }
            _ => Err(Error::Other(NOT_RUNNING.to_owned())),
        }
    }

    #[cfg(test)]
    pub(crate) fn wait(&self) {
        let done = lock(&self.state).done.clone();
        if let Some(done) = done {
            let ended = done.wait_timeout(std::time::Duration::from_secs(10));
            assert!(ended.is_some(), "Bildbeschreibung endet nicht");
        }
    }

    fn pick(&self, id: &str) -> Result<VisionModel, Error> {
        let list = (self.config.models)();
        let Some(first) = list.first() else {
            return Err(Error::Other(NO_MODEL.to_owned()));
        };
        if id.is_empty() {
            return Ok(first.clone());
        }
        list.iter().find(|m| m.id == id).cloned().ok_or_else(|| {
            let shown = crate::planning::store::quoted(id);
            Error::Other(format!(
                "unbekanntes Bildmodell {shown}: nur Dateien aus models/vision/"
            ))
        })
    }

    /// Die ausgewählten Marker-Bilder aus `numbers` (leer: alle); abgewählte nie.
    fn selected(&self, id: &str, numbers: &[i64]) -> Result<Vec<i64>, Error> {
        let wanted: HashSet<i64> = numbers.iter().copied().collect();
        let todo: Vec<i64> = self
            .config
            .store
            .timeline(id)?
            .entries
            .iter()
            .filter(|e| e.kind == "marker" && !e.frame.is_empty() && !e.skip)
            .filter(|e| wanted.is_empty() || wanted.contains(&e.nr))
            .map(|e| e.nr)
            .collect();
        if todo.is_empty() {
            return Err(Error::Other(
                "kein ausgewähltes Marker-Bild zum Beschreiben (erst auswerten, Haken prüfen)".to_owned(),
            ));
        }
        Ok(todo)
    }

    /// Beschreibt die Marker-Bilder `numbers` der Session (leer: alle ausgewählten) im Hintergrund mit dem Modell
    /// `model` (ID aus [`list_vision_models`], leer: das erste).
    pub fn start(self: &Arc<Self>, id: &str, model: &str, numbers: &[i64]) -> Result<VisionStatus, Error> {
        if self.config.store.get(id)?.running {
            return Err(Error::Running);
        }
        let exe = (self.config.run_exe)();
        if exe.is_empty() {
            return Err(Error::Other(NO_RUNTIME.to_owned()));
        }
        let model = self.pick(model)?;
        let todo = self.selected(id, numbers)?;
        let mut state = lock(&self.state);
        if state.status.running {
            return Err(Error::Other(BUSY.to_owned()));
        }
        let cancel = Cancel::default();
        let done = Signal::new();
        state.cancel = Some(cancel.clone());
        state.done = Some(Arc::clone(&done));
        state.status = VisionStatus {
            running: true,
            id: id.to_owned(),
            total: todo.len(),
            ..VisionStatus::default()
        };
        let fields = [
            ("id", json!(id)),
            ("bilder", json!(todo.len())),
            ("modell", json!(model.name)),
        ];
        self.log("info", "🚀 Bildbeschreibung gestartet", &fields);
        let job = Job {
            id: id.to_owned(),
            exe,
            model,
            todo,
            cancel,
        };
        let this = Arc::clone(self);
        let spawned = std::thread::Builder::new()
            .name("spieltest-bilder".to_owned())
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

    /// Beschreibt ein Bild und trägt es ein; ein Abbruch geht vor dem Fehler der Laufzeit.
    fn describe(&self, job: &Job, nr: i64) -> Result<(), Error> {
        let dir = self.config.store.root.join(&job.id);
        let image = under(&dir, &frame_name(nr)).to_string_lossy().into_owned();
        let start = Instant::now();
        let out = (self.config.run)(&vision_args(&job.exe, &job.model, &image), &job.cancel);
        if job.cancel.is_cancelled() {
            return Err(Error::Canceled);
        }
        let out = out.map_err(Error::Other)?;
        // Frisch lesen: andere Nummern bleiben erhalten.
        let mut descriptions = read_descriptions(&dir);
        let description = FrameDesc {
            text: clean_vision(&out),
            model: job.model.name.clone(),
            ms: i64::try_from(start.elapsed().as_millis()).unwrap_or(i64::MAX),
        };
        descriptions.insert(nr, description);
        write_descriptions(&dir, descriptions).map_err(Error::Other)
    }

    fn run(&self, job: &Job) {
        let result = job.todo.iter().enumerate().try_for_each(|(index, nr)| {
            self.describe(job, *nr)?;
            lock(&self.state).status.done = index + 1;
            Ok::<(), Error>(())
        });
        {
            let mut state = lock(&self.state);
            state.status.running = false;
            if let Err(e) = &result
                && *e != Error::Canceled
            {
                state.status.error = e.to_string();
            }
        }
        let fields = [("id", json!(job.id))];
        match result {
            Err(Error::Canceled) => self.log("info", "🛑 Bildbeschreibung abgebrochen", &fields),
            Err(e) => self.log("warn", &format!("❌ Bildbeschreibung fehlgeschlagen: {e}"), &fields),
            Ok(()) => self.log("info", "✅ Bildbeschreibung fertig", &fields),
        }
    }
}
