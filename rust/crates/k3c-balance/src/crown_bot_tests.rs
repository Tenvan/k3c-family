//! Tests zu `crown_bot.rs` (Port von `engine/balance/crown_bot_test.go`); `heal_monarchs` ist in `helps.rs`, sein Test
//! stand in Go in derselben Datei.

use k3c_sim::state::{Crown, PlayerCommand};

use crate::crown_bot::retrieve_crown;
use crate::helps::heal_monarchs;
use crate::testutil::scenario_island;

/// Ein Bot ohne Krone läuft zur liegenden Krone, ein Träger zum Besitzer (B-931), sonst gilt das Profil.
#[test]
fn bot_hebt_krone_auf() {
    let mut isl = scenario_island("1", 2);
    let w = &mut isl.stages[0];
    let hub_x = w.hub_x;
    w.players[0].x = hub_x;
    w.players[1].x = hub_x + 20.0;
    let idle = PlayerCommand {
        skill: 2,
        ..PlayerCommand::default()
    };
    let got = retrieve_crown(w, &w.players[0].clone(), idle.clone());
    assert_eq!(got, idle, "mit Krone gilt das Profil: {got:?}");

    w.players[0].crown = Some(Crown {
        owner: 0,
        state: "ground".into(),
        x: hub_x - 10.0,
        ..Crown::default()
    });
    let got = retrieve_crown(w, &w.players[0].clone(), idle.clone());
    assert!(got.move_x < 0.0, "Besitzer läuft nicht zur Krone: {got:?}");
    let got = retrieve_crown(w, &w.players[1].clone(), idle.clone());
    assert!(got.move_x < 0.0, "Mitspieler läuft nicht zur fremden Krone: {got:?}");

    let carrier = w.players[1].id;
    if let Some(c) = w.players[0].crown.as_mut() {
        c.state = "player".into();
        c.carrier = carrier;
    }
    let got = retrieve_crown(w, &w.players[1].clone(), idle);
    assert!(got.move_x < 0.0, "Träger läuft nicht zum Besitzer: {got:?}");
}

/// `heal_monarchs` lässt eine vom Gegner getragene Krone dort.
#[test]
fn help_heal_monarchs_krone_beim_gegner() {
    let mut isl = scenario_island("1", 2);
    let w = &mut isl.stages[0];
    w.players[0].crown = Some(Crown {
        state: "enemy".into(),
        ..Crown::default()
    });
    w.players[1].crown = Some(Crown {
        state: "ground".into(),
        ..Crown::default()
    });
    heal_monarchs(&mut isl);
    let (p0, p1) = (&isl.stages[0].players[0], &isl.stages[0].players[1]);
    assert!(
        p0.crown.is_some() && p1.crown.is_none(),
        "Gegner-Krone {:?}, liegende Krone {:?}",
        p0.crown,
        p1.crown
    );
}
