//! Port von `native/k3c-game/view/keys.go`.
//!
//! Tastenbelegung für die Seite „Tasten“ im Pause-Menü (B-807): game baut die Zeilen aus input (Tasten) und hud
//! (Beschriftungen) und reicht sie an menu (menu.New); menu kennt weder input noch hud.

// Aktionen der Tastenbelegung (KeyRow.Action) in der Reihenfolge der Seite; menu beschriftet sie in der Sprache.
/// laufen (links/rechts)
pub const KEY_MOVE: &str = "move";
/// Galopp
pub const KEY_SPRINT: &str = "sprint";
/// zahlen, bestätigen
pub const KEY_PAY: &str = "pay";
/// Schlag
pub const KEY_ATTACK: &str = "attack";
/// Skill-Slot 1 … 4
pub const KEY_SKILL1: &str = "skill1";
pub const KEY_SKILL2: &str = "skill2";
pub const KEY_SKILL3: &str = "skill3";
pub const KEY_SKILL4: &str = "skill4";
/// Skill-Menü
pub const KEY_SKILL_MENU: &str = "skillMenu";
/// Pause-Menü
pub const KEY_PAUSE: &str = "pause";
/// Vollbild
pub const KEY_FULLSCREEN: &str = "fullscreen";

/// KeyActions sind alle Aktionen der Tastenbelegung in der Reihenfolge der Seite.
pub const KEY_ACTIONS: [&str; 11] = [
    KEY_MOVE,
    KEY_SPRINT,
    KEY_PAY,
    KEY_ATTACK,
    KEY_SKILL1,
    KEY_SKILL2,
    KEY_SKILL3,
    KEY_SKILL4,
    KEY_SKILL_MENU,
    KEY_PAUSE,
    KEY_FULLSCREEN,
];

/// KeyRow ist eine Zeile der Tastenbelegung: Aktion (KeyActions) und die fertig beschrifteten Tasten der linken und
/// rechten Tastatur-Hälfte und des Gamepads (Standardbelegung, auch ohne angeschlossenes Pad); "" = nicht belegt.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KeyRow {
    pub action: String,
    pub left: String,
    pub right: String,
    pub pad: String,
}

/// KeyTable liefert die Tastenbelegung in der Sprache lang (DefaultLanguage oder LanguageEnglish), eine Zeile je
/// Eintrag von KeyActions. Der Vec gehört dem Aufrufer.
pub type KeyTable = Box<dyn Fn(&str) -> Vec<KeyRow>>;
