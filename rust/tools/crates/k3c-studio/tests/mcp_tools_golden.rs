//! Tool-Liste gegen die aus Go erfasste Golden (V7). Die Golden entsteht nur durch `examples/capture_go_tools.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use k3c_studio::mcpsrv::{Config, Server};
use serde_json::{Map, Value, json};

#[path = "support/mcp_client.rs"]
mod mcp_client;

/// Golden-Namen, die erst spätere Sessions von RU70 bringen; am Ende leer.
const PENDING_TOOLS: &[&str] = &[
    "assets_create",
    "assets_delete",
    "assets_get",
    "assets_guide",
    "assets_hit_add",
    "assets_hit_drop",
    "assets_hits",
    "assets_import",
    "assets_list",
    "assets_preview",
    "assets_render",
    "assets_research",
    "assets_review",
    "assets_set",
    "assets_style_get",
    "assets_style_set",
    "hub_check",
    "hub_get",
    "hub_save",
    "level_generate",
    "replay_run",
    "sim_run",
    "sim_test",
    "world_check",
    "world_chunks_set",
    "world_create",
    "world_delete",
    "world_get",
    "world_list",
    "world_override_set",
    "world_set",
    "world_story_set",
];

/// Schema auf das, was gleich sein muss: je Eigenschaft Typ (ohne `null`), Beschreibung, `enum`, `items.type`; dazu die
/// sortierten Pflichtfelder.
fn schema(raw: &Value) -> Value {
    let types = |t: &Value| match t {
        Value::Array(list) => {
            let mut left: Vec<&Value> = list.iter().filter(|v| **v != json!("null")).collect();
            if left.len() == 1 {
                left.remove(0).clone()
            } else {
                json!(left)
            }
        }
        other => other.clone(),
    };
    let mut properties = Map::new();
    for (name, property) in raw["properties"].as_object().into_iter().flatten() {
        let mut kept = Map::new();
        if let Some(t) = property.get("type") {
            kept.insert("type".to_owned(), types(t));
        }
        for key in ["description", "enum"] {
            if let Some(v) = property.get(key) {
                kept.insert(key.to_owned(), v.clone());
            }
        }
        if let Some(t) = property.get("items").and_then(|items| items.get("type")) {
            kept.insert("items".to_owned(), json!({ "type": types(t) }));
        }
        properties.insert(name.clone(), Value::Object(kept));
    }
    let mut required: Vec<&str> = raw["required"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    required.sort_unstable();
    json!({ "properties": properties, "required": required })
}

fn by_name(tools: &[Value]) -> BTreeMap<String, Value> {
    tools
        .iter()
        .map(|tool| {
            let normalized = json!({
                "description": tool["description"],
                "annotations": tool["annotations"],
                "inputSchema": schema(&tool["inputSchema"]),
            });
            (tool["name"].as_str().unwrap().to_owned(), normalized)
        })
        .collect()
}

fn golden() -> BTreeMap<String, Value> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/go-tools.json");
    let list: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    by_name(&list)
}

async fn rust_tools() -> BTreeMap<String, Value> {
    let server = Server::new(Config {
        root: std::path::PathBuf::new(),
        port: 0,
        version: "test".to_owned(),
        log: k3c_kit::logger::Logger::discard(),
        usage: None,
    });
    server.start().await.unwrap();
    let tools = mcp_client::Client::connect(server.url()).await.tools().await;
    server.stop().await.unwrap();
    by_name(&tools)
}

#[tokio::test(flavor = "multi_thread")]
async fn tools_wie_go() {
    let golden = golden();
    let rust = rust_tools().await;
    assert_eq!(golden.len(), 35, "Golden: {:?}", golden.keys());
    for (name, tool) in &rust {
        assert_eq!(Some(tool), golden.get(name), "{name} weicht von der Go-Fassung ab");
    }
    let pending: BTreeSet<&str> = PENDING_TOOLS.iter().copied().collect();
    assert_eq!(pending.len(), PENDING_TOOLS.len(), "Name doppelt in PENDING_TOOLS");
    let done: Vec<&&str> = pending.iter().filter(|name| rust.contains_key(**name)).collect();
    assert!(done.is_empty(), "schon vorhanden: {done:?}");
    let both: BTreeSet<&str> = rust.keys().map(String::as_str).chain(pending).collect();
    let wanted: BTreeSet<&str> = golden.keys().map(String::as_str).collect();
    assert_eq!(both, wanted);
}

/// Die `Name: "…"` der Tools in den Go-Quellen sind die Golden-Namen, solange es die Quellen gibt. Prompts
/// (`"asset-" + …`) passen nicht auf das Muster.
#[test]
fn golden_wie_go_quellen() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../tools/k3c-studio/internal/mcpsrv");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        eprintln!("übersprungen: {} fehlt", dir.display());
        return;
    };
    let pattern = regex::Regex::new(r#"(?m)^\s*Name:\s*"([a-z0-9_]+)""#).unwrap();
    let mut names = BTreeSet::new();
    for entry in entries {
        let path = entry.unwrap().path();
        let file = path.file_name().unwrap().to_string_lossy().into_owned();
        if !file.ends_with(".go") || file.ends_with("_test.go") {
            continue;
        }
        let source = std::fs::read_to_string(&path).unwrap();
        names.extend(pattern.captures_iter(&source).map(|found| found[1].to_owned()));
    }
    assert_eq!(names, golden().into_keys().collect::<BTreeSet<_>>());
}
