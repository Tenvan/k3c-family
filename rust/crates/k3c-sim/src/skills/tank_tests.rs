//! Tank-Linie (Go: `skills_tank_test.go`): Wirkung, Reichweite und Abklingzeit laut monarch.json (S1.2a, B-119). Wo ein Skill
//! wirkt, meldet er `skillCast` über `emit` und `unit_x` (RU19); ein Treffer meldet `hit_event`: diese Tests warten darauf.
//! Die Tests ab `own_` haben kein Go-Gegenstück und prüfen die Wirkungen ohne diese Funktionen.

use super::tank::{cast_last_stand, cast_shield, cast_stun, cast_taunt};
use super::testutil::*;
use crate::enemies::step_enemies;
use crate::enemies::stunned;
use crate::enemies::target::choose_target;
use crate::enemies::testutil::{PROBE_GOLD, add_player, probe, quiet_world, spawn, test_values};
use crate::state::common::{apply_damage, sign};
use crate::world::testutil::DT;
use crate::world::vals;

use k3c_data::monarch::SkillEffect;

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x)"]
fn tank_taunt() {
    let mut w = quiet_world();
    let p0 = tank_player(&mut w);
    let p1 = add_player(&mut w);
    let px = w.players[p0].x;
    let (near, far) = (
        spawn(&mut w, "skeleton", px + 10.0),
        spawn(&mut w, "skeleton", px - 11.0),
    );
    skills_tick(&mut w, p0, &skill_cmd(1));
    let id0 = w.players[p0].id;
    let (n, f) = (&w.enemies[near], &w.enemies[far]);
    assert!(
        n.taunt_id == id0 && n.taunt_for == 4.0 && f.taunt_for == 0.0 && w.players[p0].cooldowns[0] == 15.0,
        "Taunt: 10u {}/{}, 11u {}, CD {:?}",
        n.taunt_id,
        n.taunt_for,
        f.taunt_for,
        w.players[p0].cooldowns
    );
    // In Reichweite beider Monarchen: ohne Taunt das nähere Ziel (Spieler 1), mit Taunt der Wirkende.
    w.enemies[near].x = px + 1.2;
    w.players[p1].x = px + 1.0;
    let dir = sign(w.hub_x - w.enemies[near].x);
    let v = vals(&w);
    let c = choose_target(&w, &v, near, dir, None);
    assert!(
        c.as_ref().is_some_and(|c| c.id == id0),
        "verspotteter Gegner zielt auf {c:?}, erwartet Spieler {id0}"
    );
    for _ in 0..121 {
        // 4 s
        stunned(&mut w.enemies[near], DT);
    }
    let c = choose_target(&w, &v, near, dir, None);
    let id1 = w.players[p1].id;
    assert!(
        c.as_ref().is_some_and(|c| c.id == id1) && w.enemies[near].taunt_id == 0,
        "nach 4 s noch verspottet: Ziel {c:?}, TauntID {}",
        w.enemies[near].taunt_id
    );
}

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x, pure)"]
fn tank_shield_bash() {
    let mut w = quiet_world();
    let pi = tank_player(&mut w);
    w.players[pi].x = w.hub_x + 20.0;
    let px = w.players[pi].x;
    let far = spawn(&mut w, "skeleton", px + 2.5);
    skills_tick(&mut w, pi, &skill_cmd(2));
    assert!(
        w.enemies[far].stun == 0.0 && w.players[pi].cooldowns.is_empty(),
        "Gegner in 2,5u betäubt oder Abklingzeit ohne Treffer: Stun {}, CD {:?}",
        w.enemies[far].stun,
        w.players[pi].cooldowns
    );
    let e = spawn(&mut w, "skeleton", px + 2.0);
    skills_tick(&mut w, pi, &skill_cmd(2));
    assert!(
        w.enemies[e].stun == 2.0 && w.enemies[far].stun == 0.0 && w.players[pi].cooldowns[1] == 10.0,
        "Shield Bash in 2u: Stun {}/{}, CD {:?}",
        w.enemies[e].stun,
        w.enemies[far].stun,
        w.players[pi].cooldowns
    );
    w.players[pi].x = w.hub_x + 40.0; // aus der Reichweite: der Gegner läuft nur
    let x = w.enemies[e].x;
    for _ in 0..59 {
        step_enemies(&mut w, None, DT);
    }
    assert!(
        w.enemies[e].x == x && w.enemies[e].stun > 0.0,
        "betäubter Gegner bewegt sich: {x} → {}, Stun {}",
        w.enemies[e].x,
        w.enemies[e].stun
    );
    for _ in 0..5 {
        step_enemies(&mut w, None, DT);
    }
    assert!(
        w.enemies[e].x != x && w.enemies[e].stun == 0.0,
        "nach 2 s weiter regungslos: X {}, Stun {}",
        w.enemies[e].x,
        w.enemies[e].stun
    );
}

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x, hit_event)"]
fn tank_iron_wall() {
    let mut w = quiet_world();
    let pi = tank_player(&mut w);
    skills_tick(&mut w, pi, &skill_cmd(3));
    let p = &w.players[pi];
    assert!(
        p.shield == 500.0 && p.shield_for == 10.0 && p.cooldowns[2] == 30.0,
        "Iron Wall: Schild {} für {} s, CD {:?}",
        p.shield,
        p.shield_for,
        p.cooldowns
    );
    probe(&mut w, &[pi]);
    let id = w.players[pi].id;
    apply_damage(&mut w, id, 300.0); // der Schild fängt den Treffer ohne Goldverlust ab
    let p = &w.players[pi];
    assert!(
        p.shield == 200.0 && p.gold == PROBE_GOLD && p.crown.is_none(),
        "Schild {}, Gold {}",
        p.shield,
        p.gold
    );
    apply_damage(&mut w, id, 300.0); // der Rest-Schild nimmt auch den Überschuss
    apply_damage(&mut w, id, 300.0); // Schild leer: der Treffer kostet eine Münze
    let want = PROBE_GOLD - test_values().monarch.hit_coins;
    let p = &w.players[pi];
    assert!(
        p.shield == 0.0 && p.gold == want,
        "Schild {}, Gold {}",
        p.shield,
        p.gold
    );
    let mut w2 = quiet_world();
    let q = tank_player(&mut w2);
    skills_tick(&mut w2, q, &skill_cmd(3));
    step_idle(&mut w2, q, 301); // 10 s
    let q = &w2.players[q];
    assert!(
        q.shield == 0.0 && q.shield_for == 0.0,
        "Schild nach 10 s: {} / {}",
        q.shield,
        q.shield_for
    );
}

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x, hit_event)"]
fn tank_last_stand() {
    let mut w = quiet_world();
    let pi = tank_player(&mut w);
    skills_tick(&mut w, pi, &skill_cmd(4));
    let p = &w.players[pi];
    assert!(
        p.last_stand_for == 10.0 && p.cooldowns[3] == 120.0,
        "Last Stand: {} s, CD {:?}",
        p.last_stand_for,
        p.cooldowns
    );
    let hit_coins = test_values().monarch.hit_coins;
    w.players[pi].gold = hit_coins; // die letzte Münze
    let id = w.players[pi].id;
    apply_damage(&mut w, id, 1000.0);
    let p = &w.players[pi];
    assert!(
        p.crown.is_none() && p.gold == hit_coins && p.last_stand_for == 0.0,
        "Last Stand: Krone {:?}, Gold {}, Last Stand {}",
        p.crown,
        p.gold,
        p.last_stand_for
    );
    apply_damage(&mut w, id, 1000.0); // ohne Last Stand kostet der Treffer die letzte Münze
    apply_damage(&mut w, id, 1000.0); // ohne Gold fällt die Krone
    let p = &w.players[pi];
    assert!(p.crown.is_some(), "nach Ablauf bleibt die Krone: Gold {}", p.gold);
}

