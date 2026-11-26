//! Welten-Editor als MCP-Tools (Port von `tools_world.go`, B-954): `data/worlds/<id>.json` des Checkouts lesen, anlegen,
//! ändern und prüfen. Dieselbe Logik wie die Seite Welten (`world`); geschrieben wird atomar und format-stabil, nach
//! Prüfung wie beim Laden im Spiel (`k3c-game sim world`). Es läuft keine Simulation. Jedes Tool nimmt `checkout`.

use std::collections::BTreeMap;

use k3c_kit::mcp::or_dash;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;
use serde_json::{Map, Value};

use super::tools_world_story::{self, StoryArgs};
use crate::kitx::go_marshal;
use crate::world::{Editor, MISSING_NOTE, Warnings};

#[cfg(test)]
#[path = "tools_world_skins_tests.rs"]
mod skins_tests;
#[cfg(test)]
#[path = "tools_world_tests.rs"]
pub(super) mod tests;

/// Führt einen blockierenden Aufruf des Editors außerhalb der Laufzeit-Threads aus. Eine Panik läuft weiter, damit das
/// Gerüst sie wie bei jedem Tool meldet.
pub(super) async fn blocking<T: Send + 'static>(
    run: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    match tokio::task::spawn_blocking(run).await {
        Ok(result) => result,
        Err(e) if e.is_panic() => std::panic::resume_unwind(e.into_panic()),
        Err(e) => Err(e.to_string()),
    }
}

/// `null` ist wie bei Gos Slices die leere Liste.
fn null_list<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error> {
    Ok(Option::<Vec<String>>::deserialize(d)?.unwrap_or_default())
}

/// Argumente von `world_get` und `world_check`.
#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct IdIn {
    #[schemars(description = "Welt-ID (Dateiname ohne .json, a-z, 0-9, -)")]
    pub(super) id: String,
}

/// Argumente von `world_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct CreateIn {
    #[schemars(description = "neue Welt-ID (a-z, 0-9, -, höchstens 32 Zeichen); die Datei darf noch nicht existieren")]
    id: String,
    #[schemars(description = "Anzeigename der Welt")]
    name: String,
    #[serde(default)]
    #[schemars(description = "Eltern-Welt, deren Werte geerbt werden (leer = Basis data/)")]
    extends: String,
    #[serde(default)]
    #[schemars(
        with = "StoryArgs",
        description = "Geschichte der neuen Welt wie bei world_story_set (lexicon, start, etappen, end, bottles, islands: {island1: {arrival, boss, done}}); fehlt sie, entsteht die Welt ohne story"
    )]
    story: Option<StoryArgs>,
}

/// Argumente von `world_delete`.
#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct DeleteIn {
    #[schemars(description = "Welt-ID der zu löschenden Welt")]
    id: String,
    #[serde(default)]
    #[schemars(description = "muss true sein: die Datei wird endgültig gelöscht")]
    confirm: bool,
}

/// Argumente von `world_set`.
#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct SetIn {
    #[schemars(description = "Welt-ID")]
    id: String,
    #[schemars(
        description = "Felder: name, gewaesser, notes, extends, kind, combat {mode, groupSize}, skins {basisAsset: ersatzAsset} (null entfernt); islands: Liste von Inseln {id, name?, etappe? (Ganzzahl ≥ 1, lückenlos ab 1, nicht fallend; null entfernt), depths? | biomes?, depthScaling? {hp, damage, speed}, plannedChunks?, chunkCounts?, endBoss?, nature?, natureByDepth?}. Bekannte Insel-id: nur genannte Felder ändern; neue id: anhängen (Standard: Tiefen 0-4, Skalierung der Standard-Welt). biomes = Biom-IDs statt Tiefen (forest, cave, mine, ironhold, crystal)."
    )]
    patch: Map<String, Value>,
}

/// Argumente von `world_override_set`.
#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct OverrideIn {
    #[schemars(description = "Welt-ID")]
    id: String,
    #[schemars(
        description = "Basis-Datei ohne .json, z. B. economy (Namen nennt die Fehlermeldung bei einem unbekannten Feld)"
    )]
    file: String,
    #[serde(deserialize_with = "null_list")]
    #[schemars(description = "Pfad zum Feld in der Datei, z. B. [purse, startGold]")]
    path: Vec<String>,
    #[serde(default)]
    #[schemars(description = "neuer Wert (JSON); fehlt er oder ist null, erbt das Feld wieder")]
    value: Value,
}

/// Argumente von `world_chunks_set`.
#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct ChunksIn {
    #[schemars(description = "Welt-ID")]
    id: String,
    #[schemars(description = "Insel-ID")]
    island: String,
    #[schemars(description = "Tiefe der Stufe (Wert aus depths der Insel, nicht der Index)")]
    depth: i64,
    #[serde(default)]
    #[schemars(
        description = "geplante Chunks dieser Tiefe (plannedChunks, docs/rules/geplante-chunks.md), z. B. [{slot: -7, kind: harbor}]; [] entfernt, fehlt = unverändert"
    )]
    chunks: Option<Vec<Value>>,
    #[serde(default)]
    #[schemars(
        description = "Chunk-Anzahl dieser Tiefe (chunkCounts: 16, 24, 40, 64, 100, 160); 0 entfernt, fehlt = unverändert"
    )]
    count: Option<i64>,
}

/// Die Antwort eines Welt-Tools: der Erfolgssatz mit den Warnungen der Prüfung.
pub(super) fn done(ok: String, warnings: Warnings) -> String {
    match warnings {
        Some(warnings) if !warnings.is_empty() => format!("{ok}\nWarnungen:\n- {}", warnings.join("\n- ")),
        _ => ok,
    }
}

