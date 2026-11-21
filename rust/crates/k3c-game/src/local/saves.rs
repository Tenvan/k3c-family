//! Port des Rests von `native/k3c-game/local/saves.go` (`watchedStore` steht in `watched.rs`): Liste, Löschen, freier Name
//! und Welten. Spielstände liegen wie in Go als `<saves>/<name>.json` im Ordner der Session; Namen (`^[a-z0-9-]{1,32}$`)
//! prüft der Manager des Raums von Hand (`valid_save_name`), das Format bestimmt `k3c_room::store`.
//!
//! Abweichungen von Go: (1) `info` prüft einen Eintrag mit `parse_island_save` und denselben Tabellen wie `Manager::load_in`
//! (Spieldaten, Standardwerte, Standard-Welt), baut die Insel aber nicht auf; die Tabellen entstehen einmal je `list`.
//! (2) Gespeichert wird im Takt (`InlineWriter` des Raums), es gibt keinen Thread; `WatchedStore` teilt seine Ergebnisse
//! über `Arc<Mutex<…>>` nur, weil der Manager den Store besitzt. (3) Der Ordner der weiteren Welten ist ein `Source`
//! (Datenordner mit `worlds/<id>.json`, `worlds_source`).

use std::time::{Duration, UNIX_EPOCH};

use k3c_room::room::Log as _;
use k3c_room::store::{SaveInfo as StoreInfo, Stamp};
use k3c_sim::island::save::SaveTables;
use k3c_sim::island::{parse_island_save, standard_world};
use k3c_sim::world::base::base;
use k3c_sim::world::values_room::default_values_from;

use super::error::{UserError, describe, user_err};
use super::session::Session;
use super::worlds::game_data;
use crate::view::session::{SaveInfo, WorldInfo};

/// Liest `YYYY-MM-DDTHH:MM:SS` (RFC 3339, UTC) als Zeitpunkt; alles andere ist `None`.
fn parse_time(s: &str) -> Option<std::time::SystemTime> {
    let n = |r: std::ops::Range<usize>| s.get(r)?.parse::<i64>().ok();
    let st = Stamp::from_civil(n(0..4)?, n(5..7)?, n(8..10)?, n(11..13)?, n(14..16)?, n(17..19)?, 0);
    Some(UNIX_EPOCH + Duration::from_secs(u64::try_from(st.secs).ok()?))
}

impl Session {
    /// Go: `List`: neueste zuerst (gleiche Zeit: nach Name); kaputte und unbekannte Versionen stehen mit `loadable` false
    /// und Grund in der Liste.
    pub(super) fn saves_list(&mut self) -> Result<Vec<SaveInfo>, UserError> {
        let raw = self.saves.list();
        let checker = SaveChecker::new();
        let mut out: Vec<SaveInfo> = raw.into_iter().map(|e| self.info(&checker, e)).collect();
        out.sort_by_key(|a| std::cmp::Reverse(a.saved_at));
        Ok(out)
    }

    /// Go: `info`: prüft einen Eintrag wie beim Laden (`parse_island_save`), ohne die Insel aufzubauen.
    fn info(&self, checker: &Result<SaveChecker, String>, e: StoreInfo) -> SaveInfo {
        let mut v = SaveInfo {
            saved_at: parse_time(&e.saved_at),
            day: e.day,
            depths: e.depths.clone(),
            world: e.world.clone(),
            name: e.name.clone(),
            ..SaveInfo::default()
        };
        if !e.error.is_empty() {
            v.problem = "Spielstand unlesbar".to_string();
            return v;
        }
        let res = self
            .saves
            .load(&e.name)
            .map_err(|err| err.to_string())
            .and_then(|data| checker.as_ref().map_err(Clone::clone).and_then(|c| c.check(&data)));
        match res {
            Ok(()) => v.loadable = true,
            Err(err) => v.problem = format!("Spielstand nicht ladbar: {err}"),
        }
        v
    }

    /// Go: `Delete`: nimmt den Spielstand in die Sicherungen (`Saves::discard`, B-506); nicht den des laufenden Spiels. Läuft
    /// ein Manager, prüft er Name und offene Räume (`Manager::delete_save`).
    pub(super) fn saves_delete(&mut self, name: &str) -> Result<(), UserError> {
        if self.m.is_some() && name == self.name {
            return Err(user_err(
                "Der Spielstand des laufenden Spiels lässt sich nicht löschen",
                None,
            ));
        }
        let saves = &mut self.saves;
        let res = match self.m.as_mut() {
            Some(m) => m.delete_save(name, || saves.discard(name)),
            None => saves.discard(name),
        };
        res.map_err(|e| describe(&e, name))?;
        self.log.info("🧹 Spielstand gelöscht", &[("save", &name)]);
        Ok(())
    }

    /// Go: `freeName`: der erste freie Name spiel-1, spiel-2 … im Ordner.
    pub(super) fn free_name(&self) -> String {
        let used: Vec<String> = self.saves.list().into_iter().map(|e| e.name).collect();
        (1..)
            .map(|i| format!("spiel-{i}"))
            .find(|n| !used.contains(n))
            .unwrap_or_default()
    }

    /// Go: `WorldList`: Standard-Welt, weitere nur im Dev-Mode des Managers (`Manager::world_list`).
    pub(super) fn saves_world_list(&self) -> Vec<WorldInfo> {
        let list = match &self.m {
            Some(m) => m.world_list(),
            None => match self.manager() {
                Ok(m) => m.world_list(),
                Err(_) => Vec::new(),
            },
        };
        list.into_iter()
            .map(|w| WorldInfo {
                id: w.id,
                name: w.name,
                error: w.error,
            })
            .collect()
    }

    /// Go: `World`: die Welt-ID des laufenden Spiels (leer ohne Spiel).
    pub(super) fn saves_world(&self) -> String {
        if self.m.is_none() {
            return String::new();
        }
        self.world.clone()
    }
}

/// Die Tabellen, mit denen `parse_island_save` einen Spielstand prüft (wie `Manager::load_in_from`).
struct SaveChecker {
    data: std::sync::Arc<k3c_data::Data>,
    defaults: k3c_sim::state::WorldValues,
    std_world: k3c_sim::island::WorldDef,
}

impl SaveChecker {
    fn new() -> Result<SaveChecker, String> {
        let data = game_data().map_err(|e| e.to_string())?;
        let b = base()?;
        let std_world = standard_world(&data, &b.rules)?;
        let defaults = default_values_from(&data)?;
        Ok(SaveChecker {
            data,
            defaults,
            std_world,
        })
    }

    /// Liest `data` als Spielstand; der Fehler ist sein Text.
    fn check(&self, data: &[u8]) -> Result<(), String> {
        let raw = std::str::from_utf8(data).map_err(|e| format!("spielstand: {e}"))?;
        let tables = SaveTables {
            data: &self.data,
            defaults: &self.defaults,
            standard_world: &self.std_world,
        };
        parse_island_save(raw, &tables).map(|_| ()).map_err(|e| e.to_string())
    }
}
