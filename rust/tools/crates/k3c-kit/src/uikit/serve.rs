//! Port, Listener und Lebenszeit des Servers der Oberfläche (`uikit/serve.go`, Warten wie `devrun.Listen`).

use std::future::{Future, IntoFuture};
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::time::Duration;

use axum::{Extension, Router};
use tokio::net::TcpListener;
use tokio::sync::{oneshot, watch};
use tokio::time::Instant;

use super::events::StreamStop;

/// So lange wartet [`listen`] auf den Port, den ein eben beendeter Vorgänger noch hält.
const LISTEN_WAIT: Duration = Duration::from_secs(10);
/// So lange wartet [`serve`] beim Beenden höchstens auf offene Anfragen.
const STOP_TIMEOUT: Duration = Duration::from_secs(2);

/// Liest den Port der Oberflächen-API aus `raw` (Wert der Umgebungsvariable `env`); leer heißt `default`.
pub fn resolve_port(env: &str, raw: &str, default: u16) -> Result<u16, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(default);
    }
    match raw.parse::<u16>() {
        Ok(port) if port > 0 => Ok(port),
        _ => Err(format!("{env}={raw:?} ist keine gültige Portzahl (1–65535)")),
    }
}

/// Öffnet `127.0.0.1:port`; ein Vorgänger, der den Port noch hält, bekommt bis zu 10 s Zeit.
pub async fn listen(env: &str, port: u16) -> Result<TcpListener, String> {
    listen_within(env, port, LISTEN_WAIT).await
}

pub(super) async fn listen_within(env: &str, port: u16, wait: Duration) -> Result<TcpListener, String> {
    let deadline = Instant::now() + wait;
    loop {
        match TcpListener::bind(("127.0.0.1", port)).await {
            Ok(listener) => return Ok(listener),
            Err(e) if e.kind() == ErrorKind::AddrInUse && Instant::now() <= deadline => {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            Err(e) => {
                return Err(format!(
                    "oberflächen-port {port} nicht verfügbar ({e}); einen anderen über {env} setzen"
                ));
            }
        }
    }
}

/// Adresse des Listeners als http-URL mit abschließendem Schrägstrich.
pub fn page_url(listener: &TcpListener) -> String {
    let address = listener.local_addr().map(|a| a.to_string()).unwrap_or_default();
    format!("http://{address}/")
}

/// Beantwortet `router` auf `listener`, bis `shutdown` fertig ist. Dann enden die SSE-Ströme, offene Anfragen bekommen
/// höchstens 2 s.
pub async fn serve(
    listener: TcpListener,
    router: Router,
    shutdown: impl Future<Output = ()> + Send,
) -> std::io::Result<()> {
    let (stop_streams, streams_stopped) = watch::channel(false);
    let (stop_server, server_stopped) = oneshot::channel::<()>();
    let service = router
        .layer(Extension(StreamStop(streams_stopped)))
        .into_make_service_with_connect_info::<SocketAddr>();
    let server = axum::serve(listener, service)
        .with_graceful_shutdown(async move {
            let _ = server_stopped.await;
        })
        .into_future();
    tokio::pin!(server);
    tokio::select! {
        result = &mut server => return result,
        () = shutdown => {}
    }
    stop_streams.send_replace(true);
    let _ = stop_server.send(());
    tokio::time::timeout(STOP_TIMEOUT, server).await.unwrap_or(Ok(()))
}
