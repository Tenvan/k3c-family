//! Port von `native/k3c-game/input/bindings.go`: Belegung der Tastatur-Profile und des Gamepads.

use std::sync::LazyLock;

use k3c_platform::{Key, PadButton};

use crate::view::devkeys::{DevCmd, dev_key_of};
use crate::view::input::Action;
use crate::view::profile::PROFILE_ARROWS;

/// ACTION_COUNT ist die Zahl der Menü-Aktionen in view (Index = `Action as usize`).
pub const ACTION_COUNT: usize = Action::Cheats as usize + 1;

/// ALL_ACTIONS sind alle Menü-Aktionen in der Reihenfolge ihres Index.
pub const ALL_ACTIONS: [Action; ACTION_COUNT] = [
    Action::Up,
    Action::Down,
    Action::Left,
    Action::Right,
    Action::Confirm,
    Action::Back,
    Action::Menu,
    Action::View,
    Action::PrevPage,
    Action::NextPage,
    Action::Home,
    Action::SkillMenu,
    Action::Fullscreen,
    Action::Cheats,
];

/// KeyLayout ist die Belegung eines Tastatur-Profils; je Eintrag gelten alle genannten Tasten.
#[derive(Default)]
pub struct KeyLayout {
    pub left: Vec<Key>,
    pub right: Vec<Key>,
    pub sprint: Vec<Key>,
    pub pay: Vec<Key>,
    pub attack: Vec<Key>,
    pub skills: [Vec<Key>; 4],
    pub actions: [Vec<Key>; ACTION_COUNT],
}

/// Tabelle der Menü-Aktionen mit ihren Tasten (Go: Array-Literal mit Index = Aktion).
fn action_table(entries: Vec<(Action, Vec<Key>)>) -> [Vec<Key>; ACTION_COUNT] {
    let mut t: [Vec<Key>; ACTION_COUNT] = Default::default();
    for (a, ks) in entries {
        t[a as usize] = ks;
    }
    t
}

/// Die Taste eines Entwickler-Befehls (leer, wenn er fehlt).
fn dev_keys(c: DevCmd) -> Vec<Key> {
    dev_key_of(c).into_iter().collect()
}

/// Indizes der Tastatur-Profile in LAYOUT_TABLE (Layout).
pub const LAYOUT_WASD: i64 = 0;
pub const LAYOUT_ARROWS: i64 = 1;

/// LAYOUTS ist die Zahl der Tastatur-Belegungen (Index für `action_keys` und `command_keys`).
pub const LAYOUTS: usize = 2;

/// Layout liefert den Index der Belegung zum Tastatur-Profil p (`PROFILE_ARROWS`: 1, sonst 0 = WASD).
pub fn layout(p: &str) -> i64 {
    if p == PROFILE_ARROWS {
        return LAYOUT_ARROWS;
    }
    LAYOUT_WASD
}

/// LAYOUT_TABLE sind die Belegungen der Tastatur-Profile wasd und pfeile (Index Layout; in Go `layouts`). Z liegt auf
/// QWERTZ auf der Taste Y, darum gelten für Skill 4 beide.
pub static LAYOUT_TABLE: LazyLock<[KeyLayout; LAYOUTS]> = LazyLock::new(|| {
    [
        KeyLayout {
            left: vec![Key::A],
            right: vec![Key::D],
            sprint: vec![Key::ShiftLeft],
            pay: vec![Key::Space],
            attack: vec![Key::E],
            skills: [vec![Key::Q], vec![Key::R], vec![Key::T], vec![Key::Z, Key::Y]],
            actions: action_table(vec![
                (Action::Up, vec![Key::W]),
                (Action::Down, vec![Key::S]),
                (Action::Left, vec![Key::A]),
                (Action::Right, vec![Key::D]),
                (Action::Confirm, vec![Key::Space, Key::Enter, Key::NumpadEnter]),
                (Action::Back, vec![Key::Escape]),
                (Action::Menu, vec![Key::Escape]),
                (Action::View, vec![Key::Tab]),
                (Action::PrevPage, vec![Key::Q]),
                (Action::NextPage, vec![Key::E]),
                (Action::Home, vec![Key::Home]),
                (Action::SkillMenu, vec![Key::K]),
                (Action::Fullscreen, dev_keys(DevCmd::Fullscreen)),
                (Action::Cheats, dev_keys(DevCmd::Cheats)),
            ]),
        },
        KeyLayout {
            left: vec![Key::ArrowLeft],
            right: vec![Key::ArrowRight],
            sprint: vec![Key::ShiftRight],
            pay: vec![Key::Enter, Key::NumpadEnter, Key::Space],
            attack: vec![Key::ControlRight, Key::Numpad0],
            skills: [
                vec![Key::Insert, Key::Numpad1],
                vec![Key::Delete, Key::Numpad2],
                vec![Key::End, Key::Numpad3],
                vec![Key::PageDown, Key::Numpad4],
            ],
            actions: action_table(vec![
                (Action::Up, vec![Key::ArrowUp]),
                (Action::Down, vec![Key::ArrowDown]),
                (Action::Left, vec![Key::ArrowLeft]),
                (Action::Right, vec![Key::ArrowRight]),
                (Action::Confirm, vec![Key::Enter, Key::NumpadEnter]),
                (Action::Back, vec![Key::Escape, Key::Backspace]),
                (Action::Menu, vec![Key::Escape, Key::Backspace]),
                (Action::View, vec![Key::Tab]),
                (Action::PrevPage, vec![Key::PageUp]),
                (Action::NextPage, vec![Key::PageDown]),
                (Action::Home, vec![Key::Home]),
                (Action::SkillMenu, vec![Key::ArrowDown, Key::Numpad5]),
                (Action::Fullscreen, dev_keys(DevCmd::Fullscreen)),
                (Action::Cheats, dev_keys(DevCmd::Cheats)),
            ]),
        },
    ]
});

