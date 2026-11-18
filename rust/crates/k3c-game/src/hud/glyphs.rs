//! Port von `native/k3c-game/hud/glyphs.go`.
//!
//! Glyphen und Beschriftungen der Tasten (Spiegel von src/scenes/hints/glyphs.ts und src/input/slotBindings.ts, S6.2,
//! B-149): Aktion und Gerät → Glyph-Schlüssel und Beschriftung. Nur Xbox-Glyphen für Pads; die Tastatur hat zwei
//! Belegungen (Eingabe-Profile wasd und pfeile, B-893). B und View bekommen nie ein Bild.
//!
//! Abweichung von Go (Leitfaden § 3): Die Go-Maps `padKeys`, `leftKeys`, `rightKeys`, `padLabels` und `glyphColors`
//! sind `match`-Funktionen; "nicht belegt" ist `None`.

use super::fonts::{Rgba, rgb};
use super::texts::tr;
use super::values::SEP;
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::profile::{PROFILE_ARROWS, PROFILE_GAMEPAD};

/// Glyph ist das Bild einer Taste: `key` "" = nur Text (Rückfall), `label` = Beschriftung im Bild bzw. Text, `icon` =
/// das Papierschnitt-Icon dazu (`icon_asset`; "" ohne Bild).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Glyph {
    pub key: String,
    pub label: String,
    pub icon: String,
}

/// new_glyph ist die Glyph zum Schlüssel key mit Beschriftung label und ihrem Icon.
pub fn new_glyph(key: &str, label: &str) -> Glyph {
    Glyph {
        key: key.to_string(),
        label: label.to_string(),
        icon: icon_asset(key, label),
    }
}

/// Text-Rückfall: nur Beschriftung, kein Bild.
fn text_glyph(label: &str) -> Glyph {
    Glyph {
        label: label.to_string(),
        ..Glyph::default()
    }
}

/// GLYPH_ACTIONS: Aktionen in der Reihenfolge der Belegung (slotBindings.ts › KEY_ACTIONS).
pub const GLYPH_ACTIONS: [&str; 9] = [
    "confirm",
    "pause",
    "fullscreen",
    "attack",
    "skill1",
    "skill2",
    "skill3",
    "skill4",
    "skillMenu",
];

/// KEY_ACTIONS sind die Aktionen mit Tastatur-Taste (`key_name`).
pub const KEY_ACTIONS: [&str; 11] = [
    "confirm",
    "pause",
    "fullscreen",
    "attack",
    "skill1",
    "skill2",
    "skill3",
    "skill4",
    "skillMenu",
    "left",
    "right",
];

/// pad_key ist die Xbox-Taste der Aktion am Pad (None = nicht belegt).
fn pad_key(action: &str) -> Option<&'static str> {
    Some(match action {
        "confirm" => "A",
        "pause" => "MENU",
        "fullscreen" => "RS",
        "attack" => "X",
        "skill1" => "LB",
        "skill2" => "RB",
        "skill3" => "LT",
        "skill4" => "UP",
        "skillMenu" => "DOWN",
        _ => return None,
    })
}

/// left_key: Tastatur-Profil wasd (None = nicht belegt); left/right sind Laufen bzw. Wählen. Die Tastatur spiegelt
/// input/bindings.
fn left_key(action: &str) -> Option<&'static str> {
    Some(match action {
        "confirm" => "SPACE",
        "pause" => "ESC",
        "fullscreen" => "F11",
        "attack" => "E",
        "skill1" => "Q",
        "skill2" => "R",
        "skill3" => "T",
        "skill4" => "Z",
        "skillMenu" => "K",
        "left" => "A",
        "right" => "D",
        _ => return None,
    })
}

/// right_key: Tastatur-Profil pfeile (None = nicht belegt).
fn right_key(action: &str) -> Option<&'static str> {
    Some(match action {
        "confirm" => "ENTER",
        "pause" => "ESC",
        "fullscreen" => "F11",
        "attack" => "CONTROLRIGHT",
        "skill1" => "INSERT",
        "skill2" => "DELETE",
        "skill3" => "END",
        "skill4" => "PAGEDOWN",
        "skillMenu" => "DOWN",
        "left" => "LEFT",
        "right" => "RIGHT",
        _ => return None,
    })
}

