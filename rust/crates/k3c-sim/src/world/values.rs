//! Welt-Werte (Port von `engine/sim/world_values.go`, docs/rules/welten.md § 1, B-680): Eine Welt erbt die Basis-Werte aus
//! data/ und übersteuert mit `overrides` nur einzelne Felder; mit `extends` erbt sie von einer anderen Welt (Kette
//! Basis → A → B, höchstens `MAX_WORLD_CHAIN` Welten). `resolve_world_values` löst eine Welt zu ihrem Wertesatz auf,
//! ohne globalen Zustand zu ändern. Merge und Prüfung je Feld: `merge_file` (RU14.4); Lesen und Prüfen: `values_from`.
//!
//! Abweichungen von Go (Vorentscheidungen RU14.3): Gos `fs.FS`, `fstest.MapFS` und `overlayFS` sind eine Funktion
//! `read(name)` (`ReadFn`); die übersteuerten Dateien liegen in einer `BTreeMap<String, String>` (Schlüssel `buildings.json`),
//! `overlay` ist ein Abschluss, der erst dort und dann in der Basis sucht. Gos `overrideFiles` (Map auf Lese-Typen) ist die
//! sortierte Namensliste `override_files`; den Lese-Typ je Datei liefert RU14.4 (`values_shape`). Die Prüfung der Felder
//! `extends` und `overrides` beim Lesen der Welt-Datei (Go: `checkWorldLinks`) steht in `k3c_data::world`.

use std::collections::BTreeMap;

use k3c_data::Fehler;
use serde_json::Value;

use super::values_from::{ReadFn, values_from};
use super::values_locked::check_locked;
use super::values_merge::merge_file;
use crate::island::WorldDef;
use crate::island::worlds::world_path;
use crate::state::WorldValues;

/// Go: `maxWorldChain`: die größte Zahl Welten in einer Kette über der Basis (Basis → A → B → C → D).
pub const MAX_WORLD_CHAIN: usize = 4;

/// Go: `overrideFiles` (Schlüssel): die Basis-Dateien, die eine Welt übersteuern darf (Name ohne `.json`), sortiert.
pub fn override_files() -> &'static [&'static str] {
    &[
        "buildings",
        "economy",
        "enemies",
        "events",
        "goals",
        "hub",
        "monarch",
        "ship",
        "troops",
        "waves",
        "welt-events", // Welt-Events (B-991, B-992) stehen in eigener Datei, s. k3c-data `WELT_FILE`
    ]
}

/// Eine Welt nach ID (Go: `lookup func(id string) (WorldDef, error)`, z. B. `load_world`).
pub type LookupFn<'a> = &'a dyn Fn(&str) -> Result<WorldDef, String>;

/// Die übersteuerten Dateien: Dateiname (`buildings.json`) → Inhalt (Go: `fstest.MapFS`).
pub type Files = BTreeMap<String, String>;

/// Go: `ResolveWorldValues`: löst die Welt `id` zu ihrem Wertesatz auf: Basis-Dateien aus `base` (Aufbau wie data/),
/// darüber die Overrides der Kette von der ältesten Eltern-Welt bis zur Welt selbst. Jede Welt mit Overrides wird nach
/// ihrem Schritt mit den Basis-Regeln geprüft; ein Fehler nennt die Welt, deren Override ihn auslöst. Ohne Overrides ist
/// das Ergebnis gleich dem Laden der Basis.
pub fn resolve_world_values(base: ReadFn, lookup: LookupFn, id: &str) -> Result<WorldValues, String> {
    let files = resolve_files(base, lookup, id)?;
    values_from(&overlay(base, &files))
}

/// Go: `ResolveWorldFiles`: die übersteuerbaren Basis-Dateien (Schlüssel wie in `overrides`) als JSON, wie sie für die Welt
/// `id` nach den Overrides ihrer Kette gelten, geprüft wie `resolve_world_values`; `id` leer = die Basis selbst, ebenso mit
/// den Basis-Regeln geprüft. Für Werkzeuge, die Basiswerte neben Overrides zeigen (Welten-Editor, B-682).
pub fn resolve_world_files(base: ReadFn, lookup: LookupFn, id: &str) -> Result<BTreeMap<String, String>, String> {
    let files = if id.is_empty() {
        values_from(base)?;
        Files::new()
    } else {
        resolve_files(base, lookup, id)?
    };
    let read = overlay(base, &files);
    let mut out = BTreeMap::new();
    for name in override_files() {
        let raw = read(&format!("{name}.json")).map_err(|e| read_err(&format!("{name}.json"), &e))?;
        out.insert((*name).to_string(), raw);
    }
    Ok(out)
}

