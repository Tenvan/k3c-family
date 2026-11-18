//! Tests aus `dev_combat_test.go` (B-603/AC-01): Die Kampf-Aktionen wirken im Dev-Mode (Gerät und `/dm`), sonst
//! forbidden; falsche Felder sind bad_request.
use super::DevAction;
use super::dev_tests::{act, dev, dev_room, isl_player};
use super::testutil::{TestManager, rm, ticks};
use crate::fehler::Fehler;

fn boss_count(m: &mut TestManager, code: &str) -> usize {
    let r = rm(m, code);
    r.isl
        .stages
        .iter()
        .flat_map(|w| w.enemies.iter())
        .filter(|e| e.boss)
        .count()
}

fn boss(slot: Option<i64>, id: &str) -> DevAction {
    DevAction {
        slot,
        boss: id.into(),
        ..act("boss")
    }
}

fn event(id: &str) -> DevAction {
    DevAction {
        event: id.into(),
        ..act("event")
    }
}

// boss stellt den Boss in die Stufe des Monarchen; ein zweiter wird mit Hinweis abgelehnt.
#[test]
fn dev_boss() {
    let (mut m, code, _i) = dev_room();
    assert_eq!(dev(&mut m, &code, &boss(Some(1), "goblinLeader")), Ok(()));
    assert_eq!(boss_count(&mut m, &code), 1, "Bosse");
    assert_eq!(
        dev(&mut m, &code, &boss(Some(0), "trollKing")),
        Err(Fehler::BadRequest),
        "zweiter Boss"
    );
    let log = m.log.lines.join("\n");
    assert!(
        log.contains("aktion=boss") && log.contains("Boss schon da"),
        "Log ohne Aktion oder Hinweis:\n{log}"
    );
    for a in [
        boss(Some(0), "drache"),
        boss(None, "goblinLeader"),
        boss(Some(3), "goblinLeader"),
    ] {
        assert_eq!(dev(&mut m, &code, &a), Err(Fehler::BadRequest), "{a:?}");
    }
}

// bossPhase schaltet den Endboss (über /dm gerufen) weiter; ohne Endboss und nach der letzten Phase bad_request.
#[test]
fn dev_boss_phase() {
    let (mut m, code, _i) = dev_room();
    assert_eq!(
        dev(&mut m, &code, &act("bossPhase")),
        Err(Fehler::BadRequest),
        "ohne Endboss"
    );
    let r = rm(&mut m, &code);
    let m0 = isl_player(r, r.devices["a"].slots[&0] as i64).index;
    let page = boss(Some(m0), "crystalHeartWarden");
    assert_eq!(m.with_room(&code, |r, env| r.dev_page(env, &page)), Some(Ok(())));
    for _ in 0..2 {
        assert_eq!(dev(&mut m, &code, &act("bossPhase")), Ok(()));
    }
    assert_eq!(
        dev(&mut m, &code, &act("bossPhase")),
        Err(Fehler::BadRequest),
        "nach der letzten Phase"
    );
}

// event springt zur Nacht mit dem Event, night zur nächsten Nacht; unbekannte Events sind bad_request.
#[test]
fn dev_event_und_nacht() {
    let (mut m, code, _i) = dev_room();
    assert_eq!(dev(&mut m, &code, &event("fullMoon")), Ok(()));
    ticks(&mut m, &code, 1);
    let w = &rm(&mut m, &code).isl.stages[0];
    assert!(
        w.cycle.phase == "night" && w.night_events.len() == 1 && w.night_events[0].id == "fullMoon",
        "Phase {}, Events {:?}",
        w.cycle.phase,
        w.night_events
    );
    let day = w.cycle.day;
    assert_eq!(dev(&mut m, &code, &act("night")), Ok(()));
    ticks(&mut m, &code, 1);
    let w = &rm(&mut m, &code).isl.stages[0];
    assert!(
        w.cycle.phase == "night" && w.cycle.day == day + 1,
        "Phase {} Tag {}, erwartet Nacht {}",
        w.cycle.phase,
        w.cycle.day,
        day + 1
    );
    assert_eq!(dev(&mut m, &code, &event("merchantRaid")), Ok(()));
    for ev in ["", "neumond", "merchantRaid"] {
        assert_eq!(dev(&mut m, &code, &event(ev)), Err(Fehler::BadRequest), "event {ev:?}");
    }
}

// Ohne Dev-Mode sind alle Kampf-Aktionen verboten und ändern nichts.
#[test]
fn dev_kampf_verboten() {
    let (mut m, code, _i) = dev_room();
    m.dev = false;
    for a in [
        boss(Some(0), "goblinLeader"),
        act("bossPhase"),
        event("bloodMoon"),
        act("night"),
    ] {
        assert_eq!(dev(&mut m, &code, &a), Err(Fehler::Forbidden), "{}", a.action);
    }
    assert_eq!(boss_count(&mut m, &code), 0, "verbotene Aktion hat gewirkt");
    assert!(
        rm(&mut m, &code).isl.dev.nights.is_empty(),
        "verbotene Aktion hat gewirkt"
    );
}
