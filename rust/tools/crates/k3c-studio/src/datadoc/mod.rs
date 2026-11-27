//! Die Dateien in `data/`, die das Balancing beeinflussen, und ihre Beschreibungen unter `docs/data/` (Port von
//! `tools/k3c-studio/internal/datadoc`, WZ28.10, B-608, B-610). Die Balancing-Seite zeigt daraus die Liste, die
//! Beschreibung als Popup und die Zahlenwerte einer Datei als Pfade in der Form von `balance`-Varianten
//! (`datei.json:a.b.c`). Hier wird nur gelesen.

use std::fmt;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;
use serde::de::{DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};

use crate::kitx::go_quoted;

// Leser einer Datei (Spalte „gelesen von“).
pub const BY_SIM: &str = "Simulation";
pub const BY_LEVEL: &str = "Level-Generator";
pub const BY_TESTER: &str = "Balancing-Tester";

/// Eine Datei der Liste: Name wie in `data/`, Beschreibung `docs/data/<doc>.md`, Leser. `vary`: Varianten wirken, weil
/// ein Balancing-Lauf die Datei neu lädt (`RELOADED` in `rust/crates/k3c-balance/src/vary.rs`).
struct Entry {
    name: &'static str,
    doc: &'static str,
    read_by: &'static [&'static str],
    vary: bool,
}

const fn entry(name: &'static str, doc: &'static str, read_by: &'static [&'static str], vary: bool) -> Entry {
    Entry {
        name,
        doc,
        read_by,
        vary,
    }
}

const SIM: &[&str] = &[BY_SIM];
const LEVEL: &[&str] = &[BY_LEVEL];
const SIM_LEVEL: &[&str] = &[BY_SIM, BY_LEVEL];
const TESTER: &[&str] = &[BY_TESTER];

/// Alle Dateien, die Simulation, Level-Generator oder Balancing-Tester lesen. Der Test `liste_wie_loader` hält die
/// Liste gleich mit den Dateinamen, die `k3c-data` lädt.
const FILES: [Entry; 21] = [
    entry("economy.json", "economy", SIM_LEVEL, true),
    entry("buildings.json", "buildings", SIM_LEVEL, true),
    entry("hub.json", "hub", SIM_LEVEL, true),
    entry("troops.json", "troops", SIM, true),
    entry("enemies.json", "enemies", SIM, true),
    entry("waves.json", "waves", SIM, true),
    entry("difficulty.json", "difficulty", SIM, false),
    entry("bosses.json", "bosses", SIM, false),
    entry("guards.json", "guards", LEVEL, false),
    entry("events.json", "events", SIM, false),
    entry("goals.json", "goals", SIM, true),
    entry("island-sizes.json", "island-sizes", SIM, false),
    entry("ruins.json", "ruins", LEVEL, false),
    entry("light.json", "light", SIM, false),
    entry("monarch.json", "monarch", SIM, true),
    entry("biomes/*.json", "biomes", SIM_LEVEL, false),
    entry("worlds/*.json", "worlds", SIM, false),
    entry("balance-targets.json", "balance-targets", TESTER, false),
    entry("balance-hints.json", "balance-hints", TESTER, false),
    entry("ship.json", "ship", &[BY_SIM, BY_TESTER], true),
    entry("story.json", "story", SIM, false),
];

/// Eine Zeile der Liste auf der Balancing-Seite.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub name: &'static str,
    pub doc: &'static str,
    pub read_by: &'static [&'static str],
    pub purpose: String,
    pub key: String,
    /// Die Zahlenwerte taugen als Variante.
    pub vary: bool,
    pub missing: bool,
}

/// Ein Zahlenwert einer Datei mit seinem Pfad (`a.b.0.c`).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Value {
    pub path: String,
    pub value: f64,
}

/// Ein Zahlenwert mit Datei und Pfad (Suche des Daten-Browsers, B-662).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FileValue {
    pub file: &'static str,
    pub path: String,
    pub value: f64,
}

// `\s` wie in Go: nur ASCII-Leerraum.
static PURPOSE_LINE: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"(?m)^- \*\*Zweck:\*\*[\t\n\f\r ]*(.+)$").ok());
static KEY_LINE: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"(?m)^- \*\*Wichtigste Werte:\*\*[\t\n\f\r ]*(.+)$").ok());

/// Alle Dateien mit Zweck und wichtigsten Werten aus `docs/data/`; fehlt eine Beschreibung, ist `missing` gesetzt.
pub fn list(root: &Path) -> Vec<File> {
    FILES
        .iter()
        .map(|e| {
            let mut file = File {
                name: e.name,
                doc: e.doc,
                read_by: e.read_by,
                vary: e.vary,
                ..File::default()
            };
            match doc(root, e.doc) {
                Err(_) => file.missing = true,
                Ok(md) => {
                    file.purpose = first_match(&PURPOSE_LINE, &md);
                    file.key = first_match(&KEY_LINE, &md);
                }
            }
            file
        })
        .collect()
}

