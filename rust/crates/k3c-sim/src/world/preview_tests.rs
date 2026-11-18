//! Tests der Vorschau (Go: `preview_test.go`, WZ7.8, B-460/AC-03, AC-08). Die Tests, die eine Welt mit `create_world`
//! bauen, brauchen den Zyklus (RU14.2), Plätze, Ruinen und Spieler (RU18) und `dev_build_site` (RU19) und tragen `ignore`;
//! die Marke nennt den letzten Sprint, der fehlt. `built_wall_world` ersetzt dort, wo nur ein gebauter Platz gebraucht
//! wird, den Weg über `create_world` und `preview_hub_level` (Abweichung von Go, die Prüfung bleibt dieselbe).

use super::base::base;
use super::cycle::{cycle_at, cycle_phases};
use super::preview::{SiteLook, preview_cycle, preview_set_site};
use super::testutil::*;
use super::{Options, vals};
use crate::state::World;

/// Go: `phaseSeconds`: Beginn und Länge einer Sim-Phase im globalen Zyklus.
fn phase_seconds(phase: &str) -> (f64, f64) {
    let (phases, _) = cycle_phases(&base().expect("Daten").day_night);
    match phases.iter().find(|p| p.name == phase) {
        Some(p) => (p.start, p.length),
        None => (0.0, phases[0].length),
    }
}

fn preview_json(w: &World) -> String {
    serde_json::to_string(w).expect("JSON")
}

fn day_night() -> k3c_data::biome::Cycle {
    base().expect("Daten").day_night.clone()
}

const PHASES: [&str; 4] = ["day", "dusk", "night", "dawn"];

#[test]
#[ignore = "wartet auf RU18"]
fn preview_cycle_setzt_phase_und_fortschritt() {
    for phase in PHASES {
        for p in [0.0, 0.25, 0.5, 0.9, 0.999] {
            let mut w = new_world(0, "x", Options::default());
            preview_cycle(&mut w, phase, p).unwrap_or_else(|e| panic!("{phase} {p}: {e}"));
            assert!(
                w.cycle.phase == phase && (w.cycle.progress - p).abs() <= 1e-9,
                "{phase} {p}: {:?}",
                w.cycle
            );
            let (start, length) = phase_seconds(phase);
            let got = w.time * w.cycle_speed;
            assert!(
                (got - (start + p * length)).abs() <= 1e-6,
                "{phase} {p}: Zykluszeit {got}"
            );
            let c = cycle_at(&day_night(), w.time * w.cycle_speed);
            assert!(
                c.phase == phase && (c.progress - p).abs() <= 1e-6,
                "{phase} {p}: cycle_at liefert {c:?}"
            );
            let want = (1.0 - p) * length;
            assert!(
                (w.cycle.seconds_left - want).abs() <= 1e-6,
                "{phase} {p}: SecondsLeft {}",
                w.cycle.seconds_left
            );
        }
    }
}

#[test]
#[ignore = "wartet auf RU18"]
fn preview_cycle_behaelt_den_tag_und_rechnet_mit_zyklus_geschwindigkeit() {
    let mut w = new_world(
        0,
        "x",
        Options {
            cycle_speed: 4.0,
            ..Options::default()
        },
    );
    let (_, length) = cycle_phases(&day_night());
    w.time = (2.0 * length + 10.0) / 4.0; // dritter Tag
    preview_cycle(&mut w, "night", 0.5).expect("Vorschau");
    assert_eq!(w.cycle.day, 3);
    let c = cycle_at(&day_night(), w.time * w.cycle_speed);
    assert!(c.phase == "night" && c.day == 3, "cycle_at aus w.time: {c:?}");
}

#[test]
#[ignore = "wartet auf RU18"]
fn preview_cycle_lehnt_ungueltiges_ab() {
    let cases: [(&str, &str, f64); 8] = [
        ("Dawn", "Dawn", 0.0),
        ("leer", "", 0.0),
        ("unbekannt", "noon", 0.5),
        ("negativ", "day", -0.1),
        ("eins", "night", 1.0),
        ("groesser", "dusk", 1.5),
        ("nan", "day", f64::NAN),
        ("inf", "day", f64::INFINITY),
    ];
    for (name, phase, progress) in cases {
        let mut w = new_world(0, "x", Options::default());
        let before = preview_json(&w);
        assert!(preview_cycle(&mut w, phase, progress).is_err(), "{name}: kein Fehler");
        assert_eq!(before, preview_json(&w), "{name}: Welt nach Fehler verändert");
    }
}

