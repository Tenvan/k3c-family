//! Tests aus `island_test.go` (B-176, AC-01 bis AC-03; N1/AC-01): Einzelwechsel zwischen Stufen, Beitritt in der Stufe des
//! Geräts, Speichern und Laden einer Insel über den Datei-Store, Status mit Stufen, ein Zustand je Stufe und Tick. Die
//! Datei-Tests schreiben unter `.work/` (`WorldDir`), nie nach `%TEMP%`.
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};

use k3c_sim::state::World;

use super::manager::Manager;
use super::testutil::{WorldDir, create_room, join_room, manager, move_to_exit, peer, rm, test_data, ticks};
use super::{EMPTY_FOR_SECS, Options, RecordingLog};
use crate::store::{Saves, Stamp};

fn has(msgs: &[String], m: &str) -> bool {
    msgs.iter().any(|x| x == m)
}

// AC-01: Zwei Geräte, zwei Spieler: Nur der Wechsler wechselt die Stufe, nur sein Gerät bekommt `level` (vor `state`).
#[test]
fn einzelwechsel_schickt_level_an_richtiges_geraet() {
    let (mut m, _) = manager();
    let (code, mut x) = create_room(&mut m, "xbox", 1, "zwei", 0, &[0]);
    let mut h = join_room(&mut m, "handy", 2, &code, &[0]);
    let r = rm(&mut m, &code);
    assert_eq!((r.isl.stage_of(0), r.isl.stage_of(1)), (Some(0), Some(0)), "Start");
    let before = h.msgs().len();
    move_to_exit(rm(&mut m, &code), 0);
    ticks(&mut m, &code, 70);
    let r = rm(&mut m, &code);
    assert_eq!(
        (r.isl.stage_of(0), r.isl.stage_of(1)),
        (Some(1), Some(0)),
        "nach Wechsel"
    );
    let xm = x.msgs();
    let i = xm.iter().position(|e| e == "level 1").expect("Xbox bekam level 1");
    assert!(xm[i + 1] == "state 1", "Level vor Zustand der Stufe 1 fehlt: {xm:?}");
    let hm = h.msgs();
    assert!(
        !has(&hm, "level 1") && hm.last().map(String::as_str) == Some("state 0") && hm.len() > before,
        "Handy bekam das Level der anderen Stufe: {:?}",
        &hm[before..]
    );
    // Das Gerät folgt dem Monarchen im kleinsten Slot.
    move_to_exit(rm(&mut m, &code), 1);
    ticks(&mut m, &code, 70);
    let hm = h.msgs();
    assert!(
        has(&hm, "level 1") && hm.last().map(String::as_str) == Some("state 1"),
        "Handy folgt nicht: {:?}",
        &hm[hm.len() - 3..]
    );
}

// Ein neuer Monarch tritt in der Stufe seines Geräts bei, ein Gerät ohne Spieler in der Startstufe.
#[test]
fn beitritt_in_stufe_des_geraets() {
    let (mut m, _) = manager();
    let (code, mut x) = create_room(&mut m, "xbox", 1, "beitritt", 1, &[0]);
    assert_eq!(rm(&mut m, &code).isl.stage_of(0), Some(1));
    assert!(has(&x.msgs(), "level 1"), "Startstufe 1: {:?}", x.msgs());
    move_to_exit(rm(&mut m, &code), 0);
    ticks(&mut m, &code, 70); // Monarch 0 steht jetzt in Stufe 2
    assert_eq!(rm(&mut m, &code).add_slot("xbox", 1, 1), Ok(()));
    let mut h = join_room(&mut m, "handy", 2, &code, &[0]);
    let r = rm(&mut m, &code);
    let stages = (r.isl.stage_of(1), r.isl.stage_of(2));
    assert!(
        stages == (Some(2), Some(1)) && has(&h.msgs(), "level 1"),
        "Stufen: Slot 1 -> {:?}, Handy -> {:?}, {:?}",
        stages.0,
        stages.1,
        h.msgs()
    );
}

