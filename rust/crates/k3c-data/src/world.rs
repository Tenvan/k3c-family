//! data/worlds/*.json: Kopf einer Welt-Datei wie Gos `sim.rawWorld` und `sim.rawCombat` (engine/sim/island_worlds.go,
//! combat_data.go), geprüft wie `ParseWorld` bis zum Kopf (`decodeWorld` ohne Geschichte und Skins).
//!
//! Die Inseln bleiben roh wie in Go nach `decodeWorld`; Inseln, Geschichte, Skins und `checkStory` liest erst die
//! Welt-Logik der Sim (B-1024).

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize};

use crate::ordered::Ordered;
use crate::{Fehler, decode};

/// Go: `detvec.worldFile`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WorldFile {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "notes")]
    pub notes: String,
    #[serde(rename = "extends")]
    pub extends: Option<String>,
    /// Art der Welt: `spinoff` oder `new` (B-1217); fehlt sie, gilt `spinoff` mit `extends`, sonst `new`.
    #[serde(rename = "kind")]
    pub kind: Option<String>,
    /// Gewässer, an dem sich die Welt orientiert (Untertitel der Welt); leer = keine Angabe.
    #[serde(rename = "gewaesser")]
    pub gewaesser: String,
    #[serde(rename = "overrides")]
    pub overrides: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "combat")]
    pub combat: Option<RawCombat>,
    #[serde(rename = "islands")]
    pub islands: Option<Vec<serde_json::Value>>,
    #[serde(rename = "story")]
    pub story: serde_json::Value,
    /// `None` = Feld fehlt, `Some(Null)` = `"skins": null` (Go: `json.RawMessage` ist dann nicht `nil`).
    #[serde(rename = "skins", deserialize_with = "present")]
    pub skins: Option<serde_json::Value>,
}

/// Ein vorhandenes Feld ist `Some`, auch wenn sein Wert `null` ist.
fn present<'de, D: Deserializer<'de>>(d: D) -> Result<Option<serde_json::Value>, D::Error> {
    serde_json::Value::deserialize(d).map(Some)
}

/// Go: `detvec.rawCombat`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RawCombat {
    #[serde(rename = "mode")]
    pub mode: Option<String>,
    #[serde(rename = "groupSize")]
    pub group_size: Option<i64>,
}

/// Erlaubte Felder des Kopfes (`strictDecode` in decodeWorld).
const FIELDS: [&str; 11] = [
    "id",
    "name",
    "notes",
    "extends",
    "kind",
    "gewaesser",
    "overrides",
    "combat",
    "islands",
    "story",
    "skins",
];

/// Dateien, die eine Welt übersteuern darf (engine/sim `overrideFiles`), sortiert.
pub const OVERRIDE_FILES: [&str; 10] = [
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
];

/// `path` ist der Pfad der Welt in Meldungen (Go `worldPath`).
pub fn path(id: &str) -> String {
    format!("data/worlds/{id}.json")
}

/// `check_id` prüft eine Welt- oder Insel-ID wie Gos `idPattern` `^[a-z0-9][a-z0-9-]{0,31}$`.
pub fn valid_id(id: &str) -> bool {
    let ok = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit();
    let mut chars = id.chars();
    chars.next().is_some_and(ok) && id.len() <= 32 && chars.all(|c| ok(c) || c == '-')
}

/// `parse` liest den Kopf der Welt id wie `ParseWorld` bis `decodeWorld`: ID, Felder, Pflichtfelder, `extends`,
/// `overrides` und `combat`.
pub fn parse(raw: &str, id: &str) -> Result<WorldFile, Fehler> {
    if !valid_id(id) {
        return Err(Fehler::Pruefung(format!(
            "Welt-ID {id:?} ungültig (a-z, 0-9, -, höchstens 32 Zeichen)"
        )));
    }
    decode_world(raw, id).map_err(|e| Fehler::Pruefung(format!("{}: {e}", path(id))))
}

fn decode_world(raw: &str, id: &str) -> Result<WorldFile, String> {
    check_keys(raw)?;
    let v: WorldFile = decode("", raw).map_err(|e| format!("kein gültiges JSON{e}"))?;
    let (Some(wid), Some(name), Some(islands)) = (&v.id, &v.name, &v.islands) else {
        let missing = if v.id.is_none() {
            "id"
        } else if v.name.is_none() {
            "name"
        } else {
            "islands"
        };
        return Err(format!("{missing} fehlt (Pflichtfeld)"));
    };
    if wid != id {
        return Err(format!("id {wid:?} passt nicht zum Dateinamen"));
    }
    if name.is_empty() {
        return Err("name ist leer".into());
    }
    if islands.is_empty() {
        return Err("keine Insel".into());
    }
    check_world_links(id, v.extends.as_deref(), v.overrides.as_ref())?;
    check_kind(v.kind.as_deref(), v.extends.is_some())?;
    if let Some(c) = &v.combat {
        c.check()?;
    }
    Ok(v)
}

/// `check_keys` prüft die Schlüssel der obersten Ebene: nur bekannte, keiner doppelt (Go `checkKeys`).
fn check_keys(raw: &str) -> Result<(), String> {
    let Ok(top) = serde_json::from_str::<Ordered<serde_json::Value>>(raw) else {
        return Ok(()); // kein Objekt: das meldet das Lesen
    };
    for (i, e) in top.0.iter().enumerate() {
        if !FIELDS.contains(&e.key.as_str()) {
            return Err(format!("unbekanntes Feld {:?}", e.key));
        }
        if top.0[..i].iter().any(|o| o.key == e.key) {
            return Err(format!("doppeltes Feld {:?}", e.key));
        }
    }
    Ok(())
}

/// `check_world_links` prüft `extends` und die Dateinamen in `overrides` (Go `checkWorldLinks`).
fn check_world_links(
    id: &str,
    extends: Option<&str>,
    overrides: Option<&BTreeMap<String, serde_json::Value>>,
) -> Result<(), String> {
    if let Some(e) = extends {
        if !valid_id(e) {
            return Err(format!(
                "extends: Welt-ID {e:?} ungültig (a-z, 0-9, -, höchstens 32 Zeichen)"
            ));
        }
        if e == id {
            return Err("extends: Welt erbt von sich selbst".into());
        }
    }
    for name in overrides.into_iter().flat_map(|o| o.keys()) {
        if !OVERRIDE_FILES.contains(&name.as_str()) {
            return Err(format!(
                "overrides: unbekannte Datei {name:?} (erlaubt: {})",
                OVERRIDE_FILES.join(", ")
            ));
        }
    }
    Ok(())
}

/// `check_kind` prüft die Art der Welt wie Gos `worldKind`.
fn check_kind(kind: Option<&str>, has_extends: bool) -> Result<(), String> {
    match kind {
        None | Some("new") => Ok(()),
        Some("spinoff") if has_extends => Ok(()),
        Some("spinoff") => Err("kind: spinoff braucht extends".into()),
        Some(k) => Err(format!("kind: {k:?} unbekannt (erlaubt: spinoff, new)")),
    }
}

impl RawCombat {
    /// `check` prüft `combat` wie Gos `rawCombat.settings`.
    fn check(&self) -> Result<(), String> {
        if let Some(m) = &self.mode
            && m != "groups"
            && m != "general"
        {
            return Err(format!("combat.mode {m:?} unbekannt (groups oder general)"));
        }
        if self.group_size.is_some_and(|g| g < 1) {
            return Err("combat.groupSize muss eine ganze Zahl ≥ 1 sein".into());
        }
        Ok(())
    }
}
