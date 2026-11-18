//! Ursache im Ereignis `crownFallen` (Go: `cause_test.go`, B-182, W0 › AC-07). Die Tests rechnen ganze Schritte
//! (`step`) mit Spielern (`add_player`) und warten auf die Teilschritte von RU18 und RU19 (`pending.rs`).

use serde_json::Value;

use super::testutil::*;
use super::{add_player, step};
use crate::enemies::testutil::spawn;
use crate::state::common::{CAUSE_OTHER, apply_damage};
use crate::state::{Event, PlayerCommand, World};

/// Go: `eventsOf` (events_test.go): rechnet `steps` Ticks und sammelt die Ereignisse der Art `typ`.
fn events_of(w: &mut World, typ: &str, steps: usize, commands: &[PlayerCommand]) -> Vec<Event> {
    let mut found = Vec::new();
    for _ in 0..steps {
        step(w, commands, None, DT);
        found.extend(
            w.events
                .iter()
                .filter(|ev| ev.get("type") == Some(&Value::from(typ)))
                .cloned(),
        );
    }
    found
}

/// Go: `downCause`: lässt Gegner der Art `kind` bei Spieler 1 (ohne Gold) angreifen und liefert sein `crownFallen`.
fn down_cause(kind: &str, dx: f64) -> Event {
    let mut w = quiet_world();
    let (p0, p1) = (add_player(&mut w), add_player(&mut w));
    let hub = w.hub_x;
    w.players[p0].x = hub + 40.0;
    w.players[p1].x = hub - 40.0;
    w.players[p1].gold = 0;
    let at = w.players[p1].x + dx;
    spawn(&mut w, kind, at);
    let downs = events_of(
        &mut w,
        "crownFallen",
        30 * 10,
        &[PlayerCommand::default(), PlayerCommand::default()],
    );
    assert_eq!(
        downs.len(),
        1,
        "{kind}: erwartet genau ein crownFallen (Spieler 1), erhalten {downs:?}"
    );
    downs[0].clone()
}

#[test]
#[ignore = "wartet auf RU19"]
fn cause_nahkampf_nennt_gegnerart() {
    let ev = down_cause("goblin", -0.5);
    assert!(ev["player"] == 1 && ev["cause"] == "goblin", "Nahkampf: {ev:?}");
}

#[test]
#[ignore = "wartet auf RU19"]
fn cause_geschoss_nennt_gegnerart() {
    let ev = down_cause("goblinArcher", -6.0);
    assert!(ev["player"] == 1 && ev["cause"] == "goblinArcher", "Geschoss: {ev:?}");
}

#[test]
#[ignore = "wartet auf RU19"]
fn cause_ohne_gegner_fester_wert() {
    let mut w = quiet_world();
    add_player(&mut w);
    let p1 = add_player(&mut w);
    // Go: `kill` (crown_helpers_test.go): Gold, Schild und Last Stand weg, ein Schadenspunkt.
    let p = &mut w.players[p1];
    (p.gold, p.shield, p.last_stand_for) = (0, 0.0, 0.0);
    let id = p.id;
    apply_damage(&mut w, id, 1.0);
    let ev = w
        .events
        .iter()
        .find(|ev| ev.get("type") == Some(&Value::from("crownFallen")))
        .expect("kein Ereignis");
    assert!(ev["player"] == 1 && ev["cause"] == CAUSE_OTHER, "ohne Gegner: {ev:?}");
}
