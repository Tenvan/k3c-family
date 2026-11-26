//! Backend des Welten-Editors (Port von `internal/world`): Welten unter `data/worlds/` auflisten und lesen, das Level
//! einer Insel-Stufe für die Chunk-Vorschau holen und Werte einer Welt übersteuern (docs/rules/welten.md § 1). Level,
//! Basis-Werte und die Prüfung rechnet die Spiel-EXE (`k3c-game sim world`, `level.rs`). Geschrieben wird nur die
//! Welt-Datei, format-stabil und atomar; es läuft keine Simulation. Die Aufrufe blockieren (Aufrufer in tokio nutzen
//! `spawn_blocking`).

mod catalog;
mod delete;
mod edit;
mod edit_island;
mod endboss_skins;
mod inline;
mod kind;
mod level;
mod nature;
mod override_reject;
// `override` ist ein reserviertes Wort.
#[path = "override.rs"]
mod overrides;
mod patch;
mod rename;
mod story;
mod values;
mod watch;

#[cfg(test)]
mod edit_endboss_skins_tests;
#[cfg(test)]
mod edit_override_lists_tests;
#[cfg(test)]
mod etappe_tests;
#[cfg(test)]
mod exe_tests;
#[cfg(test)]
mod world_tests;

use std::collections::BTreeMap;
use std::path::PathBuf;

use k3c_kit::logger::Logger;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::value::RawValue;

pub use catalog::PicksView;
pub use edit::{CreateView, Warnings};
pub use endboss_skins::EndBossOption;
pub use level::{Engine, LevelView, exe};
pub use story::{IslandStory, Story, StoryPatch, StoryResult};
pub use values::{ValueEntry, ValuesView};
pub use watch::{Changed, Watcher, changed_ids, snapshot, watch};

use values::object_items;

/// Ordner der Welt-Dateien unter der Repo-Wurzel.
const WORLDS_DIR: &str = "data/worlds";
/// Die Welt, die das Spiel ohne Wahl lädt; sie steht in der Liste oben.
const STANDARD_ID: &str = "standard";
/// Hinweis an einer festen Welt ohne Datei.
pub const MISSING_NOTE: &str = "noch nicht umgesetzt";

/// Welt-ID nach dem Muster `^[a-z0-9][a-z0-9-]{0,31}$` (gleich dem Dateinamen ohne `.json`).
fn valid_id(id: &str) -> bool {
    let plain = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit();
    id.len() <= 32 && id.bytes().next().is_some_and(plain) && id.bytes().all(|b| plain(b) || b == b'-')
}

/// Lehnt eine ID außerhalb des Musters ab, bevor ein Pfad entsteht.
fn check_id(id: &str) -> Result<(), String> {
    if valid_id(id) {
        return Ok(());
    }
    Err(format!("ungültige Welt-ID {id:?}"))
}

/// `null` lässt wie in Go den Nullwert stehen.
fn null_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

/// Eine Welt der Liste.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Info {
    pub id: String,
    pub name: String,
    /// Feste Welt (`data/paper.json` › weltbilder) ohne Datei; sie lässt sich nicht öffnen (B-1168).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub missing: bool,
    /// Art der Welt ("spinoff" | "new", B-1217); leer bei fehlender Datei.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub kind: String,
    /// Untertitel der Welt in der Liste.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub gewaesser: String,
}

/// Eine Insel mit dem Biom je Tiefe (Reihenfolge wie `depths`), ihrer Planung und Chunk-Anzahl wie in der Datei.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Island {
    #[serde(deserialize_with = "null_default")]
    pub id: String,
    #[serde(deserialize_with = "null_default")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etappe: Option<i64>,
    /// ID des Endbosses der Insel; leer = Endboss der tiefsten Stufe.
    #[serde(
        rename = "endBoss",
        skip_serializing_if = "String::is_empty",
        deserialize_with = "null_default"
    )]
    pub end_boss: String,
    /// `None` wird wie Gos nil-Liste zu `null`.
    pub depths: Option<Vec<i64>>,
    #[serde(deserialize_with = "null_default")]
    pub biomes: Vec<String>,
    #[serde(rename = "plannedChunks", deserialize_with = "null_default")]
    pub planned: BTreeMap<String, Box<RawValue>>,
    #[serde(rename = "chunkCounts", deserialize_with = "null_default")]
    pub chunk_counts: BTreeMap<String, i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<IslandStory>,
}

