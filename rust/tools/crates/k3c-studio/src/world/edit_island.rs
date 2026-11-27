//! Inseln einer Welt bearbeiten (Port von `edit_island.go`): `world_set` › islands und `world_chunks_set`.

use std::collections::BTreeMap;

use serde_json::value::RawValue;

use super::edit::{ERR_STORY_TOOL, Warnings, eol, set_field};
use super::inline::inline;
use super::nature::{SpanError, island_span};
use super::overrides::{given, write_lock};
use super::patch::{Span, insert_span, value_span};
use super::story::numeric_keys;
use super::values::parse_entries;
use super::{Editor, valid_id};
use crate::kitx::{compact, go_quote};

/// Felder einer Insel im Patch (`id` wählt die Insel, `biomes` ist die Kurzform von `depths`).
const ISLAND_KEYS: [&str; 9] = [
    "name",
    "etappe",
    "depths",
    "depthScaling",
    "plannedChunks",
    "chunkCounts",
    "endBoss",
    "nature",
    "natureByDepth",
];

/// Tiefen-Skalierung einer neuen Insel (wie die Standard-Welt, Insel 1).
const DEFAULT_SCALING: &str = r#"{"hp": 1.5, "damage": 1.3, "speed": 1.1}"#;

/// JSON-Texte je Schlüssel (Gos `map[string]json.RawMessage`).
type Fields = BTreeMap<String, String>;

/// Ein Eintrag der Liste `islands`; `None` ist `null`.
type Entry = Option<BTreeMap<String, Box<RawValue>>>;

/// Das Objekt `raw` als Texte je Schlüssel (der letzte gewinnt).
fn fields_of(raw: &[u8]) -> Result<Fields, String> {
    let entries = parse_entries(raw).map_err(|e| e.to_string())?;
    Ok(entries.into_iter().map(|(k, v)| (k, v.get().to_owned())).collect())
}

/// Zeile einer neuen Insel; `extra` sind fertige `"schlüssel": wert`-Paare.
pub(super) fn new_island(id: &str, name: &str, depths: &str, extra: &[String]) -> String {
    let mut parts = vec![
        format!("\"id\": {}", go_quote(id)),
        format!("\"name\": {}", go_quote(name)),
        format!("\"depths\": {depths}"),
        "\"depthScaling\": { \"hp\": 1.5, \"damage\": 1.3, \"speed\": 1.1 }".to_owned(),
    ];
    parts.extend_from_slice(extra);
    format!("{{ {} }}", parts.join(", "))
}

/// Ändert die Insel des Eintrags feldweise oder hängt sie an.
fn patch_island(raw: &[u8], entry: &Entry, biomes: &BTreeMap<i64, String>) -> Result<Vec<u8>, String> {
    let id = entry.as_ref().and_then(|e| e.get("id"));
    let id = id.and_then(|v| serde_json::from_str::<String>(v.get()).ok());
    let (Some(entry), Some(id)) = (entry.as_ref(), id.filter(|id| valid_id(id))) else {
        return Err("id fehlt oder ist ungültig".to_owned());
    };
    let fields = island_fields(entry, biomes)?;
    let mut obj = match island_span(raw, &id) {
        Ok((obj, _)) => obj,
        Err(SpanError::Missing(_)) => return append_island(raw, &id, &fields),
        Err(e) => return Err(e.into()),
    };
    let mut out = raw.to_vec();
    for (key, value) in &fields {
        out = set_island_field(&out, obj, key, value)?;
        (obj, _) = island_span(&out, &id)?;
    }
    Ok(out)
}

/// Setzt das Feld `key` der Insel `obj`; `depthScaling` wird gemischt, `etappe` steht hinter `name`.
fn set_island_field(raw: &[u8], obj: Span, key: &str, value: &str) -> Result<Vec<u8>, String> {
    match key {
        "etappe" => set_etappe(raw, obj, value),
        "depthScaling" if given(value) => {
            let at = value_span(raw, obj, key).ok().flatten();
            let current = at.map_or_else(|| "{}".into(), |at| String::from_utf8_lossy(&raw[at.from..at.to]));
            let merged = merge_scaling(&current, value)?;
            set_field(raw, obj, key, &merged, false)
        }
        _ => set_field(raw, obj, key, value, false),
    }
}

fn whole_number(value: &str) -> Result<i64, String> {
    serde_json::from_str(value).map_err(|_| "etappe muss eine ganze Zahl sein".to_owned())
}

/// Setzt `etappe` (`null` entfernt es); ein neues Feld steht hinter `name` (docs/rules/welten.md § 2). Die Zahl prüft
/// die Engine vor dem Schreiben (≥ 1, lückenlos, nicht fallend).
fn set_etappe(raw: &[u8], obj: Span, value: &str) -> Result<Vec<u8>, String> {
    if value_span(raw, obj, "etappe")?.is_some() || !given(value) {
        return set_field(raw, obj, "etappe", value, false);
    }
    let n = whole_number(value)?;
    let Ok(Some(name)) = value_span(raw, obj, "name") else {
        return set_field(raw, obj, "etappe", value, false);
    };
    let added = format!(", \"etappe\": {n}");
    Ok([&raw[..name.to], added.as_bytes(), &raw[name.to..]].concat())
}

