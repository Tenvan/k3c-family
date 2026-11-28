//! Die Commands der Aufnahme und Auswertung auf der Seite Spieltest (Port von `app_playtest.go`, B-1218). Der
//! Ordner-Vertrag steht in [`crate::playtest`]; die Methoden reichen nur durch und melden ins Log. Die Player- und
//! Marker-Dateien liefert [`super::playtest_media`].

use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::runtime::Handle;

use super::services::STOPPED;
use super::{App, lock};
use crate::playtest::{
    self, Active, Capability, Config, Describer, Devices, Error, Finding, FindingPatch, JobStatus, LogFn, LookPath,
    ModelInfo, Options, PickCache, Plan, Probe, Recorder, Session, Spawn, Status, Store, Timeline, TranscribeConfig,
    TranscribeOptions, Transcriber, VisionConfig, VisionInfo, VisionStatus,
};

/// Fristen der kurzen ffmpeg-Aufrufe: Probe, Geräteliste, Prüfung der Auswertung.
const PROBE_TIMEOUT: Duration = Duration::from_secs(8);
const LIST_TIMEOUT: Duration = Duration::from_secs(15);
const CHECK_TIMEOUT: Duration = Duration::from_secs(30);

/// Alles mit Wirkung nach außen (Prozesse, Suche, Uhr); Tests setzen Attrappen ein.
#[derive(Clone)]
pub(crate) struct Tools {
    pub spawn: Spawn,
    pub look_path: LookPath,
    pub game: Arc<dyn Fn() -> Result<(String, std::path::PathBuf), Error> + Send + Sync>,
    /// Geräteprobe (Mikro, Monitor, Encoder).
    pub probe: Probe,
    /// Geräteliste.
    pub list: Probe,
    /// Prüfung von ffmpeg für die Auswertung.
    pub out: Probe,
    /// Modelldatei der Auswertung.
    pub model: Arc<dyn Fn() -> String + Send + Sync>,
}

impl Tools {
    fn real(root: &Path, runtime: &Handle) -> Tools {
        let run = |limit: Duration| -> Probe {
            let runtime = runtime.clone();
            Arc::new(move |args| playtest::run_output(&runtime, args, limit))
        };
        let models = root.to_path_buf();
        Tools {
            spawn: playtest::exec_spawn(runtime.clone()),
            look_path: Arc::new(playtest::find_ffmpeg),
            game: playtest::game_finder(root),
            probe: run(PROBE_TIMEOUT),
            list: run(LIST_TIMEOUT),
            out: run(CHECK_TIMEOUT),
            model: Arc::new(move || {
                let env = std::env::var(playtest::MODEL_ENV).unwrap_or_default();
                playtest::model_path(&models, &env)
            }),
        }
    }
}

/// Zustand der Seite Spieltest: höchstens ein Recorder und eine Auswertung je App.
#[derive(Default)]
pub(super) struct PlaytestSeat {
    /// Kennung der laufenden Aufnahme; die Ablage fragt sie, auch bevor es einen Recorder gibt.
    active: Arc<Mutex<String>>,
    cache: PickCache,
    tools: OnceLock<Tools>,
    recorder: OnceLock<Arc<Recorder>>,
    transcriber: OnceLock<Arc<Transcriber>>,
    describer: OnceLock<Arc<Describer>>,
}

impl App {
    /// Kennung der laufenden Aufnahme (auch für die Ablage der MCP-Tools).
    pub(super) fn playtest_active(&self) -> Active {
        let active = Arc::clone(&self.playtest.active);
        Arc::new(move || active.lock().unwrap_or_else(PoisonError::into_inner).clone())
    }

    pub(super) fn playtest_store(&self) -> Store {
        Store {
            root: self.root.join("playtests"),
            active: Some(self.playtest_active()),
        }
    }

    fn playtest_tools(&self) -> Result<&Tools, String> {
        if let Some(tools) = self.playtest.tools.get() {
            return Ok(tools);
        }
        let runtime = lock(&self.running)
            .as_ref()
            .map(|r| r.runtime.clone())
            .ok_or_else(|| STOPPED.to_owned())?;
        Ok(self.playtest.tools.get_or_init(|| Tools::real(&self.root, &runtime)))
    }

    /// Ersetzt Prozesse und Suche, solange noch nichts benutzt wurde.
    #[cfg(test)]
    pub(crate) fn set_playtest_tools(&self, tools: Tools) {
        assert!(self.playtest.tools.set(tools).is_ok(), "Werkzeuge schon gesetzt");
    }

    fn playtest_log(&self) -> LogFn {
        self.namespace_log("main")
    }

    /// Meldungen der Spieltest-Module unter dem Thema `ns`.
    fn namespace_log(&self, ns: &'static str) -> LogFn {
        let logger = self.logger();
        Arc::new(move |level, message, fields| {
            let mut all: Vec<(&str, Value)> = vec![("ns", json!(ns))];
            all.extend_from_slice(fields);
            if level == "warn" {
                logger.warn(message, &all);
            } else {
                logger.info(message, &all);
            }
        })
    }

