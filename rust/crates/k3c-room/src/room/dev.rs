//! Dev-Aktionen eines Geräts (Nachricht `dev`, B-178). Port von `dev.go`: `gold` und `material` brauchen `slot` und
//! `amount`, `material` zusätzlich `resource`, `timescale` braucht `factor`, `pause` braucht `paused` (B-231); `wave` und
//! `castleFall` (Stufenverlust in der Stufe des Monarchen, WZ14.5) brauchen `slot`, `phase` braucht `phase` (`day`, `dusk`,
//! `night`, `dawn`, B-232); `grade` braucht `grade` (Name aus `data/difficulty.json`, gilt ab der nächsten Welle, B-080).
//! `maxStock` (+100 je Aufruf), `fillStock` und `nextPhase` brauchen nichts; `cheapBuild` und `timescaleAuto` brauchen
//! `on`, `buildAll` braucht `level` (1 bis 5), `pure` braucht `on` (reiner Stressmodus, B-921). Kampf (B-603,
//! `dev_combat.go`, RU24.2): `boss`, `event`, `bossPhase`, `night`, `spawn`, `spawnCastle` (siehe `pending.rs`).
//! `dev_extra.rs` führt die selteneren Aktionen aus.
use std::collections::BTreeMap;
use std::fmt::Display;

use k3c_sim::dev::{dev_castle_fall, dev_set_phase, dev_start_wave};
use k3c_sim::island::set_grade;
use serde::{Deserialize, Serialize};

use super::env::Env;
use super::log::{Level, short};
use super::types::Room;
use crate::fehler::Fehler;

/// Eine Dev-Aktion eines Geräts. Felder und Reihenfolge wie `DevAction` in Go (ohne Tags: die Namen des Protokolls sind
/// die kleingeschriebenen Feldnamen).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DevAction {
    pub action: String,
    pub slot: Option<i64>,
    pub amount: i64,
    pub resource: String,
    pub factor: i64,
    pub paused: Option<bool>,
    pub phase: String,
    pub grade: String,
    pub on: Option<bool>,
    pub level: i64,
    pub boss: String,
    pub event: String,
    /// `kind` und `unit` (B-766): `spawn` braucht `slot`, `kind` (`enemy`, `troop`), `unit` (Schlüssel aus
    /// `data/enemies.json` bzw. `troops.json`) und `amount` (1 bis `DEV_MAX_SPAWN`).
    pub kind: String,
    pub unit: String,
}

/// Obergrenze von `amount` (`gold`, `material`).
pub const DEV_MAX_AMOUNT: i64 = 1000;

/// Größter Faktor des Zeitraffers; er schützt das Tick-Budget (B-178).
pub const MAX_TIMESCALE: i64 = 8;

/// Erlaubte Faktoren von `timescale`.
const DEV_TIMESCALES: [i64; 4] = [1, 2, 4, MAX_TIMESCALE];

/// Bildet den `slot` der Aktion auf den Spielerindex der Insel ab: beim Gerät über seine Slots, auf der Seite `/dm`
/// (`Page`) ist `slot` der Index des Monarchen.
pub enum SlotMap {
    Device(BTreeMap<i64, usize>),
    Page,
}

impl SlotMap {
    fn player(&self, slot: i64) -> Option<i64> {
        match self {
            SlotMap::Device(m) => m.get(&slot).map(|&i| i as i64),
            SlotMap::Page => Some(slot),
        }
    }

    /// Spielerindex zum `slot` der Aktion (Go `devIndex`).
    pub fn index(&self, a: &DevAction) -> Option<i64> {
        self.player(a.slot?)
    }
}

