//! Go `roundtrip_test.go` › `TestPlanungsToolsRoundtrip`, `tools_planning_{digest,index,waves}_test.go` und der
//! Abgleich mit den Golden (`testdata/planung/`). Alle Wurzeln liegen unter `.work/` und stehen ausdrücklich in der
//! `Config`.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::super::test_client::Client;
use super::super::{Config, INSTRUCTIONS, Server};
use crate::testutil::tmp_dir;

const PLAN_INDEX: &str = "## Offen\n\n| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |\n|---|---|---|---|---|---|---|\n\n\
## Archiv\n\n| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |\n|---|---|---|---|---|---|---|\n";
const ROADMAP: &str = "## Aktiv\n\n| Sprint | Domäne | Thema | Am Ende sichtbar | Ordner |\n|---|---|---|---|---|\n\n\
## Geplant\n\n| Sprint | Domäne | Thema | Am Ende sichtbar | Reife | Ordner |\n|---|---|---|---|---|---|\n\n## Erledigt\n";
const PROJECTS: &str = "# Projekte\n\n## Aktiv\n\n| Rang | Projekt | Ziel | Datei |\n|---|---|---|---|\n\n## Ruht\n\n\
| Projekt | Grund | Datei |\n|---|---|---|\n\n## Erledigt\n\n| Projekt | Datei |\n|---|---|\n";

fn repo() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(4);
    root.expect("Repo-Wurzel").to_owned()
}

fn write(root: &Path, rel: &str, text: &str) {
    super::super::test_helpers::write(&root.join(rel), text);
}

/// Ein docs/ mit den echten Vorlagen (nur gelesen), leerem Index und Fahrplan.
fn plan_repo() -> tempfile::TempDir {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(root, "docs/backlog/README.md", PLAN_INDEX);
    write(root, "docs/sprints/README.md", ROADMAP);
    write(root, "docs/projekte/README.md", PROJECTS);
    for kind in ["ticket", "sprint", "session", "projekt"] {
        let template = repo().join("docs").join("vorlagen").join(format!("{kind}.md"));
        let text = fs::read_to_string(template).expect("Vorlage");
        write(root, &format!("docs/vorlagen/{kind}.md"), &text);
    }
    for state in ["aktiv", "geplant", "erledigt"] {
        fs::create_dir_all(root.join("docs").join("sprints").join(state)).expect("Sprint-Ordner");
    }
    tmp
}

async fn connect(root: &Path) -> (Server, Client) {
    let server = Server::new(Config {
        root: root.to_owned(),
        version: "test".to_owned(),
        ..Config::default()
    });
    server.start().await.expect("Start");
    let client = Client::connect(server.url()).await;
    (server, client)
}

/// Die Textteile einer Antwort und `isError`.
async fn parts(client: &mut Client, name: &str, arguments: Value) -> (Vec<String>, bool) {
    let params = json!({"name": name, "arguments": arguments});
    let result = client.request("tools/call", params).await;
    let content = result["content"].as_array().cloned().unwrap_or_default();
    let texts = content
        .iter()
        .filter_map(|part| part["text"].as_str().map(str::to_owned))
        .collect();
    (texts, result["isError"].as_bool().unwrap_or(false))
}

async fn expect(client: &mut Client, name: &str, arguments: Value, want: &str) -> String {
    let (text, is_error) = client.call_text(name, arguments).await;
    assert!(
        !is_error && text.contains(want),
        "{name}: {text:?} (Fehler {is_error}), erwartet {want:?}"
    );
    text
}

