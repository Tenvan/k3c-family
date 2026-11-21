//! Hilfen der Raum-Tests (Go: `memstore_test.go`, der `peer` und `fixture` aus `room_test.go`): Spielstand-Speicher im
//! Speicher, mitschreibender Peer, feste Uhr und eine kleine Insel. Kein `Mutex`, kein `Rc`: der Peer meldet über einen
//! Kanal, den der Test im `Inbox` ausliest.
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, OnceLock};

use k3c_data::{Data, Source, load_all};
use k3c_level::types::Layout;
use k3c_sim::island::{Island, IslandOptions, STANDARD_WORLD_ID};
use k3c_sim::state::{Player, World};

use super::env::Env;
use super::log::RecordingLog;
use super::manager::Manager;
pub use super::memstore::MemStore;
use super::types::{MonarchState, Options, Peer, Room, Seat};
use super::{free_monarchs, start_stage};
use crate::store::Stamp;

enum Ev {
    Entry(String),
    You(Vec<Seat>),
    Monarchs(Vec<MonarchState>),
    /// `level N` oder `state N` mit der Stufe N (Go `stagePeer`).
    Stage(String),
    /// Ereignisse eines Zustands: (Tick, Stufe, Liste).
    Events(i64, i64, Vec<serde_json::Value>),
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
    fn level(&mut self, stage: i64, depth: i64, _layout: &Layout) {
        self.send(Ev::Stage(format!("level {stage}")));
        self.send(Ev::Entry(format!("level {depth}")));
    }
    fn state(&mut self, tick: i64, stage: i64, state: &serde_json::Value) {
        self.send(Ev::Stage(format!("state {stage}")));
        let switched = state
            .get("events")
            .and_then(|e| e.as_array())
            .is_some_and(|a| a.iter().any(|e| e["type"] == "islandSwitch"));
        if switched {
            self.send(Ev::Stage(format!("islandSwitch {stage}")));
        }
        let events = state
            .get("events")
            .and_then(|e| e.as_array())
            .cloned()
            .unwrap_or_default();
        self.send(Ev::Events(tick, stage, events));
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
    stage_log: Vec<String>,
    events: Vec<(i64, i64, Vec<serde_json::Value>)>,
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
            stage_log: Vec::new(),
            events: Vec::new(),
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
                Ev::Stage(s) => self.stage_log.push(s),
                Ev::Events(t, s, e) => self.events.push((t, s, e)),
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

    /// Die Nachrichten `level N` und `state N` mit ihrer Stufe, in der Reihenfolge des Eintreffens (Go `stagePeer.msgs`).
    pub fn msgs(&mut self) -> Vec<String> {
        self.pump();
        self.stage_log.clone()
    }

    /// Die Ereignisse jedes Zustands: (Tick, Stufe, Ereignisse).
    pub fn events(&mut self) -> Vec<(i64, i64, Vec<serde_json::Value>)> {
        self.pump();
        self.events.clone()
    }

    /// Plätze des Geräts als Text (wie `you()`).
    pub fn seats_now(&mut self) -> String {
        self.you()
    }
}

/// Die feste Welt der Tests: Uhr, Speicher, Log (Go `fixture`).
pub struct Fixture {
    pub store: MemStore,
    pub log: RecordingLog,
    pub now: Stamp,
    pub data: Arc<Data>,
    /// Die Uhr für Dauern (`Env::clock`): steht still auf `now` beim Anlegen.
    clock: Box<dyn FnMut() -> Stamp>,
}

impl Fixture {
    pub fn new() -> Fixture {
        let now = Stamp::from_civil(2026, 10, 1, 12, 0, 0, 0);
        Fixture {
            store: MemStore::default(),
            log: RecordingLog::default(),
            now,
            data: test_data(),
            clock: Box::new(move || now),
        }
    }

