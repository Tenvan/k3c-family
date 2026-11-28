//! Player- und Marker-Dateien der Seite Spieltest (Port von `playtestMedia`, `playtestFrame` und `windowMediaHandler`
//! aus `app_playtest.go`): nur `voice.wav`, `video.mkv` und `frames/marker-<nr>.jpg` einer geprüften Session, nie ein
//! freier Pfad. Eine Funktion ([`reply`]) für beide Wege: die Oberflächen-API (Browser-Dev) und das Protokoll `k3c` des
//! Fensters. Byte-Bereiche wie Go `http.ServeContent`, aber höchstens eine Spanne (mehrere liefern die ganze Datei).

mod http_route;
mod range;

#[cfg(test)]
#[path = "playtest_media_tests.rs"]
mod tests;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use axum::http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::json;

pub(crate) use http_route::router;
use range::{RangeError, Span};

use super::App;
use crate::playtest::{Error, Store};

/// Adresse der Player.
const MEDIA_PREFIX: &str = "/api/dev/playtest/media/";
/// Adresse der Marker-Bilder.
const FRAME_PREFIX: &str = "/api/dev/playtest/frame/";
/// Name des Protokolls im Fenster (unter Windows `http://k3c.localhost`).
pub(super) const PROTOCOL: &str = "k3c";
const HTTP_DATE: &str = "%a, %d %b %Y %H:%M:%S GMT";

/// Inhalt einer Antwort.
pub(crate) enum Body {
    Bytes(Vec<u8>),
    /// Schon auf den Anfang der Spanne gestellt; `len` Bytes gehören zur Antwort.
    File {
        file: File,
        len: u64,
    },
}

/// Antwort auf eine Anfrage, noch ohne Transport.
pub(crate) struct Reply {
    pub status: StatusCode,
    pub headers: Vec<(HeaderName, String)>,
    pub body: Body,
}

impl Reply {
    /// Fehler wie Go `uikit.Fail`.
    fn fail(status: StatusCode, message: &str) -> Reply {
        Reply {
            status,
            headers: vec![
                (header::CONTENT_TYPE, "application/json".to_owned()),
                (header::CACHE_CONTROL, "no-store".to_owned()),
            ],
            body: Body::Bytes(json!({ "error": message }).to_string().into_bytes()),
        }
    }

    fn headers(&self) -> HeaderMap {
        self.headers
            .iter()
            .filter_map(|(name, value)| Some((name.clone(), HeaderValue::from_str(value).ok()?)))
            .collect()
    }
}

/// Die Datei zur Adresse `path`. Ohne Prozent-Dekodierung: alles außer Kennung, festem Namen und Zahl fällt durch.
fn target(store: &Store, path: &str) -> Result<PathBuf, Error> {
    if let Some(rest) = path.strip_prefix(MEDIA_PREFIX) {
        let (id, file) = rest.split_once('/').ok_or(Error::NotFound)?;
        return store.media_path(id, file);
    }
    let rest = path.strip_prefix(FRAME_PREFIX).ok_or(Error::NotFound)?;
    let (id, number) = rest.split_once('/').ok_or(Error::NotFound)?;
    let digits = !number.is_empty() && number.bytes().all(|b| b.is_ascii_digit());
    let number = if digits { number.parse().unwrap_or(0) } else { 0 };
    store.frame_path(id, number)
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("wav") => "audio/wav",
        Some("mkv") => "video/x-matroska",
        Some("jpg") => "image/jpeg",
        _ => "application/octet-stream",
    }
}

fn header_text<'a>(headers: &'a HeaderMap, name: &HeaderName) -> &'a str {
    headers.get(name).and_then(|v| v.to_str().ok()).unwrap_or_default()
}

/// Sekunden seit 1970 einer HTTP-Datumsangabe.
fn http_seconds(text: &str) -> Option<i64> {
    let parsed = NaiveDateTime::parse_from_str(text, HTTP_DATE).ok()?;
    Some(parsed.and_utc().timestamp())
}

/// Die gewünschte Spanne; `If-Range` mit anderem Stand hebt sie auf.
fn wanted_span(headers: &HeaderMap, modified: i64, size: u64) -> Result<Option<Span>, RangeError> {
    let if_range = header_text(headers, &header::IF_RANGE);
    if !if_range.is_empty() && http_seconds(if_range) != Some(modified) {
        return Ok(None);
    }
    range::parse(header_text(headers, &header::RANGE), size)
}

