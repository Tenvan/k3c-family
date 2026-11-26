//! Port von `app_world_test.go` (B-879): der Welten-Editor erreicht die Welt über die Methoden wie die Oberfläche. Die
//! Engine-Seite (`k3c-game sim world`) ersetzt hier eine feste Antwort; ihre Rechnung prüfen die Tests der Spiel-EXE.
//! Dazu die Verdrahtung der Aufruf-Tabelle und das Ereignis `world:changed`.

use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

use serde_json::value::RawValue;
use serde_json::{Value, json};
use tempfile::TempDir;

use super::super::rpc::{METHODS, call};
use super::super::{App, EV_WORLD, Emit};
use crate::test_helpers::tmp_dir;
use crate::world::{Engine, EngineFactory};

const STANDARD: &str = "{\n  \"id\": \"standard\",\n  \"name\": \"Standard\",\n  \"islands\": []\n}\n";

type Ops = Arc<Mutex<Vec<String>>>;

/// Engine mit festen Antworten, die jede op festhält.
fn fake(ops: &Ops) -> EngineFactory {
    let ops = Arc::clone(ops);
    Arc::new(move |_| {
        let ops = Arc::clone(&ops);
        let engine: Engine = Arc::new(move |input| {
            let request: Value = serde_json::from_slice(&input).map_err(|e| e.to_string())?;
            let op = request["op"].as_str().unwrap_or_default().to_owned();
            let answer = if op == "values" {
                r#"{"extends":"","base":{"economy":{"purse":{"startGold":14}}},"overrides":{}}"#
            } else {
                r#"{"warnings":[]}"#
            };
            ops.lock().unwrap().push(op);
            RawValue::from_string(answer.to_owned())
                .map(Some)
                .map_err(|e| e.to_string())
        });
        engine
    })
}

fn checkout() -> TempDir {
    let dir = tmp_dir();
    let worlds = dir.path().join("data").join("worlds");
    std::fs::create_dir_all(&worlds).unwrap();
    std::fs::write(worlds.join("standard.json"), STANDARD).unwrap();
    // Ohne feste Welten lehnt das Löschen alles ab.
    let fixed = r#"{"weltbilder": {"standard": {}}}"#;
    std::fs::write(dir.path().join("data").join("paper.json"), fixed).unwrap();
    dir
}

fn app(dir: &TempDir, ops: &Ops) -> App {
    let app = App::new(dir.path().to_path_buf(), 0, None).with_world_engine(fake(ops));
    app.mark_ready();
    app
}

#[test]
fn world_bindings() {
    let (dir, ops) = (checkout(), Ops::default());
    let app = app(&dir, &ops);
    let path = dir.path().join("data/worlds/standard.json");
    let read = || std::fs::read_to_string(&path).unwrap();
    assert_eq!(app.world_list().len(), 1);
    let field = ["purse".to_owned(), "startGold".to_owned()];
    let view = app.world_override_set("standard", "economy", &field, "77");
    assert!(view.error.is_empty(), "{}", view.error);
    assert!(read().contains("\"startGold\": 77"), "{}", read());
    assert!(
        ops.lock().unwrap().iter().any(|op| op == "check"),
        "ohne Prüfung geschrieben"
    );
    let view = app.world_override_set("standard", "economy", &field, "null");
    assert!(view.error.is_empty(), "{}", view.error);
    assert_eq!(read(), STANDARD, "nach Zurücksetzen nicht wie vorher");
}

/// Command und UI-API rufen dieselbe Tabelle: Argumente positionell, `null` wie Go, JSON-Form wie das Frontend.
#[test]
fn world_methoden_ueber_die_tabelle() {
    let (dir, ops) = (checkout(), Ops::default());
    let app = app(&dir, &ops);
    let rpc = |name: &str, args: Value| call(&app, name, args.as_array().unwrap()).unwrap();
    assert_eq!(rpc("WorldList", json!([]))[0]["id"], "standard");
    assert_eq!(rpc("WorldDetail", json!(["standard"]))["name"], "Standard");
    let values = rpc(
        "WorldOverrideSet",
        json!(["standard", "economy", ["purse", "startGold"], 77]),
    );
    assert_eq!(
        (&values["error"], &values["entries"][0]["base"]),
        (&json!(""), &json!(14))
    );
    assert_eq!(rpc("WorldValues", json!(["standard"]))["world"], "standard");
    assert_eq!(rpc("WorldCreate", json!(["neu", "Neu", ""]))["error"], "");
    assert_eq!(rpc("WorldSetKind", json!(["neu", "new"]))["id"], "neu");
    assert_eq!(rpc("WorldSetExtends", json!(["neu", "standard"]))["error"], "");
    assert_eq!(rpc("WorldIslandEtappe", json!(["neu", "island1", null]))["error"], "");
    assert_eq!(rpc("WorldIslandEndBoss", json!(["neu", "island1", ""]))["error"], "");
    assert_eq!(rpc("WorldSkins", json!(["neu", null]))["error"], "");
    assert!(rpc("WorldPicks", json!(["neu"]))["error"].is_string());
    assert!(rpc("WorldSetPick", json!(["neu", "weather", ""]))["id"].is_string());
    assert_eq!(
        rpc("WorldLevel", json!(["neu", "island1", 0, "1"]))["nature"],
        json!({})
    );
    let level = rpc(
        "WorldNatureSet",
        json!(["neu", "island9", 0, "forestDensity", null, "1"]),
    );
    assert!(level["rejected"].as_str().is_some_and(|r| !r.is_empty()), "{level}");
    assert_eq!(rpc("WorldDelete", json!(["neu"]))["error"], "");
    let world_methods = METHODS.iter().filter(|(name, _)| name.starts_with("World")).count();
    assert_eq!(world_methods, 15);
    assert!(call(&app, "WorldDetail", &[]).is_err());
}

/// AC-03: Eine geänderte Welt-Datei meldet sich als `world:changed`; nach dem Beenden schweigt der Wächter.
#[tokio::test(flavor = "multi_thread")]
async fn world_changed_nach_aenderung() {
    let dir = checkout();
    let (sender, events) = mpsc::channel::<Value>();
    let forward: Emit = Arc::new(move |name: &str, data: Value| {
        if name == EV_WORLD {
            let _ = sender.send(data);
        }
    });
    let app = App::new(dir.path().to_path_buf(), 0, Some(forward)).with_usage_file(dir.path().join("usage.json"));
    app.startup().await;
    let worlds = dir.path().join("data").join("worlds");
    std::fs::write(worlds.join("neu.json"), "{}").unwrap();
    let got = events.recv_timeout(Duration::from_secs(10));
    assert_eq!(got, Ok(json!({"ids": ["neu"]})));
    std::fs::write(dir.path().join("data").join("economy.json"), "{}").unwrap();
    // Basis-Datei: ohne `ids`, die Oberfläche lädt jede Welt neu.
    assert_eq!(events.recv_timeout(Duration::from_secs(10)), Ok(json!({})));
    app.shutdown().await;
    std::fs::write(worlds.join("spaeter.json"), "{}").unwrap();
    assert!(events.recv_timeout(Duration::from_millis(1500)).is_err());
}
