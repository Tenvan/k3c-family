//! Port von `native/k3c-game/local/campaign.go`: Die Sitzung führt den Stand der Kampagne für `Frame::campaign` mit
//! (B-974): Welt, aktuelle Insel und früher abgeschlossene Inseln. Der Raum gibt die Insel nicht heraus; der Stand kommt
//! beim neuen Spiel aus der Welt (Startinsel), beim Laden aus dem Spielstand (`islandId`, `completed`) und folgt danach
//! dem Ereignis `islandSwitch`. Es rechnet keine Regel: Welche Zeilen an einem Wendepunkt gelten, sagt die Engine.
//!
//! Abweichung von Go: `view::Campaign` hat noch kein Feld `world` (Offen RU10, `WorldDef` fehlt dort); `Campaign::def`
//! hält die Welt, bis es das Feld gibt. Bis RU19 `standard_world` liefert, fängt `read_world_def` den Abbruch ab (wie Go: Welt
//! nicht lesbar, die Geschichte entfällt, das Spiel läuft).

use k3c_room::fehler::Fehler as RoomFehler;
use k3c_room::room::Log as _;
use k3c_room::room::Seat;
use k3c_sim::island::WorldDef;
use serde::Deserialize;

use super::peer::StageState;
use super::route::read_world_def;
use super::session::Session;
use crate::view::story::Campaign as ViewCampaign;

/// Der Stand der Kampagne (Go: `campaign`).
#[derive(Default)]
pub struct Campaign {
    pub run: u64,
    /// Die Welt der Kampagne (`None`, solange sie nicht gelesen ist)
    pub def: Option<WorldDef>,
    /// `def` ist gelesen
    pub ok: bool,
    pub island: String,
    pub completed: Vec<String>,
    /// neue Kampagne, noch nicht im Frame gemeldet
    pub fresh: bool,
}

/// Die Teile des Spielstands, die die Kampagne braucht.
#[derive(Deserialize, Default)]
pub(super) struct Pos {
    #[serde(rename = "islandId", default)]
    pub island_id: String,
    #[serde(default)]
    pub completed: Vec<String>,
}

impl Campaign {
    /// Führt die Insel nach einem Takt nach: Meldet er `islandSwitch` und stehen die Spieler nicht mehr an einem
    /// Wechselpunkt (der Raum hat die Insel getauscht; die neue hat noch keinen), gilt die verlassene als abgeschlossen
    /// und das Ziel als aktuelle Insel.
    pub fn follow(&mut self, st: &StageState, seats: &[Seat]) {
        for e in &st.events {
            if e.type_() != "islandSwitch" {
                continue;
            }
            let to = e.data.get("target").and_then(|v| v.as_str()).unwrap_or("");
            if to.is_empty() || to == self.island || st.at_gate(seats) {
                return;
            }
            if !self.island.is_empty() && !self.completed.contains(&self.island) {
                self.completed.push(self.island.clone());
            }
            self.island = to.to_string();
            return;
        }
    }

    /// Liefert den Stand für den Frame; `ready` = der Frame hat Stufen und Plätze (erst dann gilt die neue Kampagne als
    /// gemeldet).
    pub fn frame(&mut self, ready: bool) -> ViewCampaign {
        let mut out = ViewCampaign {
            run: self.run,
            island: self.island.clone(),
            completed: self.completed.clone(),
            fresh: false,
        };
        if self.fresh && ready {
            out.fresh = true;
            self.fresh = false;
        }
        out
    }
}

impl Session {
    /// Setzt den Stand zum Start des Spiels `name` (nach `start`, `self.world` steht). Eine unlesbare Welt oder ein
    /// unlesbarer Spielstand lässt die Geschichte aus, das Spiel läuft.
    pub(super) fn begin_campaign(&mut self, name: &str, fresh: bool) {
        self.camp = Campaign {
            run: self.camp.run + 1,
            fresh,
            ..Campaign::default()
        };
        let world = self.world.clone();
        let dir = self.worlds_dir.clone();
        let def = match read_world_def(dir.as_deref(), &world) {
            Ok(def) => def,
            Err(e) => {
                self.log.warn(
                    "💥 Welt für die Geschichte nicht gelesen",
                    &[("world", &world), ("err", &e)],
                );
                return;
            }
        };
        let first = def.islands.first().map(|i| i.id.clone());
        (self.camp.def, self.camp.ok) = (Some(def), true);
        if fresh {
            self.camp.island = first.unwrap_or_default();
            return;
        }
        match self.read_position(name) {
            Ok(pos) => (self.camp.island, self.camp.completed) = (pos.island_id, pos.completed),
            Err(e) => {
                self.log.warn(
                    "💥 Spielstand für die Geschichte nicht gelesen",
                    &[("save", &name), ("err", &e)],
                );
                self.camp.ok = false;
            }
        }
    }

    /// Liest Insel und abgeschlossene Inseln aus dem Spielstand `name`.
    pub(super) fn read_position(&self, name: &str) -> Result<Pos, String> {
        let raw = self.saves.load(name).map_err(|e: RoomFehler| e.to_string())?;
        serde_json::from_slice(&raw).map_err(|e| e.to_string())
    }
}