/// Go: `resolveFiles`: legt die Overrides der Kette von `id` über `base` und prüft nach jeder Welt mit Overrides den Stand.
fn resolve_files(base: ReadFn, lookup: LookupFn, id: &str) -> Result<Files, String> {
    let mut files = Files::new();
    for w in world_chain(lookup, id)? {
        if w.overrides.is_empty() {
            continue;
        }
        apply_overrides(base, &mut files, &w.overrides).map_err(|e| format!("{}: {e}", world_path(&w.id)))?;
        values_from(&overlay(base, &files)).map_err(|e| format!("{}: overrides: {e}", world_path(&w.id)))?;
    }
    Ok(files)
}

/// Go: `worldChain`: die Kette der Welt `id` mit ihren Eltern, die älteste zuerst. Abgelehnt: unbekannte Eltern-Welt,
/// Zyklus, mehr als `MAX_WORLD_CHAIN` Welten.
pub fn world_chain(lookup: LookupFn, id: &str) -> Result<Vec<WorldDef>, String> {
    let mut w = lookup(id)?;
    let mut chain = vec![w.clone()];
    while !w.extends.is_empty() {
        let parent = w.extends.clone();
        if chain.iter().any(|c| c.id == parent) {
            return Err(format!(
                "{}: extends: Zyklus {} → {parent}",
                world_path(id),
                chain_ids(&chain)
            ));
        }
        if chain.len() == MAX_WORLD_CHAIN {
            return Err(format!(
                "{}: extends: Kette zu lang ({} → {parent}, höchstens {MAX_WORLD_CHAIN} Welten)",
                world_path(id),
                chain_ids(&chain)
            ));
        }
        w = lookup(&parent)
            .map_err(|e| format!("{}: extends: Eltern-Welt {parent:?} unbekannt: {e}", world_path(&w.id)))?;
        chain.push(w.clone());
    }
    chain.reverse();
    Ok(chain)
}

/// Go: `chainIDs`: die IDs der Kette mit ` → ` verbunden.
fn chain_ids(chain: &[WorldDef]) -> String {
    chain.iter().map(|w| w.id.as_str()).collect::<Vec<_>>().join(" → ")
}

/// Go: `applyOverrides`: legt die Overrides einer Welt über den bisherigen Stand der Dateien (`files`, sonst `base`),
/// Datei für Datei in fester Reihenfolge, und schreibt das Ergebnis nach `files`.
fn apply_overrides(base: ReadFn, files: &mut Files, overrides: &BTreeMap<String, Value>) -> Result<(), String> {
    for (name, ov) in overrides {
        if !override_files().contains(&name.as_str()) {
            return Err(unknown_override_file(name));
        }
        let file = format!("{name}.json");
        let raw = overlay(base, files)(&file).map_err(|e| read_err(&file, &e))?;
        let merged = merge_file(name, &raw, ov).and_then(|m| check_locked(name, ov).map(|()| m));
        files.insert(
            file.clone(),
            merged.map_err(|e| format!("overrides › data/{file} › {e}"))?,
        );
    }
    Ok(())
}

/// Go: `fmt.Errorf("data/%s: %w", file, err)`; `Source::read` trägt das Präfix `data/<datei>: ` schon, es steht nur einmal.
fn read_err(file: &str, e: &Fehler) -> String {
    let msg = e.to_string();
    if msg.starts_with(&format!("data/{file}: ")) {
        msg
    } else {
        format!("data/{file}: {msg}")
    }
}

/// Go: `overlay`: die übersteuerten Dateien aus `files`, alle anderen aus `base`.
fn overlay<'a>(base: ReadFn<'a>, files: &'a Files) -> impl Fn(&str) -> Result<String, Fehler> + 'a {
    move |name| match files.get(name) {
        Some(raw) => Ok(raw.clone()),
        None => base(name),
    }
}

/// Go: `unknownOverrideFile`.
pub fn unknown_override_file(name: &str) -> String {
    format!(
        "overrides: unbekannte Datei {name:?} (erlaubt: {})",
        override_files().join(", ")
    )
}

/// Go: `WorldChildren`: die IDs der Welten aus `all`, die mit `extends` direkt von `id` erben, sortiert.
pub fn world_children(all: &[WorldDef], id: &str) -> Vec<String> {
    let mut out: Vec<String> = all.iter().filter(|w| w.extends == id).map(|w| w.id.clone()).collect();
    out.sort();
    out
}

/// Go: `CheckWorldRemovable`: Löschen oder Umbenennen der Welt `id` ist nur erlaubt, wenn keine Welt aus `all` von ihr
/// erbt; die Meldung nennt die Kind-Welten.
pub fn check_world_removable(all: &[WorldDef], id: &str) -> Result<(), String> {
    let kids = world_children(all, id);
    if kids.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{}: Welt hat Kind-Welten ({}), Löschen und Umbenennen abgelehnt",
        world_path(id),
        kids.join(", ")
    ))
}
