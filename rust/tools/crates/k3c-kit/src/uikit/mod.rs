//! HTTP-Schicht der Oberfläche im Browser-Dev-Betrieb (`workkit/uikit`): jede freigegebene Methode ist ein POST auf
//! `PREFIX + "rpc/{method}"` mit den Argumenten als JSON-Array, Antwort `{result}` oder `{error}`; Ereignisse kommen als
//! Server-Sent Events ([`Bus`]). Die Oberfläche spricht sie über `tools/webkit/lib/devRpc.ts` an. Statt der
//! Go-Reflexion trägt jede App eine Aufruf-Tabelle ([`Method`]), die auch ihre Tauri-Commands bedient.

mod args;
mod events;
mod guard;
mod methods;
mod serve;

pub use args::args;
pub use events::Bus;
pub use guard::{guard, host_allowed, loopback, same_site};
pub use methods::{check_methods, ts_methods};
pub use serve::{listen, page_url, resolve_port, serve};

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use axum::Router;
use axum::body::Bytes;
use axum::extract::rejection::BytesRejection;
use axum::extract::{DefaultBodyLimit, Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use serde::Deserialize;
use serde_json::{Value, json};

/// Prefix aller Adressen der Oberfläche.
pub const PREFIX: &str = "/api/dev/";

/// Kopfzeile, die jeder POST verlangt (Wert `1`): ein fremder Web-Auftritt kann sie ohne Vorabfrage (CORS-Preflight)
/// nicht setzen, der Server beantwortet keine.
pub const HEADER: &str = "X-K3C-Dev";

/// Größte Anfrage eines Aufrufs (Prompt für Claude, Task-Argumente).
pub const MAX_RPC_BYTES: usize = 1 << 20;

/// Zukunft eines Aufrufs.
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

/// Eine Methode der Oberfläche: JSON-Argumente rein, Ergebnis oder Fehlertext raus. Die Argumente liest sie zuerst mit
/// [`args`], dann antwortet die HTTP-API bei falschen Argumenten mit 400 statt 422.
pub type Method = Arc<dyn Fn(Vec<Value>) -> BoxFuture<Result<Value, String>> + Send + Sync>;

/// Misst einen Aufruf: bekommt `ui:<Methode>` und liefert, was am Ende läuft.
pub type Track = Arc<dyn Fn(&str) -> Box<dyn FnOnce() + Send> + Send + Sync>;

/// Beantwortet die Aufrufe der Oberfläche.
pub struct Api {
    /// Name in Meldungen, z. B. `k3c-workbench`.
    pub app: String,
    /// Freigegebene Methoden nach Namen.
    pub methods: HashMap<String, Method>,
    /// Ereignisse für den Strom.
    pub bus: Arc<Bus>,
    /// Ressourcen-Monitor; `None` misst nichts.
    pub track: Option<Track>,
}

impl Api {
    /// Alle Adressen unter [`PREFIX`] hinter [`guard`]; die Oberfläche selbst liefert Vite. Der Router braucht die
    /// Gegenstelle der Verbindung, also [`serve`] (oder `into_make_service_with_connect_info`), sonst lehnt er ab.
    pub fn router(self) -> Router {
        let app = self.app.clone();
        let router = Router::new()
            .route(&format!("{PREFIX}events"), get(events::stream))
            // GET auf rpc: wie im Go-Mux „Unbekannte Adresse“ statt 405.
            .route(&format!("{PREFIX}rpc/{{method}}"), post(rpc).get(unknown))
            .route(PREFIX, get(unknown))
            .route(&format!("{PREFIX}{{*rest}}"), get(unknown))
            .layer(DefaultBodyLimit::max(MAX_RPC_BYTES))
            .with_state(Arc::new(self));
        guard(app, router)
    }
}

/// Antwortet `{"result": …}` bzw. `{"error": …}`.
fn reply(status: StatusCode, result: Result<Value, String>) -> Response {
    let body = match result {
        Ok(value) => json!({ "result": value }),
        Err(message) => json!({ "error": message }),
    };
    let headers = [
        (header::CONTENT_TYPE, "application/json"),
        (header::CACHE_CONTROL, "no-store"),
    ];
    // Zeilenende wie Go `json.Encoder`.
    (status, headers, format!("{body}\n")).into_response()
}

/// Antwortet mit einem Fehlertext.
fn fail(status: StatusCode, message: impl Into<String>) -> Response {
    reply(status, Err(message.into()))
}

async fn unknown() -> Response {
    fail(StatusCode::NOT_FOUND, "Unbekannte Adresse")
}

/// Ruft das Ende der Messung beim Fallenlassen auf (Go `defer`).
struct Tracked(Option<Box<dyn FnOnce() + Send>>);

impl Drop for Tracked {
    fn drop(&mut self) {
        if let Some(done) = self.0.take() {
            done();
        }
    }
}

/// Liest wie Go `json.Decoder` nur den ersten JSON-Wert des Rumpfs; `null` ist eine leere Liste.
fn decode_body(body: Result<Bytes, BytesRejection>) -> Result<Vec<Value>, String> {
    let body = body.map_err(|e| e.body_text())?;
    let mut json = serde_json::Deserializer::from_slice(&body);
    let list = Option::<Vec<Value>>::deserialize(&mut json).map_err(|e| e.to_string())?;
    Ok(list.unwrap_or_default())
}

/// Ruft die Methode `{method}` mit den JSON-Argumenten aus dem Rumpf (ein Array) auf.
async fn rpc(State(api): State<Arc<Api>>, Path(name): Path<String>, body: Result<Bytes, BytesRejection>) -> Response {
    let Some(method) = api.methods.get(&name).cloned() else {
        return fail(StatusCode::NOT_FOUND, format!("Unbekannte Methode {name}"));
    };
    let tracked = Tracked(api.track.as_ref().map(|track| track(&format!("ui:{name}"))));
    let values = match decode_body(body) {
        Ok(values) => values,
        Err(e) => return fail(StatusCode::BAD_REQUEST, format!("Argumente ungültig: {e}")),
    };
    // Eigener Task: legt der Browser auf, läuft die Methode wie in Go zu Ende.
    let call = tokio::spawn(async move {
        let _tracked = tracked;
        args::watch(async move { method(values).await }).await
    });
    match call.await {
        Ok((Ok(result), _)) => reply(StatusCode::OK, Ok(result)),
        Ok((Err(e), true)) => fail(StatusCode::BAD_REQUEST, e),
        Ok((Err(e), false)) => fail(StatusCode::UNPROCESSABLE_ENTITY, e),
        Err(_) => fail(StatusCode::INTERNAL_SERVER_ERROR, format!("Methode {name} abgebrochen")),
    }
}

#[cfg(test)]
mod uikit_tests;
