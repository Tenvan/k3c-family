//! Läufe mit Lauf-ID (Port von `check_job.go` und der Sperre aus `check_run.go`). Der Lauf hängt nicht am Aufruf des
//! Clients: bricht dieser ab, läuft er zu Ende und bleibt über seine Lauf-ID abfragbar (B-372).

use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Instant;

use chrono::{DateTime, Local};
use tokio::sync::watch;

use super::compact::{FALLBACK_LINES, matches_any, report};
use super::run::RunResult;
use super::targets::CheckTarget;

/// So viele Läufe bleiben über ihre ID abfragbar.
pub(super) const MAX_JOBS: usize = 20;

#[derive(Default)]
struct Data {
    /// Fehlerzeilen des Ziels (nur sie und das Ende; die volle Ausgabe steht im Konsolenpuffer).
    output: Vec<String>,
    /// Letzte Zeilen ohne Fehlermuster, für die Antwort ohne Fehlerzeilen.
    tail: VecDeque<String>,
    res: RunResult,
}

pub(super) struct Job {
    pub id: String,
    pub target: &'static CheckTarget,
    /// Quelle samt Checkout.
    pub key: String,
    pub started_at: DateTime<Local>,
    pub start: Instant,
    done: watch::Sender<bool>,
    data: Mutex<Data>,
}

impl Job {
    pub(super) fn new(id: String, target: &'static CheckTarget, key: String) -> Job {
        Job {
            id,
            target,
            key,
            started_at: Local::now(),
            start: Instant::now(),
            done: watch::Sender::new(false),
            data: Mutex::default(),
        }
    }

    fn data(&self) -> MutexGuard<'_, Data> {
        self.data.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub(super) fn add(&self, text: &str) {
        let mut data = self.data();
        if matches_any(text, self.target.kinds) {
            data.output.push(text.to_owned());
            return;
        }
        data.tail.push_back(text.to_owned());
        if data.tail.len() > FALLBACK_LINES {
            data.tail.pop_front();
        }
    }

    pub(super) fn set_result(&self, res: RunResult) {
        self.data().res = res;
    }

    /// Verdichtete Antwort eines beendeten Laufs.
    pub(super) fn answer(&self) -> Result<String, String> {
        let data = self.data();
        if let Some(e) = &data.res.err {
            return Err(format!("{} ließ sich nicht starten: {e}", self.target.name));
        }
        let lines: Vec<String> = data.output.iter().chain(&data.tail).cloned().collect();
        Ok(report(self.target, &data.res, &lines))
    }

    pub(super) fn finish(&self) {
        self.done.send_replace(true);
    }

    pub(super) fn is_done(&self) -> bool {
        *self.done.borrow()
    }

    /// Wartet auf [`Job::finish`].
    pub(super) async fn done(&self) {
        let mut rx = self.done.subscribe();
        // Der Sender lebt in `self`, der Fehlerfall kommt nicht vor.
        let _ = rx.wait_for(|done| *done).await;
    }

    #[cfg(test)]
    pub(super) fn held(&self) -> (usize, usize) {
        let data = self.data();
        (data.output.len(), data.tail.len())
    }
}

/// Je Quelle höchstens ein Lauf, der letzte Lauf je Quelle und die gemerkten Läufe mit ID.
#[derive(Default)]
pub(super) struct Runs {
    /// Beginn des laufenden Laufs.
    pub running: BTreeMap<String, DateTime<Local>>,
    pub last: BTreeMap<String, RunResult>,
    /// Älteste zuerst.
    jobs: VecDeque<Arc<Job>>,
    seq: u64,
}

impl Runs {
    /// Sperrt die Quelle und legt den Lauf an; läuft schon einer, kommt dessen Lauf-ID als Fehler zurück.
    pub(super) fn begin(&mut self, target: &'static CheckTarget, key: &str) -> Result<Arc<Job>, String> {
        if self.running.contains_key(key) {
            let running = self.jobs.iter().rev().find(|j| j.key == key && !j.is_done());
            return Err(running.map(|j| j.id.clone()).unwrap_or_default());
        }
        self.running.insert(key.to_owned(), Local::now());
        self.seq += 1;
        let job = Arc::new(Job::new(format!("L{}", self.seq), target, key.to_owned()));
        self.jobs.push_back(Arc::clone(&job));
        while self.jobs.len() > MAX_JOBS {
            self.jobs.pop_front();
        }
        Ok(job)
    }

    pub(super) fn end(&mut self, key: &str, res: RunResult) {
        self.running.remove(key);
        self.last.insert(key.to_owned(), res);
    }

    pub(super) fn job(&self, id: &str) -> Option<Arc<Job>> {
        self.jobs.iter().find(|j| j.id == id).map(Arc::clone)
    }

    /// Zahl der gemerkten Läufe und die Bytes ihrer gesammelten Zeilen (Ressourcen-Monitoring).
    pub(super) fn stats(&self) -> (usize, usize) {
        let bytes = self
            .jobs
            .iter()
            .map(|job| {
                let data = job.data();
                data.output.iter().chain(&data.tail).map(String::len).sum::<usize>()
            })
            .sum();
        (self.jobs.len(), bytes)
    }
}
