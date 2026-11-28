//! Wächter der laufenden Aufnahme (Port von `watch.go`).

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use serde::Serialize;

use super::exec::Proc;
use super::level::wav_level;
use super::record::{Recorder, Waited};
use super::{FILE_VIDEO, FILE_VOICE, Signal, lock};

/// So viele Sekunden zeigen die Diagramme.
const STAT_SAMPLES: usize = 90;
/// So viele Takte keine Warnung (Mikro und Encoder starten).
const WARMUP: u32 = 4;
/// So viele Takte ohne Zuwachs gelten als „Ton steht“.
const IDLE_VOICE: u32 = 4;
/// So viele für „Bild steht“ (Matroska schreibt in Blöcken von höchstens 1 s, siehe `ffmpeg_args`).
const IDLE_VIDEO: u32 = 5;

/// Messwerte der laufenden Aufnahme: Bytes je Sekunde, die in `voice.wav` und `video.mkv` ankommen. Die Listen sind im
/// JSON nie `null`.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub voice_bytes: i64,
    pub video_bytes: i64,
    /// Bytes je Sekunde, älteste zuerst.
    pub voice_rate: Vec<i64>,
    pub video_rate: Vec<i64>,
    /// Pegel der letzten Sekunde in Prozent (0 bis 100).
    pub voice_level: Vec<i64>,
    /// Leer, solange alles ankommt.
    pub warn: String,
    /// Mikro-Session: kein Bild, der Wächter überwacht nur den Ton.
    pub no_video: bool,
}

/// Was der Wächter zwischen den Takten braucht.
#[derive(Default)]
pub(super) struct WatchState {
    ticks: u32,
    idle_voice: u32,
    idle_video: u32,
}

/// Ein Takt: Größen der Dateien, lebt ffmpeg noch, was sagte es zuletzt, Pegel.
pub(super) struct Sample<'a> {
    pub voice: i64,
    pub video: i64,
    pub alive: bool,
    pub tail: &'a str,
    pub level: i64,
}

fn push<T>(list: &mut Vec<T>, value: T) {
    list.push(value);
    if list.len() > STAT_SAMPLES {
        list.drain(..list.len() - STAT_SAMPLES);
    }
}

fn idle_count(n: u32, delta: i64) -> u32 {
    if delta > 0 { 0 } else { n.saturating_add(1) }
}

impl Stats {
    /// Verarbeitet einen Takt.
    pub(super) fn advance(&mut self, w: &mut WatchState, s: &Sample) {
        push(&mut self.voice_level, s.level);
        w.ticks = w.ticks.saturating_add(1);
        let voice = (s.voice - self.voice_bytes).max(0);
        let video = (s.video - self.video_bytes).max(0);
        (self.voice_bytes, self.video_bytes) = (s.voice, s.video);
        push(&mut self.voice_rate, voice);
        push(&mut self.video_rate, video);
        w.idle_voice = idle_count(w.idle_voice, voice);
        w.idle_video = idle_count(w.idle_video, video);
        if self.no_video {
            w.idle_video = 0;
        }
        let (voice_dead, video_dead) = (w.idle_voice >= IDLE_VOICE, w.idle_video >= IDLE_VIDEO);
        self.warn = if !s.alive {
            let mut warn = "ffmpeg ist beendet – es wird nichts mehr aufgezeichnet".to_owned();
            if !s.tail.is_empty() {
                warn = format!("{warn} ({})", s.tail);
            }
            warn
        } else if w.ticks <= WARMUP {
            String::new()
        } else if voice_dead && video_dead {
            format!("Ton und Bild kommen nicht mehr an (seit {} s)", w.idle_video)
        } else if voice_dead {
            format!("Ton kommt nicht mehr an (seit {} s) – Mikro prüfen", w.idle_voice)
        } else if video_dead {
            format!("Bild kommt nicht mehr an (seit {} s)", w.idle_video)
        } else {
            String::new()
        };
    }
}

fn file_size(path: &Path) -> i64 {
    std::fs::metadata(path).map_or(0, |m| i64::try_from(m.len()).unwrap_or(i64::MAX))
}

impl Recorder {
    /// Misst je Takt, was in `voice.wav` und `video.mkv` ankommt, und merkt, wenn ffmpeg stirbt; die Warnung steht im
    /// Status und einmal im Log. Endet mit der Aufnahme (`done`).
    pub(super) fn watch(&self, dir: &Path, rec: &Arc<Waited>, done: &Signal<()>) {
        let interval = Some(self.config.interval)
            .filter(|i| !i.is_zero())
            .unwrap_or(Duration::from_secs(1));
        let mut watch = WatchState::default();
        let mut warned = String::new();
        while done.wait_timeout(interval).is_none() {
            let voice = file_size(&dir.join(FILE_VOICE));
            let video = file_size(&dir.join(FILE_VIDEO));
            let level = wav_level(&dir.join(FILE_VOICE));
            let (alive, tail) = (!rec.exited(), rec.tail());
            let warn = {
                let mut state = lock(&self.state);
                let sample = Sample {
                    voice,
                    video,
                    alive,
                    tail: &tail,
                    level,
                };
                state.stats.advance(&mut watch, &sample);
                state.stats.warn.clone()
            };
            if !warn.is_empty() && warn != warned {
                self.log("warn", &format!("⏳ Aufnahme: {warn}"), &[]);
            }
            warned = warn;
        }
    }
}

#[cfg(test)]
#[path = "watch_tests.rs"]
mod tests;
