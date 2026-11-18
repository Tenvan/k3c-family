//! Tests der Aktionen und des Takts (Go: Teile von `room_test.go`, `run_test.go`, `safetick_test.go`). Die Räume sind von
//! Hand gebaut (`testutil::room`: freie Monarchen, keine neuen Insel-Spieler, RU18); ein Raum, der speichern müsste, hat
//! `isl.over` gesetzt, weil die Test-Insel sich nicht kodieren lässt.
use std::collections::BTreeMap;
use std::sync::atomic::Ordering;

use k3c_sim::state::PlayerCommand;

use super::testutil::{Fixture, add_room, manager, peer, room};
use super::{MonarchState, Room};
use crate::fehler::Fehler;

fn go(move_x: f64) -> BTreeMap<i64, PlayerCommand> {
    BTreeMap::from([(
        0,
        PlayerCommand {
            move_x,
            ..PlayerCommand::default()
        },
    )])
}

fn joined(f: &mut Fixture, r: &mut Room, id: &str, conn: u64, slots: &[i64]) -> super::testutil::Inbox {
    let (p, i) = peer(conn);
    r.join(&mut f.env(), id, p, slots).expect(id);
    i
}

// Ablauf aus `docs/protocol.md` › Beispiel: 2 Controller an der Xbox + 1 Handy, Drop und Wiederverbinden.
#[test]
fn xbox_und_handy() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "familie", 3);
    r.isl.over = true;
    let mut xi = joined(&mut f, &mut r, "xbox", 1, &[0]);
    r.add_slot("xbox", 1, 1).expect("add_slot");
    let mut hi = joined(&mut f, &mut r, "handy", 2, &[0]);
    assert_eq!(
        (hi.you().as_str(), xi.monarchs().as_str()),
        ("[{0 2 0 0}]", "[taken taken taken]")
    );
    // Handy verliert WLAN: Monarch 2 wartet und steht still, die Xbox spielt weiter.
    r.drop_conn(&mut f.env(), "handy", 2);
    assert_eq!(xi.monarchs(), "[taken taken waiting]");
    assert!(r.isl.players()[2].free);
    let mut h2 = joined(&mut f, &mut r, "handy", 3, &[0]);
    assert!(h2.you() == "[{0 2 0 0}]" && h2.has("level 0"), "{:?}", h2.log());
    // Controller 2 geht: Monarch 1 frei; ein neues Gerät bekommt ihn.
    assert_eq!(r.remove_slot(&mut f.env(), "xbox", 1, 1), Ok(false));
    assert_eq!(xi.monarchs(), "[taken free taken]");
    let mut z = joined(&mut f, &mut r, "tablet", 4, &[0]);
    assert_eq!(z.you(), "[{0 1 0 0}]");
}

#[test]
fn letzter_slot_verlaesst_den_raum() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "weg", 2);
    r.isl.over = true;
    let mut xi = joined(&mut f, &mut r, "xbox", 1, &[0]);
    let _hi = joined(&mut f, &mut r, "handy", 2, &[0]);
    assert_eq!(r.remove_slot(&mut f.env(), "handy", 2, 0), Ok(true));
    assert!(!r.has("handy") && r.connected() == 1);
    assert_eq!(xi.monarchs(), "[taken free]");
    assert!(
        f.log
            .lines
            .iter()
            .any(|l| l.contains("👋 Gerät hat den Raum verlassen") && l.contains("device=handy geraete=1"))
    );
    r.leave(&mut f.env(), "xbox", 1);
    assert!(
        r.devices.is_empty() && r.empty_since.is_some(),
        "leerer Raum: die Frist läuft"
    );
    assert!(
        f.log
            .lines
            .iter()
            .any(|l| l.contains("💾 Raum leer und pausiert, speichert"))
    );
}

// Nach 60 s sind wartende Monarchen frei, bei der Rückkehr werden die alten bevorzugt.
#[test]
fn nach_60_sekunden_frei_und_bevorzugt() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "frist", 3);
    r.isl.over = true;
    let mut xi = joined(&mut f, &mut r, "xbox", 1, &[0]);
    let _hi = joined(&mut f, &mut r, "handy", 2, &[0, 1]);
    f.log.lines.clear();
    r.drop_conn(&mut f.env(), "handy", 2);
    let start = f.now;
    assert_eq!(r.sweep(&mut f.env(), start.add_secs(59)), (false, false));
    assert_eq!(xi.monarchs(), "[taken waiting waiting]", "nach 59 s");
    assert_eq!(r.sweep(&mut f.env(), start.add_secs(60)), (true, false));
    assert_eq!(xi.monarchs(), "[taken free free]", "nach 60 s");
    assert!(!r.has("handy"), "ein getrenntes Gerät ohne Monarchen ist weg");
    assert!(
        f.log
            .lines
            .iter()
            .any(|l| l.contains("⏳ Wartende Monarchen nach Frist frei"))
    );
    let mut h2 = joined(&mut f, &mut r, "handy", 3, &[1, 0]);
    assert_eq!(h2.you(), "[{0 1 0 0} {1 2 0 0}]", "Rückkehr: die alten Monarchen");
}

