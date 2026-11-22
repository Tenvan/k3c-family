//! `markers.jsonl` im Ordner der Spieltest-Session: nach jedem Marker geschrieben, Fehler lassen das Spiel weiterlaufen.

use std::fs::{File, OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;

use k3c_platform::log;

use super::marker::Marker;

/// Name der Datei im Ordner.
pub const FILE: &str = "markers.jsonl";

/// Die offene Datei oder der Fehlerzustand (Ordner nicht beschreibbar).
#[derive(Debug)]
pub struct Sink {
    file: Option<File>,
}

impl Sink {
    /// Öffnet `<dir>/markers.jsonl` zum Anhängen; scheitert es, steht die Senke im Fehlerzustand (Log ❌).
    pub fn open(dir: &Path) -> Sink {
        let res = create_dir_all(dir).and_then(|()| OpenOptions::new().create(true).append(true).open(dir.join(FILE)));
        match res {
            Ok(f) => {
                log::info("📂 Spieltest bereit", &[("ordner", &dir.display())]);
                Sink { file: Some(f) }
            }
            Err(e) => {
                log::error(
                    "❌ Spieltest-Ordner nicht beschreibbar",
                    &[("ordner", &dir.display()), ("fehler", &e)],
                );
                Sink { file: None }
            }
        }
    }

    /// Fehlerzustand: es wird nichts geschrieben, REC erscheint durchgestrichen.
    pub fn failed(&self) -> bool {
        self.file.is_none()
    }

    /// Hängt die Zeile des Markers an; ein Schreibfehler setzt den Fehlerzustand.
    pub fn write(&mut self, m: &Marker) {
        let Some(f) = self.file.as_mut() else {
            return;
        };
        let line = format!("{}\n", m.to_line());
        if let Err(e) = f.write_all(line.as_bytes()).and_then(|()| f.flush()) {
            log::error("❌ Marker nicht geschrieben", &[("fehler", &e)]);
            self.file = None;
        }
    }
}
