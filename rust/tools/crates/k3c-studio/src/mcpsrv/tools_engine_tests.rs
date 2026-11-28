//! Port der `level_generate`-Tests aus `tools_engine_test.go`; die `sim_run`-Tests kommen mit `sim_run` (RU70.20).

use std::sync::{Arc, Mutex};

use serde_json::{Value, json};

use super::super::kit::{Logger, Workspace};
use super::super::{Config, Server};
use super::{LevelIn, level_generate};
use crate::gameexe::{BUILD_COMMAND, Event, Options};
use crate::test_helpers::{mark_root, mcp_client, tmp_dir};

/// RU34.2: `level_generate` rechnet mit der EXE des gewählten Checkouts, auch eines Worktrees, ohne Fenster.
#[tokio::test]
async fn level_generate_nimmt_checkout_der_anfrage() {
    let main = tmp_dir();
    let worktree = main.path().join(".claude").join("worktrees").join("wt-a");
    let ws = Workspace {
        root: worktree.clone(),
        name: "wt-a".to_owned(),
    };
    let input: LevelIn = serde_json::from_value(json!({ "seed": "1" })).unwrap();
    let seen: Arc<Mutex<Option<Options>>> = Arc::default();
    let got = Arc::clone(&seen);
    let text = level_generate(&ws, &input, |o| async move {
        *got.lock().unwrap() = Some(o);
        Ok(Event {
            event: "result".to_owned(),
            text: "Level".to_owned(),
            ..Event::default()
        })
    })
    .await;
    assert_eq!(text, Ok("Level".to_owned()));
    let o = seen.lock().unwrap().take().unwrap();
    assert_eq!(
        (o.root, o.sub.as_str(), o.window, o.speed),
        (worktree, "level", false, 0)
    );
    let sent: Value = serde_json::from_slice(&o.input).unwrap();
    assert_eq!(
        sent,
        json!({ "seed": "1" }),
        "leeres biome fehlt in der Eingabe der EXE"
    );
}

/// RU34.2: Ohne gebaute EXE rechnet nichts und der Fehler nennt den Bau-Befehl.
#[tokio::test(flavor = "multi_thread")]
async fn level_generate_ohne_exe() {
    let root = tmp_dir();
    mark_root(root.path());
    let server = Server::new(Config {
        root: root.path().to_path_buf(),
        port: 0,
        version: "test".to_owned(),
        log: Logger::discard(),
        usage: None,
    });
    server.start().await.unwrap();
    let mut client = mcp_client::Client::connect(server.url()).await;
    let (text, is_error) = client.call_text("level_generate", json!({ "seed": "1" })).await;
    assert!(is_error && text.contains(BUILD_COMMAND), "level_generate: {text}");
    server.stop().await.unwrap();
}
