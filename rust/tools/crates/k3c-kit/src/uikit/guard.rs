//! Schutz der Oberflächen-API. Die Apps starten Prozesse, deshalb: nur Loopback, nur localhost/Loopback-IP als Host,
//! nur Anfragen derselben Seite (auch nicht von anderen lokalen Seiten) und POST nur mit der Kopfzeile [`HEADER`].

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use axum::Router;
use axum::extract::{ConnectInfo, Request};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, Uri, header};
use axum::middleware::{self, Next};
use axum::response::Response;

use super::{HEADER, fail};

/// Setzt den Schutz vor alle Adressen von `router`; `app` ist der Name in Meldungen. Ohne bekannte Gegenstelle
/// (`ConnectInfo<SocketAddr>`) wird abgelehnt.
pub fn guard(app: impl Into<String>, router: Router) -> Router {
    let app: Arc<str> = app.into().into();
    router.layer(middleware::from_fn(move |request: Request, next: Next| {
        check(app.clone(), request, next)
    }))
}

async fn check(app: Arc<str>, mut request: Request, next: Next) -> Response {
    let peer = request.extensions().get::<ConnectInfo<SocketAddr>>().map(|info| info.0);
    // Wie Go: bei einer absoluten Adresse in der Anfragezeile gilt deren Host, nicht die Kopfzeile.
    let absolute = request
        .uri()
        .authority()
        .and_then(|a| HeaderValue::from_str(a.as_str()).ok());
    if let Some(host) = absolute {
        request.headers_mut().insert(header::HOST, host);
    }
    let refused = refusal(&app, request.method(), request.uri(), request.headers(), peer);
    match refused {
        Some(message) => fail(StatusCode::FORBIDDEN, message),
        None => next.run(request).await,
    }
}

/// Der Grund der Ablehnung, oder `None`, wenn die Anfrage durch darf.
pub(super) fn refusal(
    app: &str,
    method: &Method,
    uri: &Uri,
    headers: &HeaderMap,
    peer: Option<SocketAddr>,
) -> Option<String> {
    if !peer.is_some_and(|peer| loopback(&peer)) {
        return Some(format!("{app} ist nur auf diesem Rechner erreichbar"));
    }
    // Kodierter Pfad: so schlüpft eine Anfrage an einem Filter vorbei. Kein Pfad der API braucht `%`.
    if uri.path().contains('%') {
        return Some("Kodierter Pfad nicht erlaubt".to_string());
    }
    // DNS-Rebinding: ein fremder Name, der auf diesen Rechner zeigt.
    if !host(headers).is_some_and(host_allowed) {
        return Some(format!("{app} antwortet nur auf localhost oder eine Loopback-Adresse"));
    }
    if !same_site(headers) {
        return Some("Anfrage einer fremden Seite".to_string());
    }
    if method == Method::POST && headers.get(HEADER).is_none_or(|v| v != "1") {
        return Some(format!("Kopfzeile {HEADER}: 1 fehlt"));
    }
    None
}

/// Die einzige Kopfzeile `Host`; fehlt sie, steht sie doppelt da oder ist sie kein Text, gibt es keinen Host.
fn host(headers: &HeaderMap) -> Option<&str> {
    let mut all = headers.get_all(header::HOST).iter();
    match (all.next(), all.next()) {
        (Some(only), None) => only.to_str().ok(),
        _ => None,
    }
}

/// Erlaubt localhost und Loopback-IPs, nie einen anderen Namen oder eine LAN-Adresse.
pub fn host_allowed(host_port: &str) -> bool {
    let host = split_host(host_port).unwrap_or_else(|| host_port.trim_matches(['[', ']']));
    host == "localhost" || host.parse::<IpAddr>().is_ok_and(is_loopback_ip)
}

/// Der Host aus `host:port` bzw. `[v6]:port` nach den Regeln von Go `net.SplitHostPort`; `None`, wenn es keins ist.
fn split_host(host_port: &str) -> Option<&str> {
    let colon = host_port.rfind(':')?;
    let (host, port) = (&host_port[..colon], &host_port[colon + 1..]);
    let inner = match host.strip_prefix('[') {
        Some(rest) => rest.strip_suffix(']')?,
        None if host.contains(':') => return None,
        None => host,
    };
    let bracket = |s: &str| s.contains(['[', ']']);
    (!bracket(inner) && !bracket(port)).then_some(inner)
}

/// Wie Go `IP.IsLoopback`: auch `::ffff:127.0.0.1` ist dieser Rechner.
fn is_loopback_ip(ip: IpAddr) -> bool {
    ip.to_canonical().is_loopback()
}

/// `Origin` (falls gesendet) nennt genau den Host der Anfrage, `Sec-Fetch-Site` (falls gesendet) ist `same-origin` oder
/// `none`. Hinter Vite (changeOrigin aus) sind beide `localhost:<Vite-Port>`; curl und eigene Clients senden keines.
pub fn same_site(headers: &HeaderMap) -> bool {
    if let Some(origin) = headers.get(header::ORIGIN).filter(|o| !o.is_empty()) {
        let named = origin.to_str().ok().and_then(origin_host);
        if named.is_none() || named != host(headers) {
            return false;
        }
    }
    headers
        .get("sec-fetch-site")
        .is_none_or(|site| site.is_empty() || site == "same-origin" || site == "none")
}

/// Host samt Port aus `schema://host[:port]`. Strenger als Go `url.Parse`: ohne Schema oder mit Benutzerteil passt er
/// zu keinem Host; Browser senden nur diese Form oder `null`.
fn origin_host(origin: &str) -> Option<&str> {
    let (scheme, rest) = origin.split_once("://")?;
    let valid = scheme.starts_with(|c: char| c.is_ascii_alphabetic())
        && scheme.chars().all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c));
    valid.then(|| rest.split(['/', '?', '#']).next().unwrap_or(rest))
}

/// Ob die Gegenstelle auf diesem Rechner liegt.
pub fn loopback(remote: &SocketAddr) -> bool {
    is_loopback_ip(remote.ip())
}
