//! Tests zu `water_weather.rs` (RU62.7, B-999): Zustand `forecast` und `active` je Art ergibt die erwarteten Zeichen-Einträge,
//! unter Tage keine; Regen und Wind sind reine Rechnung innerhalb der Fläche; das Bild-Budget je Zelle ist begrenzt.

use std::rc::Rc;

use super::*;
use crate::view::frame::{Frame, Seat, Stage};
use crate::world::testutil::layer_cell;
use k3c_platform::NullCanvas;
use k3c_sim::state::weather_view::{FloodZone, WeatherState};

fn weather(kind: WeatherKind, phase: WeatherPhase) -> Option<WeatherState> {
    Some(WeatherState { kind, phase })
}

fn sim_world(w: Option<WeatherState>, depth: i64, zones: Vec<FloodZone>) -> SimWorld {
    let mut s = SimWorld {
        weather: w,
        width_units: 100.0,
        flood_zones: zones,
        ..Default::default()
    };
    s.biome.depth = depth;
    s
}

fn frame(w: SimWorld, clock: f64) -> Frame {
    Frame {
        clock,
        stages: vec![Stage {
            index: 0,
            level: "forest".to_string(),
            world: Some(Rc::new(w)),
            ..Stage::default()
        }],
        seats: vec![Seat::default()],
        ..Frame::default()
    }
}

fn counts(w: SimWorld, sky: bool) -> (u64, u64) {
    let (world, c, f) = (World::new(None), layer_cell(10.0), frame(w, 3.0));
    let mut canvas = NullCanvas::default();
    if sky {
        world.draw_weather_sky(&mut canvas, &f, &c);
    } else {
        world.draw_weather_front(&mut canvas, &f, &c);
    }
    (canvas.rects, canvas.shapes)
}

#[test]
fn sicht_je_art_und_phase_und_nichts_unter_tage() {
    let s = |k, p, d| sight_of(&sim_world(weather(k, p), d, Vec::new()));
    assert_eq!(s(WeatherKind::Taifun, WeatherPhase::Forecast, 0), Sight::TaifunForecast);
    assert_eq!(s(WeatherKind::Taifun, WeatherPhase::Active, 0), Sight::TaifunActive);
    assert_eq!(s(WeatherKind::Flut, WeatherPhase::Forecast, 0), Sight::FloodForecast);
    assert_eq!(s(WeatherKind::Flut, WeatherPhase::Active, 0), Sight::FloodActive);
    assert_eq!(
        s(WeatherKind::Taifun, WeatherPhase::Active, 1),
        Sight::None,
        "unter Tage"
    );
    assert_eq!(sight_of(&sim_world(None, 0, Vec::new())), Sight::None, "ohne Wetter");
}

#[test]
fn vorboten_des_taifuns_zeichnen_die_wolkenbank_ohne_regen() {
    let w = sim_world(weather(WeatherKind::Taifun, WeatherPhase::Forecast), 0, Vec::new());
    let (rects, _) = counts(w.clone(), true);
    assert!(rects >= 4, "Verlauf der Wolkenbank ohne Sheet: {rects}");
    assert_eq!(counts(w, false), (0, 0), "kein Regen in den Vorboten");
}

#[test]
fn aktiver_taifun_zeichnet_regen_und_wind_und_den_schleier() {
    let w = sim_world(weather(WeatherKind::Taifun, WeatherPhase::Active), 0, Vec::new());
    let (_, shapes) = counts(w.clone(), false);
    assert_eq!(shapes as usize, RAIN_STREAKS + WIND_STREAKS, "Linien");
    let (rects, _) = counts(w, true);
    assert!(rects >= 5, "Wolkenbank und Schleier: {rects}");
}

#[test]
fn vorboten_der_flut_heben_das_wasser_am_ufer() {
    let w = sim_world(weather(WeatherKind::Flut, WeatherPhase::Forecast), 0, Vec::new());
    assert_eq!(counts(w, false), (1, 0));
    assert!(rise_units(0.5) > rise_units(0.1) && rise_units(2.0) == rise_units(1.0));
}

#[test]
fn aktive_flut_zeichnet_nur_die_zonen_im_bild() {
    let zones = vec![FloodZone { from: 0.0, to: 8.0 }, FloodZone { from: 90.0, to: 100.0 }];
    let w = sim_world(weather(WeatherKind::Flut, WeatherPhase::Active), 0, zones);
    // Kamera bei x = 10: links liegt die Zone 0..8 im Bild, rechts 90..100 weit außerhalb
    let (rects, shapes) = counts(w, false);
    assert_eq!(rects, 1, "eine Wasserfläche");
    assert!(shapes >= 1, "Wasserlinie (und Grenze)");
}

#[test]
fn unter_tage_zeichnet_nichts() {
    for (k, p) in [
        (WeatherKind::Taifun, WeatherPhase::Forecast),
        (WeatherKind::Taifun, WeatherPhase::Active),
        (WeatherKind::Flut, WeatherPhase::Forecast),
        (WeatherKind::Flut, WeatherPhase::Active),
    ] {
        let w = sim_world(weather(k, p), 2, vec![FloodZone { from: 0.0, to: 8.0 }]);
        assert_eq!(counts(w.clone(), true), (0, 0), "{k:?} {p:?} Himmel");
        assert_eq!(counts(w, false), (0, 0), "{k:?} {p:?} Front");
    }
}

#[test]
fn regen_und_wind_bleiben_in_der_flaeche_und_bewegen_sich() {
    let (w, h) = (800.0, 300.0);
    for i in 0..RAIN_STREAKS {
        let a = rain_streak(i, 1.0, w, h);
        assert!(a[1] >= 0.0 && a[1] <= h && a[3] >= a[1], "Regen {i}: {a:?}");
        assert_eq!(a, rain_streak(i, 1.0, w, h), "gleiche Zeit, gleiche Lage");
        assert_ne!(a, rain_streak(i, 1.3, w, h), "Regen fällt");
    }
    for i in 0..WIND_STREAKS {
        let a = wind_streak(i, 1.0, w, h);
        assert!(a[1] >= 0.0 && a[1] <= h, "Wind {i}: {a:?}");
        assert!(wind_streak(i, 2.0, w, h)[0] != a[0], "Wind wandert");
    }
}

#[test]
fn flut_zone_wird_auf_die_zelle_begrenzt() {
    assert_eq!(flood_span(-50.0, 120.0, 0.0, 100.0), Some((0.0, 100.0)));
    assert_eq!(flood_span(150.0, 200.0, 0.0, 100.0), None, "außerhalb");
    assert_eq!(
        flood_span(120.0, -50.0, 0.0, 100.0),
        Some((0.0, 100.0)),
        "Reihenfolge egal"
    );
}
