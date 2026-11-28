//! Port von `tools_playtest_test.go` auf Wegwerf-Wurzeln unter `.work/` (Planung mit Kopien der Vorlagen). ffmpeg ist
//! immer eine Attrappe oder wird nie erreicht.

use std::path::Path;
use std::sync::Arc;

use serde_json::{Value, json};

use super::super::planning::tests::plan_repo;
use super::super::test_client::Client;
use super::super::{Config, Server};
use super::PlaytestHooks;
use crate::playtest::{Error, FindingPatch, LookPath, Proc, Spawn, Store};

const SESSION: &str = "2026-11-22-1";

/// Ein Planungs-Repo mit einer Session samt Transkript, Markern und Bild.
fn playtest_repo() -> tempfile::TempDir {
    let tmp = plan_repo();
    let dir = tmp.path().join("playtests").join(SESSION);
    std::fs::create_dir_all(dir.join("frames")).expect("Session");
    let marker = concat!(
        r#"{"ts":1005000,"kind":"pad","player":1,"world":"standard","island":2,"stage":3,"phase":"night","#,
        r#""day":4,"worst_frame_ms":12.5}"#,
        "\n"
    );
    let transcript = concat!(
        r#"{"audioStartMs":1000000,"language":"de","segments":["#,
        r#"{"startMs":1004000,"endMs":1006000,"text":"Ich finde den Bauarbeiter nicht"},"#,
        r#"{"startMs":1030000,"endMs":1031000,"text":"Satz zwei"},"#,
        r#"{"startMs":1040000,"endMs":1041000,"text":"Satz drei"}]}"#
    );
    for (name, body) in [
        (
            "session.json",
            r#"{"startMs":1002000,"endMs":1100000,"captureMs":1000000,"stand":"aufgenommen"}"#,
        ),
        ("voice.wav", "x"),
        ("video.mkv", "x"),
        ("markers.jsonl", marker),
        ("survey.json", r#"{"spass": 4}"#),
        ("transcript.json", transcript),
        ("frames/marker-1.jpg", "jpg"),
    ] {
        std::fs::write(dir.join(name), body).expect("Datei");
    }
    tmp
}

async fn connect(root: &Path, hooks: PlaytestHooks) -> (Server, Client) {
    let server = Server::new(Config {
        root: root.to_owned(),
        version: "test".to_owned(),
        playtest: hooks,
        ..Config::default()
    });
    server.start().await.expect("Start");
    let client = Client::connect(server.url()).await;
    (server, client)
}

async fn ok(client: &mut Client, name: &str, arguments: Value) -> String {
    let (text, is_error) = client.call_text(name, arguments).await;
    assert!(!is_error, "{name}: {text}");
    text
}

#[tokio::test(flavor = "multi_thread")]
async fn playtest_tools_lesen() {
    let repo = playtest_repo();
    let (server, mut client) = connect(repo.path(), PlaytestHooks::default()).await;
    let list = ok(&mut client, "playtest_list", json!({})).await;
    assert_eq!(
        list,
        "1 Spieltests (neueste zuerst):\n- 2026-11-22-1 · transkribiert · 1 min · 1 Marker · 0 Befunde · Sprache ja, \
         Video ja\n"
    );
    let got = ok(&mut client, "playtest_get", json!({"id": SESSION})).await;
    let frame = repo
        .path()
        .join("playtests")
        .join(SESSION)
        .join("frames")
        .join("marker-1.jpg");
    let marker = format!(
        "[00:05 ts=1005000] Marker #1 Marker · Spieler 1 · Insel 2 Stufe 3 · Welt \"standard\" Phase night Tag 4 · \
         schlechtester Frame 12.5 ms · Bild {}",
        frame.components().collect::<std::path::PathBuf>().display()
    );
    for want in [
        "Fragebogen: {\"spass\": 4}",
        "Sprache: Ich finde den Bauarbeiter nicht",
        marker.as_str(),
        "Zeitleiste 1–4 von 4",
    ] {
        assert!(got.contains(want), "playtest_get ohne {want:?}: {got}");
    }
    let first = ok(&mut client, "playtest_get", json!({"id": SESSION, "limit": 2})).await;
    assert!(
        first.contains("Zeitleiste 1–2 von 4") && first.contains("weiter mit cursor=2"),
        "{first}"
    );
    let rest = ok(
        &mut client,
        "playtest_get",
        json!({"id": SESSION, "limit": 2, "cursor": 2}),
    )
    .await;
    assert!(rest.contains("Zeitleiste 3–4 von 4"), "{rest}");
    assert!(!rest.contains("Fragebogen") && !rest.contains("weiter mit"), "{rest}");
    let (text, is_error) = client.call_text("playtest_get", json!({"id": "2026-01-01-9"})).await;
    assert!(is_error && text.contains("nicht gefunden"), "{text}");
    let (text, is_error) = client.call_text("playtest_get", json!({"id": "../x"})).await;
    assert!(is_error, "ungültige ID: {text}");
    server.stop().await.expect("Stopp");
}

/// Legt Projekt, Sprint und ein Ticket an und liefert dessen Nummer.
async fn planning(client: &mut Client) -> String {
    let steps = [
        json!({"kind": "projekt", "id": "PXX", "slug": "px", "title": "Projekt X"}),
        json!({"kind": "sprint", "id": "X1", "slug": "x", "title": "X",
            "fields": {"Domäne": "DEV", "Projekt": "PXX"}}),
        json!({"kind": "ticket", "slug": "alt", "title": "Alt",
            "fields": {"Domäne": "DEV", "Typ": "Problem", "Prio": "mittel", "Projekt": "PXX", "Sprint": "X1"}}),
    ];
    let mut last = String::new();
    for step in steps {
        last = ok(client, "plan_create", step).await;
    }
    let id = last.split_whitespace().next().expect("Ticket-ID");
    assert!(id.starts_with("B-"), "{last}");
    id.to_owned()
}

/// Haken setzt Ralf auf der Seite.
fn tick(root: &Path, ids: &[&str]) {
    let store = Store {
        root: root.join("playtests"),
        active: None,
    };
    let patch = FindingPatch {
        selected: Some(true),
        ..FindingPatch::default()
    };
    for id in ids {
        store.update_finding(SESSION, id, &patch).expect("Haken");
    }
}

fn read(root: &Path, relative: &str) -> String {
    std::fs::read_to_string(root.join(relative)).expect(relative)
}

fn new_ticket_file(root: &Path, id: &str) -> String {
    let dir = root.join("docs").join("backlog");
    let entry = std::fs::read_dir(&dir)
        .expect("Backlog")
        .flatten()
        .find(|e| e.file_name().to_string_lossy().starts_with(&format!("{id}-")))
        .expect("Ticket-Datei");
    std::fs::read_to_string(entry.path()).expect("Ticket")
}

#[tokio::test(flavor = "multi_thread")]
async fn playtest_findings_und_tickets() {
    let repo = playtest_repo();
    let root = repo.path();
    let (server, mut client) = connect(root, PlaytestHooks::default()).await;
    let old = planning(&mut client).await;
    let findings = json!({"id": SESSION, "findings": [
        {"id": "f1", "quote": "Ich finde den Bauarbeiter nicht", "ts": 1_005_000, "type": "Problem",
            "frame": "frames/marker-1.jpg", "context": "Insel 2 Stufe 3"},
        {"id": "f2", "quote": "Satz zwei", "ts": 1_030_000, "type": "Idee", "duplicate_of": old},
        {"id": "f3", "quote": "nicht angehakt", "ts": 1_040_000, "type": "Idee"},
    ]});
    let text = ok(&mut client, "playtest_findings_set", findings).await;
    assert!(text.contains("3 neu") && text.contains("Checkout:"), "{text}");
    for bad in [
        json!({"id": "f9", "quote": "x", "ts": 1, "type": "Bug"}),
        json!({"id": "f9", "quote": "x", "ts": 1, "selected": true}),
    ] {
        let params = json!({"name": "playtest_findings_set", "arguments": {"id": SESSION, "findings": [bad]}});
        let reply = client.request("tools/call", params).await;
        assert!(
            reply["isError"] == json!(true) || reply.is_null(),
            "angenommen: {reply}"
        );
    }
    tick(root, &["f1", "f2"]);
    let arguments = json!({"id": SESSION, "domain": "REG", "project": "PXX", "sprint": "X1"});
    let text = ok(&mut client, "playtest_tickets_create", arguments).await;
    assert!(
        text.starts_with(
            "Tickets: 1 angelegt, 1 Hinweise in vorhandenen Tickets, 0 fehlgeschlagen, 1 übersprungen (nicht angehakt \
             oder schon erledigt)\n- f1 → B-"
        ),
        "{text}"
    );
    let stored: Value = serde_json::from_str(&read(root, &format!("playtests/{SESSION}/findings.json"))).expect("JSON");
    let id = stored["findings"][0]["ticket"]
        .as_str()
        .expect("Ticket-ID in findings.json");
    let ticket = new_ticket_file(root, id);
    for want in ["PXX", "X1", "Ich finde den Bauarbeiter nicht", "Insel 2 Stufe 3"] {
        assert!(ticket.contains(want), "Ticket ohne {want}: {ticket}");
    }
    assert!(new_ticket_file(root, &old).contains("Befund f2"), "Hinweis fehlt");
    let got = ok(&mut client, "playtest_get", json!({"id": SESSION, "limit": 1})).await;
    assert!(
        got.contains("f1 [angehakt] → B-") && got.contains("Duplikat von"),
        "{got}"
    );
    server.stop().await.expect("Stopp");
}

#[tokio::test(flavor = "multi_thread")]
async fn playtest_frame_unbekannte_session() {
    let repo = playtest_repo();
    // Attrappen, damit nie die echte Suche läuft.
    let (look_path, spawn): (LookPath, Spawn) = (Arc::new(|_| Err(Error::NoGame)), Arc::new(|_, _| Err(Error::NoGame)));
    let hooks = PlaytestHooks {
        ffmpeg: Some((look_path, spawn)),
        ..PlaytestHooks::default()
    };
    let (server, mut client) = connect(repo.path(), hooks).await;
    let (text, is_error) = client
        .call_text("playtest_frame", json!({"id": "2026-01-01-9", "t": 5}))
        .await;
    assert!(is_error && text.contains("nicht gefunden"), "{text}");
    server.stop().await.expect("Stopp");
}

/// Attrappe: schreibt das Zielbild und ist fertig.
struct Done;

impl Proc for Done {
    fn wait(&self) -> Result<(), String> {
        Ok(())
    }
    fn quit(&self) {}
    fn kill(&self) {}
    fn tail(&self) -> String {
        String::new()
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn playtest_frame_nennt_pfad_und_checkout() {
    let repo = playtest_repo();
    let look_path: LookPath = Arc::new(|_| Ok("ffmpeg".to_owned()));
    let spawn: Spawn = Arc::new(|args, dir| {
        std::fs::write(dir.join(args.last().expect("Ziel")), "jpg").expect("Bild");
        Ok(Arc::new(Done) as Arc<dyn Proc>)
    });
    let hooks = PlaytestHooks {
        ffmpeg: Some((look_path, spawn)),
        ..PlaytestHooks::default()
    };
    let (server, mut client) = connect(repo.path(), hooks).await;
    let text = ok(&mut client, "playtest_frame", json!({"id": SESSION, "t": 1_012_000})).await;
    let path = repo
        .path()
        .join("playtests")
        .join(SESSION)
        .join("frames")
        .join("t-1012000.jpg");
    let path = path.components().collect::<std::path::PathBuf>();
    let start = format!("Bild: {}\nCheckout: {}", path.display(), repo.path().display());
    assert!(text.starts_with(&start), "{text}");
    server.stop().await.expect("Stopp");
}

/// Die laufende Aufnahme der App gilt für die Ablage der Repo-Wurzel.
#[tokio::test(flavor = "multi_thread")]
async fn ablage_kennt_die_aufnahme_der_app() {
    let repo = playtest_repo();
    let session = repo.path().join("playtests").join(SESSION).join("session.json");
    std::fs::write(&session, r#"{"startMs":1,"stand":"laeuft"}"#).expect("session.json");
    for (active, stage) in [(SESSION, "läuft"), ("", "unvollständig")] {
        let hooks = PlaytestHooks {
            active: Some(Arc::new(move || active.to_owned())),
            ..PlaytestHooks::default()
        };
        let (server, mut client) = connect(repo.path(), hooks).await;
        let list = ok(&mut client, "playtest_list", json!({})).await;
        assert!(list.contains(&format!("{SESSION} · {stage}")), "{list}");
        server.stop().await.expect("Stopp");
    }
}
