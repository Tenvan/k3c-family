//! Tests zu `bot_boss.rs` (Port von `engine/balance/bot_boss_test.go`).

use k3c_sim::state::Enemy;

use crate::bots::Bot;
use crate::testutil::scenario_island;

/// BS11.3/AC-04: Kennzahlen und Ziele des Insel-Laufs (B-642/AC-04). Go: `TestMessgroessenEndboss` liest die Tabelle
/// `measures` und `Metrics` (RU28.1, RU28.3).
#[test]
#[ignore = "wartet auf RU28"]
fn messgroessen_endboss() {}

/// Die Ziele des Meilensteins stehen in der Datei, mit Welt standard, und die Datei lässt sich laden (Schema). Go:
/// `TestZieleInselInDerDatei` nutzt `DefaultTargets` und `LoadTargets` (RU28.3).
#[test]
#[ignore = "wartet auf RU28"]
fn ziele_insel_in_der_datei() {}

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
