use k3c_devproto::{DevCommand, Msg};

use super::super::commands::handle;
use super::super::mock::MockHost;

fn dev(host: &mut MockHost, player: i64, c: DevCommand) -> bool {
    match handle(
        host,
        Msg::Dev {
            id: 1,
            player,
            action: c,
        },
    ) {
        Some(Msg::Ack { ok, .. }) => ok,
        other => panic!("kein ack: {other:?}"),
    }
}

fn gold(amount: i64) -> DevCommand {
    DevCommand {
        action: "gold".into(),
        amount,
        ..DevCommand::default()
    }
}

fn material(resource: &str, amount: i64) -> DevCommand {
    DevCommand {
        action: "material".into(),
        resource: resource.into(),
        amount,
        ..DevCommand::default()
    }
}

#[test]
fn gold_und_material_erreichen_die_session() {
    let mut h = MockHost::new(2);
    assert!(dev(&mut h, 1, gold(500)));
    assert!(dev(&mut h, 0, material("crystal", 50)));
    assert_eq!(h.log().len(), 2);
    assert_eq!((h.log()[0].0, h.log()[0].1.amount), (1, 500));
    assert_eq!(h.log()[1].1.resource, "crystal");
}

#[test]
fn ausbau_lager_burgfall() {
    let mut h = MockHost::new(1);
    for action in ["maxStock", "fillStock", "castleFall"] {
        let c = DevCommand {
            action: action.into(),
            ..DevCommand::default()
        };
        assert!(dev(&mut h, 0, c), "{action}");
    }
    let cheap = DevCommand {
        action: "cheapBuild".into(),
        on: Some(true),
        ..DevCommand::default()
    };
    assert!(dev(&mut h, 0, cheap));
    for level in 1..=5 {
        let c = DevCommand {
            action: "buildAll".into(),
            level,
            ..DevCommand::default()
        };
        assert!(dev(&mut h, 0, c));
    }
    assert_eq!(h.log().len(), 9);
}

#[test]
fn ungueltige_werte_ack_false_ohne_session_aufruf() {
    let mut h = MockHost::new(1);
    assert!(!dev(&mut h, 0, gold(0)));
    assert!(!dev(&mut h, 0, gold(2_000_000)));
    assert!(!dev(&mut h, 0, material("holz", 5)));
    let c = DevCommand {
        action: "buildAll".into(),
        level: 6,
        ..DevCommand::default()
    };
    assert!(!dev(&mut h, 0, c));
    assert!(h.log().is_empty());
}
