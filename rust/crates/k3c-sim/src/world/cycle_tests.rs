//! Tests des Zyklus (Go: `world_cycle_test.go`, `world_cycle_night_test.go`; W12.1, B-470/AC-01, AC-03; B-889): vier Phasen
//! Tag, Dämmerung, Nacht, Morgengrauen (Q65); das Ereignis `dawn` fällt bei Beginn des Morgengrauens, der Übergang zum Tag
//! löst nichts aus. Die Tests, die `create_world` oder `step` mit Spielern brauchen, warten auf Plätze und Spieler (RU18)
//! und tragen `ignore`; `own_*` prüft `step_cycle` und `add_pool` an einer Hand-Welt.

use k3c_data::biome::Cycle;
use serde_json::Value;

use super::base::base;
use super::cycle::{add_pool, cycle_at, cycle_phases, seconds_to_night, step_cycle};
use super::testutil::*;
use super::{Options, add_player, step, vals};
use crate::state::World;

/// Go: `forestCycle`: der Wald nach Q65 mit 6/2/4/2 min, unabhängig von späteren Werten in data/.
fn forest_cycle() -> Cycle {
    Cycle {
        r#type: "dayNight".into(),
        day_minutes: 6.0,
        twilight_minutes: 2.0,
        night_minutes: 4.0,
        dawn_minutes: 2.0,
        ..Cycle::default()
    }
}

/// Go: `globalDayNight`.
fn global() -> Cycle {
    base().expect("Daten").day_night.clone()
}

#[test]
fn cycle_at_vier_phasen_grenzen_und_mitte() {
    // (Sekunden, Phase, Tag, Fortschritt, Rest)
    let cases: [(f64, &str, i64, f64, f64); 10] = [
        (0.0, "day", 1, 0.0, 360.0),
        (180.0, "day", 1, 0.5, 180.0),
        (360.0, "dusk", 1, 0.0, 120.0),
        (420.0, "dusk", 1, 0.5, 60.0),
        (480.0, "night", 1, 0.0, 240.0),
        (600.0, "night", 1, 0.5, 120.0),
        (720.0, "dawn", 2, 0.0, 120.0),
        (780.0, "dawn", 2, 0.5, 60.0),
        (840.0, "day", 2, 0.0, 360.0),
        (840.0 + 720.0, "dawn", 3, 0.0, 120.0),
    ];
    for (seconds, phase, day, progress, left) in cases {
        let got = cycle_at(&forest_cycle(), seconds);
        assert!(
            got.phase == phase
                && got.day == day
                && (got.progress - progress).abs() <= 1e-9
                && (got.seconds_left - left).abs() <= 1e-9,
            "{seconds} s: {got:?}, erwartet {phase} Tag {day} Fortschritt {progress} Rest {left}"
        );
    }
    let got = cycle_at(&forest_cycle(), 719.999);
    assert!(
        got.phase == "night" && got.day == 1,
        "kurz vor dem Morgengrauen: {got:?}"
    );
}

/// Go: `totalGold`: das Gold der Spieler plus die Münzen am Boden.
fn total_gold(w: &World) -> i64 {
    w.coins.len() as i64 + w.players.iter().map(|p| p.gold).sum::<i64>()
}

/// Go: `dawnGold` (economy.go).
fn dawn_gold(w: &World) -> i64 {
    crate::economy::dawn_gold(w, &crate::world::vals(w))
}

