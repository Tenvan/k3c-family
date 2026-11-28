//! Ausgabe eines Dienstes als JSON-Zeilen nach `logs/<name>.jsonl` (Port von `outlog.go`; beide Workbench-Fassungen
//! schreiben dasselbe Format).

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};

use k3c_kit::applog;
use k3c_kit::logger::{Logger, time_nanos};
use regex::Regex;
use serde::Serialize;
use serde_json::json;

use super::options::Clock;

/// Level einer Ausgabezeile, geschätzt aus dem Text.
pub(crate) fn out_level(text: &str) -> &'static str {
    let low = text.to_lowercase();
    if ["error", "fehler", "fatal"].iter().any(|w| low.contains(w)) {
        "ERROR"
    } else if low.contains("warn") {
        "WARN"
    } else {
        "INFO"
    }
}

/// Entfernt Escape-Sequenzen (Farben, Cursor).
fn without_ansi(text: &str) -> String {
    static ANSI: std::sync::OnceLock<Option<Regex>> = std::sync::OnceLock::new();
    match ANSI.get_or_init(|| Regex::new(r"\x1b\[[0-9;?]*[ -/]*[@-~]").ok()) {
        Some(ansi) => ansi.replace_all(text, "").into_owned(),
        None => text.to_owned(),
    }
}

/// Felder in der Reihenfolge, in der Go eine Map schreibt (nach Namen).
#[derive(Serialize)]
struct Entry<'a> {
    level: &'a str,
    msg: &'a str,
    ns: &'a str,
    stream: &'a str,
    time: String,
}

/// Fehler sind nie fatal: der erste Schreibfehler geht einmal als Warnung ins eigene Log, danach schweigt die Datei.
pub(crate) struct OutSink {
    file: Mutex<Option<File>>,
    name: String,
    log: Logger,
    failed: AtomicBool,
    now: Clock,
}

impl OutSink {
    /// Öffnet `logs/<dienstname klein>.jsonl` unter `root` (nach Rotation ab 10 MB). Ohne `root` oder bei einem Fehler
    /// gibt es keine Datei.
    pub fn open(root: &Path, service: &str, log: &Logger, now: &Clock) -> OutSink {
        let mut file = None;
        if !root.as_os_str().is_empty() {
            let dir = root.join("logs");
            let path = dir.join(format!("{}.jsonl", service.to_lowercase()));
            let opened = std::fs::create_dir_all(&dir)
                .and_then(|()| applog::rotate(&path, applog::MAX_FILE_SIZE))
                .and_then(|()| OpenOptions::new().create(true).append(true).open(&path));
            match opened {
                Ok(f) => file = Some(f),
                Err(e) => log.warn(
                    &format!("📄 dienst {service}: Ausgabe-Log nicht geöffnet: {e}"),
                    &[("ns", json!("svc")), ("datei", json!(path.display().to_string()))],
                ),
            }
        }
        OutSink {
            file: Mutex::new(file),
            name: service.to_owned(),
            log: log.clone(),
            failed: AtomicBool::new(false),
            now: now.clone(),
        }
    }

    fn file(&self) -> MutexGuard<'_, Option<File>> {
        self.file.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn write(&self, stream: &str, text: &str) {
        let mut file = self.file();
        let Some(f) = file.as_mut() else { return };
        let text = without_ansi(text);
        let entry = Entry {
            level: out_level(&text),
            msg: &text,
            ns: "out",
            stream,
            time: time_nanos(&(self.now)()),
        };
        // Go maskiert <, > und &; sie kommen in JSON nur in Texten vor.
        let written = serde_json::to_string(&entry)
            .map_err(std::io::Error::from)
            .and_then(|line| {
                let line = line
                    .replace('<', "\\u003c")
                    .replace('>', "\\u003e")
                    .replace('&', "\\u0026");
                f.write_all(format!("{line}\n").as_bytes())
            });
        if let Err(e) = written
            && !self.failed.swap(true, Ordering::SeqCst)
        {
            let message = format!("📄 dienst {}: Ausgabe-Log nicht beschreibbar: {e}", self.name);
            self.log.warn(&message, &[("ns", json!("svc"))]);
        }
    }

    pub fn close(&self) {
        drop(self.file().take());
    }
}