/// key_name liefert den Namen der Taste der Tastatur-Belegung layout (0 wasd, 1 pfeile) für action ("" = nicht
/// belegt), wie ihn das HUD beschriftet: Buchstaben, SPACE, ESC, ENTER, LEFT, RIGHT, DOWN, INSERT … Für den Abgleich
/// mit input.
pub fn key_name(layout: i64, action: &str) -> &'static str {
    let k = if layout == 1 {
        right_key(action)
    } else {
        left_key(action)
    };
    k.unwrap_or("")
}

/// PAD_GLYPHS sind die Controller-Tasten mit Bild (B-149).
pub const PAD_GLYPHS: [&str; 11] = ["A", "X", "Y", "LB", "RB", "LT", "RT", "RS", "UP", "DOWN", "MENU"];

/// pad_label_of: Beschriftung der Steuerkreuz-Tasten (slotBindings.ts › PAD_LABELS).
fn pad_label_of(name: &str) -> Option<&'static str> {
    match name {
        "UP" => Some("D-Pad ↑"),
        "DOWN" => Some("D-Pad ↓"),
        _ => None,
    }
}

/// glyph_colors: Füllung und Schrift je Glyph-Art (Xbox-Farben für A/X/Y), Kontrast ≥ 4,5:1 (Test).
pub fn glyph_colors() -> [(&'static str, [Rgba; 2]); 5] {
    [
        ("A", [rgb(0x107c10, 1.0), rgb(0xffffff, 1.0)]),
        ("X", [rgb(0x0e6ac7, 1.0), rgb(0xffffff, 1.0)]),
        ("Y", [rgb(0xffb900, 1.0), rgb(0x000000, 1.0)]),
        ("pad", [rgb(0x2b2b2b, 1.0), rgb(0xffffff, 1.0)]),
        ("key", [rgb(0xf2f2f2, 1.0), rgb(0x111111, 1.0)]),
    ]
}

/// key_label ist die Beschriftung einer Tastatur-Taste (Leertaste und Umschalttasten aus den Texten, Ziffernblock als
/// „Num n“).
pub fn key_label(lang: &str, name: &str) -> String {
    match name {
        "SPACE" => tr(lang, "hint.space").to_string(),
        "SHIFTLEFT" => tr(lang, "key.shiftLeft").to_string(),
        "SHIFTRIGHT" => tr(lang, "key.shiftRight").to_string(),
        "CONTROLRIGHT" => tr(lang, "key.ctrlRight").to_string(),
        "INSERT" | "DELETE" | "END" | "PAGEDOWN" | "PAGEUP" => {
            tr(lang, &format!("key.{}", name.to_lowercase())).to_string()
        }
        "ESC" => "Esc".to_string(),
        "ENTER" => "Enter".to_string(),
        "BACKSPACE" => "Backspace".to_string(),
        // Wörter statt Pfeilzeichen: die Debug-Schrift kennt nur ASCII
        "LEFT" | "RIGHT" | "UP" | "DOWN" => tr(lang, &format!("hint.{}", name.to_lowercase())).to_string(),
        _ => match name.strip_prefix("NUM") {
            Some(rest) => format!("Num {rest}"),
            None => name.to_string(),
        },
    }
}

/// pad_label ist die Beschriftung der Gamepad-Taste für action (Aktionen wie `key_name`, dazu "sprint" und "move") in
/// der Sprache lang; "" = nicht belegt. Für die Tastenbelegung im Pause-Menü (game, B-807).
pub fn pad_label(lang: &str, action: &str) -> String {
    match action {
        "move" => return tr(lang, "key.padMove").to_string(),
        "sprint" => return "RT".to_string(),
        _ => {}
    }
    let Some(name) = pad_key(action) else {
        return String::new();
    };
    if let Some(l) = pad_label_of(name) {
        return l.to_string();
    }
    if name == "MENU" {
        return "Menu".to_string();
    }
    name.to_string()
}

/// glyph_of ist die zentrale Zuordnung Aktion + Eingabe-Profil → Tasten-Icon (B-894): Glyph-Schlüssel ("pad:A",
/// "key:E") und Beschriftung; alle Tasten-Hinweise des HUD gehen darüber. `profile` ist `PROFILE_WASD`,
/// `PROFILE_ARROWS` oder `PROFILE_GAMEPAD`; unbekannte oder nicht belegte Aktion: Text-Rückfall mit dem Aktionsnamen.
pub fn glyph_of(lang: &str, action: &str, profile: &str) -> Glyph {
    if profile == PROFILE_GAMEPAD {
        let Some(name) = pad_key(action) else {
            return text_glyph(action);
        };
        let label = pad_label_of(name).unwrap_or(name);
        if PAD_GLYPHS.contains(&name) {
            return new_glyph(&format!("pad:{name}"), label);
        }
        return text_glyph(label);
    }
    let name = if profile == PROFILE_ARROWS {
        right_key(action)
    } else {
        left_key(action)
    };
    let Some(name) = name else {
        return text_glyph(action);
    };
    new_glyph(&format!("key:{name}"), &key_label(lang, name))
}

/// Tastenkappen (B-911): leer, die Beschriftung setzt das HUD in der Sprache des Spiels darauf (drawglyph).
pub const CAP_NARROW: &str = "icons/taste-kappe"; // ein Zeichen
pub const CAP_WIDE: &str = "icons/taste-kappe-breit"; // längere Beschriftung

/// icon_asset ist das Papierschnitt-Icon (Gruppe icons) zum Glyph-Schlüssel key mit Beschriftung label: "pad:A" →
/// icons/xbox-a; eine Tastatur-Taste bekommt nach der Länge ihrer Beschriftung die schmale (ein Zeichen) oder die
/// breite Kappe; "" ohne Bild. Ist das Icon nicht geladen, zeichnet drawGlyph Kappe bzw. Xbox-Taste selbst.
pub fn icon_asset(key: &str, label: &str) -> String {
    let Some((kind, name)) = key.split_once(':') else {
        return String::new();
    };
    if name.is_empty() {
        return String::new();
    }
    if kind == "pad" {
        return format!("icons/xbox-{}", name.to_lowercase());
    }
    let cap = if label.chars().count() <= 1 {
        CAP_NARROW
    } else {
        CAP_WIDE
    };
    cap.to_string()
}

/// slot_labels ist die Beschriftung von Schlag, Skill-Slot 1–4 und Skill-Menü im Eingabe-Profil profile.
pub fn slot_labels(lang: &str, profile: &str) -> [String; 6] {
    std::array::from_fn(|i| glyph_of(lang, GLYPH_ACTIONS[3 + i], profile).label)
}

/// join_text ist der Beitritts-Hinweis für die freien Geräte (`Frame.unseated`): je Art des Geräts ein Satz, in der
/// Reihenfolge der Eingabe, jede Art höchstens einmal (joinHint.ts, B-391). Die Tastatur nennt nur Enter (B-946:
/// „Leertaste“ stört; Enter bestätigt in jedem Profil). Ohne freies Gerät "".
pub fn join_text(lang: &str, unseated: &[DeviceID]) -> String {
    let mut seen = [false; 2];
    let mut parts: Vec<&str> = Vec::with_capacity(2);
    for d in unseated {
        let (i, key) = if d.kind == DeviceKind::Keyboard {
            (1, "hud.join.keyboard")
        } else {
            (0, "hud.join.pad")
        };
        if !seen[i] {
            seen[i] = true;
            parts.push(tr(lang, key));
        }
    }
    parts.join(SEP)
}

/// JoinKey: welche Arten freier Geräte es gibt (0 Pad, 1 Tastatur) in Reihenfolge des ersten Auftretens.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JoinKey {
    pub lang: String,
    pub order: [i8; 2],
    pub n: usize,
}

/// new_join_key fasst die freien Geräte für `Memo` zusammen, ohne anzulegen.
pub fn new_join_key(lang: &str, unseated: &[DeviceID]) -> JoinKey {
    let mut k = JoinKey {
        lang: lang.to_string(),
        ..JoinKey::default()
    };
    for d in unseated {
        let i = i8::from(d.kind == DeviceKind::Keyboard);
        if !k.order[..k.n].contains(&i) && k.n < k.order.len() {
            k.order[k.n] = i;
            k.n += 1;
        }
    }
    k
}

/// join_from_key baut den Text aus dem Schlüssel.
pub fn join_from_key(k: &JoinKey) -> String {
    let devs: Vec<DeviceID> = k.order[..k.n]
        .iter()
        .map(|&i| {
            DeviceID::new(
                if i == 1 {
                    DeviceKind::Keyboard
                } else {
                    DeviceKind::Gamepad
                },
                0,
            )
        })
        .collect();
    join_text(&k.lang, &devs)
}
