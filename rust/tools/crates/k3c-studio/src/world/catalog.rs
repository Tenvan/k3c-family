//! Welt-Zuordnung in den Katalogen (Port von `catalog.go`, SI7.1): Wetter, Reittier und Einsiedler stehen nicht in der
//! Welt-Datei, sondern im Feld `worlds` der drei Katalog-Dateien (docs/rules/welten.md § 2 und 3). Ein Eintrag fehlt,
//! wenn die Welt die Art nicht hat; die Auswahl „keins“ entfernt ihn.

use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::value::RawValue;

use super::edit::{CreateView, ERR_STORY_TOOL, HEADER_KEYS};
use super::overrides::{given, write_atomic, write_lock};
use super::patch::{Span, value_span};
use super::values::{object_items, parse_entries};
use super::{Editor, check_id};
use crate::kitx::go_quote;

/// Eine Art: Katalog-Datei und die Liste mit `id`-Einträgen (`None`: die Schlüssel der obersten Ebene mit Objekt-Wert,
/// so die Wetter-Datei).
struct CatalogPick {
    key: &'static str,
    file: &'static str,
    list: Option<&'static str>,
}

/// Die Arten, nach Schlüssel sortiert.
const PICKS: [CatalogPick; 3] = [
    CatalogPick {
        key: "hermit",
        file: "data/hermits.json",
        list: Some("hermits"),
    },
    CatalogPick {
        key: "mount",
        file: "data/mounts.json",
        list: Some("mounts"),
    },
    CatalogPick {
        key: "weather",
        file: "data/weather.json",
        list: None,
    },
];

fn pick(key: &str) -> Option<&'static CatalogPick> {
    PICKS.iter().find(|p| p.key == key)
}

/// Felder der Katalog-Dateien, die keine Art sind.
const RESERVED: [&str; 3] = ["version", "notes", "worlds"];

/// Antwort für die Auswahl je Welt: Werte je Art (leer = keins) und die gültigen IDs je Art.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PicksView {
    pub values: BTreeMap<String, String>,
    pub options: BTreeMap<String, Vec<String>>,
    pub error: String,
}

/// Schlüssel und Werte des Blocks `worlds` in der Reihenfolge der Datei.
fn worlds_of(raw: &[u8]) -> Result<Vec<(String, String)>, String> {
    let top = parse_entries(raw).map_err(|e| e.to_string())?;
    let Some((_, block)) = top.into_iter().rfind(|(k, _)| k == "worlds") else {
        return Ok(Vec::new());
    };
    object_items(block.get())
        .into_iter()
        .map(|(world, value)| {
            let text = serde_json::from_str::<Option<String>>(value.get()).map_err(|e| format!("worlds: {e}"))?;
            Ok((world, text.unwrap_or_default()))
        })
        .collect()
}

/// Der Block `worlds` im Stil der Katalog-Dateien: je Welt eine Zeile.
fn render_worlds(worlds: &[(String, String)]) -> String {
    if worlds.is_empty() {
        return "{}".to_owned();
    }
    let lines: Vec<String> = worlds
        .iter()
        .map(|(k, v)| format!("    {}: {}", go_quote(k), go_quote(v)))
        .collect();
    format!("{{\n{}\n  }}", lines.join(",\n"))
}

/// Wert einer Art aus dem Patch: Text oder `null` (= keins, entfernt den Eintrag).
fn pick_value(value: &str) -> Result<String, String> {
    if !given(value) {
        return Ok(String::new());
    }
    serde_json::from_str(value).map_err(|_| "Katalog-ID als Text oder null".to_owned())
}

/// Prüft die Schlüssel eines `world_set`-Patches: Arten, Kopf-Felder und `islands`; `story` gehört zu
/// `world_story_set`.
pub(super) fn check_patch_keys(patch: &BTreeMap<String, Box<RawValue>>) -> Result<(), String> {
    for key in patch.keys() {
        if key == "story" {
            return Err(ERR_STORY_TOOL.to_owned());
        }
        if pick(key).is_none() && !HEADER_KEYS.contains(&key.as_str()) && key != "islands" {
            return Err(format!(
                "unbekanntes Feld {key:?} im patch (erlaubt: {}, islands, weather, mount, hermit)",
                HEADER_KEYS.join(", ")
            ));
        }
    }
    Ok(())
}

impl Editor {
    fn catalog_text(&self, file: &str) -> Result<String, String> {
        let path = self.root.join(file);
        std::fs::read_to_string(&path).map_err(|e| format!("open {}: {e}", path.display()))
    }

    /// Die gültigen IDs der Art in der Reihenfolge der Datei.
    fn catalog_ids(&self, p: &CatalogPick) -> Result<Vec<String>, String> {
        #[derive(serde::Deserialize)]
        struct Item {
            #[serde(default)]
            id: String,
        }
        let raw = self.catalog_text(p.file)?;
        let Some(list) = p.list else {
            let kinds = object_items(&raw).into_iter();
            let kinds = kinds.filter(|(k, v)| !RESERVED.contains(&k.as_str()) && v.get().starts_with('{'));
            return Ok(kinds.map(|(k, _)| k).collect());
        };
        let top = parse_entries(raw.as_bytes()).map_err(|e| format!("{}: {e}", p.file))?;
        let found = top.into_iter().rfind(|(k, _)| k == list);
        let text = found.map_or("", |(_, v)| v.get());
        let items = serde_json::from_str::<Option<Vec<Item>>>(text).map_err(|e| format!("{} › {list}: {e}", p.file))?;
        Ok(items.unwrap_or_default().into_iter().map(|i| i.id).collect())
    }

