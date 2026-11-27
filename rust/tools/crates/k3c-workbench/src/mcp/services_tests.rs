//! Port von `tools_services_test.go` und `services_console_test.go` (ohne den Teil `logs_services`, RU69.5). Alle Nähte
//! des Controllers sind gestellt: kein Test startet, prüft oder beendet einen echten Prozess.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use k3c_kit::console::Store;
use serde_json::{Value, json};

use super::test_client::Client;
use super::{Config, Server};
use crate::services::{Cancel, Controller, Metrics, Options, Service, Spawned, State};

#[derive(Default)]
pub(super) struct Stubs {
    pub healthy: AtomicBool,
    pub busy: Arc<AtomicBool>,
}

/// Läuft, bis er beendet wird.
fn stub_proc() -> Spawned {
    let done = Cancel::default();
    let kill = done.clone();
    Spawned {
        pid: 42,
        kill: Box::new(move || {
            kill.cancel();
            Ok(())
        }),
        wait: Box::pin(async move {
            done.cancelled().await;
            Ok(())
        }),
    }
}

pub(super) fn service(name: &str, port: i64) -> Service {
    Service {
        name: name.to_owned(),
        command: vec!["x".to_owned()],
        port,
        health: "http".to_owned(),
        ..Service::default()
    }
}

pub(super) fn controller(stubs: &Arc<Stubs>, store: &Arc<Store>, list: Vec<Service>) -> Arc<Controller> {
    let (health, listen, kill) = (Arc::clone(stubs), Arc::clone(stubs), Arc::clone(stubs));
    let options = Options {
        console: Arc::clone(store),
        start: Arc::new(|svc: &Service, _: &Path, out| {
            out("stdout", &format!("{}: Zeile 1", svc.name));
            out("stderr", &format!("{}: Port-Problem", svc.name));
            Ok(stub_proc())
        }),
        check: Arc::new(move |_: &Service| {
            if health.healthy.load(Ordering::SeqCst) {
                Ok(())
            } else {
                Err("antwortet nicht".to_owned())
            }
        }),
        listen: Arc::new(move |_| (4711, listen.busy.load(Ordering::SeqCst))),
        sample: Arc::new(|_| {
            Ok(Metrics {
                cpu: 0.0,
                memory: 0,
                started: None,
            })
        }),
        kill_pid: Arc::new(move |_| {
            kill.busy.store(false, Ordering::SeqCst);
            Ok(())
        }),
        supervisor: Arc::new(|_| 0),
        start_poll: Duration::from_millis(2),
        start_timeout: Duration::from_millis(60),
        watch_every: Duration::from_secs(3600),
        ..Options::default()
    };
    Controller::new(list, options)
}

struct Fixture {
    server: Server,
    ctl: Arc<Controller>,
    client: Client,
}

impl Fixture {
    async fn call(&mut self, name: &str, arguments: Value) -> (String, bool) {
        self.client.call_text(name, arguments).await
    }

    async fn end(self) {
        self.ctl.stop_all().await;
        self.server.stop().await.unwrap();
    }
}

fn vite_and_server() -> Vec<Service> {
    vec![service("Vite", 5173), service("Spielserver", 8080)]
}

async fn fixture_with(stubs: &Arc<Stubs>, adjust: impl FnOnce(&mut Vec<Service>)) -> Fixture {
    let store = Arc::new(Store::new(0, None));
    let mut list = vite_and_server();
    adjust(&mut list);
    let ctl = controller(stubs, &store, list);
    let server = Server::new(Config {
        console: Some(store),
        services: Some(Ok(Arc::clone(&ctl))),
        ..Config::default()
    });
    server.start().await.unwrap();
    let client = Client::connect(server.url()).await;
    Fixture { server, ctl, client }
}

async fn fixture(stubs: &Arc<Stubs>) -> Fixture {
    fixture_with(stubs, |_| {}).await
}

fn stubs(healthy: bool) -> Arc<Stubs> {
    let stubs = Stubs::default();
    stubs.healthy.store(healthy, Ordering::SeqCst);
    Arc::new(stubs)
}

