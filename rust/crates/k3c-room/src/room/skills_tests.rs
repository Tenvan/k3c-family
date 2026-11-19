//! Tests aus `skills_test.go` (S2.1, B-123, B-803): Schlag, Skill-Slot, Lernen und Respec über den Raum, zwei Spieler an
//! einem Gerät. Abweichung: Gos `r.isl.SkillPool` spiegelt Rust nach `World::skill_points` jeder Stufe (`put_stage_ref`),
//! darum setzt `set_pool` beides; Gos `reason(err)` ist hier `reason(&Result)`.
use std::collections::BTreeMap;

use k3c_sim::monarch::available_points;
use k3c_sim::state::{Player, PlayerCommand};

use super::Room;
use super::dev_tests::{dev_room, isl_player};
use super::testutil::{TestManager, rm};
use crate::fehler::Fehler;

fn learn(m: &mut TestManager, code: &str, id: &str, conn: u64, slot: i64, skill: &str) -> Result<(), Fehler> {
    m.with_room(code, |r, env| r.learn(env, id, conn, slot, skill))
        .expect("Raum")
}

fn respec(m: &mut TestManager, code: &str, slot: i64) -> Result<(), Fehler> {
    m.with_room(code, |r, env| r.respec(env, "a", 1, slot)).expect("Raum")
}

/// Der Grund eines `BadRequestReason` (Gos `SkillError.Reason`), sonst leer.
fn reason(res: &Result<(), Fehler>) -> String {
    match res {
        Err(Fehler::BadRequestReason { reason }) => reason.clone(),
        _ => String::new(),
    }
}

fn is_bad_request(res: &Result<(), Fehler>) -> bool {
    matches!(res, Err(e) if e.is(&Fehler::BadRequest))
}

/// Setzt den Fund-Pool der Insel samt Spiegel in den Stufen.
fn set_pool(r: &mut Room, n: i64) {
    r.isl.skill_pool = n;
    for w in &mut r.isl.stages {
        w.skill_points = n;
    }
}

fn player_mut(r: &mut Room, idx: i64) -> &mut Player {
    r.isl
        .stages
        .iter_mut()
        .flat_map(|w| w.players.iter_mut())
        .find(|p| p.index == idx)
        .expect("Spieler")
}

fn slot_idx(r: &Room, slot: i64) -> i64 {
    r.devices["a"].slots[&slot] as i64
}

/// Freie Punkte des Monarchen `idx` (Gos `sim.AvailablePoints`).
fn points(r: &Room, idx: i64) -> i64 {
    let stage = r.isl.stage_of(idx).expect("Stufe");
    let pi = r.isl.stages[stage]
        .players
        .iter()
        .position(|p| p.index == idx)
        .expect("Spieler");
    available_points(&r.isl.stages[stage], pi)
}

// (a) attack und skill kommen als PlayerCommand beim Monarchen des Slots an, der andere Slot bleibt leer.
#[test]
fn skills_eingabe_kommt_an() {
    let (mut m, code, _i) = dev_room();
    let cmd = BTreeMap::from([(
        1,
        PlayerCommand {
            attack: true,
            skill: 2,
            ..PlayerCommand::default()
        },
    )]);
    assert_eq!(m.with_room(&code, |r, _| r.input("a", 1, &cmd)), Some(Ok(())));
    let r = rm(&mut m, &code);
    let got = &r.monarchs[slot_idx(r, 1) as usize].input;
    assert!(got.attack && got.skill == 2, "Slot 1: {got:?}");
    let other = &r.monarchs[slot_idx(r, 0) as usize].input;
    assert!(!other.attack && other.skill == 0, "Slot 0 verändert: {other:?}");
}

// (c, e) Lernen kostet einen Pool-Punkt und betrifft nur den Monarchen des Slots; Fehler sind bad_request ohne Änderung.
#[test]
fn skills_lernen() {
    let (mut m, code, _i) = dev_room();
    let (i0, i1) = {
        let r = rm(&mut m, &code);
        (slot_idx(r, 0), slot_idx(r, 1))
    };
    let before0 = {
        let r = rm(&mut m, &code);
        let pool = isl_player(r, i1).skills.len() as i64 + 1; // genau ein freier Punkt für Slot 1
        set_pool(r, pool);
        isl_player(r, i0).skills.clone()
    };
    assert_eq!(learn(&mut m, &code, "a", 1, 1, "fireball"), Ok(()));
    let r = rm(&mut m, &code);
    assert!(
        isl_player(r, i1).skills.iter().any(|s| s == "fireball") && points(r, i1) == 0,
        "Slot 1: {:?}, Punkte {}",
        isl_player(r, i1).skills,
        points(r, i1)
    );
    assert_eq!(isl_player(r, i0).skills, before0, "Slot 0 verändert");
    let pool = r.isl.skill_pool + 1;
    set_pool(r, pool);
    // Fälle: Name, Gerät, Verbindung, Slot, Skill
    let cases: [(&str, &str, u64, i64, &str); 5] = [
        ("unbekannt", "a", 1, 1, "gibtsnicht"),
        ("schon gelernt", "a", 1, 1, "fireball"),
        ("fremdes Gerät", "b", 1, 0, "taunt"),
        ("fremder Slot", "a", 1, 3, "taunt"),
        ("fremde Verb.", "a", 9, 0, "taunt"),
    ];
    for (name, id, conn, slot, skill) in cases {
        let res = learn(&mut m, &code, id, conn, slot, skill);
        assert!(is_bad_request(&res), "{name}: {res:?}");
    }
    let r = rm(&mut m, &code);
    let pool = r.isl.skill_pool - 1;
    set_pool(r, pool);
    let res = learn(&mut m, &code, "a", 1, 1, "iceWall");
    assert!(
        is_bad_request(&res) && reason(&res) == r#"skill "iceWall": keine Punkte frei"#,
        "ohne Punkte: {res:?}"
    );
    let r = rm(&mut m, &code);
    assert!(
        isl_player(r, i0).skills == before0 && isl_player(r, i1).skills.len() as i64 == r.isl.skill_pool,
        "Ablehnung hat geändert: {:?} / {:?}",
        isl_player(r, i0).skills,
        isl_player(r, i1).skills
    );
}