#[test]
#[ignore = "wartet auf RU18"]
fn preview_cycle_geschwindigkeiten_und_phasenbeginn() {
    // Nicht-Zweierpotenz-Geschwindigkeiten und spätere Tage: Phasenbeginn (Fortschritt 0) bleibt in der gesetzten Phase.
    let (_, length) = cycle_phases(&day_night());
    for speed in [0.1, 3.0, 7.0, 50.0] {
        for phase in PHASES {
            for today in [0.0, 1.0, 5.0] {
                let opts = Options {
                    cycle_speed: speed,
                    ..Options::default()
                };
                let mut w = new_world(0, "x", opts);
                w.time = (today * length + 5.0) / speed;
                preview_cycle(&mut w, phase, 0.0).expect("Vorschau");
                let c = cycle_at(&day_night(), w.time * w.cycle_speed);
                let want = today as i64 + 1 + i64::from(phase == "dawn"); // das Morgengrauen gehört zum nächsten Tag
                let ok = c.phase == phase && c.day == want && w.cycle.day == c.day;
                assert!(
                    ok,
                    "Tempo {speed}, {phase}, Tag {}: cycle_at {c:?}, Cycle {:?}",
                    today + 1.0,
                    w.cycle
                );
            }
        }
    }
}

#[test]
#[ignore = "wartet auf RU18"]
fn preview_cycle_ohne_zyklus_geschwindigkeit_ist_fehler() {
    let mut w = new_world(0, "x", Options::default());
    w.cycle_speed = 0.0;
    let before = preview_json(&w);
    assert!(
        preview_cycle(&mut w, "night", 0.5).is_err(),
        "CycleSpeed 0: kein Fehler"
    );
    assert_eq!(preview_json(&w), before, "Welt nach Fehler verändert");
}

#[test]
fn preview_set_site_stufe_und_trefferpunkte() {
    let (mut w, si) = built_wall_world();
    preview_set_site(
        &mut w,
        si,
        &SiteLook {
            level: 2,
            hp_percent: 50,
            unbuilt: false,
        },
    )
    .expect("Stufe 2 mit 50 %");
    let want = vals(&w).buildings["wall"].levels[1].hp;
    let wall = &w.sites[si];
    assert!(
        wall.level == 2 && wall.max_hp == want && wall.hp == want / 2.0,
        "Mauer Stufe 2 mit 50 %: Level {} HP {}/{}, erwartet MaxHP {want}",
        wall.level,
        wall.hp,
        wall.max_hp
    );
    let level99 = SiteLook {
        level: 99,
        ..SiteLook::default()
    };
    let hp101 = SiteLook {
        hp_percent: 101,
        ..SiteLook::default()
    };
    assert!(
        preview_set_site(&mut w, si, &level99).is_err() && preview_set_site(&mut w, si, &hp101).is_err(),
        "Stufe 99 und 101 % hätten abgelehnt werden müssen"
    );
}

#[test]
fn preview_set_site_nicht_gebaut() {
    let (mut w, si) = built_wall_world();
    let unbuilt = SiteLook {
        unbuilt: true,
        ..SiteLook::default()
    };
    preview_set_site(&mut w, si, &unbuilt).expect("nicht gebaut");
    assert!(
        w.sites[si].state == "unpaid" && w.sites[si].hp == 0.0,
        "Zustand {:?}",
        w.sites[si].state
    );
    let level2 = SiteLook {
        level: 2,
        ..SiteLook::default()
    };
    preview_set_site(&mut w, si, &level2).expect("an einem leeren Platz ignoriert");
    assert_eq!(w.sites[si].state, "unpaid");
}

#[test]
fn own_preview_set_site_meldungen_und_ruine() {
    let (mut w, si) = built_wall_world();
    let levels = vals(&w).buildings["wall"].levels.len();
    let err = preview_set_site(
        &mut w,
        si,
        &SiteLook {
            level: 99,
            ..SiteLook::default()
        },
    )
    .expect_err("Stufe 99");
    assert_eq!(
        err,
        format!("vorschau: Baustufe 99 außerhalb von 1 bis {levels} (wall)")
    );
    let err = preview_set_site(
        &mut w,
        si,
        &SiteLook {
            hp_percent: 101,
            ..SiteLook::default()
        },
    )
    .expect_err("101 %");
    assert_eq!(err, "vorschau: Trefferpunkte 101 % außerhalb von 1 bis 100 (wall)");
    assert!(
        preview_set_site(&mut w, 99, &SiteLook::default()).is_err(),
        "Platz fehlt"
    );
    w.sites[si].ruin = true; // eine Ruine behält ihre Kennung wie bei destroySite
    let id = w.sites[si].id;
    let unbuilt = SiteLook {
        unbuilt: true,
        ..SiteLook::default()
    };
    preview_set_site(&mut w, si, &unbuilt).expect("nicht gebaut");
    let s = &w.sites[si];
    assert!(s.ruin && s.id == id && s.kind == "wall", "Kennung verloren: {s:?}");
}

#[test]
fn site_look_json_wie_go() {
    let look: SiteLook = serde_json::from_str(r#"{"level":2,"hpPercent":50}"#).expect("lesen");
    assert_eq!((look.level, look.hp_percent, look.unbuilt), (2, 50, false));
    assert_eq!(
        serde_json::to_string(&look).expect("schreiben"),
        r#"{"level":2,"hpPercent":50,"unbuilt":false}"#
    );
}
