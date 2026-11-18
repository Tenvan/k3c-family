//! Hilfen der Tests (Go: `local_test.go`): Wegwerf-Ordner im Repo, feste Uhr, Session und die zwei Geräte. Geschrieben wird
//! nur im Repo (`.work/`, ignoriert), nie in `%TEMP%`.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use k3c_level::types::Layout;
use k3c_room::room::{MonarchState, NullLog, Options, Peer, Room, Seat, free_monarchs, start_stage};
use k3c_room::store::Saves;
use k3c_sim::island::{Island, IslandOptions, STANDARD_WORLD_ID};
use k3c_sim::state::{Player, World};

use super::config::{Config, Zeit};
use super::peer::{LocalPeer, PeerData};
use super::session::{CONN, DEVICE_ID, Session, Started};
use super::worlds::game_data;
use crate::view::input::{Command, DeviceID, DeviceKind};

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Ein leerer Ordner unter `<Repo>/.work/local-tests/`, der beim Ende des Tests gelöscht wird (Go `t.TempDir`).
pub struct TempDir(PathBuf);

impl TempDir {
    pub fn new() -> TempDir {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work/local-tests");
        let dir = root.join(format!("{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("Testordner anlegen");
        TempDir(dir)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    /// Der Pfad als Text (für `Config::dir`).
    pub fn dir(&self) -> String {
        self.0.to_string_lossy().into_owned()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Die Uhr der Tests (Insel-ID und Zeitstempel der Spielstände): 11.11.2026 12:00:00 UTC.
pub fn fixed_now() -> Zeit {
    Zeit::from_civil(2026, 11, 11, 12, 0, 0, 0)
}

/// Eine Session mit fester Uhr im Ordner `dir`.
pub fn test_session(dir: &TempDir) -> Session {
    Session::open(Config {
        dir: dir.dir(),
        now: Some(Box::new(fixed_now)),
        ..Config::default()
    })
}

/// Tastatur und erstes Gamepad.
pub const TWO_DEVICES: [DeviceID; 2] = [
    DeviceID::new(DeviceKind::Keyboard, 0),
    DeviceID::new(DeviceKind::Gamepad, 0),
];

/// Eine kleine Insel von Hand mit zwei leeren Stufen (Tiefen 0 und 1) und zwei Spielern in Stufe 0 (wie ein geladener
/// Stand: im Raum freie Monarchen). Die Standard-Welt braucht `decode_world_story` (RU19) und `create_island` braucht
/// RU18; solange das fehlt, baut der Test die Insel selbst (wie `k3c-room`, `testutil::island`).
pub fn island(seed: &str) -> Island {
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
    // echte Biome, denn `Island::to_save` erzeugt die Vergleichswelt jeder Stufe aus ihrem Biom
    let data = game_data().expect("Spieldaten");
    for depth in [0, 1] {
        let biome = data.biomes.iter().find(|b| b.depth == depth).cloned().expect("Biom");
        let w = World {
            biome,
            ..World::default()
        };
        isl.stages.push(w);
    }
    for index in 0..2 {
        isl.stages[0].players.push(Player {
            index,
            ..Player::default()
        });
    }
    isl.next_player = 2;
    isl
}

/// Startet ein Spiel `name` mit den Geräten `devices` wie `Session::start`, aber über eine Insel von Hand statt
/// `Manager::create` (das braucht die Standard-Welt, RU19). Raum, Beitritt und alles danach laufen echt.
pub fn start_test_game(s: &mut Session, name: &str, devices: &[DeviceID]) {
    s.close_game().expect("altes Spiel beenden");
    let mut m = s.manager().expect("Manager");
    let isl = island(name);
    let monarchs = free_monarchs(&isl).expect("Monarchen");
    let start = start_stage(&isl, 0);
    let code = "TEST".to_string();
    m.rooms.insert(
        code.clone(),
        Room::new(&code, name, isl, Options::default(), start, monarchs),
    );
    let shared = Arc::new(Mutex::new(PeerData::default()));
    let slots: Vec<i64> = (0..devices.len() as i64).collect();
    m.join(
        DEVICE_ID,
        Box::new(LocalPeer::new(CONN, Arc::clone(&shared))),
        &code,
        &slots,
    )
    .expect("Beitritt");
    s.adopt(Started { m, code, peer: shared }, name, true, String::new(), devices);
}

/// Die festen Befehle des Determinismus-Tests: beide halten rechts, mit Sprint, Bezahlen und Angriff im Wechsel, Spieler 2
/// dreht alle 100 Takte um (Go: `commandAt`).
pub fn command_at(player: i64, i: usize) -> Command {
    if player == 0 {
        return Command {
            move_: 1.0,
            sprint: i % 90 < 30,
            pay: i.is_multiple_of(50),
            ..Command::default()
        };
    }
    Command {
        move_: if (i / 100) % 2 == 1 { -1.0 } else { 1.0 },
        attack: i.is_multiple_of(70),
        ..Command::default()
    }
}

/// Der Welt-Hash (Go: `hashWorlds`) als Text: das JSON der Welten nach Stufen-Index, durch Zeilenumbruch getrennt; die Tests
/// vergleichen es byte-gleich (ohne Hash-Abhängigkeit).
pub fn worlds_json<'a>(worlds: impl IntoIterator<Item = (i64, &'a World)>) -> String {
    let mut all: Vec<(i64, String)> = worlds
        .into_iter()
        .map(|(i, w)| (i, serde_json::to_value(w).map(|v| v.to_string()).unwrap_or_default()))
        .collect();
    all.sort();
    all.into_iter().map(|(_, j)| j).collect::<Vec<_>>().join(
        "
",
    )
}

/// Ein Gerät direkt am Raum, ohne `local` (Go: `refPeer`); es merkt sich den Zustand jeder Stufe als Welt.
pub struct RefPeer(pub Rc<RefCell<BTreeMap<i64, World>>>);

impl Peer for RefPeer {
    fn conn_id(&self) -> u64 {
        CONN
    }
    fn joined(&mut self, _: &str, _: &str, _: &str, _: &[Seat]) {}
    fn level(&mut self, _: i64, _: i64, _: &Layout) {}
    fn state(&mut self, _: i64, stage: i64, state: &serde_json::Value) {
        if let Ok(w) = serde_json::from_value::<World>(state.clone()) {
            self.0.borrow_mut().insert(stage, w);
        }
    }
    fn seats(&mut self, _: &[Seat], _: &[MonarchState]) {}
    fn replaced(&mut self) {}
    fn closed(&mut self, _: bool) {}
}

/// Taktet einen Raum direkt über `k3c_room` mit denselben Befehlen und liefert seine Welten nach `ticks` Takten
/// (Go: `referenceHash`); die Insel ist von Hand gebaut.
pub fn reference_worlds(dir: &TempDir, name: &str, ticks: usize) -> BTreeMap<i64, World> {
    let mut m = k3c_room::room::Manager::new(
        Saves::new(dir.path().to_path_buf(), fixed_now),
        NullLog,
        fixed_now,
        "ref",
        game_data().expect("Spieldaten"),
    );
    let isl = island(name);
    let monarchs = free_monarchs(&isl).expect("Monarchen");
    let start = start_stage(&isl, 0);
    m.rooms.insert(
        "REFR".to_string(),
        Room::new("REFR", name, isl, Options::default(), start, monarchs),
    );
    let states = Rc::new(RefCell::new(BTreeMap::new()));
    m.join("ref", Box::new(RefPeer(Rc::clone(&states))), "REFR", &[0, 1])
        .expect("Beitritt");
    for i in 0..ticks {
        let input: BTreeMap<i64, _> = (0..2).map(|p| (p, command_at(p, i).sim())).collect();
        let res = m.with_room("REFR", |r, _| r.input("ref", CONN, &input));
        assert!(matches!(res, Some(Ok(()))), "Eingabe: {res:?}");
        assert!(m.safe_tick("REFR"), "Raum läuft");
    }
    states.take()
}

/// Taktet `n` Mal mit Bewegung nach rechts und liefert den letzten Frame (Go: `stepN`).
pub fn step_n(s: &mut Session, n: usize) -> crate::view::frame::Frame {
    use crate::game::sim::Sim as _;
    let mut f = crate::view::frame::Frame::default();
    for _ in 0..n {
        let c = Command {
            move_: 1.0,
            ..Command::default()
        };
        s.command(0, c).expect("Befehl");
        f = s.step();
    }
    f
}

/// Enthält der Frame einen Hinweis der Art `kind` (Go: `hasCue`)?
pub fn has_cue(f: &crate::view::frame::Frame, kind: crate::view::frame::CueKind) -> bool {
    f.cues.iter().any(|c| c.kind == kind)
}

/// Eine Session in einem Ordner, der nicht angelegt werden kann (unter einer Datei).
pub fn unwritable_session(dir: &TempDir) -> Session {
    let file = dir.path().join("datei");
    std::fs::write(&file, b"").expect("Datei");
    Session::open(Config {
        dir: file.join("saves").to_string_lossy().into_owned(),
        now: Some(Box::new(fixed_now)),
        ..Config::default()
    })
}
