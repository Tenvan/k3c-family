//! Port von `native/k3c-game/view/session.go`: das Spiel aus Sicht der Menüs.

use std::fmt;
use std::time::SystemTime;

use k3c_sim::island::chronicle::ChronicleEntry;

use super::frame::{RouteIsland, Seat};
use super::input::DeviceID;
use super::settings::Settings;

/// Fehler der Session (Leitfaden § 3): deutsch und anzeigbar, Text wie in Go.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fehler {
    /// Meldung für die Anzeige
    Session(String),
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fehler::Session(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Fehler {}

/// NewGame sind die Angaben für ein neues Spiel (wie room.Options; leer = Standard).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NewGame {
    /// Name des Spielstands (Slot); leer = local wählt einen freien
    pub name: String,
    /// leer = Name als Seed
    pub seed: String,
    /// Schwierigkeitsgrad
    pub grade: String,
    /// Ziel der Insel
    pub goal: String,
    /// Niederlage-Regel
    pub defeat: String,
    /// Welt-ID (wie room.Options.World); leer = Standard-Welt
    pub world: String,
    /// Tiefe der Stufe, in der die Spieler beginnen (0 = Oberfläche; Aufnahme-Modus, Tests)
    pub depth: i64,
    /// Geräte der Spieler 1 … MAX_PLAYERS in dieser Reihenfolge, mindestens eins
    pub devices: Vec<DeviceID>,
}

/// SaveInfo ist ein Eintrag der Spielstand-Liste.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SaveInfo {
    /// Slot, Schlüssel für Load und Delete
    pub name: String,
    /// Zeitpunkt des letzten Speicherns (`None`, wenn unbekannt)
    pub saved_at: Option<SystemTime>,
    /// Spieltag
    pub day: i64,
    /// Tiefen der Spieler, sortiert, jede einmal
    pub depths: Vec<i64>,
    /// Welt-ID des Spielstands; leer = Standard-Welt (B-938)
    pub world: String,
    /// Schwierigkeitsgrad des Spielstands; leer = alter Stand ohne Angabe (Anzeige: Standardgrad, ECO1.2)
    pub grade: String,
    /// false: kaputt oder alte Version; dann steht der Grund in `problem`
    pub loadable: bool,
    /// deutsch, für die Anzeige
    pub problem: String,
    /// Vorschau (GS3.8): die Inseln der Welt (leer bei nicht ladbarem Stand oder unbekannter Welt: keine Vorschau)
    pub route: Vec<RouteIsland>,
    /// Vorschau: Insel, an der das Schiff liegt (`islandId` des Stands; leer = unbekannt)
    pub island: String,
    /// Vorschau: abgeschlossene Inseln in der Reihenfolge des Abschlusses
    pub completed: Vec<String>,
    /// Vorschau: Chronik der abgeschlossenen Inseln (Tage, Monarchen)
    pub chronicle: Vec<ChronicleEntry>,
}

/// WorldInfo ist ein Eintrag der Weltauswahl (wie room.WorldInfo).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorldInfo {
    pub id: String,
    pub name: String,
    /// Grund, warum die Welt nicht ladbar ist (nur Dev-Mode); leer = ladbar
    pub error: String,
    /// Welt aus `<repo>/data/worlds` statt aus dem Paket (Start aus dem Repo-dist, B-1210): Dev-Symbol in der Auswahl
    pub repo: bool,
}

/// Session ist das Spiel aus Sicht der Menüs; local erfüllt sie. Alle Methoden laufen in der Schleife und blockieren
/// nur kurz (Dateizugriff). Fehler sind deutsch und anzeigbar.
pub trait Session {
    /// neues Spiel anlegen und starten (ersetzt ein laufendes nach dessen Speichern)
    fn new_game(&mut self, g: NewGame) -> Result<(), Fehler>;
    /// Zuletzt gespeicherten Spielstand laden und starten, mit den Geräten `devices` (Spieler 1 … in dieser
    /// Reihenfolge); leer = Geräte des letzten Spiels, ohne letztes Spiel die linke Tastatur.
    fn continue_game(&mut self, devices: &[DeviceID]) -> Result<(), Fehler>;
    /// Spielstand laden und starten, Geräte wie bei `continue_game`
    fn load(&mut self, name: &str, devices: &[DeviceID]) -> Result<(), Fehler>;
    fn list(&mut self) -> Result<Vec<SaveInfo>, Fehler>;
    /// nicht den Spielstand des laufenden Spiels
    fn delete(&mut self, name: &str) -> Result<(), Fehler>;
    /// laufendes Spiel jetzt speichern (zusätzlich zum Autosave)
    fn save(&mut self) -> Result<(), Fehler>;
    /// speichern und Spiel beenden; danach `active() == false`
    fn close(&mut self) -> Result<(), Fehler>;
    /// es läuft ein Spiel (auch pausiert)
    fn active(&self) -> bool;
    /// Simulation anhalten bzw. weiterlaufen lassen
    fn pause(&mut self, on: bool);
    /// Gibt den Spieler (Slot) zurück, der dem Gerät zugeordnet wurde.
    fn join(&mut self, d: DeviceID) -> Result<i64, Fehler>;
    fn leave(&mut self, player: i64) -> Result<(), Fehler>;
    /// Gibt dem Spieler ein anderes Gerät (verlorenes Pad, B-798); das Gerät darf keinen anderen Platz haben. Meldet
    /// `Rebound`. game ruft es nach der Zuordnungs-Regel auf, Menüs dürfen es auch.
    fn rebind(&mut self, player: i64, d: DeviceID) -> Result<(), Fehler>;
    /// Lernt für den Spieler den Skill (Skill-ID der Simulation, wie sim.LearnSkill). Ob es geht, prüft die
    /// Simulation (Punkte, Tag, Burg); abgelehnt kommt ein anzeigbarer Fehler (Skill-Menü des HUD).
    fn learn(&mut self, player: i64, skill: &str) -> Result<(), Fehler>;
    /// Setzt die Skills des Spielers zurück.
    fn respec(&mut self, player: i64) -> Result<(), Fehler>;
    /// Wählt für den Spieler das Ziel des Inselwechsels am Hafen (Insel-ID aus islandSwitch.targets, B-962); die
    /// Simulation prüft Wechselpunkt, Krone und Ziel, abgelehnt kommt ein anzeigbarer Fehler.
    fn choose_island(&mut self, player: i64, island: &str) -> Result<(), Fehler>;
    /// Übernimmt die Einstellungen (normalisiert) sofort und speichert sie; ein Fehler betrifft nur das Speichern, die
    /// Werte gelten trotzdem.
    fn set_settings(&mut self, s: Settings) -> Result<(), Fehler>;
    /// aktuelle Einstellungen (wie `Frame::settings` des nächsten Frames), auch ohne Spiel
    fn settings(&self) -> Settings;
    /// Plätze des laufenden Spiels nach Player (device, monarch, stage; `Seat::lost` setzt nur game im Frame); leer
    /// ohne Spiel.
    fn seats(&self) -> Vec<Seat>;
    /// Welten, die ein neues Spiel wählen darf (Standard-Welt zuerst; weitere nur, wenn engine/room sie zulässt, B-938).
    fn world_list(&self) -> Vec<WorldInfo>;
    /// Welt-ID des laufenden Spiels (leer ohne Spiel)
    fn world(&self) -> String;
    /// Go: Typ-Zusicherung `s.(DevSession)`: die Entwickler-Rolle der Session; `None`, wenn sie keine hat.
    fn dev_role(&mut self) -> Option<&mut dyn super::devkeys::DevSession> {
        None
    }
}