/// Eine Welt mit Kopf und Inseln; `error` nennt, warum sie nicht lesbar ist (dann ohne Inseln).
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    pub id: String,
    pub name: String,
    pub notes: String,
    pub extends: String,
    /// "spinoff" (Ableger oder Episode) oder "new"; ohne Feld in der Datei spinoff mit `extends`, sonst new.
    pub kind: String,
    pub gewaesser: String,
    pub islands: Vec<Island>,
    /// Basis-Asset → Ersatz-Asset je `<gruppe>/<asset>`.
    pub skins: Option<BTreeMap<String, String>>,
    /// Wählbare Endbosse aus `data/bosses.json`.
    pub end_bosses: Option<Vec<EndBossOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,
    /// Palette und Portal-HP je Biom-ID der Inseln.
    pub biome_info: BTreeMap<String, BiomeInfo>,
    pub error: String,
}

/// Anzeige-Werte eines Bioms aus `data/biomes/<id>.json`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BiomeInfo {
    pub palette: BTreeMap<String, String>,
    pub portal_hp: i64,
}

/// Der Welten-Editor eines Checkouts.
#[derive(Clone)]
pub struct Editor {
    /// Repo-Wurzel.
    pub root: PathBuf,
    /// Engine-Seite (`k3c-game sim world`).
    pub engine: Engine,
    pub log: Logger,
}

/// Teil der Welt-Datei, den [`Editor::load`] liest.
#[derive(Default, Deserialize)]
#[serde(default)]
struct WorldFile {
    #[serde(deserialize_with = "null_default")]
    notes: String,
    #[serde(deserialize_with = "null_default")]
    extends: String,
    #[serde(deserialize_with = "null_default")]
    kind: String,
    #[serde(deserialize_with = "null_default")]
    gewaesser: String,
    #[serde(deserialize_with = "null_default")]
    islands: Vec<Island>,
    story: Option<Story>,
    skins: Option<BTreeMap<String, String>>,
}

impl Editor {
    /// Editor ohne Log.
    pub fn new(root: impl Into<PathBuf>, engine: Engine) -> Editor {
        Editor {
            root: root.into(),
            engine,
            log: Logger::discard(),
        }
    }

    /// Pfad der Datei einer Welt; `id` ist vorher geprüft.
    fn file(&self, id: &str) -> PathBuf {
        self.root.join("data").join("worlds").join(format!("{id}.json"))
    }

    /// Liest die Datei der Welt `id` nach Prüfung der ID.
    fn read(&self, id: &str) -> Result<Vec<u8>, String> {
        check_id(id)?;
        let path = self.file(id);
        std::fs::read(&path).map_err(|e| format!("{WORLDS_DIR}/{id}.json: open {}: {e}", path.display()))
    }

    /// Die festen Welten (Schlüssel von `data/paper.json` › weltbilder in Datei-Reihenfolge; ohne Datei mit `missing`),
    /// danach jede weitere Datei unter `data/worlds/` nach ID. Die Standard-Welt steht zuerst.
    pub fn list(&self) -> Vec<Info> {
        let mut files = self.world_files();
        let mut out = Vec::new();
        let fixed = self.fixed_ids();
        let others = fixed.iter().map(String::as_str).filter(|id| *id != STANDARD_ID);
        for id in std::iter::once(STANDARD_ID).chain(others) {
            out.push(files.remove(id).unwrap_or_else(|| Info {
                id: id.to_owned(),
                name: id.to_owned(),
                missing: true,
                ..Info::default()
            }));
        }
        out.extend(files.into_values());
        out
    }

    /// Je Welt-Datei ihr Listeneintrag, nach ID sortiert.
    fn world_files(&self) -> BTreeMap<String, Info> {
        let mut files = BTreeMap::new();
        let Ok(entries) = std::fs::read_dir(self.root.join("data").join("worlds")) else {
            return files;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let is_dir = entry.file_type().is_ok_and(|t| t.is_dir());
            let Some(id) = name.strip_suffix(".json").filter(|id| !is_dir && valid_id(id)) else {
                continue;
            };
            let raw = std::fs::read(self.file(id)).unwrap_or_default();
            let info = Info {
                id: id.to_owned(),
                name: name_of(&raw, id),
                missing: false,
                kind: kind_of_file(&raw),
                gewaesser: text_field(&raw, "gewaesser"),
            };
            files.insert(id.to_owned(), info);
        }
        files
    }

    /// IDs der festen Welten aus `data/paper.json` › weltbilder (fehlt die Datei oder der Schlüssel: keine).
    fn fixed_ids(&self) -> Vec<String> {
        let Ok(raw) = std::fs::read_to_string(self.root.join("data").join("paper.json")) else {
            return Vec::new();
        };
        let worlds = object_items(&raw).into_iter().find(|(k, _)| k == "weltbilder");
        let Some((_, worlds)) = worlds else {
            return Vec::new();
        };
        let keys = object_items(worlds.get()).into_iter().map(|(k, _)| k);
        keys.filter(|k| valid_id(k)).collect()
    }

