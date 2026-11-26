//! HTTP-Hilfen für Quellen und Downloads (V6, V7): Clients, Abfrage-Kodierung wie Go `net/url`, begrenztes Lesen.

use std::future::Future;
use std::time::Duration;

use reqwest::header::{LOCATION, USER_AGENT as USER_AGENT_HEADER};
use reqwest::{Method, Response};
use url::Url;

use crate::error::{Error, Result};

/// Kennung aller Anfragen des Crates.
pub(crate) const USER_AGENT: &str = "k3c-workbench/0.1 (Ressourcen-Manager, B-298)";

/// Längste Pause, bis die Antwort bzw. das nächste Stück eines Downloads kommt. Go hat hier keine eigene Frist; sie
/// begrenzt nur Stillstand, nicht die Gesamtdauer.
const IDLE_TIMEOUT: Duration = Duration::from_secs(60);

/// Client für Downloads, der nie selbst weiterleitet. Nur dieses Modul erzeugt ihn: ein fremder `reqwest::Client` folgte
/// einer Weiterleitung, bevor das Ziel geprüft ist (Go erzwingt in `downloadHit` sein `CheckRedirect`).
#[derive(Clone, Debug)]
pub struct DownloadClient(reqwest::Client);

/// Client für die Produktion: Proxy aus der Umgebung, keine Weiterleitungen (jeden Sprung prüft der Aufrufer).
pub fn client() -> reqwest::Result<DownloadClient> {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map(DownloadClient)
}

