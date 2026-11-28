//! Pegel des Mikros (Port von `level.go`).

use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// Kopf einer PCM-WAV von ffmpeg.
const WAV_HEADER: u64 = 44;
/// Letzte Sekunde (16 kHz, 16 Bit, mono) in Bytes.
const LEVEL_WINDOW: u64 = 16000 * 2;
/// dBFS, darunter gilt der Pegel als 0.
const LEVEL_FLOOR: f64 = -60.0;

/// Pegel der letzten Sekunde einer laufend geschriebenen `voice.wav` in Prozent (0 bis 100): der Effektivwert in dBFS
/// von -60 bis 0 auf die Skala gelegt. Nicht lesbar oder noch leer: 0.
pub(super) fn wav_level(path: &Path) -> i64 {
    read_tail(path).map_or(0, |pcm| level_percent(&pcm))
}

fn read_tail(path: &Path) -> Option<Vec<u8>> {
    let mut file = std::fs::File::open(path).ok()?;
    let size = file.metadata().ok()?.len();
    if size <= WAV_HEADER + 2 {
        return None;
    }
    let mut n = (size - WAV_HEADER).min(LEVEL_WINDOW);
    n -= n % 2;
    file.seek(SeekFrom::Start(size - n)).ok()?;
    let mut buf = Vec::new();
    file.take(n).read_to_end(&mut buf).ok()?;
    Some(buf)
}

/// Rechnet little-endian `i16`-Proben in 0 bis 100.
fn level_percent(pcm: &[u8]) -> i64 {
    let samples = pcm.len() / 2;
    if samples == 0 {
        return 0;
    }
    let sum: f64 = pcm
        .as_chunks::<2>()
        .0
        .iter()
        .map(|c| {
            let v = f64::from(i16::from_le_bytes(*c));
            v * v
        })
        .sum();
    #[allow(clippy::cast_precision_loss)] // höchstens eine Sekunde Proben
    let rms = (sum / samples as f64).sqrt();
    if rms < 1.0 {
        return 0;
    }
    let db = 20.0 * (rms / 32768.0).log10();
    let percent = (db - LEVEL_FLOOR) / -LEVEL_FLOOR * 100.0;
    #[allow(clippy::cast_possible_truncation)] // auf 0 bis 100 begrenzt
    let percent = percent.round().clamp(0.0, 100.0) as i64;
    percent
}

#[cfg(test)]
#[path = "level_tests.rs"]
mod tests;
