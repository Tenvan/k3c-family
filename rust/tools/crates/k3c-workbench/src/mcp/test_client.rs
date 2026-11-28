//! Kleiner MCP-Client über Streamable HTTP für Tests (auch `tests/tool_liste.rs` bindet ihn ein).

use std::time::Duration;

use serde_json::{Value, json};

/// Eine Session am Server.
pub struct Client {
    http: reqwest::Client,
    url: String,
    /// Kopfzeile `X-K3C-Root`, wie sie `.mcp.json` setzt.
    root: Option<String>,
    session: Option<String>,
    id: u64,
    /// Ergebnis von `initialize`.
    #[allow(dead_code)]
    pub init: Value,
}

/// Die Antwort mit `id` aus einem JSON- oder SSE-Rumpf.
fn answer(body: &str, id: u64) -> Option<Value> {
    body.lines()
        .filter_map(|line| serde_json::from_str::<Value>(line.strip_prefix("data:").unwrap_or(line).trim()).ok())
        .find(|v| v["id"] == json!(id))
}

impl Client {
    /// Meldet sich an (`initialize`, `notifications/initialized`).
    pub async fn connect(url: String) -> Client {
        Client::connect_from(url, None).await
    }

    /// Wie [`Client::connect`], mit dem Arbeitsverzeichnis `root` in der Kopfzeile.
    pub async fn connect_from(url: String, root: Option<String>) -> Client {
        let mut client = Client {
            http: reqwest::Client::new(),
            url,
            root,
            session: None,
            id: 0,
            init: Value::Null,
        };
        let params = json!({"protocolVersion": "2025-06-18", "capabilities": {},
            "clientInfo": {"name": "k3c-workbench-test", "version": "0"}});
        client.init = client.request("initialize", params).await;
        client
            .post(json!({"jsonrpc": "2.0", "method": "notifications/initialized"}))
            .await;
        client
    }

    async fn post(&mut self, body: Value) -> reqwest::Response {
        let mut request = self
            .http
            .post(&self.url)
            .header("Accept", "application/json, text/event-stream")
            .header("Content-Type", "application/json")
            .header("MCP-Protocol-Version", "2025-06-18")
            .body(body.to_string());
        if let Some(root) = &self.root {
            request = request.header("X-K3C-Root", root);
        }
        if let Some(session) = &self.session {
            request = request.header("Mcp-Session-Id", session);
        }
        let response = request.send().await.expect("senden");
        assert!(response.status().is_success(), "status {}", response.status());
        if let Some(session) = response.headers().get("mcp-session-id") {
            self.session = Some(session.to_str().expect("session").to_owned());
        }
        response
    }

    /// Ruft eine Methode und liefert die ganze Antwort (`result` oder `error`).
    pub async fn answer(&mut self, method: &str, params: Value) -> Value {
        self.id += 1;
        let id = self.id;
        let mut response = self
            .post(json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}))
            .await;
        let mut body = Vec::new();
        loop {
            let chunk = tokio::time::timeout(Duration::from_secs(15), response.chunk())
                .await
                .expect("Frist")
                .expect("Rumpf");
            let ended = chunk.is_none();
            body.extend_from_slice(&chunk.unwrap_or_default());
            if let Some(found) = answer(&String::from_utf8_lossy(&body), id) {
                return found;
            }
            assert!(!ended, "keine Antwort auf {method}: {}", String::from_utf8_lossy(&body));
        }
    }

    /// Ruft eine Methode und liefert ihr `result`.
    pub async fn request(&mut self, method: &str, params: Value) -> Value {
        let found = self.answer(method, params).await;
        assert!(found.get("error").is_none(), "{method}: {found}");
        found["result"].clone()
    }

    /// Die Tools aus `tools/list` (eine Seite, mehr gibt es nicht).
    pub async fn tools(&mut self) -> Vec<Value> {
        let result = self.request("tools/list", json!({})).await;
        assert!(result["nextCursor"].is_null(), "tools/list hat mehrere Seiten");
        result["tools"].as_array().expect("tools").clone()
    }

    /// `tools/call`: Text aller Teile und `isError`.
    #[allow(dead_code)]
    pub async fn call_text(&mut self, name: &str, arguments: Value) -> (String, bool) {
        let params = json!({"name": name, "arguments": arguments});
        let result = self.request("tools/call", params).await;
        let parts = result["content"].as_array().cloned().unwrap_or_default();
        let text: Vec<&str> = parts.iter().filter_map(|part| part["text"].as_str()).collect();
        (text.join("\n"), result["isError"].as_bool().unwrap_or(false))
    }
}
