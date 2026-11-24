//! Port von `engine/simcli/world.go` (RU33.2): die Engine-Seite des Welten-Editors. Es läuft keine Simulation, nur der
//! Level-Generator und die Prüfung der Welt-Werte; die Welten stehen im Daten-Ordner (`worlds/`).
//!
//! ```text
//! {"op":"level","world","island","stage","seed"}  Level der Insel-Stufe: Layout, planned, warnings, Natur
//! {"op":"values","world"}                         Basis der Welt je Datei, extends, overrides, children
//! {"op":"check","world","draft"}                  Entwurf der Welt-Datei prüfen, mit allen Nachfolgern: warnings
//! ```

use std::path::{Path, PathBuf};
use std::sync::Arc;

use k3c_data::biome::{Biome, Range, Trait};
use k3c_data::{Data, Fehler, Source, load_all};
use k3c_level::level::generate_with;
use k3c_level::planned_gen::Options as LevelOptions;
use k3c_level::rules::Rules;
use k3c_level::validate::validate_with;
use k3c_sim::island::data::biome_for_depth;
use k3c_sim::island::{WorldDef, load_world, parse_world};
use k3c_sim::world::level_sites;
use k3c_sim::world::values::{resolve_world_files, resolve_world_values};
use serde::Serialize;

use super::RunError;
use super::gojson::{compact, raw_entries, raw_field, to_string};
use super::gotext::quote;
use super::input_world::WorldInput;

/// Die Länge des Seeds wie `room.MaxSeedRunes`.
const MAX_WORLD_SEED_RUNES: usize = 64;

fn failed(msg: impl Into<String>) -> RunError {
    RunError::Failed(msg.into())
}

/// Daten-Ordner, geladene Daten und Regeln eines Aufrufs.
struct Env {
    src: Source,
    dir: PathBuf,
    data: Data,
    rules: Arc<Rules>,
}

impl Env {
    fn load(src: &Source, name: &str) -> Result<Env, RunError> {
        let Source::Dir(dir) = src else {
            return Err(failed(format!("sim {name} braucht einen Daten-Ordner (--data)")));
        };
        let data = load_all(src).map_err(|e| failed(e.to_string()))?;
        let hub = src.read("hub.json").map_err(|e| failed(e.to_string()))?;
        let rules = Arc::new(Rules::from_data(&data, &hub).map_err(|e| failed(e.to_string()))?);
        Ok(Env {
            src: src.clone(),
            dir: dir.clone(),
            data,
            rules,
        })
    }

    fn read(&self, name: &str) -> Result<String, Fehler> {
        self.src.read(name)
    }

    fn world(&self, id: &str) -> Result<WorldDef, String> {
        load_world(&self.src, id, &self.data, &self.rules)
    }
}

/// Führt `sim world` aus und liefert den Bericht als JSON-Text (Go: `report`).
pub fn run(src: &Source, input: &WorldInput) -> Result<String, RunError> {
    if !["level", "values", "check"].contains(&input.op.as_str()) {
        return Err(RunError::Usage(format!(
            "unbekannte op {} (bekannt: level, values, check)",
            quote(&input.op)
        )));
    }
    let env = Env::load(src, "world")?;
    match input.op.as_str() {
        "level" => level(&env, input),
        "values" => values(&env, &input.world),
        _ => check(&env, &input.world, &input.draft),
    }
}

/// Eigenschaft des Bioms mit ihrem Schlüssel (Go: `natureTrait`).
#[derive(Serialize)]
struct NatureTrait<'a> {
    key: &'a str,
    #[serde(flatten)]
    value: &'a Trait,
}

#[derive(Serialize)]
struct NatureOption<'a> {
    key: &'a str,
    #[serde(flatten)]
    value: &'a Range,
}

#[derive(Serialize)]
struct NatureWeights<'a> {
    key: &'a str,
    name: &'a str,
    kind: &'a str,
    options: Vec<NatureOption<'a>>,
}