fn unsatisfiable(error: RangeError, size: u64) -> Reply {
    let mut headers = vec![(header::CONTENT_TYPE, "text/plain; charset=utf-8".to_owned())];
    if error == RangeError::NoOverlap {
        headers.push((header::CONTENT_RANGE, format!("bytes */{size}")));
    }
    Reply {
        status: StatusCode::RANGE_NOT_SATISFIABLE,
        headers,
        body: Body::Bytes(format!("{error}\n").into_bytes()),
    }
}

/// Antwort mit der Datei `path`, ganz (200), als Spanne (206) oder unverändert (304).
fn serve_file(path: &Path, method: &Method, headers: &HeaderMap) -> std::io::Result<Reply> {
    let mut file = File::open(path)?;
    let meta = file.metadata()?;
    let size = meta.len();
    let modified: DateTime<Utc> = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH).into();
    let seconds = modified.timestamp();
    let mut out = vec![
        (header::CACHE_CONTROL, "no-store".to_owned()),
        (header::LAST_MODIFIED, modified.format(HTTP_DATE).to_string()),
    ];
    let since = http_seconds(header_text(headers, &header::IF_MODIFIED_SINCE));
    if since.is_some_and(|since| seconds <= since) {
        return Ok(Reply {
            status: StatusCode::NOT_MODIFIED,
            headers: out,
            body: Body::Bytes(Vec::new()),
        });
    }
    let span = match wanted_span(headers, seconds, size) {
        Ok(span) => span,
        Err(e) => return Ok(unsatisfiable(e, size)),
    };
    let (status, start, len) = match span {
        Some(Span { start, len }) => {
            let last = (start + len).saturating_sub(1);
            out.push((header::CONTENT_RANGE, format!("bytes {start}-{last}/{size}")));
            (StatusCode::PARTIAL_CONTENT, start, len)
        }
        None => (StatusCode::OK, 0, size),
    };
    out.extend([
        (header::CONTENT_TYPE, content_type(path).to_owned()),
        (header::ACCEPT_RANGES, "bytes".to_owned()),
        (header::CONTENT_LENGTH, len.to_string()),
    ]);
    file.seek(SeekFrom::Start(start))?;
    let body = if method == Method::HEAD {
        Body::Bytes(Vec::new())
    } else {
        Body::File { file, len }
    };
    Ok(Reply {
        status,
        headers: out,
        body,
    })
}

/// Beantwortet eine Anfrage nach `path` (ohne Host und Abfrage).
pub(crate) fn reply(store: &Store, method: &Method, path: &str, headers: &HeaderMap) -> Reply {
    if method != Method::GET && method != Method::HEAD {
        return Reply::fail(StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed");
    }
    match target(store, path) {
        Ok(file) => {
            serve_file(&file, method, headers).unwrap_or_else(|e| Reply::fail(StatusCode::NOT_FOUND, &e.to_string()))
        }
        Err(e) => Reply::fail(StatusCode::NOT_FOUND, &e.to_string()),
    }
}

/// Antwort des Protokolls `k3c`: der Inhalt steht ganz im Speicher (wie beim Asset-Server von Wails).
pub(crate) fn protocol_response(
    store: &Store,
    request: &tauri::http::Request<Vec<u8>>,
) -> tauri::http::Response<Vec<u8>> {
    let mut answer = reply(store, request.method(), request.uri().path(), request.headers());
    let headers = answer.headers();
    let body = match std::mem::replace(&mut answer.body, Body::Bytes(Vec::new())) {
        Body::Bytes(bytes) => bytes,
        Body::File { file, len } => {
            let mut bytes = Vec::new();
            if let Err(e) = file.take(len).read_to_end(&mut bytes) {
                answer.status = StatusCode::INTERNAL_SERVER_ERROR;
                bytes = e.to_string().into_bytes();
            }
            bytes
        }
    };
    let mut response = tauri::http::Response::new(body);
    *response.status_mut() = answer.status;
    *response.headers_mut() = headers;
    response
}

impl App {
    /// Antwortet auf eine Anfrage des Fensters; blockiert beim Lesen der Datei.
    pub(crate) fn media_response(&self, request: &tauri::http::Request<Vec<u8>>) -> tauri::http::Response<Vec<u8>> {
        protocol_response(&self.playtest_store(), request)
    }
}
