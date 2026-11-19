//! Boni des Einsiedlers (RU62.10, AC-05): Dojo (Schaden der Krieger und Schützen mal `damageFactor`, zerstört gleich Grundwert),
//! Schrein (Mauern heilen bei Tagesanbruch um `wallHealFraction`, höchstens bis voll), Zerstörung und Wiederaufbau ohne
//! zweite Zahlung.

use super::bonus::{damage_factor, hall_seconds};
use super::publish;
use super::testutil::{add_deep_player, before_dawn, count, hall, hermit_data, hermit_island, pay_at_hut, run, tick};
use crate::enemies::testutil::{bare_world, spawn, warrior_at};
use crate::island::Island;
use crate::state::common::destroy_site;
use crate::state::hermit_view::{HermitPhase, HermitState};
use crate::state::{Site, Troop};
use crate::world::vals;

/// Das Bauwerk steht (ohne Bauzeit): bezahlt, gebaut, volle HP.
fn stand_hall(isl: &mut Island) {
    let (wi, si) = isl.hall_site().expect("Platz");
    let s = &mut isl.stages[wi].sites[si];
    s.state = "built".to_string();
    s.hp = s.max_hp;
    isl.hermit.paid = true;
    publish(isl);
}

fn destroy_hall(isl: &mut Island) {
    let (wi, si) = isl.hall_site().expect("Platz");
    let id = isl.stages[wi].sites[si].id;
    destroy_site(&mut isl.stages[wi], id);
    publish(isl);
}

fn add_wall(isl: &mut Island, stage: usize, hp: f64, max_hp: f64) -> usize {
    let w = &mut isl.stages[stage];
    let (id, x) = (w.new_id(), w.hub_x + 30.0 + w.sites.len() as f64);
    w.sites.push(Site {
        id,
        kind: "wall".to_string(),
        x,
        state: "built".to_string(),
        hp,
        max_hp,
        ..Site::default()
    });
    w.sites.len() - 1
}

#[test]
fn dojo_gilt_fuer_alle_stufen_solange_es_steht() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 1);
    assert!(
        isl.stages.iter().all(|w| damage_factor(w) == 1.0),
        "vor dem Bau Grundwert"
    );
    stand_hall(&mut isl);
    assert!(
        isl.stages.iter().all(|w| damage_factor(w) == 1.15),
        "alle Stufen der Insel"
    );
    destroy_hall(&mut isl);
    assert!(
        isl.stages.iter().all(|w| damage_factor(w) == 1.0),
        "zerstört: Grundwert"
    );
    let h = isl.stages[0].hermit.as_ref().expect("Zustand");
    assert!(!h.bonus && h.phase == HermitPhase::Fallen);
}

#[test]
fn schrein_hat_keinen_schadensbonus_und_dojo_keine_heilung() {
    let (data_s, data_p) = (hermit_data("swordsmith"), hermit_data("priestess"));
    let (_, mut dojo) = hermit_island(&data_s, 1);
    let (_, mut schrein) = hermit_island(&data_p, 1);
    stand_hall(&mut dojo);
    stand_hall(&mut schrein);
    assert_eq!(damage_factor(&schrein.stages[0]), 1.0);
    assert_eq!(dojo.stages[0].hermit.as_ref().map(|h| h.wall_heal), Some(0.0));
    assert_eq!(schrein.stages[0].hermit.as_ref().map(|h| h.wall_heal), Some(0.25));
}

fn hit_damage(factor: f64) -> f64 {
    let mut w = bare_world();
    w.hermit = Some(HermitState {
        damage_factor: factor,
        ..HermitState::default()
    });
    let ti = warrior_at(&mut w, 190.0); // Posten ohne Mauer: Hub - 10
    let e = spawn(&mut w, "skeleton", 190.0);
    let (hp, v) = (w.enemies[e].hp, vals(&w));
    crate::unit::warrior::step_warrior(&mut w, &v, None, ti, 1.0 / 30.0);
    hp - w.enemies[e].hp
}

#[test]
fn krieger_schlagen_mit_dem_faktor() {
    let (base, boosted) = (hit_damage(1.0), hit_damage(1.15));
    assert!(base > 0.0, "der Krieger trifft");
    assert!((boosted - base * 1.15).abs() < 1e-9, "{boosted} gegen {base} mal 1.15");
}

fn arrow_damage(factor: f64) -> f64 {
    let mut w = bare_world();
    w.hermit = Some(HermitState {
        damage_factor: factor,
        ..HermitState::default()
    });
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: "archer".into(),
        x: 190.0,
        hp: 100.0,
        max_hp: 100.0,
        ..Troop::default()
    });
    spawn(&mut w, "skeleton", 192.0);
    let v = vals(&w);
    crate::unit::archer::step_archer(&mut w, &v, None, 0, 1.0 / 30.0);
    w.projectiles.first().map_or(0.0, |p| p.damage)
}

