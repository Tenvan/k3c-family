//! Welt-Events Überfall, Sandsturm und Nebel (B-991, B-992). Die Events laufen über die Dev-Nacht (`nights`) in
//! `island_world`, weil die Basis sie aus lässt (`everyNights: 0`).

use super::storm::{queue_raid, update};
use super::testutil::{island_world, night_of};
use crate::enemies::step_spawns;
use crate::state::World;

/// Eine Stufe mit Portalen, Tag/Nacht-Zyklus und dem Event `id` in Nacht `n` (Dev-Nacht).
fn with_event(id: &str, n: i64) -> World {
    let mut w = island_world();
    w.portals = vec![10.0, 90.0];
    w.biome.cycle.r#type = "dayNight".into();
    w.island_events.as_mut().expect("Insel").nights.insert(id.into(), n);
    night_of(&mut w, n);
    w
}

#[test]
fn ueberfall_schickt_den_anteil_der_welle_mit_tempo() {
    let mut w = with_event("raid", 5);
    queue_raid(&mut w, 10, 1.0, 1.0);
    assert_eq!(w.spawn_queue.len(), 5, "Anteil 0.5 von 10 Gegnern");
    assert!(w.spawn_queue.iter().all(|s| s.kind == "wolf" && s.speed_factor == 1.3));
    let mut klein = with_event("raid", 5);
    queue_raid(&mut klein, 1, 1.0, 1.0);
    assert_eq!(klein.spawn_queue.len(), 1, "aufgerundet, mindestens 1");
}

#[test]
fn ueberfall_nur_in_seiner_nacht_und_an_der_oberflaeche() {
    let mut w = with_event("raid", 5);
    night_of(&mut w, 6);
    queue_raid(&mut w, 10, 1.0, 1.0);
    assert!(w.spawn_queue.is_empty(), "nicht in Nacht 6");
    let mut tief = with_event("raid", 5);
    tief.biome.depth = 1;
    queue_raid(&mut tief, 10, 1.0, 1.0);
    assert!(tief.spawn_queue.is_empty(), "nur an der Oberfläche");
}

#[test]
fn ueberfall_gruppe_wird_schneller_gespawnt() {
    let mut w = with_event("raid", 5);
    queue_raid(&mut w, 4, 1.0, 1.0);
    w.time = 1000.0; // alle Gegner sind fällig
    let v = crate::state::links::values(&w);
    let base = v.enemies["wolf"].speed;
    step_spawns(&mut w, &v, None);
    assert!(!w.enemies.is_empty(), "Überfall spawnt Gegner");
    assert!(
        w.enemies.iter().all(|e| (e.speed - base * 1.3).abs() < 1e-9),
        "Tempo × 1.3 je Gegner"
    );
}

#[test]
fn sandsturm_wirkt_auf_tempo_und_reichweite_nur_an_der_oberflaeche() {
    let mut w = with_event("sandstorm", 5);
    update(&mut w);
    assert_eq!((w.weather_speed, w.weather_range), (0.7, 0.7));
    update(&mut w);
    assert_eq!(
        (w.weather_speed, w.weather_range),
        (0.7, 0.7),
        "kein Aufschaukeln je Takt"
    );
    w.biome.depth = 1;
    update(&mut w);
    assert_eq!(
        (w.weather_speed, w.weather_range),
        (1.0, 1.0),
        "unter der Oberfläche keine Wirkung"
    );
}

#[test]
fn nebel_wirkt_nur_auf_die_verteidiger() {
    let mut w = with_event("fog", 5);
    update(&mut w);
    assert_eq!(w.weather_range_def, 0.6, "Verteidiger-Reichweite");
    assert_eq!(
        (w.weather_speed, w.weather_range),
        (1.0, 1.0),
        "Gegner und Tempo unverändert"
    );
    night_of(&mut w, 6);
    update(&mut w);
    assert_eq!(w.weather_range_def, 1.0, "nur in der Event-Nacht");
}

#[test]
fn frieden_schaltet_welt_events_ab() {
    let mut w = with_event("sandstorm", 5);
    w.peace = true;
    update(&mut w);
    assert_eq!(
        (w.weather_speed, w.weather_range),
        (1.0, 1.0),
        "im Frieden kein Sandsturm"
    );
    let mut r = with_event("raid", 5);
    r.peace = true;
    queue_raid(&mut r, 10, 1.0, 1.0);
    assert!(r.spawn_queue.is_empty(), "im Frieden kein Überfall");
}

#[test]
fn welt_events_sind_ueberschreibbar() {
    assert!(
        crate::world::values::override_files().contains(&"welt-events"),
        "Welten schalten Überfall, Sandsturm und Nebel über welt-events ein"
    );
}

#[test]
fn ohne_welt_events_keine_wirkung() {
    let mut w = island_world();
    night_of(&mut w, 5);
    update(&mut w);
    assert_eq!((w.weather_speed, w.weather_range, w.weather_range_def), (1.0, 1.0, 1.0));
    let plain = World::default();
    queue_raid(&mut w, 10, 1.0, 1.0);
    assert!(w.spawn_queue.is_empty() && plain.spawn_queue.is_empty());
}
