//! Typen des Raummodells aus `docs/protocol.md` (Protokoll v2): Grenzen, Plätze, Optionen, `Peer`, Monarch, Gerät, Raum.
//! Port von `engine/room/room.go` (Typen). Kein `Mutex`, kein Rückverweis auf den Manager: Uhr, Log, Store und
//! Snapshot kommen bei jedem Aufruf als `&mut Env` (`env.rs`).
use std::collections::{BTreeMap, BTreeSet};

use k3c_level::types::Layout;
use k3c_sim::island::Island;
use k3c_sim::state::PlayerCommand;
use serde::Serialize;

use super::pending::Metrics;
use crate::store::Stamp;

/// Grenzen und Fristen aus `docs/protocol.md`.
pub const MAX_MONARCHS: usize = 4;
pub const MAX_SLOTS: i64 = 4;
pub const MAX_ROOMS: usize = 4;
pub const TICK_HZ: i64 = 30;
/// Frist, nach der ein wartender Monarch frei wird (Go `WaitFor`, 60 s).
pub const WAIT_FOR_SECS: i64 = 60;
/// Frist, nach der ein leerer Raum aufgeräumt wird (Go `EmptyFor`, 10 min).
pub const EMPTY_FOR_SECS: i64 = 10 * 60;
/// Takt des Speicherns (B-186, Q10), gezählt in Raum-Ticks: ein Tick ist 1/`TICK_HZ` s echter Zeit (Go `SaveEvery`, 60 s).
pub const SAVE_EVERY_SECS: i64 = 60;
/// `SAVE_EVERY_SECS` in Raum-Ticks (Go `saveTicks`).
pub const SAVE_TICKS: i64 = SAVE_EVERY_SECS * TICK_HZ;
/// Reserviert Spielstandnamen für Testläufe (Testseite, B-086): Räume mit diesem Präfix löschen ihren Spielstand, wenn sie
/// aufgeräumt werden.
pub const TEST_PREFIX: &str = "test-";
/// Tick-Dauern, die der Raum für p99 behält (10 s).
pub const DURATIONS: usize = 300;
/// Höchstlänge von `create.seed` (wie beim Level-Generator).
pub const MAX_SEED_RUNES: usize = 64;

/// Zustand eines Monarchen (Go: Konstanten `taken`, `waiting`, `free`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum MonarchState {
    #[serde(rename = "taken")]
    Taken,
    #[serde(rename = "waiting")]
    Waiting,
    #[default]
    #[serde(rename = "free")]
    Free,
}

impl MonarchState {
    pub fn as_str(self) -> &'static str {
        match self {
            MonarchState::Taken => "taken",
            MonarchState::Waiting => "waiting",
            MonarchState::Free => "free",
        }
    }
}

/// Ordnet einem Slot des Geräts einen Monarchen zu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Seat {
    pub slot: i64,
    pub monarch: i64,
    /// Tiefe der Stufe, in der der Monarch steht.
    pub depth: i64,
    /// Index der Stufe in `Island::stages`: gehört zum Strom mit diesem `stage` (B-176).
    pub stage: i64,
}

/// Raum-Optionen aus `create` (leer = Standard); `seed` ersetzt bei einem neuen Stand den Namen als Seed (B-095). `world`
/// ist die Welt-ID eines neuen Stands (leer = standard, eine andere nur im Dev-Mode, `docs/rules/welten.md` § 6).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Options {
    pub grade: String,
    pub goal: String,
    pub defeat: String,
    pub seed: String,
    pub world: String,
}

/// Wie Go `%+v` einer `Options`-Struktur im Log.
impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{Grade:{} Goal:{} Defeat:{} Seed:{} World:{}}}",
            self.grade, self.goal, self.defeat, self.seed, self.world
        )
    }
}

