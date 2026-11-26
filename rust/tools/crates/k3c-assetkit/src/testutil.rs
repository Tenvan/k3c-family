//! Hilfen für Tests (V3). Temp-Ordner liegen im Repo (Regel „Schreiben nur im Repo“), nie unter
//! `std::env::temp_dir()`.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use axum::Router;
use axum::http::{StatusCode, Uri};
use chrono::{DateTime, FixedOffset, TimeZone};

use crate::net;
use crate::register::{KIND_SOUND, REGISTER_PATH};
use crate::research::{Hit, add_hits, hits_of};
use crate::suggestions::{SUGGESTIONS_PATH, Suggestion, Suggestions, load_suggestions};

/// Wurzel des Checkouts, vier Ebenen über dem Crate.
pub(crate) fn checkout_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("Crate liegt vier Ebenen unter der Wurzel")
        .to_path_buf()
}

/// Frischer Temp-Ordner unter `K3C_TMP`, sonst `<Repo>/.work/tmp`; er wird beim Fallenlassen gelöscht.
pub(crate) fn tempdir() -> tempfile::TempDir {
    let base = std::env::var_os("K3C_TMP")
        .filter(|v| !v.is_empty())
        .map_or_else(|| checkout_root().join(".work").join("tmp"), PathBuf::from);
    fs::create_dir_all(&base).expect("Temp-Basis anlegen");
    tempfile::Builder::new()
        .prefix("k3c-assetkit-")
        .tempdir_in(base)
        .expect("Temp-Ordner anlegen")
}

/// Mini-Repo: `data/sounds.json` mit einem Grafik- und einem Sound-Slot, zwei Bilder, ein Sound.
pub(crate) fn repo() -> tempfile::TempDir {
    let dir = tempdir();
    let town = dir.path().join("public").join("grafik").join("town");
    write_png(&town.join("well.png"), 64, 48);
    write_png(&town.join("wagon.png"), 32, 32);
    write_file(&dir.path().join("public").join("audio").join("coin.ogg"), "ogg");
    write_file(
        &dir.path().join(REGISTER_PATH),
        r#"{
  "_comment": "Test",
  "slots": [
    { "id": "farm", "art": "grafik", "kategorie": "gebaeude", "beschreibung": "Farm", "wahl": [] },
    { "id": "sfx:coin", "art": "sound", "kategorie": "sfx", "beschreibung": "Münze", "bus": "sfx", "wahl": [] }
  ]
}
"#,
    );
    dir
}

fn create_parent(path: &Path) {
    fs::create_dir_all(path.parent().expect("Pfad mit Ordner")).expect("Ordner anlegen");
}

pub(crate) fn write_file(path: &Path, text: &str) {
    create_parent(path);
    fs::write(path, text).expect("Datei schreiben");
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for &b in data {
        crc ^= u32::from(b);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & 0u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}

fn push_chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let body = [kind.as_slice(), data].concat();
    out.extend_from_slice(&body);
    out.extend_from_slice(&crc32(&body).to_be_bytes());
}

/// PNG-Kopf (Signatur, IHDR, IEND) ohne Bilddaten: Rust liest nur die Maße.
pub(crate) fn png_bytes(w: u32, h: u32) -> Vec<u8> {
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    let ihdr = [w.to_be_bytes().as_slice(), &h.to_be_bytes(), &[8, 6, 0, 0, 0]].concat();
    push_chunk(&mut out, b"IHDR", &ihdr);
    push_chunk(&mut out, b"IEND", &[]);
    out
}

pub(crate) fn write_png(path: &Path, w: u32, h: u32) {
    create_parent(path);
    fs::write(path, png_bytes(w, h)).expect("PNG schreiben");
}

/// Startet den Router auf einem freien Loopback-Port und liefert `http://127.0.0.1:<port>` (V7). Der Server läuft auf
/// der Laufzeit des Tests und endet mit ihr.
pub(crate) async fn serve(router: axum::Router) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Loopback-Port");
    let address = listener.local_addr().expect("Adresse");
    tokio::spawn(async move { axum::serve(listener, router).await });
    format!("http://{address}")
}

/// Der feste Testtag (Go `day`).
pub(crate) fn day() -> DateTime<FixedOffset> {
    FixedOffset::east_opt(0)
        .and_then(|utc| utc.with_ymd_and_hms(2026, 10, 14, 9, 0, 0).single())
        .expect("Testtag")
}

