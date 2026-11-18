//! `CombatSpotState` (Port von `engine/sim/combat_state_test.go`, B-939): Preis, Zahlstand und Zahlbarkeit je Zahlstelle
//! und Monarch, mit 2 Spielern.

use super::data::COMBAT_GROUPS;
use super::testutil::{combat_stage, spawn_soldiers, spot_for, vals};
use super::{SpotKind, combat_cost};

#[test]
fn combat_spot_state_je_monarch() {
    let (mut w, _, right) = combat_stage(COMBAT_GROUPS);
    spawn_soldiers(&mut w, 1);
    let v = vals(&w);
    let shield = spot_for(&w, SpotKind::Shield, 0).expect("Schild");
    let st = shield.state_for(&w, &v, &w.players[0]);
    assert!(
        st.price == combat_cost(&v, SpotKind::Shield) && st.paid == 0 && st.payable,
        "Schild vor Zahlung: {st:?}"
    );
    w.players[0].x = shield.x;
    w.players[1].x = right;
    assert!(
        shield.state_for(&w, &v, &w.players[0]).near && !shield.state_for(&w, &v, &w.players[1]).near,
        "Near folgt der Position des Monarchen nicht"
    );
    w.combat.paid.insert(shield.key(&w.players[0]), 2);
    assert_eq!(
        shield.state_for(&w, &v, &w.players[1]).paid,
        2,
        "Schild-Zahlstand gilt für alle Monarchen"
    );
    let trig = spot_for(&w, SpotKind::Trigger, 1).expect("Auslöser");
    assert!(
        !trig.state_for(&w, &v, &w.players[0]).payable,
        "Auslöser ohne Gruppe darf nicht zahlbar sein"
    );
}

#[test]
fn zahlstand_naehe_ohne_zahlbarkeit() {
    // `near_of` ist rein: Zahl-Reichweite plus eine Unit Anzeige-Spielraum.
    let (mut w, _, right) = combat_stage(COMBAT_GROUPS);
    let v = vals(&w);
    let shield = spot_for(&w, SpotKind::Shield, 0).expect("Schild");
    w.players[0].x = shield.x + v.economy.pay_range_units + 1.0;
    w.players[1].x = right;
    assert!(shield.near_of(&v, &w.players[0]));
    w.players[0].x += 0.01;
    assert!(!shield.near_of(&v, &w.players[0]));
    assert!(!shield.near_of(&v, &w.players[1]));
}
