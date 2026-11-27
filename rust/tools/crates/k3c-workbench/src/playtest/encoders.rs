//! Wahl des Encoders (Port von `encoders.go`).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::devices::{Probe, last_error, strings};
use super::{LogFn, lock};

/// Bezeichnungen der Encoder, wie sie in `session.json` und in der Anzeige stehen.
pub(super) const ENCODER_QSV: &str = "h264_qsv 1080p30";
pub(super) const ENCODER_X264: &str = "libx264 ultrafast 720p30";
pub(super) const ENCODER_NVENC: &str = "h264_nvenc 1080p30";
pub(super) const ENCODER_AMF: &str = "h264_amf 1080p30";

/// Hardware-Encoder zuerst (schont die CPU beim Spielen), libx264 als sicherer Rückfall.
pub(super) const ENCODER_ORDER: [&str; 4] = [ENCODER_NVENC, ENCODER_AMF, ENCODER_QSV, ENCODER_X264];

/// Skalierung (läuft nach `hwdownload`) und Codec-Argumente des Encoders.
pub(super) fn encoder_spec(encoder: &str) -> (&'static str, &'static [&'static str]) {
    const HD: &str = "scale=1920:1080:flags=fast_bilinear,format=nv12";
    match encoder {
        ENCODER_X264 => (
            "scale=1280:720:flags=fast_bilinear,format=yuv420p",
            &["-c:v", "libx264", "-preset", "ultrafast", "-crf", "28"],
        ),
        ENCODER_NVENC => (
            HD,
            &[
                "-c:v",
                "h264_nvenc",
                "-preset",
                "p1",
                "-tune",
                "ll",
                "-rc",
                "vbr",
                "-cq",
                "28",
                "-b:v",
                "0",
            ],
        ),
        ENCODER_AMF => (
            HD,
            &[
                "-c:v", "h264_amf", "-quality", "speed", "-rc", "cqp", "-qp_i", "26", "-qp_p", "28",
            ],
        ),
        _ => (HD, &["-c:v", "h264_qsv", "-global_quality", "26", "-look_ahead", "0"]),
    }
}

/// Kurzer echter Encode (10 Bilder aus einer Testquelle), der zeigt, ob sich der Encoder öffnen lässt.
fn encoder_probe_args(ffmpeg: &str, encoder: &str) -> Vec<String> {
    let (scale, codec) = encoder_spec(encoder);
    let source = "color=c=black:s=1920x1080:r=30";
    let mut args = strings(&[
        ffmpeg,
        "-hide_banner",
        "-f",
        "lavfi",
        "-i",
        source,
        "-frames:v",
        "10",
        "-vf",
        scale,
    ]);
    args.extend(strings(codec));
    args.extend(strings(&["-f", "null", "-"]));
    args
}

/// Der erste Encoder, der sich wirklich öffnen lässt; libx264 gilt ohne Probe als Rückfall. Dazu die verworfenen mit
/// Grund fürs Log.
pub fn pick_encoder(run: &Probe, ffmpeg: &str) -> (&'static str, Vec<String>) {
    let mut failed = Vec::new();
    for encoder in ENCODER_ORDER.into_iter().take_while(|e| *e != ENCODER_X264) {
        let (out, result) = run(&encoder_probe_args(ffmpeg, encoder));
        if result.is_ok() {
            return (encoder, failed);
        }
        let codec = encoder.split_whitespace().next().unwrap_or_default();
        failed.push(format!("{codec}: {}", last_error(&out)));
    }
    (ENCODER_X264, failed)
}

/// ffmpeg-Pfad → Encoder. Die Probe öffnet GPU-Sitzungen, einmal je Prozess genügt; Geräteliste und Aufnahme teilen
/// ihn, damit die Liste nie neben einer laufenden Aufnahme erneut probt.
#[derive(Clone, Default)]
pub struct PickCache(Arc<Mutex<HashMap<String, String>>>);

impl PickCache {
    /// Aus dem Cache, sonst nach der Probe; die verworfenen stehen dann mit Grund im Log.
    pub fn pick(&self, run: &Probe, ffmpeg: &str, log: Option<&LogFn>) -> String {
        let mut cache = lock(&self.0);
        if let Some(encoder) = cache.get(ffmpeg) {
            return encoder.clone();
        }
        let (encoder, failed) = pick_encoder(run, ffmpeg);
        if let Some(log) = log {
            for reason in failed {
                log("info", &format!("🐛 Encoder nicht nutzbar: {reason}"), &[]);
            }
            log("info", &format!("🐛 Encoder gewählt: {encoder}"), &[]);
        }
        cache.insert(ffmpeg.to_owned(), encoder.to_owned());
        encoder.to_owned()
    }
}

#[cfg(test)]
#[path = "encoders_tests.rs"]
mod tests;
