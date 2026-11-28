//! Die Commands der Seite Git (und `GitHubStatus`) über die Aufruf-Tabelle, auf einem Wegwerf-Repo und mit
//! gestelltem `gh`.

use std::sync::{Arc, Mutex};

use serde_json::{Value, json};

use super::*;
use crate::github::{Client, RunError, RunFuture, Runner};
use crate::gitops::fixture_tests::Fixture;
use crate::rpc::call;

async fn rpc(app: &Arc<App>, method: &'static str, args: Value) -> Result<Value, String> {
    let app = Arc::clone(app);
    let args = args.as_array().cloned().unwrap_or_default();
    tokio::task::spawn_blocking(move || call(&app, method, &args).map_err(|e| e.0))
        .await
        .unwrap()
}

fn without_gh() -> Client {
    Client::with_runner(Arc::new(|_| -> RunFuture {
        Box::pin(async { Err(RunError::NotFound) })
    }))
}

/// `gh` aus der Konserve, mit den Aufrufen.
fn canned() -> (Client, Arc<Mutex<Vec<String>>>) {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&calls);
    let run: Runner = Arc::new(move |args: Vec<String>| -> RunFuture {
        let out = match args[1..3].join(" ").as_str() {
            "pr view" => r#"{"state":"OPEN","baseRefName":"main","headRefOid":"abc"}"#,
            "pr create" => "https://example.invalid/pull/7\n",
            "pr list" => r#"[{"number":7,"title":"T","state":"OPEN","headRefName":"sprint/x"}]"#,
            _ => "[]",
        };
        seen.lock().unwrap().push(args.join(" "));
        Box::pin(async move { Ok(out.into()) })
    });
    (Client::with_runner(run), calls)
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

#[tokio::test(flavor = "multi_thread")]
async fn pr_commands() {
    let f = Fixture::setup().await;
    let root = f.tmp.path().join("app");
    std::fs::create_dir(&root).unwrap();
    let (gh, calls) = canned();
    let app = App::new(root.clone(), 0, None)
        .with_usage_file(root.join("usage.json"))
        .with_git(f.repo(), gh);
    let app = Arc::new(app);
    app.startup().await;
    let work = f.work();
    let create = |title: &str| rpc(&app, "GitPRCreate", json!([title, ""]));

    let list = rpc(&app, "GitPRs", json!([])).await.unwrap();
    assert_eq!(list["prs"][0]["head"], "sprint/x", "{list}");
    assert!(list.get("error").is_none() && list["fetched"].is_string(), "{list}");
    let status = rpc(&app, "GitHubStatus", json!([false])).await.unwrap();
    assert_eq!(status["sprints"]["X"]["number"], 7, "{status}");

    assert_eq!(create("T").await.unwrap_err(), "von main gibt es keinen PR");
    f.sh(&work, &["checkout", "-q", "offen"]).await;
    let error = create("T").await.unwrap_err();
    assert_eq!(error, "der Branch ist nicht auf origin: erst pushen");
    f.sh(&work, &["push", "-q", "-u", "origin", "offen"]).await;
    assert_eq!(create("T").await.unwrap(), "https://example.invalid/pull/7");
    f.commit(&work, "mehr.txt", "1").await;
    assert_eq!(create("T").await.unwrap_err(), "1 Commits nicht gepusht: erst pushen");
    f.sh(&work, &["checkout", "-q", "--detach"]).await;
    assert_eq!(create("T").await.unwrap_err(), "kein gültiger Branch ausgecheckt");

    let action = |number: i64, how: &str| rpc(&app, "GitPRAction", json!([number, how]));
    assert_eq!(action(0, "merge").await.unwrap_err(), "ungültige PR-Nummer");
    assert_eq!(
        action(7, "--admin").await.unwrap_err(),
        "unbekannte PR-Aktion \"--admin\""
    );
    assert_eq!(action(7, "squash").await.unwrap(), "PR #7 gemergt");
    assert_eq!(action(7, "close").await.unwrap(), "PR #7 geschlossen");
    assert_eq!(
        calls
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.starts_with("gh pr merge"))
            .count(),
        1
    );

    app.shutdown().await;
    assert_eq!(rpc(&app, "GitPRs", json!([])).await.unwrap_err(), "backend beendet");
    let file = root.join("logs").join(format!("{}.jsonl", crate::ports::LOG_NAME));
    let log = std::fs::read_to_string(file).unwrap();
    for want in [
        "✅ PR anlegen",
        "❌ PR anlegen fehlgeschlagen",
        "✅ PR #7 squash",
        "✅ PR #7 close",
    ] {
        assert!(log.contains(want), "{want} fehlt: {log}");
    }
}
