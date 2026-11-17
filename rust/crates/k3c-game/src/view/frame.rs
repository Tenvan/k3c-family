//! Port von `native/k3c-game/view/frame.go`: der Spielzustand eines Updates, wie ihn die Darstellung sieht.

use std::rc::Rc;

use k3c_sim::state::{Event as SimEvent, Player, World};

use super::cell::Cell;
use super::devkeys::DevView;
use super::input::DeviceID;
use super::settings::Settings;
use super::story::Campaign;

/// Frame ist der Spielzustand eines Updates, wie ihn die Darstellung sieht. local baut ihn, game ergänzt die Geräte
/// (unseated, Seat.lost, Geräte-Cues) und reicht ihn an Listener und Drawer. Im Menü ohne laufendes Spiel ist `stages`
/// leer, `cues`, `clock` und `settings` sind trotzdem gesetzt. Alles im Frame ist nur zu lesen und gilt bis zum nächsten
/// Update.
#[derive(Clone, Debug, Default)]
pub struct Frame {
    /// Takt der Simulation (30 Hz), steht in der Pause
    pub tick: u64,
    /// Sekunden für Animationen; Fenster: Spielzeit der Bilder, Aufnahme-Modus: aus dem Takt
    pub clock: f64,
    /// Spiel pausiert (Pause-Menü offen); Simulation steht
    pub paused: bool,
    /// Zeitraffer der Simulation (1 = normal)
    pub timescale: i64,
    /// Stufen, in denen mindestens ein lokaler Spieler steht, nach `Stage::index`
    pub stages: Vec<Stage>,
    /// Plätze der lokalen Spieler, nach Player; Zelle i zeigt `seats[i]`
    pub seats: Vec<Seat>,
    /// Ereignisse der Simulation seit dem letzten Frame, je genau einmal
    pub events: Vec<Event>,
    /// Ereignisse außerhalb der Simulation seit dem letzten Frame (Menü, Session, Geräte)
    pub cues: Vec<Cue>,
    /// Einstellungen (Session.SetSettings), auch ohne laufendes Spiel
    pub settings: Settings,
    /// verbundene Geräte ohne Platz, Reihenfolge wie Input (Beitritts-Hinweis im HUD)
    pub unseated: Vec<DeviceID>,
    /// Ausgang des laufenden bzw. zuletzt beendeten Spiels (local); er bleibt nach Session.Close stehen, bis ein neues
    /// Spiel startet. game schließt das Spiel `END_DELAY` Takte nachdem er erscheint.
    pub outcome: Outcome,
    /// Zellen dieses Updates mit Lage und Kamera (game, nach CameraRule und --cam), Index = Zelle; leer ohne Spiel
    pub cells: Vec<Cell>,
    /// Aufnahme-Modus (--shots): kein Ton, kein Vollbild, das Bild hängt nur vom Takt ab
    pub recording: bool,
    /// Zustand der Entwickler-Anzeigen (F1, F10; game, B-888)
    pub dev: DevView,
    /// Rechenzeit des letzten Simulationstakts in ms (local, Wanduhr; 0 ohne Takt; Debug-Werte, B-889)
    pub step_ms: f64,
    /// alle Inseln der Welt des laufenden Spiels in Listenreihenfolge (local, nur lesen; leer ohne Spiel): Grundlage der
    /// Seekarte am Hafen (B-962)
    pub route: Vec<RouteIsland>,
    /// Stand der Kampagne für die Geschichte (local, story.rs; leer ohne Spiel, B-974)
    pub campaign: Campaign,
    /// game zeigt in diesem Update die Ladeanzeige statt des Spiels (Einblendungen warten so lange, B-974)
    pub loading: bool,
}

/// RouteIsland ist eine Insel der Welt mit ihrer Etappe (>= 1; local setzt für 0 den Listenplatz + 1 wie die
/// Simulation). `world`, `stages` und `chunks` tragen das Aussehen der Insel auf der Seekarte (B-964): Welt-ID (leer =
/// Standard-Welt), Zahl der Stufen und größte Chunk-Anzahl einer Stufe (0 = ohne Angabe, Länge des Bioms).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RouteIsland {
    pub id: String,
    pub name: String,
    pub etappe: i64,
    pub world: String,
    pub stages: i64,
    pub chunks: i64,
}

/// Outcome ist der Ausgang eines Spiels.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Outcome {
    /// Spiel läuft bzw. kein Spiel
    #[default]
    None,
    /// Insel komplett verloren (Ereignis gameOver oder Insel ruht)
    Lost,
    /// Ziel der Insel erreicht (Ereignis victory)
    Won,
}

/// EndDelay sind die Takte (30 Hz) zwischen Spielende (`Frame::outcome`) und Hauptmenü.
pub const END_DELAY: i64 = 90;

