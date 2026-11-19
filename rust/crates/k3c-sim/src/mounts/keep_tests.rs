//! Das Reittier bleibt (RU62.8, AC-04): Inselwechsel, Verlust der Krone, kein Frachtplatz.

use super::testutil::{mounts_data, mounts_island, stand_at_gate, tick};
use crate::island::next_island;
use crate::island::switch_testutil::fell;
use crate::state::PlayerCommand;

/// Fährt los: Endboss besiegt, rechnet bis der Wechsel bereit ist.
fn depart(isl: &mut crate::island::Island, data: &k3c_data::Data) {
    isl.endboss_defeated = true;
    for _ in 0..40 {
        crate::island::testutil::run_island(isl, data, &[], 1.0);
        if isl.switch_ready {
            return;
        }
    }
}

#[test]
fn reittier_bleibt_nach_inselwechsel_und_belegt_keinen_frachtplatz() {
    let mut data = mounts_data();
    data.cargo.worlds.push("test".to_string());
    let (_, mut isl, steg) = mounts_island(&data, 2);
    stand_at_gate(&mut isl, 0);
    tick(&mut isl, &data, &[0]);
    assert_eq!(isl.stages[0].players[0].mount, "camel");
    assert_eq!(isl.cargo.taken(), 0, "das Reittier ist keine Fracht");
    for p in &mut isl.stages[0].players {
        p.x = steg;
    }
    depart(&mut isl, &data);
    assert!(isl.switch_ready);
    let old_gate = super::testutil::gate(&isl).x;
    let def = isl.next_def().expect("Ziel");
    let next = next_island(&data, &isl, &def).expect("neue Insel");
    let w = &next.stages[0];
    assert_eq!(
        (w.players[0].mount.as_str(), w.players[1].mount.as_str()),
        ("camel", "")
    );
    assert_eq!(next.cargo.taken(), 0);
    let g = w.mount_gate.clone().expect("Gatter auf der neuen Insel");
    assert_ne!(g.x, old_gate, "andere Insel, anderer Seed, andere Stelle");
    let v = crate::world::vals(w);
    assert!(super::mount_factors(w, &v, 0).1 > super::mount_factors(w, &v, 1).1);
}

#[test]
fn reittier_bleibt_nach_kronenverlust() {
    let data = mounts_data();
    let (_, mut isl, _) = mounts_island(&data, 1);
    stand_at_gate(&mut isl, 0);
    tick(&mut isl, &data, &[0]);
    fell(&mut isl.stages[0], 0);
    let cmds = [PlayerCommand::default()];
    for _ in 0..5 {
        crate::island::step_island(&mut isl, &cmds, &data.difficulty, crate::island::testutil::DT);
    }
    assert_eq!(isl.stages[0].players[0].mount, "camel");
}

#[test]
fn ohne_krone_kein_kauf() {
    let data = mounts_data();
    let (_, mut isl, _) = mounts_island(&data, 1);
    stand_at_gate(&mut isl, 0);
    fell(&mut isl.stages[0], 0);
    stand_at_gate(&mut isl, 0);
    tick(&mut isl, &data, &[0]);
    assert_eq!(isl.stages[0].players[0].mount, "");
    assert_eq!(isl.stages[0].players[0].gold, 100);
}
