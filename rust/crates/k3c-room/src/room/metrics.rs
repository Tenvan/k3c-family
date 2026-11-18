//! Spielmetrik je Raumlauf (B-150, S2.3; Schema in `docs/protocol.md` › Spielmetrik-Report). Port von `metrics.go`. Der
//! Sammler liest nach jedem Schritt nur Ereignisse, Zyklus, Zeit und Gold; er ändert nichts an der Insel und zieht keinen
//! Zufall. Monarchen erscheinen nur mit Index, Geräte nur mit Kürzel (`short`), Spielstand-Name und `campaignId` nie.
//!
//! Abweichungen von Go: Die Metriken nutzen die Uhr des Managers (`Stamp`), keine eigene; `SessionStore` ist ein Trait,
//! den `store::Reports` erfüllt; ganze Gleitkommazahlen schreibt `serde_json` als `1.0`, Go als `1`.
use std::collections::{BTreeMap, BTreeSet};

use k3c_sim::island::Island;
use k3c_sim::state::{Event, Player, World};
use serde::Serialize;

use super::env::Env;
use super::log::{Level, short};
use super::types::{Room, TICK_HZ};
use crate::fehler::Fehler;
use crate::store::{Reports, Stamp};

/// Die Ablage der Spielmetrik-Reports (Go `SessionStore`, `store::Reports`); die Antwort ist der Pfad.
pub trait SessionStore {
    fn store_session(&mut self, data: &[u8]) -> Result<String, Fehler>;
}

impl SessionStore for Reports {
    fn store_session(&mut self, data: &[u8]) -> Result<String, Fehler> {
        Reports::store_session(self, data).map(|p| p.display().to_string())
    }
}

