//! Bild bei einem Zeitpunkt (Port von `frames.go`, dazu `FrameArgs` und `CaptureStart` aus `transcribe.go`).

use std::path::PathBuf;

use super::exec::Spawn;
use super::ffmpeg_find::LookPath;
use super::playtest::Meta;
use super::{DIR_FRAMES, Error, FILE_META, FILE_VIDEO, Store, no_ffmpeg};

/// So lange läuft ffmpeg vor `session.json` › `startMs` (Settle), wenn `captureMs` fehlt.
const LEAD_MS: i64 = 2000;

/// Ein Bild bei `seconds` Videozeit; `-ss` steht vor `-i` (schnelles Suchen).
pub fn frame_args(ffmpeg: &str, seconds: f64, out: &str) -> Vec<String> {
    let at = format!("{seconds:.3}");
    [
        ffmpeg,
        "-hide_banner",
        "-loglevel",
        "error",
        "-y",
        "-ss",
        &at,
        "-i",
        FILE_VIDEO,
        "-frames:v",
        "1",
        "-q:v",
        "3",
        out,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

/// Unix-ms bei 0 s von `voice.wav` und `video.mkv`: der Start von ffmpeg, sonst `startMs` abzüglich der Anlaufzeit.
pub fn capture_start(meta: &Meta) -> i64 {
    if meta.capture_ms > 0 {
        meta.capture_ms
    } else if meta.start_ms > 0 {
        meta.start_ms - LEAD_MS
    } else {
        0
    }
}

/// Zieht das Bild bei `ts_ms` (Unix-ms) aus `video.mkv` nach `frames/t-<ts_ms>.jpg` und liefert den Pfad; ein
/// vorhandenes Bild wird wiederverwendet. Zeitpunkte vor dem Start der Aufnahme gelten als 0 s. Blockiert.
pub fn frame_at(store: &Store, look_path: &LookPath, spawn: &Spawn, id: &str, ts_ms: i64) -> Result<PathBuf, Error> {
    let session = store.get(id)?;
    if !session.has_video {
        return Err(Error::NoVideo);
    }
    let ffmpeg = look_path("ffmpeg").map_err(no_ffmpeg)?;
    let dir = store.root.join(&session.id);
    let name = format!("t-{ts_ms}.jpg");
    let out = dir.join(DIR_FRAMES).join(&name);
    if out.is_file() {
        return Ok(out);
    }
    let failed = |reason: String| Error::Other(format!("Bild nicht erzeugt: {reason}"));
    std::fs::create_dir_all(dir.join(DIR_FRAMES)).map_err(|e| Error::Other(e.to_string()))?;
    let meta = std::fs::read(dir.join(FILE_META))
        .ok()
        .and_then(|raw| Meta::parse(&raw))
        .unwrap_or_default();
    #[allow(clippy::cast_precision_loss)] // Millisekunden einer Aufnahme
    let seconds = ts_ms.saturating_sub(capture_start(&meta)).max(0) as f64 / 1000.0;
    let relative = PathBuf::from(DIR_FRAMES).join(&name).to_string_lossy().into_owned();
    let proc = spawn(&frame_args(&ffmpeg, seconds, &relative), &dir).map_err(|e| failed(e.to_string()))?;
    proc.wait().map_err(|e| failed(format!("{e}: {}", proc.tail())))?;
    std::fs::metadata(&out).map_err(|e| failed(e.to_string()))?;
    Ok(out)
}

#[cfg(test)]
#[path = "frames_tests.rs"]
mod tests;
