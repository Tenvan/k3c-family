//! Log-Einträge eines Dienstes je Level (`LogLevels` aus `adopt.go`). Der Leser ist der Teil von `internal/logs`, den
//! die Zählung braucht; das Modul der Seite Logs (RU69.5) kann ihn ersetzen.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;
use std::time::Duration;

use chrono::{DateTime, Local};
use serde::Serialize;
use serde_json::{Map, Value};

use super::controller::Controller;

/// Höchste Zahl gelesener Bytes je Anfrage.
const BUDGET: u64 = 8 << 20;

/// Zähler der letzten 60 min.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelCounts {
    pub counts: BTreeMap<String, u64>,
    pub total: u64,
    /// Ältere Einträge der 60 min nicht gelesen.
    pub budget_hit: bool,
}

/// Level einer Zeile ab `since`: `None` heißt unlesbar, `Some(None)` zu alt.
fn level_since(line: &[u8], since: DateTime<Local>) -> Option<Option<String>> {
    let entry: Map<String, Value> = serde_json::from_slice(line).ok()?;
    let time = DateTime::parse_from_rfc3339(entry.get("time")?.as_str()?).ok()?;
    if time < since {
        return Some(None);
    }
    let level = entry.get("level").and_then(Value::as_str).unwrap_or_default();
    Some(Some(level.to_uppercase()))
}

/// Liest von hinten bis zum ersten Eintrag vor `since`, höchstens `budget` Bytes. Eine fehlende Datei ist leer.
pub(crate) fn count_since(path: &Path, since: DateTime<Local>, budget: u64) -> io::Result<LevelCounts> {
    let mut out = LevelCounts::default();
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(out),
        Err(e) => return Err(e),
    };
    let size = file.metadata()?.len();
    let from = size.saturating_sub(budget);
    file.seek(SeekFrom::Start(from))?;
    let mut data = Vec::new();
    file.take(size - from).read_to_end(&mut data)?;
    let mut lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
    // Die erste Zeile eines abgeschnittenen Stücks ist unvollständig.
    if from > 0 && !lines.is_empty() {
        lines.remove(0);
    }
    for line in lines.into_iter().rev() {
        match level_since(line.trim_ascii_end(), since) {
            Some(Some(level)) => {
                *out.counts.entry(level).or_default() += 1;
                out.total += 1;
            }
            Some(None) => return Ok(out),
            None => {}
        }
    }
    out.budget_hit = from > 0;
    Ok(out)
}

impl Controller {
    /// Zählt je Level aus `logs/<log>.jsonl`; `None`, wenn der Dienst kein Log hat.
    pub fn log_levels(&self, name: &str) -> Result<Option<LevelCounts>, String> {
        let svc = self.unit(name)?.svc();
        if svc.log.is_empty() {
            return Ok(None);
        }
        let hour = Duration::from_secs(3600);
        let since = (self.opts.now)() - hour;
        let path = self.opts.root.join("logs").join(format!("{}.jsonl", svc.log));
        count_since(&path, since, BUDGET).map(Some).map_err(|e| e.to_string())
    }
}
