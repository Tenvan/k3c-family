use k3c_devproto::{DevCommand, Msg};

use super::super::commands::handle;
use super::super::mock::MockHost;

fn dev(host: &mut MockHost, player: i64, action: &str, unit: &str) -> (bool, String) {
    let action = DevCommand {
        action: action.into(),
        unit: unit.into(),
        amount: 1,
        ..DevCommand::default()
    };
    match handle(host, Msg::Dev { id: 1, player, action }) {
        Some(Msg::Ack { ok, err, .. }) => (ok, err.unwrap_or_default()),
        other => panic!("kein ack: {other:?}"),
    }
}

#[test]
fn lernen_und_zuruecksetzen_ueber_die_session_pfade() {
    let mut h = MockHost::new(2);
    assert!(dev(&mut h, 1, "learn", "heal").0);
    assert!(dev(&mut h, 0, "respec", "").0);
    assert_eq!(h.learned, [(1, "heal".to_string())]);
    assert_eq!(h.respecs, [0]);
    assert!(h.log().is_empty(), "keine Dev-Aktion der Sim");
}

#[test]
fn ohne_skill_und_skillpunkte_ack_false() {
    let mut h = MockHost::new(1);
    assert!(!dev(&mut h, 0, "learn", "").0);
    let (ok, err) = dev(&mut h, 0, "skillPoints", "");
    assert!(!ok && err.contains("Ticket"));
    assert!(h.learned.is_empty() && h.log().is_empty());
    assert!(!dev(&mut h, 3, "respec", "").0, "Spieler 4 gibt es nicht");
}