/// Version des Reports; ein neues Feld erhöht sie.
pub const METRICS_SCHEMA: i64 = 1;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NightResult {
    pub day: i64,
    pub survived: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DeathEntry {
    pub monarch: i64,
    pub day: i64,
    pub phase: String,
    pub depth: i64,
    pub cause: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DayGold {
    pub day: i64,
    pub gold: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub struct Reached {
    pub day: i64,
    #[serde(rename = "maxDepth")]
    pub max_depth: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Disconnect {
    pub device: String,
    pub count: i64,
}

/// Schema 1 des Spielmetrik-Reports (Go `sessionReport`).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SessionReport {
    pub schema: i64,
    pub room: String,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "endedAt")]
    pub ended_at: String,
    #[serde(rename = "playSeconds")]
    pub play_seconds: f64,
    pub grade: String,
    pub goal: String,
    pub defeat: String,
    pub monarchs: i64,
    pub devices: Vec<String>,
    pub reached: Reached,
    pub nights: Vec<NightResult>,
    pub deaths: Vec<DeathEntry>,
    #[serde(rename = "firstBuildSeconds")]
    pub first_build_seconds: Option<f64>,
    #[serde(rename = "goldPerDay")]
    pub gold_per_day: Vec<DayGold>,
    #[serde(rename = "goldEnd")]
    pub gold_end: Vec<i64>,
    pub disconnects: Vec<Disconnect>,
}

/// Sammelt die Werte eines Raumlaufs (Go `metrics`); `Room::met` ist `None` ohne Sammler.
#[derive(Debug, Clone, Default)]
pub struct Metrics {
    started_at: Stamp,
    /// Kürzel.
    devices: BTreeSet<String>,
    /// Kürzel → Abbrüche.
    disconnects: BTreeMap<String, i64>,
    monarchs: usize,
    pub(super) reached: Reached,
    /// Laufende Nacht.
    night: Option<NightResult>,
    pub(super) nights: Vec<NightResult>,
    pub(super) deaths: Vec<DeathEntry>,
    first_build: Option<f64>,
    pub(super) gold_per_day: Vec<DayGold>,
}

fn golds(players: &[&Player]) -> Vec<i64> {
    players.iter().map(|p| p.gold).collect()
}

impl Metrics {
    /// Go `newMetrics`: `start` ist die Uhr des Managers beim Anlegen des Raums.
    pub fn new(start: Stamp) -> Metrics {
        Metrics {
            started_at: start,
            ..Metrics::default()
        }
    }

    /// Go `metrics.join`.
    pub fn join(&mut self, id: &str) {
        self.devices.insert(short(id));
    }

    /// Go `metrics.drop`: ein Gerät trennt sich.
    pub fn disconnect(&mut self, id: &str) {
        *self.disconnects.entry(short(id)).or_insert(0) += 1;
    }

    /// Go `metrics.observe`: liest die Ereignisse aller Stufen nach einem Schritt, in Stufenreihenfolge. Alle Stufen teilen
    /// den Zyklus und melden `night` und `dawn` gleichzeitig; jede Nacht und jeder Tag zählt nur einmal.
    pub fn observe(&mut self, isl: &Island) {
        let players = isl.players();
        self.monarchs = self.monarchs.max(players.len());
        for w in &isl.stages {
            self.reached.day = self.reached.day.max(w.cycle.day);
            if !w.players.is_empty() {
                self.reached.max_depth = self.reached.max_depth.max(w.biome.depth);
            }
            for ev in &w.events {
                self.event(w, ev, &players);
            }
        }
    }

    fn event(&mut self, w: &World, ev: &Event, players: &[&Player]) {
        match ev.get("type").and_then(|t| t.as_str()) {
            Some("night") => {
                if self.night.as_ref().is_none_or(|n| n.day != w.cycle.day) {
                    self.night = Some(NightResult {
                        day: w.cycle.day,
                        survived: true,
                    });
                }
            }
            Some("castleFallen") => {
                if let Some(n) = &mut self.night {
                    n.survived = false;
                }
            }
            Some("dawn") => {
                if let Some(n) = self.night.take() {
                    self.nights.push(n);
                }
                if self.gold_per_day.last().is_none_or(|g| g.day != w.cycle.day) {
                    self.gold_per_day.push(DayGold {
                        day: w.cycle.day,
                        gold: golds(players),
                    });
                }
            }
            Some("built") => {
                self.first_build.get_or_insert(w.time);
            }
            Some("crownFallen") => {
                let monarch = ev.get("player").and_then(|v| v.as_i64()).unwrap_or(0);
                let cause = ev.get("cause").and_then(|v| v.as_str()).unwrap_or("").to_string();
                self.deaths.push(DeathEntry {
                    monarch,
                    day: w.cycle.day,
                    phase: w.cycle.phase.clone(),
                    depth: w.biome.depth,
                    cause,
                });
            }
            _ => {}
        }
    }
}

/// Go `iso`: `2006-01-02T15:04:05.000Z07:00` in UTC.
fn iso(t: Stamp) -> String {
    t.format_report()
}

impl Room {
    /// Go `report`: baut Schema 1; Geräte sortiert, Monarchen nach Index. Eine bei Raumende laufende Nacht fehlt. Ohne
    /// Sammler `None`.
    pub fn report(&self, end: Stamp) -> Option<SessionReport> {
        let m = self.met.as_ref()?;
        let devices: Vec<String> = m.devices.iter().cloned().collect();
        let disconnects = devices
            .iter()
            .filter_map(|d| {
                let n = m.disconnects.get(d).copied().unwrap_or(0);
                (n > 0).then(|| Disconnect {
                    device: d.clone(),
                    count: n,
                })
            })
            .collect();
        let o = &self.isl.options;
        Some(SessionReport {
            schema: METRICS_SCHEMA,
            room: self.code.clone(),
            started_at: iso(m.started_at),
            ended_at: iso(end),
            play_seconds: self.tick as f64 / TICK_HZ as f64,
            grade: o.grade.clone(),
            goal: o.goal.clone(),
            defeat: o.defeat.clone(),
            monarchs: m.monarchs as i64,
            devices,
            reached: m.reached,
            nights: m.nights.clone(),
            deaths: m.deaths.clone(),
            first_build_seconds: m.first_build,
            gold_per_day: m.gold_per_day.clone(),
            gold_end: golds(&self.isl.players()),
            disconnects,
        })
    }

    /// Go `writeReport`: schreibt den Report beim Raumende (Aufräumen, Herunterfahren). Kein Report ohne Tick, ohne Ablage
    /// oder ohne Sammler; ein Fehler landet im Log, der Raum schließt trotzdem.
    pub fn write_report(&mut self, env: &mut Env) {
        if self.tick == 0 || env.sessions.is_none() {
            return;
        }
        let Some(report) = self.report(env.now) else {
            return;
        };
        let stored = serde_json::to_vec_pretty(&report)
            .map_err(|e| Fehler::Io(e.to_string()))
            .and_then(|data| match env.sessions.as_mut() {
                Some(s) => s.store_session(&data),
                None => Ok(String::new()),
            });
        let tick = self.tick;
        match stored {
            Ok(path) => self.log(
                env,
                Level::Info,
                "📄 Spielmetrik-Report geschrieben",
                &[("datei", &path), ("tick", &tick)],
            ),
            Err(e) => self.log(
                env,
                Level::Error,
                "💥 Spielmetrik-Report nicht geschrieben",
                &[("err", &e)],
            ),
        }
    }
}