fn first_match(re: &LazyLock<Option<Regex>>, text: &str) -> String {
    re.as_ref()
        .and_then(|re| re.captures(text))
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_owned())
        .unwrap_or_default()
}

/// Liest `docs/data/<name>.md`; nur Namen der Liste, nie ein anderer Pfad.
pub fn doc(root: &Path, name: &str) -> Result<String, String> {
    if !FILES.iter().any(|e| e.doc == name) {
        return Err(format!(
            "keine Beschreibung {}: nur die Dateien der Liste",
            go_quoted(name)
        ));
    }
    let path = root.join("docs").join("data").join(format!("{name}.md"));
    match fs::read(&path) {
        Ok(raw) => Ok(String::from_utf8_lossy(&raw).into_owned()),
        Err(e) if e.kind() == ErrorKind::NotFound => Err(format!("docs/data/{name}.md fehlt noch")),
        Err(e) => Err(format!("open {}: {e}", path.display())),
    }
}

/// Alle Zahlenwerte einer variierbaren Datei (z. B. `economy.json`) in Dateireihenfolge.
pub fn values(root: &Path, name: &str) -> Result<Vec<Value>, String> {
    if !FILES.iter().any(|e| e.name == name && e.vary) {
        return Err(format!("{}: nur variierbare Dateien der Liste", go_quoted(name)));
    }
    let path = root.join("data").join(name);
    let raw = fs::read(&path).map_err(|e| format!("open {}: {e}", path.display()))?;
    walk_numbers(&raw)
}

/// Die Zahlenwerte aller variierbaren Dateien in einem Aufruf, je Datei in Dateireihenfolge.
pub fn all_values(root: &Path) -> Result<Vec<FileValue>, String> {
    let mut out = Vec::new();
    for e in FILES.iter().filter(|e| e.vary) {
        let found = values(root, e.name).map_err(|err| format!("{}: {err}", e.name))?;
        out.extend(found.into_iter().map(|v| FileValue {
            file: e.name,
            path: v.path,
            value: v.value,
        }));
    }
    Ok(out)
}

/// Jede Zahl des ersten JSON-Werts in Dateireihenfolge (beim Lesen gesammelt, damit die Reihenfolge der Schlüssel
/// bleibt).
fn walk_numbers(raw: &[u8]) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    let walk = Walk {
        path: String::new(),
        out: &mut out,
    };
    walk.deserialize(&mut serde_json::Deserializer::from_slice(raw))
        .map_err(|e| e.to_string())?;
    Ok(out)
}

/// Ein Wert unter `path`.
struct Walk<'a> {
    path: String,
    out: &'a mut Vec<Value>,
}

impl Walk<'_> {
    fn child(&mut self, key: &str) -> Walk<'_> {
        let path = if self.path.is_empty() {
            key.to_owned()
        } else {
            format!("{}.{key}", self.path)
        };
        Walk { path, out: self.out }
    }

    fn number(self, value: f64) {
        self.out.push(Value { path: self.path, value });
    }
}

impl<'de> DeserializeSeed<'de> for Walk<'_> {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<(), D::Error> {
        d.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for Walk<'_> {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JSON")
    }

    // wie Go `Number.Float64`: große ganze Zahlen werden gerundet
    #[allow(clippy::cast_precision_loss)]
    fn visit_i64<E>(self, v: i64) -> Result<(), E> {
        self.number(v as f64);
        Ok(())
    }

    #[allow(clippy::cast_precision_loss)]
    fn visit_u64<E>(self, v: u64) -> Result<(), E> {
        self.number(v as f64);
        Ok(())
    }

    fn visit_f64<E>(self, v: f64) -> Result<(), E> {
        self.number(v);
        Ok(())
    }

    fn visit_bool<E>(self, _: bool) -> Result<(), E> {
        Ok(())
    }

    fn visit_str<E>(self, _: &str) -> Result<(), E> {
        Ok(())
    }

    fn visit_unit<E>(self) -> Result<(), E> {
        Ok(())
    }

    fn visit_map<A: MapAccess<'de>>(mut self, mut map: A) -> Result<(), A::Error> {
        while let Some(key) = map.next_key::<String>()? {
            map.next_value_seed(self.child(&key))?;
        }
        Ok(())
    }

    fn visit_seq<A: SeqAccess<'de>>(mut self, mut seq: A) -> Result<(), A::Error> {
        let mut i = 0_usize;
        while seq.next_element_seed(self.child(&i.to_string()))?.is_some() {
            i += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod datadoc_tests;
#[cfg(test)]
mod docs_tests;