/// Ein Manager mit Datei-Store unter `.work/`; die Uhr des Managers und des Stores zählt aus `clock`.
fn file_manager(dir: &WorldDir) -> (Manager<Saves, RecordingLog>, std::sync::Arc<AtomicI64>) {
    let secs = std::sync::Arc::new(AtomicI64::new(Stamp::from_civil(2026, 10, 1, 12, 0, 0, 0).secs));
    let (c1, c2) = (secs.clone(), secs.clone());
    let store = Saves::new(dir.path().join("saves"), move || {
        Stamp::new(c1.load(Ordering::SeqCst), 0)
    });
    let m = Manager::new(
        store,
        RecordingLog::default(),
        move || Stamp::new(c2.load(Ordering::SeqCst), 0),
        "codes",
        test_data(),
    );
    (m, secs)
}

fn json(bytes: &[u8]) -> serde_json::Value {
    serde_json::from_slice(bytes).expect("JSON")
}

// AC-02: Speichern und Laden einer Insel über den Store (Rundlauf).
#[test]
fn insel_rundlauf() {
    let dir = WorldDir::new("rundlauf", &[]);
    let (mut m, clock) = file_manager(&dir);
    let (p, _x) = peer(1);
    let code = m
        .create("xbox", p, "insel", true, 0, &[0, 1], Options::default())
        .expect("create");
    for pl in m
        .rooms
        .get_mut(&code)
        .expect("Raum")
        .isl
        .stages
        .iter_mut()
        .flat_map(|w| w.players.iter_mut())
    {
        if pl.index == 1 {
            pl.gold = 77;
        }
    }
    move_to_exit(m.rooms.get_mut(&code).expect("Raum"), 0);
    for _ in 0..70 {
        assert_eq!(m.with_room(&code, |r, env| r.tick(env)), Some(true));
    }
    m.with_room(&code, |r, env| r.leave(env, "xbox", 1));
    let got = json(&m.store.load("insel").expect("Spielstand"));
    assert_eq!(got["version"], k3c_sim::island::ISLAND_SAVE_VERSION);
    assert_eq!(
        (
            got["stages"].as_array().map(Vec::len),
            got["players"].as_array().map(Vec::len)
        ),
        (Some(5), Some(2)),
        "gespeichert"
    );
    clock.fetch_add(EMPTY_FOR_SECS + 60, Ordering::SeqCst); // leerer Raum wird aufgeräumt
    m.sweep();
    assert!(m.rooms().is_empty());
    let (p, mut y) = peer(2);
    let code = m
        .create("xbox", p, "insel", false, 0, &[0], Options::default())
        .expect("laden");
    let r = m.rooms.get(&code).expect("Raum");
    let ps = r.isl.players();
    assert!(
        ps.len() == 2 && ps[1].gold == 77 && r.isl.stage_of(0) == Some(1) && r.isl.stage_of(1) == Some(0),
        "geladen: {} Spieler, Stufen {:?}/{:?}",
        ps.len(),
        r.isl.stage_of(0),
        r.isl.stage_of(1)
    );
    assert_eq!(y.monarchs(), "[taken free]", "Plätze");
}

