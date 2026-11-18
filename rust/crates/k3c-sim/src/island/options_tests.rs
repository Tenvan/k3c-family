//! Raum-Optionen, Grade und Wellenfaktor der Insel (Go: `island_options_test.go`, SP13.1, B-101). Die sieben Go-Tests bauen
//! eine Insel (`must_island`) und rechnen Wellen: Sie warten auf RU14 (`create_world_with`, `step`). Die Tests `own_*`
//! (ohne Go-Gegenstück) prüfen, was ohne Stufenaufbau läuft: Standards, Prüfung, `set_options`, `set_grade`, Wellenfaktor.

use std::sync::Arc;

use k3c_data::difficulty::GRADE_NAMES;
use k3c_data::{Data, Source, load_all};
use serde_json::Value;

use super::options::{default_options, default_options_for, scaled, set_grade, set_options, validate_options};
use super::testutil::*;
use super::{Island, IslandOptions, add_island_player, step_island};
use crate::enemies::waves::start_wave;
use crate::enemies::{spawn_enemy, step_spawns};
use crate::state::{PlayerCommand, World, WorldValues};

fn opts(grade: &str, goal: &str, defeat: &str) -> IslandOptions {
    IslandOptions {
        grade: grade.into(),
        goal: goal.into(),
        defeat: defeat.into(),
    }
}

/// Go: `islandWith` baut eine Insel mit `n` Spielern und Grad.
fn island_with(data: &Data, seed: &str, grade: &str, n: usize) -> Island {
    let mut isl = must_island(data, seed, &[0]);
    set_grade(&mut isl, grade, true).expect("Grad");
    for _ in 0..n {
        add_island_player(&mut isl, 0);
    }
    isl
}

/// Go: `waveCount` startet eine Welle (vorherige Welle = wave-1) und liefert die Gegnerzahl des wave-Ereignisses.
fn wave_count(isl: &mut Island, data: &Data, wave: i64) -> i64 {
    isl.stages[0].wave = wave - 1;
    isl.stages[0].events.clear();
    let mut r = isl.stage_ref(0, &data.difficulty);
    start_wave(&mut isl.stages[0], Some(&mut r));
    isl.put_stage_ref(r);
    isl.stages[0]
        .events
        .iter()
        .find(|e| e.get("type").and_then(Value::as_str) == Some("wave"))
        .and_then(|e| e.get("count").and_then(Value::as_i64))
        .unwrap_or(-1)
}

