//! Die beiden Lesewege in der Oberflächen-API, hinter demselben Wächter wie die Aufrufe (Go `uiHandler`). Dateien gehen
//! stückweise über die Leitung.

use std::io::Read;
use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use k3c_kit::uikit;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use super::{Body, FRAME_PREFIX, MEDIA_PREFIX, Reply, reply};
use crate::app::{APP_NAME, App};

/// Größe eines Stücks beim Ausliefern.
const CHUNK: usize = 64 * 1024;

/// Liest `len` Bytes in einem eigenen Thread; ein geschlossener Empfänger (Player springt) beendet ihn.
fn stream(file: std::fs::File, len: u64) -> axum::body::Body {
    let (send, receive) = mpsc::channel::<std::io::Result<Vec<u8>>>(2);
    tokio::task::spawn_blocking(move || {
        let mut rest = file.take(len);
        loop {
            let mut chunk = vec![0; CHUNK];
            let part = match rest.read(&mut chunk) {
                Ok(0) => return,
                Ok(n) => {
                    chunk.truncate(n);
                    Ok(chunk)
                }
                Err(e) => Err(e),
            };
            let failed = part.is_err();
            if send.blocking_send(part).is_err() || failed {
                return;
            }
        }
    });
    axum::body::Body::from_stream(ReceiverStream::new(receive))
}

fn respond(answer: Reply) -> Response {
    let headers = answer.headers();
    let body = match answer.body {
        Body::Bytes(bytes) => axum::body::Body::from(bytes),
        Body::File { file, len } => stream(file, len),
    };
    (answer.status, headers, body).into_response()
}

async fn serve(State(app): State<Arc<App>>, method: Method, uri: Uri, headers: HeaderMap) -> Response {
    let path = uri.path().to_owned();
    let answer = tokio::task::spawn_blocking(move || reply(&app.playtest_store(), &method, &path, &headers)).await;
    answer.map_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response(), respond)
}

/// Router der Player- und Marker-Adressen samt Schutz.
pub(crate) fn router(app: &Arc<App>) -> Router {
    let routes = Router::new()
        .route(&format!("{MEDIA_PREFIX}{{id}}/{{file}}"), get(serve))
        .route(&format!("{FRAME_PREFIX}{{id}}/{{nr}}"), get(serve))
        .with_state(Arc::clone(app));
    uikit::guard(APP_NAME, routes)
}
