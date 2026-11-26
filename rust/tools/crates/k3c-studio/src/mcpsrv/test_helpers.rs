//! Hilfen der Asset-Tool-Tests (Port von `tools_assets_paper_test.go`). Test-Repos entstehen nur unter `.work/`; die
//! Tools laufen über echtes Streamable HTTP gegen die CLI-Attrappe.

use std::path::Path;
use std::sync::Arc;

use serde_json::{Value, json};

use super::tools::Studio;
use super::tools_assets_review::{ReviewFactory, ReviewQuelle};
use super::{Config, Server};
use crate::assetsui::{Frame, PixelDiff, Review};
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
    verbinde(Server::new(config(root))).await
}

/// Wie [`connect`], vorher ersetzt `anpassen` Teile des Tool-Empfängers (Go ersetzt Paketvariablen).
pub(super) async fn connect_mit(root: &Path, anpassen: impl FnOnce(&mut Studio)) -> Verbindung {
    let mut studio = Studio::new(crate::world::exe_factory());
    anpassen(&mut studio);
    verbinde(Server::with_studio(config(root), studio)).await
}

fn config(root: &Path) -> Config {
    Config {
        root: root.to_path_buf(),
        port: 0,
        version: "test".to_owned(),
        log: k3c_kit::logger::Logger::discard(),
        usage: None,
    }
}

async fn verbinde(server: Server) -> Verbindung {
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

    /// Das ganze Ergebnis eines Aufrufs (Bild-Tools).
    pub async fn result(&mut self, name: &str, arguments: Value) -> Value {
        let params = json!({"name": name, "arguments": arguments});
        self.client.request("tools/call", params).await
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

/// Beantwortet die Review-Methoden mit den Beispielantworten `tools/paper/fixtures/ui/` (Basis `HEAD~1`); den echten
/// Vergleich prüfen `review` und `assetsui` (Go `fixtureReview`).
struct FixtureReview;

fn fixture(name: &str) -> Value {
    let pfad = crate::review::test_helpers::repo_root().join(format!("tools/paper/fixtures/ui/{name}.json"));
    let text = std::fs::read_to_string(&pfad).unwrap_or_else(|e| panic!("{}: {e}", pfad.display()));
    serde_json::from_str(&text).expect("Fixture")
}

fn nur_monarch(id: &str, name: &str) -> Result<Value, String> {
    if id != "monarch" {
        return Err(format!("Asset „{id}“ fehlt"));
    }
    Ok(fixture(name))
}

impl ReviewQuelle for FixtureReview {
    fn review(&self, _basis: &str) -> Result<Review, String> {
        serde_json::from_value(fixture("AssetsReview")).map_err(|e| e.to_string())
    }

    fn frame(&self, id: &str, _animation: &str, _frame: i64, _variante: &str, _basis: &str) -> Result<Frame, String> {
        let v = nur_monarch(id, "AssetsFrame")?;
        Ok(Frame {
            bild: v["bild"].as_str().expect("bild").to_owned(),
            teile: serde_json::from_value(v["teile"].clone()).map_err(|e| e.to_string())?,
        })
    }

    fn pixel_diff(&self, id: &str, _animation: &str, _frame: i64, _basis: &str) -> Result<PixelDiff, String> {
        let v = nur_monarch(id, "AssetsPixelDiff")?;
        Ok(PixelDiff {
            anteil: v["anteil"].as_f64().expect("anteil"),
            boxen: serde_json::from_value(v["boxen"].clone()).map_err(|e| e.to_string())?,
            overlay: v["overlay"].as_str().expect("overlay").to_owned(),
        })
    }
}

/// Die Fixture-Quelle (Go `mitFixtureReview`).
pub(super) fn fixture_review() -> ReviewFactory {
    Arc::new(|_| Box::new(FixtureReview))
}