/// Prüft die Felder des Eintrags und macht aus `biomes` die `depths`.
fn island_fields(entry: &BTreeMap<String, Box<RawValue>>, biomes: &BTreeMap<i64, String>) -> Result<Fields, String> {
    let mut fields = Fields::new();
    for (key, value) in entry {
        match key.as_str() {
            "id" => {}
            "biomes" => {
                let depths = biome_depths(value.get(), biomes)?;
                if entry.contains_key("depths") {
                    return Err("depths oder biomes, nicht beides".to_owned());
                }
                fields.insert("depths".to_owned(), depths);
            }
            k if ISLAND_KEYS.contains(&k) => {
                fields.insert(key.clone(), value.get().to_owned());
            }
            "story" => return Err(ERR_STORY_TOOL.to_owned()),
            _ => {
                return Err(format!(
                    "unbekanntes Feld {key:?} (erlaubt: id, biomes, {})",
                    ISLAND_KEYS.join(", ")
                ));
            }
        }
    }
    Ok(fields)
}

/// Übersetzt Biom-IDs in die Tiefen (Feld `depth` der Biome unter `data/biomes/`).
fn biome_depths(list: &str, biomes: &BTreeMap<i64, String>) -> Result<String, String> {
    let ids = serde_json::from_str::<Option<Vec<Option<String>>>>(list)
        .map_err(|_| "biomes muss eine Liste von Biom-IDs sein".to_owned())?;
    let mut depths = Vec::new();
    for id in ids.unwrap_or_default() {
        let id = id.unwrap_or_default();
        let before = depths.len();
        depths.extend(biomes.iter().filter(|(_, b)| **b == id).map(|(d, _)| d.to_string()));
        if depths.len() == before {
            let mut known: Vec<&str> = biomes.values().map(String::as_str).collect();
            known.sort_unstable();
            return Err(format!("unbekanntes Biom {id:?} (bekannt: {})", known.join(", ")));
        }
    }
    Ok(format!("[{}]", depths.join(",")))
}

/// Legt die Felder von `next` über `current` (hp, damage, speed, in dieser Reihenfolge).
fn merge_scaling(current: &str, next: &str) -> Result<String, String> {
    let object = |text: &str| -> Result<Fields, String> {
        if !given(text) {
            return Ok(Fields::new());
        }
        fields_of(text.as_bytes())
    };
    let mut merged = object(current).map_err(|e| format!("depthScaling: {e}"))?;
    merged.extend(object(next).map_err(|_| "depthScaling muss ein Objekt sein (hp, damage, speed)".to_owned())?);
    let mut keys = vec!["hp", "damage", "speed"];
    let rest: Vec<&str> = merged
        .keys()
        .map(String::as_str)
        .filter(|k| !keys.contains(k))
        .collect();
    keys.extend(rest);
    let parts: Vec<String> = keys
        .iter()
        .filter_map(|k| merged.get(*k).map(|v| format!("{}: {v}", go_quote(k))))
        .collect();
    Ok(format!("{{{}}}", parts.join(", ")))
}

/// Hängt eine neue Insel an die Liste an.
fn append_island(raw: &[u8], id: &str, fields: &Fields) -> Result<Vec<u8>, String> {
    let line = island_line(id, fields)?;
    let Ok(Some(list)) = value_span(raw, Span::whole(raw), "islands") else {
        return Err("islands fehlt in der Welt-Datei".to_owned());
    };
    let at = insert_span(raw, list);
    let separator = if raw[at.from - 1] == b'[' { "" } else { "," };
    let added = eol(raw, &format!("{separator}\n    {line}"));
    Ok([&raw[..at.from], added.as_slice(), &raw[at.from..]].concat())
}

/// Zeile einer neuen Insel: Name = id, Tiefen 0-4 und die Skalierung der Standard-Welt, soweit `fields` nichts anderes
/// nennt; übrige Felder folgen nach Namen sortiert.
fn island_line(id: &str, fields: &Fields) -> Result<String, String> {
    let name = match fields.get("name") {
        Some(value) => {
            let text = serde_json::from_str::<String>(value).ok().filter(|t| !t.is_empty());
            go_quote(&text.ok_or_else(|| "name muss ein Text sein".to_owned())?)
        }
        None => go_quote(id),
    };
    let depths = match fields.get("depths") {
        Some(value) => inline(value)?,
        None => "[0, 1, 2, 3, 4]".to_owned(),
    };
    let scaling = match fields.get("depthScaling") {
        Some(value) => merge_scaling(DEFAULT_SCALING, value)?,
        None => DEFAULT_SCALING.to_owned(),
    };
    let scaling = inline(&scaling)?;
    let extra = island_extras(fields)?;
    let mut parts = vec![format!("\"id\": {}", go_quote(id)), format!("\"name\": {name}")];
    if let Some(value) = fields.get("etappe").filter(|v| given(v)) {
        parts.push(format!("\"etappe\": {}", whole_number(value)?));
    }
    parts.push(format!("\"depths\": {depths}"));
    parts.push(format!("\"depthScaling\": {scaling}"));
    parts.extend(extra);
    Ok(format!("{{ {} }}", parts.join(", ")))
}