/// Aufruf, Argumente und ein Teil der erwarteten Antwort in dieser Reihenfolge.
fn roundtrip_steps() -> [(&'static str, Value, &'static str); 11] {
    let ticket = |slug: &str, title: &str, domain: &str, prio: &str| {
        json!({"kind": "ticket", "slug": slug, "title": title,
            "fields": {"Domäne": domain, "Typ": "Idee", "Prio": prio, "Projekt": "PXX", "Sprint": "X1"}})
    };
    [
        (
            "plan_create",
            json!({"kind": "projekt", "id": "PXX", "slug": "px", "title": "Projekt X"}),
            "PXX angelegt",
        ),
        (
            "plan_create",
            json!({"kind": "sprint", "id": "X1", "slug": "x", "title": "X",
                "fields": {"Domäne": "SRV", "Projekt": "PXX"}}),
            "X1 angelegt",
        ),
        ("plan_create", ticket("neu", "Neu", "SRV", "hoch"), "B-001 angelegt"),
        (
            "plan_section",
            json!({"id": "B-001", "section": "Ziel", "text": "Ein Ziel."}),
            "Ziel ersetzt",
        ),
        (
            "plan_sections",
            json!({"id": "B-001", "sections": {"Ziel": "Ein Ziel.", "Ausgangslage": "Heute."}}),
            "B-001 · Ausgangslage, Ziel ersetzt",
        ),
        (
            "plan_next",
            json!({}),
            "Keine Session frei. Gründe je Projekt:\n- PXX: kein aktiver Sprint",
        ),
        (
            "plan_set",
            json!({"id": "B-001", "fields": {"Prio": "niedrig"}}),
            "B-001 geändert",
        ),
        ("plan_get", json!({"id": "B-001"}), "## Ziel\n\nEin Ziel."),
        (
            "plan_create",
            ticket("dev", "Werkzeug", "DEV", "mittel"),
            "B-002 angelegt",
        ),
        ("plan_list", json!({"domain": "DEV"}), "B-002 DEV Idee mittel"),
        ("plan_list", json!({"domain": "SRV"}), "B-001 SRV Idee niedrig ?"),
    ]
}

#[tokio::test(flavor = "multi_thread")]
async fn planungs_tools_roundtrip() {
    let tmp = plan_repo();
    let (server, mut client) = connect(tmp.path()).await;
    for (tool, arguments, want) in roundtrip_steps() {
        expect(&mut client, tool, arguments, want).await;
    }
    let loose = json!({"kind": "ticket", "slug": "lose", "title": "Lose",
        "fields": {"Domäne": "DEV", "Typ": "Idee", "Prio": "mittel"}});
    let (text, is_error) = parts(&mut client, "plan_create", loose).await;
    assert!(is_error, "{text:?}");
    // Der Text von Go, danach die Checkout-Zeile.
    assert_eq!(
        text[0],
        "neues Ticket ohne Projekt und Sprint: es gibt keine ungeplanten Tickets, Projekt und Sprint gleich \
         mitgeben (fehlt der Sprint, zuerst plan_create kind sprint)"
    );
    assert!(text[1].starts_with("Checkout: Repo-Wurzel "), "{text:?}");
    let dash = json!({"kind": "ticket", "slug": "lose", "title": "Lose",
        "fields": {"Domäne": "DEV", "Typ": "Idee", "Prio": "mittel", "Projekt": "PXX", "Sprint": " – "}});
    let (text, is_error) = client.call_text("plan_create", dash).await;
    assert!(is_error && text.starts_with("neues Ticket ohne Sprint:"), "{text}");
    let (text, is_error) = client
        .call_text("plan_set", json!({"id": "X1", "fields": {"Prio": "hoch"}}))
        .await;
    assert!(is_error && text.contains("unbekanntes Feld"), "{text}");
    expect(&mut client, "plan_delete", json!({"id": "X1"}), "X1 gelöscht").await;
    let (text, is_error) = client.call_text("plan_delete", json!({"id": "B-001"})).await;
    assert!(is_error, "Ticket gelöscht: {text}");
    server.stop().await.expect("Stopp");
}

#[tokio::test(flavor = "multi_thread")]
async fn unbekannter_parameter_nennt_die_gueltigen() {
    let tmp = plan_repo();
    let (server, mut client) = connect(tmp.path()).await;
    let (text, is_error) = client.call_text("plan_get", json!({"nr": "B-001"})).await;
    assert!(is_error && text.contains("gültige Parameter: id"), "{text}");
    server.stop().await.expect("Stopp");
}

#[tokio::test(flavor = "multi_thread")]
async fn plan_get_mit_kurzname_bei_doppelter_nummer() {
    let tmp = plan_repo();
    for slug in ["erstes", "zweites"] {
        let text = format!("# B-123 · {slug}\n\n- **Status:** offen\n");
        write(tmp.path(), &format!("docs/backlog/B-123-{slug}.md"), &text);
    }
    let (server, mut client) = connect(tmp.path()).await;
    let text = expect(
        &mut client,
        "plan_get",
        json!({"id": "B-123-zweites"}),
        "# B-123 · zweites",
    )
    .await;
    assert!(text.starts_with("docs/backlog/B-123-zweites.md"), "{text}");
    // Ohne Kurzname gilt wie in Go die erste Datei.
    expect(&mut client, "plan_get", json!({"id": "B-123"}), "B-123-erstes.md").await;
    server.stop().await.expect("Stopp");
}

