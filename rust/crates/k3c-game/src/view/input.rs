//! Port von `native/k3c-game/view/input.go`: Geräte, Befehle, Menü-Aktionen und die Quelle der Eingabe.

use std::fmt;

use k3c_platform::InputState;
use k3c_sim::state::PlayerCommand;

/// DeviceKind ist die Art eines Eingabegeräts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DeviceKind {
    /// Tastatur; ein Gerät (Index 0), Belegung nach Tastatur-Profil (B-893)
    #[default]
    Keyboard,
    /// Gamepad; Index = Pad-ID der Plattform (stabil, solange verbunden; neu verbunden: neue ID)
    Gamepad,
}

/// DeviceID benennt ein Eingabegerät. game ordnet Geräte Spielern zu (Seat.Device, Session.Rebind), input kennt keine
/// Spieler. Ein Pad bekommt nach Trennen und Wiederverbinden meist eine neue ID; game ordnet es dann über die
/// Zuordnungs-Regel dem verwaisten Platz zu, nie über einen festen Index.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeviceID {
    pub kind: DeviceKind,
    /// Tastatur: immer 0; Gamepad: Pad-ID der Plattform (`PadState::id`)
    pub index: i64,
}

impl DeviceID {
    pub const fn new(kind: DeviceKind, index: i64) -> Self {
        Self { kind, index }
    }
}

impl fmt::Display for DeviceID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.kind == DeviceKind::Gamepad {
            return write!(f, "pad{}", self.index + 1);
        }
        write!(f, "keys{}", self.index + 1)
    }
}

/// Command ist die gehaltene Spiel-Eingabe eines Geräts in einem Update (Stand, keine Flanke).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Command {
    /// -1 (links) bis 1 (rechts), Totzone schon abgezogen
    pub move_: f64,
    pub sprint: bool,
    /// Bezahl-Taste gehalten
    pub pay: bool,
    pub attack: bool,
    /// aktiver Skill-Slot 1 bis 4, 0 = keiner
    pub skill: i64,
}

impl Command {
    /// Sim liefert den Befehl der Simulation; move wird auf -1 … 1 begrenzt, NaN zählt als 0, Skill außerhalb 1 … 4 als 0.
    pub fn sim(&self) -> PlayerCommand {
        let m = if self.move_.is_nan() {
            0.0
        } else {
            self.move_.clamp(-1.0, 1.0)
        };
        let skill = if (0..=4).contains(&self.skill) { self.skill } else { 0 };
        PlayerCommand {
            move_x: m,
            sprint: self.sprint,
            pay: self.pay,
            attack: self.attack,
            skill,
        }
    }
}

/// Action ist eine Menü-Aktion als Flanke (einmal je Druck, Wiederholung beim Halten liefert input).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    /// A / Enter
    Confirm,
    /// B / Esc: in jedem Menü zurück, im laufenden Spiel unbelegt
    Back,
    /// Menu / Esc im Spiel: Pause-Menü öffnen bzw. schließen
    Menu,
    /// View / Tab
    View,
    /// LB / Q
    PrevPage,
    /// RB / E
    NextPage,
    /// View + Menu gemeinsam / Pos1: zurück ins Menü (im Spiel wie ActionMenu; in Menüs wertet der Screen sie aus)
    Home,
    /// ActionSkillMenu öffnet das Skill-Menü im Spiel (Steuerkreuz ↓ / K / Num 5, wie src/input/slotBindings.ts).
    /// input kennt Spiel und Menü nicht: Steuerkreuz ↓ liefert immer ActionDown und ActionSkillMenu. Nur das Overlay
    /// (hud) wertet ActionSkillMenu aus, also nur im Spiel; Menüs und Overlays mit offenem Menü nehmen ActionDown.
    SkillMenu,
    /// F11 / rechter Stick gedrückt: Vollbild umschalten (game über Session.SetSettings, nicht im Aufnahme-Modus)
    Fullscreen,
    /// ActionCheats öffnet bzw. schließt das Cheat-Menü des Spielers (F9 auf der linken Tastatur-Hälfte / View + Y,
    /// B-888); nur das Overlay devtools wertet sie aus.
    Cheats,
}

/// DeviceInput ist die Eingabe eines verbundenen Geräts in einem Update.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceInput {
    pub device: DeviceID,
    pub command: Command,
    /// Flanken dieses Updates in Druck-Reihenfolge
    pub actions: Vec<Action>,
}

/// Input ist die Eingabe aller verbundenen Geräte eines Updates, Tastatur zuerst, dann Gamepads nach Index.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Input {
    pub devices: Vec<DeviceInput>,
}

impl Input {
    /// Of liefert die Eingabe des Geräts d (leer, wenn es nicht verbunden ist).
    pub fn of(&self, d: DeviceID) -> (DeviceInput, bool) {
        for di in &self.devices {
            if di.device == d {
                return (di.clone(), true);
            }
        }
        (
            DeviceInput {
                device: d,
                ..Default::default()
            },
            false,
        )
    }

    /// Has meldet, ob irgendein Gerät die Aktion a ausgelöst hat; dev ist das erste solche Gerät.
    pub fn has(&self, a: Action) -> (DeviceID, bool) {
        for di in &self.devices {
            if di.actions.contains(&a) {
                return (di.device, true);
            }
        }
        (DeviceID::default(), false)
    }
}

/// Source liefert die Eingabe je Update (input: Tastatur und Gamepads; Aufnahme-Modus: feste Folge je Takt).
/// `raw` ist der Eingabe-Schnappschuss der Plattform für diesen Takt (in Go las die Quelle Ebitengine direkt).
pub trait Source {
    fn poll(&mut self, raw: &InputState) -> Input;
}

/// KeyboardSource ist eine Quelle mit Tastatur, deren Belegung dem Tastatur-Profil folgt (input, B-893); game stellt
/// sie vor jedem Poll auf Settings.KeyboardProfile.
pub trait KeyboardSource {
    fn set_keyboard(&mut self, profile: &str);
    /// SetMenu meldet, ob ein Menü offen ist: dann gelten Pfeile, Enter, Leertaste, Esc zusätzlich (B-945).
    fn set_menu(&mut self, open: bool);
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod input_tests;
