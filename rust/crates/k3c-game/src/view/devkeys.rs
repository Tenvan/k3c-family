//! Port von `native/k3c-game/view/devkeys.go`.
//!
//! Entwickler-Tasten (B-888): eine Tabelle für Auswertung (game, input) und Tastenübersicht (F1, devtools).
//! Spieltasten stehen in input/bindings.rs und auf der Seite „Tasten“ des Pause-Menüs. Keine dieser Tasten wirkt im
//! Aufnahme-Modus.

use k3c_platform::Key;

use super::input::Action;

/// DevCmd ist der Befehl einer Entwickler-Taste.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DevCmd {
    #[default]
    None,
    /// Tastenübersicht ein/aus
    Help,
    /// Leistungsanzeige ein/aus (Einstellungen)
    Stats,
    /// Fluss-Fassung durchschalten (Einstellungen)
    River,
    /// Schnellspeichern
    Save,
    /// Spielstand neu laden (letzter gespeicherter Stand des laufenden Spiels)
    Load,
    /// Pause; in der Pause ein Takt Einzelschritt
    Pause,
    /// Zeitraffer 1× → 2× → 4× → 8× → 1×; in der Pause: weiter
    Speed,
    /// Cheat-Menü des Spielers ein/aus (input: ActionCheats, Pad View + Y)
    Cheats,
    /// Debug-Overlay: aus → Geometrie → Werte → aus
    Debug,
    /// Vollbild umschalten (input: ActionFullscreen, Pad rechter Stick)
    Fullscreen,
    /// Screenshot nach reports/
    Shot,
    /// Cheat-Fenster öffnen bzw. nach vorn holen (RU56)
    Window,
}

/// DevKey ist eine Zeile der Tabelle: Taste, Befehl, Beschriftung in der Übersicht und, falls die Taste über input als
/// Geräte-Aktion läuft (Cheat-Menü je Spieler, Vollbild), deren Aktion; sonst liest game die Taste direkt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DevKey {
    pub key: Key,
    pub cmd: DevCmd,
    /// Tastenname, z. B. "F1"
    pub label: &'static str,
    /// Wirkung, deutsch
    pub text: &'static str,
    /// Gamepad-Kombi ("" = keine)
    pub pad: &'static str,
    /// Geräte-Aktion über input; `None` (Go: -1) = game liest die Taste selbst
    pub action: Option<Action>,
}

const fn dev(key: Key, cmd: DevCmd, label: &'static str, text: &'static str) -> DevKey {
    DevKey {
        key,
        cmd,
        label,
        text,
        pad: "",
        action: None,
    }
}

/// DevKeys ist die Tabelle der Entwickler-Tasten in der Reihenfolge der Übersicht.
pub const DEV_KEYS: [DevKey; 12] = [
    dev(Key::F1, DevCmd::Help, "F1", "Tastenübersicht ein/aus"),
    dev(Key::F2, DevCmd::Stats, "F2", "Leistungsanzeige ein/aus"),
    dev(Key::F3, DevCmd::River, "F3", "Fluss-Fassung wechseln"),
    dev(Key::F4, DevCmd::Window, "F4", "Cheat-Fenster öffnen / nach vorn"),
    dev(Key::F5, DevCmd::Save, "F5", "Schnellspeichern"),
    dev(Key::F6, DevCmd::Load, "F6", "Spielstand neu laden"),
    dev(Key::F7, DevCmd::Pause, "F7", "Pause · in der Pause: ein Takt"),
    dev(
        Key::F8,
        DevCmd::Speed,
        "F8",
        "Zeitraffer 1× 2× 4× 8× · beendet die Pause",
    ),
    DevKey {
        pad: "View + Y",
        action: Some(Action::Cheats),
        ..dev(Key::F9, DevCmd::Cheats, "F9", "Cheat-Menü ein/aus")
    },
    dev(Key::F10, DevCmd::Debug, "F10", "Debug-Overlay: Geometrie · Werte · aus"),
    DevKey {
        pad: "rechter Stick",
        action: Some(Action::Fullscreen),
        ..dev(Key::F11, DevCmd::Fullscreen, "F11", "Vollbild ein/aus")
    },
    dev(Key::F12, DevCmd::Shot, "F12", "Screenshot nach reports/"),
];