/// Go: `natureOf`: Eigenschaften und Gewichts-Gruppen des Bioms in Datei-Reihenfolge.
fn nature_of(b: &Biome) -> (Vec<NatureTrait<'_>>, Vec<NatureWeights<'_>>) {
    let traits = b
        .traits
        .0
        .iter()
        .map(|e| NatureTrait {
            key: &e.key,
            value: &e.value,
        })
        .collect();
    let groups = b
        .weights
        .0
        .iter()
        .map(|g| NatureWeights {
            key: &g.key,
            name: &g.value.name,
            kind: &g.value.kind,
            options: g
                .value
                .options
                .0
                .iter()
                .map(|o| NatureOption {
                    key: &o.key,
                    value: &o.value,
                })
                .collect(),
        })
        .collect();
    (traits, groups)
}

/// Go: `worldLevel`: das Level wie im Spiel plus die geplanten Chunks der Stufe und die Warnungen der Prüfung.
fn level(env: &Env, input: &WorldInput) -> Result<String, RunError> {
    let n = input.seed.chars().count();
    if n == 0 || n > MAX_WORLD_SEED_RUNES {
        return Err(failed(format!("der Seed braucht 1 bis {MAX_WORLD_SEED_RUNES} Zeichen")));
    }
    let def = env.world(&input.world).map_err(failed)?;
    let isl = def
        .islands
        .iter()
        .find(|i| i.id == input.island)
        .ok_or_else(|| failed(format!("die Insel {} gibt es nicht", quote(&input.island))))?;
    let depth = usize::try_from(input.stage)
        .ok()
        .and_then(|s| isl.depths.get(s))
        .copied()
        .ok_or_else(|| {
            failed(format!(
                "die Stufe {} gibt es auf der Insel {} nicht",
                input.stage,
                quote(&input.island)
            ))
        })?;
    let b = biome_for_depth(&env.data.biomes, depth).ok_or_else(|| failed(format!("kein Biom für Tiefe {depth}")))?;
    let key = depth.to_string();
    let base = |name: &str| env.read(name);
    let lookup = |w: &str| env.world(w);
    let vals = resolve_world_values(&base, &lookup, &input.world).map_err(failed)?;
    // der Bau liegt wie im Spiel in der tiefsten Stufe der Insel (B-981)
    let lair = depth == isl.lair_depth();
    let opts = LevelOptions {
        planned: isl.planned_chunks.get(&key).cloned().unwrap_or_default(),
        chunk_count: isl.chunk_counts.get(&key).copied().unwrap_or(0),
        nature: isl.nature_at(&env.data.biomes, depth),
        hub_sites: Some(level_sites(&vals.hub)),
        lair: Some(lair),
    };
    let layout = generate_with(b, &input.seed, &opts, &env.rules)
        .map_err(|e| failed(format!("das Level ist nicht erzeugbar: {e}")))?;
    let warnings = validate_with(&layout, b, &opts, &env.rules);
    let (traits, groups) = nature_of(b);
    let head = to_string(&layout);
    let head = head.strip_suffix('}').unwrap_or(&head);
    Ok(format!(
        "{head},\"planned\":{},\"warnings\":{},\"nature\":{},\"natureFixed\":{},\"natureTraits\":{},\"natureWeights\":{}}}",
        to_string(&opts.planned),
        to_string(&warnings),
        to_string(&layout.nature),
        to_string(&opts.nature),
        to_string(&traits),
        to_string(&groups)
    ))
}

