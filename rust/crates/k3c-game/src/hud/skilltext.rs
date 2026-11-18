//! Port von `native/k3c-game/hud/skilltext.go`: Texte der Skill-Leiste und des Skill-Menüs (Spiegel von
//! `src/scenes/hud/skillMenuText.ts`, B-755): Die Leiste nennt keine Tasten, das Menü zeigt die Slot-Taste des Geräts
//! vor dem Skill (Pad: Xbox-Glyph, Tastatur: „[Q] “).

use super::data::line_name;
use super::glyphs::{GLYPH_ACTIONS, Glyph, glyph_of, slot_labels};
use super::keyline::{KeyLine, arg, new_key_line, text_part};
use super::skillmenu::{MenuEntry, RESPEC_ID, SlotView};
use super::texts::{tf, tr};
use super::values::SEP;
use crate::view::profile::PROFILE_GAMEPAD;

/// MENU_WINDOW: sichtbare Menüzeilen um den Cursor.
pub const MENU_WINDOW: usize = 5;

const TIER_NAMES: [&str; 5] = ["", "I", "II", "III", "IV"];

/// skill_name macht aus der ID einen Namen („shieldBash“ → „Shield Bash“); die Texte haben keine Skill-Namen.
pub fn skill_name(id: &str) -> String {
    let mut b = String::with_capacity(id.len() + 2);
    for (i, r) in id.chars().enumerate() {
        if i == 0 {
            b.extend(r.to_uppercase());
        } else {
            if r.is_uppercase() {
                b.push(' ');
            }
            b.push(r);
        }
    }
    b
}

/// entry_text ist eine Menüzeile: Cursor, Taste (nur belegter Skill, "" = keine), Linie, Tier und Name, dann Zustand.
pub fn entry_text(lang: &str, e: &MenuEntry, selected: bool, key: &str) -> String {
    let mark = if selected { "► " } else { "   " };
    if e.id == RESPEC_ID {
        return format!("{mark}{}", tr(lang, "skill.respec"));
    }
    let state = if e.learned {
        tr(lang, "skill.learned")
    } else if e.learnable {
        tr(lang, "skill.learnable")
    } else {
        tr(lang, "skill.locked")
    };
    let passive = if e.passive {
        format!(" ({})", tr(lang, "skill.passive"))
    } else {
        String::new()
    };
    let key = if key.is_empty() {
        String::new()
    } else {
        format!("[{key}] ")
    };
    let tier = usize::try_from(e.tier)
        .ok()
        .and_then(|t| TIER_NAMES.get(t))
        .map_or_else(|| e.tier.to_string(), |t| (*t).to_string());
    format!(
        "{mark}{key}{} {tier} · {}{passive}{SEP}{state}",
        line_name(&e.line),
        skill_name(&e.id)
    )
}

/// MenuRow ist eine sichtbare Zeile des Menüs: Glyph der Slot-Taste (Xbox-Taste bzw. Tastenkappe, B-894) und Text.
#[derive(Clone, Debug, Default)]
pub struct MenuRow {
    pub glyph: Glyph,
    pub text: String,
    pub selected: bool,
}

/// MenuView ist das gezeichnete Menü: Titel, Zeilen um den Cursor, Bedienhinweis als Text (`footer`) und als Zeile mit
/// Tasten-Icons (`line`, B-911).
#[derive(Clone, Debug, Default)]
pub struct MenuView {
    pub title: String,
    pub rows: [MenuRow; MENU_WINDOW],
    pub n: usize,
    pub footer: String,
    pub line: KeyLine,
}

/// MenuKey ist alles, wovon der Text des Menüs abhängt.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MenuKey {
    /// Sprache
    pub lang: String,
    /// Eingabe-Profil des Spielers (B-894)
    pub profile: String,
    pub cursor: i64,
    pub points: i64,
    pub n: usize,
    pub learned: u64,
    pub learnable: u64,
    pub respec: bool,
    pub slots: [String; 4],
}

