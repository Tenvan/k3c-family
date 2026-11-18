//! Tests zu den Zahlstellen am Tor (Port von `engine/sim/combat_gate_test.go`, KS7, B-941, B-942): Auslöser und Banner vor
//! dem äußersten Tor, Schilde erst ab Hub-Stufe, Gestell mit Stapel; mit 2 Spielern.

use super::data::{COMBAT_GENERAL, COMBAT_GROUPS};
use super::testutil::{combat_stage, spawn_soldiers, spot_for, vals};
use super::{SpotKind, combat_payable, shield_rack};
use crate::state::Site;

#[test]
fn ausloeser_und_banner_liegen_ausserhalb_des_tors() {
    for (mode, kind) in [(COMBAT_GROUPS, SpotKind::Trigger), (COMBAT_GENERAL, SpotKind::Banner)] {
        let (w, left, right) = combat_stage(mode);
        let out = vals(&w).economy.combat.gate_out_units;
        assert!(out > 0.0, "gateOutUnits {out}");
        let r = spot_for(&w, kind, 1).expect("rechts");
        assert!(
            (r.x - (right + out)).abs() <= 1e-9,
            "{mode} rechts: X {}, erwartet {}",
            r.x,
            right + out
        );
        let l = spot_for(&w, kind, -1).expect("links");
        assert!(
            (l.x - (left - out)).abs() <= 1e-9,
            "{mode} links: X {}, erwartet {}",
            l.x,
            left - out
        );
    }
}

#[test]
fn zahlstelle_weicht_bauplaetzen_aus() {
    let (mut w, _, right) = combat_stage(COMBAT_GROUPS);
    let v = vals(&w);
    let c = &v.economy.combat;
    let id = w.new_id();
    let bx = right + c.gate_out_units + 0.5;
    w.sites.push(Site {
        id,
        kind: "tower".into(),
        x: bx,
        ..Site::default()
    });
    let got = spot_for(&w, SpotKind::Trigger, 1).expect("rechts").x;
    let want = bx + c.spot_clear_units;
    assert!(
        (got - want).abs() <= 1e-9,
        "Auslöser bei {got}, erwartet hinter dem Bauplatz bei {want}"
    );
    for s in w.sites.iter().filter(|s| s.id != id && s.kind != "wall") {
        assert!(
            (s.x - got).abs() >= c.spot_clear_units,
            "Auslöser liegt auf Platz {} bei {}",
            s.kind,
            s.x
        );
    }
    let left = spot_for(&w, SpotKind::Trigger, -1).expect("links").x;
    assert!(
        (left - (w.hub_x - 10.0 - c.gate_out_units)).abs() <= 1e-9,
        "linke Seite unberührt, X {left}"
    );
}

#[test]
fn schild_erst_ab_hub_stufe_mit_gestell() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let min = vals(&w).economy.combat.shield_min_hub_level;
    assert!(min >= 2, "shieldMinHubLevel {min}");
    w.hub_level = min - 1;
    assert!(
        spot_for(&w, SpotKind::Shield, 0).is_none(),
        "unter der Stufe keine Schild-Zahlstelle"
    );
    assert!(
        !shield_rack(&w, &vals(&w)).2,
        "Gestell darf unter der Stufe nicht sichtbar sein"
    );
    w.hub_level = min;
    let spot = spot_for(&w, SpotKind::Shield, 0).expect("ab der Stufe gibt es die Schild-Zahlstelle");
    let (x, n, vis) = shield_rack(&w, &vals(&w));
    assert!(vis && n == 0 && x == spot.x, "Gestell leer: x {x} n {n} sichtbar {vis}");
}

#[test]
fn gestell_stapel_begrenzt_und_zahlt_nicht_ueber_max() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    spawn_soldiers(&mut w, 8);
    let v = vals(&w);
    let max = v.economy.combat.shield_stack_max;
    w.combat.shields = max + 2;
    assert_eq!(shield_rack(&w, &v).1, max, "Stapel höchstens {max}");
    let spot = spot_for(&w, SpotKind::Shield, 0).expect("Schild");
    for i in 0..w.players.len() {
        // beide Spieler: bei vollem Gestell nimmt die Zahlstelle nichts
        w.players[i].gold = 40;
        assert!(
            !combat_payable(&w, &v, &w.players[i], &spot),
            "Spieler {i}: Schild zahlbar bei vollem Gestell"
        );
    }
    w.combat.shields = 2;
    assert_eq!(shield_rack(&w, &v).1, 2, "Stapel 2");
    assert!(
        combat_payable(&w, &v, &w.players[1], &spot),
        "Spieler 2 kann bei freiem Platz zahlen"
    );
}

// Review KS7: Ein Tor nahe dem Levelrand legt das Zahlziel nicht hinter den Rand.
#[test]
fn zahlstelle_bleibt_im_level() {
    let (mut w, _, right) = combat_stage(COMBAT_GROUPS);
    w.width_units = right + 1.0;
    let x = spot_for(&w, SpotKind::Trigger, 1).expect("rechts").x;
    assert!(
        x <= w.width_units && x >= 0.0,
        "Auslöser bei {x} hinter dem Rand {}",
        w.width_units
    );
}
