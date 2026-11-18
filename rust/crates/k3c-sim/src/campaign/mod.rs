//! Kampagne = alle Stufen eines Spielstands (Port von `engine/sim/campaign.go`, src/world/sim/campaign.ts). Jede Stufe hat
//! ihren eigenen Hub, der beim Stufenwechsel erhalten bleibt. Den Wechsel löst der Aufrufer aus: nach `step` mit
//! `w.travel.progress >= 1` ruft er `travel` (wie src/online/room.ts).
//!
//! Gos Paket-Variablen `biomes` und `globalDayNight` sind die eingebetteten Daten (`world::base`). Gos `panic` bei
//! fehlendem Biom ist hier ein `Err` (Leitplanke § 11); `new_world` meldet es wie `create_world`.

use std::collections::BTreeMap;
use std::sync::Arc;

use serde_json::json;

use k3c_level::nature::neutral_nature;

use crate::island::data::biome_for_depth;
use crate::state::save::HubSave;
use crate::state::save_apply::apply_hub;
use crate::state::{Event, World, WorldValues};
use crate::world::base::base;
use crate::world::pending::cycle_at;
use crate::world::{Options, add_player, create_world};

mod save;

#[cfg(test)]
mod campaign_tests;

/// Go: `Campaign`, eine laufende Kampagne.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Campaign {
    pub id: String,
    pub seed: String,
    pub cycle_speed: f64,
    pub depth: i64,
    pub unlocked_depth: i64,
    /// Stufen, die in dieser Sitzung schon betreten wurden.
    pub(crate) worlds: BTreeMap<i64, World>,
    /// Geladene Hubs, deren Welt noch nicht erzeugt wurde.
    pub(crate) pending_hubs: BTreeMap<i64, HubSave>,
    /// Gold pro Spielerplatz aus dem Spielstand.
    pub(crate) player_gold: Vec<i64>,
    /// Wertesatz der Stufen; `None` = Basis-Wertesatz. Ersetzt Gos Paket-Variablen, die Tests umstellen (`goldenPurse`).
    pub vals: Option<Arc<WorldValues>>,
}

/// Go: `newWorld`: erzeugt eine Stufe. Die Kampagne ist der Altpfad vor den Inseln (Golden-Kampagnen, Tests): Ohne feste
/// Werte gilt die neutrale Natur (`neutral_nature`), damit aufgezeichnete Eingaben dieselben Bäume und Felsen treffen
/// (B-891).
pub fn new_world(depth: i64, seed: &str, mut opts: Options) -> Result<World, String> {
    let base = base()?;
    let b = biome_for_depth(&base.data.biomes, depth).ok_or_else(|| format!("Kein Biom für Tiefe {depth}"))?;
    if opts.nature.is_empty() {
        opts.nature = neutral_nature(b);
    }
    create_world(b, seed, &opts)
}

impl Campaign {
    /// Go: `CreateCampaign`: startet eine neue Kampagne in Tiefe 0. `cycle_speed` 0 bedeutet 1.
    pub fn create(seed: &str, id: &str, cycle_speed: f64) -> Result<Campaign, String> {
        Campaign::create_with(seed, id, cycle_speed, None)
    }

    /// `create` mit eigenem Wertesatz der Stufen (`None` = Basis).
    pub fn create_with(
        seed: &str,
        id: &str,
        cycle_speed: f64,
        vals: Option<Arc<WorldValues>>,
    ) -> Result<Campaign, String> {
        let cycle_speed = if cycle_speed == 0.0 { 1.0 } else { cycle_speed };
        let mut c = Campaign {
            id: id.to_string(),
            seed: seed.to_string(),
            cycle_speed,
            vals,
            ..Campaign::default()
        };
        let world = new_world(0, seed, c.options(0.0))?;
        c.worlds.insert(0, world);
        Ok(c)
    }

    /// Die Optionen einer neuen Stufe zur Zeit `time`: Zyklusgeschwindigkeit und Wertesatz der Kampagne.
    pub(crate) fn options(&self, time: f64) -> Options {
        Options {
            cycle_speed: self.cycle_speed,
            time,
            vals: self.vals.clone(),
            ..Options::default()
        }
    }