    /// Liest die Welt `id` mit ihren Inseln; das Biom je Tiefe kommt aus `data/biomes/*.json` (Feld `depth`).
    pub fn load(&self, id: &str) -> Detail {
        let mut view = Detail {
            id: id.to_owned(),
            ..Detail::default()
        };
        let raw = match self.read(id) {
            Ok(raw) => raw,
            Err(e) => {
                view.error = e;
                return view;
            }
        };
        let file = match serde_json::from_slice::<Option<WorldFile>>(&raw) {
            Ok(file) => file.unwrap_or_default(),
            Err(e) => {
                view.error = format!("Welt-Datei: {e}");
                return view;
            }
        };
        view.name = name_of(&raw, id);
        view.kind = kind_of(&file.kind, &file.extends);
        (view.notes, view.extends, view.gewaesser) = (file.notes, file.extends, file.gewaesser);
        view.story = file.story;
        view.skins = Some(file.skins.unwrap_or_default());
        view.end_bosses = Some(self.end_boss_options());
        self.add_islands(&mut view, file.islands);
        view
    }

    fn add_islands(&self, view: &mut Detail, islands: Vec<Island>) {
        let biomes = self.biomes_by_depth();
        for mut island in islands {
            island.biomes = Vec::new();
            for depth in island.depths.iter().flatten() {
                let biome = biomes.get(depth).cloned().unwrap_or_default();
                if !biome.is_empty() && !view.biome_info.contains_key(&biome) {
                    view.biome_info.insert(biome.clone(), self.biome_info(&biome));
                }
                island.biomes.push(biome);
            }
            view.islands.push(island);
        }
    }

    /// Je Tiefe die ID ihres Bioms (Dateiname unter `data/biomes/`).
    fn biomes_by_depth(&self) -> BTreeMap<i64, String> {
        #[derive(Deserialize)]
        struct Biome {
            depth: Option<i64>,
        }
        let mut files: Vec<PathBuf> = std::fs::read_dir(self.root.join("data").join("biomes"))
            .map(|dir| dir.flatten().map(|e| e.path()).collect())
            .unwrap_or_default();
        files.sort();
        let mut out = BTreeMap::new();
        for path in files {
            let name = path.file_name().map(|n| n.to_string_lossy().into_owned());
            let Some(id) = name.as_deref().and_then(|n| n.strip_suffix(".json")) else {
                continue;
            };
            let biome = std::fs::read(&path)
                .ok()
                .and_then(|raw| serde_json::from_slice::<Biome>(&raw).ok());
            if let Some(depth) = biome.and_then(|b| b.depth) {
                out.insert(depth, id.to_owned());
            }
        }
        out
    }

    /// Palette und Portal-HP des Bioms `id`; fehlt etwas, bleibt es leer bzw. 0.
    fn biome_info(&self, id: &str) -> BiomeInfo {
        #[derive(Default, Deserialize)]
        #[serde(default)]
        struct Portals {
            #[serde(deserialize_with = "null_default")]
            hp: i64,
        }
        #[derive(Default, Deserialize)]
        #[serde(default)]
        struct Biome {
            palette: Option<BTreeMap<String, String>>,
            #[serde(deserialize_with = "null_default")]
            portals: Portals,
        }
        let path = self.root.join("data").join("biomes").join(format!("{id}.json"));
        let biome = std::fs::read(path)
            .ok()
            .and_then(|raw| serde_json::from_slice::<Biome>(&raw).ok())
            .unwrap_or_default();
        BiomeInfo {
            palette: biome.palette.unwrap_or_default(),
            portal_hp: biome.portals.hp,
        }
    }
}

/// Textfeld der obersten Ebene; leer, wenn die Datei nicht lesbar ist oder das Feld kein Text ist.
fn text_field(raw: &[u8], key: &str) -> String {
    let value = serde_json::from_slice::<serde_json::Value>(raw).ok();
    let text = value.as_ref().and_then(|v| v.get(key)).and_then(|v| v.as_str());
    text.unwrap_or_default().to_owned()
}

fn name_of(raw: &[u8], id: &str) -> String {
    let name = text_field(raw, "name");
    if name.is_empty() { id.to_owned() } else { name }
}

/// Art der Welt wie `sim.worldKind`: ohne Angabe spinoff mit `extends`, sonst new.
fn kind_of(kind: &str, extends: &str) -> String {
    match (kind, extends) {
        ("", "") => "new",
        ("", _) => "spinoff",
        _ => kind,
    }
    .to_owned()
}

fn kind_of_file(raw: &[u8]) -> String {
    kind_of(&text_field(raw, "kind"), &text_field(raw, "extends"))
}
