use super::*;

#[test]
#[ignore = "wartet auf RU8"]
fn command_sim_clamps() {
    // Port von TestCommandSimClamps braucht `Command::sim()` und `sim::PlayerCommand` (RU8, Port in RU7.5).
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