    fn recorder(&self) -> Result<&Arc<Recorder>, String> {
        if let Some(recorder) = self.playtest.recorder.get() {
            return Ok(recorder);
        }
        let tools = self.playtest_tools()?.clone();
        let log = self.playtest_log();
        let (probe, devices, cache, picked) = (
            Arc::clone(&tools.probe),
            Arc::clone(&tools.probe),
            self.playtest.cache.clone(),
            Arc::clone(&log),
        );
        let config = Config {
            store: self.playtest_store(),
            spawn: tools.spawn,
            look_path: tools.look_path,
            check: Some(Arc::new(move |ffmpeg: &str, o: &Options| {
                let monitor = if o.voice_only { -1 } else { o.monitor };
                playtest::check_devices(&probe, ffmpeg, &o.mic, monitor)
            })),
            pick: Some(Arc::new(move |ffmpeg: &str| {
                cache.pick(&devices, ffmpeg, Some(&picked))
            })),
            ready: Duration::ZERO,
            interval: Duration::ZERO,
            game: tools.game,
            now: Arc::new(|| chrono::Local::now().fixed_offset()),
            log: Some(log),
            settle: Duration::from_secs(2),
            quit_wait: Duration::from_secs(10),
        };
        let active = Arc::clone(&self.playtest.active);
        Ok(self.playtest.recorder.get_or_init(|| Recorder::new(config, active)))
    }

    /// Die Sessions, neueste zuerst.
    pub fn playtest_list(&self) -> Result<Vec<Session>, String> {
        self.wait();
        let logger = self.logger();
        match self.playtest_store().list() {
            Ok(list) => {
                let fields = [("ns", json!("main")), ("anzahl", json!(list.len()))];
                logger.info("📂 Spieltests geladen", &fields);
                Ok(list)
            }
            Err(e) => {
                logger.warn(&format!("❌ Spieltests nicht gelesen: {e}"), &[("ns", json!("main"))]);
                Err(e.to_string())
            }
        }
    }

    /// Was die Aktion (video, session, ausgewertet) löschen würde; es wird nichts gelöscht.
    pub fn playtest_plan(&self, action: &str, id: &str) -> Result<Plan, String> {
        self.wait();
        self.playtest_store().plan(action, id).map_err(|e| e.to_string())
    }

    /// Führt die Aktion aus, aber nur mit `confirm` (die Oberfläche fragt vorher mit [`App::playtest_plan`]).
    pub fn playtest_delete(&self, action: &str, id: &str, confirm: bool) -> Result<Plan, String> {
        self.wait();
        let logger = self.logger();
        let fields = [("ns", json!("main")), ("aktion", json!(action)), ("id", json!(id))];
        match self.playtest_store().delete(action, id, confirm) {
            Ok(plan) => {
                let mut fields = fields.to_vec();
                fields.extend([("dateien", json!(plan.files.len())), ("bytes", json!(plan.total))]);
                logger.info("🧹 Spieltests aufgeräumt", &fields);
                Ok(plan)
            }
            Err(e) => {
                if e != Error::NotConfirmed {
                    logger.warn(&format!("❌ Aufräumen der Spieltests fehlgeschlagen: {e}"), &fields);
                }
                Err(e.to_string())
            }
        }
    }

    /// Mikrofone, aufnehmbare Monitore (Probebild) und ob ffmpeg gefunden wird.
    pub fn playtest_devices(&self) -> Result<Devices, String> {
        self.wait();
        let tools = self.playtest_tools()?;
        Ok(playtest::list_devices(
            &tools.look_path,
            &tools.list,
            &tools.probe,
            &self.playtest.cache,
        ))
    }

    /// Nimmt Mikro und Bild auf und startet das Spiel mit `--playtest`.
    pub fn playtest_start(&self, options: &Options) -> Result<Status, String> {
        self.wait();
        self.recorder()?.start(options).map_err(|e| e.to_string())
    }

    /// Beendet das Spiel; die Aufnahme endet danach wie bei Spielende.
    pub fn playtest_stop(&self) -> Result<(), String> {
        self.wait();
        let recorder = self
            .playtest
            .recorder
            .get()
            .ok_or_else(|| Error::NotRunning.to_string())?;
        recorder.stop().map_err(|e| e.to_string())
    }

    /// Stand der Aufnahme.
    pub fn playtest_status(&self) -> Status {
        self.wait();
        self.playtest
            .recorder
            .get()
            .map_or_else(Status::default, |r| r.status())
    }

    /// Ist die Auswertung möglich (ffmpeg ab 8 mit whisper-Filter, Modelldatei), sonst warum nicht.
    pub fn playtest_check(&self) -> Result<Capability, String> {
        self.wait();
        let tools = self.playtest_tools()?;
        Ok(playtest::check(&tools.look_path, &tools.out, &(tools.model)()))
    }

    fn transcriber(&self) -> Result<&Arc<Transcriber>, String> {
        if let Some(transcriber) = self.playtest.transcriber.get() {
            return Ok(transcriber);
        }
        let tools = self.playtest_tools()?.clone();
        let root = self.root.clone();
        let config = TranscribeConfig {
            store: self.playtest_store(),
            spawn: tools.spawn,
            look_path: tools.look_path,
            out: tools.out,
            model: tools.model,
            models: Arc::new(move || models(&root)),
            log: Some(self.playtest_log()),
        };
        Ok(self.playtest.transcriber.get_or_init(|| Transcriber::new(config)))
    }

