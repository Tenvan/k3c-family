//! Einsiedler (RU62.10, B-972, docs/rules/buerger.md § 1, materialien-gebaeude.md § 3; Entscheidung 013: neue Mechanik nur in
//! Rust, ohne Go-Orakel): Eine Welt aus `hermits.json › worlds` hat einen besonderen Bürger. Auf jeder Insel mit mindestens
//! zwei Stufen steht in Tiefe 1 am fernen Rand seine Hütte; wer dort `price` Münzen einwirft, löst sofort den Bau seines
//! Bauwerks auf dem Hub-Platz `hubSite.id` (Tiefe 0) aus. Solange es steht, wirkt sein Bonus auf der ganzen Insel (Dojo:
//! Schaden der Krieger und Schützen, Schrein: Mauerheilung bei Tagesanbruch). Daten: data/hermits.json (`k3c-data/hermits.rs`).
//!
//! Was gespeichert wird: der Platz (Zustand `hermitBuilding`, Fortschritt, HP wie jeder Platz) und `Island::hermit.paid`
//! (`IslandSave::hermit_paid`, `ISLAND_SAVE_VERSION` 12); die Hütte wird aus dem Insel-Seed berechnet (`k3c-rng`), ohne Eingriff
//! in den Level-Generator. `standard` und Welten ohne Eintrag haben weder Hütte noch Platz (Verhalten wie heute).
//!
//! - `mod.rs`: Einschalten der Insel, Platz der Hütte, Zustand für die Anzeige.
//! - `pay.rs`: der Einwurf an der Hütte (Spielerschritt in `economy/players.rs`).
//! - `step.rs`: Takt der Insel: Bau starten, Bauzeit, Wiederaufbau.
//! - `bonus.rs`: Schadensfaktor, Mauerheilung, Bauzeit für die Reparatur.
//! - `validate.rs`: der Platz besteht die Hub-Prüfung.
use std::sync::Arc;

use k3c_data::Data;
use k3c_data::hermits::{HermitsFile, HutData};
use k3c_rng::Rng;

use crate::island::Island;
use crate::state::hermit_view::{HermitHut, HermitPhase, HermitState};
use crate::state::{Site, World};
use crate::world::empty_site;

pub mod bonus;
pub mod pay;
pub mod step;
pub mod validate;

#[cfg(test)]
mod bonus_tests;
#[cfg(test)]
mod hut_tests;
#[cfg(test)]
mod pay_tests;
#[cfg(test)]
mod save_tests;
#[cfg(test)]
mod testutil;

/// Wert von `Site::state` während des Baus durch den Einsiedler (kein Bauer baut, nur die Zeit zählt).
pub const BUILDING_STATE: &str = "hermitBuilding";

/// Der Einsiedler einer Insel (alle Stufen teilen ihn).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Hermit {
    /// Der Katalog; `None` = Insel ohne Einsiedler (Welt ohne Eintrag, `standard`, weniger als zwei Stufen).
    pub cfg: Option<Arc<HermitsFile>>,
    /// Katalog-ID des Einsiedlers der Welt.
    pub id: String,
    /// Der Einsiedler ist bezahlt. Bleibt wahr, auch wenn das Bauwerk zerstört wird (keine zweite Zahlung).
    pub paid: bool,
}

impl Hermit {
    /// `s` ist der Hub-Platz des Bauwerks.
    pub fn is_hall(&self, s: &Site) -> bool {
        self.cfg.as_ref().is_some_and(|c| c.hub_site.id == s.kind)
    }
}

/// Der Ort der Hütte (Units) in einer Stufe der Breite `width` mit der Hub-Mitte `hub_x`: am Rand, der am weitesten vom Hub
/// entfernt ist, um einen Abstand zwischen `edgeMarginMinUnits` und `edgeMarginMaxUnits` nach innen, aus dem Insel-Seed.
pub fn hut_x(seed: &str, width: f64, hub_x: f64, hut: &HutData) -> f64 {
    let mut rng = Rng::new(&format!("{seed}|hermitHut"));
    let margin = hut.edge_margin_min_units + rng.next() * (hut.edge_margin_max_units - hut.edge_margin_min_units);
    let x = if hub_x <= width - hub_x { width - margin } else { margin };
    x.max(0.0).min(width)
}

impl Island {
    /// Index der Stufe der Tiefe `depth`.
    pub(crate) fn stage_at_depth(&self, depth: i64) -> Option<usize> {
        self.stages.iter().position(|w| w.biome.depth == depth)
    }

