//! Test-Hilfen für Räume in einer Welt mit einer Insel (WL2): Die Standard-Welt hat mehrere Inseln, Tests der letzten
//! Insel nehmen die Welt `einzel`.

use super::Options;
use super::testutil::{Inbox, TestManager, WorldDir, dev_manager, peer};

/// Test-Welt `einzel` mit genau einer Insel (die Standard-Welt hat mehrere): für Tests der letzten Insel.
pub const EINZEL_RAW: &str = r#"{"id":"einzel","name":"Einzel","islands":[{"id":"island1","name":"Insel 1","depths":[0,4],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1}}]}"#;

/// Dev-Manager mit der Welt `einzel`; der Ordner muss so lange leben wie der Manager.
pub fn einzel_manager(tag: &str) -> (TestManager, WorldDir) {
    let dir = WorldDir::new(tag, &[("einzel", EINZEL_RAW)]);
    let mut m = dev_manager();
    m.worlds = Some(dir.src.clone());
    (m, dir)
}

/// Wie `create_room`, aber in der Welt `world` (die Standard-Welt hat seit WL fünf Inseln; Tests der letzten Insel
/// nehmen eine Welt mit einer Insel, z. B. `nil`).
pub fn create_room_in(
    m: &mut TestManager,
    world: &str,
    id: &str,
    conn: u64,
    name: &str,
    depth: i64,
    slots: &[i64],
) -> (String, Inbox) {
    let (p, i) = peer(conn);
    let opts = Options {
        world: world.into(),
        ..Options::default()
    };
    let code = m.create(id, p, name, true, depth, slots, opts).expect("create");
    (code, i)
}