    /// Wertet eine Session im Hintergrund aus: Transkript aus `voice.wav`, ein Bild je Marker.
    pub fn playtest_transcribe(&self, id: &str, options: &TranscribeOptions) -> Result<JobStatus, String> {
        self.wait();
        self.transcriber()?.start(id, options).map_err(|e| e.to_string())
    }

    /// Die wählbaren Modelldateien (`K3C_WHISPER_MODEL`, `models/`, `models/whisper/`) mit Größe.
    pub fn playtest_models(&self) -> Vec<ModelInfo> {
        self.wait();
        models(&self.root)
    }

    /// Stand der Auswertung.
    pub fn playtest_transcribe_status(&self) -> JobStatus {
        self.wait();
        self.playtest
            .transcriber
            .get()
            .map_or_else(JobStatus::default, |t| t.status())
    }

    /// Bricht die Auswertung ab.
    pub fn playtest_transcribe_cancel(&self) -> Result<(), String> {
        self.wait();
        let transcriber = self
            .playtest
            .transcriber
            .get()
            .ok_or_else(|| Error::NoJob.to_string())?;
        transcriber.cancel().map_err(|e| e.to_string())
    }

    /// Zeitleiste einer Session: Sätze, Marker und deren Bilder.
    pub fn playtest_timeline(&self, id: &str) -> Result<Timeline, String> {
        self.wait();
        self.playtest_store().timeline(id).map_err(|e| e.to_string())
    }

    /// Die Befunde einer Session (`findings.json`).
    pub fn playtest_findings(&self, id: &str) -> Result<Vec<Finding>, String> {
        self.wait();
        self.playtest_store().findings(id).map_err(|e| e.to_string())
    }

    /// Setzt Haken und Vorschlag (Domäne, Typ, Prio, Projekt, Sprint) eines Befunds.
    pub fn playtest_finding_update(&self, id: &str, finding: &str, patch: &FindingPatch) -> Result<Finding, String> {
        self.wait();
        self.playtest_store()
            .update_finding(id, finding, patch)
            .map_err(|e| e.to_string())
    }

    /// Blendet das Bild eines Markers aus oder ein (die Datei bleibt).
    pub fn playtest_frame_skip(&self, id: &str, nr: i64, skip: bool) -> Result<(), String> {
        self.wait();
        self.playtest_store()
            .set_frame_skip(id, nr, skip)
            .map_err(|e| e.to_string())
    }
}

impl App {
    fn describer(&self) -> Result<&Arc<Describer>, String> {
        if let Some(describer) = self.playtest.describer.get() {
            return Ok(describer);
        }
        let runtime = lock(&self.running)
            .as_ref()
            .map(|r| r.runtime.clone())
            .ok_or_else(|| STOPPED.to_owned())?;
        let (models, exe) = (self.root.clone(), self.root.clone());
        let config = VisionConfig {
            store: self.playtest_store(),
            models: Arc::new(move || playtest::list_vision_models(&models)),
            run_exe: Arc::new(move || {
                let env = std::env::var(playtest::VISION_ENV).unwrap_or_default();
                playtest::vision_runtime(&exe, &env)
            }),
            run: playtest::exec_vision(runtime),
            log: Some(self.namespace_log("playtest")),
        };
        Ok(self.playtest.describer.get_or_init(|| Describer::new(config)))
    }

    /// Die Bildmodelle (`models/vision/`) und ob die Laufzeit llama.cpp bereitsteht.
    pub fn playtest_vision_info(&self) -> Result<VisionInfo, String> {
        self.wait();
        Ok(self.describer()?.info())
    }

    /// Beschreibt die Marker-Bilder (leer: alle ausgewählten) mit dem Bildmodell im Hintergrund.
    pub fn playtest_describe(&self, id: &str, model: &str, numbers: &[i64]) -> Result<VisionStatus, String> {
        self.wait();
        self.describer()?.start(id, model, numbers).map_err(|e| e.to_string())
    }

    /// Stand der Bildbeschreibung.
    pub fn playtest_describe_status(&self) -> VisionStatus {
        self.wait();
        self.playtest
            .describer
            .get()
            .map_or_else(VisionStatus::default, |d| d.status())
    }

    /// Bricht die Bildbeschreibung ab; fertige Beschreibungen bleiben.
    pub fn playtest_describe_cancel(&self) -> Result<(), String> {
        self.wait();
        self.describer()?.cancel().map_err(|e| e.to_string())
    }
}

/// Die wählbaren Modelle der Wurzel, mit `K3C_WHISPER_MODEL` aus der Umgebung.
fn models(root: &Path) -> Vec<ModelInfo> {
    let env = std::env::var(playtest::MODEL_ENV).unwrap_or_default();
    playtest::list_models(root, &env)
}

#[cfg(test)]
#[path = "playtest_tests.rs"]
mod tests;
