//! Neu (Go hat keinen Test zu `app_hub.go`): die Methoden `Hub*` über die Aufruf-Tabelle wie die Oberfläche. Die
//! Engine-Seite (`sim hub`) ersetzt eine feste Antwort; ihre Rechnung prüfen die Tests in `hubplan`.

use std::sync::Arc;

use serde_json::value::RawValue;
use serde_json::{Value, json};
use tempfile::TempDir;

use super::super::App;
use super::super::rpc::call;
use crate::hubplan::{Engine, EngineFactory};
use crate::test_helpers::tmp_dir;

const HUB: &str = "{\n  \"homeRadiusUnits\": 12\n}\n";

fn fake() -> EngineFactory {
    Arc::new(|_| {
        let engine: Engine = Arc::new(|_| {
            RawValue::from_string(r#"{"kinds":["workshop"]}"#.to_owned())
                .map(Some)
                .map_err(|e| e.to_string())
        });
        engine
    })
}

fn checkout() -> TempDir {
    let dir = tmp_dir();
    std::fs::create_dir_all(dir.path().join("data")).unwrap();
    std::fs::write(dir.path().join("data").join("hub.json"), HUB).unwrap();
    dir
}

#[test]
fn hub_methoden_ueber_die_tabelle() {
    let dir = checkout();
    let mut app = App::new(dir.path().to_path_buf(), 0, None);
    app.hub_engine = fake();
    app.mark_ready();

    let plan = call(&app, "HubPlan", &[]).unwrap();
    assert_eq!((&plan["raw"], &plan["error"]), (&json!(HUB), &json!("")));
    assert_eq!(plan["kinds"], json!(["workshop"]));
    let candidate = HUB.replace("12", "13");
    let check = call(&app, "HubCheck", &[json!(candidate)]).unwrap();
    assert_eq!(check["raw"], json!(candidate));
    assert_eq!(check["stand"], json!({"hub": "", "rules": ""}));
    let rules = call(&app, "HubRules", &[json!("")]).unwrap();
    assert_eq!(rules["task"], json!("task gen:rules"));
    assert!(
        rules["error"].as_str().unwrap().contains("materialien-gebaeude.md"),
        "{rules}"
    );

    // Ohne Stand ist die Datei „geändert“: nichts geschrieben.
    let stale = call(&app, "HubSave", &[json!(candidate), Value::Null]).unwrap();
    assert_eq!(stale["saved"], json!(false), "{stale}");
    assert!(
        stale["stale"]
            .as_str()
            .unwrap()
            .starts_with("data/hub.json seit dem Laden"),
        "{stale}"
    );
    let saved = call(&app, "HubSave", &[json!(candidate), plan["stand"].clone()]).unwrap();
    assert_eq!(saved["saved"], json!(true), "{saved}");
    assert!(
        saved["rulesError"]
            .as_str()
            .unwrap()
            .starts_with("Regeltabelle veraltet"),
        "{saved}"
    );
    let written = std::fs::read_to_string(dir.path().join("data").join("hub.json")).unwrap();
    assert_eq!(written, candidate);

    assert!(call(&app, "HubCheck", &[]).is_err());
    assert!(call(&app, "HubSave", &[json!(candidate)]).is_err());
}
