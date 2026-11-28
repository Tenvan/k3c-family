//! Zeitleiste (Port von `timeline.go`, B-1223): Sätze des Transkripts, Marker aus `markers.jsonl` und deren Bilder in
//! zeitlicher Folge. Zeiten in Unix-ms (`ms`) und in Videozeit ab Aufnahmestart (`videoMs`, für den Sprung im Player).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::frames::capture_start;
use super::playtest::valid_id;
use super::record::write_meta;
use super::stats::{EvalStats, Transcript};
use super::transcribe::frame_name;
use super::transcribe_job::read_meta;
use super::{Error, FILE_MARKERS, FILE_TRANSCRIPT, Store, null_default};

/// Beschreibungen der Marker-Bilder (Schreiber: [`super::vision`]).
pub(super) const FILE_DESCRIPTIONS: &str = "descriptions.json";

#[allow(clippy::trivially_copy_pass_by_ref)] // Signatur für `skip_serializing_if`
fn zero<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

/// Eine Zeile von `markers.jsonl` (Schreiber: das Spiel; Feldnamen wie dort).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Marker {
    #[serde(deserialize_with = "null_default")]
    pub ts: i64,
    /// `pad` oder `ruckler`.
    #[serde(deserialize_with = "null_default")]
    pub kind: String,
    #[serde(deserialize_with = "null_default")]
    pub player: i64,
    #[serde(deserialize_with = "null_default")]
    pub world: String,
    #[serde(deserialize_with = "null_default")]
    pub island: i64,
    #[serde(deserialize_with = "null_default")]
    pub stage: i64,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub island_id: String,
    #[serde(skip_serializing_if = "zero", deserialize_with = "null_default")]
    pub day: i64,
    #[serde(deserialize_with = "null_default")]
    pub night: bool,
    #[serde(deserialize_with = "null_default")]
    pub phase: String,
    #[serde(deserialize_with = "null_default")]
    pub worst_frame_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_ms: Option<f64>,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub save: String,
}

/// Ein Eintrag der Zeitleiste.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    /// `sprache` oder `marker`.
    pub kind: String,
    pub ms: i64,
    pub video_ms: i64,
    /// Nur Sprache.
    #[serde(skip_serializing_if = "zero")]
    pub end_ms: i64,
    /// Sprache: der Satz; Marker: Kurzbeschreibung.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<Marker>,
    /// Marker: Zeilennummer ab 1.
    #[serde(skip_serializing_if = "zero")]
    pub nr: i64,
    /// Marker: Bild relativ zur Session, wenn es existiert.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub frame: String,
    /// Marker: Beschreibung des Bilds durch das Bildmodell.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub desc: String,
    /// Marker: Bild abgewählt (nicht für den Chat).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub skip: bool,
}

/// Die Zeitleiste einer Session.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeline {
    pub id: String,
    /// Unix-ms bei 0 s Videozeit.
    pub capture_ms: i64,
    pub has_transcript: bool,
    /// Dauer der letzten Auswertung.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<EvalStats>,
    pub entries: Vec<Entry>,
}

/// Beschreibung eines Marker-Bilds.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FrameDesc {
    #[serde(deserialize_with = "null_default")]
    pub text: String,
    #[serde(deserialize_with = "null_default")]
    pub model: String,
    /// Dauer der Beschreibung.
    #[serde(deserialize_with = "null_default")]
    pub ms: i64,
}

/// Führende Zahl eines Schlüssels wie Go `Sscanf("%d")`.
fn leading_number(key: &str) -> Option<i64> {
    let key = key.trim_start();
    let key = key.strip_prefix('+').unwrap_or(key);
    let end = key.find(|c: char| !c.is_ascii_digit()).unwrap_or(key.len());
    key.get(..end)?.parse().ok().filter(|n| *n > 0)
}

/// Liest `descriptions.json` (Marker-Nummer → Beschreibung); fehlt oder bricht sie, ist die Karte leer.
pub(super) fn read_descriptions(dir: &Path) -> BTreeMap<i64, FrameDesc> {
    let parsed: Option<BTreeMap<String, Option<FrameDesc>>> = std::fs::read(dir.join(FILE_DESCRIPTIONS))
        .ok()
        .and_then(|raw| serde_json::from_slice(&raw).ok());
    parsed
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(key, desc)| Some((leading_number(&key)?, desc.unwrap_or_default())))
        .collect()
}

