//! Port von `native/k3c-game/game/keytable.go`: Tastenbelegung für das Pause-Menü (B-807): game kennt `input` (Tasten)
//! und `hud` (Beschriftungen) und baut daraus die Zeilen von `view::keys::KeyRow`; `menu` bekommt nur die fertigen Zeilen
//! (Importregel, `view/keys.rs`). Statt `ebiten.Key` steht `k3c_platform::Key`; die Namen der Varianten sind die von
//! Ebitengine ohne `Key`-Präfix (`Debug` der Variante, großgeschrieben, ergibt `ebiten.Key.String()` in Großschrift).

use k3c_platform::Key;

use crate::hud::glyphs::{key_label, pad_label};
use crate::input::{action_keys, command_keys};
use crate::view::input::Action;
use crate::view::keys::{
    KEY_ACTIONS, KEY_ATTACK, KEY_FULLSCREEN, KEY_MOVE, KEY_PAUSE, KEY_PAY, KEY_SKILL_MENU, KEY_SKILL1, KEY_SKILL2,
    KEY_SKILL3, KEY_SKILL4, KEY_SPRINT, KeyRow,
};

/// Name des gehaltenen Befehls in `input` (und `hud::pad_label`) für eine Aktion der Tastenbelegung; `move` sind zwei
/// Befehle (links, rechts) und steht hier nicht.
fn key_command(action: &str) -> Option<&'static str> {
    Some(match action {
        KEY_SPRINT => "sprint",
        KEY_PAY => "pay",
        KEY_ATTACK => "attack",
        KEY_SKILL1 => "skill1",
        KEY_SKILL2 => "skill2",
        KEY_SKILL3 => "skill3",
        KEY_SKILL4 => "skill4",
        _ => return None,
    })
}

/// Menü-Aktion in `input` und ihr Name in `hud` für die Aktionen der Tastenbelegung, die Menü-Aktionen sind.
fn key_action(action: &str) -> Option<(Action, &'static str)> {
    Some(match action {
        KEY_SKILL_MENU => (Action::SkillMenu, "skillMenu"),
        KEY_PAUSE => (Action::Menu, "pause"),
        KEY_FULLSCREEN => (Action::Fullscreen, "fullscreen"),
        _ => return None,
    })
}

/// hud_key_name ist der Name einer Taste in der Schreibweise des HUD (`hud::key_name`, `hud::key_label`): großgeschrieben,
/// `ESCAPE` wird `ESC`, `ARROWLEFT` wird `LEFT`, `NUMPAD7` wird `NUM7`.
pub fn hud_key_name(k: Key) -> String {
    let n = format!("{k:?}").to_uppercase();
    if n == "ESCAPE" {
        return "ESC".to_string();
    }
    if let Some(rest) = n.strip_prefix("ARROW") {
        return rest.to_string();
    }
    if let Some(rest) = n.strip_prefix("NUMPAD")
        && rest.len() == 1
    {
        return format!("NUM{rest}");
    }
    n
}

/// keys_for liefert die Tasten der Tastatur-Belegung `half` (0 wasd, 1 pfeile) für eine Aktion der Tastenbelegung
/// (move: links, dann rechts).
pub fn keys_for(half: i64, action: &str) -> Vec<Key> {
    if action == KEY_MOVE {
        let (l, r) = (command_keys(half, "left"), command_keys(half, "right"));
        return match (l.first(), r.first()) {
            (Some(&l), Some(&r)) => vec![l, r],
            _ => Vec::new(),
        };
    }
    if let Some(c) = key_command(action) {
        return command_keys(half, c).to_vec();
    }
    key_action(action).map_or_else(Vec::new, |(a, _)| action_keys(half, a).to_vec())
}

/// key_text beschriftet die Tasten einer Aktion: bei `both` (move) beide Richtungen, sonst die erste (weitere sind
/// Ersatz, z. B. Y für Z auf QWERTY, Num-Enter); "" = nicht belegt.
fn key_text(lang: &str, keys: &[Key], both: bool) -> String {
    match keys {
        [] => String::new(),
        [a, b, ..] if both => format!(
            "{} / {}",
            key_label(lang, &hud_key_name(*a)),
            key_label(lang, &hud_key_name(*b))
        ),
        [a, ..] => key_label(lang, &hud_key_name(*a)),
    }
}

/// pad_action ist der Name einer Aktion der Tastenbelegung für `hud::pad_label`.
fn pad_action(action: &str) -> &str {
    match action {
        KEY_PAY => "confirm",
        KEY_MOVE | KEY_SPRINT => action,
        _ => key_command(action)
            .or_else(|| key_action(action).map(|(_, h)| h))
            .unwrap_or(""),
    }
}

/// key_rows ist die Tastenbelegung für `menu` (`view::keys::KeyTable`): eine Zeile je `KEY_ACTIONS`, Tastatur aus
/// `input`, Gamepad mit der Standardbelegung (auch ohne Pad), beschriftet wie im HUD.
pub fn key_rows(lang: &str) -> Vec<KeyRow> {
    KEY_ACTIONS
        .iter()
        .map(|&a| {
            let mv = a == KEY_MOVE;
            KeyRow {
                action: a.to_string(),
                left: key_text(lang, &keys_for(0, a), mv),
                right: key_text(lang, &keys_for(1, a), mv),
                pad: pad_label(lang, pad_action(a)),
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "keytable_tests.rs"]
mod keytable_tests;
