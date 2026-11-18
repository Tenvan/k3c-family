//! Hilfen der Raum-Tests (Go: `memstore_test.go`, der `peer` und `fixture` aus `room_test.go`): Spielstand-Speicher im
//! Speicher, mitschreibender Peer, feste Uhr und eine kleine Insel. Kein `Mutex`, kein `Rc`: der Peer meldet über einen
//! Kanal, den der Test im `Inbox` ausliest.
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, OnceLock};

use k3c_data::{Data, Source, load_all};
use k3c_level::types::Layout;
use k3c_sim::island::{Island, IslandOptions, STANDARD_WORLD_ID};
use k3c_sim::state::{Player, World};

use super::env::{Env, Store};
use super::log::RecordingLog;
use super::manager::Manager;
use super::types::{MonarchState, Options, Peer, Room, Seat};
use super::{free_monarchs, start_stage};
use crate::fehler::Fehler;
use crate::store::Stamp;

/// Spielstand-Speicher im Speicher (Go `memStore`).
#[derive(Debug, Default)]
pub struct MemStore {
    pub data: BTreeMap<String, Vec<u8>>,
    pub saves: usize,
    pub deleted: Vec<String>,
}

impl Store for MemStore {
    fn load(&mut self, name: &str) -> Result<Vec<u8>, Fehler> {
        self.data.get(name).cloned().ok_or(Fehler::NotFound)
    }

    fn store(&mut self, name: &str, data: &[u8]) -> Result<String, Fehler> {
        self.data.insert(name.to_string(), data.to_vec());
        self.saves += 1;
        Ok(String::new())
    }

    fn delete(&mut self, name: &str) -> Result<(), Fehler> {
        self.data.remove(name);
        self.deleted.push(name.to_string());
        Ok(())
    }
}

enum Ev {
    Entry(String),
    You(Vec<Seat>),
    Monarchs(Vec<MonarchState>),
}

/// Go `fmt.Sprint([]Seat)`: `[{0 2 0 0}]`.
pub fn seats_text(you: &[Seat]) -> String {
    let parts: Vec<String> = you
        .iter()
        .map(|s| format!("{{{} {} {} {}}}", s.slot, s.monarch, s.depth, s.stage))
        .collect();
    format!("[{}]", parts.join(" "))
}

/// Go `fmt.Sprint([]string)` der Monarchen-Zustände: `[taken free]`.
pub fn states_text(states: &[MonarchState]) -> String {
    let parts: Vec<&str> = states.iter().map(|s| s.as_str()).collect();
    format!("[{}]", parts.join(" "))
}

/// Der Peer, der mitschreibt, was der Raum schickt (Go `peer`).
pub struct FakePeer {
    id: u64,
    tx: Sender<Ev>,
}

impl FakePeer {
    fn send(&self, e: Ev) {
        let _ = self.tx.send(e); // der Test hält den Empfänger; ohne ihn ist das Mitschreiben egal
    }
}

impl Peer for FakePeer {
    fn conn_id(&self) -> u64 {
        self.id
    }
    fn joined(&mut self, room: &str, name: &str, _world: &str, you: &[Seat]) {
        self.send(Ev::You(you.to_vec()));
        self.send(Ev::Entry(format!("joined {room} {name} {}", seats_text(you))));
    }
    fn level(&mut self, _stage: i64, depth: i64, _layout: &Layout) {
        self.send(Ev::Entry(format!("level {depth}")));
    }
    fn state(&mut self, tick: i64, _stage: i64, _state: &serde_json::Value) {
        self.send(Ev::Entry(format!("state {tick}")));
    }
    fn seats(&mut self, you: &[Seat], monarchs: &[MonarchState]) {
        self.send(Ev::You(you.to_vec()));
        self.send(Ev::Monarchs(monarchs.to_vec()));
        self.send(Ev::Entry("seats".to_string()));
    }
    fn replaced(&mut self) {
        self.send(Ev::Entry("replaced".to_string()));
    }
    fn closed(&mut self, _final: bool) {
        self.send(Ev::Entry("closed".to_string()));
    }
}

/// Die Sicht des Tests auf einen `FakePeer`: Protokoll, zuletzt gemeldete Plätze und Monarchen-Zustände.
pub struct Inbox {
    rx: Receiver<Ev>,
    log: Vec<String>,
    you: Vec<Seat>,
    monarchs: Vec<MonarchState>,
}

/// Ein neuer Peer mit der Verbindungs-Nummer `id` und sein `Inbox`.
pub fn peer(id: u64) -> (Box<FakePeer>, Inbox) {
    let (tx, rx) = channel();
    (
        Box::new(FakePeer { id, tx }),
        Inbox {
            rx,
            log: Vec::new(),
            you: Vec::new(),
            monarchs: Vec::new(),
        },
    )
}

