//! Die Commands der Seite Planung über die Aufruf-Tabelle und das Ereignis `planning:changed`, auf Wurzeln unter
//! `.work/`.

use std::fs;
use std::path::Path;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use serde_json::{Value, json};

use super::events::{Emit, PLANNING};
use super::*;
use crate::gitops::fixture_tests::Fixture;
use crate::rpc::call;
use crate::testutil::tmp_dir;

const TICKET: &str = "# B-001 · Altes Ticket\n\n- **Status:** offen\n- **Prio:** mittel\n";

fn write(root: &Path, rel: &str, text: &str) {
    let path = root.join(rel);
    fs::create_dir_all(path.parent().expect("Elternordner")).expect("Ordner");
    fs::write(path, text).expect("schreiben");
}

/// Eine kleine Planung: ein Ticket, ein aktiver Sprint mit einer Session, das Glossar.
fn planning_files(root: &Path) {
    write(
        root,
        "docs/backlog/README.md",
        "## Offen\n\n| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |\n|---|---|---|---|---|---|---|\n\
         | [B-001](B-001-alt.md) | DEV | Idee | mittel | offen | – | Altes Ticket |\n",
    );
    write(root, "docs/backlog/B-001-alt.md", TICKET);
    write(
        root,
        "docs/sprints/aktiv/X1-a/README.md",
        "# X1 · DEV · Sprint\n\n- **Status:** aktiv\n\n## Sessions\n\n| Session | Datei | Typ | Agent | Status |\n\
         |---|---|---|---|---|\n| X1.1 | `X1.1-eins.md` | Umsetzung | autonom | offen |\n",
    );
    write(
        root,
        "docs/sprints/aktiv/X1-a/X1.1-eins.md",
        "# X1.1 · Eins\n\nSessiontext\n",
    );
    write(root, "docs/glossar.md", "# Glossar\n");
    for state in ["geplant", "erledigt"] {
        fs::create_dir_all(root.join("docs").join("sprints").join(state)).expect("Sprint-Ordner");
    }
}

fn ready(root: &Path) -> Arc<App> {
    let app = App::new(root.to_path_buf(), 0, None).with_usage_file(root.join("usage.json"));
    app.mark_ready();
    Arc::new(app)
}

fn ui(app: &Arc<App>, method: &str, args: Value) -> Result<Value, String> {
    let args = args.as_array().cloned().unwrap_or_default();
    call(app, method, &args).map_err(|e| e.0)
}

#[test]
fn commands_der_planungsseite() {
    let tmp = tmp_dir();
    let root = tmp.path();
    planning_files(root);
    // `PlanningSet` prüft gegen die Vorlage; sie kommt nur lesend aus diesem Repo.
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("Repo-Wurzel");
    let template = fs::read_to_string(repo.join("docs/vorlagen/ticket.md")).expect("Vorlage");
    write(root, "docs/vorlagen/ticket.md", &template);
    let app = ready(root);
    let data = ui(&app, "PlanningData", json!([])).expect("PlanningData");
    assert_eq!(data["tickets"][0]["nr"], "B-001", "{data}");
    assert_eq!(data["sprints"][0]["id"], "X1", "{data}");
    let text = data["sprints"][0]["sessions"][0]["text"].as_str().unwrap_or_default();
    assert!(text.contains("Sessiontext"), "{data}");
    let list = ui(&app, "PlanningList", json!([])).expect("PlanningList");
    assert!(list["sprints"][0]["sessions"][0].get("text").is_none(), "{list}");
    let items = ui(&app, "PlanningItems", json!([["X1.1", "B-404"]])).expect("PlanningItems");
    assert_eq!(items[0]["kind"], "session", "{items}");
    assert!(!items[1]["error"].as_str().unwrap_or_default().is_empty(), "{items}");
    assert_eq!(ui(&app, "PlanningItems", json!([null])), Ok(json!([])));
    assert_eq!(ui(&app, "PlanningDocs", json!([])), Ok(json!(["glossar"])));
    assert_eq!(ui(&app, "PlanningDoc", json!(["glossar"])), Ok(json!("# Glossar\n")));
    assert!(ui(&app, "PlanningDoc", json!(["../x"])).is_err());
    let got = ui(&app, "PlanningGet", json!(["B-001"])).expect("PlanningGet");
    assert!(
        got.as_str()
            .is_some_and(|t| t.starts_with("docs/backlog/B-001-alt.md\n\n")),
        "{got}"
    );
    ui(&app, "PlanningSet", json!(["B-001", "Prio", "hoch"])).expect("PlanningSet");
    let ticket = fs::read_to_string(root.join("docs/backlog/B-001-alt.md")).expect("Ticket");
    assert!(ticket.contains("**Prio:** hoch"), "{ticket}");
}

