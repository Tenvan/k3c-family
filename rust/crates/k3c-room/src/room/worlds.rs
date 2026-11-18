//! Welten im Raum (WZ18.4, `docs/rules/welten.md` § 5/6). Port von `world`, `loadWorld`, `createIn`, `loadIn`,
//! `worldErrorf` und `WorldList` aus `worlds.go`: Die Standard-Welt kommt immer aus der Einbettung, weitere
//! Welten nur im Dev-Mode frisch aus dem Ordner der Welten, bei jedem Anlegen und Laden eines Raums. Der Raum behält die
//! geladene Welt, solange er läuft (sie steckt in seiner Insel).
//!
//! Abweichungen von Go: Gos `Manager.Worlds` (`fs.FS` mit `<id>.json`) ist hier der Datenordner (`Source`) mit
//! `worlds/<id>.json`, weil `k3c_sim::island::load_world` so liest; `Manager::worlds` hält ihn (seit RU24.1), die
//! `*_from`-Funktionen nehmen die Quelle zusätzlich als Parameter.
use k3c_data::Source;
use k3c_sim::island::save::SaveTables;
use k3c_sim::island::{
    CreateOptions, Island, IslandSave, STANDARD_WORLD_ID, WorldDef, create_island_in, from_island_save_in, load_world,
    parse_island_save, standard_world,
};
use k3c_sim::world::base::base;
use k3c_sim::world::values_room::{default_values_from, resolve_world};
use serde::Serialize;

use super::env::Store;
use super::log::Log;
use super::manager::Manager;
use super::types::Options;
use crate::fehler::Fehler;

/// Gos `worldErrorf`: die Meldung aus welten.md § 5/6 mit dem Fehlerwert `code` in Klammern.
fn world_error(code: Fehler, text: String) -> Fehler {
    Fehler::meldung(code, text)
}

/// Existiert die Datei der Welt `id` in `src` nicht (Gos `errors.Is(err, fs.ErrNotExist)`)? Nur für gültige IDs gefragt.
fn missing(src: &Source, id: &str) -> bool {
    match src {
        Source::Dir(dir) => !dir.join("worlds").join(format!("{id}.json")).is_file(),
        Source::Embedded => src.read(&format!("worlds/{id}.json")).is_err(),
    }
}

impl<S: Store, L: Log> Manager<S, L> {
    /// Der Ordner der weiteren Welten (Gos `Manager.Worlds`); `None`: keine.
    pub fn worlds_source(&self) -> Option<&Source> {
        self.worlds.as_ref()
    }

    /// Gos `world`: die Welt `id` (leer = standard) mit ihrem aufgelösten Wertesatz (`resolve_world`, welten.md § 1,
    /// B-681), einmal je Anlegen oder Laden des Raums. Fehler nennen die Welt: `Forbidden` ohne Dev-Mode, `WorldUnknown`
    /// für fehlende Datei, ungültige ID, ungültige Welt-Datei oder ungültige Overrides und `extends`-Kette.
    pub fn world(&self, id: &str) -> Result<WorldDef, Fehler> {
        self.world_from(self.worlds_source(), id)
    }

    /// `world` mit dem Ordner der Welten `worlds` als Parameter.
    pub fn world_from(&self, worlds: Option<&Source>, id: &str) -> Result<WorldDef, Fehler> {
        let w = self.load_world_from(worlds, id)?;
        let b = base().map_err(Fehler::Io)?;
        let lookup = |parent: &str| self.load_world_from(worlds, parent).map_err(|e| e.to_string()); // Eltern wie die Welt selbst
        resolve_world(&w, &Source::Embedded, &self.data, &b.rules, &lookup)
            .map_err(|e| world_error(Fehler::WorldUnknown, e))
    }

    /// Gos `loadWorld`: die Welt `id` ohne Auflösung: standard aus der Einbettung, weitere nur im Dev-Mode aus `worlds`.
    pub fn load_world_from(&self, worlds: Option<&Source>, id: &str) -> Result<WorldDef, Fehler> {
        if id.is_empty() || id == STANDARD_WORLD_ID {
            let b = base().map_err(Fehler::Io)?;
            return standard_world(&self.data, &b.rules).map_err(Fehler::Io);
        }
        if !self.dev {
            return Err(world_error(Fehler::Forbidden, format!("Welt {id:?} nur im Dev-Mode")));
        }
        let Some(src) = worlds else {
            return Err(world_error(Fehler::WorldUnknown, format!("Welt {id:?} unbekannt")));
        };
        if k3c_data::world::valid_id(id) && missing(src, id) {
            return Err(world_error(Fehler::WorldUnknown, format!("Welt {id:?} unbekannt")));
        }
        let b = base().map_err(Fehler::Io)?;
        load_world(src, id, &self.data, &b.rules).map_err(|e| world_error(Fehler::WorldUnknown, e))
    }