    /// Die Zuordnung der Welt `id` je Art (weather, mount, hermit); fehlt ein Eintrag, steht dort "".
    pub fn picks(&self, id: &str) -> Result<BTreeMap<String, String>, String> {
        check_id(id)?;
        let mut out = BTreeMap::new();
        for p in &PICKS {
            let raw = self.catalog_text(p.file)?;
            let worlds = worlds_of(raw.as_bytes()).map_err(|e| format!("{}: {e}", p.file))?;
            let value = worlds.into_iter().find(|(world, _)| world == id);
            out.insert(p.key.to_owned(), value.map(|(_, v)| v).unwrap_or_default());
        }
        Ok(out)
    }

    /// Setzt die Art `key` der Welt `id` auf die Katalog-ID `value`; "" entfernt den Eintrag. Die ID muss im Katalog
    /// stehen, die Welt muss es geben. Geschrieben wird nur der Block `worlds`, die Reihenfolge der Datei bleibt.
    pub fn set_pick(&self, id: &str, key: &str, value: &str) -> Result<(), String> {
        let p = self.check_pick(id, key, value)?;
        let _guard = write_lock();
        let path = self.root.join(p.file);
        let raw = self.catalog_text(p.file)?.into_bytes();
        let Some(block) = value_span(&raw, Span::whole(&raw), "worlds")? else {
            return Err(format!("{}: Feld worlds fehlt", p.file));
        };
        let mut worlds = worlds_of(&raw).map_err(|e| format!("{}: {e}", p.file))?;
        match worlds.iter().position(|(world, _)| world == id) {
            Some(i) if value.is_empty() => {
                worlds.remove(i);
            }
            Some(i) => value.clone_into(&mut worlds[i].1),
            None if !value.is_empty() => worlds.push((id.to_owned(), value.to_owned())),
            None => {}
        }
        let text = render_worlds(&worlds);
        let out = [&raw[..block.from], text.as_bytes(), &raw[block.to..]].concat();
        if out == raw {
            return Ok(());
        }
        write_atomic(&path, &out, &self.log)
    }

    /// Prüft Art, Welt und ID, ohne etwas zu schreiben.
    fn check_pick(&self, id: &str, key: &str, value: &str) -> Result<&'static CatalogPick, String> {
        let Some(p) = pick(key) else {
            let known: Vec<&str> = PICKS.iter().map(|p| p.key).collect();
            return Err(format!("unbekannte Art {key:?} (erlaubt: {})", known.join(", ")));
        };
        check_id(id)?;
        if !self.file(id).exists() {
            return Err(format!("welt {id:?} gibt es nicht"));
        }
        if value.is_empty() {
            return Ok(p);
        }
        let ids = self.catalog_ids(p)?;
        if !ids.iter().any(|known| known == value) {
            return Err(format!(
                "unbekannte ID {value:?} für {key} (gültig: {})",
                ids.join(", ")
            ));
        }
        Ok(p)
    }

    /// Schreibt die Arten des Patches in die Katalog-Dateien, vor den Welt-Dateien. Erst alle Werte prüfen, dann
    /// schreiben: ein Patch mit einer ungültigen ID lässt keine der anderen Arten stehen.
    pub(super) fn apply_picks(&self, id: &str, patch: &BTreeMap<String, Box<RawValue>>) -> Result<(), String> {
        let mut picks = Vec::new();
        for (key, raw) in patch.iter().filter(|(k, _)| pick(k).is_some()) {
            let value = pick_value(raw.get()).map_err(|e| format!("{key}: {e}"))?;
            self.check_pick(id, key, &value)?;
            picks.push((key, value));
        }
        picks
            .into_iter()
            .try_for_each(|(key, value)| self.set_pick(id, key, &value))
    }

    /// Werte und Optionen der Arten für die Oberfläche (Binding `WorldPicks`).
    pub fn picks_view(&self, id: &str) -> PicksView {
        let mut view = PicksView::default();
        match self.picks(id) {
            Ok(values) => view.values = values,
            Err(e) => {
                view.error = e;
                return view;
            }
        }
        for p in &PICKS {
            match self.catalog_ids(p) {
                Ok(ids) => view.options.insert(p.key.to_owned(), ids),
                Err(e) => {
                    view.error = e;
                    return view;
                }
            };
        }
        view
    }

    /// Setzt eine Art der Welt und antwortet wie die anderen Änderungen (Binding `WorldSetPick`).
    pub fn set_pick_view(&self, id: &str, key: &str, value: &str) -> CreateView {
        CreateView {
            id: id.to_owned(),
            warnings: None,
            error: self.set_pick(id, key, value).err().unwrap_or_default(),
        }
    }
}

#[cfg(test)]
#[path = "catalog_tests.rs"]
mod tests;
