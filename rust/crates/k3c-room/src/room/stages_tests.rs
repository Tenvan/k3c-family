//! Tests aus `stages_test.go` (B-176) und `stages_bench_test.go` (`TestStufenEreignisBudgetJeGeraet`): Stufen je Gerät im
//! Raum. `BenchmarkStufenZweiSpieler` wird nicht portiert (Benchmark, die Messung übernimmt RU39).
use std::collections::BTreeMap;

use k3c_sim::state::PlayerCommand;

use super::testutil::{Inbox, TestManager, create_room, join_room, manager, move_to_exit, rm, ticks};

/// Xbox mit Slots 0 und 1 (Monarchen 0 und 1), Handy mit Slot 0 (Monarch 2); Monarch 0 wechselt in Stufe 1 (Go `zweiStufen`).
pub(super) fn zwei_stufen() -> (TestManager, String, Inbox, Inbox) {
    let (mut m, _) = manager();
    let (code, x) = create_room(&mut m, "xbox", 1, "stufen", 0, &[0, 1]);
    let h = join_room(&mut m, "handy", 2, &code, &[0]);
    move_to_exit(rm(&mut m, &code), 0);
    ticks(&mut m, &code, 70);
    let r = rm(&mut m, &code);
    let stages = (r.isl.stage_of(0), r.isl.stage_of(1), r.isl.stage_of(2));
    assert_eq!(stages, (Some(1), Some(0), Some(0)), "Stufen: {:?}", r.depths());
    (m, code, x, h)
}

fn msg(kind: &str, stage: usize) -> String {
    format!("{kind} {stage}")
}

fn pos(msgs: &[String], m: &str) -> Option<usize> {
    msgs.iter().position(|x| x == m)
}

/// Für Stufe `s` kommt kein Zustand vor ihrem Level, und das Level steht vor dem ersten Zustand (Go `beginsWithLevel`).
pub(super) fn begins_with_level(msgs: &[String], s: usize) -> bool {
    match (pos(msgs, &msg("level", s)), pos(msgs, &msg("state", s))) {
        (Some(lv), Some(st)) => st > lv,
        _ => false,
    }
}