/// DevKeyOf liefert die Taste des Befehls c (`None` statt `ebiten.KeyMax`, wenn er fehlt).
pub fn dev_key_of(c: DevCmd) -> Option<Key> {
    DEV_KEYS.iter().find(|k| k.cmd == c).map(|k| k.key)
}

/// Timescales sind die Stufen des Zeitraffers (F8, Cheat-Menü), wie room.MaxTimescale.
pub const TIMESCALES: [i64; 4] = [1, 2, 4, 8];

/// NextTimescale liefert die Stufe nach cur (8 → 1; unbekannt → 2).
pub fn next_timescale(cur: i64) -> i64 {
    for (i, f) in TIMESCALES.iter().enumerate() {
        if *f == cur {
            return TIMESCALES[(i + 1) % TIMESCALES.len()];
        }
    }
    TIMESCALES[1]
}

/// DevView ist der Zustand der Entwickler-Anzeigen eines Frames (game setzt ihn nach Session.Step).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DevView {
    /// Tastenübersicht (F1)
    pub help: bool,
    /// Debug-Overlay (F10): DebugOff, DebugGeometry, DebugValues
    pub debug: i64,
    /// gemessene Takte je Sekunde (game, Wanduhr; Seite Werte)
    pub tps: f64,
    /// Pick ist die per Maus gewählte Stelle: Spieler der Zelle und Welt-x (Picked false: keine Wahl, dann gilt das
    /// Objekt am Monarchen der Zelle).
    pub pick: Pick,
}

/// Pick ist eine per Maus gewählte Stelle in der Welt eines Spielers.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pick {
    pub picked: bool,
    pub player: i64,
    pub x: f64,
}

// Seiten des Debug-Overlays.
pub const DEBUG_OFF: i64 = 0;
pub const DEBUG_GEOMETRY: i64 = 1;
pub const DEBUG_VALUES: i64 = 2;
const DEBUG_PAGES: i64 = 3;

/// NextDebug liefert die Seite nach cur (Werte → aus).
pub fn next_debug(cur: i64) -> i64 {
    (cur + 1) % DEBUG_PAGES
}

/// DevAction ist eine Dev-Aktion an die Simulation (Cheat-Menü, F8), wie room.DevAction ohne Slot: Action und die
/// Felder, die sie braucht (engine/room/dev.go).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DevAction {
    pub action: String,
    pub amount: i64,
    pub resource: String,
    pub factor: i64,
    pub phase: String,
    pub grade: String,
    pub on: Option<bool>,
    pub level: i64,
    pub boss: String,
    pub event: String,
    pub kind: String,
    pub unit: String,
}

/// DevSession ist eine optionale Rolle der Session für die Entwickler-Tasten und das Cheat-Menü (local erfüllt sie).
pub trait DevSession {
    /// Dev führt die Aktion für den Spieler player aus; raumweite Aktionen (Phase, Zeitraffer) ignorieren player.
    fn dev(&mut self, player: i64, a: &DevAction) -> Result<(), String>;
    /// Halt hält die Simulation an bzw. lässt sie weiterlaufen (F7, getrennt von der Pause des Menüs).
    fn halt(&mut self, on: bool);
    /// Halted meldet, ob Halt gilt.
    fn halted(&self) -> bool;
    /// StepOnce lässt im Halt den nächsten Step genau einen Takt rechnen.
    fn step_once(&mut self);
    /// Reload lädt den zuletzt gespeicherten Stand des laufenden Spiels, ohne vorher zu speichern (F6).
    fn reload(&mut self) -> Result<(), String>;
}

#[cfg(test)]
#[path = "devkeys_tests.rs"]
mod devkeys_tests;
