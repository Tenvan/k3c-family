//! Zwei Linien gleichzeitig (Go: `skills_linien_test.go`): Tank und Zauberer zweier Spieler in einer Welt (SK1.2, B-007). Der
//! erste Test rechnet `step` (`pure`) und meldet Skills über `emit` (RU19) und wartet darauf.

use super::testutil::*;
use crate::enemies::testutil::{add_player, quiet_world};
use crate::island::testutil::{add_pool_points, available_points, must_learn};
use crate::monarch::learn_skill;
use crate::state::PlayerCommand;
use crate::world::testutil::DT;
use crate::world::{step, vals};

#[test]
#[ignore = "wartet auf RU19 (pure, emit, unit_x, hit_event)"]
fn skills_tank_und_zauberer_im_selben_tick() {
    let mut w = quiet_world();
    let (tank, mage) = (add_player(&mut w), add_player(&mut w));
    add_pool_points(&mut w, 1);
    must_learn(&mut w, tank, &["taunt"]);
    must_learn(&mut w, mage, &["fireball"]);
    // weit genug auseinander: jede Wirkung trifft nur den eigenen Gegner
    (w.players[tank].x, w.players[mage].x) = (w.hub_x + 40.0, w.hub_x + 80.0);
    let (tx, mx) = (w.players[tank].x, w.players[mage].x);
    let (near_tank, near_mage) = (tough_enemy(&mut w, tx + 5.0), tough_enemy(&mut w, mx + 5.0));
    let cmd = PlayerCommand {
        skill: 1,
        ..PlayerCommand::default()
    };

    step(&mut w, &[cmd.clone(), cmd], None, DT);

    let tank_id = w.players[tank].id;
    let nt = &w.enemies[near_tank];
    assert!(
        nt.taunt_id == tank_id && nt.taunt_for > 0.0 && lost(&w, near_tank) == 0.0,
        "Taunt: Gegner beim Tank {}/{}, verlorene HP {}",
        nt.taunt_id,
        nt.taunt_for,
        lost(&w, near_tank)
    );
    assert!(
        lost(&w, near_mage) > 0.0 && w.enemies[near_mage].taunt_id == 0,
        "Fireball: Gegner beim Zauberer verlor {} HP, TauntID {}",
        lost(&w, near_mage),
        w.enemies[near_mage].taunt_id
    );
    let (t, m) = (&w.players[tank], &w.players[mage]);
    assert!(
        t.cooldowns == [15.0, 0.0, 0.0, 0.0] && m.cooldowns == [8.0, 0.0, 0.0, 0.0],
        "Abklingzeiten nicht je Spieler: Tank {:?}, Zauberer {:?}",
        t.cooldowns,
        m.cooldowns
    );
    assert!(
        t.slots == ["taunt"] && m.slots == ["fireball"],
        "Slots: Tank {:?}, Zauberer {:?}",
        t.slots,
        m.slots
    );
}

#[test]
fn skills_fremde_linie_ohne_punkte_abgelehnt() {
    let mut w = quiet_world();
    let (tank, mage) = (add_player(&mut w), add_player(&mut w));
    let v = vals(&w);
    add_pool_points(&mut w, 1);
    must_learn(&mut w, tank, &["taunt"]);
    must_learn(&mut w, mage, &["fireball"]);
    let free = available_points(&w, mage);
    for id in ["taunt", "iceWall"] {
        // Pool leer, danach (nach Punkt) Tier fehlt
        assert!(
            learn_skill(&mut w, &v, mage, id).is_err(),
            "{id} ohne freien Punkt gelernt"
        );
    }
    add_pool_points(&mut w, 5);
    assert!(
        learn_skill(&mut w, &v, mage, "lightningStorm").is_err(),
        "Tier 3 des Zauberers mit 1 Skill der Linie gelernt"
    );
    assert!(
        w.players[mage].skills == ["fireball"] && available_points(&w, mage) == free + 5,
        "Zustand nach Ablehnung: Skills {:?}, frei {}",
        w.players[mage].skills,
        available_points(&w, mage)
    );
    assert!(
        w.players[tank].skills == ["taunt"],
        "Tank verändert: {:?}",
        w.players[tank].skills
    );
}
