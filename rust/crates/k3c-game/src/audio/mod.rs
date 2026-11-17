//! Port des Pakets `native/k3c-game/audio` (Sprint RU17, B-1014): Ton im nativen Spiel.
//!
//! Klang-Effekte, Menü-Klänge und Musik zu Ereignissen, Cues und Lage des Frames wie der Browser-Client (src/audio/,
//! B-801). Zwei Schichten: der Planer (`mix`) entscheidet als reine, getestete Logik, welcher Klang wann und wie laut
//! klingt und welche Musik läuft; das Gerät (`output`) setzt das über `k3c_platform::AudioOut` um (B-805). Ohne
//! Ton-Gerät, bei einem Treiberfehler, ohne Ton-Dateien, in Tests und im Aufnahme-Modus bleibt das Spiel stumm, nie
//! ein Absturz.
//!
//! Teil 1 (RU17.3): `register`, `atlas`, `duck`, `events`. Teil 2 (RU17.4): `mix`, `music`. Teil 3 (RU17.5): `bank`,
//! `output` und `Sound` (Inhalt von `audio.go`).
//!
//! Wegfall von `sink.go`: Die Senke kapselt oto (Ebitengine). In Rust übernimmt `k3c_platform::AudioOut` (RU4.8,
//! `open_audio()`) diese Aufgabe; es gibt keine `sink.rs`, Tests nehmen `NullAudio` oder `testutil::FakeAudio`.

pub mod atlas;
pub mod bank;
pub mod duck;
pub mod events;
pub mod mix;
pub mod music;
pub mod output;
pub mod register;

use std::path::PathBuf;
use std::sync::Arc;

use k3c_platform::log;

use self::bank::DEFAULT_PUBLIC_DIR;
use self::mix::Planner;
use self::output::{Output, Silent, new_device};
use self::register::{Register, load_register};
use crate::view::frame::Frame;
use crate::view::layer::Listener;

/// Sound ist der Ton-Listener: je Frame genau einmal `listen` (`view::layer::Listener`).
pub struct Sound {
    pub plan: Planner,
    pub out: Box<dyn Output>,
}

impl Sound {
    /// new liefert den Ton-Listener mit den Ton-Dateien unter public/ (Pfad relativ zur Repo-Wurzel). In Tests bleibt
    /// er stumm; im Aufnahme-Modus (`Frame::recording`) rechnet er den Mix, gibt ihn aber nicht aus.
    pub fn new() -> Sound {
        let (reg, ok) = match load_register(&read_data) {
            Ok(r) => (r, true),
            Err(e) => {
                log::warn("❌ Ton-Register nicht ladbar, Spiel läuft stumm", &[("err", &e)]);
                (Register::default(), false)
            }
        };
        let reg = Arc::new(reg);
        if ok && !cfg!(test) {
            let out = Box::new(new_device(DEFAULT_PUBLIC_DIR, Arc::clone(&reg)));
            return new_sound(reg, out);
        }
        new_sound(reg, Box::new(Silent))
    }

    /// new_silent liefert den Listener ohne Ton-Gerät (rechnet nur den Mix).
    pub fn new_silent(reg: Arc<Register>) -> Sound {
        new_sound(reg, Box::new(Silent))
    }
}

impl Default for Sound {
    fn default() -> Self {
        Sound::new()
    }
}

pub fn new_sound(reg: Arc<Register>, out: Box<dyn Output>) -> Sound {
    Sound {
        plan: Planner::new(reg),
        out,
    }
}

impl Listener for Sound {
    /// listen rechnet den Mix des Frames und setzt ihn um (nicht im Aufnahme-Modus).
    fn listen(&mut self, f: &Frame) {
        let m = self.plan.plan(f);
        if !f.recording {
            self.out.apply(m);
        }
    }
}

/// Liest `name` aus data/ (Go: `data.Files`): Ordner aus `K3C_DATA_DIR`, sonst `data` relativ zum Arbeitsordner.
fn read_data(name: &str) -> Result<Vec<u8>, String> {
    let dir = match std::env::var_os("K3C_DATA_DIR") {
        Some(d) if !d.is_empty() => PathBuf::from(d),
        _ => PathBuf::from("data"),
    };
    let path = dir.join(name);
    std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))
}

#[cfg(test)]
mod atlas_tests;
#[cfg(test)]
mod audio_tests;
#[cfg(test)]
mod events_tests;
#[cfg(test)]
mod klang_tests;
#[cfg(test)]
mod music_tests;
#[cfg(test)]
mod output_tests;
#[cfg(test)]
mod testutil;