#[test]
fn schuetzen_schiessen_mit_dem_faktor() {
    let (base, boosted) = (arrow_damage(1.0), arrow_damage(1.15));
    assert!(base > 0.0, "der Schütze schießt");
    assert!((boosted - base * 1.15).abs() < 1e-9, "{boosted} gegen {base} mal 1.15");
}

#[test]
fn schrein_heilt_jede_stehende_mauer_bei_tagesanbruch_hoechstens_bis_voll() {
    let data = hermit_data("priestess");
    let (_, mut isl) = hermit_island(&data, 0);
    let (a, b, c, d) = (
        add_wall(&mut isl, 0, 100.0, 300.0),
        add_wall(&mut isl, 0, 290.0, 300.0),
        add_wall(&mut isl, 0, 300.0, 300.0),
        add_wall(&mut isl, 1, 40.0, 200.0),
    );
    stand_hall(&mut isl);
    before_dawn(&mut isl);
    let ev = run(&mut isl, &data, 0.2);
    assert!(count(&ev, "dawn") >= 1, "Tagesanbruch kam");
    assert_eq!(isl.stages[0].sites[a].hp, 175.0, "100 + 25 % von 300");
    assert_eq!(isl.stages[0].sites[b].hp, 300.0, "höchstens bis voll");
    assert_eq!(isl.stages[0].sites[c].hp, 300.0);
    assert_eq!(
        isl.stages[1].sites[d].hp, 90.0,
        "auch in der tieferen Stufe: 40 + 25 % von 200"
    );
    assert!(count(&ev, "wallsHealed") >= 1);
}

#[test]
fn ohne_schrein_oder_mit_zerstoertem_schrein_keine_heilung() {
    let data = hermit_data("priestess");
    for state in ["unpaid", "destroyed"] {
        let (_, mut isl) = hermit_island(&data, 0);
        let a = add_wall(&mut isl, 0, 100.0, 300.0);
        if state == "destroyed" {
            stand_hall(&mut isl);
            destroy_hall(&mut isl);
        }
        before_dawn(&mut isl);
        // Morgengrauen ist noch nicht Tag: der Wiederaufbau (nur bei Tag) hat noch nicht begonnen
        run(&mut isl, &data, 0.2);
        assert_eq!(isl.stages[0].sites[a].hp, 100.0, "{state}: keine Heilung");
    }
}

#[test]
fn zerstoertes_bauwerk_wird_ohne_zweite_zahlung_wieder_gebaut() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 1);
    let m = add_deep_player(&mut isl);
    pay_at_hut(&mut isl, &data, m);
    run(&mut isl, &data, 21.0);
    assert_eq!(hall(&isl).state, "built");
    let gold = isl.stages[1].players[0].gold;
    destroy_hall(&mut isl);
    assert_eq!(hall(&isl).state, "unpaid");
    assert!(isl.hermit.paid, "bezahlt bleibt bezahlt");
    let ev = tick(&mut isl, &data, &[]);
    assert_eq!(count(&ev, "hermitBuilding"), 1, "bei Tag beginnt der Wiederaufbau");
    assert_eq!(
        hall(&isl).max_hp,
        250.0,
        "destroy_site setzt max_hp auf 0, der Einsiedler stellt es her"
    );
    let ev = run(&mut isl, &data, 21.0);
    assert_eq!(count(&ev, "built"), 1);
    assert_eq!(hall(&isl).state, "built");
    assert_eq!(isl.stages[1].players[0].gold, gold, "keine Münzen für den Wiederaufbau");
    // an der Hütte bleibt es abgelehnt
    let ev = pay_at_hut(&mut isl, &data, m);
    assert_eq!((count(&ev, "hermitPaid"), count(&ev, "hermitRejected")), (0, 1));
}

#[test]
fn beschaedigtes_bauwerk_wird_in_der_bauzeit_repariert() {
    let data = hermit_data("swordsmith");
    let (_, isl) = hermit_island(&data, 1);
    let w = &isl.stages[0];
    let s = hall(&isl);
    assert_eq!(hall_seconds(w, s), 20.0, "Bauzeit aus hermits.json");
    let other = &w.sites[0];
    assert_eq!(hall_seconds(w, other), 0.0, "andere Plätze: nicht zuständig");
    assert_eq!(crate::building::repair::level_seconds(w, &vals(w), s), 20.0);
}

#[test]
fn bauwerk_zaehlt_nicht_zum_sieg_alle_gebaut() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 1);
    let hall_id = hall(&isl).id;
    for w in &mut isl.stages {
        for s in &mut w.sites {
            if s.id != hall_id {
                s.state = "built".to_string();
            }
        }
    }
    assert!(
        crate::island::victory::all_built(&isl),
        "das freiwillige Bauwerk hält den Sieg nicht auf"
    );
}
