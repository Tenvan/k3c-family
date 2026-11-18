//! Port von `engine/sim/crown_review_test.go` (Review-Fixes KS3). Tests mit `Step` (RU14), `applyDamage` (RU18) oder
//! Inseln (RU10) tragen `ignore`.

use super::testutil::{crown_world, fell};
use super::{CROWN_GROUND, step_crowns, take_crown_or_loot};
use crate::enemies::testutil::spawn;

#[test]
#[ignore = "wartet auf RU18 (applyDamage, Fortified-Aura)"]
fn fortified_aura_fuer_truppen() {}

#[test]
#[ignore = "wartet auf RU10 (Island, geschützte Nacht über Grad easy)"]
fn kein_raub_in_geschuetzter_nacht() {}

#[test]
fn ohne_insel_gibt_es_keine_geschuetzte_nacht() {
    // Gegenprobe zum ignorierten Insel-Test: ohne Insel (None) gilt der Raub.
    let (mut w, v) = crown_world();
    fell(&mut w, 0);
    let ex = w.players[0].x;
    let e = spawn(&mut w, "goblin", ex);
    take_crown_or_loot(&mut w, &v, e, None);
    assert!(w.enemies[e].crown.is_some());
}

#[test]
fn freier_monarch_bekommt_die_krone_zurueck() {
    let (mut w, v) = crown_world();
    fell(&mut w, 0);
    w.players[0].x -= 20.0;
    w.players[0].free = true;
    let cx = w.players[0].crown.as_ref().map_or(0.0, |c| c.x);
    spawn(&mut w, "goblin", cx);
    step_crowns(&mut w, &v, 1.0 / 30.0);
    assert!(w.players[0].crown.is_none(), "{:?}", w.players[0].crown);
}

#[test]
fn krone_eines_freien_monarchen_ist_nicht_raubbar() {
    let (mut w, v) = crown_world();
    fell(&mut w, 0);
    w.players[0].free = true;
    let ex = w.players[0].x;
    let e = spawn(&mut w, "goblin", ex);
    take_crown_or_loot(&mut w, &v, e, None);
    let c = w.players[0].crown.as_ref().expect("Krone");
    assert_eq!(c.state, CROWN_GROUND);
}

#[test]
#[ignore = "wartet auf RU18 (applyDamage, Last Stand)"]
fn last_stand_nur_mit_letzter_muenze() {}
