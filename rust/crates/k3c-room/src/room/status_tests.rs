//! Tests von `status.rs` (Go: `TestSummary`, `TestGeraeteKennungen`). `Summary` des neu angelegten Stands (Gold, Truppen)
//! braucht `create_in` (RU24); hier rechnet die Test-Insel.
use super::MonarchState;
use super::testutil::{Fixture, add_room, manager, peer, room};

#[test]
fn summary() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "kurz", 2);
    r.paused = true; // ohne Schritte der Insel (`step_players`, RU18)
    let (p, _pi) = peer(1);
    r.join(&mut f.env(), "a", p, &[0, 1]).expect("join");
    for _ in 0..3 {
        r.tick(&mut f.env());
    }
    let s = r.summary();
    assert_eq!(
        (s.code.as_str(), s.depth, s.tick, s.scale, s.paused),
        ("KRNZ", 0, 3, 1, true)
    );
    assert_eq!(s.gold.len(), 2);
    assert_eq!(s.enemies, 0);
    assert_eq!(s.devices.len(), 1);
    assert_eq!((s.devices[0].id.as_str(), s.devices[0].connected), ("a", true));
    assert_eq!(s.devices[0].slots, vec![0, 1]);
    assert_eq!(s.stages.len(), 2);
    assert_eq!(s.stages[0].players, vec![0, 1]);
    assert!(s.stages[1].players.is_empty());
    let json = serde_json::to_value(&s).expect("json");
    for key in [
        "code",
        "castleHp",
        "timescale",
        "devCheapBuild",
        "devStockBonus",
        "devPurseBonus",
        "devTimescaleAuto",
    ] {
        assert!(json.get(key).is_some(), "{key} fehlt: {json}");
    }
}

// B-088: Die Kennung ist der kürzeste gemeinsame Anfang (mindestens 6 Zeichen), der alle Geräte des Raums unterscheidet.
#[test]
fn geraete_kennungen() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "kennung", 2);
    let ((a, _ai), (b, _bi)) = (peer(1), peer(2));
    r.join(&mut f.env(), "b1f4c2e0-5d7a-4c1e-9a33-7e2f0d6c8a15", a, &[0])
        .expect("a");
    r.join(&mut f.env(), "b1f4c2ff-0000-4c1e-9a33-7e2f0d6c8a15", b, &[1])
        .expect("b");
    let got = r.device_infos();
    assert_eq!(got.len(), 2);
    assert_eq!((got[0].id.as_str(), got[1].id.as_str()), ("b1f4c2e", "b1f4c2f"));
}

#[test]
fn status_je_raum_nach_code() {
    let (mut m, _) = manager();
    add_room(&mut m, "ZZZZ", "zwei", 2);
    add_room(&mut m, "KRNZ", "eins", 2);
    m.rooms.get_mut("KRNZ").expect("Raum").paused = true; // ohne Schritte der Insel (RU18)
    let (x, _xi) = peer(1);
    m.join("xbox", x, "KRNZ", &[0]).expect("join");
    m.tick_all();
    let (rooms, failures) = m.status();
    assert!(failures.is_empty());
    assert_eq!(rooms.len(), 2);
    assert_eq!(
        (rooms[0].info.code.as_str(), rooms[0].devices, rooms[0].tick),
        ("KRNZ", 1, 1)
    );
    assert_eq!(rooms[0].monarchs, vec![MonarchState::Taken, MonarchState::Free]);
    assert_eq!(
        (rooms[1].info.code.as_str(), rooms[1].devices, rooms[1].tick),
        ("ZZZZ", 0, 0)
    );
    let json = serde_json::to_value(&rooms[0]).expect("json");
    for key in ["code", "name", "devices", "monarchs", "tick", "tickMs", "stages"] {
        assert!(json.get(key).is_some(), "{key} fehlt: {json}");
    }
}
