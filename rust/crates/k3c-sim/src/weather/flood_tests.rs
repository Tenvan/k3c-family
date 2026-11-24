//! Flut (RU62.6, AC-03): Zonen, Sperre für Spieler und Gegner, Heraus-Laufen, Portal-Haltung, schmale Insel, Stufen.
use super::flood::{apply, in_zone, update};
use super::testutil::{at, first_weather_night, flood_data, taifun_data, weather_island};
use crate::economy::players::move_player;
use crate::enemies::step_spawns;
use crate::enemies::testutil::{DT, spawn, warrior_at};
use crate::island::{Island, add_island_player};
use crate::state::weather_view::{FloodZone, WeatherKind, WeatherPhase};
use crate::state::{PlayerCommand, QueuedSpawn, World};
use crate::world::vals;

/// Insel in der aktiven Flutnacht; liefert die Insel und die Nacht.
fn flood_night(players: usize) -> (Island, i64) {
    let data = flood_data();
    let mut isl = weather_island(&data, players);
    let n = first_weather_night(&isl);
    at(&mut isl, n, "night");
    (isl, n)
}

fn zones(w: &World) -> (FloodZone, FloodZone) {
    assert_eq!(w.flood_zones.len(), 2, "{:?}", w.flood_zones);
    (w.flood_zones[0], w.flood_zones[1])
}

#[test]
fn zustand_nennt_art_phase_und_zonen() {
    let data = flood_data();
    let mut isl = weather_island(&data, 1);
    let n = first_weather_night(&isl);
    for p in ["dawn", "day", "dusk"] {
        at(&mut isl, n, p);
        let w = &isl.stages[0];
        let s = w.weather.expect("Vorboten");
        assert_eq!((s.kind, s.phase), (WeatherKind::Flut, WeatherPhase::Forecast), "{p}");
        assert!(w.flood_zones.is_empty(), "Vorboten sperren nichts ({p})");
    }
    at(&mut isl, n, "night");
    let w = &isl.stages[0];
    let s = w.weather.expect("Wirkung");
    assert_eq!((s.kind, s.phase), (WeatherKind::Flut, WeatherPhase::Active));
    let (l, r) = zones(w);
    assert_eq!((l.from, l.to), (0.0, 24.0));
    assert_eq!((r.from, r.to), (w.width_units - 24.0, w.width_units));
    let json = serde_json::to_value(w).expect("JSON");
    assert_eq!(json["weather"], serde_json::json!({"kind": "flut", "phase": "active"}));
    assert_eq!(json["floodZones"][0], serde_json::json!({"from": 0.0, "to": 24.0}));
    at(&mut isl, n + 1, "dawn");
    assert!(isl.stages[0].flood_zones.is_empty(), "Ende bei Tagesanbruch");
    let json = serde_json::to_value(&isl.stages[0]).expect("JSON");
    assert!(json.get("floodZones").is_none());
}

#[test]
fn nur_an_der_oberflaeche_zwei_spieler_auf_zwei_stufen() {
    let (mut isl, _) = flood_night(1);
    let p1 = add_island_player(&mut isl, 1);
    isl.stages[1].players[p1].x = 3.0; // unter Tage liegt der Rand im Trockenen
    isl.stages[0].players[0].x = 3.0;
    assert!(!isl.stages[0].flood_zones.is_empty());
    assert!(isl.stages[1].flood_zones.is_empty());
    assert_eq!(isl.stages[1].weather, isl.stages[0].weather, "Zeitplan läuft mit");
    for w in &mut isl.stages {
        apply(w, 2.0);
    }
    assert_eq!(isl.stages[0].players[0].x, 24.0, "oben läuft er heraus");
    assert_eq!(isl.stages[1].players[p1].x, 3.0, "unten bleibt er stehen");
}

#[test]
fn spieler_laeuft_heraus_und_kommt_nicht_zurueck() {
    let (mut isl, _) = flood_night(1);
    let w = &mut isl.stages[0];
    let (l, r) = zones(w);
    w.players[0].x = 5.0;
    let mut prev = w.players[0].x;
    for _ in 0..200 {
        apply(w, DT);
        assert!(w.players[0].x >= prev, "läuft nur nach außen heraus");
        prev = w.players[0].x;
    }
    assert_eq!(w.players[0].x, l.to);
    // Mit Sprint nach links in die Zone: bleibt an der Kante.
    let v = vals(w);
    let cmd = PlayerCommand {
        move_x: -1.0,
        sprint: true,
        ..PlayerCommand::default()
    };
    for _ in 0..600 {
        move_player(w, &v, 0, &cmd, DT);
        apply(w, DT);
        assert!(w.players[0].x >= l.to - 1e-9, "x = {}", w.players[0].x);
    }
    // Rechts genauso.
    w.players[0].x = r.from - 10.0;
    w.players[0].vx = 0.0;
    let cmd = PlayerCommand { move_x: 1.0, ..cmd };
    for _ in 0..600 {
        move_player(w, &v, 0, &cmd, DT);
        apply(w, DT);
        assert!(w.players[0].x <= r.from + 1e-9, "x = {}", w.players[0].x);
    }
    assert!(w.players[0].x > r.from - 1.0, "er lief bis an die Kante");
}