/// Alle Dateien unter `dir` als Pfad ab `dir` (mit `/`) und ihr Inhalt mit LF.
fn tree(dir: &Path) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::new();
    let mut open = vec![dir.to_owned()];
    while let Some(folder) = open.pop() {
        for entry in fs::read_dir(folder).into_iter().flatten() {
            let path = entry.expect("Eintrag").path();
            if path.is_dir() {
                open.push(path);
                continue;
            }
            let rel = path.strip_prefix(dir).expect("unter dir");
            let text = fs::read_to_string(&path).expect("lesen");
            out.insert(rel.to_string_lossy().replace('\\', "/"), text.replace("\r\n", "\n"));
        }
    }
    out
}

/// Wie die Maske des Golden-Läufers: LF, das Datum der Zeile „Erstellt“ und die Wurzel.
fn mask(text: &str, root: &Path) -> String {
    let path = root.to_string_lossy().into_owned();
    let today = chrono::Local::now().format("%Y-%m-%d");
    text.replace("\r\n", "\n")
        .replace(&path, "<WURZEL>")
        .replace(&path.replace('\\', "/"), "<WURZEL>")
        .replace(&format!("**Erstellt:** {today}"), "**Erstellt:** <HEUTE>")
}

/// Ruft das Werkzeug eines Golden-Falls per MCP und vergleicht Antwort und Dateibaum.
async fn golden(case: &str, writes: bool) {
    let golden = repo().join("testdata").join("planung");
    let tmp = tmp_dir();
    let root = tmp.path();
    for (rel, text) in tree(&golden.join(case).join("vorher")) {
        write(root, &rel, &text);
    }
    for (rel, text) in tree(&golden.join("_vorlagen")) {
        write(root, &format!("docs/vorlagen/{rel}"), &text);
    }
    for state in ["aktiv", "geplant", "erledigt"] {
        fs::create_dir_all(root.join("docs").join("sprints").join(state)).expect("Sprint-Ordner");
    }
    let call = fs::read_to_string(golden.join(case).join("aufruf.json")).expect("aufruf.json");
    let call: Value = serde_json::from_str(&call).expect("JSON");
    let tool = call["werkzeug"].as_str().expect("werkzeug");
    let (server, mut client) = connect(root).await;
    let (text, is_error) = parts(&mut client, tool, call["args"].clone()).await;
    server.stop().await.expect("Stopp");
    assert!(!is_error, "{case}: {text:?}");
    let want = fs::read_to_string(golden.join(case).join("antwort.txt")).expect("antwort.txt");
    assert_eq!(mask(&text[0], root), want.replace("\r\n", "\n"), "{case}");
    assert_eq!(text.len(), if writes { 2 } else { 1 }, "{case}: {text:?}");
    if writes {
        assert_eq!(text[1], format!("Checkout: Repo-Wurzel {}", root.display()), "{case}");
    }
    let mut after: std::collections::BTreeMap<String, String> = tree(&root.join("docs"))
        .into_iter()
        .filter(|(rel, _)| !rel.starts_with("vorlagen/"))
        .map(|(rel, text)| (format!("docs/{rel}"), mask(&text, root)))
        .collect();
    for (rel, text) in tree(&golden.join(case).join("nachher")) {
        assert_eq!(after.remove(&rel), Some(text), "{case}: {rel}");
    }
    assert!(after.is_empty(), "{case}: Dateien zu viel: {:?}", after.keys());
}

#[tokio::test(flavor = "multi_thread")]
async fn golden_ueber_mcp() {
    golden("get-ticket", false).await;
    golden("ticket-prio-aendern", true).await;
    golden("ticket-anlegen-mit-sprint-und-projekt", true).await;
}

/// B-783: `plan_waves`, `plan_queue` und `plan_check` sind per MCP erreichbar und stehen in den Instruktionen.
#[tokio::test(flavor = "multi_thread")]
async fn plan_waves_queue_und_check_ueber_mcp() {
    let tmp = plan_repo();
    let (server, mut client) = connect(tmp.path()).await;
    let cases = [
        (
            "plan_waves",
            "Arbeitswellen: 0 in Arbeit, 0 eingeplant in 1 Wellen (1 gezeigt), 0 in keiner Welle · Lanes: zweit (Second \
             Worker), haupt (Hauptrechner)",
        ),
        (
            "plan_queue",
            "Ralf-Queue: 0 Freigaben · 0 Entscheidungen · 0 Abnahmen an 0 Geräten",
        ),
    ];
    for (tool, want) in cases {
        let text = expect(&mut client, tool, json!({}), want).await;
        assert!(text.starts_with(want), "{tool}: {text}");
    }
    let (text, is_error) = client.call_text("plan_waves", json!({"wave_count": -1})).await;
    assert!(is_error && text.contains("mindestens 1"), "{text}");
    let (text, is_error) = client.call_text("plan_check", json!({"id": "B-404"})).await;
    assert!(is_error && text.contains("nicht gefunden"), "{text}");
    for name in ["`plan_waves", "`plan_queue", "`plan_check"] {
        assert!(INSTRUCTIONS.contains(name), "Instruktionen nennen {name} nicht");
    }
    server.stop().await.expect("Stopp");
}

