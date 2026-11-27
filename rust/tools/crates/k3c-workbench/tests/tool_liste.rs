//! Tool-Liste gegen den Schnappschuss der Go-Workbench (`tests/data/go-tools.json`): was Rust anbietet, ist gleich, der
//! Rest steht in [`AUSSTEHEND`].
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use k3c_workbench::mcp::{Config, Server};
use serde_json::{Map, Value, json};

#[path = "../src/mcp/test_client.rs"]
mod test_client;

/// Tools, die erst spätere Sessions von RU69 bringen; am Ende leer.
const AUSSTEHEND: &[&str] = &[
    "gh_status",
    "git_commit",
    "plan_check",
    "plan_create",
    "plan_delete",
    "plan_digest",
    "plan_get",
    "plan_index",
    "plan_list",
    "plan_next",
    "plan_queue",
    "plan_section",
    "plan_sections",
    "plan_set",
    "plan_waves",
    "playtest_findings_set",
    "playtest_frame",
    "playtest_get",
    "playtest_list",
    "playtest_tickets_create",
    "workbench_resources",
];

fn snapshot_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/go-tools.json")
}

/// `type` ohne `null`; bleibt nur eines, als Text.
fn types(raw: &Value) -> Value {
    match raw {
        Value::Array(list) => {
            let mut left: Vec<&Value> = list.iter().filter(|v| **v != json!("null")).collect();
            if left.len() == 1 {
                left.remove(0).clone()
            } else {
                json!(left)
            }
        }
        other => other.clone(),
    }
}

/// Auf das, was gleich sein muss: Name, Beschreibung, Annotations, je Eigenschaft Typ, Beschreibung und `enum`, dazu die
/// sortierten Pflichtfelder. Zweimal angewandt ändert sich nichts.
fn normalize(tool: &Value) -> Value {
    let schema = &tool["inputSchema"];
    let mut properties = Map::new();
    for (name, property) in schema["properties"].as_object().into_iter().flatten() {
        let mut kept = Map::new();
        if let Some(t) = property.get("type") {
            kept.insert("type".to_owned(), types(t));
        }
        for key in ["description", "enum"] {
            if let Some(v) = property.get(key) {
                kept.insert(key.to_owned(), v.clone());
            }
        }
        properties.insert(name.clone(), Value::Object(kept));
    }
    let mut required: Vec<&str> = schema["required"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    required.sort_unstable();
    json!({
        "name": tool["name"],
        "description": tool["description"],
        "annotations": tool["annotations"],
        "inputSchema": { "properties": properties, "required": required },
    })
}

fn by_name(tools: &[Value]) -> BTreeMap<String, Value> {
    tools
        .iter()
        .map(|tool| (tool["name"].as_str().unwrap().to_owned(), normalize(tool)))
        .collect()
}

fn snapshot() -> BTreeMap<String, Value> {
    let list: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(snapshot_path()).unwrap()).unwrap();
    by_name(&list)
}

async fn rust_tools() -> BTreeMap<String, Value> {
    let server = Server::new(Config::default());
    server.start().await.unwrap();
    let tools = test_client::Client::connect(server.url()).await.tools().await;
    server.stop().await.unwrap();
    by_name(&tools)
}

#[tokio::test(flavor = "multi_thread")]
async fn tools_wie_go() {
    let go = snapshot();
    let rust = rust_tools().await;
    assert_eq!(go.len(), 45, "Schnappschuss: {:?}", go.keys());
    assert_eq!(AUSSTEHEND.len(), 21);
    for (name, tool) in &rust {
        assert_eq!(Some(tool), go.get(name), "{name} weicht von der Go-Fassung ab");
    }
    let pending: BTreeSet<&str> = AUSSTEHEND.iter().copied().collect();
    assert_eq!(pending.len(), AUSSTEHEND.len(), "Name doppelt in AUSSTEHEND");
    let done: Vec<&&str> = pending.iter().filter(|name| rust.contains_key(**name)).collect();
    assert!(done.is_empty(), "schon vorhanden: {done:?}");
    let both: BTreeSet<&str> = rust.keys().map(String::as_str).chain(pending).collect();
    let wanted: BTreeSet<&str> = go.keys().map(String::as_str).collect();
    assert_eq!(both, wanted);
}

/// Erneuert den Schnappschuss aus der laufenden Go-Workbench (nur `initialize` und `tools/list`, nur lesend):
/// `UPDATE_GO_TOOLS=1 cargo test -p k3c-workbench --test tool_liste -- --ignored go_tools_schnappschuss_erneuern`
#[tokio::test]
#[ignore = "braucht die laufende Go-Workbench und UPDATE_GO_TOOLS=1"]
async fn go_tools_schnappschuss_erneuern() {
    if std::env::var("UPDATE_GO_TOOLS").as_deref() != Ok("1") {
        eprintln!("🚫 übersprungen: UPDATE_GO_TOOLS=1 fehlt");
        return;
    }
    let url = "http://127.0.0.1:5180/mcp".to_owned();
    let tools = test_client::Client::connect(url).await.tools().await;
    let list: Vec<Value> = by_name(&tools).into_values().collect();
    let text = serde_json::to_string_pretty(&list).unwrap() + "\n";
    std::fs::write(snapshot_path(), text).unwrap();
    eprintln!("💾 {} Tools in {}", list.len(), snapshot_path().display());
}
