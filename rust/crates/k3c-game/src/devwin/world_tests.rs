use k3c_devproto::{DevCommand, Msg};

use super::super::commands::handle;
use super::super::mock::MockHost;
use crate::view::devkeys::{DevAction, DevCmd};

fn dev(host: &mut MockHost, player: i64, c: DevCommand) -> (bool, String) {
    ack(handle(
        host,
        Msg::Dev {
            id: 1,
            player,
            action: c,
        },
    ))
}

fn ui(host: &mut MockHost, key: &str, value: Option<i64>) -> (bool, String) {
    ack(handle(
        host,
        Msg::Ui {
            id: 1,
            key: key.into(),
            value,
        },
    ))
}

fn ack(m: Option<Msg>) -> (bool, String) {
    match m {
        Some(Msg::Ack { ok, err, .. }) => (ok, err.unwrap_or_default()),
        other => panic!("kein ack: {other:?}"),
    }
}

fn act(action: &str) -> DevAction {
    DevAction {
        action: action.into(),
        ..DevAction::default()
    }
}

fn cmd(action: &str) -> DevCommand {
    DevCommand {
        action: action.into(),
        ..DevCommand::default()
    }
}

#[test]
fn tageszeit_zeitraffer_grad_wie_im_f9_menue() {
    let mut h = MockHost::new(2);
    let night = DevCommand {
        phase: "night".into(),
        ..cmd("phase")
    };
    assert!(dev(&mut h, 1, night).0);
    let x4 = DevCommand {
        factor: 4,
        ..cmd("timescale")
    };
    assert!(dev(&mut h, 0, x4).0);
    let hard = DevCommand {
        grade: "hard".into(),
        ..cmd("grade")
    };
    assert!(dev(&mut h, 0, hard).0);
    assert!(dev(&mut h, 0, cmd("nextPhase")).0);
    let auto = DevCommand {
        on: Some(true),
        ..cmd("timescaleAuto")
    };
    assert!(dev(&mut h, 0, auto).0);
    let want = [
        (
            1,
            DevAction {
                phase: "night".into(),
                ..act("phase")
            },
        ),
        (
            0,
            DevAction {
                factor: 4,
                ..act("timescale")
            },
        ),
        (
            0,
            DevAction {
                grade: "hard".into(),
                ..act("grade")
            },
        ),
        (0, act("nextPhase")),
        (
            0,
            DevAction {
                on: Some(true),
                ..act("timescaleAuto")
            },
        ),
    ];
    assert_eq!(h.log(), want);
}

#[test]
fn falsche_werte_erreichen_die_session_nicht() {
    let mut h = MockHost::new(1);
    for c in [
        DevCommand {
            phase: "mittag".into(),
            ..cmd("phase")
        },
        DevCommand {
            grade: "dev".into(),
            ..cmd("grade")
        },
        DevCommand {
            factor: 3,
            ..cmd("timescale")
        },
    ] {
        let (ok, err) = dev(&mut h, 0, c);
        assert!(!ok && !err.is_empty());
    }
    assert!(h.log().is_empty());
}

#[test]
fn pause_einzelschritt_speichern_laden() {
    let mut h = MockHost::new(1);
    assert!(!ui(&mut h, "step", None).0, "Einzelschritt nur im Halt");
    assert!(ui(&mut h, "pause", None).0);
    assert!(h.session.as_ref().unwrap().halted);
    assert!(ui(&mut h, "step", None).0);
    assert_eq!(h.session.as_ref().unwrap().steps, 1);
    assert!(ui(&mut h, "pause", Some(0)).0);
    assert!(!h.session.as_ref().unwrap().halted);
    assert!(ui(&mut h, "pause", Some(1)).0);
    assert!(h.session.as_ref().unwrap().halted);
    assert!(ui(&mut h, "save", None).0);
    assert_eq!(h.cmds, [DevCmd::Save]);
    assert!(ui(&mut h, "load", None).0);
    assert_eq!(h.session.as_ref().unwrap().reloads, 1);
}

#[test]
fn zeitraffer_taste_mit_und_ohne_wert() {
    let mut h = MockHost::new(1);
    assert!(ui(&mut h, "speed", None).0);
    assert_eq!(h.cmds, [DevCmd::Speed], "wie F8");
    assert!(ui(&mut h, "speed", Some(8)).0);
    assert_eq!(h.log().len(), 1);
    assert_eq!(h.log()[0].1.factor, 8);
    assert!(!ui(&mut h, "speed", Some(3)).0);
}

#[test]
fn ohne_entwickler_rolle_ack_false() {
    let mut h = MockHost::new(1);
    h.session = None;
    for key in ["pause", "step", "load"] {
        let (ok, err) = ui(&mut h, key, None);
        assert!(!ok && err.contains("Entwickler-Rolle"), "{key}");
    }
    assert!(!dev(&mut h, 0, cmd("nextPhase")).0);
}
