//! Port von `app_playtest_test.go`: Bindungen und Player-Weg, dazu die Pfadprüfung und das Protokoll des Fensters.
//! Fixtures nur unter `.work/tmp`.

use std::sync::Arc;

use k3c_kit::uikit::{self, HEADER, PREFIX};
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::*;
use crate::rpc::{self, call};
use crate::testutil::tmp_dir;

const ID: &str = "2026-11-21-1";

/// App mit einer Session unter `playtests/` und einer Datei daneben, die nie ausgeliefert werden darf.
fn playtest_app() -> (tempfile::TempDir, Arc<App>) {
    let root = tmp_dir();
    let dir = root.path().join("playtests").join(ID);
    std::fs::create_dir_all(dir.join("frames")).unwrap();
    for (name, body) in [
        ("session.json", r#"{"startMs":1,"endMs":60001,"stand":"aufgenommen"}"#),
        ("video.mkv", "0123456789"),
        ("voice.wav", "abc"),
        ("frames/marker-1.jpg", "bild"),
    ] {
        std::fs::write(dir.join(name), body).unwrap();
    }
    std::fs::write(root.path().join("playtests").join("voice.wav"), "geheim").unwrap();
    std::fs::write(root.path().join("voice.wav"), "geheim").unwrap();
    let app = App::new(root.path().to_path_buf(), 1, None);
    app.mark_ready();
    (root, Arc::new(app))
}

/// Anfragen, die aus der Ablage herauswollen oder anderes als die festen Dateien meinen.
const REFUSED: &[&str] = &[
    "/api/dev/playtest/media/../voice.wav",
    "/api/dev/playtest/media/./voice.wav",
    "/api/dev/playtest/media//voice.wav",
    "/api/dev/playtest/media/2026-11-21-1/../voice.wav",
    "/api/dev/playtest/media/2026-11-21-1/session.json",
    "/api/dev/playtest/media/2026-11-21-1/frames/marker-1.jpg",
    "/api/dev/playtest/media/2026-11-21-1\\..\\..\\voice.wav",
    "/api/dev/playtest/media/..\\2026-11-21-1/voice.wav",
    "/api/dev/playtest/media/2026-11-21-1/voice.wav::$DATA",
    "/api/dev/playtest/media/%2e%2e/voice.wav",
    "/api/dev/playtest/media/2026-11-21-1%2f..%2f../voice.wav",
    "/api/dev/playtest/frame/../1",
    "/api/dev/playtest/frame/2026-11-21-1/..",
    "/api/dev/playtest/frame/2026-11-21-1/0",
    "/api/dev/playtest/frame/2026-11-21-1/+1",
    "/api/dev/playtest/frame/2026-11-21-1/9",
    "/api/dev/playtest/frame/2026-11-21-1/1/2",
    "/api/dev/playtest/andere/2026-11-21-1/1",
];

fn window(app: &App, method: &str, path: &str, headers: &[(&str, &str)]) -> tauri::http::Response<Vec<u8>> {
    let mut request = tauri::http::Request::builder().method(method).uri(format!(
        "http://k3c.localhost{}",
        path.replace('\\', "%5C").replace('$', "%24")
    ));
    for (name, value) in headers {
        request = request.header(*name, *value);
    }
    app.media_response(&request.body(Vec::new()).unwrap())
}

fn text<'a>(response: &'a tauri::http::Response<Vec<u8>>, name: &str) -> &'a str {
    response.headers().get(name).map_or("", |v| v.to_str().unwrap())
}

#[test]
fn protokoll_200_206_und_404() {
    let (_root, app) = playtest_app();
    let voice = format!("{MEDIA_PREFIX}{ID}/voice.wav");
    let video = format!("{MEDIA_PREFIX}{ID}/video.mkv");
    let all = window(&app, "GET", &voice, &[]);
    assert_eq!((all.status().as_u16(), all.body().as_slice()), (200, b"abc".as_slice()));
    assert_eq!(text(&all, "content-type"), "audio/wav");
    assert_eq!(text(&all, "cache-control"), "no-store");
    assert_eq!(text(&all, "accept-ranges"), "bytes");
    assert_eq!(text(&all, "content-length"), "3");
    let part = window(&app, "GET", &video, &[("range", "bytes=2-4")]);
    assert_eq!(
        (part.status().as_u16(), part.body().as_slice()),
        (206, b"234".as_slice())
    );
    assert_eq!(text(&part, "content-range"), "bytes 2-4/10");
    assert_eq!(text(&part, "content-length"), "3");
    let behind = window(&app, "GET", &video, &[("range", "bytes=10-")]);
    assert_eq!(behind.status().as_u16(), 416);
    assert_eq!(text(&behind, "content-range"), "bytes */10");
    let frame = window(&app, "GET", &format!("{FRAME_PREFIX}{ID}/1"), &[]);
    assert_eq!(
        (frame.status().as_u16(), frame.body().as_slice()),
        (200, b"bild".as_slice())
    );
    assert_eq!(text(&frame, "content-type"), "image/jpeg");
    let head = window(&app, "HEAD", &video, &[]);
    assert_eq!(
        (head.status().as_u16(), head.body().len(), text(&head, "content-length")),
        (200, 0, "10")
    );
    let missing = window(&app, "GET", &format!("{MEDIA_PREFIX}2026-11-21-2/voice.wav"), &[]);
    assert_eq!(missing.status().as_u16(), 404);
    let body: Value = serde_json::from_slice(missing.body()).unwrap();
    assert_eq!(body, json!({"error": "Session oder Datei nicht gefunden"}));
    assert_eq!(window(&app, "POST", &voice, &[]).status().as_u16(), 405);
}