impl Inbox {
    fn pump(&mut self) {
        while let Ok(ev) = self.rx.try_recv() {
            match ev {
                Ev::Entry(e) => self.log.push(e),
                Ev::You(y) => self.you = y,
                Ev::Monarchs(m) => self.monarchs = m,
            }
        }
    }

    pub fn log(&mut self) -> &[String] {
        self.pump();
        &self.log
    }

    pub fn has(&mut self, entry: &str) -> bool {
        self.log().iter().any(|e| e == entry)
    }

    /// Plätze wie Go `fmt.Sprint(p.you)`.
    pub fn you(&mut self) -> String {
        self.pump();
        seats_text(&self.you)
    }

    /// Zustände wie Go `fmt.Sprint(p.monarchs)`.
    pub fn monarchs(&mut self) -> String {
        self.pump();
        states_text(&self.monarchs)
    }
}

/// Die feste Welt der Tests: Uhr, Speicher, Log (Go `fixture`).
pub struct Fixture {
    pub store: MemStore,
    pub log: RecordingLog,
    pub now: Stamp,
    pub data: Arc<Data>,
}

impl Fixture {
    pub fn new() -> Fixture {
        Fixture {
            store: MemStore::default(),
            log: RecordingLog::default(),
            now: Stamp::from_civil(2026, 10, 1, 12, 0, 0, 0),
            data: test_data(),
        }
    }

    pub fn env(&mut self) -> Env<'_> {
        Env {
            now: self.now,
            data: &self.data,
            store: &mut self.store,
            log: &mut self.log,
            dev: false,
            snapshot: None,
        }
    }
}

/// Eine kleine Insel von Hand mit zwei leeren Stufen (Tiefen 0 und 1) und `players` Spielern in Stufe 0 (Index 0 bis
/// `players - 1`, wie ein geladener Stand: im Raum freie Monarchen). `create_island` braucht `new_hub_site` und
/// `add_island_player` braucht `preset_for` (beide RU18), die Standard-Welt `decode_world_story` (RU19); auf diesem Stand
/// fehlt das, deshalb baut der Test die Insel selbst und der Raum legt keine Spieler neu an.
pub fn island(seed: &str, players: i64) -> Island {
    let mut isl = Island {
        id: seed.into(),
        seed: seed.into(),
        cycle_speed: 1.0,
        ..Island::default()
    };
    isl.world.id = STANDARD_WORLD_ID.into();
    isl.options = IslandOptions {
        grade: "normal".into(),
        goal: "endboss".into(),
        defeat: "resources".into(),
    };
    for depth in [0, 1] {
        let mut w = World::default();
        w.biome.depth = depth;
        isl.stages.push(w);
    }
    for index in 0..players {
        isl.stages[0].players.push(Player {
            index,
            ..Player::default()
        });
    }
    isl.next_player = players;
    isl
}

/// Ein Raum ohne Geräte mit `players` freien Monarchen (Go: `Manager.create` eines geladenen Stands).
pub fn room(code: &str, name: &str, players: i64) -> Room {
    let isl = island(name, players);
    let monarchs = free_monarchs(&isl).expect("keine Spieler");
    let start = start_stage(&isl, 0);
    Room::new(code, name, isl, Options::default(), start, monarchs)
}

/// Die eingebetteten Spiel-Daten, einmal geladen und geteilt.
pub fn test_data() -> Arc<Data> {
    static DATA: OnceLock<Arc<Data>> = OnceLock::new();
    DATA.get_or_init(|| Arc::new(load_all(&Source::Embedded).expect("eingebettete Daten")))
        .clone()
}

/// Der Manager der Tests.
pub type TestManager = Manager<MemStore, RecordingLog>;

/// Ein Manager mit Speicher im Speicher und Log zum Mitlesen; die Uhr zählt Sekunden aus dem zweiten Wert und steht auf
/// dem 1.10.2026 12:00:00 UTC (Go `fixture`, `f.wait`: den Wert erhöhen).
pub fn manager() -> (Manager<MemStore, RecordingLog>, Arc<AtomicI64>) {
    let secs = Arc::new(AtomicI64::new(Stamp::from_civil(2026, 10, 1, 12, 0, 0, 0).secs));
    let clock = secs.clone();
    let m = Manager::new(
        MemStore::default(),
        RecordingLog::default(),
        move || Stamp::new(clock.load(Ordering::SeqCst), 0),
        "codes",
        test_data(),
    );
    (m, secs)
}

/// Legt einen Raum mit `players` freien Monarchen unter `code` in den Manager (ohne `create_in`, RU24).
pub fn add_room(m: &mut Manager<MemStore, RecordingLog>, code: &str, name: &str, players: i64) {
    m.rooms.insert(code.to_string(), room(code, name, players));
}
