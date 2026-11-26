//! Hilfen der Asset-Tool-Tests (Port von `tools_assets_paper_test.go`). Test-Repos entstehen nur unter `.work/`; die
//! Tools laufen über echtes Streamable HTTP gegen die CLI-Attrappe.

use std::path::Path;

use serde_json::{Value, json};

use super::{Config, Server};
use crate::paperassets::test_helpers::{Wurzel, schreibe, stub_skript, test_repo, ts_text};
pub(crate) use crate::paperassets::test_helpers::{hash_baum, lies};
use crate::test_helpers::mcp_client::Client;

/// Git-Repo mit Stil-Datei, Schema, Prompt-Vorlagen und den Fixture-Specs monarch, pferd, mauer; `None` ohne bun oder
/// git. Statt `K3C_PAPER_CLI` (Go `TestMain`) liegt dort, wo `Cli::new` die Attrappe sucht, ein Verweis auf die
/// Attrappe des Repos.
pub(crate) fn paper_repo() -> Option<Wurzel> {
    let (w, _) = test_repo()?;
    let prompts = std::fs::read(crate::review::test_helpers::repo_root().join("data/asset-prompts.json"));
    schreibe(&w.pfad, "data/asset-prompts.json", prompts.expect("Vorlagen"));
    let verweis = format!(
        "import {{ run }} from {};\n\
         const r = run(process.argv.slice(2));\n\
         if (r.stdout) process.stdout.write(r.stdout + '\\n');\n\
         if (r.stderr) process.stderr.write(r.stderr + '\\n');\n\
         process.exit(r.code);\n",
        ts_text(&stub_skript())
    );
    schreibe(&w.pfad, "tools/paper/fixtures/cli-stub.ts", verweis);
    Some(w)
}

/// Server und angemeldeter Client.
pub(crate) struct Verbindung {
    _server: Server,
    client: Client,
}

pub(crate) async fn connect(root: &Path) -> Verbindung {
    let server = Server::new(Config {
        root: root.to_path_buf(),
        port: 0,
        version: "test".to_owned(),
        log: k3c_kit::logger::Logger::discard(),
        usage: None,
    });
    server.start().await.expect("Start");
    let client = Client::connect(server.url()).await;
    Verbindung {
        _server: server,
        client,
    }
}

impl Verbindung {
    /// Text und `isError` eines Aufrufs.
    pub async fn call(&mut self, name: &str, arguments: Value) -> (String, bool) {
        self.client.call_text(name, arguments).await
    }

    /// Text eines Aufrufs, der gelingen muss.
    pub async fn ok(&mut self, name: &str, arguments: Value) -> String {
        let (text, is_error) = self.call(name, arguments).await;
        assert!(!is_error, "{name}:\n{text}");
        text
    }

    /// Text eines Aufrufs, der abgelehnt werden muss.
    pub async fn abgelehnt(&mut self, name: &str, arguments: Value) -> String {
        let (text, is_error) = self.call(name, arguments).await;
        assert!(is_error, "{name} nicht abgelehnt:\n{text}");
        text
    }

    pub async fn tools(&mut self) -> Vec<Value> {
        self.client.tools().await
    }

    pub async fn request(&mut self, method: &str) -> Value {
        self.client.request(method, json!({})).await
    }
}

pub(crate) fn enthaelt(text: &str, teile: &[&str]) {
    for teil in teile {
        assert!(text.contains(teil), "ohne {teil:?}:\n{text}");
    }
}
