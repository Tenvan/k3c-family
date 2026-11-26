//! HTTP-Hilfen für Quellen und Downloads (V6, V7): Clients, Abfrage-Kodierung wie Go `net/url`, begrenztes Lesen.

use url::Url;

/// Client für die Produktion: Proxy aus der Umgebung, keine Weiterleitungen (jeden Sprung prüft der Aufrufer).
pub fn client() -> reqwest::Result<reqwest::Client> {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
}

/// Wie [`client`], aber ohne Proxy (B-474: Tests erreichen ihren Loopback-Server immer direkt).
#[cfg(test)]
pub(crate) fn test_client() -> reqwest::Client {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .no_proxy()
        .build()
        .expect("Test-Client")
}

/// Go `url.QueryEscape`: nur `A–Z a–z 0–9 - _ . ~` bleiben, Leerzeichen wird `+`.
pub(crate) fn query_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(char::from(b)),
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Go `url.Values.Encode`: nach Schlüssel sortiert, Werte eines Schlüssels in ihrer Reihenfolge.
pub(crate) fn encode_query(pairs: &[(&str, &str)]) -> String {
    let mut sorted = pairs.to_vec();
    sorted.sort_by_key(|(key, _)| *key);
    let encoded: Vec<String> = sorted
        .iter()
        .map(|(key, value)| format!("{}={}", query_escape(key), query_escape(value)))
        .collect();
    encoded.join("&")
}

/// Adresse mit Host im Sinne von Go `url.Parse`: nur `schema://…` oder `//…` hat dort einen Host. Der Parser des Crates
/// `url` ergänzt sonst fehlende Schrägstriche (`https:host`, `https:///x`); Steuerzeichen lehnt Go ab.
pub(crate) fn parse_with_host(raw: &str) -> Option<Url> {
    if raw.chars().any(|c| c.is_ascii_control()) || raw.starts_with(' ') {
        return None;
    }
    let absolute = if raw.starts_with("//") {
        format!("http:{raw}")
    } else {
        raw.to_string()
    };
    let (_, rest) = absolute.split_once("://")?;
    if rest.starts_with(['/', '?', '#', '\\']) {
        return None;
    }
    let url = Url::parse(&absolute).ok()?;
    url.host_str().is_some_and(|h| !h.is_empty()).then_some(url)
}

/// Liest höchstens `limit` Bytes der Antwort (Go `io.ReadAll(io.LimitReader(body, limit))`). Wer eine Überschreitung
/// erkennen will, übergibt die Grenze plus eins.
pub(crate) async fn read_limited(mut res: reqwest::Response, limit: usize) -> reqwest::Result<Vec<u8>> {
    let mut out = Vec::new();
    while out.len() < limit {
        let Some(chunk) = res.chunk().await? else { break };
        let take = chunk.len().min(limit - out.len());
        out.extend_from_slice(&chunk[..take]);
    }
    Ok(out)
}

/// Test-Naht (V7): Ersatz für Go `allowTestServer`/`allowTestHost`.
#[cfg(test)]
pub(crate) mod testing {
    use std::sync::{Mutex, PoisonError};

    use url::Url;

    static ALLOWED: Mutex<Vec<String>> = Mutex::new(Vec::new());

    /// Lässt den Ursprung (`http://127.0.0.1:<port>`) durch https-Zwang und Host-Prüfungen.
    pub(crate) fn allow(origin: &str) {
        let origin = Url::parse(origin).expect("Ursprung").origin().ascii_serialization();
        ALLOWED.lock().unwrap_or_else(PoisonError::into_inner).push(origin);
    }

    pub(super) fn allowed(url: &Url) -> bool {
        let origin = url.origin().ascii_serialization();
        ALLOWED.lock().unwrap_or_else(PoisonError::into_inner).contains(&origin)
    }
}

/// Ist die Adresse ein freigegebener Testserver? Außerhalb der Tests immer `false`.
// Erster Aufrufer ist der Download (RU67.3).
#[allow(dead_code)]
pub(crate) fn is_test_origin(url: &Url) -> bool {
    #[cfg(test)]
    {
        testing::allowed(url)
    }
    #[cfg(not(test))]
    {
        let _ = url;
        false
    }
}

#[cfg(test)]
#[path = "net_tests.rs"]
mod tests;