// Über zwei Zyklen: dawn genau einmal je Zyklus, im ersten Schritt des Morgengrauens, samt Gold je lebendem Spieler.
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn dawn_bei_beginn_des_morgengrauens() {
    let opts = Options {
        cycle_speed: 60.0,
        ..Options::default()
    };
    let mut w = new_world(0, "dawn", opts);
    w.troops.clear();
    w.camps.clear();
    w.pickups.clear();
    add_player(&mut w);
    add_player(&mut w);
    let (_, length) = cycle_phases(&global());
    let per_player = vals(&w).economy.dawn_gold_per_player;
    let (mut dawns, mut nights): (Vec<i64>, Vec<i64>) = (Vec::new(), Vec::new());
    while w.time * w.cycle_speed < 2.0 * length + 1.0 {
        let gold = total_gold(&w);
        let before = w.cycle.clone();
        step(&mut w, &[], None, DT);
        for e in &w.events {
            match e.get("type").and_then(Value::as_str) {
                Some("night") => nights.push(w.cycle.day),
                Some("dawn") => {
                    dawns.push(w.cycle.day);
                    assert!(
                        before.phase == "night"
                            && w.cycle.phase == "dawn"
                            && e.get("day").and_then(Value::as_i64) == Some(w.cycle.day),
                        "dawn bei {before:?} → {:?}, Ereignis {e:?}",
                        w.cycle
                    );
                    let got = total_gold(&w) - gold;
                    assert_eq!(got, 2 * per_player, "Tag {}: {got} Gold", w.cycle.day);
                }
                _ => {}
            }
        }
    }
    assert!(
        dawns == [2, 3] && nights.len() == 2,
        "dawn an Tagen {dawns:?}, Nächte {nights:?}, erwartet je zwei (dawn Tag 2 und 3)"
    );
    assert!(
        w.cycle.phase == "day" && w.cycle.day == 3,
        "nach zwei Zyklen {:?}, erwartet Tag 3",
        w.cycle
    );
}

// B-645: Das Morgengold wächst mit der Tiefe der Stufe (Wald: Grundeinkommen, tiefer: plus dawnGoldPerDepth je Tiefe).
#[test]
fn morgengold_nach_tiefe() {
    let eco = vals(&bare_world()).economy.clone();
    for depth in 0..=4 {
        let mut w = World::default();
        w.biome.depth = depth;
        let want = eco.dawn_gold_per_player + eco.dawn_gold_per_depth * depth;
        assert_eq!(dawn_gold(&w), want, "Tiefe {depth}");
    }
    assert!(
        eco.dawn_gold_per_depth > 0,
        "dawnGoldPerDepth muss positiv sein (Option b, wirtschaft.md § 8)"
    );
}

// SecondsToNight zählt bis zum Beginn der Nacht: am Tag über die Dämmerung, im Morgengrauen über den nächsten Tag, in der
// Nacht 0; der Zyklus-Faktor teilt (B-889).
#[test]
fn seconds_to_night_zaehlt_bis_zum_nachtbeginn() {
    let (phases, length) = cycle_phases(&global());
    let night = phases[2].start;
    let cases = [
        (0.0, 1.0, night),
        (10.0, 1.0, night - 10.0),
        (phases[3].start + 1.0, 1.0, length - phases[3].start - 1.0 + night),
        (10.0, 2.0, (night - 20.0) / 2.0),
    ];
    for (time, speed, want) in cases {
        let mut w = World {
            time,
            cycle_speed: speed,
            ..World::default()
        };
        w.cycle = cycle_at(&global(), time * speed);
        let got = seconds_to_night(&w);
        assert!(
            (got - want).abs() <= 1e-9,
            "Zeit {time} × {speed}: {got}, erwartet {want}"
        );
    }
    let mut w = World {
        time: night + 1.0,
        cycle_speed: 1.0,
        ..World::default()
    };
    w.cycle = cycle_at(&global(), night + 1.0);
    assert_eq!(seconds_to_night(&w), 0.0, "in der Nacht nicht 0");
}

#[test]
fn own_seconds_to_night_steht_bei_stillstand() {
    let w = World {
        time: 10.0,
        cycle_speed: 0.0,
        ..World::default()
    };
    assert_eq!(seconds_to_night(&w), 0.0);
}