/// new_menu_key fasst Einträge, Punkte und Slots zusammen.
pub fn new_menu_key(
    lang: &str,
    profile: &str,
    cursor: i64,
    points: i64,
    entries: &[MenuEntry],
    slots: &[SlotView; 4],
) -> MenuKey {
    let mut k = MenuKey {
        lang: lang.to_string(),
        profile: profile.to_string(),
        cursor,
        points,
        n: entries.len(),
        slots: std::array::from_fn(|i| slots[i].skill.clone()),
        ..MenuKey::default()
    };
    for (i, e) in entries.iter().enumerate() {
        if e.id == RESPEC_ID {
            k.respec = true;
        } else if i < 64 && e.learned {
            k.learned |= 1 << i;
        }
        if i < 64 && e.learnable {
            k.learnable |= 1 << i;
        }
    }
    k
}

/// build_menu baut die Zeilen aus den Einträgen (Fenster von `MENU_WINDOW` Zeilen um den Cursor).
pub fn build_menu(k: &MenuKey, entries: &[MenuEntry]) -> MenuView {
    let mut v = MenuView {
        title: tf(&k.lang, "skill.title", &[("n", k.points.into())]),
        ..MenuView::default()
    };
    let labels = slot_labels(&k.lang, &k.profile);
    let window = MENU_WINDOW as i64;
    let from = (k.cursor - window / 2).min(entries.len() as i64 - window).max(0) as usize;
    for (i, e) in entries.iter().enumerate().skip(from).take(MENU_WINDOW) {
        let mut row = MenuRow {
            selected: i as i64 == k.cursor,
            ..MenuRow::default()
        };
        let mut key = "";
        for (s, id) in k.slots.iter().enumerate() {
            if !id.is_empty() && *id == e.id {
                row.glyph = glyph_of(&k.lang, GLYPH_ACTIONS[4 + s], &k.profile);
                if row.glyph.key.is_empty() {
                    key = &labels[s + 1]; // ohne Icon: Taste als Text
                }
            }
        }
        row.text = entry_text(&k.lang, e, row.selected, key);
        v.rows[v.n] = row;
        v.n += 1;
    }
    v.line = menu_line(&k.lang, &k.profile);
    v.footer = v.line.plain.clone();
    v
}

/// menu_line ist der Bedienhinweis mit den Tasten des Eingabe-Profils; am Pad schließt B (ohne Bild; Steuerkreuz ↓ wählt
/// im offenen Menü weiter).
pub fn menu_line(lang: &str, profile: &str) -> KeyLine {
    let menu = if profile == PROFILE_GAMEPAD {
        text_part("B")
    } else {
        glyph_of(lang, "skillMenu", profile)
    };
    new_key_line(
        tr(lang, "skill.hint"),
        &[
            move_arg(lang, profile),
            arg("ok", vec![glyph_of(lang, "confirm", profile)]),
            arg("menu", vec![menu]),
        ],
    )
}

/// move_arg sind die Tasten zum Wählen (Platzhalter `move`): am Pad die Pfeile als Text, an der Tastatur Links und
/// Rechts des Eingabe-Profils.
pub(super) fn move_arg(lang: &str, profile: &str) -> super::keyline::LineArg {
    if profile == PROFILE_GAMEPAD {
        return arg("move", vec![text_part("◄ ►")]);
    }
    arg(
        "move",
        vec![
            glyph_of(lang, "left", profile),
            text_part(" / "),
            glyph_of(lang, "right", profile),
        ],
    )
}

/// BarKey: Skill und Abklingzeit (ganze Sekunden, aufgerundet) je Slot.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BarKey {
    pub lang: String,
    pub skills: [String; 4],
    pub cd: [i64; 4],
}

pub fn new_bar_key(lang: &str, slots: &[SlotView; 4]) -> BarKey {
    BarKey {
        lang: lang.to_string(),
        skills: std::array::from_fn(|i| slots[i].skill.clone()),
        cd: std::array::from_fn(|i| slots[i].cooldown.ceil() as i64),
    }
}

/// slot_bar ist die Skill-Leiste: Skill und Abklingzeit je Slot, ohne Taste.
pub fn slot_bar(k: &BarKey) -> String {
    let parts: Vec<String> = k
        .skills
        .iter()
        .enumerate()
        .map(|(i, id)| {
            if id.is_empty() {
                tr(&k.lang, "skill.free").to_string()
            } else if k.cd[i] > 0 {
                format!("{} {} s", skill_name(id), k.cd[i])
            } else {
                format!("{} {}", skill_name(id), tr(&k.lang, "skill.ready"))
            }
        })
        .collect();
    parts.join(SEP)
}
