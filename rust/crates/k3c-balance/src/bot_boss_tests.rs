//! Tests zu `bot_boss.rs` (Port von `engine/balance/bot_boss_test.go`).

use k3c_sim::state::Enemy;

use crate::bots::Bot;
use crate::metrics::{DayMetrics, Metrics};
use crate::metrics_island::StageMetrics;
use crate::targets::{default_targets, load_targets, measure_values};
use crate::testutil::scenario_island;

/// BS11.3/AC-04: Kennzahlen und Ziele des Insel-Laufs (B-642/AC-04).
#[test]
fn messgroessen_endboss() {
    let mut m = Metrics {
        endboss_tick: Some(120),
        endboss_day: Some(17),
        stages: vec![
            StageMetrics::default(),
            StageMetrics {
                arrival_day: Some(5),
                ..StageMetrics::default()
            },
        ],
        ..Metrics::default()
    };
    assert_eq!(measure_values("endbossDefeated", &m), vec![1.0]);
    assert_eq!(measure_values("endbossDay", &m), vec![17.0]);
    assert_eq!(measure_values("caveBeforeDay6", &m), vec![1.0]);
    m.endboss_tick = None;
    m.endboss_day = None;
    m.stages[1].arrival_day = Some(6);
    assert_eq!(measure_values("endbossDefeated", &m), vec![0.0], "nicht besiegt");
    m.days = vec![DayMetrics::default(); 41];
    assert_eq!(
        measure_values("endbossDay", &m),
        vec![42.0],
        "ohne Sieg zählt der Tag nach dem Lauf"
    );
    assert_eq!(measure_values("caveBeforeDay6", &m), vec![0.0], "Höhle erst Tag 6");
}

/// Die Ziele des Meilensteins stehen in der Datei, mit Welt standard, und die Datei lässt sich laden (Schema).
#[test]
fn ziele_insel_in_der_datei() {
    let ts = default_targets().expect("Ziele");
    let found = |measure: &str| {
        ts.targets
            .iter()
            .any(|g| g.world == "standard" && g.bot == "coop2" && g.days == 40 && g.measure == measure)
    };
    for name in ["endbossDefeated", "endbossDay"] {
        assert!(found(name), "Ziel {name} mit Welt standard fehlt");
    }
    // Rolle und Gruppe prüft schon das Laden (`check_roles`), darum trägt das Ziel beide.
    let bad = r#"{"version":1,"margin":{},"seeds":["1"],"targets":[{"measure":"endbossDefeated","group":"basis","name":"x","kind":"share","players":2,"bot":"coop2","depth":0,"days":1,"world":"gibtsnicht","role":"goal","lower":1,"upper":2,"rule":"r"}]}"#;
    assert!(load_targets(bad).is_err(), "unbekannte Welt muss abgelehnt werden");
}

/// Bosskampf: Ein verletzter Bot zieht sich zur Burg zurück, ein Bot neben einem Gegner schlägt zu.
#[test]
fn bosskampf_rueckzug_und_schlag() {
    let mut isl = scenario_island("1", 1);
    let bot = Bot::BossFight {
        inner: Box::new(Bot::Passive),
        deepest: 4,
    };
    let hub_x = isl.stages[0].hub_x;
    isl.stages[0].players[0].x = hub_x + 20.0;
    isl.stages[0].players[0].gold = 0;
    let (w, p) = (&isl.stages[0], &isl.stages[0].players[0]);
    let cmd = bot.decide(&isl, w, p);
    assert!(cmd.move_x < 0.0, "Rückzug zur Burg erwartet: {cmd:?}");

    isl.stages[0].players[0].gold = 100;
    let x = isl.stages[0].players[0].x;
    isl.stages[0].enemies.push(Enemy {
        id: 99,
        x: x + 1.0,
        hp: 5.0,
        max_hp: 5.0,
        ..Enemy::default()
    });
    let (w, p) = (&isl.stages[0], &isl.stages[0].players[0]);
    let cmd = bot.decide(&isl, w, p);
    assert!(cmd.attack, "Schlag erwartet: {cmd:?}");
}