/// Go: `worldValues`: die Basis der Welt je Basis-Datei (ohne die eigenen Overrides), ihre Overrides und die Nachfolger.
fn values(env: &Env, id: &str) -> Result<String, RunError> {
    let def = env.world(id).map_err(failed)?;
    let base = |name: &str| env.read(name);
    let lookup = |w: &str| env.world(w);
    let files = resolve_world_files(&base, &lookup, &def.extends).map_err(failed)?;
    let mut base_json = Vec::new();
    for (k, raw) in &files {
        base_json.push(format!("{}:{}", to_string(k), compact(raw).map_err(failed)?));
    }
    // Go gibt die Overrides als `json.RawMessage` der Datei zurück: Text und Reihenfolge bleiben
    let text = env
        .read(&format!("worlds/{id}.json"))
        .map_err(|e| failed(e.to_string()))?;
    let mut over: Vec<(String, String)> = Vec::new();
    for (k, raw) in raw_field(&text, "overrides").map(raw_entries).unwrap_or_default() {
        over.push((k, compact(raw).map_err(failed)?));
    }
    over.sort();
    let over_json: Vec<String> = over.iter().map(|(k, v)| format!("{}:{v}", to_string(k))).collect();
    Ok(format!(
        "{{\"extends\":{},\"children\":{},\"base\":{{{}}},\"overrides\":{{{}}}}}",
        to_string(&def.extends),
        to_string(&descendants(&env.dir.join("worlds"), id)),
        base_json.join(","),
        over_json.join(",")
    ))
}

/// Go: `worldCheck`: prüft den Entwurf der Welt-Datei wie der Server: die Welt und jeden Nachfolger. Ein Nachfolger
/// blockiert nur, wenn der Entwurf ihn neu oder anders ungültig macht; war er schon gleich ungültig oder lädt er nicht,
/// kommt eine Warnung zurück.
fn check(env: &Env, id: &str, draft: &str) -> Result<String, RunError> {
    let def = parse_world(draft, id, &env.data, &env.rules).map_err(failed)?;
    let base = |name: &str| env.read(name);
    let disk = |w: &str| env.world(w);
    let with_draft = |w: &str| if w == id { Ok(def.clone()) } else { env.world(w) };
    resolve_world_values(&base, &with_draft, id).map_err(failed)?;
    let mut warnings: Vec<String> = Vec::new();
    for kid in descendants(&env.dir.join("worlds"), id) {
        if let Err(bad) = env.world(&kid) {
            warnings.push(format!(
                "Welt {kid} erbt von {id}, ist aber ungültig und nicht geprüft: {bad}"
            ));
            continue;
        }
        let Err(err) = resolve_world_values(&base, &with_draft, &kid) else {
            continue;
        };
        if resolve_world_values(&base, &disk, &kid).is_err_and(|was| was == err) {
            warnings.push(format!("Welt {kid} war schon vorher ungültig: {err}"));
            continue;
        }
        return Err(failed(err));
    }
    Ok(format!("{{\"warnings\":{}}}", to_string(&warnings)))
}

/// Go: `descendants`: alle Welten unter `worlds`, die direkt oder über Zwischenwelten von `id` erben, sortiert.
fn descendants(worlds: &Path, id: &str) -> Vec<String> {
    let mut extends: Vec<(String, String)> = Vec::new();
    let mut names: Vec<_> = std::fs::read_dir(worlds)
        .map(|d| {
            d.flatten()
                .filter(|e| e.path().is_file())
                .map(|e| e.file_name())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    for name in names {
        let Some(w) = name.to_string_lossy().strip_suffix(".json").map(ToString::to_string) else {
            continue;
        };
        let parent = std::fs::read_to_string(worlds.join(&name))
            .ok()
            .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
            .map(|v| v["extends"].as_str().unwrap_or_default().to_string());
        if let Some(p) = parent {
            extends.push((w, p));
        }
    }
    let mut seen = vec![id.to_string()];
    loop {
        let before = seen.len();
        for (w, parent) in &extends {
            if !seen.contains(w) && seen.contains(parent) {
                seen.push(w.clone());
            }
        }
        if seen.len() == before {
            break;
        }
    }
    let mut out: Vec<String> = seen.into_iter().filter(|w| w != id).collect();
    out.sort();
    out
}