// Tasten des Standard-Layouts mit Xbox-Namen.
pub const PAD_A: PadButton = PadButton::A;
pub const PAD_B: PadButton = PadButton::B;
pub const PAD_X: PadButton = PadButton::X;
pub const PAD_LB: PadButton = PadButton::LB;
pub const PAD_RB: PadButton = PadButton::RB;
pub const PAD_LT: PadButton = PadButton::LT;
pub const PAD_RT: PadButton = PadButton::RT;
pub const PAD_VIEW: PadButton = PadButton::View;
pub const PAD_MENU: PadButton = PadButton::Menu;
pub const PAD_UP: PadButton = PadButton::Up;
pub const PAD_DOWN: PadButton = PadButton::Down;
pub const PAD_LEFT: PadButton = PadButton::Left;
pub const PAD_RIGHT: PadButton = PadButton::Right;
pub const PAD_RS: PadButton = PadButton::RS;
pub const PAD_Y: PadButton = PadButton::Y;

/// PAD_SKILLS sind die Skill-Slots 1 … 4 des Gamepads.
pub const PAD_SKILLS: [PadButton; 4] = [PAD_LB, PAD_RB, PAD_LT, PAD_UP];

/// PAD_ACTIONS ist die Menü-Belegung des Gamepads; `None` = keine Taste (Home ist die Kombi View + Menu). Y zählt nur
/// mit gehaltenem View als `Action::Cheats` (pad_chord).
pub const PAD_ACTIONS: [Option<PadButton>; ACTION_COUNT] = {
    let mut t = [None; ACTION_COUNT];
    t[Action::Up as usize] = Some(PAD_UP);
    t[Action::Down as usize] = Some(PAD_DOWN);
    t[Action::Left as usize] = Some(PAD_LEFT);
    t[Action::Right as usize] = Some(PAD_RIGHT);
    t[Action::Confirm as usize] = Some(PAD_A);
    t[Action::Back as usize] = Some(PAD_B);
    t[Action::Menu as usize] = Some(PAD_MENU);
    t[Action::View as usize] = Some(PAD_VIEW);
    t[Action::PrevPage as usize] = Some(PAD_LB);
    t[Action::NextPage as usize] = Some(PAD_RB);
    t[Action::Home as usize] = None;
    t[Action::SkillMenu as usize] = Some(PAD_DOWN);
    t[Action::Fullscreen as usize] = Some(PAD_RS);
    t[Action::Cheats as usize] = Some(PAD_Y);
    t
};

/// MENU_KEYS sind die Tasten, die in jedem Menü zusätzlich zum Profil gelten (B-945): Pfeile, Enter, Leertaste, Esc.
pub static MENU_KEYS: LazyLock<[Vec<Key>; ACTION_COUNT]> = LazyLock::new(|| {
    action_table(vec![
        (Action::Up, vec![Key::ArrowUp]),
        (Action::Down, vec![Key::ArrowDown]),
        (Action::Left, vec![Key::ArrowLeft]),
        (Action::Right, vec![Key::ArrowRight]),
        (Action::Confirm, vec![Key::Enter, Key::NumpadEnter, Key::Space]),
        (Action::Back, vec![Key::Escape]),
    ])
});

/// USED_KEYS sind alle Tasten beider Profile; darunter Einfg, die Marker-Taste des Spieltests (das Backend wird nur nach ihnen gefragt).
pub static USED_KEYS: LazyLock<Vec<Key>> = LazyLock::new(collect_keys);

fn collect_keys() -> Vec<Key> {
    let mut out: Vec<Key> = Vec::new();
    let mut add = |ks: &[Key]| {
        for &k in ks {
            if !out.contains(&k) {
                out.push(k);
            }
        }
    };
    for l in LAYOUT_TABLE.iter() {
        for ks in [&l.left, &l.right, &l.sprint, &l.pay, &l.attack] {
            add(ks);
        }
        for ks in &l.skills {
            add(ks);
        }
        for ks in &l.actions {
            add(ks);
        }
    }
    out
}

/// ActionKeys liefert die Tasten der Belegung layout (Layout: 0 wasd, 1 pfeile) für die Menü-Aktion a (leer = keine).
/// Für die Tastenbelegung und Abgleich-Tests anderer Pakete (Tasten-Texte im HUD, game/keys_test.go).
pub fn action_keys(layout: i64, a: Action) -> &'static [Key] {
    if layout < 0 || layout as usize >= LAYOUTS {
        return &[];
    }
    &LAYOUT_TABLE[layout as usize].actions[a as usize]
}

/// CommandKeys liefert die Tasten der Belegung layout für einen gehaltenen Befehl: left, right, sprint, pay, attack,
/// skill1 … skill4 (leer = unbekannt).
pub fn command_keys(layout: i64, name: &str) -> &'static [Key] {
    if layout < 0 || layout as usize >= LAYOUTS {
        return &[];
    }
    let l = &LAYOUT_TABLE[layout as usize];
    match name {
        "left" => &l.left,
        "right" => &l.right,
        "sprint" => &l.sprint,
        "pay" => &l.pay,
        "attack" => &l.attack,
        "skill1" | "skill2" | "skill3" | "skill4" => &l.skills[(name.as_bytes()[5] - b'1') as usize],
        _ => &[],
    }
}

#[cfg(test)]
#[path = "bindings_tests.rs"]
mod bindings_tests;
