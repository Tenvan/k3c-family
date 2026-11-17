//! Tongerät über rodio (cpal): Klänge beim Laden dekodieren und als Samples halten, je Stimme ein Player mit
//! Lautstärke; Schleife über eine unendlich wiederholte Quelle. Einziger Ort mit `rodio`.
//!
//! Gerätefehler (Fehler-Rückruf von cpal) setzen `failed` mit Warnung genau einmal; danach spielt nichts mehr
//! (stumm bis zum Neustart wie Go `audio/output.go`, B-805).

use std::io::Cursor;
use std::num::NonZero;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rodio::{CpalErrorKind, Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};

use crate::audio::{AudioOut, SoundId, VoiceId};
use crate::log;

/// Höchstens so viele gleichzeitige Stimmen; darüber entfällt der neue Klang (Last im Mixer begrenzt).
pub const MAX_VOICES: usize = 32;

/// Fatale Gerätefehler (Gerät weg, Rechte, Backend); Aussetzer wie Xrun oder Gerätewechsel sind es nicht.
fn fatal(kind: CpalErrorKind) -> bool {
    matches!(
        kind,
        CpalErrorKind::DeviceNotAvailable
            | CpalErrorKind::HostUnavailable
            | CpalErrorKind::PermissionDenied
            | CpalErrorKind::BackendError
    )
}

/// Lautstärke auf 0 … 1 (NaN = stumm), wie Go `Player.SetVolume`.
fn vol(v: f32) -> f32 {
    if v.is_nan() { 0.0 } else { v.clamp(0.0, 1.0) }
}

/// Ein dekodierter Klang: verschränkte Samples (f32), Kanäle, Rate.
#[derive(Clone, Debug)]
pub struct Pcm {
    pub samples: Arc<[f32]>,
    pub channels: u16,
    pub rate: u32,
}

/// Dekodiert OGG (Vorbis), MP3 oder WAV vollständig in den Speicher.
pub fn decode(bytes: &[u8]) -> Result<Pcm, String> {
    let d = Decoder::new(Cursor::new(bytes.to_vec())).map_err(|e| e.to_string())?;
    let (channels, rate) = (d.channels().get(), d.sample_rate().get());
    let samples: Arc<[f32]> = d.collect();
    if samples.is_empty() {
        return Err("Klang ohne Samples".into());
    }
    Ok(Pcm {
        samples,
        channels,
        rate,
    })
}

/// Quelle über die geteilten Samples eines Klangs (ohne Kopie je Stimme).
#[derive(Clone)]
struct Play {
    pcm: Pcm,
    pos: usize,
}

impl Iterator for Play {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let v = self.pcm.samples.get(self.pos).copied();
        self.pos += 1;
        v
    }
}

impl Source for Play {
    fn current_span_len(&self) -> Option<usize> {
        Some(self.pcm.samples.len().saturating_sub(self.pos))
    }
    fn channels(&self) -> NonZero<u16> {
        NonZero::new(self.pcm.channels).unwrap_or(NonZero::<u16>::MIN)
    }
    fn sample_rate(&self) -> NonZero<u32> {
        NonZero::new(self.pcm.rate).unwrap_or(NonZero::<u32>::MIN)
    }
    fn total_duration(&self) -> Option<Duration> {
        let frames = self.pcm.samples.len() as f64 / f64::from(self.pcm.channels.max(1));
        Some(Duration::from_secs_f64(frames / f64::from(self.pcm.rate.max(1))))
    }
}

/// Tonausgabe am Gerät.
pub struct Sound {
    // Stimmen vor dem Gerät freigeben (Reihenfolge der Felder = Reihenfolge beim Freigeben).
    voices: Vec<(VoiceId, Player)>,
    sink: MixerDeviceSink,
    sounds: Vec<Pcm>,
    next: u32,
    failed: Arc<Mutex<Option<String>>>,
}