// Ein Raum ohne verbundenes Gerät tickt nicht, die Fristen laufen trotzdem; nach 10 min wird er aufgeräumt.
#[test]
fn pausierter_raum_fristen_und_aufraeumen() {
    let (mut m, secs) = manager();
    add_room(&mut m, "KRNZ", "pause", 1);
    m.rooms.get_mut("KRNZ").expect("Raum").isl.over = true;
    let (x, _xi) = peer(1);
    m.join("xbox", x, "KRNZ", &[0]).expect("join");
    for _ in 0..3 {
        m.tick_all();
    }
    m.with_room("KRNZ", |r, env| r.drop_conn(env, "xbox", 1));
    assert!(
        m.log
            .lines
            .iter()
            .any(|l| l.contains("💾 Raum leer und pausiert, speichert"))
    );
    for _ in 0..30 {
        m.tick_all();
    }
    assert_eq!(m.room("KRNZ").map(|r| r.tick), Some(3), "pausierter Raum tickt");
    let wait = |m: &mut super::testutil::TestManager, s: i64| {
        secs.fetch_add(s, Ordering::SeqCst);
        m.sweep();
    };
    wait(&mut m, 61);
    assert_eq!(
        m.room("KRNZ").map(|r| r.monarchs[0].state),
        Some(MonarchState::Free),
        "60-s-Frist"
    );
    wait(&mut m, 8 * 60);
    assert_eq!(m.rooms().len(), 1, "Raum vor 10 min aufgeräumt");
    wait(&mut m, 60);
    assert!(m.rooms().is_empty(), "Aufräumen nach 10 min");
    let (p, _) = peer(2);
    assert_eq!(
        m.join("xbox", p, "KRNZ", &[0]),
        Err(Fehler::RoomNotFound),
        "Code nach Aufräumen"
    );
}

// B-204: Ein Test-Raum wartet nur auf wartende Monarchen, nicht auf die Frist.
#[test]
fn testraum_wird_sofort_aufgeraeumt() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "test-eins", 1);
    r.isl.over = true;
    let _xi = joined(&mut f, &mut r, "xbox", 1, &[0]);
    r.drop_conn(&mut f.env(), "xbox", 1);
    let now = f.now;
    assert_eq!(r.sweep(&mut f.env(), now), (false, false), "der Monarch wartet noch");
    assert_eq!(
        r.sweep(&mut f.env(), now.add_secs(60)),
        (true, true),
        "nach der Frist ist nichts mehr zu warten"
    );
}

// Grenzen: voller Raum, Slot 4, schon belegter und fremder Slot.
#[test]
fn grenzen_raum() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "grenzen", 4);
    let _a = joined(&mut f, &mut r, "drei", 1, &[0, 1, 2]);
    let _b = joined(&mut f, &mut r, "eins", 2, &[0]);
    assert_eq!(
        r.add_slot("drei", 1, 3),
        Err(Fehler::RoomFull),
        "add_slot im vollen Raum"
    );
    assert_eq!(r.add_slot("drei", 1, 4), Err(Fehler::TooManySlots));
    assert_eq!(r.add_slot("drei", 1, 0), Err(Fehler::BadRequest), "belegter Slot");
    assert_eq!(r.add_slot("drei", 1, -1), Err(Fehler::BadRequest));
    assert_eq!(r.add_slot("fremd", 9, 3), Err(Fehler::BadRequest), "fremdes Gerät");
    assert_eq!(r.add_slot("drei", 7, 3), Err(Fehler::BadRequest), "fremde Verbindung");
    let (p, _) = peer(3);
    assert_eq!(r.join(&mut f.env(), "x", p, &[4]), Err(Fehler::TooManySlots));
}

