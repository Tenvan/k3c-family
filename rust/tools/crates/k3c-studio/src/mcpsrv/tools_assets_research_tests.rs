//! Port von `tools_assets_research_test.go` und des Rests von `tools_assets_slots_test.go` (die Recherche bleibt). Netz
//! ersetzt ein lokaler Server mit einer OpenGameArt-Zeile; kein Test erreicht das Internet.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use k3c_assetkit::{Searcher, hits_of};
use serde_json::json;

use super::super::checkout_policy::write_tool;
use super::super::test_helpers::{connect, connect_mit, enthaelt, lies};
use super::SearcherFactory;
use crate::paperassets::test_helpers::{schreibe, unter};
use crate::review::test_helpers::repo_root;

const BILD: &str = "public/grafik/town/well.png";
const TOOLS: [&str; 5] = [
    "assets_research",
    "assets_hits",
    "assets_hit_add",
    "assets_hit_drop",
    "assets_import",
];

/// Mini-Repo mit Slot-Register, einem Bild und einer Credit-Zeile (Go `assetRepo`).
fn asset_repo() -> tempfile::TempDir {
    let dir = crate::test_helpers::tmp_dir();
    let register = r#"{"_comment": "Test", "slots": [
  {"id": "farm", "art": "grafik", "kategorie": "gebaeude", "beschreibung": "Farm", "wahl": []},
  {"id": "sfx:coin", "art": "sound", "kategorie": "sfx", "beschreibung": "Münze", "bus": "sfx", "wahl": []}]}"#;
    schreibe(dir.path(), "data/sounds.json", register);
    let credits = "| Ordner | Pack | Urheber | Lizenz | Quelle |\n|---|---|---|---|---|\n| town | Town | Zuno | CC BY-NC 4.0 | https://example.org |\n";
    schreibe(dir.path(), "public/grafik/CREDITS.md", credits);
    let png = std::fs::read(repo_root().join("tools/paper/fixtures/render/monarch/monarch-idle-0.png"));
    schreibe(dir.path(), BILD, png.expect("Fixture"));
    dir
}

/// Beantwortet die Suchseite von OpenGameArt mit einer Zeile, alles andere mit 404.
fn antworte(mut s: TcpStream) {
    let mut anfrage = Vec::new();
    let mut puffer = [0; 1024];
    while !anfrage.windows(4).any(|w| w == b"\r\n\r\n") {
        match s.read(&mut puffer) {
            Ok(0) | Err(_) => return,
            Ok(n) => anfrage.extend_from_slice(&puffer[..n]),
        }
    }
    let (status, text) = if anfrage.starts_with(b"GET /art-search-advanced") {
        let zeile = "<span class=\"art-preview-title\"><a href=\"/content/coin\">Coin</a></span><img src='https://x/coin.png'>\n";
        ("200 OK", zeile)
    } else {
        ("404 Not Found", "")
    };
    let _ = write!(
        s,
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{text}",
        text.len()
    );
}

/// Quellen, die alle auf den lokalen Server zeigen; der Thread endet mit dem Testprozess.
fn lokale_quellen() -> SearcherFactory {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Port");
    let url = format!("http://{}", listener.local_addr().expect("Adresse"));
    thread::spawn(move || {
        for s in listener.incoming().flatten() {
            thread::spawn(move || antworte(s));
        }
    });
    Arc::new(move |_| {
        Ok(Searcher {
            client: reqwest::Client::builder()
                .no_proxy()
                .build()
                .map_err(|e| e.to_string())?,
            freesound: url.clone(),
            commons: url.clone(),
            oga: url.clone(),
            token: String::new(),
        })
    })
}