/// Log-Felder einer Dev-Aktion (Name, Wert als Text).
pub type Attrs = Vec<(&'static str, String)>;

pub(super) fn attr(k: &'static str, v: impl Display) -> (&'static str, String) {
    (k, v.to_string())
}

impl Room {
    /// Führt eine Dev-Aktion eines Geräts aus; `slot` ist ein Slot des Geräts. Ohne Dev-Mode (`Env::dev`) ist sie
    /// verboten, noch vor jeder Feldprüfung, damit ein Server ohne Dev-Mode nichts über Felder verrät; die Ablehnung steht
    /// als Warnung im Log, jede gelungene Aktion als Info. Beobachter ändern nichts, auch nicht raumweit (Review TR4.2).
    pub fn dev(&mut self, env: &mut Env, id: &str, conn: u64, a: &DevAction) -> Result<(), Fehler> {
        if !self.dev_allowed(env, &short(id), a) {
            return Err(Fehler::Forbidden);
        }
        self.dev_of_device(env, id, conn, a)
    }

    /// Führt eine Dev-Aktion des eingebetteten nativen Spiels aus (B-888): wie `dev`, aber ohne Dev-Mode, denn der würde
    /// auch Grad und Welten neuer Spiele ändern; das native Spiel läuft nur lokal.
    pub fn dev_local(&mut self, env: &mut Env, id: &str, conn: u64, a: &DevAction) -> Result<(), Fehler> {
        self.dev_of_device(env, id, conn, a)
    }

    fn dev_of_device(&mut self, env: &mut Env, id: &str, conn: u64, a: &DevAction) -> Result<(), Fehler> {
        if !self.own(id, conn) {
            return Err(Fehler::BadRequest);
        }
        let slots = match self.devices.get(id) {
            Some(d) if !d.observer => d.slots.clone(),
            _ => return Err(Fehler::BadRequest),
        };
        self.dev_apply(env, &short(id), a, &SlotMap::Device(slots))
    }

    /// Führt eine Dev-Aktion der Seite `/dm` aus (B-232): ohne Gerät, `slot` ist der Index des Monarchen.
    pub fn dev_page(&mut self, env: &mut Env, a: &DevAction) -> Result<(), Fehler> {
        if !self.dev_allowed(env, "dm", a) {
            return Err(Fehler::Forbidden);
        }
        self.dev_apply(env, "dm", a, &SlotMap::Page)
    }

    /// Der Faktor des Zeitraffers (mindestens 1).
    pub fn timescale(&self) -> i64 {
        self.scale()
    }

    /// Prüft den Dev-Mode und loggt eine Ablehnung.
    fn dev_allowed(&self, env: &mut Env, who: &str, a: &DevAction) -> bool {
        if !env.dev {
            self.log(
                env,
                Level::Warn,
                "🚫 Dev-Aktion abgelehnt",
                &[("device", &who), ("aktion", &a.action)],
            );
        }
        env.dev
    }

    /// Wendet die Aktion an; `map` bildet `slot` auf den Spielerindex der Insel ab.
    fn dev_apply(&mut self, env: &mut Env, who: &str, a: &DevAction, map: &SlotMap) -> Result<(), Fehler> {
        let mut attrs: Attrs = vec![attr("device", who), attr("aktion", &a.action)];
        let ok = match a.action.as_str() {
            "gold" | "material" => match self.dev_stock(env, a, map) {
                Some(more) => {
                    attrs.extend(more);
                    true
                }
                None => false,
            },
            "wave" | "castleFall" => self.dev_wave_or_fall(env, a, map, &mut attrs),
            "phase" => {
                attrs.push(attr("phase", &a.phase));
                dev_set_phase(&mut self.isl, &a.phase)
            }
            "grade" => {
                attrs.push(attr("grad", &a.grade));
                set_grade(&mut self.isl, &a.grade, env.dev).is_ok()
            }
            "timescale" => {
                let ok = DEV_TIMESCALES.contains(&a.factor);
                if ok {
                    self.timescale = a.factor;
                }
                attrs.push(attr("faktor", a.factor));
                ok
            }
            "pause" => match a.paused {
                Some(p) => {
                    self.paused = p;
                    attrs.push(attr("angehalten", p));
                    true
                }
                None => false,
            },
            _ => match self.dev_extra(env, who, a, map) {
                Some(more) => {
                    attrs.extend(more);
                    true
                }
                None => false,
            },
        };
        if !ok {
            return Err(Fehler::BadRequest);
        }
        let fields: Vec<(&str, &dyn Display)> = attrs.iter().map(|(k, v)| (*k, v as &dyn Display)).collect();
        self.log(env, Level::Info, "🐛 Dev-Aktion", &fields);
        Ok(())
    }

    /// `wave` und `castleFall`: die Aktion in der Stufe des Monarchen des Slots.
    fn dev_wave_or_fall(&mut self, env: &Env, a: &DevAction, map: &SlotMap, attrs: &mut Attrs) -> bool {
        let (Some(idx), Some(slot)) = (map.index(a), a.slot) else {
            return false;
        };
        let ok = if a.action == "wave" {
            dev_start_wave(&mut self.isl, &env.data.difficulty, idx)
        } else {
            dev_castle_fall(&mut self.isl, &env.data.difficulty, idx)
        };
        attrs.push(attr("slot", slot));
        ok
    }

    /// Zeitraffer-Faktor, mindestens 1.
    pub fn scale(&self) -> i64 {
        self.timescale.max(1)
    }

    /// Simulationsschritte eines Ticks: 0 in der Dev-Pause und in der Pause des Couch-Raums (`pause.rs`), sonst
    /// `scale()`.
    pub fn steps(&self) -> i64 {
        if self.paused || self.couch_paused() {
            return 0;
        }
        self.scale()
    }

    /// Beendet den Zeitraffer beim ersten Tageszeitwechsel nach `timescaleAuto`.
    pub fn reset_scale_on_phase(&mut self, env: &mut Env) {
        let Some(phase) = self.isl.stages.get(self.start).map(|w| w.cycle.phase.clone()) else {
            return;
        };
        if !self.auto_reset || phase == self.reset_phase {
            return;
        }
        let factor = self.timescale;
        self.log(
            env,
            Level::Info,
            "⏩ Zeitraffer endet mit der Tageszeit",
            &[("faktor", &factor), ("phase", &phase)],
        );
        self.timescale = 1;
        self.auto_reset = false;
    }
}
