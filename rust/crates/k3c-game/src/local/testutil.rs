//! Hilfen der Tests (Go: `local_test.go`): Wegwerf-Ordner im Repo, feste Uhr, Session und die zwei Geräte. Geschrieben wird
//! nur im Repo (`.work/`, ignoriert), nie in `%TEMP%`.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use std::sync::{Arc, Mutex};

use k3c_room::room::{Options, Room, free_monarchs, start_stage};
use k3c_sim::island::{Island, IslandOptions, STANDARD_WORLD_ID};
use k3c_sim::state::{Player, World};

use super::config::{Config, Zeit};
use super::peer::{LocalPeer, PeerData};
use super::session::{CONN, DEVICE_ID, Session, Started};
use super::worlds::game_data;
use crate::view::input::{DeviceID, DeviceKind};

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
fn island(seed: &str) -> Island {
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