/// `TestTankSchildNurGroesser` (B-220/AC-01): Divine Shield (100) ersetzt einen laufenden Schild nur, wenn er größer ist; die
/// Abklingzeit startet trotzdem.
#[test]
fn tank_schild_nur_groesser() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    let divine = test_values().monarch.skills["divineShield"].effect.clone();
    (w.players[pi].shield, w.players[pi].shield_for) = (400.0, 5.0);
    let ok = cast_shield(&mut w, pi, &divine);
    let p = &w.players[pi];
    assert!(
        ok && p.shield == 400.0 && p.shield_for == 5.0,
        "laufender 400er-Schild: {} für {} s",
        p.shield,
        p.shield_for
    );
    w.players[pi].shield = 50.0;
    let ok = cast_shield(&mut w, pi, &divine);
    let p = &w.players[pi];
    assert!(
        ok && p.shield == 100.0 && p.shield_for == 10.0,
        "Rest 50: {} für {} s",
        p.shield,
        p.shield_for
    );
}

/// Zwei Tanks: Taunt, Stun und Last Stand wirken je Besitzer; ein Spieler verspottet nur im eigenen Radius.
#[test]
fn own_taunt_stun_last_stand_je_besitzer_fuer_zwei_spieler() {
    let mut w = quiet_world();
    let (p0, p1) = (add_player(&mut w), add_player(&mut w));
    (w.players[p0].x, w.players[p1].x) = (100.0, 200.0);
    let (a, b) = (spawn(&mut w, "skeleton", 105.0), spawn(&mut w, "skeleton", 205.0));
    let taunt = SkillEffect {
        radius: 10.0,
        duration: 4.0,
        ..SkillEffect::default()
    };
    assert!(cast_taunt(&mut w, p1, &taunt));
    let (id0, id1) = (w.players[p0].id, w.players[p1].id);
    assert!(w.enemies[b].taunt_id == id1 && w.enemies[b].taunt_for == 4.0);
    assert!(
        w.enemies[a].taunt_id == 0 && w.enemies[a].taunt_for == 0.0,
        "Gegner um Spieler 0 bleibt unverspottet"
    );
    assert!(cast_taunt(&mut w, p0, &taunt) && w.enemies[a].taunt_id == id0);
    let stun = SkillEffect {
        range: 6.0,
        duration: 2.0,
        ..SkillEffect::default()
    };
    assert!(cast_stun(&mut w, p0, &stun) && w.enemies[a].stun == 2.0 && w.enemies[b].stun == 0.0);
    w.enemies[a].x = 150.0;
    assert!(!cast_stun(&mut w, p0, &stun), "ohne Gegner in Range keine Wirkung");
    let last = SkillEffect {
        duration: 10.0,
        ..SkillEffect::default()
    };
    assert!(cast_last_stand(&mut w, p1, &last));
    assert!(w.players[p1].last_stand_for == 10.0 && w.players[p0].last_stand_for == 0.0);
}
