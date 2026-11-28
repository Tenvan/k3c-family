//! Bündelung von `console:line` (Port von `console_batch.go`): jede Zeile einzeln brachte den WebView2-Renderer bei
//! parallelen Prüfläufen auf über 1 GiB. Der Bündler sammelt höchstens [`BATCH_WINDOW`] lang und schickt eine Liste;
//! mehr als [`BATCH_MAX`] Zeilen je Fenster schickt er nur die jüngsten. Die Lücke in `seq` lädt die Oberfläche nach.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use k3c_kit::console::Line;

pub(crate) const BATCH_WINDOW: Duration = Duration::from_millis(200);
pub(crate) const BATCH_MAX: usize = 400;

#[derive(Default)]
struct Pending {
    lines: VecDeque<Line>,
    scheduled: bool,
}

type Deliver = Arc<dyn Fn(Vec<Line>) + Send + Sync>;

#[derive(Clone)]
pub(crate) struct LineBatcher {
    pending: Arc<Mutex<Pending>>,
    window: Duration,
    send: Deliver,
}

fn lock(pending: &Mutex<Pending>) -> MutexGuard<'_, Pending> {
    pending.lock().unwrap_or_else(PoisonError::into_inner)
}

impl LineBatcher {
    pub fn new(send: impl Fn(Vec<Line>) + Send + Sync + 'static) -> LineBatcher {
        LineBatcher::with_window(BATCH_WINDOW, send)
    }

    pub fn with_window(window: Duration, send: impl Fn(Vec<Line>) + Send + Sync + 'static) -> LineBatcher {
        LineBatcher {
            pending: Arc::default(),
            window,
            send: Arc::new(send),
        }
    }

    /// Nimmt eine Zeile auf; die erste Zeile eines Fensters plant das Senden.
    pub fn add(&self, line: Line) {
        let mut pending = lock(&self.pending);
        pending.lines.push_back(line);
        if pending.lines.len() > BATCH_MAX {
            pending.lines.pop_front();
        }
        if !pending.scheduled {
            pending.scheduled = true;
            let batch = self.clone();
            // Höchstens ein Faden je Fenster; er kommt auch ohne tokio-Laufzeit aus.
            std::thread::spawn(move || {
                std::thread::sleep(batch.window);
                batch.flush();
            });
        }
    }

    /// Schickt die gesammelten Zeilen als eine Liste.
    fn flush(&self) {
        let lines: Vec<Line> = {
            let mut pending = lock(&self.pending);
            pending.scheduled = false;
            std::mem::take(&mut pending.lines).into()
        };
        if !lines.is_empty() {
            (self.send)(lines);
        }
    }
}

#[cfg(test)]
#[path = "console_batch_tests.rs"]
mod tests;
