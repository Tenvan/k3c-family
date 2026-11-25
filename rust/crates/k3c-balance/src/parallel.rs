//! Läufe parallel (Port von `engine/balance/parallel.go`, B-634, BS1.2): Jeder Lauf hat seine eigene Insel und liest nur
//! gemeinsame Tabellen, daher rechnen die Szenarien eines Berichts gleichzeitig. Die Ergebnisse stehen in der Reihenfolge
//! der Szenarien, das Ergebnis jedes Laufs ist dasselbe wie nacheinander (deterministisch je Szenario).
//!
//! Dies ist die einzige Stelle des Crates mit Threads: Die Arbeiter holen sich den nächsten Lauf über einen Zähler und
//! schicken ihr Ergebnis über einen Kanal an den aufrufenden Thread; dort läuft auch `progress` (kein `Mutex`).

use std::any::Any;
use std::fmt;
use std::num::NonZeroUsize;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{Sender, channel};
use std::thread;

use crate::run::{RunResult, Scenario, Tick, run_one};
use crate::vary::{current_data, set_current_data};

/// Fortschrittsmeldung: Zahl der fertigen Läufe und aller Läufe (Go: `progress func(done, total int)`).
pub type ProgressFn<'a> = Box<dyn FnMut(usize, usize) + 'a>;

/// Die Panik eines Laufs oder von `progress`, die `run_all` an den Aufrufer zurückgibt (Go: `Crash`, dort erneut
/// ausgelöst). `stack` bleibt leer: Rust hat an der Fangstelle keinen Stack der ursprünglichen Panik.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crash {
    pub value: String,
    pub stack: String,
}

impl fmt::Display for Crash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.value, self.stack)
    }
}

impl std::error::Error for Crash {}

/// Der Text einer abgefangenen Panik (Go: `%v` des Wertes aus `recover`).
pub fn panic_text(payload: &(dyn Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unbekannte Panik".to_string()
    }
}

/// Die Zahl gleichzeitiger Läufe (Go: `workers = runtime.NumCPU()`).
pub fn worker_count() -> usize {
    thread::available_parallelism().map_or(1, NonZeroUsize::get)
}

/// Rechnet alle Szenarien mit so vielen Läufen gleichzeitig wie Kerne da sind (Go: `runAll`).
pub fn run_all(all: &[Scenario], threshold: i64, progress: Option<ProgressFn>) -> Result<Vec<RunResult>, Crash> {
    run_all_with(all, threshold, progress, worker_count())
}

/// Rechnet alle Szenarien mit höchstens `workers` Läufen gleichzeitig; `progress` erfährt nach jedem fertigen Lauf die
/// Zahl der fertigen und aller Läufe. Eine Panik in `progress` (B-924: die Workbench beendet so einen abgebrochenen Lauf)
/// lässt die übrigen Läufe aus und kommt beim Aufrufer als `Err(Crash)` an. Tests setzen `workers` auf 1, um die
/// Reihenfolge zu vergleichen.
pub fn run_all_with(
    all: &[Scenario],
    threshold: i64,
    mut progress: Option<ProgressFn>,
    workers: usize,
) -> Result<Vec<RunResult>, Crash> {
    let total = all.len();
    let next = AtomicUsize::new(0);
    let stop = AtomicBool::new(false);
    let (tx, rx) = channel::<(usize, RunResult)>();
    let mut results: Vec<Option<RunResult>> = (0..total).map(|_| None).collect();
    let mut crash: Option<Crash> = None;
    let data = current_data(); // die Daten einer Variante (`vary::with_data`) gelten auch in den Arbeitern
    thread::scope(|s| {
        for _ in 0..workers.min(total).max(1) {
            let tx = tx.clone();
            let (next, stop) = (&next, &stop);
            s.spawn(move || {
                set_current_data(data);
                work(all, threshold, next, stop, &tx);
            });
        }
        drop(tx);
        for (n, (i, r)) in rx.into_iter().enumerate() {
            results[i] = Some(r);
            if crash.is_none() {
                crash = report(progress.as_mut(), n + 1, total);
                if crash.is_some() {
                    stop.store(true, Ordering::Relaxed);
                }
            }
        }
    });
    if let Some(c) = crash {
        return Err(c);
    }
    let out: Vec<RunResult> = results.into_iter().flatten().collect();
    if out.len() != total {
        return Err(Crash {
            value: "balance: ein Lauf hat kein Ergebnis geliefert".to_string(),
            stack: String::new(),
        });
    }
    Ok(out)
}

/// Meldet den Fortschritt; eine Panik in `progress` wird zum `Crash`.
fn report(progress: Option<&mut ProgressFn>, done: usize, total: usize) -> Option<Crash> {
    let p = progress?;
    catch_unwind(AssertUnwindSafe(|| p(done, total))).err().map(|e| Crash {
        value: panic_text(e.as_ref()),
        stack: String::new(),
    })
}

/// Ein Arbeiter: holt sich den nächsten Lauf, bis alle vergeben sind oder ein Absturz die übrigen auslässt.
fn work(all: &[Scenario], threshold: i64, next: &AtomicUsize, stop: &AtomicBool, tx: &Sender<(usize, RunResult)>) {
    while !stop.load(Ordering::Relaxed) {
        let i = next.fetch_add(1, Ordering::Relaxed);
        let Some(sc) = all.get(i) else { return };
        if tx.send((i, run_one(sc, threshold))).is_err() {
            return;
        }
    }
}

/// Hängt `tick` (darf fehlen) vor `progress` (darf fehlen); beide fehlend ergibt `None` (Go: `tickProgress`).
pub fn tick_progress<'a>(tick: Option<Tick>, progress: Option<ProgressFn<'a>>) -> Option<ProgressFn<'a>> {
    let Some(tick) = tick else { return progress };
    let mut progress = progress;
    Some(Box::new(move |done, total| {
        (tick.0)();
        if let Some(p) = progress.as_mut() {
            p(done, total);
        }
    }))
}