/// Die Verbindung eines Geräts. Der Raum ruft die Methoden auf, sie dürfen nicht blockieren und nicht in den Raum oder
/// Manager zurückrufen (Borrow-Regeln erzwingen das). `level` und `state` tragen die Stufe (Index in `Island::stages`),
/// je Stufe ein eigener Strom (B-176). Geräte derselben Stufe teilen den Zustand, er ist unveränderlich.
pub trait Peer {
    /// Gos Vergleich `d.peer == peer` (Identität des Interface-Werts): gleiche Verbindung, gleicher Schlüssel.
    fn conn_id(&self) -> u64;
    /// `world`: Welt-ID des Raums.
    fn joined(&mut self, room: &str, name: &str, world: &str, you: &[Seat]);
    fn level(&mut self, stage: i64, depth: i64, layout: &Layout);
    fn state(&mut self, tick: i64, stage: i64, state: &serde_json::Value);
    fn seats(&mut self, you: &[Seat], monarchs: &[MonarchState]);
    /// Dieselbe Geräte-ID ist über eine neue Verbindung beigetreten.
    fn replaced(&mut self);
    /// Raum geschlossen; `final_`: Server fährt herunter, die Verbindung endet danach.
    fn closed(&mut self, final_: bool);
}

/// Ein Monarch des Raums (Go `monarch`); Monarch `i` ist der Insel-Spieler mit Index `i`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Monarch {
    pub state: MonarchState,
    /// Besetzendes bzw. zuletzt besetzendes Gerät.
    pub device: String,
    pub slot: i64,
    /// Beginn von `Waiting`.
    pub since: Option<Stamp>,
    pub input: PlayerCommand,
}

/// Ein Gerät im Raum (Go `device`).
pub struct Device {
    pub peer: Box<dyn Peer>,
    /// Slot -> Monarch.
    pub slots: BTreeMap<i64, usize>,
    pub connected: bool,
    /// Stufen, deren Level das Gerät bekam und deren Strom läuft.
    pub stages: BTreeSet<usize>,
    /// Pause des Geräts (`pause`, B-214, RU24).
    pub paused: bool,
    /// Restliche Ticks des Schutzes online (`pause`, RU24).
    pub protect: i64,
    /// Beobachter ohne Platz (`observer`, B-352, RU24).
    pub observer: bool,
}

impl Device {
    pub fn new(peer: Box<dyn Peer>, slots: BTreeMap<i64, usize>) -> Device {
        Device {
            peer,
            slots,
            connected: true,
            stages: BTreeSet::new(),
            paused: false,
            protect: 0,
            observer: false,
        }
    }
}

/// Ein laufendes Spiel: eine Insel, ein Spielstand, ein Code. Monarch `i` ist der Insel-Spieler mit Index `i`. Felder
/// späterer Sessions (Tick-Dauern, Speicher-Schreiber, Zeitraffer-Teile …) kommen mit RU24.
pub struct Room {
    pub code: String,
    pub name: String,
    pub isl: Island,
    /// Im letzten Tick gewonnen (`Island::won` wurde wahr, Victory).
    pub won_now: bool,
    /// Angeforderte Raum-Optionen (gelten nur für neue Stände).
    pub opts: Options,
    /// Startstufe: dort treten neue Geräte ohne Spieler ein.
    pub start: usize,
    pub monarchs: Vec<Monarch>,
    pub devices: BTreeMap<String, Device>,
    pub tick: i64,
    pub empty_since: Option<Stamp>,
    pub closed: bool,
    /// Zeitraffer (dev timescale): Schritte je Tick, 0 = 1.
    pub timescale: i64,
    /// Dev: Zeitraffer endet beim nächsten Tageszeitwechsel.
    pub auto_reset: bool,
    /// Tageszeit beim Setzen von `auto_reset`.
    pub reset_phase: String,
    /// Dev-Pause (dev pause, B-231): Ticks rechnen keine Schritte.
    pub paused: bool,
    /// Dauern der letzten Ticks in Nanosekunden (`run`), höchstens `DURATIONS`.
    pub durations: Vec<i64>,
    /// Test-Naht: läuft im Tick vor dem Schritt der Insel.
    pub before_step: Option<Box<dyn FnMut()>>,
    /// Spielmetrik des Raumlaufs (`metrics`, RU24); `None` = kein Sammler.
    pub met: Option<Metrics>,
    /// Letzte Meldung eines langsamen Ticks (`logging.rs`); `None`: noch keine.
    pub slow_logged: Option<Stamp>,
    /// Langsame Ticks seit dieser Meldung.
    pub slow_count: i64,
    /// Ticks seit der letzten Speicherung (`early.rs`, Go `saves.due`).
    pub save_due: i64,
}
