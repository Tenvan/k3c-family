use k3c_sim::state::PlayerCommand;

use super::*;

#[test]
fn command_sim_clamps() {
    let cases = [
        (
            Command {
                move_: 0.5,
                pay: true,
                skill: 2,
                ..Default::default()
            },
            PlayerCommand {
                move_x: 0.5,
                pay: true,
                skill: 2,
                ..Default::default()
            },
        ),
        (
            Command {
                move_: 3.0,
                sprint: true,
                ..Default::default()
            },
            PlayerCommand {
                move_x: 1.0,
                sprint: true,
                ..Default::default()
            },
        ),
        (
            Command {
                move_: -7.0,
                attack: true,
                skill: 5,
                ..Default::default()
            },
            PlayerCommand {
                move_x: -1.0,
                attack: true,
                ..Default::default()
            },
        ),
        (
            Command {
                move_: f64::NAN,
                skill: -1,
                ..Default::default()
            },
            PlayerCommand::default(),
        ),
    ];
    for (c, want) in cases {
        assert_eq!(c.sim(), want, "{c:?}.sim()");
    }
}

#[test]
fn input_lookup() {
    let kb = DeviceID::new(DeviceKind::Keyboard, 0);
    let pad = DeviceID::new(DeviceKind::Gamepad, 0);
    let input = Input {
        devices: vec![
            DeviceInput {
                device: kb,
                command: Command {
                    move_: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            DeviceInput {
                device: pad,
                actions: vec![Action::Down, Action::Confirm],
                ..Default::default()
            },
        ],
    };
    let (di, ok) = input.of(kb);
    assert!(ok && di.command.move_ == 1.0, "Tastatur nicht gefunden");
    let (di, ok) = input.of(DeviceID::new(DeviceKind::Gamepad, 3));
    assert!(!ok && di.device.index == 3, "nicht verbundenes Pad gefunden");
    let (d, ok) = input.has(Action::Confirm);
    assert!(ok && d == pad, "Confirm von {d}");
    assert!(!input.has(Action::Back).1, "Back ohne Druck");
    assert!(kb.to_string() == "keys1" && pad.to_string() == "pad1", "Gerätenamen");
}
