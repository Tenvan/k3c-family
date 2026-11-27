//! Die fünf Commands der Seite Git über die Aufruf-Tabelle, auf einem Wegwerf-Repo und mit gestellten PRs statt `gh`.

use std::sync::Arc;

use serde_json::{Value, json};

use super::*;
use crate::github::MergedSource;
use crate::gitops::fixture_tests::Fixture;
use crate::rpc::call;

async fn rpc(app: &Arc<App>, method: &'static str, args: Value) -> Result<Value, String> {
    let app = Arc::clone(app);
    let args = args.as_array().cloned().unwrap_or_default();
    tokio::task::spawn_blocking(move || call(&app, method, &args).map_err(|e| e.0))
        .await
        .unwrap()
}

fn without_gh() -> MergedSource {
    Arc::new(|| Box::pin(async { Err("gh nicht gefunden".to_owned()) }))
}

fn branch<'a>(list: &'a Value, name: &str) -> &'a Value {
    list.as_array().unwrap().iter().find(|b| b["name"] == name).unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn fuenf_commands() {
    let f = Fixture::setup().await;
    // Eigener Ordner für die App: ihr Log und ihre Statistik ändern das Repo nicht.
    let root = f.tmp.path().join("app");
    std::fs::create_dir(&root).unwrap();
    let app = App::new(root.clone(), 0, None)
        .with_usage_file(root.join("usage.json"))
        .with_git(f.repo(), without_gh());
    let app = Arc::new(app);
    app.startup().await;

    let view = rpc(&app, "GitOverview", json!([])).await.unwrap();
    assert_eq!(view["status"]["branch"], "main", "{view}");
    assert_eq!(view["status"]["changes"], json!([]));
    assert_eq!(view["files"], json!({"base": "origin/main", "files": []}));
    assert_eq!(branch(&view["branches"], "main")["state"], "basis");
    let stamp = rpc(&app, "GitStamp", json!([])).await.unwrap();
    assert_eq!(stamp.as_str().unwrap().len(), 16);

    let message = rpc(&app, "GitAction", json!(["fetch"])).await.unwrap();
    assert_eq!(message, "Fetch fertig");
    let error = rpc(&app, "GitAction", json!(["--force"])).await.unwrap_err();
    assert_eq!(error, "unbekannte Aktion \"--force\"");

    // Ohne gh prüft git allein.
    let list = rpc(&app, "GitCheckMerged", json!([null])).await.unwrap();
    assert_eq!(branch(&list, "squash")["state"], "squash", "{list}");
    let error = rpc(&app, "GitDeleteBranches", json!([null, false])).await.unwrap_err();
    assert_eq!(error, "keine Branches gewählt");
    let message = rpc(&app, "GitDeleteBranches", json!([["squash"], false]))
        .await
        .unwrap();
    assert!(message.as_str().unwrap().starts_with("1 Branches gelöscht: squash ("));

    app.shutdown().await;
    assert_eq!(rpc(&app, "GitStamp", json!([])).await.unwrap_err(), "backend beendet");
    let file = root.join("logs").join(format!("{}.jsonl", crate::ports::LOG_NAME));
    let log = std::fs::read_to_string(file).unwrap();
    for want in [
        "✅ git fetch",
        "❌ git --force fehlgeschlagen",
        "❌ Gemergte PRs nicht gelesen",
        "❌ Branches löschen fehlgeschlagen",
        "🧹 Branches gelöscht",
    ] {
        assert!(log.contains(want), "{want} fehlt: {log}");
    }
}
