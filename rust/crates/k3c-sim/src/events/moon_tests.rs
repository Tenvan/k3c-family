//! Go: `events_moon_test.go` (K3.1, B-131/AC-01): Vollmond und Blutmond. Die Tests ohne Insel bauen die Stufe aus
//! `World::default()` mit dem Spiegel `island_events`; die Tests mit echter Insel warten auf `decode_world_story`.

use super::moon::{event_active, event_in_night, full_moon_extra, moon_damage, moon_drop, night_events_end};
use super::testutil::{island_world, moon_island, night_of};
use super::values::night;
use crate::enemies::waves::SpawnOrder;
use crate::state::CycleInfo;

fn order(kind: &str) -> SpawnOrder {
    SpawnOrder {
        kind: kind.to_string(),
        x: 0.0,
        delay: 0.0,
    }
}

#[test]
fn vollmond_und_blutmond_rhythmus() {
    let w = island_world();
    let (full, blood) = (
        night(&w, "fullMoon").expect("fullMoon"),
        night(&w, "bloodMoon").expect("bloodMoon"),
    );
    let cases = [
        (6, false, false),
        (7, true, false),
        (8, false, false),
        (13, false, true),
        (14, true, false),
        (91, true, true),
    ];
    for (n, f, b) in cases {
        assert_eq!(event_in_night(&w, &full, n), f, "Nacht {n}: Vollmond");
        assert_eq!(event_in_night(&w, &blood, n), b, "Nacht {n}: Blutmond");
    }
    let single = crate::state::World::default();
    assert!(
        !event_in_night(&single, &full, 7),
        "ohne Insel kein Event (Golden-Läufe bleiben gleich)"
    );
    assert!(!event_in_night(&w, &full, 0), "Nacht 0 gibt es nicht");
}

#[test]
fn dev_nacht_erzwingt_das_event_zusaetzlich_zum_rhythmus() {
    let mut w = island_world();
    w.island_events
        .as_mut()
        .expect("Insel")
        .nights
        .insert("bloodMoon".into(), 5);
    let blood = night(&w, "bloodMoon").expect("bloodMoon");
    assert!(event_in_night(&w, &blood, 5) && event_in_night(&w, &blood, 13) && !event_in_night(&w, &blood, 6));
}

#[test]
fn blutmond_schaden_und_drop_nur_in_dieser_nacht() {
    let mut w = island_world();
    night_of(&mut w, 13);
    assert!((moon_damage(&w, 20.0) - 26.0).abs() < 1e-9, "Blutmond-Schaden 26");
    assert_eq!(moon_drop(&w, 7), 14, "Blutmond-Drop 14");
    night_of(&mut w, 14); // Nacht 14: Tag davor, kein Blutmond
    assert!(moon_damage(&w, 20.0) == 20.0 && moon_drop(&w, 7) == 7);
    w.cycle = CycleInfo {
        phase: "day".into(),
        day: 13,
        ..Default::default()
    };
    assert_eq!(moon_drop(&w, 7), 7, "am Tag kein Blutmond");
}

#[test]
fn vollmond_welle_mehr_woelfe_und_ein_alpha_wolf() {
    let mut w = island_world();
    w.portals = vec![10.0, 90.0];
    w.biome.cycle.r#type = "dayNight".into();
    night_of(&mut w, 7);
    let plan = [order("wolf"), order("wolf"), order("wolf"), order("greed")];
    let mut kinds = std::collections::BTreeMap::new();
    for s in full_moon_extra(&mut w, &plan) {
        *kinds.entry(s.kind).or_insert(0) += 1;
    }
    assert_eq!(
        kinds,
        [("alphaWolf".to_string(), 1), ("wolf".to_string(), 2)].into(),
        "3 Wölfe: 2 Wölfe (50 %, aufgerundet) und 1 Alpha-Wolf"
    );
    w.biome.cycle.r#type = "none".into();
    assert!(
        full_moon_extra(&mut w, &plan).is_empty(),
        "Vollmond ändert nur Wellen der Tag/Nacht-Stufen"
    );
}

#[test]
fn vollmond_und_blutmond_in_nacht_91_wirken_zusammen() {
    let mut w = island_world();
    w.portals = vec![10.0];
    w.biome.cycle.r#type = "dayNight".into();
    night_of(&mut w, 91);
    assert!(event_active(&w, &night(&w, "fullMoon").expect("fullMoon")));
    assert_eq!(moon_damage(&w, 10.0), 13.0);
    assert_eq!(full_moon_extra(&mut w, &[]).len(), 1);
}

#[test]
fn vollmond_belohnung_nur_wenn_die_burg_hielt() {
    let reward = |fell: bool| {
        let mut w = island_world();
        w.portals = vec![10.0];
        w.players.push(Default::default());
        w.cycle = CycleInfo {
            phase: "day".into(),
            day: 8,
            ..Default::default()
        };
        if fell {
            w.castle_fell_night = 7;
        }
        night_events_end(&mut w);
        w.coins.len() as i64
    };
    let want = night(&island_world(), "fullMoon").expect("fullMoon").reward_gold;
    assert_eq!(reward(false), want, "Burg hielt");
    assert_eq!(reward(true), 0, "Burg gefallen");
}

#[test]
fn vollmond_belohnung_braucht_spieler_und_portale() {
    let mut w = island_world();
    w.cycle = CycleInfo {
        phase: "day".into(),
        day: 8,
        ..Default::default()
    };
    night_events_end(&mut w);
    assert!(w.coins.is_empty(), "keine Portale, kein Spieler");
    w.portals = vec![10.0];
    w.players.push(crate::state::Player {
        free: true,
        ..Default::default()
    });
    night_events_end(&mut w);
    assert!(w.coins.is_empty(), "ein getrennter Monarch spielt nicht");
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story) und RU18 (step_players)"]
fn vollmond_ereignisse_je_nacht_einmal() {
    let (data, mut isl) = moon_island(7);
    let events = crate::island::testutil::run_island(&mut isl, &data, &[], 8.0);
    let all: Vec<_> = events.iter().flatten().collect();
    let started: Vec<_> = all.iter().filter(|e| e["type"] == "eventStarted").collect();
    let ended: Vec<_> = all.iter().filter(|e| e["type"] == "eventEnded").collect();
    assert!(started.len() == 1 && ended.len() == 1 && started[0]["event"] == "fullMoon" && ended[0]["day"] == 7);
}
