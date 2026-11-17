//! winit-Tastencode → [`Key`] (physische Taste, unabhängig vom Layout; Y und Z wie bei Ebitengine physisch).

use winit::keyboard::KeyCode;

use crate::Key;

pub fn map(code: KeyCode) -> Option<Key> {
    use KeyCode as C;
    Some(match code {
        C::KeyA => Key::A,
        C::KeyD => Key::D,
        C::KeyE => Key::E,
        C::KeyF => Key::F,
        C::KeyK => Key::K,
        C::KeyQ => Key::Q,
        C::KeyR => Key::R,
        C::KeyS => Key::S,
        C::KeyT => Key::T,
        C::KeyW => Key::W,
        C::KeyY => Key::Y,
        C::KeyZ => Key::Z,
        C::ArrowDown => Key::ArrowDown,
        C::ArrowLeft => Key::ArrowLeft,
        C::ArrowRight => Key::ArrowRight,
        C::ArrowUp => Key::ArrowUp,
        C::Backspace => Key::Backspace,
        C::ControlRight => Key::ControlRight,
        C::Delete => Key::Delete,
        C::End => Key::End,
        C::Enter => Key::Enter,
        C::Escape => Key::Escape,
        C::Home => Key::Home,
        C::Insert => Key::Insert,
        C::PageDown => Key::PageDown,
        C::PageUp => Key::PageUp,
        C::ShiftLeft => Key::ShiftLeft,
        C::ShiftRight => Key::ShiftRight,
        C::Space => Key::Space,
        C::Tab => Key::Tab,
        C::NumpadEnter => Key::NumpadEnter,
        _ => return function_or_numpad(code),
    })
}

fn function_or_numpad(code: KeyCode) -> Option<Key> {
    use KeyCode as C;
    Some(match code {
        C::F1 => Key::F1,
        C::F2 => Key::F2,
        C::F3 => Key::F3,
        C::F5 => Key::F5,
        C::F6 => Key::F6,
        C::F7 => Key::F7,
        C::F8 => Key::F8,
        C::F9 => Key::F9,
        C::F10 => Key::F10,
        C::F11 => Key::F11,
        C::F12 => Key::F12,
        C::Numpad0 => Key::Numpad0,
        C::Numpad1 => Key::Numpad1,
        C::Numpad2 => Key::Numpad2,
        C::Numpad3 => Key::Numpad3,
        C::Numpad4 => Key::Numpad4,
        C::Numpad5 => Key::Numpad5,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jede_spieltaste_hat_einen_code() {
        let all = [
            KeyCode::KeyA,
            KeyCode::KeyD,
            KeyCode::KeyE,
            KeyCode::KeyF,
            KeyCode::KeyK,
            KeyCode::KeyQ,
            KeyCode::KeyR,
            KeyCode::KeyS,
            KeyCode::KeyT,
            KeyCode::KeyW,
            KeyCode::KeyY,
            KeyCode::KeyZ,
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
            KeyCode::ArrowUp,
            KeyCode::Backspace,
            KeyCode::ControlRight,
            KeyCode::Delete,
            KeyCode::End,
            KeyCode::Enter,
            KeyCode::Escape,
            KeyCode::Home,
            KeyCode::Insert,
            KeyCode::PageDown,
            KeyCode::PageUp,
            KeyCode::ShiftLeft,
            KeyCode::ShiftRight,
            KeyCode::Space,
            KeyCode::Tab,
            KeyCode::NumpadEnter,
            KeyCode::F1,
            KeyCode::F2,
            KeyCode::F3,
            KeyCode::F5,
            KeyCode::F6,
            KeyCode::F7,
            KeyCode::F8,
            KeyCode::F9,
            KeyCode::F10,
            KeyCode::F11,
            KeyCode::F12,
            KeyCode::Numpad0,
            KeyCode::Numpad1,
            KeyCode::Numpad2,
            KeyCode::Numpad3,
            KeyCode::Numpad4,
            KeyCode::Numpad5,
        ];
        let mut keys: Vec<Key> = all.iter().filter_map(|c| map(*c)).collect();
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), Key::ALL.len(), "jede Key genau einmal erreichbar");
        assert_eq!(map(KeyCode::KeyB), None);
    }
}
