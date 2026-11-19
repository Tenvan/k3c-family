//! Port von `native/k3c-game/game/measure_load_test.go` (`TestParseMeasureLoad`) und Tests der Last-Szene über die
//! Dev-Rolle der Simulation (Go prüft `applyMeasureLoad` und `refillMeasureLoad` nur im Messlauf).

use super::*;
use crate::game::play_testutil::play_game_with;
use crate::game::testutil::FakeSim;

/// Go: `TestParseMeasureLoad`: drei Zahlen ab 0, leer = keine.
#[test]
fn parse_measure_load_liest_drei_zahlen() {
    assert_eq!(parse_measure_load(""), Ok(MeasureLoad::default()));
    assert_eq!(
        parse_measure_load("5,160,120"),
        Ok(MeasureLoad {
            build: 5,
            troops: 160,
            enemies: 120
        })
    );
    for bad in ["5", "5,1", "5,1,2,3", "a,1,2", "5,-1,2"] {
        assert!(parse_measure_load(bad).is_err(), "{bad:?}: Fehler erwartet");
    }
    let total: i64 = LOAD_TROOPS.iter().map(|s| s.1).sum();
    assert_eq!(total, 100, "Anteile der Truppen");
    let total: i64 = LOAD_ENEMIES.iter().map(|s| s.1).sum();
    assert_eq!(total, 100, "Anteile der Gegner");
}

/// Die Fehlertexte stehen wörtlich wie in Go.
#[test]
fn parse_measure_load_meldet_wie_go() {
    assert_eq!(
        parse_measure_load("5").unwrap_err(),
        "🚫 --measure-load \"5\": drei Zahlen ausbau,truppen,gegner"
    );
    assert_eq!(
        parse_measure_load("a,1,2").unwrap_err(),
        "🚫 --measure-load \"a,1,2\": drei Zahlen ab 0"
    );
}

fn dev_game() -> (Game, crate::game::testutil::Probe) {
    let (sim, probe) = FakeSim::new(2);
    play_game_with(sim.with_dev(), probe, Vec::new())
}

/// Die Last-Szene baut aus, stellt die Nacht, dann Truppen und Gegner nach ihrem Anteil (Reihenfolge wie Go).
#[test]
fn apply_baut_aus_und_stellt_die_mischungen_auf() {
    let (mut g, probe) = dev_game();
    g.measure_load("5,100,10").unwrap();
    let log = probe.dev.borrow();
    let acts: Vec<String> = log
        .actions
        .iter()
        .map(|a| format!("{}:{}:{}:{}:{}", a.action, a.kind, a.unit, a.amount, a.level))
        .collect();
    assert_eq!(
        acts,
        [
            "buildAll:::0:5",
            "night:::0:0",
            "spawn:troop:warrior:35:0",
            "spawn:troop:archer:25:0",
            "spawn:troop:peasant:25:0",
            "spawn:troop:vagrant:15:0",
            "spawnCastle:enemy:goblin:6:0",
            "spawnCastle:enemy:skeleton:4:0",
        ]
    );
    assert!(log.players.iter().all(|&p| p == 0), "{:?}", log.players);
    drop(log);
    assert_eq!(g.mload.enemies, 10);
}

/// Ohne Last (leer oder 0,0,0) geschieht nichts.
#[test]
fn apply_ohne_last_tut_nichts() {
    let (mut g, probe) = dev_game();
    g.measure_load("").unwrap();
    g.measure_load("0,0,0").unwrap();
    assert!(probe.dev.borrow().actions.is_empty());
}

/// Ohne Entwickler-Rolle meldet die Last-Szene den Fehler, ein falscher Wert den Fehlertext.
#[test]
fn apply_ohne_dev_rolle_ist_fehler() {
    let (sim, probe) = FakeSim::new(2);
    let (mut g, _) = play_game_with(sim, probe, Vec::new());
    assert_eq!(g.measure_load("1,1,1"), Err(ERR_NO_DEV.to_string()));
    assert!(g.measure_load("x").unwrap_err().starts_with("🚫 --measure-load"));
}

/// Alle 150 Takte füllt sie die Hälfte der Gegner auf, wenn weniger als die Hälfte lebt; sonst nicht.
#[test]
fn refill_fuellt_im_abstand_auf() {
    let (mut g, probe) = dev_game();
    g.mload = MeasureLoad {
        build: 0,
        troops: 0,
        enemies: 20,
    };
    g.frame.tick = 149;
    g.refill_measure_load();
    assert!(probe.dev.borrow().actions.is_empty(), "nicht im Abstand");
    g.frame.tick = 150;
    g.refill_measure_load();
    let log = probe.dev.borrow();
    let n: Vec<(String, i64)> = log.actions.iter().map(|a| (a.unit.clone(), a.amount)).collect();
    assert_eq!(n, [("goblin".to_string(), 6), ("skeleton".to_string(), 4)]);
}

/// Ohne Last-Szene (Gegner 0) füllt nichts auf.
#[test]
fn refill_ohne_gegner_tut_nichts() {
    let (mut g, probe) = dev_game();
    g.frame.tick = 150;
    g.refill_measure_load();
    assert!(probe.dev.borrow().actions.is_empty());
}
