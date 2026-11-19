//! Tests zum Fest im Hub (RU62.13, B-1002): freie Bürger tanzen, Truppen jubeln, Arbeit und Bewegung bleiben.

use super::*;
use crate::actors::anim::{ANIM_RUN, Want};

fn guest(kind: &'static str, id: i64, x: f64, hub_x: f64, working: bool, now: f64) -> Guest<'static> {
    Guest {
        kind,
        id,
        x,
        hub_x,
        working,
        now,
    }
}

fn idle() -> Want {
    Want {
        anim: ANIM_IDLE.to_string(),
        ..Want::default()
    }
}

#[test]
fn freie_buerger_im_hub_tanzen_gespiegelt_und_versetzt() {
    let (w, d) = festival_want(guest("peasant", 1, 10.0, 12.0, false, 0.0), idle(), 0);
    assert_eq!(w.anim, ANIM_ATTACK, "Tanz-Pose");
    assert!(d == 1 || d == -1);
    // im Lauf der Zeit wechselt die Richtung
    let dirs: Vec<i64> = (0..40)
        .map(|i| festival_want(guest("vagrant", 1, 10.0, 12.0, false, f64::from(i) * 0.1), idle(), 0).1)
        .collect();
    assert!(dirs.contains(&1) && dirs.contains(&-1), "gespiegelt: {dirs:?}");
    // versetzt: zwei Figuren stehen nicht immer gleich
    let differ = (0..40).any(|i| mirror(f64::from(i) * 0.1, DANCE_HZ, 1) != mirror(f64::from(i) * 0.1, DANCE_HZ, 5));
    assert!(differ, "Versatz je Figur");
}

#[test]
fn arbeit_bewegung_und_ferne_buerger_bleiben() {
    let run = Want {
        anim: ANIM_RUN.to_string(),
        ..Want::default()
    };
    assert_eq!(
        festival_want(guest("peasant", 1, 10.0, 12.0, true, 0.0), idle(), 0),
        (idle(), 0),
        "Arbeit"
    );
    assert_eq!(
        festival_want(guest("peasant", 1, 10.0, 12.0, false, 0.0), run.clone(), 1),
        (run, 1),
        "Lauf"
    );
    assert_eq!(
        festival_want(guest("peasant", 1, 100.0, 12.0, false, 0.0), idle(), 0),
        (idle(), 0),
        "weit vom Hub"
    );
}

#[test]
fn truppen_jubeln_in_der_ruhe_pose() {
    let (w, d) = festival_want(guest("warrior", 3, 40.0, 12.0, false, 0.0), idle(), 0);
    assert_eq!(w.anim, ANIM_IDLE, "Ruhe-Pose bleibt");
    assert!(d == 1 || d == -1, "gespiegelt");
}
