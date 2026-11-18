//! Skill-Rahmen (Go: `skills_test.go`): Slot-Befehl, Abklingzeit je Spieler, nicht gelernt = nichts, Gating (S1.2a, B-119).
//! Wo ein Skill wirkt, meldet er `skillCast` über `emit` und `unit_x` (events.go): diese Tests warten auf RU19, ebenso jeder
//! Test über `step` (`pure`). Die Tests ab `own_` haben kein Go-Gegenstück und prüfen die Teile ohne diese Funktionen.

use super::testutil::*;
use crate::enemies::testutil::{add_player, quiet_world, spawn};
use crate::island::testutil::{add_pool_points, must_learn};
use crate::monarch::learn_skill;
use crate::state::PlayerCommand;
use crate::world::{step, vals};

#[test]
fn skills_leer_oder_nicht_gelernt() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    add_pool_points(&mut w, 3);
    let px = w.players[pi].x;
    let ei = spawn(&mut w, "skeleton", px + 1.0);
    for slot in 1..=4 {
        // nichts gelernt, alle Slots leer
        skills_tick(&mut w, pi, &skill_cmd(slot));
    }
    w.players[pi].slots = vec!["taunt".into(), "shieldBash".into()]; // im Slot, aber nicht gelernt
    skills_tick(&mut w, pi, &skill_cmd(1));
    skills_tick(&mut w, pi, &skill_cmd(2));
    let (p, e) = (&w.players[pi], &w.enemies[ei]);
    assert!(
        p.cooldowns.is_empty() && e.taunt_for == 0.0 && e.stun == 0.0,
        "Wirkung ohne gelernten Skill: CD {:?}, Taunt {}, Stun {}",
        p.cooldowns,
        e.taunt_for,
        e.stun
    );
}

#[test]
fn skills_abklingzeit_blockiert_zweiten_einsatz() {
    let mut w = quiet_world();
    let pi = tank_player(&mut w);
    let px = w.players[pi].x;
    let ei = spawn(&mut w, "skeleton", px + 1.0);
    skills_tick(&mut w, pi, &skill_cmd(2));
    assert!(
        w.enemies[ei].stun == 2.0 && w.players[pi].cooldowns[1] == 10.0,
        "Shield Bash: Stun {}, CD {:?}",
        w.enemies[ei].stun,
        w.players[pi].cooldowns
    );
    w.enemies[ei].stun = 0.0;
    skills_tick(&mut w, pi, &skill_cmd(2));
    step_idle(&mut w, pi, 297); // mit den beiden Befehls-Ticks 299 Ticks < 10 s
    skills_tick(&mut w, pi, &skill_cmd(2));
    assert_eq!(
        w.enemies[ei].stun, 0.0,
        "zweiter Einsatz in der Abklingzeit: CD {}",
        w.players[pi].cooldowns[1]
    );
    step_idle(&mut w, pi, 2);
    skills_tick(&mut w, pi, &skill_cmd(2));
    assert!(
        w.enemies[ei].stun == 2.0 && w.players[pi].cooldowns[1] == 10.0,
        "nach 10 s kein Einsatz: Stun {}, CD {}",
        w.enemies[ei].stun,
        w.players[pi].cooldowns[1]
    );
}

#[test]
fn skills_zwei_spieler_getrennt() {
    let mut w = quiet_world();
    let (p0, p1) = (add_player(&mut w), add_player(&mut w));
    add_pool_points(&mut w, 7);
    for pi in [p0, p1] {
        must_learn(
            &mut w,
            pi,
            &["taunt", "shieldBash", "armorAura", "thickSkin", "ironWall"],
        );
    }
    let cmd = |s| PlayerCommand {
        skill: s,
        ..PlayerCommand::default()
    };
    step(&mut w, &[cmd(3), cmd(0)], None, crate::world::testutil::DT);
    let (a, b) = (&w.players[p0], &w.players[p1]);
    assert!(
        a.shield == 500.0 && a.cooldowns[2] == 30.0 && b.shield == 0.0 && b.cooldowns.is_empty(),
        "Spieler 0 wirkt Iron Wall: P0 {}/{:?}, P1 {}/{:?}",
        a.shield,
        a.cooldowns,
        b.shield,
        b.cooldowns
    );
    step(&mut w, &[cmd(0), cmd(3)], None, crate::world::testutil::DT);
    let (a, b) = (&w.players[p0], &w.players[p1]);
    assert!(
        b.shield == 500.0 && b.cooldowns[2] == 30.0 && a.cooldowns[2] < 30.0,
        "Spieler 1 wirkt Iron Wall: P1 {}/{:?}, P0 CD {:?}",
        b.shield,
        b.cooldowns,
        a.cooldowns
    );
}

