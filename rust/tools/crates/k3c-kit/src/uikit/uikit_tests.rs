//! Port von `uikit_test.go`, über echten Listener auf Port 0 (`ConnectInfo` gibt es nur dort).

use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::http::{HeaderMap, HeaderName, HeaderValue, Method as HttpMethod, Uri};
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::oneshot;

use super::guard::refusal;
use super::serve::listen_within;
use super::*;

fn method(call: fn(Vec<Value>) -> Result<Value, String>) -> Method {
    Arc::new(move |raw| Box::pin(async move { call(raw) }))
}

fn info(raw: Vec<Value>) -> Result<Value, String> {
    args::<()>(raw)?;
    Ok(json!({ "version": "test" }))
}

fn doc(raw: Vec<Value>) -> Result<Value, String> {
    let (name,) = args::<(String,)>(raw)?;
    if name == "gibtsnicht" {
        return Err("unbekanntes Dokument".to_string());
    }
    Ok(json!(format!("# {name}")))
}

/// Eine App mit Aufruf-Tabelle im Kleinen.
fn test_api() -> Api {
    Api {
        app: "k3c-test".to_string(),
        methods: HashMap::from([("Info".to_string(), method(info)), ("Doc".to_string(), method(doc))]),
        bus: Arc::new(Bus::new()),
        track: None,
    }
}

struct Running {
    url: String,
    address: SocketAddr,
    client: reqwest::Client,
    stop: oneshot::Sender<()>,
    server: tokio::task::JoinHandle<std::io::Result<()>>,
}

async fn start(api: Api) -> Running {
    let listener = listen("X", 0).await.unwrap();
    let address = listener.local_addr().unwrap();
    let url = page_url(&listener);
    assert_eq!(url, format!("http://{address}/"));
    let (stop, stopped) = oneshot::channel::<()>();
    let server = tokio::spawn(serve(listener, api.router(), async move {
        let _ = stopped.await;
    }));
    let client = reqwest::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    Running {
        url: format!("{url}api/dev/"),
        address,
        client,
        stop,
        server,
    }
}

impl Running {
    async fn call(&self, path: &str, body: &str) -> (u16, Value) {
        let request = self.client.post(format!("{}{path}", self.url)).header(HEADER, "1");
        answer(request.body(body.to_string())).await
    }

    /// Schickt eine Anfrage Byte für Byte (Host-Schreibweisen, die ein Client nicht zulässt).
    async fn raw(&self, request: &str) -> String {
        let mut stream = tokio::net::TcpStream::connect(self.address).await.unwrap();
        stream.write_all(request.as_bytes()).await.unwrap();
        let mut out = String::new();
        let read = stream.read_to_string(&mut out);
        tokio::time::timeout(Duration::from_secs(5), read)
            .await
            .unwrap()
            .unwrap();
        out
    }
}

async fn answer(request: reqwest::RequestBuilder) -> (u16, Value) {
    let response = request.send().await.unwrap();
    let status = response.status().as_u16();
    let text = response.text().await.unwrap();
    (status, serde_json::from_str(&text).unwrap_or(Value::Null))
}

type Case = (
    &'static str,
    &'static str,
    &'static [(&'static str, Option<&'static str>)],
);

/// Prüft die Regeln von [`guard`] an einer Anfrage wie aus Go `TestGuard`, nach den Änderungen in `edits`.
fn judge(peer: &str, path: &str, method: HttpMethod, edits: &[(&str, Option<&str>)]) -> Option<String> {
    let mut headers = HeaderMap::new();
    headers.insert("host", HeaderValue::from_static("127.0.0.1:8191"));
    headers.insert("x-k3c-dev", HeaderValue::from_static("1"));
    for (name, value) in edits {
        let name = HeaderName::from_bytes(name.as_bytes()).unwrap();
        match value {
            Some(value) => headers.insert(name, HeaderValue::from_str(value).unwrap()),
            None => headers.remove(name),
        };
    }
    let uri: Uri = path.parse().unwrap();
    refusal("k3c-test", &method, &uri, &headers, peer.parse().ok())
}

