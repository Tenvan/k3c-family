//! Port von `roundtrip_test.go` über echtes Streamable HTTP (Port 0).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use k3c_studio::mcpsrv::{Config, INSTRUCTIONS, NAME, Server};
use serde_json::{Value, json};

#[path = "support/mcp_client.rs"]
mod mcp_client;

/// Tools, die k3c-studio seit WZ63.5 für Testläufe, Engine-Läufe und Spieldaten führt.
const BALANCE_TOOLS: [&str; 6] = [
    "sim_test",
    "sim_run",
    "replay_run",
    "reports_list",
    "report_read",
    "saves_list",
];

fn studio_tool(name: &str) -> bool {
    name == "level_generate"
        || ["world_", "assets_", "hub_"].iter().any(|p| name.starts_with(p))
        || BALANCE_TOOLS.contains(&name)
}

async fn started() -> (Server, Vec<Value>, Value) {
    let server = Server::new(Config {
        root: std::path::PathBuf::new(),
        port: 0,
        version: "test".to_owned(),
        log: k3c_kit::logger::Logger::discard(),
        usage: None,
    });
    server.start().await.unwrap();
    let mut client = mcp_client::Client::connect(server.url()).await;
    let tools = client.tools().await;
    (server, tools, client.init.clone())
}

fn names(tools: &[Value]) -> Vec<&str> {
    tools.iter().filter_map(|t| t["name"].as_str()).collect()
}

/// AC-04 (WZ63): headless listet der Server nur Studio-Tools, jedes mit „Nutze es bei … Statt …“ und Annotations, und
/// keins der Workbench. Dass am Ende alle Studio-Tools da sind, prüft die Golden (`PENDING_TOOLS`).
#[tokio::test(flavor = "multi_thread")]
async fn headless_listet_nur_studio_tools() {
    let (server, tools, init) = started().await;
    assert_eq!(init["serverInfo"]["name"], json!(NAME), "{init}");
    assert_eq!(init["serverInfo"]["version"], json!("test"), "{init}");
    assert_eq!(init["instructions"], json!(INSTRUCTIONS));
    for tool in &tools {
        let name = tool["name"].as_str().unwrap();
        assert!(studio_tool(name), "{name} gehört nicht ins Studio");
        let text = tool["description"].as_str().unwrap_or_default();
        assert!(
            tool["annotations"].is_object() && text.contains("Nutze es bei:") && text.contains("Statt:"),
            "{name}: Beschreibung ohne „Nutze es bei / Statt“ oder ohne Annotations"
        );
    }
    let listed = names(&tools);
    for want in ["reports_list", "report_read", "saves_list"] {
        assert!(listed.contains(&want), "{want} fehlt: {listed:?}");
    }
    for workbench in [
        "plan_list",
        "plan_create",
        "svc_status",
        "check_run",
        "task_start",
        "workbench_status",
        "logs_errors",
    ] {
        assert!(!listed.contains(&workbench), "Workbench-Tool {workbench} im Studio");
    }
    server.stop().await.unwrap();
}

/// Jedes registrierte Tool steht in `instructions.md`, und die Zähler führen jedes.
#[tokio::test(flavor = "multi_thread")]
async fn instructions_nennen_jedes_tool() {
    let (server, tools, _) = started().await;
    let stats = server.stats().await;
    assert_eq!(stats.tools.len(), tools.len());
    for tool in &stats.tools {
        assert!(
            INSTRUCTIONS.contains(&format!("`{}", tool.name)),
            "{} fehlt in instructions.md",
            tool.name
        );
    }
    server.stop().await.unwrap();
}

/// RU70.12/AC-02: `prompts/list` führt je Vorlage `asset-<id>`, Argumente = Platzhalter im Text, Pflicht außer `zustand`
/// und `basis` (wie `tools_assets_prompts.go`); `prompts/get` füllt sie aus.
#[tokio::test(flavor = "multi_thread")]
async fn prompts_je_vorlage() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let datei = std::fs::read_to_string(root.join("data/asset-prompts.json")).unwrap();
    let vorlagen: Vec<Value> = serde_json::from_str(&datei).unwrap();
    assert!(!vorlagen.is_empty());
    let server = Server::new(Config {
        root,
        port: 0,
        version: "test".to_owned(),
        log: k3c_kit::logger::Logger::discard(),
        usage: None,
    });
    server.start().await.unwrap();
    let mut client = mcp_client::Client::connect(server.url()).await;
    assert!(client.init["capabilities"]["prompts"].is_object(), "{}", client.init);
    let list = client.request("prompts/list", json!({})).await;
    let mut want: Vec<Value> = vorlagen.iter().map(erwarteter_prompt).collect();
    want.sort_by_key(|p| p["name"].as_str().unwrap().to_owned());
    assert_eq!(list["prompts"], json!(want));
    for (vorlage, prompt) in vorlagen.iter().map(|v| (v, erwarteter_prompt(v))) {
        let mut args = serde_json::Map::new();
        let mut text = vorlage["text"].as_str().unwrap().to_owned();
        for name in prompt["arguments"].as_array().into_iter().flatten() {
            let name = name["name"].as_str().unwrap();
            args.insert(name.to_owned(), json!(format!("<{name}>")));
            text = text.replace(&format!("{{{name}}}"), &format!("<{name}>"));
        }
        let params = json!({"name": prompt["name"], "arguments": args});
        let got = client.request("prompts/get", params).await;
        assert_eq!(got["description"], vorlage["titel"]);
        assert_eq!(
            got["messages"],
            json!([{"role": "user", "content": {"type": "text", "text": text}}])
        );
    }
    server.stop().await.unwrap();
}

fn erwarteter_prompt(vorlage: &Value) -> Value {
    let feld = |name: &str| vorlage[name].as_str().unwrap();
    let muster = regex::Regex::new(
        r"\{(asset|rig|teil|pfad|zustand|basis|eingabe|material|ton|slot|datei|pack|lizenz|treffer)\}",
    );
    let mut namen: Vec<String> = Vec::new();
    for found in muster.unwrap().captures_iter(feld("text")) {
        if !namen.iter().any(|n| n == &found[1]) {
            namen.push(found[1].to_owned());
        }
    }
    let mut prompt = json!({"name": format!("asset-{}", feld("id")), "title": feld("titel"),
        "description": format!("Asset-Prompt ({}): {}", feld("ort"), feld("titel"))});
    let argumente: Vec<Value> = namen
        .iter()
        .map(|n| match n.as_str() {
            "zustand" | "basis" => json!({ "name": n }),
            _ => json!({"name": n, "required": true}),
        })
        .collect();
    if !argumente.is_empty() {
        prompt["arguments"] = json!(argumente);
    }
    prompt
}
