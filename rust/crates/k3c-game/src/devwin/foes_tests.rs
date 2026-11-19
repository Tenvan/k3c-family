use k3c_devproto::{DevCommand, Msg};

use super::super::commands::handle;
use super::super::mock::MockHost;

fn dev(host: &mut MockHost, c: DevCommand) -> bool {
    match handle(
        host,
        Msg::Dev {
            id: 1,
            player: 0,
            action: c,
        },
    ) {
        Some(Msg::Ack { ok, .. }) => ok,
        other => panic!("kein ack: {other:?}"),
    }
}

fn act(action: &str) -> DevCommand {
    DevCommand {
        action: action.into(),
        ..DevCommand::default()
    }
}

#[test]
fn welle_nacht_boss_ereignis() {
    let mut h = MockHost::new(1);
    assert!(dev(&mut h, act("wave")));
    assert!(dev(&mut h, act("night")));
    assert!(dev(&mut h, act("bossPhase")));
    assert!(dev(
        &mut h,
        DevCommand {
            boss: "b1".into(),
            ..act("boss")
        }
    ));
    assert!(dev(
        &mut h,
        DevCommand {
            event: "fullMoon".into(),
            ..act("event")
        }
    ));
    assert_eq!(h.log().len(), 5);
    assert_eq!(h.log()[3].1.boss, "b1");
    assert!(!dev(&mut h, act("boss")), "ohne Boss");
    assert!(!dev(&mut h, act("event")), "ohne Ereignis");
    assert_eq!(h.log().len(), 5);
}

#[test]
fn erzeugen_prueft_art_sorte_anzahl() {
    let mut h = MockHost::new(1);
    let spawn = |action: &str, kind: &str, unit: &str, amount: i64| DevCommand {
        kind: kind.into(),
        unit: unit.into(),
        amount,
        ..act(action)
    };
    assert!(dev(&mut h, spawn("spawn", "enemy", "goblin", 5)));
    assert!(dev(&mut h, spawn("spawnCastle", "troop", "archer", 200)));
    assert!(!dev(&mut h, spawn("spawn", "tier", "goblin", 5)));
    assert!(!dev(&mut h, spawn("spawn", "enemy", "", 5)));
    assert!(!dev(&mut h, spawn("spawn", "enemy", "goblin", 0)));
    assert!(!dev(&mut h, spawn("spawn", "enemy", "goblin", 201)));
    assert_eq!(h.log().len(), 2);
}