/// B-696/AC-11: ohne `dry` zeigt `plan_digest` nur an, erst `dry: false` schreibt.
#[tokio::test(flavor = "multi_thread")]
async fn plan_digest_dry_ist_standard() {
    let tmp = plan_repo();
    write(
        tmp.path(),
        "docs/sprints/erledigt/Z1-alt/README.md",
        "# Z1 · DEV · Alt\n\n- **Status:** erledigt\n\n## Ziel\n\nEin Ziel.\n",
    );
    let summary = tmp.path().join("docs/sprints/erledigt/Z1-alt/ZUSAMMENFASSUNG.md");
    let (server, mut client) = connect(tmp.path()).await;
    for arguments in [json!({}), json!({"dry": null}), json!({"dry": true})] {
        let (text, is_error) = parts(&mut client, "plan_digest", arguments.clone()).await;
        assert!(!is_error, "{text:?}");
        assert!(
            text[0].starts_with("Zusammenfassungen:") && text[0].contains("Z1"),
            "{text:?}"
        );
        assert!(text[1].starts_with("Checkout: "), "{text:?}");
        assert!(!summary.exists(), "{arguments}: geschrieben trotz dry");
    }
    expect(&mut client, "plan_digest", json!({"dry": false}), "Zusammenfassungen:").await;
    assert!(summary.exists(), "dry false schreibt nicht");
    assert!(INSTRUCTIONS.contains("`plan_digest"), "Instruktionen");
    server.stop().await.expect("Stopp");
}

/// B-864: ohne `dry` ändert `plan_index` nichts, erst `dry: false` schreibt.
#[tokio::test(flavor = "multi_thread")]
async fn plan_index_dry_ist_standard() {
    let tmp = plan_repo();
    write(
        tmp.path(),
        "docs/backlog/B-005-fehlt.md",
        "# B-005 · Fehlt im Index\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** mittel\n- **Status:** offen\n",
    );
    let index = tmp.path().join("docs/backlog/README.md");
    let (server, mut client) = connect(tmp.path()).await;
    for arguments in [json!({}), json!({"dry": null}), json!({"dry": true})] {
        let (text, is_error) = parts(&mut client, "plan_index", arguments.clone()).await;
        assert!(!is_error && text[1].starts_with("Checkout: "), "{text:?}");
        let now = fs::read_to_string(&index).expect("Index");
        assert_eq!(now, PLAN_INDEX, "{arguments}: geschrieben trotz dry");
    }
    let (text, is_error) = client.call_text("plan_index", json!({"dry": false})).await;
    assert!(!is_error, "{text}");
    let now = fs::read_to_string(&index).expect("Index");
    assert!(now.contains("B-005"), "{now}");
    assert!(INSTRUCTIONS.contains("`plan_index"), "Instruktionen");
    server.stop().await.expect("Stopp");
}

/// Schreibende Tools nehmen `checkout`; lesende nicht.
#[tokio::test(flavor = "multi_thread")]
async fn checkout_nur_bei_schreibenden_planungs_tools() {
    let tmp = plan_repo();
    let (server, mut client) = connect(tmp.path()).await;
    let writing = [
        "plan_create",
        "plan_set",
        "plan_section",
        "plan_sections",
        "plan_delete",
        "plan_digest",
        "plan_index",
    ];
    let mut seen = 0;
    for tool in client.tools().await {
        let name = tool["name"].as_str().expect("name");
        if !name.starts_with("plan_") {
            continue;
        }
        seen += 1;
        let takes = tool["inputSchema"]["properties"]["checkout"].is_object();
        assert_eq!(takes, writing.contains(&name), "{name}");
    }
    assert_eq!(seen, 13);
    server.stop().await.expect("Stopp");
}
