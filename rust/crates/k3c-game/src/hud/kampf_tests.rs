//! Port von `native/k3c-game/hud/kampf_test.go`: Boss-Leiste und Event-Banner. Die zwei Radar-Tests der Datei
//! (`TestRadarMarker`, `TestRadarAusschnittUndGroesse`) prüfen `radar.rs` und stehen seit RU20.3 in `radar_tests.rs`
//! (`radar_marker`, `radar_ausschnitt_und_groesse`).

use super::kampf::*;
use k3c_sim::state::{ActiveEvent, Enemy, Merchant};

fn end_boss(hp: f64, max_hp: f64, phase: i64) -> Enemy {
    Enemy {
        id: 7,
        kind: "crystalHeartWarden".to_string(),
        boss: true,
        hp,
        max_hp,
        phase,
        ..Default::default()
    }
}

fn enemy(id: i64, kind: &str, boss: bool, hp: f64, max_hp: f64) -> Enemy {
    Enemy {
        id,
        kind: kind.to_string(),
        boss,
        hp,
        max_hp,
        ..Default::default()
    }
}

#[test]
fn boss_leiste() {
    let goblin = enemy(1, "goblin", false, 10.0, 10.0);
    let b = boss_bar("de", &[goblin.clone(), end_boss(50.0, 100.0, 2)]);
    let want = BossView {
        ok: true,
        id: 7,
        share: 0.5,
        name: "Kristallherz-Wächter".to_string(),
        phase: "Phase 2 von 3".to_string(),
    };
    assert_eq!(b, want, "Endboss");
    for (hp, max_hp, want) in [(0.0, 100.0, 0.0), (100.0, 100.0, 1.0), (50.0, 0.0, 0.0)] {
        assert_eq!(
            boss_bar("de", &[end_boss(hp, max_hp, 1)]).share,
            want,
            "Anteil {hp}/{max_hp}"
        );
    }
    let mini = enemy(3, "goblinLeader", true, 10.0, 10.0);
    assert_eq!(
        boss_bar("de", &[mini.clone(), end_boss(1.0, 1.0, 1)]).id,
        7,
        "Endboss vor Miniboss"
    );
    let got = boss_bar("de", std::slice::from_ref(&mini));
    let want = BossView {
        ok: true,
        id: 3,
        share: 1.0,
        name: "Goblin-Anführer".to_string(),
        phase: String::new(),
    };
    assert_eq!(got, want, "Miniboss ohne Phase");
    assert_eq!(
        boss_bar("de", &[enemy(0, "neu", true, 0.0, 0.0)]).name,
        "neu",
        "unbekannter Boss"
    );
    let got = boss_bar("en", &[end_boss(1.0, 2.0, 2)]);
    assert!(
        got.name == "Crystal Heart Warden" && got.phase == "Phase 2 of 3",
        "englisch: {got:?}"
    );
    assert!(
        !boss_bar("de", &[goblin]).ok && !boss_bar("de", &[]).ok,
        "ohne Boss keine Leiste"
    );
    assert_eq!(boss_title(&b), "Kristallherz-Wächter  ·  Phase 2 von 3", "Titel");
}

#[test]
fn event_banner_texte() {
    let ev = |id: &str, left: f64| ActiveEvent {
        id: id.to_string(),
        seconds_left: left,
    };
    let raid = Merchant {
        resource: "wood".to_string(),
        leaves: 3,
        raid: true,
        ..Default::default()
    };
    let calm = Merchant {
        resource: "wood".to_string(),
        ..Default::default()
    };
    let banner = |events: &[ActiveEvent], speed: f64, m: Option<&Merchant>| {
        event_banner(&new_events_key("de", events, speed, m))
    };
    let cases = [
        (
            "ein Event",
            banner(&[ev("fullMoon", 90.0)], 1.0, None),
            "Vollmond  ·  1:30",
        ),
        (
            "Zeitraffer",
            banner(&[ev("bloodMoon", 120.0)], 2.0, None),
            "Blutmond  ·  1:00",
        ),
        (
            "zwei",
            banner(&[ev("fullMoon", 61.0), ev("bloodMoon", 61.0)], 1.0, None),
            "Vollmond  ·  1:01\nBlutmond  ·  1:01",
        ),
        ("Überfall", banner(&[], 1.0, Some(&raid)), "Händler-Überfall"),
        ("unbekannt", banner(&[ev("neu", 5.0)], 1.0, None), "neu  ·  0:05"),
        ("keins", banner(&[], 1.0, Some(&calm)), ""),
    ];
    for (name, got, want) in cases {
        assert_eq!(got, want, "{name}");
    }
}

/// B-993: Ein Endboss ohne Text in den Übersetzungen zeigt den Namen aus bosses.json.
#[test]
fn boss_name_aus_den_werten() {
    let e = enemy(9, "forestChieftain", true, 10.0, 10.0);
    assert_eq!(boss_bar("de", &[e.clone()]).name, "Wald-Häuptling");
    assert_eq!(boss_bar("en", &[e]).name, "Wald-Häuptling");
}
