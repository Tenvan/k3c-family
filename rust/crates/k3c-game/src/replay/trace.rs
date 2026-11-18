//! Leser der Aufzeichnung `.jsonl.gz`: Kopfzeile mit dem Level, danach je Takt Zustand und Ereignisse.
//!
//! Der Zustand lädt in `k3c_sim::state::World`, das Level in `k3c_level::types::Layout`. Takt 0 ist der Startzustand,
//! `ereignisse` kopiert `zustand.events` (RU38.1).

use std::io::Read;
use std::path::Path;

use k3c_level::types::Layout;
use k3c_sim::state::{Event, World};
use serde::Deserialize;

/// Erste Zeile: Szene, Seed (Text), Spielerzahl und Level.
#[derive(Debug, Clone, Deserialize)]
pub struct Head {
    pub szene: String,
    pub seed: String,
    pub spieler: u32,
    pub level: Layout,
}

/// Zeile eines Takts.
#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    pub takt: u64,
    pub zustand: World,
    /// Go schreibt eine leere Liste als `null`.
    #[serde(default)]
    pub ereignisse: Option<Vec<Event>>,
}

/// Eine geladene Aufzeichnung.
#[derive(Debug, Clone)]
pub struct Trace {
    pub head: Head,
    pub rows: Vec<Row>,
}

/// Fehler beim Lesen: Datei, Entpacken oder Zeile.
#[derive(Debug)]
pub enum TraceError {
    Io(std::io::Error),
    Line(usize, serde_json::Error),
    Empty,
}

impl std::fmt::Display for TraceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "Aufzeichnung lesen: {e}"),
            Self::Line(n, e) => write!(f, "Aufzeichnung Zeile {n}: {e}"),
            Self::Empty => write!(f, "Aufzeichnung ist leer"),
        }
    }
}

impl std::error::Error for TraceError {}

/// Entpackt `raw` (gzip) in die Zeilen ohne Zeilenende.
pub fn unpack(raw: &[u8]) -> Result<Vec<String>, TraceError> {
    let mut text = String::new();
    flate2::read::GzDecoder::new(raw)
        .read_to_string(&mut text)
        .map_err(TraceError::Io)?;
    Ok(text.lines().map(str::to_owned).collect())
}

/// Liest eine `.jsonl.gz` von der Platte und liefert die Zeilen.
pub fn read_lines(path: impl AsRef<Path>) -> Result<Vec<String>, TraceError> {
    unpack(&std::fs::read(path).map_err(TraceError::Io)?)
}

/// Lädt die Zeilen in Kopf und Takte.
pub fn parse(lines: &[String]) -> Result<Trace, TraceError> {
    let (first, rest) = lines.split_first().ok_or(TraceError::Empty)?;
    let head = serde_json::from_str(first).map_err(|e| TraceError::Line(1, e))?;
    let mut rows = Vec::with_capacity(rest.len());
    for (i, l) in rest.iter().enumerate() {
        rows.push(serde_json::from_str(l).map_err(|e| TraceError::Line(i + 2, e))?);
    }
    Ok(Trace { head, rows })
}

/// Lädt eine Aufzeichnung Zeile für Zeile aus der `.jsonl.gz` (RU39.1): ohne den ganzen Text im Speicher, damit die
/// Messung des Arbeitsspeichers nur die geladenen Zustände sieht.
pub fn load_stream(path: impl AsRef<Path>) -> Result<Trace, TraceError> {
    use std::io::BufRead;
    let f = std::fs::File::open(path).map_err(TraceError::Io)?;
    let mut lines = std::io::BufReader::new(flate2::read::GzDecoder::new(std::io::BufReader::new(f))).lines();
    let first = lines.next().ok_or(TraceError::Empty)?.map_err(TraceError::Io)?;
    let head = serde_json::from_str(&first).map_err(|e| TraceError::Line(1, e))?;
    let mut rows = Vec::new();
    for (i, l) in lines.enumerate() {
        let l = l.map_err(TraceError::Io)?;
        rows.push(serde_json::from_str(&l).map_err(|e| TraceError::Line(i + 2, e))?);
    }
    Ok(Trace { head, rows })
}

/// Liest und lädt eine Aufzeichnung von der Platte.
pub fn load(path: impl AsRef<Path>) -> Result<Trace, TraceError> {
    parse(&read_lines(path)?)
}