#[test]
fn truppe_und_gegner_laufen_heraus_flieger_nicht() {
    let (mut isl, _) = flood_night(1);
    let w = &mut isl.stages[0];
    let (l, r) = zones(w);
    let t = warrior_at(w, r.to - 2.0);
    let e = spawn(w, "skeleton", l.from + 4.0);
    let fly = spawn(w, "skeleton", l.from + 6.0);
    w.enemies[fly].traits.push("ignoresWalls".to_string());
    for _ in 0..100 {
        apply(w, DT);
    }
    assert_eq!(w.troops[t].x, r.from, "Truppe an der Kante der rechten Zone");
    assert_eq!(w.enemies[e].x, l.to, "Gegner an der Kante der linken Zone");
    assert_eq!(w.enemies[fly].x, l.from + 6.0, "Flieger ignorieren die Flut");
}

#[test]
fn portal_in_der_zone_haelt_bis_zum_morgen() {
    let (mut isl, n) = flood_night(1);
    let w = &mut isl.stages[0];
    let (l, _) = zones(w);
    let v = vals(w);
    let queue = |x: f64| QueuedSpawn {
        kind: "skeleton".into(),
        x,
        at: 0.0,
        hp_factor: 1.0,
        damage_factor: 1.0,
        speed_factor: 0.0,
    };
    w.spawn_queue.push(queue(l.from + 5.0));
    w.spawn_queue.push(queue(l.to + 30.0));
    let before = w.enemies.len();
    step_spawns(w, &v, None);
    assert_eq!(w.enemies.len(), before + 1, "nur das trockene Portal schickt Gegner");
    assert_eq!(
        w.spawn_queue.len(),
        1,
        "der Spawn im Wasser wartet, nichts geht verloren"
    );
    assert!(in_zone(&w.flood_zones, w.spawn_queue[0].x));
    step_spawns(w, &v, None);
    assert_eq!(w.spawn_queue.len(), 1, "wartet die ganze Nacht");
    at(&mut isl, n + 1, "dawn");
    let w = &mut isl.stages[0];
    step_spawns(w, &v, None);
    assert!(w.spawn_queue.is_empty(), "nach der Flut regulär");
}

#[test]
fn schmale_insel_endet_vor_dem_burg_radius() {
    let (mut isl, _) = flood_night(1);
    let w = &mut isl.stages[0];
    let radius = vals(w).hub.castle_radius_units;
    w.width_units = w.hub_x + radius + 10.0; // rechts bleibt nur 10 Units neben dem Burg-Radius
    update(w);
    let (l, r) = zones(w);
    assert_eq!(l.to, 24.0);
    assert!(r.from >= w.hub_x + radius && r.to == w.width_units, "{r:?}");
    assert_eq!(r.to - r.from, 10.0, "gekürzt statt 24");
    w.width_units = w.hub_x + radius; // rechts ist nur noch die Burg: keine Zone
    update(w);
    assert_eq!(w.flood_zones.len(), 1);
    w.hub_x = 10.0 + radius; // links bleibt nur 10 Units
    update(w);
    assert_eq!(w.flood_zones[0].to, 10.0);
}

/// Der Takt der Stufe ruft die Sperre selbst (nicht nur der Test): Der Monarch in der Zone rückt je Takt heraus.
#[test]
fn takt_der_stufe_ruft_die_sperre() {
    let (mut isl, _) = flood_night(1);
    let w = &mut isl.stages[0];
    w.players[0].x = 3.0;
    crate::world::step(w, &[], None, DT);
    let moved = w.players[0].x - 3.0;
    assert!(moved > 0.0 && moved <= 20.0 * DT + 1e-9, "{moved}");
}

#[test]
fn welt_ohne_flut_unveraendert() {
    // Taifun-Welt und Welt ohne Wetter kennen keine Zonen.
    let data = taifun_data();
    let mut isl = weather_island(&data, 1);
    let n = first_weather_night(&isl);
    at(&mut isl, n, "night");
    assert!(isl.stages[0].flood_zones.is_empty());
    let none = crate::island::testutil::test_data();
    let mut isl = weather_island(&none, 1);
    at(&mut isl, 5, "night");
    let w = &mut isl.stages[0];
    w.players[0].x = 3.0;
    apply(w, 1.0);
    assert_eq!(w.players[0].x, 3.0);
    assert!(serde_json::to_value(&*w).expect("JSON").get("floodZones").is_none());
}
