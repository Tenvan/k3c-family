//! Tests des Beobachters (`observer.go`, B-352, TR4.2). Go hat keinen eigenen Test; die Abdeckung dort ist indirekt
//! (`sim_test`, Pause und Dev-Aktionen). Hier: Beitritt ohne Platz, keine Eingabe, Zählung, Frist, Plätze freigeben.
use std::collections::BTreeMap;

use k3c_sim::state::PlayerCommand;

use super::save_testutil::{create_with, logged};
use super::testutil::{create_room, join_room, manager, peer, rm, wait};
use super::{EMPTY_FOR_SECS, MonarchState, Options};
use crate::fehler::Fehler;

#[test]
fn beobachter_ohne_platz_und_ohne_eingabe() {
    let (mut m, _) = manager();
    let (code, _xi) = create_room(&mut m, "xbox", 1, "beo", 0, &[0]);
    let (o, mut oi) = peer(5);
    m.observe("sim", o, &code).expect("observe");
    let r = rm(&mut m, &code);
    assert_eq!((r.connected(), r.watching(), r.monarchs.len()), (1, 1, 1));
    assert!(oi.has(&format!("joined {code} beo []")), "{:?}", oi.log());
    assert!(oi.has("level 0"), "Beobachter bekommt das Level der Startstufe");
    assert!(
        oi.msgs().iter().any(|e| e.starts_with("state ")),
        "Beobachter bekommt Zustand"
    );
    let cmd = BTreeMap::from([(0, PlayerCommand::default())]);
    assert_eq!(r.input("sim", 5, &cmd), Err(Fehler::BadRequest));
    assert_eq!(r.add_slot("sim", 5, 1), Err(Fehler::BadRequest));
    assert!(logged(&m, "🔌 Beobachter im Raum") && logged(&m, "device=sim geraete=1 beobachter=1"));
}

#[test]
fn beobachter_unbekannter_raum_und_nach_close() {
    let (mut m, _) = manager();
    let (o, _) = peer(5);
    assert_eq!(m.observe("sim", o, "ZZZZ"), Err(Fehler::RoomNotFound));
    assert!(logged(
        &m,
        "🚫 Beitritt abgelehnt ns=room op=observe device=sim ziel=ZZZZ code=room_not_found"
    ));
    m.close();
    let (o, _) = peer(6);
    assert_eq!(m.observe("sim", o, "ZZZZ"), Err(Fehler::Closed));
}

#[test]
fn beobachter_verlaesst_raum_frist_laeuft() {
    let (mut m, clock) = manager();
    let (p, _oi) = peer(5);
    let code = m
        .create_observed("sim", p, "beo", true, 0, Options::default())
        .expect("create_observed");
    {
        let r = rm(&mut m, &code);
        assert_eq!((r.connected(), r.watching()), (0, 1), "nur ein Beobachter");
        assert_eq!(r.observer_stages(), vec![r.start], "ohne Monarchen die Startstufe");
    }
    m.with_room(&code, |r, env| r.leave(env, "sim", 5)).expect("Raum");
    assert_eq!(rm(&mut m, &code).watching(), 0);
    assert!(logged(&m, "👋 Beobachter hat den Raum verlassen") && logged(&m, "device=sim beobachter=0"));
    wait(&mut m, &clock, EMPTY_FOR_SECS + 60);
    assert!(m.rooms().is_empty(), "leerer Raum nach der Frist aufgeräumt");
}

// Ein Gerät mit Plätzen, das als Beobachter beitritt, gibt seine Monarchen frei.
#[test]
fn beobachter_gibt_plaetze_frei() {
    let (mut m, _) = manager();
    let (code, _xi) = create_with(&mut m, "xbox", 1, "beo", true, Options::default());
    let _hi = join_room(&mut m, "handy", 2, &code, &[0]);
    let (o, _oi) = peer(3);
    m.observe("handy", o, &code).expect("observe");
    let r = rm(&mut m, &code);
    assert_eq!(r.monarchs[0].state, MonarchState::Taken, "xbox hält Monarch 0");
    assert_eq!((r.connected(), r.watching()), (1, 1));
    assert_eq!(r.observer_stages(), vec![0]);
}