/// `TestSvcToolsStartStatusStop`.
#[tokio::test(flavor = "multi_thread")]
async fn start_status_stop() {
    let mut f = fixture(&stubs(true)).await;
    let (text, is_error) = f.call("svc_start", json!({"service": "Vite"})).await;
    assert!(
        !is_error && text.starts_with("Vite · läuft · Port 5173 · PID 42 · < 1 min · "),
        "svc_start: {text}"
    );
    assert!(text.contains("\nCheckout: Repo-Wurzel "), "{text}");
    let (text, is_error) = f.call("svc_start", json!({"service": "Vite"})).await;
    assert!(is_error && text.contains("läuft bereits"), "zweiter Start: {text}");
    let (text, _) = f.call("svc_status", json!({})).await;
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 2, "{text}");
    assert!(lines[1].starts_with("Spielserver · gestoppt · Port 8080"), "{text}");
    let (text, _) = f.call("workbench_status", json!({})).await;
    assert!(
        text.contains("Dienste: Vite läuft (5173) · Spielserver gestoppt (8080)"),
        "{text}"
    );
    let (text, is_error) = f.call("svc_restart", json!({"service": "Vite", "confirm": true})).await;
    assert!(!is_error && text.starts_with("Vite · läuft"), "svc_restart: {text}");
    let (text, is_error) = f.call("svc_stop", json!({"service": "Vite"})).await;
    assert!(
        !is_error && text.starts_with("Vite · gestoppt · Port 5173"),
        "svc_stop: {text}"
    );
    let (text, is_error) = f.call("svc_stop", json!({"service": "Gibtsnicht"})).await;
    assert!(
        is_error && text.contains("gültig: [Vite Spielserver]"),
        "unbekannter Dienst: {text}"
    );
    f.end().await;
}

/// `TestSvcToolsFehlerMitKonsoleUndForce`.
#[tokio::test(flavor = "multi_thread")]
async fn fehler_mit_konsole_und_force() {
    let stubs = stubs(false);
    let mut f = fixture(&stubs).await;
    let (text, is_error) = f.call("svc_start", json!({"service": "Vite"})).await;
    assert!(is_error && text.contains("nicht gesund"), "{text}");
    assert!(
        text.contains("letzte Zeilen:\nVite: Zeile 1\nVite: Port-Problem"),
        "{text}"
    );
    stubs.healthy.store(true, Ordering::SeqCst);
    stubs.busy.store(true, Ordering::SeqCst);
    f.ctl.adopt().await;
    assert_eq!(f.ctl.statuses()[1].state, State::Adopted);
    let (text, is_error) = f.call("svc_stop", json!({"service": "Spielserver"})).await;
    assert!(is_error && text.contains("nur mit force"), "Stopp ohne force: {text}");
    let arguments = json!({"service": "Spielserver", "force": true});
    let (text, is_error) = f.call("svc_stop", arguments).await;
    assert!(
        !is_error && text.starts_with("Spielserver · gestoppt"),
        "Stopp mit force: {text}"
    );
    f.end().await;
}

/// `TestSvcToolsOhneDienste`.
#[tokio::test(flavor = "multi_thread")]
async fn ohne_dienste() {
    for (services, want) in [
        (
            Some(Err("services.json: unknown field \"x\"".to_owned())),
            "keine Dienste: services.json nicht geladen: services.json: unknown field \"x\"",
        ),
        (None, "keine Dienste konfiguriert"),
    ] {
        let server = Server::new(Config {
            services,
            ..Config::default()
        });
        server.start().await.unwrap();
        let mut client = Client::connect(server.url()).await;
        let (text, is_error) = client.call_text("svc_status", json!({})).await;
        assert!(is_error && text == want, "{text}");
        let (text, is_error) = client.call_text("svc_start", json!({"service": "Vite"})).await;
        assert!(is_error && text.starts_with(want), "{text}");
        let (text, _) = client.call_text("workbench_status", json!({})).await;
        assert!(text.contains(&format!("Dienste: {want}")), "{text}");
        server.stop().await.unwrap();
    }
}