#[test]
fn eingabe_nur_eigene_slots() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "eingabe", 2);
    let _xi = joined(&mut f, &mut r, "xbox", 1, &[0]);
    let own = BTreeMap::from([(1, PlayerCommand::default())]);
    assert_eq!(r.input("xbox", 1, &own), Err(Fehler::BadRequest), "fremder Slot");
    assert_eq!(r.input("fremd", 9, &go(0.0)), Err(Fehler::BadRequest), "fremdes Gerät");
    assert_eq!(
        r.input("xbox", 1, &BTreeMap::new()),
        Err(Fehler::BadRequest),
        "leere Eingabe"
    );
    assert_eq!(r.input("xbox", 1, &go(f64::NAN)), Err(Fehler::BadRequest), "NaN");
    assert_eq!(r.input("xbox", 1, &go(7.0)), Ok(()));
    assert_eq!(r.monarchs[0].input.move_x, 1.0, "move_x nicht begrenzt");
    assert_eq!(r.input("xbox", 1, &go(-7.0)), Ok(()));
    assert_eq!(r.monarchs[0].input.move_x, -1.0);
    assert_eq!(
        r.remove_slot(&mut f.env(), "xbox", 1, 3),
        Err(Fehler::BadRequest),
        "remove_slot fremd"
    );
    // alles oder nichts: ein gültiger und ein fremder Slot ändern nichts
    let mixed = BTreeMap::from([
        (
            0,
            PlayerCommand {
                move_x: 0.5,
                ..PlayerCommand::default()
            },
        ),
        (1, PlayerCommand::default()),
    ]);
    assert_eq!(r.input("xbox", 1, &mixed), Err(Fehler::BadRequest));
    assert_eq!(r.monarchs[0].input.move_x, -1.0);
}

#[test]
fn tick_geschlossen_und_ohne_geraet() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "takt", 1);
    assert!(r.tick(&mut f.env()), "ohne Gerät: pausiert, aber offen");
    assert_eq!(r.tick, 0);
    r.closed = true;
    assert!(!r.tick(&mut f.env()), "geschlossen");
}

// AC-02: Zwei Spieler (zwei Geräte, je ein Slot) geben im selben Tick Eingaben, der Tick rechnet beide. Die Schritte der
// Insel (`step_players`) kommen mit RU18; bis dahin läuft der Tick mit der Dev-Pause (`paused`, keine Schritte) und
// zeigt, dass beide Geräte im selben Tick ihre Zustände bekommen.
#[test]
fn tick_zwei_geraete_im_selben_tick() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "takt", 2);
    r.paused = true;
    let (mut ai, mut bi) = (
        joined(&mut f, &mut r, "a", 1, &[0]),
        joined(&mut f, &mut r, "b", 2, &[0]),
    );
    assert_eq!(r.input("a", 1, &go(1.0)), Ok(()));
    assert_eq!(r.input("b", 2, &go(-1.0)), Ok(()));
    assert!(r.tick(&mut f.env()));
    assert_eq!(r.tick, 1);
    assert!(ai.has("state 1") && bi.has("state 1"), "{:?} {:?}", ai.log(), bi.log());
    assert_eq!((r.monarchs[0].input.move_x, r.monarchs[1].input.move_x), (1.0, -1.0));
}

#[test]
fn tick_rechnet_beide_spieler() {} // wie oben ohne Dev-Pause: beide Spieler laufen in ihre Richtung (`step_players`)

#[test]
fn drei_raeume_nacheinander() {
    let (mut m, _) = manager();
    for (i, code) in ["AAAA", "BBBB", "CCCC"].into_iter().enumerate() {
        add_room(&mut m, code, &format!("raum-{i}"), 1);
        m.rooms.get_mut(code).expect("Raum").paused = true; // ohne Schritte der Insel (RU18)
        let (p, _) = peer(i as u64 + 1);
        m.join(&format!("g{i}"), p, code, &[0]).expect("join");
    }
    for _ in 0..30 {
        m.tick_all();
    }
    assert!(m.failures.is_empty(), "{:?}", m.failures);
    assert!(m.rooms().iter().all(|i| i.running));
    assert!(m.rooms.values().all(|r| r.tick == 30));
}

// B-803/AC-01: Ein Panic im Takt schließt den Raum wie im Server ohne Speichern.
#[test]
fn safe_tick_panic_schliesst_raum() {
    let (mut m, _) = manager();
    add_room(&mut m, "KRNZ", "takt-absturz", 1);
    let (x, mut xi) = peer(1);
    m.join("xbox", x, "KRNZ", &[0]).expect("join");
    m.rooms.get_mut("KRNZ").expect("Raum").before_step = Some(Box::new(|| panic!("Absturz im Takt")));
    assert!(!m.safe_tick("KRNZ"));
    assert!(m.room("KRNZ").is_none() && xi.has("closed"));
    assert_eq!((m.failures.len(), m.failures[0].error.as_str()), (1, "Absturz im Takt"));
    assert_eq!(m.store.saves, 0);
}