/// Mini-Repo mit Credit-Datei und zwei Treffern zu `farm`; liefert die IDs (itch.io: prüfen, Pixabay: ausgeschlossen).
pub(crate) fn import_repo() -> (tempfile::TempDir, String, String) {
    let dir = repo();
    write_file(
        &dir.path().join("public").join("grafik").join("CREDITS.md"),
        "# Grafik\n\n| Ordner | Pack | Urheber | Lizenz | Quelle |\n|---|---|---|---|---|\n\
         | town | Town | Zuno | CC0 1.0 | https://x |\n",
    );
    let hit = |title: &str, link: &str, artist: &str, license: &str| Hit {
        title: title.to_string(),
        link: link.to_string(),
        source: "Agent".to_string(),
        artist: artist.to_string(),
        license: license.to_string(),
        ..Hit::default()
    };
    let hits = [
        hit("../../Farm Häuschen", "https://x.itch.io/farm", "Anna", ""),
        hit("Farm", "https://pixabay.com/farm", "", "Pixabay"),
    ];
    add_hits(dir.path(), "farm", &hits, day()).expect("add_hits");
    let got = hits_of(dir.path(), "farm").expect("hits_of");
    (dir, got[0].id.clone(), got[1].id.clone())
}

/// Quelldatei in einem eigenen Temp-Ordner.
pub(crate) fn src_file(name: &str, data: &[u8]) -> (tempfile::TempDir, PathBuf) {
    let dir = tempdir();
    let path = dir.path().join(name);
    fs::write(&path, data).expect("Quelldatei");
    (dir, path)
}

pub(crate) fn ogg_data() -> Vec<u8> {
    [b"OggS".as_slice(), &[b'x'; 60]].concat()
}

fn zip_data() -> Vec<u8> {
    [b"PK\x03\x04".as_slice(), &[b'z'; 20]].concat()
}

/// Mini-Repo samt Vorschlagsliste, deren Dateien ein lokaler Testserver liefert; `hits` zählt die Anfragen.
pub(crate) struct VorratRepo {
    pub(crate) dir: tempfile::TempDir,
    pub(crate) client: net::DownloadClient,
    pub(crate) hits: Arc<AtomicI32>,
}

impl VorratRepo {
    pub(crate) fn root(&self) -> &Path {
        self.dir.path()
    }
}

async fn vorrat_server(hits: Arc<AtomicI32>) -> String {
    let router = Router::new().fallback(move |uri: Uri| async move {
        hits.fetch_add(1, Ordering::SeqCst);
        match uri.path() {
            "/coin.ogg" => (StatusCode::OK, ogg_data()),
            "/pack.zip" => (StatusCode::OK, zip_data()),
            "/fake.ogg" => (StatusCode::OK, [b"<html>".as_slice(), &[b'y'; 58]].concat()),
            _ => (StatusCode::NOT_FOUND, Vec::new()),
        }
    });
    serve(router).await
}

/// Slot `sfx:coin`, Credit-Datei und sechs Vorschläge (Go `vorratRepo`); der Server ist als Quelle freigegeben.
pub(crate) async fn vorrat_repo() -> VorratRepo {
    let dir = repo();
    write_file(&dir.path().join("public").join("audio").join("README.md"), "# Audio\n");
    let hits = Arc::new(AtomicI32::new(0));
    let base = vorrat_server(hits.clone()).await;
    net::testing::allow(&base);
    let proposal = |id: &str, file: &str, size: usize, license: &str| Suggestion {
        id: id.to_string(),
        slot: "sfx:coin".to_string(),
        kind: KIND_SOUND.to_string(),
        title: format!("Münze {id}"),
        source: "OpenGameArt".to_string(),
        page: format!("https://opengameart.org/content/{id}"),
        file_url: format!("{base}/{file}"),
        artist: "Anna".to_string(),
        license: license.to_string(),
        file_name: file.to_string(),
        size: size as i64,
        priority: 1,
        status: "offen".to_string(),
        ..Suggestion::default()
    };
    let list = Suggestions {
        list: vec![
            proposal("v1", "coin.ogg", ogg_data().len(), "CC0"),
            proposal("v2", "pack.zip", zip_data().len(), "CC-BY 4.0"),
            proposal("v3", "fake.ogg", ogg_data().len(), "CC0"),
            proposal("v4", "coin.ogg", ogg_data().len(), "CC-BY-NC 4.0"),
            proposal("v5", "missing.ogg", 10, "CC0"),
            proposal("v6", "coin.ogg", 99, "CC0"),
        ],
        ..Suggestions::default()
    };
    let json = serde_json::to_vec(&list).expect("Liste");
    fs::write(dir.path().join(SUGGESTIONS_PATH), json).expect("Liste schreiben");
    VorratRepo {
        dir,
        client: net::test_download_client(),
        hits,
    }
}

/// Der Vorschlag mit der ID.
pub(crate) fn suggestion(root: &Path, id: &str) -> Suggestion {
    load_suggestions(root)
        .expect("Vorschläge")
        .list
        .into_iter()
        .find(|s| s.id == id)
        .unwrap_or_else(|| panic!("{id} fehlt"))
}