/// Die Apps starten Prozesse: nur Loopback, nur localhost/Loopback-IP als Host, POST nur mit Kopfzeile.
#[test]
fn guard_lehnt_ab() {
    let local = "127.0.0.1:50000";
    let info = "/api/dev/rpc/Info";
    for peer in ["192.168.1.20:50000", "[2001:db8::1]:50000", ""] {
        assert!(judge(peer, info, HttpMethod::POST, &[]).is_some(), "Gegenstelle {peer}");
    }
    assert!(judge(local, "/api/dev/rpc/%49nfo", HttpMethod::POST, &[]).is_some());
    let cases: &[Case] = &[
        ("fremder Host", info, &[("host", Some("evil.example:8191"))]),
        ("LAN-Host", info, &[("host", Some("192.168.1.20:8191"))]),
        ("ohne Host", info, &[("host", None)]),
        ("localhost als Teil", info, &[("host", Some("localhost.evil.example"))]),
        ("Benutzerteil", info, &[("host", Some("localhost@evil.example"))]),
        ("v6 ohne Klammer-Ende", info, &[("host", Some("[::1:8191"))]),
        ("alle Adressen", info, &[("host", Some("0.0.0.0:8191"))]),
        ("ohne Kopfzeile", info, &[("x-k3c-dev", None)]),
        ("falscher Wert", info, &[("x-k3c-dev", Some("ja"))]),
        ("fremde Origin", info, &[("origin", Some("https://evil.example"))]),
        ("lokale Seite", info, &[("origin", Some("http://127.0.0.1:5183"))]),
        ("Origin null", info, &[("origin", Some("null"))]),
        ("Origin ohne Schema", info, &[("origin", Some("127.0.0.1:8191"))]),
        (
            "Origin mit Benutzer",
            info,
            &[("origin", Some("http://a@127.0.0.1:8191"))],
        ),
        ("cross-site", info, &[("sec-fetch-site", Some("cross-site"))]),
        ("same-site", info, &[("sec-fetch-site", Some("same-site"))]),
    ];
    for (name, path, edits) in cases {
        assert!(judge(local, path, HttpMethod::POST, edits).is_some(), "{name}");
    }
}

#[test]
fn guard_laesst_diesen_rechner_durch() {
    let info = "/api/dev/rpc/Info";
    for peer in ["127.0.0.1:50000", "[::1]:50000", "[::ffff:127.0.0.1]:50000"] {
        assert_eq!(judge(peer, info, HttpMethod::POST, &[]), None, "{peer}");
    }
    for host in ["localhost:5191", "127.0.0.1:5191", "[::1]:5191", "localhost", "[::1]"] {
        let edits = [("host", Some(host))];
        assert_eq!(
            judge("127.0.0.1:1", info, HttpMethod::POST, &edits),
            None,
            "Host {host}"
        );
    }
    // Die Oberfläche hinter Vite: Origin gleich Host, Sec-Fetch-Site same-origin.
    let vite = [
        ("host", Some("localhost:5191")),
        ("origin", Some("http://localhost:5191")),
        ("sec-fetch-site", Some("same-origin")),
    ];
    assert_eq!(judge("127.0.0.1:1", info, HttpMethod::POST, &vite), None);
    // GET braucht die Kopfzeile nicht.
    let bare = [("x-k3c-dev", None)];
    assert_eq!(judge("127.0.0.1:1", "/api/dev/events", HttpMethod::GET, &bare), None);
}

#[tokio::test]
async fn guard_am_server() {
    let app = start(test_api()).await;
    let info = format!("{}rpc/Info", app.url);
    let (status, out) = answer(app.client.post(&info).body("[]")).await;
    assert_eq!((status, out), (403, json!({"error": "Kopfzeile X-K3C-Dev: 1 fehlt"})));
    let foreign = app.client.post(&info).header(HEADER, "1").body("[]");
    let (status, out) = answer(foreign.header("host", "evil.example:8191")).await;
    let wanted = "k3c-test antwortet nur auf localhost oder eine Loopback-Adresse";
    assert_eq!((status, out), (403, json!({ "error": wanted })));
    // Ereignisstrom für eine fremde Seite.
    let events = app.client.get(format!("{}events", app.url));
    let (status, _) = answer(events.header("origin", "http://localhost:5183")).await;
    assert_eq!(status, 403);
    // Zwei Host-Zeilen, und eine absolute Adresse, deren Host die Kopfzeile verdrängt.
    for request in [
        "GET /api/dev/ HTTP/1.1\r\nHost: localhost\r\nHost: evil.example\r\nConnection: close\r\n\r\n",
        "GET http://evil.example/api/dev/ HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    ] {
        assert!(app.raw(request).await.starts_with("HTTP/1.1 403"), "{request}");
    }
    // Der Schutz gilt auch dort, wo keine Route passt.
    let request = "PUT /api/dev/rpc/Info HTTP/1.1\r\nHost: evil.example\r\nConnection: close\r\n\r\n";
    assert!(app.raw(request).await.starts_with("HTTP/1.1 403"));
    let request = "GET /woanders HTTP/1.1\r\nHost: evil.example\r\nConnection: close\r\n\r\n";
    assert!(app.raw(request).await.starts_with("HTTP/1.1 403"));
}