    /// Go: `CurrentWorld`: die Stufe, auf der die Spieler gerade sind.
    pub fn current_world(&self) -> Option<&World> {
        self.worlds.get(&self.depth)
    }

    /// `current_world` zum Ändern.
    pub fn current_world_mut(&mut self) -> Option<&mut World> {
        self.worlds.get_mut(&self.depth)
    }

    /// Go: `JoinPlayer`: lässt einen Spieler beitreten; liefert seinen Platz in `players` der aktuellen Stufe. Gold aus dem
    /// Spielstand gilt pro Spielerplatz.
    pub fn join_player(&mut self) -> Option<usize> {
        let w = self.worlds.get_mut(&self.depth)?;
        let pi = add_player(w);
        let index = w.players.get(pi)?.index;
        if let (Some(p), Some(&gold)) = (
            w.players.get_mut(pi),
            usize::try_from(index).ok().and_then(|i| self.player_gold.get(i)),
        ) {
            p.gold = gold;
        }
        Some(pi)
    }

    /// Go: `worldFor`: holt die Stufe der Tiefe (erzeugt sie bei Bedarf mit ihrem gespeicherten Hub) und gleicht Zeit und
    /// Zyklus an: Die Stufe stand still, während niemand dort war; wartende Gegner behalten ihren Abstand.
    pub(crate) fn world_for(&mut self, depth: i64, time: f64, skill_points: i64) -> Result<(), String> {
        if !self.worlds.contains_key(&depth) {
            let mut w = new_world(depth, &self.seed, self.options(time))?;
            if let Some(hub) = self.pending_hubs.remove(&depth) {
                apply_hub(&mut w, &hub);
            }
            self.worlds.insert(depth, w);
        }
        let day_night = &base()?.day_night;
        let w = self
            .worlds
            .get_mut(&depth)
            .ok_or_else(|| format!("Stufe {depth} fehlt"))?;
        let shift = time - w.time;
        for q in &mut w.spawn_queue {
            q.at += shift;
        }
        w.time = time;
        w.cycle = cycle_at(day_night, time * w.cycle_speed);
        w.skill_points = skill_points;
        w.travel = None;
        Ok(())
    }

    /// Go: `Travel`: wechselt die Stufe. Die Spieler nehmen Gold und Reihenfolge mit und stehen danach an der Burg der
    /// Zielstufe. Truppen, Gebäude und Vorräte bleiben in ihrem Hub. Liefert die Tiefe der Zielstufe (Go: `*World`; der
    /// Aufrufer holt sie mit `current_world`).
    pub fn travel(&mut self, to_depth: i64) -> Result<i64, String> {
        let from_depth = self.depth;
        let (time, skill_points, from_players) = {
            let from = self
                .current_world()
                .ok_or_else(|| format!("Stufe {from_depth} fehlt"))?;
            (from.time, from.skill_points, from.players.clone())
        };
        self.world_for(to_depth, time, skill_points)?;
        let target = self
            .worlds
            .get_mut(&to_depth)
            .ok_or_else(|| format!("Stufe {to_depth} fehlt"))?;
        let mut players = Vec::with_capacity(from_players.len());
        for mut q in from_players {
            q.id = target.new_id(); // IDs sind nur innerhalb einer Welt eindeutig
            q.x = if q.index % 2 == 0 {
                target.hub_x - 3.0
            } else {
                target.hub_x + 3.0
            };
            q.vx = 0.0;
            q.crown = None;
            q.pay_cooldown = 0.5;
            q.paying = false;
            players.push(q);
        }
        target.players = players;
        if let Some(from) = self.worlds.get_mut(&from_depth) {
            from.players.clear();
            from.travel = None;
        }
        self.depth = to_depth;
        self.unlocked_depth = self.unlocked_depth.max(to_depth);
        if let Some(target) = self.worlds.get_mut(&to_depth) {
            let name = target.biome.name.clone();
            let ev: Event = match json!({"type": "arrived", "depth": to_depth, "name": name}) {
                serde_json::Value::Object(m) => m,
                _ => Event::new(),
            };
            target.events.push(ev);
        }
        Ok(to_depth)
    }
}