/// Eine Hand-Welt, deren Zyklus bei `seconds` Zykluszeit steht (Tempo 1, kein Tag/Nacht-Biom: keine Wellen).
fn cycle_world(seconds: f64) -> World {
    let mut w = bare_world();
    w.cycle_speed = 1.0;
    w.time = seconds;
    w.cycle = cycle_at(&global(), seconds);
    w
}

fn event_types(w: &World) -> Vec<String> {
    w.events
        .iter()
        .filter_map(|e| e.get("type").and_then(Value::as_str).map(str::to_string))
        .collect()
}

#[test]
fn own_step_cycle_meldet_die_daemmerung_einmal() {
    let (phases, _) = cycle_phases(&global());
    let mut w = cycle_world(phases[1].start - 0.5);
    w.time = phases[1].start + 0.01; // Zeit rückt vor, der Zyklus steht noch am Tag
    step_cycle(&mut w, None, DT);
    assert_eq!(w.cycle.phase, "dusk");
    assert_eq!(event_types(&w), ["dusk"]);
    w.events.clear();
    w.time += DT;
    step_cycle(&mut w, None, DT);
    assert!(w.events.is_empty(), "dusk nur beim Übergang: {:?}", w.events);
}

#[test]
fn own_step_cycle_rechnet_zeit_mal_geschwindigkeit() {
    let (phases, _) = cycle_phases(&global());
    let mut w = cycle_world(0.0);
    w.cycle_speed = 2.0;
    w.time = (phases[1].start + 20.0) / 2.0;
    step_cycle(&mut w, None, DT);
    let want = cycle_at(&global(), phases[1].start + 20.0);
    assert_eq!(w.cycle, want);
}

/// Eine Welt mit Aggressionspool: 60 % je Minute bei Tempo 1 sind 1 % je Sekunde.
fn pool_world() -> World {
    let mut w = cycle_world(0.0);
    w.biome.cycle.r#type = "aggressionPool".into();
    w.biome.cycle.percent_per_minute = 60.0;
    w.aggression = Some(0.0);
    w
}

#[test]
fn own_step_cycle_fuellt_den_aggressionspool() {
    let mut w = pool_world();
    step_cycle(&mut w, None, 1.0);
    let a = w.aggression.unwrap_or(-1.0);
    assert!((a - 1.0).abs() <= 1e-9, "Pool {a}, erwartet 1");
}

#[test]
fn own_add_pool_kappt_bei_100_und_kennt_nur_den_pool() {
    let mut w = pool_world();
    add_pool(&mut w, 40.0);
    add_pool(&mut w, 90.0);
    assert_eq!(w.aggression, Some(100.0));
    let mut day = bare_world();
    day.aggression = Some(5.0);
    add_pool(&mut day, 10.0); // kein Pool-Biom
    assert_eq!(day.aggression, Some(5.0));
    let mut none = pool_world();
    none.aggression = None;
    add_pool(&mut none, 10.0);
    assert_eq!(none.aggression, None);
}

#[test]
fn own_add_pool_friert_bei_lebenden_gegnern_und_nach_der_welle_ein() {
    let mut w = pool_world();
    w.wave = 1;
    w.last_wave_at = 0.0;
    w.time = 100.0; // innerhalb des Mindestabstands von 240 s
    add_pool(&mut w, 10.0);
    assert_eq!(w.aggression, Some(0.0), "Mindestabstand nach der Welle");
    w.time = 300.0;
    add_pool(&mut w, 10.0);
    assert_eq!(w.aggression, Some(10.0));
    let i = spawn(&mut w, "goblin", 10.0);
    add_pool(&mut w, 10.0);
    assert_eq!(w.aggression, Some(10.0), "ein lebender Gegner friert den Pool ein");
    w.enemies[i].guard = true; // Wachen zählen nicht
    add_pool(&mut w, 10.0);
    assert_eq!(w.aggression, Some(20.0));
    w.spawn_queue.push(crate::state::QueuedSpawn::default());
    add_pool(&mut w, 10.0);
    assert_eq!(w.aggression, Some(20.0), "wartende Gegner frieren ein");
}