/// `WorktreeCheck` auf einem eigenen Wegwerf-Repo, nie auf dem umgebenden Checkout.
#[tokio::test(flavor = "multi_thread")]
async fn worktree_check_auf_eigenem_repo() {
    let fixture = Fixture::empty();
    let root = fixture.tmp.path().join("work");
    planning_files(&root);
    fixture.sh(&root, &["init", "-q"]).await;
    let top = fixture.sh(&root, &["rev-parse", "--show-toplevel"]).await;
    assert!(top.ends_with("/work"), "{top}");
    fixture.sh(&root, &["add", "-A"]).await;
    fixture.sh(&root, &["commit", "-q", "-m", "init"]).await;
    let wt = fixture.tmp.path().join("wt");
    let target = wt.display().to_string();
    fixture
        .sh(&root, &["worktree", "add", "-q", "-b", "claude/x", &target])
        .await;
    write(
        &wt,
        "docs/backlog/B-002-neu.md",
        "# B-002 · Neu\n\n- **Status:** offen\n",
    );
    for state in ["geplant", "erledigt"] {
        fs::create_dir_all(wt.join("docs").join("sprints").join(state)).expect("Sprint-Ordner");
    }
    let app = ready(&root);
    let diffs = tokio::task::spawn_blocking(move || ui(&app, "WorktreeCheck", json!([])))
        .await
        .expect("Aufruf")
        .expect("WorktreeCheck");
    assert_eq!(diffs.as_array().map(Vec::len), Some(1), "{diffs}");
    assert_eq!(diffs[0]["branch"], "claude/x", "{diffs}");
    assert_eq!(
        diffs[0]["changes"],
        json!([{"kind": "neu", "item": "Ticket", "id": "B-002", "text": "Neu"}])
    );
    assert!(diffs[0].get("error").is_none(), "{diffs}");
}

/// Nutzlasten der `planning:changed`-Ereignisse.
fn watching(root: &Path) -> (Arc<App>, Receiver<Value>) {
    let (sender, seen) = mpsc::channel();
    let sender = Mutex::new(sender);
    let forward: Emit = Arc::new(move |name: &str, data: Value| {
        if name == PLANNING {
            // Nach dem Ende des Tests hört niemand mehr zu.
            let _ = sender.lock().unwrap_or_else(PoisonError::into_inner).send(data);
        }
    });
    let app = App::new(root.to_path_buf(), 0, Some(forward)).with_usage_file(root.join("usage.json"));
    (Arc::new(app), seen)
}

/// AC-03: Eine geänderte Planungsdatei löst `planning:changed` mit ihrer ID aus; nach `shutdown` ist Ruhe.
#[tokio::test(flavor = "multi_thread")]
async fn aenderung_loest_planning_changed_aus() {
    let tmp = tmp_dir();
    planning_files(tmp.path());
    let (app, seen) = watching(tmp.path());
    app.startup().await;
    write(tmp.path(), "docs/backlog/B-001-alt.md", &format!("{TICKET}\nmehr\n"));
    let (found, seen) = tokio::task::spawn_blocking(move || (seen.recv_timeout(Duration::from_secs(10)), seen))
        .await
        .expect("warten");
    assert_eq!(found, Ok(json!({"ids": ["B-001"]})));
    app.shutdown().await;
    drop(app);
    write(tmp.path(), "docs/backlog/B-002-neu.md", "# B-002 · Neu\n");
    // Der Wächter ist beendet und mit ihm der letzte Sender.
    assert!(seen.try_recv().is_err());
}
