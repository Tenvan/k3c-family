//! Nachrichten zwischen Spiel und Cheat-Fenster. Unbekannte Felder werden ignoriert (Vorwärtskompatibilität).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// PROTO_VERSION ist die Hauptversion des Protokolls (`v` im `hello`).
pub const PROTO_VERSION: u32 = 1;

/// Msg ist eine Nachricht; `t` benennt die Art. Eine unbekannte Art ergibt `Unknown`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "lowercase")]
pub enum Msg {
    /// Beide Seiten beginnen damit; das Spiel nennt zusätzlich `game`.
    Hello {
        v: u32,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        game: Option<GameInfo>,
    },
    /// Fenster → Spiel: Dev-Aktion für einen Spieler.
    Dev { id: u64, player: i64, action: DevCommand },
    /// Fenster → Spiel: Bedienung einer Spieltaste (`debug`, `stats`, `river`, `help`, `pause`, `step`, `speed`,
    /// `save`, `load`, `shot`, `fullscreen`).
    Ui {
        id: u64,
        key: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<i64>,
    },
    /// Fenster → Spiel: Welt, Insel und Stufe wechseln.
    Goto {
        id: u64,
        world: String,
        island: i64,
        stage: i64,
    },
    /// Fenster → Spiel: das Spielfenster nach vorn holen.
    Focus { id: u64 },
    /// Spiel → Fenster: Auswahllisten, einmal nach `hello`.
    Catalog(Catalog),
    /// Spiel → Fenster: aktueller Zustand.
    State(State),
    /// Spiel → Fenster: Antwort auf einen Befehl.
    Ack {
        id: u64,
        ok: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        err: Option<String>,
    },
    /// Eine Seite beendet die Verbindung.
    Bye,
    /// Unbekannte Art `t` (nie ein Fehler).
    #[serde(other)]
    Unknown,
}

/// GameInfo nennt Name und Version des Spiels im `hello`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameInfo {
    pub name: String,
    pub version: String,
}

/// DevCommand spiegelt `DevAction` des Spiels (die Umwandlung steht im Spiel).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DevCommand {
    pub action: String,
    pub amount: i64,
    pub resource: String,
    pub factor: i64,
    pub phase: String,
    pub grade: String,
    pub on: Option<bool>,
    pub level: i64,
    pub boss: String,
    pub event: String,
    pub kind: String,
    pub unit: String,
}

/// Entry ist ein Auswahleintrag (Kennung und Anzeigename).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Entry {
    pub id: String,
    pub label: String,
}

/// IslandInfo beschreibt eine Insel mit der Zahl ihrer Stufen.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IslandInfo {
    pub name: String,
    pub stages: i64,
}

/// WorldInfo beschreibt eine Welt mit ihren Inseln.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WorldInfo {
    pub id: String,
    pub name: String,
    pub islands: Vec<IslandInfo>,
}

/// Catalog sind die Auswahllisten für die Controls des Fensters.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Catalog {
    pub worlds: Vec<WorldInfo>,
    pub bosses: Vec<Entry>,
    pub events: Vec<Entry>,
    pub skills: Vec<Entry>,
    pub resources: Vec<Entry>,
    pub units: Vec<Entry>,
    pub phases: Vec<String>,
    pub grades: Vec<String>,
    pub timescales: Vec<i64>,
}

/// PlayerState ist der Stand eines Spielers.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerState {
    pub gold: i64,
    pub stock: BTreeMap<String, i64>,
}

/// State ist der Zustand des Spiels für die Anzeige im Fenster.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct State {
    pub world: String,
    pub island: i64,
    pub stage: i64,
    pub phase: String,
    pub timescale: i64,
    pub halted: bool,
    pub players: Vec<PlayerState>,
    pub debug_page: i64,
    pub perf: bool,
    pub river: String,
    pub ticks_per_s: f64,
}
