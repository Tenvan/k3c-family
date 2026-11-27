//! Erfasst die Tool-Liste der Go-Fassung (V7): `cargo run -p k3c-studio --example capture_go_tools -- <MCP-URL>`
//! schreibt `tests/golden/go-tools.json`. Nur lesend (`initialize`, `tools/list`); die Golden wird nie von Hand geändert.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use serde_json::{Value, json};

#[path = "../tests/support/mcp_client.rs"]
mod mcp_client;

/// Der Teil eines Tools, den die Golden behält.
fn entry(tool: &Value) -> Value {
    json!({
        "name": tool["name"],
        "description": tool["description"],
        "annotations": tool["annotations"],
        "inputSchema": tool["inputSchema"],
    })
}

fn main() {
    let url = std::env::args().nth(1).expect("Aufruf: capture_go_tools <MCP-URL>");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Laufzeit");
    let mut tools = runtime.block_on(async {
        let mut client = mcp_client::Client::connect(url).await;
        let server = client.init["serverInfo"]["name"].clone();
        assert_eq!(server, json!("k3c-studio"), "falscher Server: {server}");
        client.tools().await
    });
    tools.sort_by_key(|tool| tool["name"].as_str().unwrap_or_default().to_owned());
    let golden: Vec<Value> = tools.iter().map(entry).collect();
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/go-tools.json");
    let text = serde_json::to_string_pretty(&golden).expect("JSON");
    std::fs::write(path, text + "\n").expect("schreiben");
    println!("✅ {} Tools nach {path}", golden.len());
}