/// Stage ist eine Stufe der Insel mit ihrer Welt.
#[derive(Clone, Debug, Default)]
pub struct Stage {
    /// Index der Stufe in der Insel (wie room.Seat.Stage)
    pub index: i64,
    /// Tiefe der Stufe (0 = Oberfläche)
    pub depth: i64,
    /// Level der Stufe = `World::biome.id`; Schlüssel für AssetSource und art
    pub level: String,
    /// nur lesen; `None` = Stufe ohne Welt
    pub world: Option<Rc<World>>,
}

/// Seat verbindet einen lokalen Spieler mit Gerät, Monarch und Stufe. `player` ist der Slot im Raum und bleibt fest,
/// solange der Spieler spielt; die Zelle ergibt sich aus der Lage in `Frame::seats` (`Cell::index`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Seat {
    /// lokaler Spieler (Slot) 0 … MAX_PLAYERS-1, nicht der Zellen-Index
    pub player: i64,
    /// steuerndes Gerät (für Tastensymbole im HUD)
    pub device: DeviceID,
    /// `Player::index` des Monarchen
    pub monarch: i64,
    /// `Stage::index` der Stufe, in der der Monarch steht
    pub stage: i64,
    /// Gerät getrennt: Monarch steht, ein freies Gerät übernimmt den Platz (Verbinden oder Bestätigen)
    pub lost: bool,
}

/// Event ist ein Ereignis der Simulation (`World::events`) mit seiner Stufe.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event {
    /// `Stage::index`
    pub stage: i64,
    /// Takt, in dem es entstand
    pub tick: u64,
    /// `{"type": "night", …}`, nur lesen
    pub data: SimEvent,
}

impl Event {
    /// Type liefert den Typ des Ereignisses ("" ohne Typ).
    pub fn type_(&self) -> &str {
        self.data.get("type").and_then(|v| v.as_str()).unwrap_or("")
    }
}

/// CueKind ist die Art eines Ereignisses außerhalb der Simulation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CueKind {
    /// Auswahl im Menü bewegt
    #[default]
    Move,
    /// Auswahl bestätigt
    Confirm,
    /// zurück
    Back,
    /// nicht möglich (z. B. Spielstand nicht ladbar)
    Denied,
    /// Spiel pausiert
    Pause,
    /// Spiel läuft weiter
    Resume,
    /// Spieler beigetreten (player gesetzt)
    Joined,
    /// Spieler gegangen (player gesetzt)
    Left,
    /// Spielstand gespeichert
    Saved,
    /// Speichern fehlgeschlagen (text: Grund)
    SaveFailed,
    /// Fehler, z. B. Panic im Takt (text: Meldung)
    Error,
    /// Gerät verbunden (device gesetzt; player, wenn es seinen Platz wieder hat)
    DeviceConnected,
    /// Gerät getrennt (device gesetzt; player, wenn es einen Platz hatte)
    DeviceLost,
    /// Spieler steuert jetzt ein anderes Gerät (player und device gesetzt)
    Rebound,
}

/// Cue ist ein Ereignis außerhalb der Simulation, für Ton und Hinweise.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Cue {
    pub kind: CueKind,
    /// betroffener Spieler, -1 = keiner
    pub player: i64,
    /// Meldung für Hinweise (deutsch), sonst leer
    pub text: String,
    /// betroffenes Gerät bei DeviceConnected, DeviceLost, Rebound
    pub device: DeviceID,
}

impl Frame {
    /// Stage liefert die Stufe mit Index i.
    pub fn stage(&self, i: i64) -> Option<&Stage> {
        self.stages.iter().find(|s| s.index == i)
    }

    /// Seat liefert den Platz des lokalen Spielers p.
    pub fn seat(&self, p: i64) -> Option<Seat> {
        self.seats.iter().find(|s| s.player == p).copied()
    }

    /// World liefert die Welt, in der der Spieler der Zelle c steht (None ohne Spieler oder Stufe).
    pub fn world(&self, c: &Cell) -> Option<&World> {
        let s = self.seat(c.player)?;
        self.stage(s.stage)?.world.as_deref()
    }

    /// Monarch liefert den Monarchen des lokalen Spielers p in seiner Welt (None, wenn es ihn nicht gibt).
    pub fn monarch(&self, p: i64) -> Option<&Player> {
        let s = self.seat(p)?;
        let w = self.stage(s.stage)?.world.as_deref()?;
        w.players.iter().find(|m| m.index == s.monarch)
    }

    /// EventsOf liefert die Ereignisse der Stufe i in Entstehungs-Reihenfolge, ohne Vec anzulegen.
    pub fn events_of(&self, i: i64) -> impl Iterator<Item = &Event> + '_ {
        self.events.iter().filter(move |e| e.stage == i)
    }
}

#[cfg(test)]
#[path = "frame_tests.rs"]
mod frame_tests;
