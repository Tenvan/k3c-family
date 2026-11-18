//! Ereignis `skillCast` (Go: `skills_cast_event_test.go`, B-674/AC-01): Aktive Skills melden Spieler, Skill, Wirkort und Radius;
//! Flächenzauber am Ziel, ohne Ziel kein Ereignis (nur `castFailed`). Zwei Spieler melden getrennt. Alle Tests lesen
//! Ereignisse und runden den Ort mit `unit_x` (events.go): sie warten auf RU19. Die Tests ab `own_` prüfen den Wirkort ohne
//! Ereignisse.

use super::caster::cast_place;
use super::testutil::*;
use crate::enemies::events_of_type;
use crate::enemies::testutil::{add_player, quiet_world};
use crate::island::testutil::{add_pool_points, must_learn};
use crate::monarch::can_learn;
use crate::pending::unit_x;
use crate::world::vals;

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x, hit_event)"]
fn skill_cast_wirkort_und_radius() {
    let mut w = quiet_world();
    let pi = caster_player(&mut w);
    let px = w.players[pi].x;
    let target = tough_enemy(&mut w, px + 8.0);
    skills_tick(&mut w, pi, &skill_cmd(1)); // fireball: range 10, radius 3
    let evs = events_of_type(&w, "skillCast");
    assert_eq!(evs.len(), 1, "fireball: {} skillCast, erwartet 1", evs.len());
    let ev = evs[0];
    let tx = unit_x(w.enemies[target].x);
    assert!(
        ev["from"] == w.players[pi].id && ev["skill"] == "fireball" && ev["x"] == tx && ev["radius"] == 3.0,
        "fireball: {ev:?}, Ziel bei {tx}"
    );
}

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x)"]
fn skill_cast_ohne_ziel_kein_ereignis() {
    let mut w = quiet_world();
    let pi = caster_player(&mut w);
    let px = w.players[pi].x;
    tough_enemy(&mut w, px + 16.0); // außerhalb von meteor (range 15)
    skills_tick(&mut w, pi, &skill_cmd(4));
    assert_eq!(events_of_type(&w, "skillCast").len(), 0, "meteor ohne Ziel: skillCast");
    assert_eq!(
        events_of_type(&w, "castFailed").len(),
        1,
        "meteor ohne Ziel: castFailed, erwartet 1"
    );
}

/// Ziel stirbt im selben Takt: Das Ereignis trägt den Ort, an dem gewirkt wurde; zweiter Spieler meldet getrennt.
#[test]
#[ignore = "wartet auf RU19 (emit, unit_x, hit_event)"]
fn skill_cast_ziel_stirbt_zwei_spieler() {
    let mut w = quiet_world();
    let p = caster_player(&mut w);
    let q = add_player(&mut w); // Startverteilung von Spieler 2 kann Zauber schon enthalten
    add_pool_points(&mut w, 7);
    let v = vals(&w);
    for id in MAGE_SKILLS {
        if can_learn(&w, &v, q, id).is_ok() {
            must_learn(&mut w, q, &[id]);
        }
    }
    w.players[q].x = w.players[p].x - 30.0;
    assert_eq!(
        w.players[q].slots.get(1).map(String::as_str),
        Some("iceWall"),
        "Slots Spieler 2: {:?}",
        w.players[q].slots
    );
    let px = w.players[p].x;
    let weak = crate::enemies::spawn_scaled(&mut w, "skeleton", px + 4.0, 0.01, 1.0, None);
    let qx = w.players[q].x;
    tough_enemy(&mut w, qx - 6.0);
    skills_tick(&mut w, p, &skill_cmd(1));
    skills_tick(&mut w, q, &skill_cmd(2)); // iceWall: range 8, radius 3
    let evs = events_of_type(&w, "skillCast");
    assert!(
        evs.len() == 2 && w.enemies[weak].hp <= 0.0,
        "{} skillCast, Ziel HP {}",
        evs.len(),
        w.enemies[weak].hp
    );
    let (pid, qid) = (w.players[p].id, w.players[q].id);
    assert!(
        evs[0]["from"] == pid && evs[0]["x"] == unit_x(px + 4.0),
        "Spieler 1: {:?}",
        evs[0]
    );
    assert!(
        evs[1]["from"] == qid && evs[1]["skill"] == "iceWall" && evs[1]["x"] == unit_x(qx - 6.0),
        "Spieler 2: {:?}",
        evs[1]
    );
}

/// Skills ohne Fläche melden am Wirkenden mit Radius 0 (B-674: wie heute am Monarchen).
#[test]
#[ignore = "wartet auf RU19 (emit, unit_x)"]
fn skill_cast_ohne_flaeche_am_monarchen() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    add_pool_points(&mut w, 2);
    must_learn(&mut w, pi, &["taunt"]);
    w.players[pi].x = w.hub_x + 40.0;
    let px = w.players[pi].x;
    tough_enemy(&mut w, px + 2.0);
    skills_tick(&mut w, pi, &skill_cmd(1));
    let evs = events_of_type(&w, "skillCast");
    assert!(
        evs.len() == 1 && evs[0]["x"] == unit_x(px) && evs[0]["radius"] == 0.0,
        "taunt: {evs:?}"
    );
}

/// Wirkort ohne Ereignis (`cast_place`): Flächenzauber am Ziel mit ihrem Radius, alle anderen Skills am Wirkenden mit Radius 0;
/// zwei Spieler haben je ihr eigenes Ziel.
#[test]
fn own_cast_place_flaeche_am_ziel_sonst_am_wirkenden_fuer_zwei_spieler() {
    let mut w = quiet_world();
    let (a, b) = (caster_player(&mut w), add_player(&mut w));
    w.players[b].x = w.hub_x + 140.0;
    let (ax, bx) = (w.players[a].x, w.players[b].x);
    tough_enemy(&mut w, ax + 8.0);
    tough_enemy(&mut w, bx - 4.0);
    let v = vals(&w);
    let skill = |id: &str| v.monarch.skills[id].effect.clone();
    let fireball = skill("fireball"); // range 10, radius 3
    assert_eq!(cast_place(&w, a, &fireball), (ax + 8.0, 3.0));
    assert_eq!(
        cast_place(&w, b, &fireball),
        (bx - 4.0, 3.0),
        "jeder Wirkende sucht sein eigenes Ziel"
    );
    let taunt = skill("taunt");
    assert_eq!(
        cast_place(&w, a, &taunt),
        (ax, 0.0),
        "ohne Fläche am Wirkenden, Radius 0"
    );
    w.enemies.clear();
    assert_eq!(cast_place(&w, a, &fireball), (ax, 0.0), "ohne Ziel am Wirkenden");
}
