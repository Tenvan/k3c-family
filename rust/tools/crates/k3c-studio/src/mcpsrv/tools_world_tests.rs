//! Port von `tools_world_test.go` (B-954): Roundtrip der Tools `world_*` auf einer Kopie von `data/` mit der Spiel-EXE;
//! die Welt-Dateien des Checkouts werden nur gelesen. Dazu die Hilfen der Tests zu Skins und Geschichte.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::{Value, json};
use tempfile::TempDir;

use super::super::kit::Logger;
use super::super::{Config, Server};
use crate::exeengine;
use crate::test_helpers::{mcp_client, tmp_dir};
use crate::world::Editor;
use crate::world::exe_tests::fixture_engine;

/// Wie [`super::OverrideIn`] die erlaubten Dateien in der Beschreibung nennt.
const OVERRIDE_FILES_TEXT: &str =
    "buildings, economy, enemies, events, goals, hub, monarch, ship, troops, waves, welt-events";

fn repo() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../.."))
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// Temp-Checkout mit Kopie von `data/`, Server darauf und ein verbundener Client.
pub(in super::super) struct World {
    pub(in super::super) root: PathBuf,
    server: Server,
    client: mcp_client::Client,
    _dir: TempDir,
}

impl World {
    /// `None` ohne frische Spiel-EXE (Welten- und Hub-Tools rechnen über sie).
    pub(in super::super) async fn start() -> Option<World> {
        if !exeengine::require() {
            return None;
        }
        let dir = tmp_dir();
        let root = dir.path().to_path_buf();
        copy_dir(&repo().join("data"), &root.join("data"));
        let config = Config {
            root: root.clone(),
            port: 0,
            version: "test".to_owned(),
            log: Logger::discard(),
            usage: None,
        };
        let server = Server::with_world_engine(config, Arc::new(fixture_engine));
        server.start().await.unwrap();
        let client = mcp_client::Client::connect(server.url()).await;
        Some(World {
            root,
            server,
            client,
            _dir: dir,
        })
    }

    /// Text und `isError` eines Aufrufs.
    pub(in super::super) async fn call(&mut self, tool: &str, args: Value) -> (String, bool) {
        self.client.call_text(tool, args).await
    }

    /// Der Aufruf gelingt und die Antwort enthält alle `wants`.
    pub(in super::super) async fn ok(&mut self, tool: &str, args: Value, wants: &[&str]) -> String {
        let (text, is_error) = self.call(tool, args).await;
        assert!(!is_error, "{tool}: {text}");
        for want in wants {
            assert!(text.contains(want), "{tool} ohne {want:?}:\n{text}");
        }
        text
    }

    /// Der Aufruf wird abgelehnt und die Meldung enthält `want`.
    pub(in super::super) async fn rejects(&mut self, tool: &str, args: Value, want: &str) {
        let (text, is_error) = self.call(tool, args).await;
        assert!(is_error && text.contains(want), "{tool}, erwartet {want:?}: {text}");
    }

    pub(in super::super) async fn stop(self) {
        self.server.stop().await.unwrap();
    }
}

async fn create_and_delete(w: &mut World) {
    let create = json!({"id": "hanse", "name": "Hanse", "extends": "standard"});
    w.ok("world_create", create, &["Welt hanse angelegt", "Checkout:"])
        .await;
    let again = json!({"id": "hanse", "name": "Nochmal"});
    w.rejects("world_create", again, "gibt es schon").await;
    w.rejects("world_delete", json!({"id": "hanse"}), "confirm=true").await;
    let standard = json!({"id": "standard", "confirm": true});
    w.rejects("world_delete", standard, "feste Welt").await;
}

async fn change(w: &mut World) {
    let patch = json!({"notes": "Handelsstädte", "combat": {"mode": "groups", "groupSize": 5},
        "islands": [{"id": "island1", "name": "Lübeck", "biomes": ["forest", "cave", "mine", "ironhold", "crystal"]}]});
    w.ok(
        "world_set",
        json!({"id": "hanse", "patch": patch}),
        &["Welt hanse gespeichert"],
    )
    .await;
    let gold = json!({"id": "hanse", "file": "economy", "path": ["purse", "startGold"], "value": 99});
    let done = "Override economy › purse.startGold der Welt hanse gesetzt";
    w.ok("world_override_set", gold, &[done]).await;
    let chunks = json!({"id": "hanse", "island": "island1", "depth": 0, "chunks": [{"slot": -7, "kind": "harbor"}]});
    let done = "Chunks der Welt hanse, Insel island1, Tiefe 0 gespeichert";
    w.ok("world_chunks_set", chunks, &[done]).await;
    w.ok("world_check", json!({"id": "hanse"}), &["ist gültig"]).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn world_tools_roundtrip() {
    let Some(mut w) = World::start().await else { return };
    w.ok("world_list", json!({}), &["standard · Standard"]).await;
    create_and_delete(&mut w).await;
    change(&mut w).await;
    let wants = [
        "hanse · Hanse · extends standard · 1 Inseln",
        "Lübeck · Tiefen [0 1 2 3 4] → forest, cave, mine, ironhold, crystal",
        "plannedChunks[0]:",
        "\"startGold\": 99",
        "\"notes\": \"Handelsstädte\"",
        "Arten: weather ",
    ];
    w.ok("world_get", json!({"id": "hanse"}), &wants).await;
    let unknown = json!({"id": "hanse", "patch": {"farbe": "rot"}});
    w.rejects("world_set", unknown, "unbekanntes Feld").await;
    assert!(
        !repo().join("data/worlds/hanse.json").exists(),
        "Welt-Datei im Checkout geschrieben"
    );
    // Nur bekannte Checkouts, nie ein beliebiger Pfad.
    let foreign = json!({"id": "hanse", "checkout": repo().join("rust")});
    let (text, is_error) = w.call("world_get", foreign).await;
    assert!(is_error && text.starts_with("checkout"), "{text}");
    w.ok(
        "world_delete",
        json!({"id": "hanse", "confirm": true}),
        &["Welt hanse gelöscht"],
    )
    .await;
    assert!(!w.root.join("data/worlds/hanse.json").exists());
    w.stop().await;
}

/// B-986: Die Liste der erlaubten Dateien in der Beschreibung von `world_override_set` gleicht der Basis der Engine.
#[tokio::test(flavor = "multi_thread")]
async fn override_files_text_matches_engine() {
    let Some(mut w) = World::start().await else { return };
    let editor = Editor::new(&w.root, fixture_engine(&w.root));
    let view = tokio::task::spawn_blocking(move || editor.values("standard"))
        .await
        .unwrap();
    let files: BTreeSet<&str> = view.entries.iter().map(|v| v.file.as_str()).collect();
    let files: Vec<&str> = files.into_iter().collect();
    assert_eq!(files.join(", "), OVERRIDE_FILES_TEXT, "{}", view.error);
    let tools = w.client.tools().await;
    let tool = tools.iter().find(|t| t["name"] == "world_override_set").unwrap();
    let description = tool["description"].as_str().unwrap();
    assert!(description.contains(&format!("Erlaubte Dateien: {OVERRIDE_FILES_TEXT};")));
    w.stop().await;
}