// (d) Respec am Tag an der Burg leert die Skills; in der Nacht bad_request, die Skills bleiben.
#[test]
fn skills_respec() {
    let (mut m, code, _i) = dev_room();
    let i0 = slot_idx(rm(&mut m, &code), 0);
    {
        let r = rm(&mut m, &code);
        let pool = isl_player(r, i0).skills.len() as i64 + 1;
        set_pool(r, pool);
    }
    assert_eq!(learn(&mut m, &code, "a", 1, 0, "taunt"), Ok(()));
    let stage = {
        let r = rm(&mut m, &code);
        let stage = r.isl.stage_of(i0).expect("Stufe");
        r.isl.stages[stage].cycle.phase = "night".into();
        stage
    };
    let res = respec(&mut m, &code, 0);
    let r = rm(&mut m, &code);
    assert!(
        is_bad_request(&res)
            && reason(&res).starts_with("respec nur am Tag")
            && isl_player(r, i0).skills.iter().any(|s| s == "taunt"),
        "Nacht: {res:?}, {:?}",
        isl_player(r, i0).skills
    );
    r.isl.stages[stage].cycle.phase = "day".into();
    let castle = r.isl.stages[stage].castle.x;
    player_mut(r, i0).x = castle;
    assert_eq!(respec(&mut m, &code, 0), Ok(()));
    let p = isl_player(rm(&mut m, &code), i0);
    assert!(
        p.skills.is_empty() && p.slots.is_empty(),
        "nach Respec: {:?} {:?}",
        p.skills,
        p.slots
    );
}

// B-803/AC-02: Eine Ablehnung der Sim trägt ihren Grund (BadRequestReason), eine Ablehnung des Raums (fremder Slot) nicht.
#[test]
fn skills_grund() {
    let (mut m, code, _i) = dev_room();
    {
        let r = rm(&mut m, &code);
        let i1 = slot_idx(r, 1);
        let pool = isl_player(r, i1).skills.len() as i64; // kein freier Punkt
        set_pool(r, pool);
    }
    let res = learn(&mut m, &code, "a", 1, 1, "fireball");
    assert_eq!(reason(&res), r#"skill "fireball": keine Punkte frei"#, "ohne Punkte");
    assert_eq!(
        res.as_ref().err().map(ToString::to_string).as_deref(),
        Some(r#"bad_request: skill "fireball": keine Punkte frei"#),
        "Text wie in Go"
    );
    assert_eq!(
        learn(&mut m, &code, "a", 1, 3, "fireball"),
        Err(Fehler::BadRequest),
        "fremder Slot"
    );
}

// Respec am Tag, aber fern der Burg: bad_request mit Grund, die Skills bleiben (Go `CanRespec`).
#[test]
fn skills_respec_ferne_burg() {
    let (mut m, code, _i) = dev_room();
    let i0 = slot_idx(rm(&mut m, &code), 0);
    {
        let r = rm(&mut m, &code);
        let pool = isl_player(r, i0).skills.len() as i64 + 1;
        set_pool(r, pool);
    }
    assert_eq!(learn(&mut m, &code, "a", 1, 0, "taunt"), Ok(()));
    {
        let r = rm(&mut m, &code);
        let stage = r.isl.stage_of(i0).expect("Stufe");
        r.isl.stages[stage].cycle.phase = "day".into();
        let far = r.isl.stages[stage].castle.x + 1000.0;
        player_mut(r, i0).x = far;
    }
    let res = respec(&mut m, &code, 0);
    assert!(
        is_bad_request(&res) && reason(&res) == "respec nur an der Burg",
        "ferne Burg: {res:?}"
    );
    assert!(isl_player(rm(&mut m, &code), i0).skills.iter().any(|s| s == "taunt"));
}

// Inselwahl: ohne offenen Wechselpunkt, mit Krone im Flug und mit fremdem Slot bad_request mit Grund der Sim (B-961).
#[test]
fn skills_choose_island_ablehnung() {
    let (mut m, code, _i) = dev_room();
    let choose = |m: &mut TestManager, slot: i64, isle: &str| {
        m.with_room(&code, |r, env| r.choose_island(env, "a", 1, slot, isle))
            .expect("Raum")
    };
    let res = choose(&mut m, 0, "x");
    assert!(
        is_bad_request(&res) && reason(&res) == "insel: Wechselpunkt nicht offen",
        "zu: {res:?}"
    );
    let i0 = slot_idx(rm(&mut m, &code), 0);
    {
        let r = rm(&mut m, &code);
        r.isl.gate_open = true;
        r.isl.switch_ready = false;
        player_mut(r, i0).crown = Some(k3c_sim::state::Crown {
            owner: i0,
            ..Default::default()
        });
    }
    let res = choose(&mut m, 0, "x");
    assert!(
        is_bad_request(&res) && reason(&res) == "insel: ohne Krone keine Wahl",
        "ohne Krone: {res:?}"
    );
    assert_eq!(choose(&mut m, 3, "x"), Err(Fehler::BadRequest), "fremder Slot");
    assert!(rm(&mut m, &code).isl.pending_choice.is_empty());
}
