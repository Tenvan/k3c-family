//! Lesen und Prüfen des Wertesatzes (Port von `valuesFrom` und `checkPools` aus `engine/sim/world_values.go`).
//!
//! Go liest aus einem `fs.FS`; hier ist die Quelle eine Funktion `read(name) -> Result<String, Fehler>` (Vorentscheidung
//! RU14.3). Die Fehler der Einzeldateien werden wie `errors.Join` mit `\n` verbunden, in der Reihenfolge von Go.

use std::collections::BTreeMap;

use k3c_data::enemies::EnemyData;
use k3c_data::{Fehler, buildings, economy, enemies, events, goals, hub, monarch, ship, troops, waves};

use super::base::base;
use super::links::{check_presets, check_world_hub_sites};
use super::ship::with_ship;
use super::values_room::new_skill_table;
use crate::state::WorldValues;
use crate::state::data::check_waves;

/// Die Quelle einer Datei unter data/ (Name wie `buildings.json`, `biomes/cave.json`).
pub type ReadFn<'a> = &'a dyn Fn(&str) -> Result<String, Fehler>;

/// Go: `errors.Join`: die Meldungen der Fehler mit `\n` verbunden; `None`, wenn es keinen gibt.
pub fn join_errors(errs: Vec<String>) -> Option<String> {
    if errs.is_empty() { None } else { Some(errs.join("\n")) }
}

/// Liest eine Datei und dekodiert sie; der Fehler ist der Text wie in Go, das Ergebnis bei Fehler der Standardwert.
fn load_one<T: Default>(
    errs: &mut Vec<String>,
    name: &str,
    load: impl FnOnce(&str) -> Result<T, Fehler>,
    read: ReadFn,
) -> T {
    match read(name).and_then(|raw| load(&raw)) {
        Ok(v) => v,
        Err(e) => {
            errs.push(e.to_string());
            T::default()
        }
    }
}

/// Go: `valuesFrom`: liest den Wertesatz aus `read` wie `UseData` und prüft die Basis-Regeln: Traits (`loadEnemies`),
/// Presets, Wellen und die Gegner-Pools der Biome. Der Platz `ship` kommt aus ship.json (`with_ship`).
///
/// Abweichung von Go: `bosses` (Gos Paket-Variablen `bosses`/`endBosses`) ist für jede Welt die der eingebetteten Daten.
pub fn values_from(read: ReadFn) -> Result<WorldValues, String> {
    let mut errs = Vec::new();
    let mut v = WorldValues {
        native_rules: true, // HB1.4: native Regel, in jeder Welt
        ..WorldValues::default()
    };
    let mut buildings = load_one(&mut errs, buildings::FILE, buildings::load, read);
    let ship = load_one(&mut errs, ship::FILE, ship::load, read);
    v.troops = load_one(&mut errs, troops::FILE, troops::load, read);
    v.economy = load_one(&mut errs, economy::FILE, economy::load, read);
    v.hub = load_one(&mut errs, hub::FILE, hub::load, read);
    v.monarch = load_one(&mut errs, monarch::FILE, monarch::load, read);
    v.enemies = load_one(&mut errs, enemies::FILE, enemies::load, read);
    v.waves = load_one(&mut errs, waves::FILE, waves::load, read);
    v.goals = load_one(&mut errs, goals::FILE, goals::load, read);
    v.events = load_one(&mut errs, events::FILE, events::load, read).events;
    if errs.is_empty() {
        let checks = [
            check_presets(&v.monarch),
            check_waves(&v.waves),
            check_pools(&v.enemies),
            events::check_night_events(&v.events, &v.enemies).map_err(|e| e.to_string()),
        ];
        errs.extend(checks.into_iter().filter_map(Result::err));
    }
    if errs.is_empty() && ship.stages.is_empty() {
        errs.push("data/ship.json: keine Abschnitte".to_string());
    }
    if let Some(e) = join_errors(errs) {
        return Err(e);
    }
    with_ship(&mut buildings, &ship)?;
    v.buildings = buildings;
    v.skills = new_skill_table(&v.monarch.skills);
    v.bosses = base()?.data.bosses.clone();
    check_world_hub_sites(&v.hub, &v.economy, &v.buildings, &base()?.data.biomes)?;
    Ok(v)
}

/// Go: `checkPools`: prüft die Gegner-Pools aller Biome gegen `enemies` (`check_pool`); die Biome sind für alle Welten
/// gleich (Go: Paket-Variable `biomes`, hier die eingebetteten Daten).
pub fn check_pools(enemies: &BTreeMap<String, EnemyData>) -> Result<(), String> {
    for b in &base()?.data.biomes {
        k3c_data::biome::check_pool(b, enemies).map_err(|e| e.to_string())?;
    }
    Ok(())
}