#[test]
fn skills_gating_ohne_punkte() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    let v = vals(&w);
    add_pool_points(&mut w, 1);
    assert!(
        learn_skill(&mut w, &v, pi, "ironWall").is_err(),
        "Tier 3 ohne Punkte der Linie gelernt"
    );
    add_pool_points(&mut w, 9);
    must_learn(&mut w, pi, &["taunt"]);
    assert!(
        learn_skill(&mut w, &v, pi, "ironWall").is_err(),
        "Tier 3 mit 1 Tank-Skill gelernt (verlangt 4)"
    );
    w.players[pi].slots.push("ironWall".into()); // erzwungen im Slot: ohne Lernen keine Wirkung
    skills_tick(&mut w, pi, &skill_cmd(2));
    let p = &w.players[pi];
    assert!(
        p.shield == 0.0 && p.cooldowns.is_empty(),
        "Iron Wall ohne Gating: Schild {}, CD {:?}",
        p.shield,
        p.cooldowns
    );
}

#[test]
fn own_abklingzeit_sinkt_je_spieler_und_schild_endet() {
    let mut w = quiet_world();
    let (p0, p1) = (add_player(&mut w), add_player(&mut w));
    w.players[p0].cooldowns = vec![1.0, 0.0, 5.0, 0.0];
    w.players[p1].cooldowns = vec![10.0, 0.0, 0.0, 0.0];
    for pi in [p0, p1] {
        (w.players[pi].shield, w.players[pi].shield_for) = (50.0, 0.01);
        w.players[pi].stealth_for = 1.0;
        w.players[pi].last_stand_for = 1.0;
    }
    skills_tick(&mut w, p0, &skill_cmd(0));
    skills_tick(&mut w, p1, &skill_cmd(0));
    let dt = crate::world::testutil::DT;
    assert!(
        (w.players[p0].cooldowns[2] - (5.0 - dt)).abs() < 1e-9,
        "P0 CD {:?}",
        w.players[p0].cooldowns
    );
    assert!(
        (w.players[p1].cooldowns[0] - (10.0 - dt)).abs() < 1e-9,
        "P1 CD {:?}",
        w.players[p1].cooldowns
    );
    assert_eq!(w.players[p0].cooldowns[1], 0.0, "Abklingzeit sinkt nie unter 0");
    for pi in [p0, p1] {
        let p = &w.players[pi];
        assert!(
            p.shield == 0.0 && p.shield_for == 0.0,
            "Schild endet mit seiner Dauer: {}",
            p.shield
        );
        assert!((p.stealth_for - (1.0 - dt)).abs() < 1e-9 && (p.last_stand_for - (1.0 - dt)).abs() < 1e-9);
    }
}

#[test]
fn own_unbekannter_effekt_wirkt_nicht() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    let e = k3c_data::monarch::SkillEffect {
        r#type: "gibtEsNicht".into(),
        ..Default::default()
    };
    assert_eq!(
        super::cast_effect(&mut w, pi, &e),
        None,
        "unbekannter Effekt = fehlender Map-Eintrag"
    );
    let e = k3c_data::monarch::SkillEffect {
        r#type: "lastStand".into(),
        duration: 3.0,
        ..Default::default()
    };
    assert_eq!(super::cast_effect(&mut w, pi, &e), Some(true));
    assert_eq!(w.players[pi].last_stand_for, 3.0);
}