/// Öffnet das Standard-Tongerät.
pub fn open() -> Result<Box<dyn AudioOut>, String> {
    let failed = Arc::new(Mutex::new(None::<String>));
    let flag = failed.clone();
    let mut sink = DeviceSinkBuilder::from_default_device()
        .map_err(|e| e.to_string())?
        .with_error_callback(move |e| {
            if !fatal(e.kind()) {
                return;
            }
            let mut f = flag.lock().unwrap_or_else(|p| p.into_inner());
            if f.is_none() {
                log::warn("🤒 Tongerät ausgefallen, Spiel läuft stumm weiter", &[("grund", &e)]);
                *f = Some(e.to_string());
            }
        })
        .open_stream()
        .map_err(|e| e.to_string())?;
    sink.log_on_drop(false);
    Ok(Box::new(Sound {
        voices: Vec::new(),
        sink,
        sounds: Vec::new(),
        next: 0,
        failed,
    }))
}

impl Sound {
    fn start(&mut self, sound: SoundId, volume: f32, looped: bool) -> VoiceId {
        let id = VoiceId(self.next);
        self.next += 1;
        self.voices.retain(|(_, p)| !p.empty());
        let Some(pcm) = self.sounds.get(sound.0 as usize) else {
            return id;
        };
        if self.failed().is_some() || self.voices.len() >= MAX_VOICES {
            return id;
        }
        let player = Player::connect_new(self.sink.mixer());
        player.set_volume(vol(volume));
        let src = Play {
            pcm: pcm.clone(),
            pos: 0,
        };
        if looped {
            player.append(src.repeat_infinite());
        } else {
            player.append(src);
        }
        self.voices.push((id, player));
        id
    }

    fn voice(&self, v: VoiceId) -> Option<&Player> {
        self.voices.iter().find(|(id, _)| *id == v).map(|(_, p)| p)
    }
}

impl AudioOut for Sound {
    fn load(&mut self, bytes: &[u8]) -> Result<SoundId, String> {
        self.sounds.push(decode(bytes)?);
        Ok(SoundId(self.sounds.len() as u32 - 1))
    }

    fn play(&mut self, sound: SoundId, volume: f32) -> VoiceId {
        self.start(sound, volume, false)
    }

    fn play_loop(&mut self, sound: SoundId, volume: f32) -> VoiceId {
        self.start(sound, volume, true)
    }

    fn set_volume(&mut self, voice: VoiceId, volume: f32) {
        if let Some(p) = self.voice(voice) {
            p.set_volume(vol(volume));
        }
    }

    fn stop(&mut self, voice: VoiceId) {
        if let Some(i) = self.voices.iter().position(|(id, _)| *id == voice) {
            self.voices.swap_remove(i).1.stop();
        }
    }

    fn playing(&self, voice: VoiceId) -> bool {
        self.failed().is_none() && self.voice(voice).is_some_and(|p| !p.empty())
    }

    fn failed(&self) -> Option<String> {
        self.failed.lock().unwrap_or_else(|p| p.into_inner()).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nur_echte_geraetefehler_schalten_stumm() {
        assert!(fatal(CpalErrorKind::DeviceNotAvailable));
        assert!(!fatal(CpalErrorKind::Xrun) && !fatal(CpalErrorKind::DeviceChanged));
        assert!(!fatal(CpalErrorKind::StreamInvalidated) && !fatal(CpalErrorKind::RealtimeDenied));
    }

    #[test]
    fn lautstaerke_bleibt_in_null_bis_eins() {
        assert_eq!(
            [vol(-1.0), vol(0.5), vol(3.0), vol(f32::NAN), vol(f32::INFINITY)],
            [0.0, 0.5, 1.0, 0.0, 1.0]
        );
    }

    #[test]
    fn schleife_beginnt_von_vorn() {
        let pcm = Pcm {
            samples: Arc::from([0.1f32, 0.2, 0.3, 0.4]),
            channels: 2,
            rate: 4,
        };
        let p = Play { pcm, pos: 0 };
        assert_eq!(p.total_duration(), Some(Duration::from_millis(500)));
        let got: Vec<f32> = p.repeat_infinite().take(6).collect();
        assert_eq!(got, [0.1, 0.2, 0.3, 0.4, 0.1, 0.2]);
    }
}