/// Liest `markers.jsonl`; fehlt die Datei, ist die Liste leer, kaputte Zeilen werden übergangen.
pub(super) fn read_markers(dir: &Path) -> Result<Vec<Marker>, Error> {
    let raw = match std::fs::read(dir.join(FILE_MARKERS)) {
        Ok(raw) => raw,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(Error::Other(e.to_string())),
    };
    Ok(String::from_utf8_lossy(&raw)
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str::<Option<Marker>>(line).ok())
        .map(Option::unwrap_or_default)
        .collect())
}

/// Kurzbeschreibung eines Markers.
fn describe(marker: &Marker) -> String {
    let who = if marker.player == 0 {
        "Spiel".to_owned()
    } else {
        format!("Spieler {}", marker.player)
    };
    let what = if marker.kind == "ruckler" { "Ruckler" } else { "Marker" };
    let mut out = format!("{what} · {who}");
    if marker.island > 0 {
        out = format!("{out} · Insel {} Stufe {}", marker.island, marker.stage);
    }
    out
}

/// Die Sätze aus `transcript.json`; ohne lesbare Datei `false`.
fn speech(dir: &Path, timeline: &mut Timeline) {
    let Some(transcript) = std::fs::read(dir.join(FILE_TRANSCRIPT))
        .ok()
        .and_then(|raw| serde_json::from_slice::<Transcript>(&raw).ok())
    else {
        return;
    };
    timeline.has_transcript = true;
    timeline.stats = transcript.stats;
    for segment in transcript.segments.unwrap_or_default() {
        timeline.entries.push(Entry {
            kind: "sprache".to_owned(),
            ms: segment.start_ms,
            end_ms: segment.end_ms,
            video_ms: segment.start_ms.saturating_sub(timeline.capture_ms),
            text: segment.text,
            ..Entry::default()
        });
    }
}

impl Store {
    /// Die Zeitleiste; ohne `transcript.json` zeigt sie nur Marker (und deren Bilder, falls schon da).
    pub fn timeline(&self, id: &str) -> Result<Timeline, Error> {
        let dir = self.session_dir(id)?;
        let meta = read_meta(&dir);
        let capture = capture_start(&meta);
        let mut timeline = Timeline {
            id: id.to_owned(),
            capture_ms: capture,
            ..Timeline::default()
        };
        speech(&dir, &mut timeline);
        let descriptions = read_descriptions(&dir);
        for (nr, marker) in (1_i64..).zip(read_markers(&dir)?) {
            let mut entry = Entry {
                kind: "marker".to_owned(),
                ms: marker.ts,
                video_ms: marker.ts.saturating_sub(capture),
                text: describe(&marker),
                marker: Some(marker),
                nr,
                ..Entry::default()
            };
            let frame = frame_name(nr);
            if dir.join(&frame).is_file() {
                entry.frame = frame;
                entry.skip = meta.skip_frames.contains(&nr);
                entry.desc = descriptions.get(&nr).map(|d| d.text.clone()).unwrap_or_default();
            }
            timeline.entries.push(entry);
        }
        timeline.entries.sort_by_key(|e| e.ms);
        Ok(timeline)
    }

    /// Pfad eines Marker-Bilds (nur `frames/marker-<nr>.jpg`).
    pub fn frame_path(&self, id: &str, nr: i64) -> Result<PathBuf, Error> {
        if !valid_id(id) || nr < 1 {
            return Err(Error::BadId);
        }
        let path = self.root.join(id).join(frame_name(nr));
        if path.is_file() { Ok(path) } else { Err(Error::NotFound) }
    }

    /// Nimmt das Bild eines Markers aus der Auswahl (`skip`) oder wieder hinein; die Datei bleibt, nur
    /// `session.json` › `skipFrames` ändert sich.
    pub fn set_frame_skip(&self, id: &str, nr: i64, skip: bool) -> Result<(), Error> {
        if !valid_id(id) || nr < 1 {
            return Err(Error::BadId);
        }
        if self.get(id)?.running {
            return Err(Error::Running);
        }
        let dir = self.root.join(id);
        let mut meta = read_meta(&dir);
        meta.skip_frames.retain(|n| *n != nr);
        if skip {
            meta.skip_frames.push(nr);
        }
        meta.skip_frames.sort_unstable();
        write_meta(&dir, &meta);
        Ok(())
    }
}
