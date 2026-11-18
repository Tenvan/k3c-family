//! Tests zu `repair.rs` (Go: `building_repair_test.go`, W1.3, B-112/AC-04, AC-05, AC-06, Sprint W1 AC-03, AC-04): Zerstörung
//! setzt die Stufe zurück, Reparatur ohne Gold und Material zwischen den Wellen, Spielstand mit Hub- und Platz-Stufen. Die
//! Tests über `step`, `create_world` und `level_of` warten auf RU18.
#![allow(clippy::unimplemented)] // Rumpf des Spielstand-Tests: wartet auf RU18 (Insel-Spielstand mit Zahlplatz)

use super::repair::{repair, repair_job};
use super::testutil::*;
use crate::hub::level_of;
use k3c_data::hub::Stock;

use crate::state::World;
use crate::state::common::apply_damage;
use crate::world::vals;

/// Mauer Stufe 2 zerstört: unpaid, Stufe 1, MaxHP der Stufe 1, gleiches X; Hub-Stufe und äußere Linie bleiben.
#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprueft: not implemented: RU18: protected_night)"]
fn zerstoerung_setzt_stufe_zurueck() {
    let mut w = day_world();
    w.hub_level = 3;
    let (inner, outer) = (line_site(&mut w, "wall", -1, 1), line_site(&mut w, "wall", -1, 2));
    built(&mut w.sites[inner]);
    built(&mut w.sites[outer]);
    let levels = vals(&w).buildings["wall"].levels.clone();
    let s = &mut w.sites[inner];
    s.level = 2;
    s.max_hp = levels[1].hp;
    s.hp = s.max_hp;
    s.upgrade = "waitingMaterial".into();
    s.upgrade_paid = 10;
    let (x, id, hp) = (s.x, s.id, s.hp);
    apply_damage(&mut w, id, hp);
    let s = &w.sites[inner];
    assert!(
        s.state == "unpaid" && level_of(&w, s) == 1 && s.max_hp == levels[0].hp && s.x == x,
        "nach Zerstörung: {s:?}"
    );
    assert!(
        s.upgrade.is_empty() && s.upgrade_paid == 0,
        "laufender Ausbau nicht verloren: {s:?}"
    );
    assert!(
        w.hub_level == 3 && w.sites[outer].state == "built",
        "Hub-Stufe {}, äußere Mauer {}",
        w.hub_level,
        w.sites[outer].state
    );
}

/// damagedTower ist ein gebauter Turm mit halben HP in einer Welt mit zwei Spielern.
fn damaged_tower() -> (World, usize) {
    let mut w = day_world();
    add_player(&mut w);
    add_player(&mut w);
    let tower = line_site(&mut w, "tower", 1, 1);
    built(&mut w.sites[tower]);
    w.sites[tower].hp = w.sites[tower].max_hp / 2.0;
    w.troops[0].x = w.sites[tower].x;
    (w, tower)
}

/// Reparatur zwischen den Wellen: HP voll nach dem fehlenden Anteil der Bauzeit, Vorrat und Gold unverändert.
#[test]
fn reparatur_ohne_gold_und_material() {
    let (mut w, tower) = damaged_tower();
    w.stock = Stock {
        wood: 9,
        stone: 4,
        ..Stock::default()
    };
    let (stock, gold) = (w.stock.clone(), [w.players[0].gold, w.players[1].gold]);
    let bs = vals(&w).buildings["tower"].build_seconds;
    let done = |w: &World| w.sites[tower].hp >= w.sites[tower].max_hp;
    let took = run_until(&mut w, 3.0 * bs, &done);
    let half = bs / 2.0;
    assert!(
        took >= half - DT && took <= half + 0.5,
        "Reparatur dauerte {took:.2} s, erwartet die halbe Bauzeit {half} s"
    );
    assert!(
        w.stock == stock && w.players[0].gold == gold[0] && w.players[1].gold == gold[1],
        "Reparatur kostet: Vorrat {stock:?} → {:?}",
        w.stock
    );
}

/// Bei Gefahr (Gegner in der Welt, Nacht) keine Reparatur.
#[test]
fn reparatur_nicht_bei_gefahr() {
    let (mut w, tower) = damaged_tower();
    let x = w.width_units - 1.0;
    let e = spawn(&mut w, "goblin", x);
    w.enemies[e].stun = f64::INFINITY;
    let before = w.sites[tower].hp;
    let bs = vals(&w).buildings["tower"].build_seconds;
    run(&mut w, bs);
    assert_eq!(w.sites[tower].hp, before, "mit Gegnern repariert");
    w.enemies.clear();
    w.cycle_speed = 1.0;
    let dn = crate::world::base::base().expect("Daten").day_night.clone();
    w.time = (dn.day_minutes + dn.twilight_minutes) * 60.0;
    run(&mut w, 1.0);
    assert!(
        w.cycle.phase == "night" && w.sites[tower].hp == before,
        "nachts repariert (Phase {})",
        w.cycle.phase
    );
}

