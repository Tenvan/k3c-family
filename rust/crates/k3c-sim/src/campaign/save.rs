//! Spielstand der Kampagne (Go: `Campaign.ToSave` und `FromSave`, save.go; der Rest steht in `state/save.rs`).

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::state::WorldValues;
use crate::state::save::{HubSave, PlayerSave, SAVE_VERSION, SaveGame, hub_save};
use crate::world::Options;

use super::{Campaign, new_world};

impl Campaign {
    /// Go: `ToSave`: schreibt den Spielstand. `saved_at` ist ein Zeitstempel (ISO 8601).
    pub fn to_save(&self, saved_at: &str) -> Result<SaveGame, String> {
        let w = self
            .current_world()
            .ok_or_else(|| format!("Stufe {} fehlt", self.depth))?;
        // Die Tiefen sind eindeutig; die Tiefe ordnet (Go: stabil sortiert).
        let mut hubs: BTreeMap<i64, HubSave> = self.pending_hubs.clone();
        for (&depth, world) in &self.worlds {
            // Vergleichswelt frisch aus dem Seed: was dort ist, hier aber fehlt, wurde entfernt.
            let opts = Options {
                vals: self.vals.clone(),
                ..Options::default()
            };
            let fresh = new_world(depth, &self.seed, opts)?;
            let h = hub_save(world, &fresh);
            hubs.insert(h.depth, h);
        }
        let mut gold = self.player_gold.clone();
        for p in &w.players {
            let index = usize::try_from(p.index).map_err(|_| format!("Spielerindex {}", p.index))?;
            if gold.len() <= index {
                gold.resize(index + 1, 0);
            }
            gold[index] = p.gold;
        }
        Ok(SaveGame {
            version: SAVE_VERSION,
            campaign_id: self.id.clone(),
            saved_at: saved_at.to_string(),
            seed: self.seed.clone(),
            depth: self.depth,
            unlocked_depth: self.unlocked_depth,
            time: w.time,
            skill_points: w.skill_points,
            players: gold.into_iter().map(|gold| PlayerSave { gold }).collect(),
            hubs: hubs.into_values().collect(),
        })
    }

    /// Go: `FromSave`: baut die Kampagne aus einem Spielstand (aus `parse_save`). `cycle_speed` 0 bedeutet 1.
    pub fn from_save(s: &SaveGame, cycle_speed: f64) -> Result<Campaign, String> {
        Campaign::from_save_with(s, cycle_speed, None)
    }

    /// `from_save` mit eigenem Wertesatz der Stufen (`None` = Basis).
    pub fn from_save_with(s: &SaveGame, cycle_speed: f64, vals: Option<Arc<WorldValues>>) -> Result<Campaign, String> {
        let mut c = Campaign {
            id: s.campaign_id.clone(),
            seed: s.seed.clone(),
            cycle_speed: if cycle_speed == 0.0 { 1.0 } else { cycle_speed },
            depth: s.depth,
            unlocked_depth: s.unlocked_depth,
            vals,
            ..Campaign::default()
        };
        for h in &s.hubs {
            c.pending_hubs.insert(h.depth, h.clone());
        }
        c.player_gold = s.players.iter().map(|p| p.gold).collect();
        c.world_for(s.depth, s.time, s.skill_points)?;
        Ok(c)
    }
}