// AC-02: Ein Stand Version 1 wird geladen; die Datei bleibt als Sicherung, der neue Stand ist Version 2 (heute 9).
#[test]
fn version1_wird_ueberfuehrt_und_gesichert() {
    let dir = WorldDir::new("version1", &[]);
    let (mut m, _) = file_manager(&dir);
    let mut c = k3c_sim::campaign::Campaign::create("alt", "alt-id", 1.0).expect("Kampagne");
    let pi = c.join_player().expect("Spieler");
    c.current_world_mut().expect("Stufe").players[pi].gold = 12;
    let v1 = serde_json::to_vec(&c.to_save("2026-01-01T00:00:00.000Z").expect("Stand")).expect("JSON");
    m.store.store("alt", &v1).expect("speichern");
    let (p, _x) = peer(1);
    let code = m
        .create("xbox", p, "alt", false, 0, &[0], Options::default())
        .expect("create");
    assert_eq!(m.rooms[&code].isl.players()[0].gold, 12, "Gold aus Version 1 fehlt");
    m.with_room(&code, |r, env| r.leave(env, "xbox", 1)); // speichert als neue Version
    let got = json(&m.store.load("alt").expect("Spielstand"));
    assert_eq!(
        got["version"],
        k3c_sim::island::ISLAND_SAVE_VERSION,
        "Version nach Speichern"
    );
    assert_eq!(m.rooms[&code].isl.stages.len(), 3, "Stufen (sonst kein Weiterreisen)");
    // Die Datei Version 1 liegt dauerhaft neben dem Slot (nicht in der rotierenden Sicherung).
    let list: Vec<std::path::PathBuf> = std::fs::read_dir(dir.path().join("saves"))
        .expect("Ordner")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("alt-") && n.ends_with(".json"))
        })
        .collect();
    assert_eq!(list.len(), 1, "dauerhafte Sicherung: {list:?}");
    assert_eq!(
        std::fs::read(&list[0]).expect("Sicherung"),
        v1,
        "Sicherung ist nicht der Stand Version 1"
    );
}

// AC-03: Der Status nennt die Stufen mit Tiefe, Phase, Tag und Spielern.
#[test]
fn status_zeigt_stufen() {
    let (mut m, _) = manager();
    let (code, _x) = create_room(&mut m, "xbox", 1, "status", 0, &[0]);
    let _h = join_room(&mut m, "handy", 2, &code, &[0]);
    move_to_exit(rm(&mut m, &code), 0);
    ticks(&mut m, &code, 70);
    let (st, _) = m.status();
    assert_eq!(st.len(), 1);
    let line: Vec<String> = st[0]
        .stages
        .iter()
        .map(|s| format!("{} {} {} {:?}", s.depth, s.phase, s.day, s.players))
        .collect();
    assert_eq!(
        line,
        ["0 day 1 [1]", "1 day 1 [0]", "2 day 1 []", "3 day 1 []", "4 day 1 []"],
        "Status.Stages"
    );
    assert_eq!(rm(&mut m, &code).summary().stages, st[0].stages, "Summary.Stages");
}

static BUILT: AtomicUsize = AtomicUsize::new(0);

fn counting_snapshot(w: &World, _scale: i64, _paused: bool) -> serde_json::Value {
    BUILT.fetch_add(1, Ordering::SeqCst);
    serde_json::to_value(w).unwrap_or_default()
}

// N1/AC-01 (B-276/AC-01): Zwei Geräte auf derselben Stufe teilen einen Zustand je Tick; auf zwei Stufen sind es zwei.
#[test]
fn ein_zustand_je_stufe_und_tick() {
    let (mut m, _) = manager();
    m.snapshot = Some(counting_snapshot);
    let (code, _x) = create_room(&mut m, "xbox", 1, "geteilt", 0, &[0]);
    let _h = join_room(&mut m, "handy", 2, &code, &[0]);
    BUILT.store(0, Ordering::SeqCst);
    ticks(&mut m, &code, 1);
    assert_eq!(
        BUILT.load(Ordering::SeqCst),
        1,
        "eine Stufe, 2 Geräte: ein Zustand je Tick"
    );
    move_to_exit(rm(&mut m, &code), 0);
    ticks(&mut m, &code, 70);
    let r = rm(&mut m, &code);
    assert_ne!(r.isl.stage_of(0), r.isl.stage_of(1), "Stufenwechsel nicht erreicht");
    BUILT.store(0, Ordering::SeqCst);
    ticks(&mut m, &code, 1);
    assert_eq!(BUILT.load(Ordering::SeqCst), 2, "zwei Stufen: zwei Zustände je Tick");
}