/// `world_list`: eine Zeile je Welt.
pub(super) fn list(editor: &Editor) -> Result<String, String> {
    let lines: Vec<String> = editor
        .list()
        .into_iter()
        .map(|w| {
            if w.missing {
                return format!("{} · {MISSING_NOTE}", w.id);
            }
            let d = editor.load(&w.id);
            let extends = or_dash(&d.extends);
            format!("{} · {} · extends {extends} · {} Inseln", w.id, w.name, d.islands.len())
        })
        .collect();
    Ok(lines.join("\n"))
}

/// Schlüssel, die Zahlen sind, aufsteigend (Tiefen einer Planung, Etappen der Geschichte); andere entfallen.
pub(super) fn number_keys<V>(map: &BTreeMap<String, V>) -> Vec<i64> {
    let mut keys: Vec<i64> = map.keys().filter_map(|k| k.parse().ok()).collect();
    keys.sort_unstable();
    keys
}

fn planned_lines(planned: &BTreeMap<String, Box<RawValue>>) -> String {
    number_keys(planned)
        .into_iter()
        .map(|depth| {
            let raw = planned.get(&depth.to_string()).map_or("", |v| v.get());
            format!("    plannedChunks[{depth}]: {}\n", raw.trim())
        })
        .collect()
}

/// `world_get`: Kopf, Inseln (Tiefe → Biom, Planung), Arten, Geschichte und der Text der Datei.
pub(super) fn get(editor: &Editor, id: &str) -> Result<String, String> {
    let d = editor.load(id);
    if !d.error.is_empty() {
        return Err(d.error);
    }
    let raw = editor.raw(id)?;
    let extends = or_dash(&d.extends);
    let mut out = format!(
        "{} · {} · extends {extends} · {} Inseln\n",
        d.id,
        d.name,
        d.islands.len()
    );
    for island in &d.islands {
        let stage = island.etappe.map_or_else(String::new, |n| format!(" · Etappe {n}"));
        let depths: Vec<String> = island.depths.iter().flatten().map(i64::to_string).collect();
        let (depths, biomes) = (depths.join(" "), island.biomes.join(", "));
        out += &format!(
            "- {} · {} · Tiefen [{depths}] → {biomes}{stage}\n",
            island.id, island.name
        );
        out += &planned_lines(&island.planned);
    }
    let picks = editor.picks(id)?;
    let pick = |key: &str| or_dash(picks.get(key).map_or("", String::as_str)).to_owned();
    out += &format!(
        "Arten: weather {} · mount {} · hermit {} (Katalog-Dateien, „–“ = keins)\n",
        pick("weather"),
        pick("mount"),
        pick("hermit")
    );
    out += &tools_world_story::story_text(&d);
    out += "Datei:\n";
    out += raw.trim_end_matches('\n');
    Ok(out)
}

/// `world_check`: Warnungen der Prüfung und Hinweise zur Geschichte; ändert nichts.
pub(super) fn check(editor: &Editor, id: &str) -> Result<String, String> {
    let warnings = editor.check(id)?;
    let text = done(format!("Welt {id} ist gültig"), warnings);
    Ok(text + &tools_world_story::story_hints(&editor.load(id)))
}

/// `world_create`: ohne Geschichte endet die Antwort mit dem Hinweis auf `world_story_set`.
pub(super) fn create(editor: &Editor, input: CreateIn) -> Result<String, String> {
    let story = input.story.map(StoryArgs::patch);
    let warnings = editor.create_with_story(&input.id, &input.name, &input.extends, story.as_ref())?;
    let id = &input.id;
    let mut text = done(format!("Welt {id} angelegt (data/worlds/{id}.json)"), warnings);
    if editor.load(id).story.is_none() {
        text.push('\n');
        text.push_str(tools_world_story::STORY_MISSING);
    }
    Ok(text)
}

/// `world_delete`: nur mit `confirm=true`.
pub(super) fn delete(editor: &Editor, input: &DeleteIn) -> Result<String, String> {
    let id = &input.id;
    if !input.confirm {
        return Err(format!(
            "world_delete braucht confirm=true (die Datei der Welt {id} wird endgültig gelöscht)"
        ));
    }
    let warnings = editor.delete(id)?;
    Ok(done(format!("Welt {id} gelöscht (data/worlds/{id}.json)"), warnings))
}

/// `world_set`.
pub(super) fn set(editor: &Editor, input: SetIn) -> Result<String, String> {
    let warnings = editor.set(&input.id, &go_marshal(&Value::Object(input.patch)))?;
    Ok(done(format!("Welt {} gespeichert", input.id), warnings))
}

/// `world_override_set`.
pub(super) fn override_set(editor: &Editor, input: &OverrideIn) -> Result<String, String> {
    let value = go_marshal(&input.value);
    let view = editor.set_override(&input.id, &input.file, &input.path, &value);
    if !view.error.is_empty() {
        return Err(view.error);
    }
    let (file, path, id) = (&input.file, input.path.join("."), &input.id);
    let ok = format!("Override {file} › {path} der Welt {id} gesetzt");
    Ok(done(ok, Some(view.warnings)))
}

/// `world_chunks_set`.
pub(super) fn chunks_set(editor: &Editor, input: ChunksIn) -> Result<String, String> {
    let chunks = input.chunks.map(|list| go_marshal(&Value::Array(list)));
    let (id, island, depth) = (&input.id, &input.island, input.depth);
    let warnings = editor.set_chunks(id, island, depth, chunks.as_deref(), input.count)?;
    let ok = format!("Chunks der Welt {id}, Insel {island}, Tiefe {depth} gespeichert");
    Ok(done(ok, warnings))
}
