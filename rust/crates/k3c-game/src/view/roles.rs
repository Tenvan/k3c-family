//! Port von `native/k3c-game/view/roles.go`: optionale Rollen eines Fachpakets.
//!
//! game prüft jeden Beitrag eines Pakets (Drawer, Listener, Source, Menus) auf jede Rolle und meldet ihn an; ein Paket
//! braucht dafür keinen Eingriff in game (Go: Typ-Zusicherung, Rust: ein Trait-Objekt je Rolle bei der Anmeldung).
//! CameraRule gibt es höchstens einmal (world), AssetSource und Overlay beliebig oft.

use k3c_sim::state::World;

use super::MAX_PLAYERS;
use super::cell::{Camera, Cell};
use super::frame::{Cue, Frame};
use super::input::Input;
use super::session::Session;

/// CameraRule führt die Kamera einer Zelle (world, B-795: Totzone, Grenzen der Stufe). game ruft `follow` im Update
/// genau einmal je Zelle, nachdem der Frame gebaut ist, nie in Draw; ein Zustand in der Regel ist damit erlaubt.
/// `c` ist die Zelle mit Lage und Maßstab dieses Updates, ihre Kamera steht noch auf `prev.x`. `prev` ist die Kamera
/// desselben Spielers (`Seat::player`) aus dem vorigen Update; `prev.scale == 0` heißt: keine (erstes Bild, Spieler
/// neu), die Regel springt dann direkt ans Ziel. game übernimmt das Ergebnis als `c.camera` (ein NaN- oder
/// unendliches X verwirft es). Der Szenen-Schalter --cam setzt X danach fest.
pub trait CameraRule {
    fn follow(&mut self, f: &Frame, c: &Cell, prev: Camera) -> Camera;
}

/// AssetSource meldet die Papier-Assets (`gruppe/id`, wie art.PlanLevel), die ein Fachpaket in einem Level braucht.
/// game fragt alle Quellen, sobald ein Level in `Frame::stages` neu auftaucht (`w` ist dessen Welt, nur lesen), lädt die
/// Vereinigung über art.Library.SetLevels und gibt frei, was kein Level mehr braucht. Die Antwort muss alles nennen,
/// was das Paket im Level zeichnen kann; fehlt eine Datei, beendet sich das Spiel mit dem Pfad.
pub trait AssetSource {
    fn assets(&self, level: &str, w: &World) -> Vec<String>;
}

/// Overlay ist ein Menü im laufenden Spiel je Spieler (hud: Skill-Menü, B-799). game ruft `update` in jedem Update im
/// Spiel (kein Screen offen) vor Pause, Beitritt und Befehlen, mit der Eingabe aller Geräte und dem Frame des vorigen
/// Updates. Das Overlay bedient die Session selbst (Learn, Respec).
pub trait Overlay {
    fn update(&mut self, input: &Input, f: &Frame, s: &mut dyn Session) -> OverlayStep;
}

/// OverlayStep ist das Ergebnis eines `Overlay::update`. Für einen verbrauchten Spieler gibt game in diesem Update einen
/// leeren Befehl (Monarch steht) und wertet die Aktionen seines Geräts nicht aus (kein Pause-Menü, kein Home).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OverlayStep {
    /// Index = `Seat::player`
    pub consumed: [bool; MAX_PLAYERS],
    /// z. B. Move, Confirm, Denied; game legt sie in den nächsten Frame
    pub cues: Vec<Cue>,
}

/// Finisher ist eine optionale Rolle eines Listeners: game ruft `finish` einmal beim Programmende, bevor das Spiel
/// geschlossen wird (Sitzungsbericht der Chronik, devtools).
pub trait Finisher {
    fn finish(&mut self);
}

/// ReportsDir ist der Ordner der Berichte und Screenshots des nativen Spiels (relativ zum Arbeitsordner, wie
/// task native:game; Workbench: report_read).
pub const REPORTS_DIR: &str = "reports";
