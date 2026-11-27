//! Schritt-Protokoll und Sperre für schwere Läufe (Port von `check_steps.go`, B-739).
//!
//! `task check:diff` meldet seine Schritte als Markerzeilen `⏱▶ <name>` (Start) und `⏱■ <name> exit=<n>` (Ende). Das
//! Zeitmessen übernimmt `check_run`, denn die Shell des Tasks hat nicht überall `date`. Jeder Schritt wird ein
//! Log-Eintrag (`ns=check`, `step`, `ms`, `exit`), damit `logs_query` zeigt, wo ein Lauf Zeit braucht.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Instant;

use k3c_kit::logger::Logger;
use serde_json::json;
use tokio::sync::{Semaphore, SemaphorePermit};

const STEP_START: &str = "⏱▶ ";
const STEP_END: &str = "⏱■ ";

pub(super) type Clock = Arc<dyn Fn() -> Instant + Send + Sync>;

/// Verfolgt die Schritte eines Laufs. Schritte der parallelen Ketten überlappen, daher je Name.
pub(super) struct StepLog {
    log: Logger,
    target: String,
    now: Clock,
    started: Mutex<BTreeMap<String, Instant>>,
}

impl StepLog {
    pub(super) fn new(log: Logger, target: &str) -> StepLog {
        StepLog::with_clock(log, target, Arc::new(Instant::now))
    }

    pub(super) fn with_clock(log: Logger, target: &str, now: Clock) -> StepLog {
        StepLog {
            log,
            target: target.to_owned(),
            now,
            started: Mutex::default(),
        }
    }

    /// Wertet eine Ausgabezeile aus; Zeilen ohne Marker bleiben unberührt.
    pub(super) fn line(&self, text: &str) {
        let text = text.trim();
        if let Some(rest) = text.strip_prefix(STEP_START) {
            let name = rest.trim();
            let at = (self.now)();
            self.started
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .insert(name.to_owned(), at);
            let attrs = [
                ("ns", json!("check")),
                ("target", json!(self.target)),
                ("step", json!(name)),
            ];
            self.log.info("🚦 schritt beginnt", &attrs);
        } else if let Some(rest) = text.strip_prefix(STEP_END) {
            self.end(rest);
        }
    }

    fn end(&self, rest: &str) {
        let (name, exit) = parse_step_end(rest);
        let at = self.started.lock().unwrap_or_else(PoisonError::into_inner).remove(name);
        let mut attrs = vec![
            ("ns", json!("check")),
            ("target", json!(self.target)),
            ("step", json!(name)),
            ("exit", json!(exit)),
        ];
        if let Some(at) = at {
            let ms = (self.now)().duration_since(at).as_millis() as u64;
            attrs.push(("ms", json!(ms)));
        }
        if exit != 0 {
            self.log.warn("❌ schritt beendet", &attrs);
        } else {
            self.log.info("✅ schritt beendet", &attrs);
        }
    }
}

/// Zerlegt `<name> exit=<n>`; ohne Zahl gilt exit -1.
pub(super) fn parse_step_end(s: &str) -> (&str, i64) {
    let s = s.trim();
    let (name, rest) = s.split_once(' ').unwrap_or((s, ""));
    let exit = rest.trim().strip_prefix("exit=").and_then(|v| v.parse().ok());
    (name, exit.unwrap_or(-1))
}

/// Höchstens ein schwerer Lauf (`check:diff`, `check`) gleichzeitig, auch über Worktrees hinweg: Mehrere Sessions auf
/// einem Rechner verdoppeln sonst die Laufzeit und lösen Zeitlimits aus (B-739).
pub(super) static HEAVY: Semaphore = Semaphore::const_new(1);

pub(super) fn is_heavy_target(name: &str) -> bool {
    name == "task:check:diff" || name == "task:check"
}

/// Wartet auf den Slot (nur bei schweren Zielen); solange der Rückgabewert lebt, ist er belegt. Das Zeitlimit des
/// Laufs beginnt erst danach. Ein fallen gelassenes Future belegt nichts.
pub(super) async fn acquire_heavy(log: &Logger, target: &str, key: &str) -> Option<SemaphorePermit<'static>> {
    if !is_heavy_target(target) {
        return None;
    }
    if let Ok(slot) = HEAVY.try_acquire() {
        return Some(slot);
    }
    let waited = Instant::now();
    log.info(
        "⏳ wartet auf freien Prüf-Slot",
        &[("ns", json!("check")), ("target", json!(key))],
    );
    // Die Sperre wird nie geschlossen, der Fehlerfall kommt nicht vor.
    let slot = HEAVY.acquire().await.ok()?;
    let ms = waited.elapsed().as_millis() as u64;
    let attrs = [("ns", json!("check")), ("target", json!(key)), ("ms", json!(ms))];
    log.info("🚦 Prüf-Slot frei", &attrs);
    Some(slot)
}
