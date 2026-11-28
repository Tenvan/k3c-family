//! Attrappen der Auswertung (Port von `trJob`, `trProc` und `trRig` aus `transcribe_test.go`): sie schreiben, was ein
//! echtes ffmpeg schriebe, und merken sich die Aufrufe. Nie ein echtes ffmpeg oder Modell.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::devices::Probe;
use super::record::write_meta;
use super::rig_tests::put;
use super::*;
use crate::testutil::tmp_dir;

pub(crate) const SESSION: &str = "2026-11-22-1";

const GOOD_VERSION: &str = "ffmpeg version 9.0.2-full_build-www.gyan.dev Copyright (c) 2000-2025\n";
const GOOD_FILTERS: &str =
    " ... whisper           A->A       Transcribe audio using Whisper.\n ... volume  A->A  Change volume.\n";
const SEGMENTS: &str = concat!(
    r#"{"start":0,"end":2500,"text":" Hallo Welt."}"#,
    "\n",
    r#"{"start":4000,"end":6000,"text":"  "}"#,
    "\n",
    r#"{"start":9000,"end":11000,"text":"Der Turm fehlt."}"#,
    "\n"
);

#[derive(Default)]
pub(crate) struct Calls {
    list: Mutex<Vec<Vec<String>>>,
    /// Teil eines Aufrufs, bei dem der Prozess scheitert.
    pub fail: Mutex<String>,
    /// Prozesse laufen, bis sie beendet werden.
    pub block: AtomicBool,
}

impl Calls {
    pub(crate) fn lines(&self) -> Vec<String> {
        self.list.lock().unwrap().iter().map(|args| args.join(" ")).collect()
    }
}

struct EvalProc {
    calls: Arc<Calls>,
    args: Vec<String>,
    dir: PathBuf,
    killed: Arc<Signal<()>>,
}

impl Proc for EvalProc {
    fn wait(&self) -> Result<(), String> {
        if self.calls.block.load(Ordering::SeqCst) {
            // Ein hängender Test endet nach 10 s.
            let _ = self.killed.wait_timeout(Duration::from_secs(10));
            return Err("killed".to_owned());
        }
        let line = self.args.join(" ");
        let fail = self.calls.fail.lock().unwrap().clone();
        if !fail.is_empty() && line.contains(&fail) {
            return Err("exit status 1".to_owned());
        }
        let (name, body) = if line.contains("whisper=") {
            ("transcript.part", SEGMENTS)
        } else {
            (self.args.last().unwrap().as_str(), "jpg")
        };
        std::fs::write(self.dir.join(name), body).map_err(|e| e.to_string())
    }

    fn quit(&self) {}

    fn kill(&self) {
        self.killed.set(());
    }

    fn tail(&self) -> String {
        "Fehlerzeile".to_owned()
    }
}

/// Ablage, Modell und Attrappen; die Session hat die Startzeit 1 000 000 ms und zwei Marker.
pub(crate) struct EvalRig {
    pub root: tempfile::TempDir,
    pub dir: PathBuf,
    pub calls: Arc<Calls>,
    pub look_path: LookPath,
    pub out: Probe,
    pub model: String,
}

impl EvalRig {
    pub(crate) fn new(video: bool) -> EvalRig {
        let root = tmp_dir();
        let dir = root.path().join("playtests").join(SESSION);
        std::fs::create_dir_all(&dir).unwrap();
        let meta = Meta {
            capture_ms: 1_000_000,
            start_ms: 1_002_000,
            end_ms: 1_100_000,
            stand: "aufgenommen".to_owned(),
            ..Meta::default()
        };
        write_meta(&dir, &meta);
        put(&dir, "voice.wav", "wav");
        let markers = concat!(
            r#"{"ts":1005000,"kind":"pad","player":1,"island":2,"stage":3}"#,
            "\n",
            r#"{"ts":1020500,"kind":"ruckler","player":0,"frame_ms":48.5}"#,
            "\n"
        );
        put(&dir, "markers.jsonl", markers);
        if video {
            put(&dir, "video.mkv", "mkv");
        }
        let model = root.path().join("models").join("whisper").join("ggml-large-v3.bin");
        put(model.parent().unwrap(), "ggml-large-v3.bin", "m");
        let out: Probe = Arc::new(|args| {
            let filters = args.last().is_some_and(|a| a == "-filters");
            let text = if filters { GOOD_FILTERS } else { GOOD_VERSION };
            (text.to_owned(), Ok(()))
        });
        EvalRig {
            dir,
            calls: Arc::default(),
            look_path: Arc::new(|_| Ok("ffmpeg".to_owned())),
            out,
            model: model.to_string_lossy().into_owned(),
            root,
        }
    }

    pub(crate) fn store(&self) -> Store {
        Store {
            root: self.root.path().join("playtests"),
            active: None,
        }
    }

    pub(crate) fn spawn(&self) -> Spawn {
        let calls = Arc::clone(&self.calls);
        Arc::new(move |args, dir: &Path| {
            calls.list.lock().unwrap().push(args.to_vec());
            Ok(Arc::new(EvalProc {
                calls: Arc::clone(&calls),
                args: args.to_vec(),
                dir: dir.to_path_buf(),
                killed: Signal::new(),
            }) as Arc<dyn Proc>)
        })
    }

    pub(crate) fn transcriber(&self) -> Arc<Transcriber> {
        let (model, root) = (self.model.clone(), self.root.path().to_path_buf());
        Transcriber::new(TranscribeConfig {
            store: self.store(),
            spawn: self.spawn(),
            look_path: Arc::clone(&self.look_path),
            out: Arc::clone(&self.out),
            model: Arc::new(move || model.clone()),
            models: Arc::new(move || list_models(&root, "")),
            log: None,
        })
    }

    /// Eine Auswertung mit den Vorgaben, bis zum Ende.
    pub(crate) fn evaluate(&self) -> Arc<Transcriber> {
        let transcriber = self.transcriber();
        transcriber.start(SESSION, &TranscribeOptions::default()).unwrap();
        transcriber.wait();
        transcriber
    }

    pub(crate) fn text(&self, name: &str) -> String {
        std::fs::read_to_string(self.dir.join(name)).unwrap_or_default()
    }
}