/// Die Felder neben Name, Etappe, Tiefen und Skalierung als fertige `"schlüssel": wert`-Paare.
fn island_extras(fields: &Fields) -> Result<Vec<String>, String> {
    fields
        .iter()
        .filter(|(k, v)| !matches!(k.as_str(), "name" | "etappe" | "depths" | "depthScaling") && given(v))
        .map(|(k, v)| {
            let text = inline(v).map_err(|e| format!("{k}: {e}"))?;
            Ok(format!("{}: {text}", go_quote(k)))
        })
        .collect()
}

/// Setzt den Eintrag `key` im Tiefen-Objekt `field` der Insel (`None`, `null` und `[]` entfernen ihn; ein leeres Objekt
/// entfällt ganz). Die Schlüssel stehen nach Tiefe sortiert.
fn set_depth_entry(raw: &[u8], island: &str, field: &str, key: &str, value: Option<&str>) -> Result<Vec<u8>, String> {
    let (obj, _) = island_span(raw, island)?;
    let mut entries = Fields::new();
    if let Ok(Some(at)) = value_span(raw, obj, field) {
        let text = &raw[at.from..at.to];
        if text != b"null" {
            entries = fields_of(text).map_err(|e| format!("{field}: {e}"))?;
        }
    }
    match value.map(str::trim).filter(|t| given(t) && compact(t) != "[]") {
        Some(text) => entries.insert(key.to_owned(), text.to_owned()),
        None => entries.remove(key),
    };
    let mut text = "null".to_owned();
    if !entries.is_empty() {
        let parts: Vec<String> = numeric_keys(&entries)
            .into_iter()
            .map(|k| format!("{}: {}", go_quote(k), entries[k]))
            .collect();
        text = format!("{{{}}}", parts.join(", "));
    }
    set_field(raw, obj, field, &text, false)
}

impl Editor {
    /// Wendet die Einträge der Liste `list` der Reihe nach an (`world_set` › islands): Eine Insel mit bekannter id wird
    /// feldweise geändert (`null` entfernt ein Feld), eine neue id wird am Ende angehängt.
    pub(super) fn patch_islands(&self, raw: &[u8], list: &str) -> Result<Vec<u8>, String> {
        let entries = serde_json::from_str::<Option<Vec<Entry>>>(list)
            .map_err(|_| "islands im patch muss eine Liste von Objekten sein".to_owned())?;
        let biomes = self.biomes_by_depth();
        let mut out = raw.to_vec();
        for (i, entry) in entries.unwrap_or_default().iter().enumerate() {
            out = patch_island(&out, entry, &biomes).map_err(|e| format!("islands[{i}]: {e}"))?;
        }
        Ok(out)
    }

    /// Setzt die geplanten Chunks (`plannedChunks`) und/oder die Chunk-Anzahl (`chunkCounts`) der Tiefe `depth` der
    /// Insel. `chunks` `None` lässt sie, `null` oder `[]` entfernt sie; `count` `None` lässt die Anzahl, 0 entfernt sie.
    pub fn set_chunks(
        &self,
        id: &str,
        island: &str,
        depth: i64,
        chunks: Option<&str>,
        count: Option<i64>,
    ) -> Result<Warnings, String> {
        if chunks.is_none() && count.is_none() {
            return Err("chunks oder count fehlt".to_owned());
        }
        if count.is_some_and(|c| c < 0) {
            return Err("count darf nicht negativ sein".to_owned());
        }
        let _guard = write_lock();
        let raw = self.read(id)?;
        let (_, head) = island_span(&raw, island)?;
        if !head.depths.contains(&depth) {
            let depths: Vec<String> = head.depths.iter().map(i64::to_string).collect();
            return Err(format!(
                "die Tiefe {depth} gehört nicht zur Insel {island:?} (Tiefen: [{}])",
                depths.join(" ")
            ));
        }
        let key = depth.to_string();
        let mut out = raw.clone();
        if chunks.is_some() {
            out = set_depth_entry(&out, island, "plannedChunks", &key, chunks)?;
        }
        if let Some(count) = count {
            let value = (count != 0).then(|| count.to_string());
            out = set_depth_entry(&out, island, "chunkCounts", &key, value.as_deref())?;
        }
        self.commit_change(id, &raw, &out)
    }
}
