//! Testberichte der Gamepad-Testseite und Spielmetrik-Reports als Dateien. Port von `engine/store/reports.go`.
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::stamp::Stamp;
use crate::fehler::Fehler;

/// Obergrenze eines Testberichts (wie `server/reports.mjs`).
pub const MAX_REPORT_BYTES: usize = 256 << 10;

/// Zahl der Berichte, die liegen bleiben (B-142); dieselbe Grenze gilt getrennt für die Spielmetrik-Reports (B-272).
pub const MAX_REPORTS: usize = 100;

const REPORT_PREFIX: &str = "gamepad-";
const SESSION_PREFIX: &str = "session-";

/// Die Testberichte unter `dir`. Die Uhr gibt der Aufrufer vor; Log-Zeilen sammelt `take_log`.
pub struct Reports {
    pub dir: PathBuf,
    pub now: Box<dyn FnMut() -> Stamp>,
    log: Vec<String>,
}

/// Go schreibt `<`, `>`, `&`, U+2028 und U+2029 in JSON-Zeichenketten als `\u00XX`; `serde_json` nicht. Diese Zeichen
/// kommen im JSON nur innerhalb von Zeichenketten vor, die Ersetzung der ganzen Ausgabe ist daher sicher.
fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            _ => out.push(c),
        }
    }
    out
}

/// Prüft Größe und Form: ein JSON-Objekt (kein `null`, keine Liste).
fn parse_report(data: &[u8]) -> Result<serde_json::Map<String, Value>, Fehler> {
    if data.len() > MAX_REPORT_BYTES {
        return Err(Fehler::TooLarge);
    }
    match serde_json::from_slice::<Value>(data) {
        Ok(Value::Object(m)) => Ok(m),
        _ => Err(Fehler::Invalid),
    }
}

fn file_stamp(stamp: &Stamp) -> String {
    stamp.format_report().replace([':', '.'], "-")
}

impl Reports {
    pub fn new(dir: impl Into<PathBuf>, now: impl FnMut() -> Stamp + 'static) -> Reports {
        Reports {
            dir: dir.into(),
            now: Box::new(now),
            log: Vec::new(),
        }
    }

    /// Holt die gesammelten Log-Zeilen ab (🧹 aufgeräumt, ❌ Löschen gescheitert) und leert den Puffer.
    pub fn take_log(&mut self) -> Vec<String> {
        std::mem::take(&mut self.log)
    }

    /// Ergänzt `receivedAt` und `remoteAddress` und schreibt den Bericht als `gamepad-<zeit>.json`. Die Antwort ist
    /// der Pfad der Datei. Ein Bericht muss ein JSON-Objekt sein.
    pub fn store(&mut self, data: &[u8], remote: &str) -> Result<PathBuf, Fehler> {
        let mut report = parse_report(data)?;
        let received = (self.now)().format_report();
        report.insert("receivedAt".into(), Value::String(received.clone()));
        report.insert("remoteAddress".into(), Value::String(remote.into()));
        let out = serde_json::to_string_pretty(&Value::Object(report)).map_err(|_| Fehler::Invalid)?;
        let out = escape_html(&out);
        std::fs::create_dir_all(&self.dir)?;
        let n = self.prune(REPORT_PREFIX, MAX_REPORTS - 1)?;
        if n > 0 {
            self.log.push(format!("🧹 alte Berichte gelöscht ns=report anzahl={n}"));
        }
        let base = format!("{REPORT_PREFIX}{}", received.replace([':', '.'], "-"));
        write_new(&self.dir, &base, out.as_bytes())
    }

    /// Schreibt einen Spielmetrik-Report (`docs/protocol.md` › Spielmetrik-Report, B-150) als `session-<zeit>.json`,
    /// ohne eine vorhandene Datei zu überschreiben. Ältere Reports über `MAX_REPORTS` löscht er vorher (B-272);
    /// scheitert das Löschen, steht es im Log und der neue Report wird trotzdem geschrieben.
    pub fn store_session(&mut self, data: &[u8]) -> Result<PathBuf, Fehler> {
        parse_report(data)?;
        let stamp = file_stamp(&(self.now)());
        std::fs::create_dir_all(&self.dir)?;
        match self.prune(SESSION_PREFIX, MAX_REPORTS - 1) {
            Err(e) => self
                .log
                .push(format!("❌ alte Spielmetrik-Reports nicht gelöscht ns=report err={e}")),
            Ok(n) if n > 0 => self
                .log
                .push(format!("🧹 alte Spielmetrik-Reports gelöscht ns=report anzahl={n}")),
            Ok(_) => {}
        }
        write_new(&self.dir, &format!("{SESSION_PREFIX}{stamp}"), data)
    }

    /// Zahl der gespeicherten Gamepad-Berichte.
    pub fn count(&self) -> usize {
        self.matching(REPORT_PREFIX).len()
    }

    /// Dateien `<prefix>*.json` im Ordner, nach Name sortiert (wie `filepath.Glob`; ein unlesbarer Ordner ist leer).
    fn matching(&self, prefix: &str) -> Vec<String> {
        let Ok(entries) = std::fs::read_dir(&self.dir) else {
            return Vec::new();
        };
        let mut names: Vec<String> = entries
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.starts_with(prefix) && n.ends_with(".json"))
            .collect();
        names.sort_unstable();
        names
    }

    /// Löscht die ältesten Dateien zu `prefix`, bis höchstens `keep` übrig sind, und meldet, wie viele es waren. Die
    /// Ordnung folgt dem Zeitstempel im Dateinamen, nicht der Dateizeit; andere Dateien und Ordner bleiben unberührt.
    fn prune(&self, prefix: &str, keep: usize) -> Result<usize, Fehler> {
        let files = self.matching(prefix);
        if files.len() <= keep {
            return Ok(0);
        }
        let old = &files[..files.len() - keep];
        for f in old {
            match std::fs::remove_file(self.dir.join(f)) {
                Err(e) if e.kind() != ErrorKind::NotFound => return Err(e.into()),
                _ => {}
            }
        }
        Ok(old.len())
    }
}

/// Legt `<base>.json` an, ohne eine vorhandene Datei zu überschreiben (zwei Berichte in derselben Millisekunde
/// bekommen -1, -2 …).
fn write_new(dir: &Path, base: &str, data: &[u8]) -> Result<PathBuf, Fehler> {
    for i in 0..100 {
        let name = if i > 0 { format!("{base}-{i}") } else { base.to_string() };
        let path = dir.join(format!("{name}.json"));
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut f) => {
                f.write_all(data)?;
                f.flush()?;
                return Ok(path);
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e.into()),
        }
    }
    Err(Fehler::Io(format!(
        "{base}: zu viele Berichte in derselben Millisekunde"
    )))
}