/// `TestSvcToolsHealthPortsUrlsAll`, dazu ein Prozess-Dienst ohne Port.
#[tokio::test(flavor = "multi_thread")]
async fn health_ports_urls() {
    let stubs = stubs(true);
    let mut f = fixture(&stubs).await;
    let (text, _) = f.call("svc_health", json!({"service": "Vite"})).await;
    assert_eq!(text, "Vite · gesund · http://127.0.0.1:5173/");
    stubs.healthy.store(false, Ordering::SeqCst);
    stubs.busy.store(true, Ordering::SeqCst);
    let (text, _) = f.call("svc_health", json!({"service": "Vite"})).await;
    assert_eq!(
        text,
        "Vite · ungesund · http://127.0.0.1:5173/ · antwortet nicht · Port: PID 4711"
    );
    let (text, is_error) = f.call("svc_health", json!({"service": "x"})).await;
    assert!(is_error && text.starts_with("unbekannter Dienst \"x\""), "{text}");
    let (text, _) = f.call("ports_status", json!({})).await;
    assert!(
        text.starts_with("5173 · Vite · PID 4711\n8080 · Spielserver · PID 4711\n127.0.0.1:"),
        "{text}"
    );
    assert!(text.ends_with(" · MCP k3c-workbench · eigener Prozess"), "{text}");
    let (text, _) = f.call("get_urls", json!({"target": "spielserver"})).await;
    assert_eq!(
        text,
        "Spielserver · http://localhost:8080/ · Health http://127.0.0.1:8080/"
    );
    let (text, _) = f.call("get_urls", json!({"target": "mcp"})).await;
    assert_eq!(text, format!("MCP · {}", f.server.url()));
    let (text, _) = f.call("get_urls", json!({})).await;
    assert_eq!(text.lines().count(), 3, "{text}");
    let (text, is_error) = f.call("get_urls", json!({"target": "x"})).await;
    assert!(is_error && text.contains("gültig: Vite, Spielserver, mcp"), "{text}");
    f.end().await;
    let mut f = fixture_with(&stubs, |list| {
        list.push(Service {
            health: "process".to_owned(),
            ..service("game-dev", 0)
        });
    })
    .await;
    let (text, _) = f.call("get_urls", json!({"target": "game-dev"})).await;
    assert_eq!(text, "game-dev · ohne Port · Health process");
    let (text, _) = f.call("ports_status", json!({})).await;
    assert!(!text.contains("game-dev"), "{text}");
    f.end().await;
}

/// `TestSvcToolsConfirmWaitAll`.
#[tokio::test(flavor = "multi_thread")]
async fn confirm_wait_und_alle() {
    let stubs = stubs(false);
    let mut f = fixture(&stubs).await;
    let (text, is_error) = f.call("svc_restart", json!({"service": "Vite"})).await;
    assert!(is_error && text.contains("confirm=true"), "{text}");
    for seconds in [99, -1] {
        let arguments = json!({"service": "Vite", "waitSeconds": seconds});
        let (text, is_error) = f.call("svc_start", arguments).await;
        assert!(is_error && text.contains("erlaubt 1–60"), "waitSeconds: {text}");
    }
    let (text, is_error) = f.call("svc_start_all", json!({})).await;
    assert!(is_error && text.contains("nach 60ms nicht gesund"), "{text}");
    assert!(text.contains("\nVite · fehlgeschlagen · Port 5173"), "{text}");
    stubs.healthy.store(true, Ordering::SeqCst);
    let (text, is_error) = f.call("svc_start_all", json!({})).await;
    assert!(!is_error && text.matches("läuft").count() == 2, "svc_start_all: {text}");
    let (text, is_error) = f.call("svc_stop_all", json!({})).await;
    assert!(
        !is_error && text.matches("gestoppt").count() == 2,
        "svc_stop_all: {text}"
    );
    f.end().await;
}

/// `waitSeconds` bricht das Warten mit der Meldung der Frist ab.
#[tokio::test(flavor = "multi_thread")]
async fn wait_seconds_bricht_ab() {
    let options = Options {
        start_timeout: Duration::from_secs(30),
        check: Arc::new(|_: &Service| Err("antwortet nicht".to_owned())),
        start: Arc::new(|_, _, _| Ok(stub_proc())),
        listen: Arc::new(|_| (0, false)),
        kill_pid: Arc::new(|_| Ok(())),
        supervisor: Arc::new(|_| 0),
        ..Options::default()
    };
    let ctl = Controller::new(vite_and_server(), options);
    let server = Server::new(Config {
        services: Some(Ok(Arc::clone(&ctl))),
        ..Config::default()
    });
    server.start().await.unwrap();
    let mut client = Client::connect(server.url()).await;
    let arguments = json!({"service": "Vite", "waitSeconds": 1});
    let (text, is_error) = client.call_text("svc_start", arguments).await;
    assert!(
        is_error && text.starts_with("Vite: start abgebrochen: context deadline exceeded"),
        "{text}"
    );
    ctl.stop_all().await;
    server.stop().await.unwrap();
}

/// `TestDienstAusgabeInLogsSources`: die Ausgabe eines Dienstes liegt als Quelle in der Konsole.
#[tokio::test(flavor = "multi_thread")]
async fn dienst_ausgabe_in_der_konsole() {
    let mut f = fixture(&stubs(true)).await;
    f.call("svc_start", json!({"service": "Vite"})).await;
    let (text, is_error) = f.call("console_tail", json!({"service": "Vite"})).await;
    assert!(!is_error, "{text}");
    assert_eq!(
        text,
        "Vite · letzte 2 Zeilen (bis Nr. 2)\nVite: Zeile 1\n! Vite: Port-Problem"
    );
    f.end().await;
}
