//! Eigenes Log eines Werkzeugs (Port von `workkit/applog`, B-046): JSON nach `logs/<quelle>.jsonl` im Format von slog
//! `JSONHandler`, das die Workbench liest, und dieselben Einträge lesbar als Strom `log` im Konsolenpuffer.
//! [`open`] nimmt die Quelle `k3c-workbench`, [`open_named`] eine eigene (z. B. `k3c-studio`).

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, PoisonError};

use serde_json::Value;

use crate::conlog::ConsoleSink;
use crate::console::{LineWriter, Store};
use crate::logger::{Level, Logger, Record, Sink, time_nanos};

/// Name der Log-Datei (ohne `.jsonl`) und der Konsolen-Quelle der Workbench ([`open`]).
pub const SOURCE: &str = "k3c-workbench";
/// Umgebungsvariable für die Log-Stufe (debug|info|warn|error); gesetzt gilt sie für Datei und Konsole.
pub const LEVEL_ENV: &str = "K3C_DEV_LOG_LEVEL";
/// Größe, ab der eine Log-Datei beim Öffnen rotiert wird.
pub const MAX_FILE_SIZE: u64 = 10 << 20;

/// Liest debug|info|warn|warning|error (Groß-/Kleinschreibung und Leerraum egal); `None` bei allem anderen.
pub fn parse_level(s: &str) -> Option<Level> {
    match s.trim().to_lowercase().as_str() {
        "debug" => Some(Level::Debug),
        "info" => Some(Level::Info),
        "warn" | "warning" => Some(Level::Warn),
        "error" => Some(Level::Error),
        _ => None,
    }
}

/// Stufen für Datei und Konsolen-Spiegel: Standard debug und info, `K3C_DEV_LOG_LEVEL` setzt beide.
pub fn levels() -> (Level, Level) {
    levels_from(std::env::var(LEVEL_ENV).ok().as_deref())
}

/// [`levels`] mit dem Wert der Umgebungsvariable als Argument (Tests ohne `set_var`).
pub(crate) fn levels_from(env: Option<&str>) -> (Level, Level) {
    match env.and_then(parse_level) {
        Some(l) => (l, l),
        None => (Level::Debug, Level::Info),
    }
}

/// Benennt `path` nach `path.1` um (ein altes `.1` wird überschrieben), wenn die Datei größer als `max` ist. Eine
/// fehlende Datei ist kein Fehler.
pub fn rotate(path: &Path, max: u64) -> io::Result<()> {
    match std::fs::metadata(path) {
        Ok(st) if st.len() > max => {}
        _ => return Ok(()),
    }
    let mut one = path.as_os_str().to_owned();
    one.push(".1");
    let _ = std::fs::remove_file(&one);
    std::fs::rename(path, &one)
}

/// Logger samt offener Datei und Konsolen-Spiegel.
pub struct Log {
    logger: Logger,
    file: Option<Arc<Mutex<Option<File>>>>,
    mirror: LineWriter,
}

impl Log {
    /// Der Logger (günstig zu klonen).
    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    /// Gibt eine angefangene Spiegelzeile weiter und schließt die Datei; spätere Einträge gehen nur noch in den Spiegel.
    pub fn close(&self) -> io::Result<()> {
        self.mirror.flush_line();
        let Some(file) = &self.file else { return Ok(()) };
        let taken = file.lock().unwrap_or_else(PoisonError::into_inner).take();
        match taken {
            Some(mut f) => f.flush(),
            None => Ok(()),
        }
    }
}

/// Öffnet `logs/k3c-workbench.jsonl` unter `logs_dir` zum Anhängen ([`open_named`] mit [`SOURCE`]).
pub fn open(logs_dir: &Path, store: Arc<Store>) -> (Log, Option<io::Error>) {
    open_named(logs_dir, SOURCE, store)
}

/// Öffnet `<source>.jsonl` unter `logs_dir` zum Anhängen und spiegelt in die Konsolen-Quelle `source`. Lässt sich die
/// Datei nicht öffnen, schreibt der Logger nur in den Konsolenpuffer, und der Fehler kommt zurück, damit der Aufrufer
/// ihn melden kann.
pub fn open_named(logs_dir: &Path, source: &str, store: Arc<Store>) -> (Log, Option<io::Error>) {
    open_with(logs_dir, source, store, levels())
}

