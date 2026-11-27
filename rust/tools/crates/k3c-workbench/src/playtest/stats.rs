//! Statistik der Auswertung (Port von `stats.go`) samt der Form von `transcript.json`, in der sie steht. Den Lauf
//! selbst bringt RU69.14.

use std::path::Path;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize, Serializer};

use super::{DIR_FRAMES, Error, FILE_TRANSCRIPT, go_json, null_default};

/// Ein Satz des Transkripts in Unix-ms der Systemuhr.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Segment {
    #[serde(deserialize_with = "null_default")]
    pub start_ms: i64,
    #[serde(deserialize_with = "null_default")]
    pub end_ms: i64,
    #[serde(deserialize_with = "null_default")]
    pub text: String,
}

/// `transcript.json`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Transcript {
    /// Unix-ms von `voice.wav` bei 0 s.
    #[serde(deserialize_with = "null_default")]
    pub audio_start_ms: i64,
    #[serde(deserialize_with = "null_default")]
    pub language: String,
    #[serde(deserialize_with = "null_default")]
    pub model: String,
    /// `null`, wenn die Datei keine Liste hat.
    pub segments: Option<Vec<Segment>>,
    /// Dauer der letzten Auswertung.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<EvalStats>,
}

/// Wie lange die letzte Auswertung gedauert hat, im Verhältnis zur Länge der Aufnahme.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct EvalStats {
    /// Transkript und Bilder.
    pub total_ms: i64,
    pub transcribe_ms: i64,
    pub frames_ms: i64,
    /// Länge der Aufnahme.
    pub audio_ms: i64,
    pub segments: usize,
    pub words: usize,
    pub frames: usize,
    /// Auswertungs-Sekunden je Aufnahme-Minute.
    #[serde(serialize_with = "go_float")]
    pub sec_per_min: f64,
    /// Auswertung geteilt durch Aufnahme (unter 1: schneller als Echtzeit).
    #[serde(serialize_with = "go_float")]
    pub factor: f64,
}

/// Ganze Zahlen ohne `.0` wie Go.
#[allow(clippy::trivially_copy_pass_by_ref)] // Signatur für `serialize_with`
fn go_float<S: Serializer>(value: &f64, s: S) -> Result<S::Ok, S::Error> {
    if value.fract() == 0.0 && value.abs() < 9e15 {
        #[allow(clippy::cast_possible_truncation)] // ganzzahlig und im Bereich
        let whole = *value as i64;
        return s.serialize_i64(whole);
    }
    s.serialize_f64(*value)
}

fn millis(duration: Duration) -> i64 {
    i64::try_from(duration.as_millis()).unwrap_or(i64::MAX)
}

/// Rechnet die Kennzahlen; ohne Aufnahmelänge bleiben Verhältnis und Faktor 0.
pub fn make_stats(
    transcribe: Duration,
    frames: Duration,
    audio_ms: i64,
    transcript: &Transcript,
    frame_count: usize,
) -> EvalStats {
    let segments = transcript.segments.as_deref().unwrap_or_default();
    let mut stats = EvalStats {
        total_ms: millis(transcribe + frames),
        transcribe_ms: millis(transcribe),
        frames_ms: millis(frames),
        audio_ms,
        segments: segments.len(),
        words: segments.iter().map(|s| s.text.split_whitespace().count()).sum(),
        frames: frame_count,
        ..EvalStats::default()
    };
    if audio_ms > 0 {
        #[allow(clippy::cast_precision_loss)] // Millisekunden einer Aufnahme
        let factor = stats.total_ms as f64 / audio_ms as f64;
        stats.factor = factor;
        stats.sec_per_min = factor * 60.0;
    }
    stats
}

/// Trägt die Statistik in `transcript.json` ein (`t0` Start, `t1` Ende des Transkripts, `t2` Ende der Bilder).
pub fn save_stats(dir: &Path, t0: SystemTime, t1: SystemTime, t2: SystemTime, audio_ms: i64) -> Result<(), Error> {
    let failed = |e: &dyn std::fmt::Display| Error::Other(e.to_string());
    let path = dir.join(FILE_TRANSCRIPT);
    let raw = std::fs::read(&path).map_err(|e| failed(&e))?;
    let mut transcript: Transcript = serde_json::from_slice(&raw).map_err(|e| failed(&e))?;
    let frames = std::fs::read_dir(dir.join(DIR_FRAMES)).map_or(0, Iterator::count);
    let since = |from: SystemTime, to: SystemTime| to.duration_since(from).unwrap_or_default();
    let stats = make_stats(since(t0, t1), since(t1, t2), audio_ms, &transcript, frames);
    transcript.stats = Some(stats);
    let out = go_json(&transcript).map_err(|e| failed(&e))?;
    let tmp = dir.join(format!("{FILE_TRANSCRIPT}.tmp"));
    std::fs::write(&tmp, out).map_err(|e| failed(&e))?;
    std::fs::rename(&tmp, &path).map_err(|e| failed(&e))
}

#[cfg(test)]
#[path = "stats_tests.rs"]
mod tests;
