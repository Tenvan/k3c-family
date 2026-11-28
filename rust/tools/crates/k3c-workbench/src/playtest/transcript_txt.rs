//! `transcript.txt` (Port von `transcript_txt.go`): die lesbare Fassung neben `transcript.json`, Aufnahmezeit mm:ss, ein
//! Satz je Zeile.

use std::fmt::Write;
use std::path::Path;

use super::stats::Transcript;

pub(super) const FILE_TRANSCRIPT_TEXT: &str = "transcript.txt";

/// Zeilen `[mm:ss] Satz`; die Zeit zählt ab Beginn von `voice.wav`.
pub fn transcript_text(transcript: &Transcript) -> String {
    let mut out = String::new();
    for segment in transcript.segments.as_deref().unwrap_or_default() {
        let seconds = (segment.start_ms.saturating_sub(transcript.audio_start_ms) / 1000).max(0);
        let text = segment.text.trim();
        if !text.is_empty() {
            // Schreiben in eine Zeichenkette scheitert nicht.
            let _ = writeln!(out, "[{:02}:{:02}] {text}", seconds / 60, seconds % 60);
        }
    }
    out
}

/// Speichert `transcript.txt` atomar (erst `.tmp`, dann umbenennen).
pub(super) fn write_transcript_text(dir: &Path, transcript: &Transcript) -> std::io::Result<()> {
    let tmp = dir.join(format!("{FILE_TRANSCRIPT_TEXT}.tmp"));
    std::fs::write(&tmp, transcript_text(transcript))?;
    std::fs::rename(&tmp, dir.join(FILE_TRANSCRIPT_TEXT))
}

#[cfg(test)]
#[path = "transcript_txt_tests.rs"]
mod tests;