/// Lässt alle wartenden Gegner der Stufe 0 fällig werden (Go: `w.Time = t; stepSpawns(w)`).
fn spawn_due(isl: &mut Island, data: &Data, time: f64) {
    isl.stages[0].time = time;
    let r = isl.stage_ref(0, &data.difficulty);
    let v: Arc<WorldValues> = isl.stages[0].vals.clone().expect("Wertesatz der Stufe");
    step_spawns(&mut isl.stages[0], &v, Some(&r));
    isl.put_stage_ref(r);
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_options_wave_size_by_players() {
    let data = test_data();
    let base = wave_count(&mut island_with(&data, "opt-a", "normal", 1), &data, 1);
    assert!(base > 0, "Basiswelle leer: {base}");
    for n in 1..=4 {
        let want = (base as f64 * (1.0 + data.waves.per_extra_player * (n - 1) as f64)).round() as i64;
        let got = wave_count(&mut island_with(&data, "opt-a", "normal", n), &data, 1);
        assert_eq!(got, want, "{n} Spieler, Welle 1: {got} Gegner, erwartet {want}");
    }
    let b6 = wave_count(&mut island_with(&data, "opt-a", "normal", 1), &data, 6);
    let got = wave_count(&mut island_with(&data, "opt-a", "normal", 4), &data, 6);
    let min = (b6 as f64 * (1.0 + 3.0 * data.waves.per_extra_player)) as i64 - 1;
    assert!(got >= min, "Welle 6 mit 4 Spielern: {got}, erwartet mindestens {min}");
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_options_grade_factors() {
    let data = test_data();
    let mut reference = island_with(&data, "opt-b", "normal", 1);
    let ref_count = wave_count(&mut reference, &data, 1);
    let ref_enemy = spawn_enemy(&mut reference.stages[0], "goblin", 0.0, None);
    for g in GRADE_NAMES {
        let mut isl = island_with(&data, "opt-b", g, 1);
        let f = &data.difficulty.grades[g];
        let (got, want) = (
            wave_count(&mut isl, &data, 1),
            (ref_count as f64 * f.wave_size).round() as i64,
        );
        assert_eq!(got, want, "{g}: {got} Gegner, erwartet {want}");
        isl.stages[0].enemies.clear();
        spawn_due(&mut isl, &data, 1e6);
        let e = &isl.stages[0].enemies[0];
        let base = &data.enemies[&e.kind];
        assert_eq!(e.max_hp, (base.hp * f.enemy_hp).round(), "{g}: HP {}", e.max_hp);
        assert_eq!(
            e.damage,
            (base.damage * f.enemy_damage).round(),
            "{g}: Schaden {}",
            e.damage
        );
    }
    let goblin = &data.enemies["goblin"];
    let e = &reference.stages[0].enemies[ref_enemy];
    assert!(
        e.max_hp == goblin.hp && e.damage == goblin.damage,
        "spawn_enemy ohne Grad muss die Basiswerte liefern"
    );
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_options_grade_change_applies_next_wave() {
    let data = test_data();
    let mut isl = island_with(&data, "opt-c", "hard", 1);
    wave_count(&mut isl, &data, 1);
    let queued = isl.stages[0].spawn_queue.len();
    set_grade(&mut isl, "easy", false).expect("Grad easy");
    spawn_due(&mut isl, &data, 1e6);
    assert_eq!(isl.stages[0].enemies.len(), queued, "Gegner der Warteschlange");
    let hard = &data.difficulty.grades["hard"];
    let e = &isl.stages[0].enemies[0];
    assert_eq!(
        e.max_hp,
        (data.enemies[&e.kind].hp * hard.enemy_hp).round(),
        "wartender Gegner hat nach dem Gradwechsel andere HP: {}",
        e.max_hp
    );
    isl.stages[0].enemies.clear();
    wave_count(&mut isl, &data, 2);
    spawn_due(&mut isl, &data, 2e6);
    let easy = &data.difficulty.grades["easy"];
    let e = &isl.stages[0].enemies[0];
    assert_eq!(
        e.max_hp,
        (data.enemies[&e.kind].hp * easy.enemy_hp).round(),
        "nächste Welle nutzt nicht den neuen Grad: {}",
        e.max_hp
    );
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_options_set_options() {
    let data = test_data();
    let mut isl = must_island(&data, "opt-d", &[0]);
    assert!(
        isl.options == default_options(&data.difficulty) && isl.options.defeat == "resources",
        "Standard falsch: {:?}",
        isl.options
    );
    set_options_und_grade_regeln(&mut isl, &data);
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_options_save_round_trip() {
    let data = test_data();
    let mut isl = must_island(&data, "opt-e", &[0, 1]);
    set_options(&mut isl, opts("hard", "gold", "stage"), false).expect("Optionen");
    let got = load_island(&data, &save_json(&isl, "t"));
    assert_eq!(got.options, isl.options, "Optionen nach dem Laden");
    // Stand ohne Optionen (Feld fehlt) lädt mit Standard
    let mut m: Value = serde_json::from_str(&save_json(&isl, "t")).expect("JSON");
    m.as_object_mut().expect("Objekt").remove("options");
    let got = load_island(&data, &m.to_string());
    assert_eq!(got.options, default_options(&data.difficulty), "Stand ohne Optionen");
    // Stand der Version 1 (Campaign) ebenfalls: `island_from_v1` setzt den Standard (`save_tests.rs`).
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_options_deterministic() {
    let data = test_data();
    let run = || {
        let mut isl = island_with(&data, "opt-f", "ultra", 3);
        let cmds = vec![PlayerCommand::default(); 3];
        for _ in 0..(60.0 / DT) as usize {
            step_island(&mut isl, &cmds, &data.difficulty, DT);
        }
        island_json(&isl)
    };
    assert_eq!(run()[0], run()[0], "gleiche Eingaben, gleicher Seed: Zustand weicht ab");
}

#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn island_options_world_without_island_unchanged() {
    // Go: `CreateWorld(biomeForDepth(0), "opt-g", Options{CycleSpeed: islandSpeed})`, `startWave(w)`; jede Warteschlange
    // hat die Faktoren 1/1 (ohne Insel keine Skalierung).
    let mut w = crate::enemies::testutil::quiet_world();
    start_wave(&mut w, None);
    for s in &w.spawn_queue {
        assert!(
            s.hp_factor == 1.0 && s.damage_factor == 1.0,
            "Welt ohne Insel: Faktor {}/{}",
            s.hp_factor,
            s.damage_factor
        );
    }
}

/// Die Prüfungen von `island_options_set_options` ab der Standard-Insel.
fn set_options_und_grade_regeln(isl: &mut Island, data: &Data) {
    for bad in [
        opts("x", "endboss", "stage"),
        opts("normal", "x", "stage"),
        opts("normal", "endboss", "x"),
    ] {
        assert!(
            set_options(isl, bad.clone(), true).is_err(),
            "{bad:?} sollte abgelehnt werden"
        );
    }
    assert!(
        set_grade(isl, "dev", false).is_err(),
        "dev ohne Dev-Mode muss abgelehnt werden"
    );
    let r = set_grade(isl, "ultra", false);
    assert!(
        r.is_ok() && isl.options.defeat == "resources",
        "Gradwechsel ändert den Niederlage-Modus nicht: {r:?} {:?}",
        isl.options
    );
    assert!(
        default_options_for("ultra", &data.difficulty).defeat == "stage"
            && default_options_for("easy", &data.difficulty).defeat == "resources",
        "Standards je Grad falsch"
    );
}

// ---- Tests ohne Go-Gegenstück: laufen ohne `create_world_with` ----

fn data() -> Data {
    load_all(&Source::Embedded).expect("eingebettete Daten")
}

#[test]
fn own_standard_optionen_und_standards_je_grad() {
    let d = data();
    assert_eq!(default_options(&d.difficulty), opts("normal", "endboss", "resources"));
    let want = [
        ("dev", "resources"),
        ("easy", "resources"),
        ("normal", "resources"),
        ("hard", "stage"),
        ("ultra", "stage"),
    ];
    for (g, w) in want {
        let o = default_options_for(g, &d.difficulty);
        assert_eq!(
            (o.grade.as_str(), o.goal.as_str(), o.defeat.as_str()),
            (g, "endboss", w),
            "{g}"
        );
    }
    // Unbekannter Grad: Normal.
    assert_eq!(default_options_for("x", &d.difficulty), default_options(&d.difficulty));
}

#[test]
fn own_validate_options_nennt_den_wert_und_das_feld() {
    assert!(validate_options(&opts("hard", "gold", "stage")).is_ok());
    assert_eq!(
        validate_options(&opts("x", "endboss", "stage")).unwrap_err(),
        "insel: unbekannter Wert \"x\" für Grad"
    );
    assert_eq!(
        validate_options(&opts("normal", "x", "stage")).unwrap_err(),
        "insel: unbekannter Wert \"x\" für Ziel"
    );
    assert_eq!(
        validate_options(&opts("normal", "endboss", "")).unwrap_err(),
        "insel: unbekannter Wert \"\" für Niederlage-Modus"
    );
}

#[test]
fn own_set_options_und_set_grade_ohne_stufenaufbau() {
    let d = data();
    let mut isl = Island {
        options: default_options(&d.difficulty),
        ..Island::default()
    };
    set_options_und_grade_regeln(&mut isl, &d);
    set_options(&mut isl, opts("dev", "days", "stage"), true).expect("dev im Dev-Mode");
    assert_eq!(isl.options, opts("dev", "days", "stage"));
    // Ein abgelehntes Setzen lässt die Optionen unverändert.
    assert!(set_options(&mut isl, opts("normal", "x", "stage"), true).is_err());
    assert_eq!(isl.options, opts("dev", "days", "stage"));
}

#[test]
fn own_wave_factors_der_insel_nutzen_den_weg_der_stufe() {
    let d = data();
    let mut isl = Island {
        options: opts("hard", "endboss", "stage"),
        ..Island::default()
    };
    let g = &d.difficulty.grades["hard"];
    // Keine Stufe, kein Spieler: Wellenzuschlag 0, Faktor des Grads.
    assert_eq!(
        isl.wave_factors(&d.difficulty),
        (g.wave_size, g.enemy_hp, g.enemy_damage)
    );
    // Der Bezug der Stufe rechnet gleich (eine Funktion, nicht zwei Kopien).
    isl.stages.push(World {
        vals: Some(Arc::new(WorldValues::default())),
        ..World::default()
    });
    let r = isl.stage_ref(0, &d.difficulty);
    assert_eq!(r.wave_factors(), isl.wave_factors(&d.difficulty));
}

#[test]
fn own_scaled_rundet_und_haelt_mindestens_eins() {
    assert_eq!(scaled(0, 2.0), 0);
    assert_eq!(scaled(7, 1.0), 7);
    assert_eq!(scaled(5, 1.5), 8, "7,5 rundet weg von 0");
    assert_eq!(scaled(1, 0.1), 1, "mindestens 1 bei n > 0");
}