    /// Schaltet den Einsiedler der Insel ein (nur Welten aus hermits.json mit Stufen der Tiefe 0 und 1; Aufruf beim Bauen der
    /// Insel): Zustand in alle Stufen, die Hütte in die Stufe der Tiefe 1, der Platz des Bauwerks in die der Tiefe 0. Der
    /// Platz muss die Hub-Prüfung bestehen (sonst Fehler). Ohne Eintrag oder mit einer Stufe geschieht nichts.
    pub fn enable_hermit(&mut self, data: &Data) -> Result<(), String> {
        let Some(id) = data.hermits.worlds.get(self.world_id()) else {
            return Ok(());
        };
        let Some(entry) = data.hermits.entry(id) else {
            return Ok(());
        };
        let (Some(top), Some(deep)) = (self.stage_at_depth(0), self.stage_at_depth(1)) else {
            return Ok(());
        };
        validate::check_hall_site(data).map_err(|e| format!("insel: Einsiedler: {e}"))?;
        let cfg = Arc::new(data.hermits.clone());
        let seed = self.seed.clone();
        for w in &mut self.stages {
            w.hermit_cfg = Some(Arc::clone(&cfg));
            w.hermit = Some(HermitState {
                hermit: entry.id.clone(),
                building: entry.building.clone(),
                price: entry.price,
                damage_factor: 1.0,
                ..HermitState::default()
            });
        }
        let w = &mut self.stages[deep];
        let x = hut_x(&seed, w.width_units, w.hub_x, &cfg.hut);
        if let Some(h) = w.hermit.as_mut() {
            h.hut = Some(HermitHut {
                x,
                range: cfg.hut.range_units,
            });
        }
        let w = &mut self.stages[top];
        let x = w.hub_x + cfg.hub_site.offset_units;
        let mut site = empty_site(w, &cfg.hub_site.id, x);
        site.max_hp = entry.hp; // der Platz steht in keiner buildings.json: die HP kommen aus hermits.json
        w.sites.push(site);
        self.hermit = Hermit {
            cfg: Some(cfg),
            id: entry.id.clone(),
            paid: false,
        };
        Ok(())
    }

    /// Stufe und Index des Platzes des Bauwerks; `None` ohne Einsiedler.
    pub(crate) fn hall_site(&self) -> Option<(usize, usize)> {
        let cfg = self.hermit.cfg.as_ref()?;
        self.stages.iter().enumerate().find_map(|(wi, w)| {
            w.sites
                .iter()
                .position(|s| s.kind == cfg.hub_site.id)
                .map(|si| (wi, si))
        })
    }

    /// Je Takt vor den Stufen: Zustand für die Anzeige und Faktoren in jeder Stufe (Bonus, Zustand der Hütte). Ohne
    /// Einsiedler geschieht nichts.
    pub fn prepare_hermit(&mut self) {
        publish(self);
    }

    /// Übernimmt `paid` aus dem Spielstand (Laden): ein gespeicherter Platz im Bau oder gebaut zählt auch ohne das Feld als
    /// bezahlt. Gibt es das Feld in einer Welt ohne Einsiedler, ist der Stand ungültig.
    pub fn restore_hermit(&mut self, data: &Data, paid: bool) -> Result<(), String> {
        if self.hermit.cfg.is_none() {
            let known = data.hermits.worlds.contains_key(self.world_id());
            return if paid && !known {
                Err("spielstand: Einsiedler in einer Welt ohne Einsiedler".to_string())
            } else {
                Ok(())
            };
        }
        let started = self
            .hall_site()
            .is_some_and(|(wi, si)| self.stages[wi].sites[si].state != "unpaid");
        self.hermit.paid = paid || started;
        publish(self);
        Ok(())
    }
}

/// Schreibt Phase, Fortschritt, Bonus und Faktoren in den Zustand jeder Stufe (ohne Zuteilung: nur Zahlen und Flaggen).
pub fn publish(isl: &mut Island) {
    let Some(cfg) = isl.hermit.cfg.clone() else {
        return;
    };
    let Some(e) = cfg.entry(&isl.hermit.id) else {
        return;
    };
    let paid = isl.hermit.paid;
    let (phase, progress) = match isl.hall_site() {
        Some((wi, si)) => {
            let s = &isl.stages[wi].sites[si];
            match s.state.as_str() {
                BUILDING_STATE => (HermitPhase::Walking, s.build_progress.min(1.0)),
                "built" => (HermitPhase::Built, 0.0),
                _ if paid => (HermitPhase::Fallen, 0.0),
                _ => (HermitPhase::Hut, 0.0),
            }
        }
        None => (HermitPhase::Hut, 0.0),
    };
    let bonus = phase == HermitPhase::Built;
    let top_level = isl.stage_at_depth(0).map_or(1, |i| isl.stages[i].hub_level);
    // nur ein leerer Platz kann bezahlt werden (sonst ginge das Gold ohne Bau verloren)
    let hall_empty = isl
        .hall_site()
        .is_some_and(|(wi, si)| isl.stages[wi].sites[si].state == "unpaid");
    let open = !paid && hall_empty && top_level >= cfg.hub_site.hub_level;
    for w in &mut isl.stages {
        let Some(h) = w.hermit.as_mut() else {
            continue;
        };
        h.phase = phase;
        h.progress = progress;
        h.bonus = bonus;
        h.open = open && !h.pending;
        h.damage_factor = if bonus && e.damage_factor > 1.0 {
            e.damage_factor
        } else {
            1.0
        };
        h.wall_heal = if bonus { e.wall_heal_fraction } else { 0.0 };
    }
}

impl World {
    /// Die Hütte des Einsiedlers in dieser Stufe (nur Tiefe 1 einer Welt mit Einsiedler).
    pub fn hermit_hut(&self) -> Option<&HermitHut> {
        self.hermit.as_ref()?.hut.as_ref()
    }
}