fn texts(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

// (a) B-176/AC-02: Ein Gerät mit zwei Spielern in zwei Stufen bekommt Level und Zustand beider Stufen, je Tick aufsteigend;
// ein Gerät mit einem Spieler nur seine Stufe. `seats` nennt die Stufe je Slot.
#[test]
fn stufen_zwei_spieler_ein_geraet() {
    let (mut m, code, mut x, mut h) = zwei_stufen();
    let (xm, hm) = (x.msgs(), h.msgs());
    assert!(
        begins_with_level(&xm, 1) && !hm.contains(&msg("level", 1)) && !hm.contains(&msg("state", 1)),
        "Xbox {xm:?}, Handy {hm:?}"
    );
    let (xi, hi) = (xm.len(), hm.len());
    ticks(&mut m, &code, 2);
    assert_eq!(
        x.msgs()[xi..],
        texts(&["state 0", "state 1", "state 0", "state 1"]),
        "Xbox je Tick"
    );
    assert_eq!(h.msgs()[hi..], texts(&["state 0", "state 0"]), "Handy je Tick");
    assert_eq!(x.seats_now(), "[{0 0 1 1} {1 1 0 0}]");
}

// (b) B-176/AC-03: Verlässt der einzige Spieler eines Geräts seine Stufe, endet der alte Strom; der neue beginnt mit Level
// und Zustand. Verlässt einer von zwei Spielern eine Stufe, die das Gerät weiter hat, kommt kein neues Level.
#[test]
fn stufen_wechsel_beendet_alten_strom() {
    let (mut m, code, mut x, mut h) = zwei_stufen();
    let (hi, xi) = (h.msgs().len(), x.msgs().len());
    move_to_exit(rm(&mut m, &code), 2);
    ticks(&mut m, &code, 70);
    assert_eq!(rm(&mut m, &code).isl.stage_of(2), Some(1), "Handy-Monarch in Stufe 1");
    let got = h.msgs()[hi..].to_vec();
    let lv = pos(&got, "level 1").expect("level 1");
    assert!(
        begins_with_level(&got, 1) && !got[lv..].contains(&msg("state", 0)),
        "Handy: {got:?}"
    );
    move_to_exit(rm(&mut m, &code), 1); // der zweite Xbox-Spieler folgt in Stufe 1, Stufe 0 hat kein Xbox-Slot mehr
    ticks(&mut m, &code, 70);
    let got = x.msgs()[xi..].to_vec();
    let mut last = pos(&got, "state 1").expect("state 1");
    for (i, mm) in got.iter().enumerate() {
        if mm == "state 0" {
            last = i;
        }
    }
    assert!(
        !got.contains(&msg("level", 1))
            && !got.contains(&msg("level", 0))
            && !got[last + 1..].contains(&msg("state", 0)),
        "Xbox: {got:?}"
    );
    let after = &got[last + 1..];
    assert!(
        after.len() >= 10 && after.iter().all(|m| m == "state 1"),
        "Xbox nach dem Wechsel: {after:?}"
    );
}

// (c) B-176/AC-03: Wiederverbinden liefert Level und Zustand aller Stufen des Geräts, aufsteigend.
#[test]
fn stufen_wiederverbinden() {
    let (mut m, code, x, _) = zwei_stufen();
    drop(x);
    m.with_room(&code, |r, env| r.drop_conn(env, "xbox", 1));
    let mut x2 = join_room(&mut m, "xbox", 3, &code, &[0, 1]);
    assert_eq!(
        x2.msgs(),
        texts(&["level 0", "state 0", "level 1", "state 1"]),
        "nach Wiederverbinden"
    );
    ticks(&mut m, &code, 1);
    assert_eq!(x2.msgs()[4..], texts(&["state 0", "state 1"]), "danach");
}

// B-176/AC-04: Das Mittel der Ereignis-Bytes je Tick und Gerät (Summe beider Stufen) bleibt im Budget aus Q08: 200 Byte
// je Tick = 6 KB/s bei 30 Ticks/s. Aufstellung: ein Gerät, zwei Spieler in Stufe 0 und 1 (Seed `bench`), schneller Zyklus
// (60), aufgewärmt bis in die Nacht; danach 1800 Ticks (1 min) gemessen.
const EVENT_BUDGET_BYTES: f64 = 200.0;
const STUFEN_TICKS: usize = 1800;

/// Rechnet, bis Monarch 0 in Stufe 1 und es in Stufe 0 Nacht ist (höchstens 2 min).
fn warm_up_to_night(m: &mut TestManager, code: &str) {
    for i in 0.. {
        let r = rm(m, code);
        if r.isl.stage_of(0) == Some(1) && r.isl.stages[0].cycle.phase == "night" {
            return;
        }
        assert!(i <= 30 * 120, "keine Nacht in zwei Stufen nach 2 min");
        ticks(m, code, 1);
    }
}

/// Die Bytes der Ereignisse je Tick, summiert über die Stufen des Geräts (Go `countPeer`).
fn event_bytes_per_tick(events: &[(i64, i64, Vec<serde_json::Value>)]) -> Vec<usize> {
    let mut per_tick: Vec<usize> = Vec::new();
    let mut last = -1;
    for (tick, _, evs) in events {
        let bytes = serde_json::to_string(evs).map_or(0, |s| s.len());
        match per_tick.last_mut() {
            Some(b) if *tick == last => *b += bytes,
            _ => {
                per_tick.push(bytes);
                last = *tick;
            }
        }
    }
    per_tick
}

#[test]
fn stufen_ereignis_budget_je_geraet() {
    let (mut m, _) = manager();
    let (code, mut p) = create_room(&mut m, "bench", 1, "bench", 0, &[0, 1]);
    move_to_exit(rm(&mut m, &code), 0);
    for w in &mut rm(&mut m, &code).isl.stages {
        w.cycle_speed = 60.0;
    }
    warm_up_to_night(&mut m, &code);
    let cmd = |move_x: f64, pay: bool| PlayerCommand {
        move_x,
        pay,
        ..Default::default()
    };
    let both = BTreeMap::from([(0, cmd(1.0, false)), (1, cmd(0.0, true))]);
    assert_eq!(rm(&mut m, &code).input("bench", 1, &both), Ok(()));
    let skip = p.events().len();
    let mut nights = 0;
    for _ in 0..STUFEN_TICKS {
        ticks(&mut m, &code, 1);
        if rm(&mut m, &code).isl.stages[0].cycle.phase == "night" {
            nights += 1;
        }
    }
    let per_tick = event_bytes_per_tick(&p.events()[skip..]);
    let r = rm(&mut m, &code);
    assert!(
        per_tick.len() == STUFEN_TICKS && r.isl.stage_of(0) != r.isl.stage_of(1),
        "{} Ticks gezählt, Stufen {:?}",
        per_tick.len(),
        r.depths()
    );
    let mean = per_tick.iter().sum::<usize>() as f64 / per_tick.len() as f64;
    assert!(nights > 0, "der Lauf erreicht keine Nacht");
    assert!(
        mean <= EVENT_BUDGET_BYTES,
        "Ereignis-Budget je Gerät überschritten: Mittel {mean:.1} Byte je Tick > {EVENT_BUDGET_BYTES}"
    );
}