/// [`open_named`] mit festen Stufen (Datei, Spiegel).
pub(crate) fn open_with(
    logs_dir: &Path,
    source: &str,
    store: Arc<Store>,
    (file_level, mirror_level): (Level, Level),
) -> (Log, Option<io::Error>) {
    let src = source.to_string();
    let mirror = LineWriter::new(move |text| store.add(&src, "log", &text));
    // Farben immer: die Oberfläche zeichnet sie, console_tail entfernt sie.
    let mut sinks: Vec<Box<dyn Sink>> = vec![Box::new(ConsoleSink::new(Box::new(mirror.clone()), mirror_level, true))];
    let opened = std::fs::create_dir_all(logs_dir).and_then(|()| {
        let path = logs_dir.join(format!("{source}.jsonl"));
        rotate(&path, MAX_FILE_SIZE)?;
        OpenOptions::new().create(true).append(true).open(&path)
    });
    let (file, err) = match opened {
        Ok(f) => {
            let shared = Arc::new(Mutex::new(Some(f)));
            sinks.push(Box::new(JsonSink {
                file: Arc::clone(&shared),
                level: file_level,
            }));
            (Some(shared), None)
        }
        Err(e) => (None, Some(e)),
    };
    (
        Log {
            logger: Logger::new(sinks),
            file,
            mirror,
        },
        err,
    )
}

/// JSON-Zeilen wie slog `JSONHandler`: `{"time":…,"level":…,"msg":…,<attrs…>}` in fester Reihenfolge.
struct JsonSink {
    file: Arc<Mutex<Option<File>>>,
    level: Level,
}

impl Sink for JsonSink {
    fn enabled(&self, level: Level) -> bool {
        level >= self.level
    }

    fn handle(&self, r: &Record) -> io::Result<()> {
        let line = json_line(r);
        let mut g = self.file.lock().unwrap_or_else(PoisonError::into_inner);
        match g.as_mut() {
            Some(f) => f.write_all(line.as_bytes()),
            None => Err(io::Error::other("Log-Datei geschlossen")),
        }
    }
}

/// Eine JSON-Zeile samt Zeilenende.
pub(crate) fn json_line(r: &Record) -> String {
    let mut b = String::from("{\"time\":");
    json_string(&mut b, &time_nanos(&r.time));
    b.push_str(",\"level\":");
    json_string(&mut b, r.level.name());
    b.push_str(",\"msg\":");
    json_string(&mut b, &r.msg);
    // Leere Attribute (ohne Schlüssel und Wert) fallen weg wie bei slog.
    for (k, v) in r.attrs.iter().filter(|(k, v)| !(k.is_empty() && v.is_null())) {
        b.push(',');
        json_string(&mut b, k);
        b.push(':');
        json_value(&mut b, v);
    }
    b.push_str("}\n");
    b
}

/// Schreibt einen JSON-Wert; Zahlen wie Go `encoding/json`, Objekte mit sortierten Schlüsseln wie `json.Marshal`.
fn json_value(b: &mut String, v: &Value) {
    match v {
        Value::Null => b.push_str("null"),
        Value::Bool(x) => b.push_str(if *x { "true" } else { "false" }),
        Value::String(s) => json_string(b, s),
        Value::Number(n) => match (n.as_i64(), n.as_u64(), n.as_f64()) {
            (Some(i), _, _) => b.push_str(&i.to_string()),
            (_, Some(u), _) => b.push_str(&u.to_string()),
            (_, _, Some(f)) => b.push_str(&go_json_float(f)),
            _ => b.push_str(&n.to_string()),
        },
        Value::Array(items) => {
            b.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    b.push(',');
                }
                json_value(b, item);
            }
            b.push(']');
        }
        Value::Object(map) => {
            b.push('{');
            for (i, (k, item)) in map.iter().enumerate() {
                if i > 0 {
                    b.push(',');
                }
                json_string(b, k);
                b.push(':');
                json_value(b, item);
            }
            b.push('}');
        }
    }
}

/// Text in Anführungszeichen wie slog `appendEscapedJSONString`: `\\`, `\"`, `\n`, `\r`, `\t`, übrige Steuerzeichen als
/// `\u00XX`, U+2028/U+2029 als ` `/` `; `<`, `>` und `&` bleiben.
pub(crate) fn json_string(b: &mut String, s: &str) {
    b.push('"');
    for c in s.chars() {
        match c {
            '\\' | '"' => {
                b.push('\\');
                b.push(c);
            }
            '\n' => b.push_str("\\n"),
            '\r' => b.push_str("\\r"),
            '\t' => b.push_str("\\t"),
            '\u{2028}' | '\u{2029}' => b.push_str(&format!("\\u{:04x}", c as u32)),
            c if (c as u32) < 0x20 => b.push_str(&format!("\\u{:04x}", c as u32)),
            c => b.push(c),
        }
    }
    b.push('"');
}

/// Gleitkommazahl wie Go `encoding/json`: Dezimal zwischen 1e-6 und 1e21, sonst Exponent (`1e+21`, `1e-7`).
pub(crate) fn go_json_float(f: f64) -> String {
    let abs = f.abs();
    if abs == 0.0 || (1e-6..1e21).contains(&abs) {
        return format!("{f}");
    }
    let sci = format!("{f:e}");
    let (mant, exp) = sci.split_once('e').unwrap_or((sci.as_str(), "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    if exp < 0 {
        format!("{mant}e-{}", -exp)
    } else {
        format!("{mant}e+{exp:02}")
    }
}

#[cfg(test)]
#[path = "applog_tests.rs"]
mod tests;
