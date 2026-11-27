//! Aufnehmbare Bildschirme und die Prüfung der Geräte vor dem Start (Port von `devices.go`).

use std::sync::{Arc, LazyLock};

use regex::Regex;
use serde::Serialize;

use super::Error;

/// Höchstens so viele Monitore werden probiert.
const MAX_MONITORS: usize = 8;
/// Länge der Mikroprobe in Sekunden.
const MIC_PROBE_SECS: &str = "0.4";

static MONITOR_SIZE: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"Video:[^\n]*?([0-9]{3,5})x([0-9]{3,5})").ok());

/// Ein kurzer ffmpeg-Aufruf: stdout und stderr, dazu ob er gelang.
pub type Probe = Arc<dyn Fn(&[String]) -> (String, Result<(), String>) + Send + Sync>;

/// Aufnehmbarer Bildschirm (ddagrab `output_idx`), geprüft mit einem echten Probebild.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Monitor {
    pub index: usize,
    pub width: u32,
    pub height: u32,
    pub label: String,
}

pub(super) fn strings(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|p| (*p).to_owned()).collect()
}

/// Der Aufruf, der ein Bild von Monitor `index` holt (nichts wird geschrieben).
pub(super) fn monitor_probe_args(ffmpeg: &str, index: i64) -> Vec<String> {
    let input = format!("ddagrab=output_idx={index}:framerate=1");
    strings(&[
        ffmpeg,
        "-hide_banner",
        "-f",
        "lavfi",
        "-i",
        &input,
        "-frames:v",
        "1",
        "-f",
        "null",
        "-",
    ])
}

/// Der Aufruf, der kurz vom DirectShow-Gerät `mic` liest (nichts wird geschrieben).
pub(super) fn mic_probe_args(ffmpeg: &str, mic: &str) -> Vec<String> {
    let input = format!("audio={mic}");
    strings(&[
        ffmpeg,
        "-hide_banner",
        "-f",
        "dshow",
        "-i",
        &input,
        "-t",
        MIC_PROBE_SECS,
        "-f",
        "null",
        "-",
    ])
}

/// Bildgröße aus der Ausgabe eines Probelaufs; (0, 0), wenn sie fehlt.
pub(super) fn parse_monitor_size(out: &str) -> (u32, u32) {
    let Some(found) = MONITOR_SIZE.as_ref().and_then(|re| re.captures(out)) else {
        return (0, 0);
    };
    let number = |i: usize| found.get(i).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
    (number(1), number(2))
}

/// Probiert die Monitore 0, 1, … bis zum ersten, der sich nicht aufnehmen lässt (die Indizes laufen lückenlos). Leer,
/// wenn schon Monitor 0 scheitert.
pub(super) fn list_monitors(run: &Probe, ffmpeg: &str) -> Vec<Monitor> {
    let mut out = Vec::new();
    for (index, number) in (0..MAX_MONITORS).zip(0i64..) {
        let (text, result) = run(&monitor_probe_args(ffmpeg, number));
        if result.is_err() {
            break;
        }
        let (width, height) = parse_monitor_size(&text);
        let mut label = format!("Monitor {}", index + 1);
        if width > 0 {
            label.push_str(&format!(" ({width}×{height})"));
        }
        out.push(Monitor {
            index,
            width,
            height,
            label,
        });
    }
    out
}

/// Die letzte nichtleere Zeile der Ausgabe, wo ffmpeg den Grund nennt.
pub(super) fn last_error(out: &str) -> String {
    out.lines()
        .rev()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or_default()
        .to_owned()
}

/// Prüft vor dem Start gegen die Geräte: Das Mikro lässt sich öffnen und der Monitor liefert ein Bild
/// (`monitor` unter 0: Mikro-Session ohne Bild). Der Fehler nennt Gerät und Grund.
pub fn check_devices(run: &Probe, ffmpeg: &str, mic: &str, monitor: i64) -> Result<(), Error> {
    let (out, result) = run(&mic_probe_args(ffmpeg, mic));
    if result.is_err() {
        return Err(Error::Other(format!(
            "Mikrofon „{mic}“ lässt sich nicht öffnen (abgezogen, belegt oder umbenannt?): {}",
            last_error(&out)
        )));
    }
    if monitor < 0 {
        return Ok(());
    }
    let (out, result) = run(&monitor_probe_args(ffmpeg, monitor));
    if result.is_err() {
        return Err(Error::Other(format!(
            "Monitor {} lässt sich nicht aufnehmen (nicht angeschlossen?): {}",
            monitor + 1,
            last_error(&out)
        )));
    }
    Ok(())
}

#[cfg(test)]
#[path = "devices_tests.rs"]
mod tests;