/// [`test_client`] als [`DownloadClient`].
#[cfg(test)]
pub(crate) fn test_download_client() -> DownloadClient {
    DownloadClient(test_client())
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

/// Dekodierter Pfad der Adresse wie Go `url.URL.Path` (ohne Normalisieren von `..`, anders als das Crate `url`);
/// `None`, wo Go `url.Parse` wegen Steuerzeichen, fehlendem Schema oder kaputtem `%` scheitert.
pub(crate) fn url_path(raw: &str) -> Option<String> {
    if raw.chars().any(|c| c.is_ascii_control()) || raw.starts_with(':') {
        return None;
    }
    let raw = raw.split('#').next().unwrap_or_default();
    let scheme = raw.split_once(':').filter(|(s, _)| {
        s.starts_with(|c: char| c.is_ascii_alphabetic())
            && s.chars().all(|c| c.is_ascii_alphanumeric() || "+.-".contains(c))
    });
    let rest = scheme.map_or(raw, |(_, rest)| rest);
    let rest = rest.split('?').next().unwrap_or_default();
    if !rest.starts_with('/') {
        if scheme.is_some() {
            return Some(String::new());
        }
        if rest.split('/').next().is_some_and(|first| first.contains(':')) {
            return None;
        }
    }
    let path = match rest.strip_prefix("//") {
        Some(authority) => authority.find('/').map_or("", |i| &authority[i..]),
        None => rest,
    };
    let broken = path
        .split('%')
        .skip(1)
        .any(|part| !part.get(..2).is_some_and(|h| h.chars().all(|c| c.is_ascii_hexdigit())));
    (!broken).then(|| percent_decode(path))
}

/// Löst `%xx` auf; alles andere bleibt, ungültiges UTF-8 wird ersetzt.
pub(crate) fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let hex = |i: usize| bytes.get(i).and_then(|&b| char::from(b).to_digit(16));
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match (bytes[i], hex(i + 1), hex(i + 2)) {
            (b'%', Some(high), Some(low)) => {
                out.push((high * 16 + low) as u8);
                i += 3;
            }
            (b, _, _) => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Prüft den Rohtext einer Adresse auf das, was `url` still entfernt, Go aber ablehnt: leere Zugangsdaten (`https://@host`)
/// und einen Port mit führender Null (`:0443`). Adressen ohne Host-Teil (relative `Location`) gelten als schlicht.
pub(crate) fn plain_authority(raw: &str) -> bool {
    let rest = match raw.split_once("://") {
        Some((_, rest)) => rest,
        None => match raw.strip_prefix("//") {
            Some(rest) => rest,
            None => return true,
        },
    };
    let authority = rest.split(['/', '?', '#', '\\']).next().unwrap_or_default();
    let port = authority.rsplit_once(':').map(|(_, port)| port);
    !authority.contains('@') && !port.is_some_and(|p| p.len() > 1 && p.starts_with('0'))
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

/// Netzfehler ohne Adresse: so gelangt nichts aus der Anfrage in Hinweise und Logs.
pub(crate) fn network_error(e: reqwest::Error) -> Error {
    let e = e.without_url();
    let mut text = e.to_string();
    let mut cause = std::error::Error::source(&e);
    while let Some(c) = cause {
        text.push_str(&format!(": {c}"));
        cause = c.source();
    }
    Error::Msg(text)
}

/// Ein Netzschritt mit der Frist [`IDLE_TIMEOUT`].
pub(crate) async fn bounded<T>(step: impl Future<Output = reqwest::Result<T>>) -> Result<T> {
    match tokio::time::timeout(IDLE_TIMEOUT, step).await {
        Ok(result) => result.map_err(network_error),
        Err(_) => Err(Error::Msg("zeitüberschreitung beim Laden".to_string())),
    }
}

/// Sendet die Anfrage und folgt 301/302/303/307/308 selbst (V6): vor jedem Sprung läuft `check(ziel, sprünge)` (wie
/// Go `CheckRedirect`, `sprünge` = bisher gestellte Anfragen), mehr als `max_hops` Sprünge sind immer ein Fehler. HEAD
/// bleibt HEAD, sonst wie Go: 301 bis 303 werden zu GET. Mitgeschickt wird nur die Kennung, nie ein Token.
///
/// Leitet der Client doch selbst weiter, wird die Antwort verworfen.
pub(crate) async fn send_following(
    client: &DownloadClient,
    method: Method,
    url: &Url,
    max_hops: usize,
    check: impl Fn(&Url, usize) -> Result<()>,
) -> Result<Response> {
    let (mut method, mut url, mut hops) = (method, url.clone(), 0);
    loop {
        let request = client
            .0
            .request(method.clone(), url.clone())
            .header(USER_AGENT_HEADER, USER_AGENT);
        let response = bounded(request.send()).await?;
        if !same_target(response.url(), &url) {
            return Err(Error::Msg(
                "weiterleitung abgelehnt: der Client folgt selbst".to_string(),
            ));
        }
        let status = response.status().as_u16();
        let location = response.headers().get(LOCATION).filter(|l| !l.is_empty());
        let (301 | 302 | 303 | 307 | 308, Some(location)) = (status, location) else {
            return Ok(response);
        };
        let raw = std::str::from_utf8(location.as_bytes()).ok();
        let next = raw
            .and_then(|l| url.join(l).ok())
            .ok_or_else(|| Error::Msg("weiterleitung: ungültiges Ziel".to_string()))?;
        // Nicht leere Zugangsdaten sieht `check`; leere und `:0443` verschwinden beim Parsen.
        if next.username().is_empty() && next.password().is_none() && !raw.is_some_and(plain_authority) {
            return Err(Error::Msg("weiterleitung abgelehnt: ungültiges Ziel".to_string()));
        }
        hops += 1;
        if hops > max_hops {
            return Err(Error::Msg("zu viele Weiterleitungen".to_string()));
        }
        check(&next, hops)?;
        if status <= 303 && method != Method::HEAD {
            method = Method::GET;
        }
        url = next;
    }
}

/// Gleiches Ziel? Zugangsdaten (reqwest macht daraus eine Kopfzeile) und Fragment zählen nicht.
fn same_target(a: &Url, b: &Url) -> bool {
    let key = |u: &Url| {
        (
            u.origin().ascii_serialization(),
            u.path().to_string(),
            u.query().map(str::to_string),
        )
    };
    key(a) == key(b)
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
