//! Hilfen der Tests des Pakets audio (Leitfaden § 6).
//!
//! Ein Ton-Gerät gibt es im Test nicht: Tests ohne Prüfung der Stimmen nehmen `k3c_platform::NullAudio`, Tests der
//! Wiedergabe `FakeAudio` (Go: `fakeSink`/`fakePlayer`, alles im Test-Thread).

use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use k3c_platform::{AudioOut, SoundId, VoiceId};

use super::output::{Device, new_device};
use super::register::{Register, load_register};

/// Eine Stimme am Fake-Gerät.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FakePlayer {
    pub sound: u32,
    pub vol: f32,
    pub stopped: bool,
    /// Lesefehler: klingt nicht mehr
    pub err: bool,
}

impl FakePlayer {
    pub fn is_playing(&self) -> bool {
        !self.stopped && !self.err
    }
}

/// Zustand des Fake-Geräts, geteilt mit dem Test.
#[derive(Debug, Default)]
pub struct FakeState {
    pub players: Vec<FakePlayer>,
    pub loads: u32,
    /// Treiberfehler (`failed`)
    pub err: Option<String>,
}

/// AudioOut für Tests: lädt alles außer Inhalten, die mit `kaputt` beginnen; Stimmen klingen bis `stop` oder `err`.
pub struct FakeAudio(pub Rc<RefCell<FakeState>>);

impl AudioOut for FakeAudio {
    fn load(&mut self, bytes: &[u8]) -> Result<SoundId, String> {
        if bytes.starts_with(b"kaputt") {
            return Err("unbekanntes Ton-Format".to_string());
        }
        let mut s = self.0.borrow_mut();
        s.loads += 1;
        Ok(SoundId(s.loads - 1))
    }

    fn play(&mut self, sound: SoundId, volume: f32) -> VoiceId {
        let mut s = self.0.borrow_mut();
        s.players.push(FakePlayer {
            sound: sound.0,
            vol: volume,
            ..FakePlayer::default()
        });
        VoiceId(u32::try_from(s.players.len() - 1).expect("Stimmen"))
    }

    fn play_loop(&mut self, sound: SoundId, volume: f32) -> VoiceId {
        self.play(sound, volume)
    }

    fn set_volume(&mut self, voice: VoiceId, volume: f32) {
        self.0.borrow_mut().players[voice.0 as usize].vol = volume;
    }

    fn stop(&mut self, voice: VoiceId) {
        self.0.borrow_mut().players[voice.0 as usize].stopped = true;
    }

    fn playing(&self, voice: VoiceId) -> bool {
        self.0.borrow().players[voice.0 as usize].is_playing()
    }

    fn failed(&self) -> Option<String> {
        self.0.borrow().err.clone()
    }
}

/// Neuer, leerer Ordner je Test (Go: `t.TempDir()`).
pub fn temp_dir(name: &str) -> PathBuf {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work/tmp").join(format!("k3c-audio-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("Test-Ordner");
    dir
}

/// fake_device liefert ein Device über `FakeAudio` mit `st`; `opens` zählt die Öffnungen.
pub fn fake_device(name: &str, reg: Register, st: &Rc<RefCell<FakeState>>, opens: &Rc<Cell<u32>>) -> Device {
    let mut d = new_device(temp_dir(name), Arc::new(reg));
    let (st, opens) = (Rc::clone(st), Rc::clone(opens));
    d.open = Box::new(move || {
        opens.set(opens.get() + 1);
        Ok(Box::new(FakeAudio(Rc::clone(&st))) as Box<dyn AudioOut>)
    });
    d
}

/// Ordner `data/` des Repos (Go: `data.Files`).
pub fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data")
}

/// `public/` des Repos (Go: `repoPublic`).
pub fn public_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public")
}

/// Das Register aus den echten Dateien data/sounds.json und data/monarch.json.
pub fn real_register() -> Register {
    let dir = data_dir();
    let read = |name: &str| std::fs::read(dir.join(name)).map_err(|e| e.to_string());
    load_register(&read).expect("Register aus data/")
}
