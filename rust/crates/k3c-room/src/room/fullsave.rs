//! Spielstand „voll ausgebaut“ (B-315, SV1.3): Dev-Funktion für Abnahmen und Messläufe. Port von
//! `engine/room/fullsave.go`. Eine neue Insel mit den Tiefen der Standard-Welt wie bei einem neuen Raum (B-500); die
//! Hub-Stufe fällt von der höchsten (erste Stufe) je Tiefe um eins, und jede Inselstufe wird mit `dev_build_all` auf ihre
//! Hub-Stufe ausgebaut (B-425): Mauern, Türme und Gebäude halten die Ausbauregeln ein, über alle Stufen steht jede
//! Ausbaustufe mindestens einmal.
//!
//! Gos `fullData` (`sync.OnceValues` über `data/hub.json`) entfällt: die Zahl der Hub-Stufen kommt aus den geladenen
//! Daten des Managers (`Data::hub`); `readJSON` entfällt ebenso.
use k3c_sim::dev::dev_build_all;
use k3c_sim::island::save::SaveTables;
use k3c_sim::island::{CreateOptions, Island, IslandSave, create_island_in, from_island_save_in};
use k3c_sim::world::values_room::default_values_from;

use super::env::Store;
use super::log::{Level, Log};
use super::manager::Manager;
use crate::fehler::Fehler;

/// Namensanfang dieser Spielstände (nicht `TEST_PREFIX`: die löscht der Server beim Start).
pub const FULL_SAVE_PREFIX: &str = "voll-";

/// Go `fullSeed` (`^[a-z0-9-]{1,27}$`): `voll-<seed>` muss ins Namensformat der Spielstände passen (höchstens 32 Zeichen),
/// sonst scheitert erst das Speichern.
fn full_seed(seed: &str) -> bool {
    (1..=27).contains(&seed.len())
        && seed
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

impl<S: Store, L: Log> Manager<S, L> {
    /// Legt den Spielstand „voll ausgebaut“ zum Seed an (überschreibt einen gleichnamigen) und liefert seinen Namen. Nur im
    /// Dev-Mode (sonst `Forbidden`); gleicher Seed ergibt denselben Stand bis auf `savedAt`. Ein offener Raum mit dem
    /// Namen wird vorher geschlossen, wenn kein Gerät verbunden ist, sonst `SaveInUse` (B-424).
    pub fn full_save(&mut self, seed: &str) -> Result<String, Fehler> {
        if !self.dev {
            return Err(Fehler::Forbidden);
        }
        if !full_seed(seed) {
            return Err(Fehler::BadRequest);
        }
        let saved_at = (self.now)().format_report();
        let s = self.full_island(seed, &saved_at)?;
        let raw = serde_json::to_vec(&s).map_err(|e| Fehler::Io(e.to_string()))?;
        let name = format!("{FULL_SAVE_PREFIX}{seed}");
        self.close_empty(&name)?;
        if let Err(e) = self.store.store(&name, &raw) {
            self.log.error(
                "💥 Spielstand voll ausgebaut nicht gespeichert",
                &[("save", &name), ("err", &e)],
            );
            return Err(e);
        }
        let stufen = s.stages.len();
        self.log.info(
            "💾 Spielstand voll ausgebaut angelegt",
            &[("save", &name), ("stufen", &stufen)],
        );
        Ok(name)
    }

    /// Go `fullIsland`: neue Insel der Standard-Welt, jede Inselstufe über `dev_build_all` auf ihre Hub-Stufe ausbauen,
    /// serialisieren und zur Probe laden.
    pub fn full_island(&self, seed: &str, saved_at: &str) -> Result<IslandSave, Fehler> {
        let hub = (self.data.hub.levels.len() as i64).max(1);
        let w = self.world("")?;
        let depths = w.islands.first().map(|i| i.depths.clone()).unwrap_or_default();
        let mut isl =
            create_island_in(&self.data, &w, seed, &depths, 1.0, CreateOptions::default()).map_err(Fehler::Io)?;
        for i in 0..isl.stages.len() {
            // `dev_build_all` baut alle Stufen einer Insel auf eine Hub-Stufe; je Inselstufe eine eigene über eine Sicht
            let level = hub - i as i64;
            let mut view = Island {
                stages: vec![std::mem::take(&mut isl.stages[i])],
                ..Island::default()
            };
            let ok = dev_build_all(&mut view, level.max(1));
            isl.stages[i] = view.stages.pop().unwrap_or_default();
            if !ok {
                let depth = isl.stages[i].biome.depth;
                return Err(Fehler::Io(format!(
                    "voll ausgebaut: Hub-Stufe {level} für Tiefe {depth} ungültig"
                )));
            }
        }
        let s = isl.to_save(saved_at).map_err(|e| Fehler::Io(e.to_string()))?;
        // Der Vollausbau baut immer einen Stand der Standard-Welt, daher die Probe gegen sie (B-694).
        let defaults = default_values_from(&self.data).map_err(Fehler::Io)?;
        let tables = SaveTables {
            data: &self.data,
            defaults: &defaults,
            standard_world: &w,
        };
        from_island_save_in(&tables, &s, 1.0, &w).map_err(|e| Fehler::Io(e.to_string()))?;
        Ok(s)
    }

    /// Go `closeEmpty`: schließt den offenen Raum mit diesem Spielstand, damit er den neuen Stand nicht überschreibt:
    /// ausstehende Speicherungen abwarten, Bericht schreiben, Code frei. Mit verbundenem Gerät `SaveInUse`. `true`:
    /// geschlossen.
    pub fn close_empty(&mut self, name: &str) -> Result<bool, Fehler> {
        let Some(code) = self.rooms.iter().find(|(_, r)| r.name == name).map(|(c, _)| c.clone()) else {
            return Ok(false);
        };
        let (rooms, mut env) = self.split();
        let Some(r) = rooms.get_mut(&code) else {
            return Ok(false);
        };
        let geraete = r.connected();
        if geraete > 0 {
            r.log(
                &mut env,
                Level::Info,
                "🚫 Spielstand voll ausgebaut abgelehnt, Raum hat verbundene Geräte",
                &[("save", &name), ("geraete", &geraete)],
            );
            return Err(Fehler::SaveInUse);
        }
        r.flush();
        r.write_report(&mut env);
        r.closed = true;
        r.log(
            &mut env,
            Level::Info,
            "🧹 Raum geschlossen, Spielstand voll ausgebaut wird neu angelegt",
            &[("save", &name)],
        );
        rooms.remove(&code);
        Ok(true)
    }
}
