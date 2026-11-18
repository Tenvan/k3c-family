//! Port von `native/k3c-game/game/keys_test.go`.

use super::*;
use crate::hud::glyphs::{KEY_ACTIONS as HUD_ACTIONS, key_name};

/// Tasten einer Aktion des HUD (Menü-Aktion oder gehaltener Befehl) aus `input` (Go: `keysOf`).
fn keys_of(half: i64, action: &str) -> Vec<Key> {
    match action {
        "confirm" => action_keys(half, Action::Confirm).to_vec(),
        "pause" => action_keys(half, Action::Menu).to_vec(),
        "fullscreen" => action_keys(half, Action::Fullscreen).to_vec(),
        "skillMenu" => action_keys(half, Action::SkillMenu).to_vec(),
        _ => command_keys(half, action).to_vec(),
    }
}

/// Die Tasten-Texte des HUD je Tastatur-Belegung stimmen mit der Belegung in `input` überein (`hud` darf `input` nicht
/// importieren, `game` kennt beide); nicht belegte Aktionen haben auf beiden Seiten keine Taste.
#[test]
fn hud_key_names_match_input() {
    for half in 0..2 {
        for a in HUD_ACTIONS {
            let (name, keys) = (key_name(half, a), keys_of(half, a));
            let names: Vec<String> = keys.iter().map(|&k| hud_key_name(k)).collect();
            assert!(
                name.is_empty() == keys.is_empty() && (name.is_empty() || names.iter().any(|n| n == name)),
                "Hälfte {half}, {a}: HUD {name:?}, input {names:?}"
            );
        }
    }
}

/// Die Namen in Go-Schreibweise (`ebiten.Key.String()` großgeschrieben, dann wie `hudKeyName`) für jede Taste.
#[test]
fn hud_key_name_je_taste() {
    let table: &[(Key, &str)] = &[
        (Key::A, "A"),
        (Key::ArrowDown, "DOWN"),
        (Key::ArrowLeft, "LEFT"),
        (Key::ArrowRight, "RIGHT"),
        (Key::ArrowUp, "UP"),
        (Key::Backspace, "BACKSPACE"),
        (Key::ControlRight, "CONTROLRIGHT"),
        (Key::D, "D"),
        (Key::Delete, "DELETE"),
        (Key::E, "E"),
        (Key::End, "END"),
        (Key::Enter, "ENTER"),
        (Key::Escape, "ESC"),
        (Key::F, "F"),
        (Key::F1, "F1"),
        (Key::F2, "F2"),
        (Key::F3, "F3"),
        (Key::F5, "F5"),
        (Key::F6, "F6"),
        (Key::F7, "F7"),
        (Key::F8, "F8"),
        (Key::F9, "F9"),
        (Key::F10, "F10"),
        (Key::F11, "F11"),
        (Key::F12, "F12"),
        (Key::Home, "HOME"),
        (Key::Insert, "INSERT"),
        (Key::K, "K"),
        (Key::Numpad0, "NUM0"),
        (Key::Numpad1, "NUM1"),
        (Key::Numpad2, "NUM2"),
        (Key::Numpad3, "NUM3"),
        (Key::Numpad4, "NUM4"),
        (Key::Numpad5, "NUM5"),
        (Key::NumpadEnter, "NUMPADENTER"),
        (Key::PageDown, "PAGEDOWN"),
        (Key::PageUp, "PAGEUP"),
        (Key::Q, "Q"),
        (Key::R, "R"),
        (Key::S, "S"),
        (Key::ShiftLeft, "SHIFTLEFT"),
        (Key::ShiftRight, "SHIFTRIGHT"),
        (Key::Space, "SPACE"),
        (Key::T, "T"),
        (Key::Tab, "TAB"),
        (Key::W, "W"),
        (Key::Y, "Y"),
        (Key::Z, "Z"),
    ];
    for &(k, want) in table {
        assert_eq!(hud_key_name(k), want, "{k:?}");
    }
    for &k in Key::ALL {
        assert!(table.iter().any(|&(t, _)| t == k), "{k:?} fehlt in der Tabelle");
    }
}

/// Tastenbelegung für das Pause-Menü (B-807): eine Zeile je `KEY_ACTIONS`, Tastatur aus `input` mit den Beschriftungen
/// des HUD, Gamepad mit der Standardbelegung; Spalten WASD und Pfeile (B-893).
#[test]
fn key_rows_zeilen() {
    let rows = key_rows("de");
    assert_eq!(rows.len(), KEY_ACTIONS.len());
    let row = |l: &str, r: &str, p: &str| (l.to_string(), r.to_string(), p.to_string());
    let want = [
        (
            KEY_MOVE,
            row("A / D", "Pfeil links / Pfeil rechts", "Stick / Steuerkreuz"),
        ),
        (KEY_SPRINT, row("Shift links", "Shift rechts", "RT")),
        (KEY_ATTACK, row("E", "Strg rechts", "X")),
        (KEY_PAY, row("Leertaste", "Enter", "A")),
        (KEY_SKILL4, row("Z", "Bild ab", "D-Pad ↑")),
        (KEY_PAUSE, row("Esc", "Esc", "Menu")),
        (KEY_FULLSCREEN, row("F11", "F11", "RS")),
    ];
    for (i, r) in rows.iter().enumerate() {
        assert!(
            r.action == KEY_ACTIONS[i] && !r.left.is_empty() && !r.pad.is_empty(),
            "Zeile {i}: {r:?}"
        );
        assert_eq!(
            r.right.is_empty(),
            keys_for(1, &r.action).is_empty(),
            "{} rechts: {:?} gegen input {:?}",
            r.action,
            r.right,
            keys_for(1, &r.action)
        );
        if let Some((_, w)) = want.iter().find(|(a, _)| *a == r.action) {
            assert_eq!((r.left.clone(), r.right.clone(), r.pad.clone()), *w, "{}", r.action);
        }
    }
    assert_eq!(key_rows("en")[2].left, "Space");
}
