//! Inseln der Welt-Datei lesen und prüfen (Port von `engine/sim/island_worlds.go` ab `decodeIslands`): strenges Lesen
//! (`strict_decode`, `check_keys`), Pflichtfelder, Etappen und Häfen, Fehlermeldungen wie in Go (`Insel <id>: <Grund>`).
//!
//! Die Meldungen des Lesens selbst (`kein gültiges JSON: …`) tragen den Text von `serde_json`, nicht den von Go; Gos
//! Meldungen der Prüfung stehen Zeichen für Zeichen hier.

use std::collections::BTreeMap;

use k3c_data::Data;
use k3c_data::waves::WavesDataDepthScaling;
use k3c_data::world::valid_id;
use k3c_level::level::HARBOR_KIND;
use k3c_level::rules::Rules;
use k3c_level::types::{NatureValues, PlannedChunk};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use super::data::{IslandDef, check_island};
use super::keys::Keys;
use crate::state::pending::decode_island_story;

/// Erlaubte Felder einer Insel (`strictDecode` in `decodeIslands`).
const ISLAND_FIELDS: [&str; 11] = [
    "id",
    "name",
    "etappe",
    "depths",
    "depthScaling",
    "plannedChunks",
    "chunkCounts",
    "endBoss",
    "nature",
    "natureByDepth",
    "story",
];

/// Go: `rawIsland`: `Option` erkennt fehlende Pflichtfelder; ein Feld mit `null` zählt wie in Go als nicht gesetzt (Karten,
/// Texte) oder, bei `depthScaling`, als gesetzt (`json.RawMessage`).
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct RawIsland {
    id: Option<String>,
    name: Option<String>,
    etappe: Option<i64>,
    depths: Option<Vec<i64>>,
    #[serde(rename = "depthScaling", deserialize_with = "present")]
    depth_scaling: Option<Value>,
    #[serde(rename = "plannedChunks")]
    planned_chunks: Option<BTreeMap<String, Vec<PlannedChunk>>>,
    #[serde(rename = "chunkCounts")]
    chunk_counts: Option<BTreeMap<String, i64>>,
    #[serde(rename = "endBoss")]
    end_boss: Option<String>,
    nature: Option<Value>,
    #[serde(rename = "natureByDepth")]
    nature_by_depth: Option<BTreeMap<String, Value>>,
    story: Option<Value>,
}

/// Go: `rawScaling`.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct RawScaling {
    hp: Option<f64>,
    damage: Option<f64>,
    speed: Option<f64>,
}

/// Ein vorhandenes Feld ist `Some`, auch wenn sein Wert `null` ist (Go: `json.RawMessage` ist dann nicht `nil`).
fn present<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Value>, D::Error> {
    Value::deserialize(d).map(Some)
}

/// Go: `strictDecode`: Schlüssel exakt aus `fields` (Groß-/Kleinschreibung zählt, keiner doppelt), dann Lesen in `T`.
fn strict_decode<T: for<'de> Deserialize<'de>>(raw: &Value, keys: &Keys, fields: &[&str]) -> Result<T, String> {
    keys.check(fields)?;
    T::deserialize(raw).map_err(|e| format!("kein gültiges JSON: {e}"))
}

/// Go: `missingField`, ein fehlendes Pflichtfeld (welten.md § 2).
fn missing_field(name: &str) -> String {
    format!("{name} fehlt (Pflichtfeld)")
}

/// Go: `islandError`, `Insel <insel-id>: <Grund>` (welten.md § 3).
fn island_error(island: &str, err: &str) -> String {
    format!("Insel {island}: {err}")
}

/// Go: `decodeIslands` liest und prüft die Inseln der Reihe nach; die erste verletzte Regel wird gemeldet. Ohne ID heißt die
/// Insel in der Meldung „Nr. <Platz in der Liste>“. `keys` sind die Schlüsselbäume der Inseln (`Keys`).
pub fn decode_islands(islands: &[Value], keys: &[Keys], data: &Data, rules: &Rules) -> Result<Vec<IslandDef>, String> {
    let mut out: Vec<IslandDef> = Vec::new();
    for (i, raw) in islands.iter().enumerate() {
        let none = Keys::default();
        let k = keys.get(i).unwrap_or(&none);
        match decode_island(raw, k, i, &out, data, rules) {
            Ok(d) => out.push(d),
            Err(e) => return Err(island_error(&island_label(raw, i), &e)),
        }
    }
    for (i, d) in out.iter().enumerate() {
        if !has_harbor(d) && !sole_of_last_etappe(&out, d) {
            return Err(island_error(
                &island_label(&islands[i], i),
                "kein Hafen in den geplanten Chunks der Tiefe 0, die Insel ist nicht die einzige der letzten Etappe (jede Insel einer Etappe mit mehreren Inseln braucht einen Hafen, geplante-chunks.md § 10)",
            ));
        }
    }
    Ok(out)
}

/// Eine Insel lesen und prüfen (Schleifenrumpf von `decodeIslands`): Lesen, Pflichtfelder, Geschichte, ID, `check_island`,
/// Etappe.
fn decode_island(
    raw: &Value,
    keys: &Keys,
    i: usize,
    before: &[IslandDef],
    data: &Data,
    rules: &Rules,
) -> Result<IslandDef, String> {
    let r: RawIsland = strict_decode(raw, keys, &ISLAND_FIELDS)?;
    let mut d = r.def(keys)?;
    d.etappe = r.etappe.unwrap_or(i as i64 + 1);
    if let Some(s) = r.story.as_ref().filter(|s| !s.is_null()) {
        d.story = decode_island_story(s)?;
    }
    check_island_in(&d, before, data, rules)?;
    check_etappe(&d, before)?;
    Ok(d)
}

