//! Etappen und Zielwahl am Hafen (Port von `engine/sim/island_targets.go`, B-961, docs/rules/welten.md › Inselwechsel):
//! Inseln einer Etappe sind in beliebiger Reihenfolge anfahrbar, die nächste Etappe öffnet erst, wenn alle Inseln der
//! laufenden abgeschlossen sind. Abgeschlossen ist eine Insel mit besiegtem Endboss beim Ablegen (`Island::completed`,
//! Insel-IDs). Der Raum reicht die Wahl über `choose_island` durch; mehrere Wahlen im selben Takt entscheidet der höchste
//! Spieler-Index (`apply_choices`).

use super::{Island, WorldDef};
use crate::state::links::crowned;
use crate::state::shared::SwitchTarget;

impl WorldDef {
    /// Go: `etappeOf`, die Etappe der Insel am Listenplatz `i`; ohne Angabe (0) gilt Listenplatz + 1.
    pub fn etappe_of(&self, i: i64) -> i64 {
        match self.islands.get(i as usize) {
            Some(d) if d.etappe > 0 => d.etappe,
            _ => i + 1,
        }
    }

    /// Go: `targetsFrom`, die Listenplätze der Ziele für die Insel am Listenplatz `cur`: offene Inseln der Etappe der
    /// aktuellen Insel, sonst die Inseln der nächsten Etappe; leer = letzte Insel der Welt. `done` sagt je Listenplatz, ob
    /// die Insel abgeschlossen gilt (reine Funktion, auch für die Wendepunkte der Geschichte, story_points.go).
    pub fn targets_from(&self, cur: i64, done: impl Fn(i64) -> bool) -> Vec<i64> {
        let n = self.islands.len() as i64;
        if cur < 0 || cur >= n {
            return Vec::new();
        }
        let etappe = self.etappe_of(cur);
        let (mut same, mut next) = (Vec::new(), Vec::new());
        for i in 0..n {
            if i == cur || done(i) {
                continue;
            }
            let e = self.etappe_of(i);
            if e == etappe {
                same.push(i);
            } else if e == etappe + 1 {
                next.push(i);
            }
        }
        if same.is_empty() { next } else { same }
    }
}

impl Island {
    /// Go: `done`: Die Insel am Listenplatz `i` gilt als abgeschlossen (früher abgeschlossen oder die laufende nach dem
    /// Endboss).
    pub fn done(&self, i: i64) -> bool {
        let Some(d) = self.world.islands.get(i as usize) else {
            return false;
        };
        self.completed.contains(&d.id) || (i == self.number && self.endboss_defeated)
    }

    /// Go: `targetIndexes`, die Listenplätze der Ziele (leer = letzte Insel der Welt).
    pub fn target_indexes(&self) -> Vec<i64> {
        self.world.targets_from(self.number, |i| self.done(i))
    }

    /// Go: `Targets`, die Ziele am Hafen in Listenreihenfolge.
    pub fn targets(&self) -> Vec<SwitchTarget> {
        let mut out = Vec::new();
        for i in self.target_indexes() {
            if let Some(d) = self.world.islands.get(i as usize) {
                out.push(SwitchTarget {
                    id: d.id.clone(),
                    name: d.name.clone(),
                    etappe: self.world.etappe_of(i),
                });
            }
        }
        out
    }

    /// Go: `chosenIndex`, der Listenplatz des Ziels: die gewählte Insel, sonst das erste Ziel; `None` ohne Ziele.
    pub fn chosen_index(&self) -> Option<i64> {
        let idx = self.target_indexes();
        let first = *idx.first()?;
        for &i in &idx {
            let id = self.world.islands.get(i as usize).map(|d| d.id.as_str());
            if !self.chosen.is_empty() && id == Some(self.chosen.as_str()) {
                return Some(i);
            }
        }
        Some(first)
    }

    /// Go: `ChosenTarget`, die ID des Ziels, das der Wechsel anfährt (Wahl oder erstes Ziel); leer ohne Ziele.
    pub fn chosen_target(&self) -> String {
        match self.chosen_index().and_then(|i| self.world.islands.get(i as usize)) {
            Some(d) => d.id.clone(),
            None => String::new(),
        }
    }

    /// Go: `applyChoices` übernimmt die Wahlen des Takts: bei mehreren gewinnt der höchste Spieler-Index
    /// (deterministisch, die Map ist nach Index geordnet).
    pub fn apply_choices(&mut self) {
        if let Some((_, id)) = self.pending_choice.last_key_value() {
            self.chosen = id.clone();
            self.pending_choice.clear();
        }
    }
}

/// Go: `ChooseIsland`, die Wahl-Aktion eines gekrönten Spielers (Platz `pi` in Stufe `stage`) am geöffneten Wechselpunkt;
/// nur aus den Zielen. Die Wahl wirkt im nächsten Inselschritt (`step_island_switch`); eine Ablehnung lässt das Ziel
/// unverändert. Go liest die Insel über `w.island`; hier bekommt die Funktion die Insel selbst.
pub fn choose_island(isl: &mut Island, stage: usize, pi: usize, island_id: &str) -> Result<(), String> {
    if !isl.gate_open || isl.switch_ready {
        return Err("insel: Wechselpunkt nicht offen".to_string());
    }
    let Some(p) = isl.stages.get(stage).and_then(|w| w.players.get(pi)) else {
        return Err("insel: ohne Krone keine Wahl".to_string());
    };
    if !crowned(p) || p.free {
        return Err("insel: ohne Krone keine Wahl".to_string());
    }
    let index = p.index;
    if !isl.targets().iter().any(|t| t.id == island_id) {
        return Err(format!("insel: {island_id:?} ist kein Ziel"));
    }
    isl.pending_choice.insert(index, island_id.to_string());
    Ok(())
}