    pub fn env(&mut self) -> Env<'_> {
        Env {
            now: self.now,
            clock: &mut *self.clock,
            data: &self.data,
            store: &mut self.store,
            log: &mut self.log,
            sessions: None,
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

/// Ein Datenordner mit `worlds/<id>.json` unter `.work/` (Go: `fstest.MapFS` mit den Welten); er verschwindet mit dem Wert.
pub struct WorldDir {
    pub src: Source,
    dir: std::path::PathBuf,
}

impl WorldDir {
    /// Schreibt die Dateien `(id, Inhalt)` in einen eigenen Ordner; `tag` hält die Ordner der Tests auseinander.
    pub fn new(tag: &str, files: &[(&str, &str)]) -> WorldDir {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work");
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, Ordering::SeqCst); // gleiche Marke in parallelen Tests: eigener Ordner je Aufruf
        let dir = root.join(format!("room-worlds-{tag}-{}-{n}", std::process::id()));
        std::fs::create_dir_all(dir.join("worlds")).expect("Ordner der Welten");
        for (id, text) in files {
            std::fs::write(dir.join("worlds").join(format!("{id}.json")), text).expect("Welt-Datei");
        }
        WorldDir {
            src: Source::Dir(dir.clone()),
            dir,
        }
    }
}

impl Drop for WorldDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir); // ein Rest in `.work/` stört nicht
    }
}

/// Ein Manager im Dev-Mode (Go `f.m.Dev = true`).
pub fn dev_manager() -> TestManager {
    let (mut m, _) = manager();
    m.dev = true;
    m
}

/// Legt einen Raum an (`create`, neuer Stand, Tiefe `depth`) und liefert den Code.
pub fn create_room(m: &mut TestManager, id: &str, conn: u64, name: &str, depth: i64, slots: &[i64]) -> (String, Inbox) {
    let (p, i) = peer(conn);
    let code = m
        .create(id, p, name, true, depth, slots, Options::default())
        .expect("create");
    (code, i)
}

/// Tritt dem Raum `code` bei.
pub fn join_room(m: &mut TestManager, id: &str, conn: u64, code: &str, slots: &[i64]) -> Inbox {
    let (p, i) = peer(conn);
    m.join(id, p, code, slots).expect("join");
    i
}

/// Der Raum `code` (Test-Zugriff).
pub fn rm<'a>(m: &'a mut TestManager, code: &str) -> &'a mut Room {
    m.rooms.get_mut(code).expect("Raum")
}

/// `n` Takte des Raums `code` (Go `ticks`).
pub fn ticks(m: &mut TestManager, code: &str, n: usize) {
    for _ in 0..n {
        assert_eq!(m.with_room(code, |r, env| r.tick(env)), Some(true));
    }
}

/// Stellt Monarch `i` an den Tiefen-Eingang seiner Stufe (Ausgang nach unten, Go `moveToExit`).
pub fn move_to_exit(r: &mut Room, i: i64) {
    let Some(stage) = r.isl.stage_of(i) else {
        panic!("Monarch {i} ohne Stufe")
    };
    let w = &mut r.isl.stages[stage];
    let Some(x) = w.level.entities.iter().find(|e| e.kind == "exit").map(|e| e.x) else {
        panic!("kein Ausgang für Monarch {i}")
    };
    if let Some(p) = w.players.iter_mut().find(|p| p.index == i) {
        p.x = x;
        return;
    }
    panic!("kein Spieler {i} in Stufe {stage}");
}

impl WorldDir {
    /// Der Ordner (für Spielstände im Test).
    pub fn path(&self) -> &std::path::Path {
        &self.dir
    }
}

/// Lässt die Uhr des Managers um `secs` Sekunden weiterlaufen und prüft die Fristen (Go `f.wait`).
pub fn wait(m: &mut Manager<MemStore, RecordingLog>, clock: &AtomicI64, secs: i64) {
    clock.fetch_add(secs, Ordering::SeqCst);
    m.sweep();
}