#[tokio::test]
async fn rpc_antwortet() {
    let app = start(test_api()).await;
    assert_eq!(
        app.call("rpc/Info", "[]").await,
        (200, json!({"result": {"version": "test"}}))
    );
    assert_eq!(app.call("rpc/Info", "null").await.0, 200);
    assert_eq!(app.call("rpc/Doc", r#"["a"] rest"#).await.1, json!({"result": "# a"}));
    // Ein Fehler der Methode kommt als {error} mit 422.
    let failed = app.call("rpc/Doc", r#"["gibtsnicht"]"#).await;
    assert_eq!(failed, (422, json!({"error": "unbekanntes Dokument"})));
    let (status, out) = app.call("rpc/Doc", "[]").await;
    assert_eq!(
        (status, out),
        (400, json!({"error": "erwartet 1 Argumente, bekommen 0"}))
    );
    let (status, out) = app.call("rpc/Doc", "[5]").await;
    assert_eq!(status, 400);
    assert!(out["error"].as_str().unwrap().starts_with("ungültiges Argument 1: "));
    for body in ["{}", "", "[\"a\""] {
        let (status, out) = app.call("rpc/Doc", body).await;
        assert_eq!(status, 400, "{body}");
        assert!(out["error"].as_str().unwrap().starts_with("Argumente ungültig: "));
    }
    let huge = format!("[\"{}\"]", "x".repeat(MAX_RPC_BYTES));
    assert_eq!(app.call("rpc/Doc", &huge).await.0, 400);
}

/// Nur die eingetragenen Methoden; andere Adressen und Verben wie im Go-Mux.
#[tokio::test]
async fn rpc_unbekannt() {
    let app = start(test_api()).await;
    for name in ["Intern", "Gibtsnicht", "info"] {
        let (status, out) = app.call(&format!("rpc/{name}"), "[]").await;
        let wanted = json!({ "error": format!("Unbekannte Methode {name}") });
        assert_eq!((status, out), (404, wanted));
    }
    for path in ["", "rpc/Info", "sonst/wo"] {
        let got = answer(app.client.get(format!("{}{path}", app.url))).await;
        assert_eq!(got, (404, json!({"error": "Unbekannte Adresse"})), "{path}");
    }
    assert_eq!(app.call("events", "").await.0, 405);
}

/// Track misst jeden Aufruf unter `ui:<Methode>`, auch einen mit falschen Argumenten.
#[tokio::test]
async fn rpc_track() {
    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let seen = log.clone();
    let mut api = test_api();
    api.track = Some(Arc::new(move |name| {
        seen.lock().unwrap().push(name.to_string());
        let seen = seen.clone();
        Box::new(move || seen.lock().unwrap().push("fertig".to_string()))
    }));
    let app = start(api).await;
    app.call("rpc/Info", "[]").await;
    app.call("rpc/Doc", "{}").await;
    app.call("rpc/Fehlt", "[]").await;
    assert_eq!(*log.lock().unwrap(), ["ui:Info", "fertig", "ui:Doc", "fertig"]);
}

#[test]
fn args_als_tupel() {
    assert_eq!(args::<()>(vec![]), Ok(()));
    assert_eq!(
        args::<()>(vec![json!(1)]),
        Err("erwartet 0 Argumente, bekommen 1".to_string())
    );
    let two = args::<(String, u8)>(vec![json!("a"), json!(2)]);
    assert_eq!(two, Ok(("a".to_string(), 2)));
    let wrong = args::<(String, u8)>(vec![json!("a"), json!("b")]).unwrap_err();
    assert!(wrong.starts_with("ungültiges Argument 2: "), "{wrong}");
}

#[test]
fn pruefung_der_methoden() {
    let ts = "interface GoApp {\n  Info(): Promise<Info>;\n  Doc(name: string): Promise<string>;\n}\n";
    let set = |names: &[&str]| names.iter().map(|n| n.to_string()).collect::<HashSet<_>>();
    assert_eq!(ts_methods(ts).unwrap(), ["Doc", "Info"]);
    assert_eq!(check_methods(ts, &set(&["Info", "Doc"])), Ok(()));
    let missing = check_methods(ts, &set(&["Info"])).unwrap_err();
    assert_eq!(missing, "GoApp = [Doc Info]\nuiMethods = [Info]");
    assert!(check_methods(ts, &set(&["Info", "Doc", "Fehlt"])).is_err());
    assert!(ts_methods("type X = {}").is_err());
}

/// Liest den Strom, bis `lines` Zeilen mit Inhalt da sind; die Frist verhindert Hängen.
async fn read_lines(response: &mut reqwest::Response, text: &mut String, lines: usize) -> Vec<String> {
    let filled = |t: &str| {
        let complete = &t[..t.rfind('\n').map_or(0, |i| i + 1)];
        complete.lines().filter(|l| !l.is_empty()).count()
    };
    while filled(text) < lines {
        let chunk = tokio::time::timeout(Duration::from_secs(5), response.chunk()).await;
        let chunk = chunk.expect("Frist").unwrap().expect("Strom zu früh zu Ende");
        text.push_str(std::str::from_utf8(&chunk).unwrap());
    }
    text.lines().filter(|l| !l.is_empty()).map(String::from).collect()
}

/// `publish` erreicht offene Ströme mit dem ersten Argument als JSON, ohne Argument `null`; das Stoppen beendet sie.
#[tokio::test]
async fn ereignisse() {
    let api = test_api();
    let bus = api.bus.clone();
    let app = start(api).await;
    let request = reqwest::Client::builder().no_proxy().build().unwrap();
    let mut response = request.get(format!("{}events", app.url)).send().await.unwrap();
    assert_eq!(response.headers()["content-type"], "text/event-stream");
    assert_eq!(response.headers()["cache-control"], "no-store");
    let mut text = String::new();
    read_lines(&mut response, &mut text, 1).await;
    assert_eq!(text, ": offen\n\n");
    let state = json!({"addr": "127.0.0.1:5190", "listening": true});
    bus.publish("mcp:state", vec![state, json!("zweites Argument")]);
    bus.publish("boese\nevent: x", vec![]);
    bus.publish("planning:changed", vec![Value::Null]);
    bus.publish("planning:changed", vec![]);
    read_lines(&mut response, &mut text, 7).await;
    let wanted = concat!(
        ": offen\n\n",
        "event: mcp:state\ndata: {\"addr\":\"127.0.0.1:5190\",\"listening\":true}\n\n",
        "event: planning:changed\ndata: null\n\n",
        "event: planning:changed\ndata: null\n\n",
    );
    assert_eq!(text, wanted);
    app.stop.send(()).unwrap();
    let end = tokio::time::timeout(Duration::from_secs(5), response.chunk()).await;
    assert_eq!(end.expect("Strom endet nicht").unwrap(), None);
    let server = tokio::time::timeout(Duration::from_secs(5), app.server).await;
    server.expect("Server endet nicht").unwrap().unwrap();
}

#[test]
fn port_aus_der_umgebung() {
    assert_eq!(resolve_port("X", " ", 8191), Ok(8191));
    assert_eq!(resolve_port("X", "8193", 8191), Ok(8193));
    for raw in ["abc", "0", "70000", "-1"] {
        assert!(resolve_port("X", raw, 8191).is_err(), "{raw}");
    }
    let message = resolve_port("X", "abc", 1).unwrap_err();
    assert_eq!(message, "X=\"abc\" ist keine gültige Portzahl (1–65535)");
}

#[tokio::test]
async fn belegter_port() {
    let first = listen("X", 0).await.unwrap();
    let port = first.local_addr().unwrap().port();
    let refused = listen_within("X", port, Duration::from_millis(150)).await;
    let message = refused.map(|_| ()).unwrap_err();
    assert!(message.starts_with(&format!("oberflächen-port {port} nicht verfügbar (")));
    assert!(message.ends_with("); einen anderen über X setzen"));
    let release = async {
        tokio::time::sleep(Duration::from_millis(250)).await;
        drop(first);
    };
    let (second, ()) = tokio::join!(listen_within("X", port, Duration::from_secs(5)), release);
    assert_eq!(second.unwrap().local_addr().unwrap().port(), port);
}
