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
