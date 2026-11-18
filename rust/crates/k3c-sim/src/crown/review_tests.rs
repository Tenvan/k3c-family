//! Port von `engine/sim/crown_review_test.go` (Review-Fixes KS3). Tests mit `Step` (RU14), `applyDamage` (RU18) oder
//! Inseln (RU10) tragen `ignore`.

use super::testutil::{crown_world, fell};
use super::{CROWN_GROUND, step_crowns, take_crown_or_loot};
use crate::enemies::testutil::spawn;

#[test]
#[ignore = "wartet auf RU19 (hit_event: apply_damage meldet jeden Treffer)"]
fn fortified_aura_fuer_truppen() {
    use crate::enemies::testutil::{add_player, quiet_world};
    use crate::island::testutil::{add_pool_points, must_learn};
    use crate::skills::testutil::troop_at;
    use crate::state::common::apply_damage;
    let mut w = quiet_world();
    let p0 = add_player(&mut w);
    add_player(&mut w);
    add_pool_points(&mut w, 3);
    let x = w.hub_x + 40.0;
    w.players[p0].x = x;
    must_learn(&mut w, p0, &["taunt", "shieldBash", "fortified"]);
    let e = crate::world::vals(&w).monarch.skills["fortified"].effect.clone();
    let (inside, out) = (
        troop_at(&mut w, x + e.aura, 100.0),
        troop_at(&mut w, x + e.aura + 1.0, 100.0),
    );
    let (iid, oid) = (w.troops[inside].id, w.troops[out].id);
    apply_damage(&mut w, iid, 10.0);
    apply_damage(&mut w, oid, 10.0);
    assert!(
        w.troops[inside].hp == 100.0 - (10.0 - e.defense) && w.troops[out].hp == 90.0,
        "Fortified: in der Aura {}, außerhalb {}",
        w.troops[inside].hp,
        w.troops[out].hp
    );
}

/// Go: `TestKroneKeinRaubInGeschuetzterNacht` ohne `Island`: Die Insel ist nur der Bezug `IslandRef` mit dem Grad `easy` (die
/// ersten Nächte sind geschützt, `protectedNights`).
#[test]
fn kein_raub_in_geschuetzter_nacht() {
    use crate::state::IslandRef;
    let (mut w, v) = crown_world();
    w.cycle.phase = "night".into();
    w.cycle.day = 1;
    w.portals = vec![w.hub_x + 90.0];
    fell(&mut w, 0);
    let cx = w.hub_x + 30.0;
    w.players[0].x = w.hub_x;
    if let Some(c) = w.players[0].crown.as_mut() {
        c.x = cx;
    }
    let e = spawn(&mut w, "goblin", cx);
    let mut isl = IslandRef::default();
    isl.grade.protected_nights = 2;
    take_crown_or_loot(&mut w, &v, e, Some(&isl));
    assert!(
        w.enemies[e].crown.is_none() && w.players[0].crown.as_ref().is_some_and(|c| c.state == CROWN_GROUND),
        "Raub in geschützter Nacht"
    );
}

#[test]
#[ignore = "wartet auf RU19 (emit)"]
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
#[ignore = "wartet auf RU19 (emit)"]
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
#[ignore = "wartet auf RU19 (hit_event: apply_damage meldet jeden Treffer)"]
fn last_stand_nur_mit_letzter_muenze() {
    use crate::enemies::testutil::quiet_world;
    use crate::skills::testutil::tank_player;
    use crate::state::common::apply_damage;
    let mut w = quiet_world();
    let pi = tank_player(&mut w);
    (w.players[pi].last_stand_for, w.players[pi].gold) = (10.0, 0);
    let id = w.players[pi].id;
    apply_damage(&mut w, id, 5.0);
    assert!(
        w.players[pi].crown.is_some(),
        "ohne Gold fällt die Krone auch mit Last Stand"
    );
}
