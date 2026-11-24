//! Abbruch eines Laufs über `--stop-stdin` (RU33.1): stdin wird bis zum Ende gelesen; das Ende (die Workbench schließt
//! die Pipe) setzt das Zeichen. Strg+C und CTRL_BREAK beenden die EXE ohne Zeile `aborted` (std kennt keinen
//! Signal-Handler, `unsafe` ist verboten).

use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Das Abbruch-Zeichen; Klone teilen es.
#[derive(Debug, Clone, Default)]
pub struct Stop(Arc<AtomicBool>);

impl Stop {
    /// Ein Zeichen, das nie gesetzt wird.
    pub fn never() -> Self {
        Stop::default()
    }

    /// Setzt das Zeichen.
    pub fn trigger(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    /// Ist der Abbruch verlangt?
    pub fn is_set(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }

    /// Startet den Thread, der `input` bis zum Ende liest und dann das Zeichen setzt.
    pub fn on_end_of<R: Read + Send + 'static>(mut input: R) -> Self {
        let stop = Stop::never();
        let flag = stop.clone();
        std::thread::spawn(move || {
            let _ = std::io::copy(&mut input, &mut std::io::sink());
            flag.trigger();
        });
        stop
    }
}