/// Go: `soleOfLastEtappe`: `d` ist die einzige Insel der letzten Etappe, also die letzte der Welt (nur sie braucht keinen
/// Hafen).
fn sole_of_last_etappe(islands: &[IslandDef], d: &IslandDef) -> bool {
    let last = islands.last().map_or(0, |l| l.etappe);
    d.etappe == last && !islands.iter().any(|o| o.id != d.id && o.etappe == last)
}

/// Go: `checkEtappe` prüft die Etappe von `d` gegen die Inseln davor (B-961): ab 1, lückenlos, in der Liste nicht fallend.
fn check_etappe(d: &IslandDef, before: &[IslandDef]) -> Result<(), String> {
    let prev = before.last().map_or(0, |l| l.etappe);
    if d.etappe < 1 {
        Err(format!("etappe {} ungültig (mindestens 1)", d.etappe))
    } else if d.etappe < prev {
        Err(format!(
            "etappe {} fällt unter die Etappe {prev} der Insel davor (nicht fallend)",
            d.etappe
        ))
    } else if d.etappe > prev + 1 {
        Err(format!(
            "etappe {} lässt Etappe {} aus (lückenlos ab 1)",
            d.etappe,
            prev + 1
        ))
    } else {
        Ok(())
    }
}

/// Go: `hasHarbor`: Die Insel plant in Tiefe 0 einen Hafen-Chunk (BAS, B-637).
fn has_harbor(d: &IslandDef) -> bool {
    d.planned_chunks
        .get("0")
        .is_some_and(|p| p.iter().any(|c| c.kind == HARBOR_KIND))
}

/// Go: `islandLabel` nennt die Insel in Meldungen: ihre ID, außerhalb des Musters in Anführungszeichen, ohne ID ihr Platz
/// in der Liste („Nr. 2“).
fn island_label(raw: &Value, i: usize) -> String {
    match raw.get("id").and_then(Value::as_str) {
        None => format!("Nr. {}", i + 1),
        Some(id) if valid_id(id) => id.to_string(),
        Some(id) => format!("{id:?}"),
    }
}

/// Go: `checkIslandIn`: ID nach Muster und eindeutig unter den Inseln davor, dann `check_island`.
fn check_island_in(d: &IslandDef, before: &[IslandDef], data: &Data, rules: &Rules) -> Result<(), String> {
    if !valid_id(&d.id) {
        return Err("id ungültig (a-z, 0-9, -, höchstens 32 Zeichen)".to_string());
    }
    if before.iter().any(|o| o.id == d.id) {
        return Err("doppelt".to_string());
    }
    check_island(d, data, rules)
}

impl RawIsland {
    /// Go: `rawIsland.def` prüft die Pflichtfelder der Insel und liefert sie als `IslandDef`.
    fn def(&self, keys: &Keys) -> Result<IslandDef, String> {
        let (Some(id), Some(name), Some(depths), Some(scaling)) =
            (&self.id, &self.name, &self.depths, &self.depth_scaling)
        else {
            let missing = if self.id.is_none() {
                "id"
            } else if self.name.is_none() {
                "name"
            } else if self.depths.is_none() {
                "depths"
            } else {
                "depthScaling"
            };
            return Err(missing_field(missing));
        };
        // `null` ist wie in Go ein vorhandenes Feld ohne Faktoren (Zeiger bleiben `nil`).
        let s: RawScaling = if scaling.is_null() {
            RawScaling::default()
        } else {
            strict_decode(scaling, keys.child("depthScaling"), &["hp", "damage", "speed"])
                .map_err(|e| format!("depthScaling: {e}"))?
        };
        let (Some(hp), Some(damage), Some(speed)) = (s.hp, s.damage, s.speed) else {
            let missing = if s.hp.is_none() {
                "depthScaling.hp"
            } else if s.damage.is_none() {
                "depthScaling.damage"
            } else {
                "depthScaling.speed"
            };
            return Err(missing_field(missing));
        };
        let mut d = IslandDef {
            id: id.clone(),
            name: name.clone(),
            depths: depths.clone(),
            planned_chunks: self.planned_chunks.clone().unwrap_or_default(),
            chunk_counts: self.chunk_counts.clone().unwrap_or_default(),
            end_boss: self.end_boss.clone().unwrap_or_default(),
            depth_scaling: WavesDataDepthScaling { hp, damage, speed },
            ..IslandDef::default()
        };
        d.nature = decode_nature(self.nature.as_ref(), keys.child("nature"), "nature")?;
        let by_depth = keys.child("natureByDepth");
        for (key, raw) in self.nature_by_depth.iter().flatten() {
            let v = decode_nature(Some(raw), by_depth.child(key), &format!("natureByDepth.{key}"))?;
            d.nature_by_depth.insert(key.clone(), v);
        }
        Ok(d)
    }
}

/// Go: `decodeNature` liest Natur-Werte streng (nur `traits` und `weights`); fehlt das Feld, sind sie leer.
fn decode_nature(raw: Option<&Value>, keys: &Keys, name: &str) -> Result<NatureValues, String> {
    let Some(raw) = raw else {
        return Ok(NatureValues::default());
    };
    strict_decode(raw, keys, &["traits", "weights"]).map_err(|e| format!("{name}: {e}"))
}
