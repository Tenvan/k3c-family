//! Ton-Schnittstelle: Klänge laden (OGG, MP3, WAV) und als Stimmen mit Lautstärke und Schleife spielen.
//!
//! `open_audio()` liefert das Gerät oder bei Fehler [`NullAudio`] mit Warnung; headless gibt es nur `NullAudio`.
//! Fällt das Gerät im Lauf aus, meldet `failed()` den Grund und alles läuft stumm weiter (wie Go `audio/output.go`,
//! B-805). Der Ton hängt nicht an `run_window`. Das Gerät selbst steht in `backend/sound.rs`.

use crate::log;

/// Handle eines geladenen Klangs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SoundId(pub u32);

/// Handle einer spielenden Stimme (je `play` neu, nie wiederverwendet).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VoiceId(pub u32);

/// Tonausgabe: Klänge laden, Stimmen spielen, Lautstärke je Stimme (0 … 1, wie Go `Player.SetVolume`).
pub trait AudioOut {
    /// Dekodiert OGG, MP3 oder WAV (Rate wie die Datei, Spiel-Dateien haben 44,1 kHz).
    fn load(&mut self, bytes: &[u8]) -> Result<SoundId, String>;
    /// Spielt den Klang einmal.
    fn play(&mut self, sound: SoundId, volume: f32) -> VoiceId;
    /// Spielt den Klang in Schleife bis `stop`.
    fn play_loop(&mut self, sound: SoundId, volume: f32) -> VoiceId;
    fn set_volume(&mut self, voice: VoiceId, volume: f32);
    fn stop(&mut self, voice: VoiceId);
    /// Wahr, solange die Stimme noch klingt.
    fn playing(&self, voice: VoiceId) -> bool;
    /// Grund, wenn das Gerät ausgefallen ist (dann bleibt alles stumm).
    fn failed(&self) -> Option<String>;
}

/// Stumme Tonausgabe: zählt nur, `playing` ist immer falsch, nie ein Fehler.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NullAudio {
    pub loads: u32,
    pub plays: u64,
    voices: u32,
}

impl AudioOut for NullAudio {
    fn load(&mut self, _bytes: &[u8]) -> Result<SoundId, String> {
        self.loads += 1;
        Ok(SoundId(self.loads - 1))
    }

    fn play(&mut self, _sound: SoundId, _volume: f32) -> VoiceId {
        self.plays += 1;
        self.voices += 1;
        VoiceId(self.voices - 1)
    }

    fn play_loop(&mut self, sound: SoundId, volume: f32) -> VoiceId {
        self.play(sound, volume)
    }

    fn set_volume(&mut self, _voice: VoiceId, _volume: f32) {}

    fn stop(&mut self, _voice: VoiceId) {}

    fn playing(&self, _voice: VoiceId) -> bool {
        false
    }

    fn failed(&self) -> Option<String> {
        None
    }
}

/// Öffnet das Standard-Tongerät; fehlt es, stumm mit Warnung.
pub fn open_audio() -> Box<dyn AudioOut> {
    open_audio_with(crate::backend::open_sound)
}

/// Wie [`open_audio`] mit eigenem Öffner (Test des Fehlerpfads).
pub fn open_audio_with(open: impl FnOnce() -> Result<Box<dyn AudioOut>, String>) -> Box<dyn AudioOut> {
    match open() {
        Ok(a) => a,
        Err(e) => {
            log::warn("🤒 Tongerät nicht verfügbar, Spiel läuft stumm", &[("grund", &e)]);
            Box::new(NullAudio::default())
        }
    }
}

#[cfg(test)]
#[path = "audio_tests.rs"]
mod tests;