/// B-298/AC-06: die Recherche-Tools antworten.
#[tokio::test(flavor = "multi_thread")]
async fn assets_research_tools() {
    let dir = asset_repo();
    let quellen = lokale_quellen();
    let mut cs = connect_mit(dir.path(), |s| s.searcher = quellen).await;
    let text = cs.ok("assets_research", json!({"id": "farm", "query": "coin"})).await;
    enthaelt(
        &text,
        &[
            "farm: 1 neu, 1 gespeichert · Hinweise: Wikimedia Commons: HTTP 404",
            "pruefen · OpenGameArt · Lizenz ? · Coin",
            "\nCheckout: ",
        ],
    );
    let add = json!({"id": "farm", "titel": "Farm itch", "link": "https://x.itch.io/farm", "lizenz": "CC0"});
    enthaelt(
        &cs.ok("assets_hit_add", add).await,
        &["farm: Treffer gespeichert", "\nCheckout: "],
    );
    let add = json!({"id": "farm", "titel": "Farm itch", "link": "https://x.itch.io/farm"});
    enthaelt(&cs.ok("assets_hit_add", add).await, &["Link schon gespeichert"]);
    let text = cs.ok("assets_hits", json!({"id": "farm"})).await;
    enthaelt(
        &text,
        &[
            "farm: 2 Treffer\n",
            "erkannt · Agent · CC0 · Farm itch · https://x.itch.io/farm",
        ],
    );
    let hit = text
        .lines()
        .nth(1)
        .and_then(|l| l.split_whitespace().next())
        .expect("Treffer")
        .to_owned();
    let text = cs.ok("assets_hit_drop", json!({"id": "farm", "hit": hit})).await;
    enthaelt(&text, &["verworfen", "\nCheckout: "]);
    let args = json!({"id": "farm", "titel": "x", "link": "http://unsicher"});
    let text = cs.abgelehnt("assets_hit_add", args).await;
    enthaelt(&text, &["link muss mit https:// beginnen: \"http://unsicher\""]);
    let text = cs.ok("assets_hits", json!({"id": "sfx:coin"})).await;
    assert!(text.starts_with("sfx:coin: keine Treffer"), "{text}");
    let args = json!({"id": "nix", "titel": "x", "link": "https://x.example"});
    enthaelt(&cs.abgelehnt("assets_hit_add", args).await, &["unbekannter Asset-Slot"]);
}

/// B-298/AC-05 über MCP: lokale Datei zu einem Treffer importieren; Ablage und Credit stimmen.
#[tokio::test(flavor = "multi_thread")]
async fn assets_import_tool() {
    let dir = asset_repo();
    let root = dir.path();
    let mut cs = connect(root).await;
    let add = json!({"id": "farm", "titel": "Farm Haus", "link": "https://x.itch.io/farm"});
    cs.ok("assets_hit_add", add).await;
    let hit = hits_of(root, "farm").expect("Treffer")[0].id.clone();
    let src = unter(root, BILD);
    let args = json!({"id": "farm", "hit": hit, "path": src.to_string_lossy()});
    let text = cs.ok("assets_import", args).await;
    enthaelt(&text, &["grafik/kandidaten/farm-haus.png importiert", "\nCheckout: "]);
    let credit =
        "| kandidaten/farm-haus.png | Farm Haus | unbekannt | unbekannt | https://x.itch.io/farm | Lizenz prüfen |";
    enthaelt(&lies(root, "public/grafik/CREDITS.md"), &[credit]);
    let anderswo = crate::test_helpers::tmp_dir();
    let outside = anderswo.path().join("fremd.png");
    std::fs::copy(&src, &outside).expect("kopieren");
    let args = json!({"id": "farm", "hit": hit, "path": outside.to_string_lossy()});
    let text = cs.abgelehnt("assets_import", args).await;
    enthaelt(&text, &["nur Dateien unterhalb der Repo-Wurzel"]);
    let text = cs.abgelehnt("assets_import", json!({"id": "farm", "hit": hit})).await;
    enthaelt(&text, &["manuell"]);
}

/// MC1/AC-06: die Recherche-Tools bleiben neben den Asset-Tools auf Spec-Basis; schreibende stehen in der Policy.
#[tokio::test(flavor = "multi_thread")]
async fn recherche_tools_bleiben() {
    let dir = asset_repo();
    let tools = connect(dir.path()).await.tools().await;
    for name in TOOLS {
        assert!(tools.iter().any(|t| t["name"] == name), "{name} fehlt");
        assert_eq!(write_tool(name), name != "assets_hits", "{name}");
    }
}
