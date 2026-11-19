//! Taifun (RU62.5, AC-03): Zustand, nur Tiefe 0, Welt ohne Wetter, Tempo und Reichweite beider Seiten.
use super::testutil::{at, first_weather_night, taifun_data, weather_island};
use crate::enemies::step_enemies;
use crate::enemies::testutil::{DT, add_player, bare_world, quiet_world, spawn, step_for, warrior_at};
use crate::island::testutil::test_data;
use crate::state::Troop;
use crate::state::weather_view::{WeatherKind, WeatherPhase};
use crate::world::vals;

const FACTOR: f64 = 0.7;

#[test]
fn zustand_vorboten_wirkung_ende() {
    let data = taifun_data();
    let mut isl = weather_island(&data, 1);
    let n = first_weather_night(&isl);
    for p in ["dawn", "day", "dusk"] {
        at(&mut isl, n, p);
        let s = isl.stages[0].weather.expect("Vorboten");
        assert_eq!((s.kind, s.phase), (WeatherKind::Taifun, WeatherPhase::Forecast), "{p}");
        assert_eq!(
            (isl.stages[0].weather_speed, isl.stages[0].weather_range),
            (1.0, 1.0),
            "{p}"
        );
    }
    at(&mut isl, n, "night");
    let s = isl.stages[0].weather.expect("Wirkung");
    assert_eq!((s.kind, s.phase), (WeatherKind::Taifun, WeatherPhase::Active));
    assert_eq!(
        (isl.stages[0].weather_speed, isl.stages[0].weather_range),
        (FACTOR, FACTOR)
    );
    at(&mut isl, n + 1, "dawn");
    assert!(isl.stages[0].weather.is_none(), "Dauer eine Nacht");
    assert_eq!((isl.stages[0].weather_speed, isl.stages[0].weather_range), (1.0, 1.0));
}

#[test]
fn wirkung_nur_an_der_oberflaeche_zwei_stufen() {
    let data = taifun_data();
    let mut isl = weather_island(&data, 2);
    let n = first_weather_night(&isl);
    at(&mut isl, n, "night");
    let (top, deep) = (&isl.stages[0], &isl.stages[1]);
    assert_eq!((top.biome.depth, deep.biome.depth), (0, 1));
    assert_eq!((top.weather_speed, top.weather_range), (FACTOR, FACTOR));
    // Unter Tage läuft der Zeitplan mit (Zustand), die Wirkung bleibt aus.
    assert_eq!(deep.weather, top.weather);
    assert_eq!((deep.weather_speed, deep.weather_range), (1.0, 1.0));
}

#[test]
fn plan_aus_dem_seed_ohne_spielstand_feld() {
    let data = taifun_data();
    let (a, b) = (weather_island(&data, 1), weather_island(&data, 1));
    assert_eq!(
        a.stages[0].weather_cfg, b.stages[0].weather_cfg,
        "gleicher Seed, gleicher Plan"
    );
    assert!(std::sync::Arc::ptr_eq(
        a.stages[0].weather_cfg.as_ref().expect("Plan"),
        a.stages[1].weather_cfg.as_ref().expect("Plan")
    ));
    let n = first_weather_night(&a);
    let (mut a, mut b) = (a, b);
    at(&mut a, n, "night");
    at(&mut b, n, "night"); // wie nach dem Laden mitten in der Wetternacht: nur Seed, Tag und Phase zählen
    assert_eq!(a.stages[0].weather, b.stages[0].weather);
    let json = serde_json::to_value(&a.stages[0]).expect("JSON");
    assert_eq!(
        json["weather"],
        serde_json::json!({"kind": "taifun", "phase": "active"})
    );
}

#[test]
fn welt_ohne_wetter_wie_bisher() {
    let data = test_data(); // `test` steht nicht in weather.json › worlds
    let mut isl = weather_island(&data, 1);
    assert!(isl.stages.iter().all(|w| w.weather_cfg.is_none()));
    for night in 1..30 {
        at(&mut isl, night, "night");
        let w = &isl.stages[0];
        assert!(w.weather.is_none() && (w.weather_speed, w.weather_range) == (1.0, 1.0));
    }
    let json = serde_json::to_value(&isl.stages[0]).expect("JSON");
    assert!(json.get("weather").is_none());
}

#[test]
fn monarch_ist_langsamer() {
    let mut w = bare_world();
    let pi = add_player(&mut w);
    let v = vals(&w);
    let normal = crate::monarch::speed_of(&w, &v, pi);
    w.weather_speed = FACTOR;
    assert!((crate::monarch::speed_of(&w, &v, pi) - normal * FACTOR).abs() < 1e-9);
}

#[test]
fn truppe_ist_langsamer() {
    let run = |factor: f64| {
        let mut w = bare_world();
        w.weather_speed = factor;
        let v = vals(&w);
        let ti = warrior_at(&mut w, 100.0);
        crate::unit::walk_to(&mut w, &v, ti, 150.0, 1.0);
        w.troops[ti].x - 100.0
    };
    let (normal, taifun) = (run(1.0), run(FACTOR));
    assert!(
        normal > 0.0 && (taifun - normal * FACTOR).abs() < 1e-9,
        "{normal} {taifun}"
    );
}

#[test]
fn gegner_ist_langsamer() {
    let run = |factor: f64| {
        let mut w = quiet_world();
        w.weather_speed = factor;
        let start = w.hub_x - 80.0;
        let e = spawn(&mut w, "skeleton", start);
        step_for(&mut w, 1.0);
        (w.enemies[e].x - start).abs()
    };
    let (normal, taifun) = (run(1.0), run(FACTOR));
    assert!(
        normal > 1.0 && (taifun - normal * FACTOR).abs() < 0.05,
        "{normal} {taifun}"
    );
}

/// Ein Fernkämpfer-Gegner (Reichweite 10) schießt auf eine Truppe in 9 Units; mit Taifun reicht es nicht mehr (7 + Körper).
#[test]
fn gegner_fernkaempfer_kuerzere_reichweite() {
    let shots = |range_factor: f64| {
        let mut w = quiet_world();
        w.weather_range = range_factor;
        let at_x = w.hub_x - 30.0;
        let e = spawn(&mut w, "goblinArcher", at_x);
        let tx = w.enemies[e].x + 9.0;
        warrior_at(&mut w, tx);
        step_enemies(&mut w, None, DT);
        w.projectiles.len()
    };
    assert_eq!(shots(1.0), 1, "ohne Wetter in Reichweite");
    assert_eq!(shots(FACTOR), 0, "mit Taifun außer Reichweite");
}

/// Der Bogenschütze (Reichweite 10) der Spielerseite trifft einen Gegner in 9 Units nur ohne Taifun.
#[test]
fn bogenschuetze_kuerzere_reichweite() {
    let shots = |range_factor: f64| {
        let mut w = bare_world();
        w.weather_range = range_factor;
        let v = vals(&w);
        let x = w.hub_x - 10.0; // Posten hinter dem Hub auf der linken Seite
        let id = w.new_id();
        w.troops.push(Troop {
            id,
            kind: "archer".into(),
            x,
            hp: 100.0,
            max_hp: 100.0,
            ..Troop::default()
        });
        spawn(&mut w, "skeleton", x - 9.0);
        crate::unit::archer::step_archer(&mut w, &v, None, 0, DT);
        w.projectiles.len()
    };
    assert_eq!(shots(1.0), 1, "ohne Wetter in Reichweite");
    assert_eq!(shots(FACTOR), 0, "mit Taifun außer Reichweite");
}
