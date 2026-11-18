//! Hilfen der Speicher-Tests (`saver_tests`, `autosave_tests`, `save_tests`, `gameover_save_tests`, `fullsave_tests`):
//! Anlegen mit Optionen, Verlassen und Trennen über den Manager, Spielstand lesen, Ereignisse des letzten Zustands.
use k3c_sim::island::IslandSave;
use serde_json::Value;

use super::testutil::{Inbox, TestManager, peer, rm};
use super::types::Options;

/// Legt einen Raum an (neu oder aus dem Stand `name`) und liefert Code und `Inbox` (Go `f.m.Create`).
pub fn create_with(
    m: &mut TestManager,
    id: &str,
    conn: u64,
    name: &str,
    fresh: bool,
    opts: Options,
) -> (String, Inbox) {
    let (p, i) = peer(conn);
    let code = m.create(id, p, name, fresh, 0, &[0], opts).expect("create");
    (code, i)
}

/// Das Gerät `id` verlässt den Raum (Go `r.Leave`).
pub fn leave(m: &mut TestManager, code: &str, id: &str, conn: u64) {
    m.with_room(code, |r, env| r.leave(env, id, conn)).expect("Raum");
}

/// Die Verbindung des Geräts `id` bricht ab (Go `r.Drop`).
pub fn drop_device(m: &mut TestManager, code: &str, id: &str, conn: u64) {
    m.with_room(code, |r, env| r.drop_conn(env, id, conn)).expect("Raum");
}

/// Der gespeicherte Stand `name` (aktuelle Version, Go `sim.ParseIslandSave`).
pub fn saved(m: &TestManager, name: &str) -> IslandSave {
    serde_json::from_slice(m.store.data.get(name).expect("Spielstand")).expect("Spielstand lesbar")
}

/// Zählt die Ereignisse vom Typ `typ` im letzten Zustand und liefert das letzte davon (Go `eventPeer.count`).
pub fn count(i: &mut Inbox, typ: &str) -> (usize, Option<Value>) {
    let events = i.events();
    let Some((_, _, last)) = events.last() else {
        return (0, None);
    };
    let found: Vec<&Value> = last.iter().filter(|e| e["type"] == typ).collect();
    (found.len(), found.last().map(|e| (*e).clone()))
}

/// Ist die Zeile mit `text` im Log des Managers?
pub fn logged(m: &TestManager, text: &str) -> bool {
    m.log.lines.iter().any(|l| l.contains(text))
}

/// Beschleunigt den Zyklus aller Stufen (Go `w.CycleSpeed = 1000`).
pub fn fast_cycle(m: &mut TestManager, code: &str) {
    for w in &mut rm(m, code).isl.stages {
        w.cycle_speed = 1000.0;
    }
}