/// Hub-Stufe, Mauer-Stufe (auch auf einer äußeren Linie) und laufender Ausbau überleben Speichern und Laden.
#[test]
fn spielstand_hub_und_platz_stufen() {
    use crate::enemies::testutil::line_site;
    use crate::island::testutil::{load_island, must_island, save_json, test_data};
    use crate::world::testutil::built;
    let data = test_data();
    let mut isl = must_island(&data, "stufen", &[0, 1]);
    let a = &mut isl.stages[0];
    a.hub_level = 3;
    let oi = line_site(a, "wall", 1, 2);
    built(&mut a.sites[oi]);
    let max_hp = crate::world::vals(a).buildings["wall"].levels[2].hp;
    let o = &mut a.sites[oi];
    (o.level, o.max_hp, o.hp) = (3, max_hp, 700.0);
    (o.upgrade, o.upgrade_paid) = ("waitingMaterial".into(), 20);
    if let Some(h) = a.hub_site.as_mut() {
        h.upgrade_paid = 30;
    }
    let mut loaded = load_island(&data, &save_json(&isl, "2026-10-05T12:00:00Z"));
    let loi = line_site(&mut loaded.stages[0], "wall", 1, 2);
    let la = &loaded.stages[0];
    let lo = &la.sites[loi];
    assert!(
        la.hub_level == 3 && loaded.stages[1].hub_level == 1,
        "Hub-Stufen nach dem Laden {}/{}",
        la.hub_level,
        loaded.stages[1].hub_level
    );
    assert!(
        lo.level == 3
            && lo.max_hp == max_hp
            && lo.hp == 700.0
            && lo.upgrade == "waitingMaterial"
            && lo.upgrade_paid == 20,
        "äußere Mauer nach dem Laden: {lo:?}"
    );
    assert_eq!(
        la.hub_site.as_ref().map(|h| h.upgrade_paid),
        Some(30),
        "Hub-Ausbau nach dem Laden"
    );
}

/// Ein beschädigtes Hammer-Bauer-Gerüst ohne `create_world`: ein halb zerstörtes Gebäude (Schmiede, ohne Stufen) und ein
/// Bauer mit Hammer am Platz.
fn repair_world() -> (World, usize, usize) {
    let mut w = bare_world();
    let hub = w.hub_x;
    let si = built_site(&mut w, "smithy", hub + 40.0);
    w.sites[si].max_hp = 250.0;
    w.sites[si].hp = 125.0;
    let ti = add_peasant(&mut w, hub + 40.0, "");
    w.troops[ti].hammer = true;
    (w, si, ti)
}

/// Eigener Test (ohne Go-Gegenstück): Der Bauer mit Hammer nimmt das nächste beschädigte Gebäude ohne Gefahr; ohne Hammer,
/// bei Gefahr oder mit lebendem Bauer am Platz gibt es keinen Auftrag.
#[test]
fn repair_job_nur_mit_hammer_ohne_gefahr() {
    let (mut w, si, ti) = repair_world();
    w.troops[ti].hammer = false;
    assert!(repair_job(&mut w, ti).is_none(), "ohne Hammer");
    w.troops[ti].hammer = true;
    w.cycle.phase = "night".into();
    assert!(repair_job(&mut w, ti).is_none(), "bei Gefahr");
    w.cycle.phase = "day".into();
    let j = repair_job(&mut w, ti).expect("Reparatur-Auftrag");
    assert!(j.r#type == "repair" && j.site_id == w.sites[si].id);
    assert_eq!(w.sites[si].worker_id, Some(w.troops[ti].id), "Bauer am Platz");
    let hub = w.hub_x;
    let other = add_peasant(&mut w, hub, "");
    w.troops[other].hammer = true;
    assert!(repair_job(&mut w, other).is_none(), "Platz hat schon einen Bauer");
}

/// Eigener Test (ohne Go-Gegenstück): Die Reparatur braucht den fehlenden Anteil der Bauzeit, kostet nichts und endet mit
/// dem Ereignis `repaired`.
#[test]
fn repair_dauert_den_fehlenden_anteil_der_bauzeit() {
    let (mut w, si, ti) = repair_world();
    w.troops[ti].job = repair_job(&mut w, ti);
    let (stock, v) = (w.stock.clone(), vals(&w));
    let mut took = 0.0;
    while w.troops[ti].job.is_some() && took < 100.0 {
        repair(&mut w, &v, ti, DT);
        took += DT;
    }
    let half = v.buildings["smithy"].build_seconds / 2.0;
    assert!(
        took >= half - DT && took <= half + 0.5,
        "Reparatur dauerte {took:.2} s, erwartet {half} s"
    );
    assert_eq!(w.sites[si].hp, w.sites[si].max_hp);
    assert!(w.sites[si].worker_id.is_none() && w.stock == stock);
    assert!(has_event(&w.events, "repaired"), "kein Ereignis repaired");
    // Ein Platz, der nicht (mehr) beschädigt ist, gibt den Auftrag sofort frei.
    w.troops[ti].job = Some(crate::state::Job {
        r#type: "repair".into(),
        site_id: w.sites[si].id,
        ..Default::default()
    });
    repair(&mut w, &v, ti, DT);
    assert!(w.troops[ti].job.is_none());
}
