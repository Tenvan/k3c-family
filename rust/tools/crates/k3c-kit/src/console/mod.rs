//! Konsolenausgabe eines Werkzeugs je Quelle (Port von `workkit/console`, B-046): Prüfläufe `check:<ziel>`, das eigene
//! Log und die Dienste. Nur im Speicher; der Puffer stirbt mit dem Programm.

mod writer;

use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;

use serde::{Deserialize, Serialize};

pub use writer::LineWriter;

/// Zahl der Zeilen je Quelle.
pub const DEFAULT_CAPACITY: usize = 2000;
/// Puffer der Benachrichtigungen; ist er voll, gehen Benachrichtigungen verloren, nie Zeilen.
pub(crate) const NOTIFY_QUEUE: usize = 256;

/// Eine Zeile einer Quelle. `seq` zählt je Quelle fortlaufend, auch über [`Store::reset`] hinweg; damit sortiert die
/// Oberfläche Zeilen ein, die während des Ladens kamen.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Line {
    pub source: String,
    /// stdout, stderr oder log
    pub stream: String,
    pub text: String,
    pub seq: i64,
}

#[derive(Default)]
struct Ring {
    lines: VecDeque<Line>,
    seq: i64,
}

/// Empfänger-Faden der Benachrichtigungen.
struct Notify {
    tx: SyncSender<Line>,
    done: Arc<AtomicBool>,
    thread: JoinHandle<()>,
}

/// Ringpuffer je Quelle. Alle Methoden sind nebenläufig sicher und blockieren nie auf den Empfänger.
pub struct Store {
    capacity: usize,
    rings: Mutex<BTreeMap<String, Ring>>,
    notify: Mutex<Option<Notify>>,
}

impl Store {
    /// Legt einen Store an (`capacity` 0: [`DEFAULT_CAPACITY`]); `on_line` (optional) bekommt jede neue Zeile in einem
    /// eigenen Faden.
    pub fn new(capacity: usize, on_line: Option<Box<dyn Fn(Line) + Send>>) -> Store {
        let capacity = if capacity == 0 { DEFAULT_CAPACITY } else { capacity };
        let notify = on_line.map(|f| {
            let (tx, rx) = sync_channel(NOTIFY_QUEUE);
            let done = Arc::new(AtomicBool::new(false));
            let stop = Arc::clone(&done);
            let thread = std::thread::spawn(move || dispatch(&rx, &stop, f.as_ref()));
            Notify { tx, done, thread }
        });
        Store {
            capacity,
            rings: Mutex::new(BTreeMap::new()),
            notify: Mutex::new(notify),
        }
    }

    fn rings(&self) -> MutexGuard<'_, BTreeMap<String, Ring>> {
        self.rings.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Beendet die Benachrichtigungen; danach wird `on_line` nicht mehr gerufen.
    pub fn close(&self) {
        let notify = self.notify.lock().unwrap_or_else(PoisonError::into_inner).take();
        if let Some(Notify { tx, done, thread }) = notify {
            done.store(true, Ordering::SeqCst);
            drop(tx);
            // Aus `on_line` selbst (z. B. letzter Arc fällt dort) nicht auf den eigenen Faden warten: Selbst-Join hinge.
            if thread.thread().id() != std::thread::current().id() {
                let _ = thread.join();
            }
        }
    }

    /// Hängt eine Zeile an; die älteste fällt heraus, wenn der Puffer voll ist.
    pub fn add(&self, source: &str, stream: &str, text: &str) {
        let line = {
            let mut rings = self.rings();
            let r = rings.entry(source.to_string()).or_default();
            r.seq += 1;
            let l = Line {
                source: source.into(),
                stream: stream.into(),
                text: text.into(),
                seq: r.seq,
            };
            if r.lines.len() == self.capacity {
                r.lines.pop_front();
            }
            r.lines.push_back(l.clone());
            l
        };
        let notify = self.notify.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(n) = notify.as_ref() {
            // Volle Queue oder beendeter Empfänger: Benachrichtigung verwerfen, nie blockieren.
            let _ = n.tx.try_send(line);
        }
    }

    /// Leert eine Quelle (neuer Lauf) und legt sie an, falls es sie noch nicht gibt.
    pub fn reset(&self, source: &str) {
        self.rings().entry(source.to_string()).or_default().lines.clear();
    }

    /// Die letzten `n` Zeilen einer Quelle (`n` 0: alle) oder `None`, wenn es die Quelle nicht gibt.
    pub fn tail(&self, source: &str, n: usize) -> Option<Vec<Line>> {
        let rings = self.rings();
        let r = rings.get(source)?;
        let skip = if n > 0 && r.lines.len() > n {
            r.lines.len() - n
        } else {
            0
        };
        Some(r.lines.iter().skip(skip).cloned().collect())
    }

    /// Alle Quellen, sortiert.
    pub fn sources(&self) -> Vec<String> {
        self.rings().keys().cloned().collect()
    }

    /// Zählt Quellen, gehaltene Zeilen und deren Text in Bytes (Ressourcen-Monitoring der Workbench).
    pub fn stats(&self) -> (usize, usize, i64) {
        let rings = self.rings();
        let mut lines = 0;
        let mut bytes = 0i64;
        for r in rings.values() {
            lines += r.lines.len();
            for l in &r.lines {
                bytes += i64::try_from(l.text.len() + l.source.len() + l.stream.len()).unwrap_or(i64::MAX);
            }
        }
        (rings.len(), lines, bytes)
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        self.close();
    }
}

/// Gibt Zeilen an `on_line`, bis der Sender weg ist oder `done` gesetzt wurde (wartende Zeilen verfallen dann, wie Go).
fn dispatch(rx: &Receiver<Line>, done: &AtomicBool, on_line: &(dyn Fn(Line) + Send)) {
    while let Ok(l) = rx.recv() {
        if done.load(Ordering::SeqCst) {
            return;
        }
        on_line(l);
    }
}

#[cfg(test)]
#[path = "console_tests.rs"]
mod tests;
