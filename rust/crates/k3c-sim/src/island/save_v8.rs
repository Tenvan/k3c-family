//! Spielstand Version 8 (Port von `engine/sim/island_save_v8.go`, WZ18.3, B-488/AC-03, docs/rules/welten.md § 5): Der Stand
//! nennt die Welt (`world`) und die aktuelle Insel (`islandId`) statt des Index `island`. Stände bis Version 7 kommen in die
//! Standard-Welt, auf die Insel am alten Index; die Datei wird dabei nicht umgeschrieben (Version 8 entsteht beim nächsten
//! Speichern). Planung und Erzeugungs-Optionen stehen nicht im Stand, sie kommen beim Laden aus der Welt.
//!
//! Gos Paket-Variable `standardWorld` ist der Parameter `standard_world`.

use super::{IslandSave, WorldDef};
use crate::state::Fehler;

/// Go: `migrateWorld` setzt Welt und Insel eines Stands bis Version 7: Standard-Welt, Insel am alten Index.
pub fn migrate_world(mut s: IslandSave, index: i64, standard_world: &WorldDef) -> Result<IslandSave, Fehler> {
    let id = legacy_island_id(standard_world, index)?;
    s.world = standard_world.id.clone();
    s.island_id = id;
    Ok(s)
}

/// Go: `legacyIslandID` ist die ID der Insel am Index `index` der Welt `w`; ein Index außerhalb der Liste ist ein Fehler.
pub fn legacy_island_id(w: &WorldDef, index: i64) -> Result<String, Fehler> {
    match usize::try_from(index).ok().and_then(|i| w.islands.get(i)) {
        Some(d) => Ok(d.id.clone()),
        None => Err(Fehler::Spielstand(format!(
            "spielstand: Insel {index}, die Daten kennen {}",
            w.islands.len()
        ))),
    }
}

/// Go: `islandIn` liefert den Index der Insel des Stands in der Welt `w`. Fehler (welten.md § 5): Welt des Stands ist nicht
/// `w`, Insel-ID gibt es in `w` nicht.
pub fn island_in(w: &WorldDef, s: &IslandSave) -> Result<usize, Fehler> {
    if s.world != w.id {
        return Err(Fehler::Spielstand(format!("spielstand: Welt {:?} unbekannt", s.world)));
    }
    match w.islands.iter().position(|d| d.id == s.island_id) {
        Some(i) => Ok(i),
        None => Err(Fehler::Spielstand(format!(
            "spielstand: Insel {:?} gibt es in Welt {:?} nicht",
            s.island_id, s.world
        ))),
    }
}
