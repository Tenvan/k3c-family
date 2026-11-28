//! Go `app_ticket_test.go` über die Aufruf-Tabelle. Vorlage und Index kommen nur lesend aus diesem Repo in eine
//! Wurzel unter `.work/`.

use std::fs;
use std::path::Path;
use std::sync::Arc;

use serde_json::{Value, json};

use crate::app::App;
use crate::planning;
use crate::rpc::call;
use crate::testutil::tmp_dir;

fn ticket_repo() -> (tempfile::TempDir, Arc<App>) {
    let tmp = tmp_dir();
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("Repo-Wurzel");
    for file in ["docs/vorlagen/ticket.md", "docs/backlog/README.md"] {
        let target = tmp.path().join(file);
        fs::create_dir_all(target.parent().expect("Elternordner")).expect("Ordner");
        fs::copy(repo.join(file), target).expect("kopieren");
    }
    // Seed weit über dem echten Backlog: die nächste Nummer kollidiert nie mit dem kopierten Index.
    let seed = tmp.path().join("docs/backlog/B-9998-neuer.md");
    fs::write(seed, "# B-9998 · Neueres Ticket\n").expect("Seed");
    let app = App::new(tmp.path().to_path_buf(), 0, None).with_usage_file(tmp.path().join("usage.json"));
    app.mark_ready();
    (tmp, Arc::new(app))
}

fn ui(app: &Arc<App>, method: &str, args: &[Value]) -> Result<Value, String> {
    call(app, method, args).map_err(|e| e.0)
}

#[test]
fn ticket_create_und_recent() {
    let (tmp, app) = ticket_repo();
    let input = json!({"titel": "Probe aus dem Test", "beschreibung": "Text", "typ": "Idee", "prio": "niedrig",
        "domaene": "DEV",
        "kontext": {"app": "k3c-workbench", "seite": "grafik", "zeit": "2026-10-24 12:00"}});
    let created = ui(&app, "TicketCreate", &[input]).expect("anlegen");
    assert_eq!(created["id"], "B-9999");
    assert_eq!(created["titel"], "Probe aus dem Test");
    let path = created["pfad"].as_str().expect("pfad");
    assert!(tmp.path().join(path).is_file(), "Ticketdatei fehlt: {path}");
    let recent = ui(&app, "TicketRecent", &[json!(0)]).expect("recent");
    assert_eq!(recent.as_array().map(Vec::len), Some(2), "{recent}");
    assert_eq!(recent[0]["id"], created["id"]);
    let one = ui(&app, "TicketRecent", &[json!(1)]).expect("recent 1");
    assert_eq!(one.as_array().map(Vec::len), Some(1), "{one}");
    assert!(
        ui(&app, "TicketCreate", &[json!({})]).is_err(),
        "leere Eingabe angenommen"
    );
}

/// Ein Ticket vom Ticket-Button steht im Haupt-Index; erledigt wandert seine Zeile in den Archiv-Index und zurück
/// (B-696/AC-09).
#[test]
fn ticket_button_und_archiv_index() {
    let (tmp, app) = ticket_repo();
    let root = tmp.path();
    let input = json!({"titel": "Knopf-Ticket", "typ": "Idee", "prio": "niedrig", "domaene": "DEV"});
    let created = ui(&app, "TicketCreate", &[input]).expect("anlegen");
    let id = created["id"].as_str().expect("id").to_owned();
    let row = format!("| [{id}](");
    let listed = |index: &str| {
        let text = fs::read_to_string(root.join("docs").join(index)).unwrap_or_default();
        text.contains(&row)
    };
    let (main, archive) = ("backlog/README.md", "backlog/archiv/README.md");
    assert!(listed(main), "nicht im Haupt-Index");
    planning::rebuild_index(root, false).expect("neues Format mit Archiv-Index");
    let status = |value: &str| {
        let args = [json!(id), json!("Status"), json!(value)];
        ui(&app, "PlanningSet", &args).expect("PlanningSet");
    };
    status("erledigt");
    assert!(!listed(main) && listed(archive), "Zeile nicht im Archiv-Index");
    status("offen");
    assert!(listed(main) && !listed(archive), "Zeile nicht zurück im Haupt-Index");
}