#[test]
fn protokoll_bedingte_anfragen() {
    let (_root, app) = playtest_app();
    let video = format!("{MEDIA_PREFIX}{ID}/video.mkv");
    let first = window(&app, "GET", &video, &[]);
    let modified = text(&first, "last-modified").to_owned();
    assert!(modified.ends_with(" GMT"), "{modified}");
    let same = window(&app, "GET", &video, &[("if-modified-since", &modified)]);
    assert_eq!((same.status().as_u16(), same.body().len()), (304, 0));
    let kept = window(&app, "GET", &video, &[("range", "bytes=0-0"), ("if-range", &modified)]);
    assert_eq!(kept.status().as_u16(), 206);
    let older = "Thu, 01 Jan 1970 00:00:00 GMT";
    let whole = window(&app, "GET", &video, &[("range", "bytes=0-0"), ("if-range", older)]);
    assert_eq!((whole.status().as_u16(), whole.body().len()), (200, 10));
}

#[test]
fn protokoll_lehnt_ausbruch_ab() {
    let (_root, app) = playtest_app();
    for path in REFUSED {
        let response = window(&app, "GET", path, &[]);
        assert_eq!(response.status().as_u16(), 404, "{path}");
        assert!(!String::from_utf8_lossy(response.body()).contains("geheim"), "{path}");
    }
}

/// Eine Anfrage Zeile für Zeile: Clients wie `reqwest` räumen `..` sonst vorher weg.
async fn raw(port: u16, path: &str, headers: &[(&str, &str)]) -> (u16, String) {
    let mut socket = tokio::net::TcpStream::connect(("127.0.0.1", port)).await.unwrap();
    let mut request = format!("GET {path} HTTP/1.1\r\nConnection: close\r\n");
    if !headers.iter().any(|(name, _)| *name == "Host") {
        request.push_str("Host: localhost:5191\r\n");
    }
    for (name, value) in headers {
        request.push_str(&format!("{name}: {value}\r\n"));
    }
    request.push_str("\r\n");
    socket.write_all(request.as_bytes()).await.unwrap();
    let mut answer = Vec::new();
    socket.read_to_end(&mut answer).await.unwrap();
    let answer = String::from_utf8_lossy(&answer).into_owned();
    let status = answer.split(' ').nth(1).unwrap().parse().unwrap();
    let body = answer.split_once("\r\n\r\n").map_or("", |(_, body)| body);
    (status, body.to_owned())
}

async fn serve(app: &Arc<App>) -> u16 {
    let listener = uikit::listen("TEST_PORT", 0).await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(uikit::serve(listener, rpc::uiapi::router(app), std::future::pending()));
    port
}

/// Aus `TestPlaytestMedia`: nur die festen Dateien, mit Bereichen, hinter dem Schutz der Oberfläche.
#[tokio::test(flavor = "multi_thread")]
async fn oberflaechen_api_liefert_medien() {
    let (_root, app) = playtest_app();
    let port = serve(&app).await;
    let voice = format!("{MEDIA_PREFIX}{ID}/voice.wav");
    let video = format!("{MEDIA_PREFIX}{ID}/video.mkv");
    assert_eq!(raw(port, &voice, &[]).await, (200, "abc".to_owned()));
    assert_eq!(
        raw(port, &video, &[("Range", "bytes=2-4")]).await,
        (206, "234".to_owned())
    );
    assert_eq!(
        raw(port, &format!("{FRAME_PREFIX}{ID}/1"), &[]).await,
        (200, "bild".to_owned())
    );
    assert_eq!(raw(port, &format!("{MEDIA_PREFIX}{ID}/session.json"), &[]).await.0, 404);
    assert_eq!(raw(port, &voice, &[("Host", "fremd.example:80")]).await.0, 403);
    assert_eq!(raw(port, &voice, &[("Sec-Fetch-Site", "cross-site")]).await.0, 403);
    for path in REFUSED {
        let (status, body) = raw(port, path, &[]).await;
        // Kodierte Pfade weist schon der Wächter ab.
        assert!(
            status == 404 || status == 403 || status == 400,
            "{path}: {status} {body}"
        );
        assert!(!body.contains("geheim"), "{path}: {body}");
    }
}

/// Aus `TestPlaytestBindings`: listen, löschen erst mit Bestätigung, über RPC erreichbar.
#[tokio::test(flavor = "multi_thread")]
async fn bindungen_loeschen_erst_mit_bestaetigung() {
    let (root, app) = playtest_app();
    let list = app.playtest_list().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].stage, "aufgenommen");
    assert!(
        app.playtest_delete("video", ID, false).is_err(),
        "ohne Bestätigung gelöscht"
    );
    let video = root.path().join("playtests").join(ID).join("video.mkv");
    assert!(video.exists(), "Video fehlt ohne Bestätigung");
    assert_eq!(app.playtest_delete("video", ID, true).unwrap().total, 10);
    assert!(!video.exists(), "Video noch da");
    assert_eq!(call(&app, "PlaytestList", &[]).unwrap().as_array().unwrap().len(), 1);
    let port = serve(&app).await;
    let response = reqwest::Client::new()
        .post(format!("http://127.0.0.1:{port}{PREFIX}rpc/PlaytestList"))
        .header(HEADER, "1")
        .body("[]")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
}
