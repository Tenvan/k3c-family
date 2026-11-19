//! Zeitplan des Wetters (RU62.5, AC-03): gleicher Seed gleiche Nächte, Vorboten am Vortag, eine Nacht, nie in Event-Nächten.
use super::schedule::Schedule;
use crate::state::weather_view::WeatherPhase;

fn plan(offset: i64, every: i64, first: i64) -> Schedule {
    Schedule {
        every,
        first,
        offset,
        moons: vec![7, 13],
    }
}

fn nights(s: &Schedule) -> Vec<i64> {
    (1..=300).filter(|&n| s.is_weather_night(n)).collect()
}

#[test]
fn gleicher_seed_gleiche_naechte() {
    let a = Schedule::new("insel-a", 5, 3, vec![7, 13]);
    let b = Schedule::new("insel-a", 5, 3, vec![7, 13]);
    assert_eq!(a, b);
    assert_eq!(nights(&a), nights(&b));
    let offsets: std::collections::BTreeSet<i64> = (0..30)
        .map(|i| Schedule::new(&format!("insel-{i}"), 5, 3, vec![]).offset)
        .collect();
    assert!(offsets.len() > 1, "der Versatz hängt vom Seed ab: {offsets:?}");
    assert!(offsets.iter().all(|o| (0..5).contains(o)));
}

#[test]
fn rhythmus_und_fruehestens_first_night() {
    let s = plan(2, 5, 10);
    assert!(!s.is_weather_night(2) && !s.is_weather_night(7), "vor firstNight");
    assert!(s.is_weather_night(12) && s.is_weather_night(17) && s.is_weather_night(22));
    assert!(!s.is_weather_night(0) && !s.is_weather_night(-5));
    let n = nights(&plan(1, 5, 3));
    assert!(n.iter().all(|&x| x >= 3));
    assert!((40..=60).contains(&n.len()), "etwa jede 5. Nacht: {}", n.len());
}

#[test]
fn nie_in_vollmond_oder_blutmond() {
    for offset in 0..5 {
        for n in nights(&plan(offset, 5, 1)) {
            assert!(n % 7 != 0 && n % 13 != 0, "Nacht {n} (Versatz {offset})");
        }
    }
}

#[test]
fn event_nacht_rueckt_eine_nacht_weiter() {
    // Nacht 35 ist Vollmond (7): das Wetter weicht auf Nacht 36 aus.
    let s = plan(0, 5, 1);
    assert!(!s.is_weather_night(35) && s.is_weather_night(36));
    // 13 (Blutmond) und 14 (Vollmond) folgen aufeinander: es rückt bis Nacht 15.
    let s = plan(0, 13, 1);
    assert!(!s.is_weather_night(13) && !s.is_weather_night(14) && s.is_weather_night(15));
}

#[test]
fn vorboten_am_vortag_wirkung_eine_nacht() {
    let s = plan(2, 5, 1);
    let n = 12;
    assert!(s.is_weather_night(n) && !s.is_weather_night(n + 1) && !s.is_weather_night(n - 1));
    for p in ["dawn", "day", "dusk"] {
        assert_eq!(s.phase(n, p), Some(WeatherPhase::Forecast), "{p} am Vortag");
    }
    assert_eq!(s.phase(n, "night"), Some(WeatherPhase::Active));
    // Das Morgengrauen nach der Wetternacht gehört schon zum nächsten Tag: nichts mehr.
    assert_eq!(s.phase(n + 1, "dawn"), None);
    assert_eq!(s.phase(n - 1, "night"), None, "die Nacht davor");
    assert_eq!(s.phase(n - 1, "dusk"), None);
}