    /// Gos `createIn`: legt die erste Insel der Welt `opts.world` an, mit den Tiefen ihrer ersten Insel; `depth` muss eine
    /// davon sein (sonst `BadRequest`, unbekannte Startstufe).
    pub fn create_in(&self, seed: &str, depth: i64, opts: &Options) -> Result<Island, Fehler> {
        self.create_in_from(self.worlds_source(), seed, depth, opts)
    }

    /// `create_in` mit dem Ordner der Welten `worlds` als Parameter.
    pub fn create_in_from(
        &self,
        worlds: Option<&Source>,
        seed: &str,
        depth: i64,
        opts: &Options,
    ) -> Result<Island, Fehler> {
        let w = self.world_from(worlds, &opts.world)?;
        let depths = w.islands.first().map(|i| i.depths.clone()).unwrap_or_default();
        if !depths.contains(&depth) {
            return Err(Fehler::BadRequest); // unbekannte Startstufe
        }
        create_island_in(&self.data, &w, seed, &depths, 1.0, CreateOptions::default()).map_err(Fehler::Io)
    }

    /// Gos `ParseIslandSave` und `loadIn`: liest einen Spielstand und baut die Insel in seiner gespeicherten Welt; der
    /// Stand selbst bleibt unverändert.
    pub fn load_in(&self, data: &[u8]) -> Result<Island, Fehler> {
        self.load_in_from(self.worlds_source(), data)
    }

    /// `load_in` mit dem Ordner der Welten `worlds` als Parameter.
    pub fn load_in_from(&self, worlds: Option<&Source>, data: &[u8]) -> Result<Island, Fehler> {
        let raw = std::str::from_utf8(data).map_err(|e| Fehler::Io(format!("spielstand: {e}")))?;
        let std_world = self.load_world_from(worlds, STANDARD_WORLD_ID)?;
        let defaults = default_values_from(&self.data).map_err(Fehler::Io)?;
        let tables = SaveTables {
            data: &self.data,
            defaults: &defaults,
            standard_world: &std_world,
        };
        let s: IslandSave = parse_island_save(raw, &tables).map_err(|e| Fehler::Io(e.to_string()))?;
        let w = self
            .world_from(worlds, &s.world)
            .map_err(|e| e.mit_vorsatz("spielstand: "))?;
        from_island_save_in(&tables, &s, 1.0, &w).map_err(|e| Fehler::Io(e.to_string()))
    }
}

/// Ein Eintrag der Weltliste für die Lobby (Go `WorldInfo`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorldInfo {
    pub id: String,
    pub name: String,
}

/// Die Dateinamen `<id>` der Welt-Dateien in `src`, nach Dateiname sortiert (Go `fs.ReadDir`); eingebettet gibt es nur die
/// Standard-Welt, keine weiteren.
fn world_ids(src: &Source) -> Vec<String> {
    let Source::Dir(dir) = src else {
        return Vec::new();
    };
    let Ok(entries) = std::fs::read_dir(dir.join("worlds")) else {
        return Vec::new();
    };
    let mut files: Vec<(String, bool)> = entries
        .flatten()
        .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path().is_dir()))
        .collect();
    files.sort();
    files
        .into_iter()
        .filter_map(|(name, is_dir)| name.strip_suffix(".json").filter(|_| !is_dir).map(str::to_string))
        .collect()
}

impl<S: Store, L: Log> Manager<S, L> {
    /// Gos `WorldList`: die Welten, die ein neuer Spielstand wählen darf: die Standard-Welt immer zuerst, im Dev-Mode dazu
    /// jede gültige Welt-Datei aus `worlds` (nach Dateiname sortiert). Ungültige Dateien fehlen in der Liste (Fehler beim
    /// Anlegen).
    pub fn world_list(&self) -> Vec<WorldInfo> {
        let mut out = Vec::new();
        if let Ok(std) = self.load_world_from(None, STANDARD_WORLD_ID) {
            out.push(WorldInfo {
                id: std.id,
                name: std.name,
            });
        }
        let Some(src) = self.worlds.as_ref().filter(|_| self.dev) else {
            return out;
        };
        for id in world_ids(src) {
            if id == STANDARD_WORLD_ID {
                continue;
            }
            if let Ok(w) = self.load_world_from(Some(src), &id) {
                out.push(WorldInfo { id: w.id, name: w.name });
            }
        }
        out
    }
}
