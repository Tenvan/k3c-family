//! Keine neuen Gegner im Frieden (RU62.12, AC-06): Wellen, Portal-Verteidiger, Aggressionspool, Wachen, Vollmond, Blutmond
//! und Händler-Überfall; Tag und Nacht, Händler und Schiffsbau laufen weiter. Jeder Lauf hat eine Gegenprobe ohne Frieden.
use super::testutil::{count, day_seconds, festival_data, festival_island, pending_total, tick};
use std::sync::Arc;

use crate::island::Island;
use crate::island::testutil::{DT, test_data};
use crate::state::festival_view::FestivalRule;
use crate::state::{IslandRef, Troop, World};
use k3c_data::Data;

/// Rechnet `days` Tage; liefert (Wellen-Ereignisse, größte Zahl Gegner und Warteschlange, Ereignisse `eventStarted`,
/// Überfall-Ereignisse des Händlers).
fn run_days(isl: &mut Island, data: &Data, days: f64) -> (usize, usize, usize, usize) {
    let (mut waves, mut peak, mut events, mut raids) = (0, 0, 0, 0);
    let ticks = (days * day_seconds(isl) / DT).ceil() as usize;
    for _ in 0..ticks {
        let ev = tick(isl, data);
        waves += count(&ev, "wave");
        events += count(&ev, "eventStarted");
        raids += ev
            .iter()
            .filter(|e| e["type"] == "eventStarted" && e["event"] == "merchantRaid")
            .count();
        peak = peak.max(pending_total(isl));
    }
    (waves, peak, events, raids)
}

/// Eine Insel nach dem Sieg mit erzwungenem Vollmond in Nacht 2 und Blutmond in Nacht 3.
fn won_island(data: &Data) -> Island {
    let (_, mut isl) = festival_island(data, 1);
    isl.endboss_defeated = true;
    isl.dev.nights.insert("fullMoon".to_string(), 2);
    isl.dev.nights.insert("bloodMoon".to_string(), 3);
    isl
}

/// AC-06: Über drei Tage und eine Vollmond-Nacht (dazu Blutmond) entsteht in keiner Stufe ein Gegner: keine Welle, keine
/// Warteschlange, kein Event. Gegenprobe: Dieselbe Insel in einer Welt ohne Fest hat Wellen, Gegner und Events.
#[test]
fn nach_dem_sieg_entstehen_keine_neuen_gegner() {
    let data = festival_data();
    let mut isl = won_island(&data);
    let (waves, peak, events, _) = run_days(&mut isl, &data, 3.5);
    assert_eq!(
        (waves, peak, events),
        (0, 0, 0),
        "Frieden: keine Welle, kein Gegner, kein Event"
    );
    assert!(isl.stages[0].cycle.day >= 3, "Tag {}", isl.stages[0].cycle.day);
    assert!(isl.stages.iter().all(|w| w.wave == 0));

    let data = test_data();
    let mut control = won_island(&data);
    let (waves, peak, events, _) = run_days(&mut control, &data, 3.5);
    assert!(
        waves > 0 && peak > 0 && events > 0,
        "Gegenprobe: Wellen {waves}, Gegner {peak}, Events {events}"
    );
}

/// AC-06: Der Händler kommt im Frieden ohne Überfall; Gegenprobe: ohne Frieden überfällt ihn jeder 4. Besuch.
#[test]
fn haendler_kommt_ohne_ueberfall() {
    let data = festival_data();
    let mut isl = won_island(&data);
    isl.merchant_visits = 3; // der nächste Besuch wäre der 4. und damit ein Überfall
    let mut arrived = 0;
    for _ in 0..((4.0 * day_seconds(&isl) / DT).ceil() as usize) {
        let ev = tick(&mut isl, &data);
        arrived += count(&ev, "merchantArrived");
        assert_eq!(pending_total(&isl), 0);
    }
    assert!(arrived > 0, "der Händler kam");
    assert!(isl.merchant_visits > 3);
    assert!(isl.stages.iter().all(|w| w.merchant.as_ref().is_none_or(|m| !m.raid)));

    let data = test_data();
    let mut control = won_island(&data);
    control.merchant_visits = 3;
    let (_, _, _, raids) = run_days(&mut control, &data, 4.0);
    assert!(raids > 0, "Gegenprobe: Überfall {raids}");
}

/// Ein ruhiges Welt-Stufen-Modell mit Fest-Regel und Ziel Endboss, zwei Portalen und einem Krieger im Angriff vor dem
/// ersten Portal; liefert Welt und Index der Truppe.
fn siege_world() -> (World, usize) {
    let mut w = crate::enemies::testutil::quiet_world();
    crate::world::add_player(&mut w);
    w.troops.clear();
    w.enemies.clear();
    w.spawn_queue.clear();
    w.camps.clear();
    w.aggression = Default::default();
    w.festival_rule = Some(Arc::new(FestivalRule {
        effect: "lanterns".to_string(),
        flee_speed_factor: 1.2,
        exit_margin_units: 0.3,
    }));
    w.festival_goal = true;
    let hp = 1e9;
    let id = w.new_id();
    let x = w.portals[0];
    w.troops.push(Troop {
        id,
        kind: "warrior".into(),
        x,
        hp,
        max_hp: hp,
        anchor_x: w.hub_x,
        attacking: true,
        ..Troop::default()
    });
    (w, 0)
}

/// Verteidiger-Wellen (`portalDefense`) nach `seconds` Sekunden Belagerung des ersten Portals.
fn defenders(defeated: bool, seconds: f64) -> usize {
    let (mut w, ti) = siege_world();
    let x = w.portals[0];
    let mut r = IslandRef {
        endboss_defeated: defeated,
        ..IslandRef::default()
    };
    let mut n = 0;
    for _ in 0..(seconds / DT) as usize {
        w.troops[ti].x = x;
        crate::world::step(&mut w, &[], Some(&mut r), DT);
        n += count(&w.events, "portalDefense");
    }
    n
}

/// AC-06: Ein angegriffenes Portal schickt im Frieden keine Verteidiger; Gegenprobe: ohne Sieg wehrt es sich in Wellen.
#[test]
fn portale_schicken_keine_verteidiger() {
    assert!(defenders(false, 40.0) > 0, "Gegenprobe: das Portal wehrt sich");
    assert_eq!(defenders(true, 40.0), 0, "Frieden: keine Verteidiger");
}
