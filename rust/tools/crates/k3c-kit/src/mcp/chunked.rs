//! Erzwingt chunked für jede Antwort ohne Event-Stream (Port von `mcpkit/chunked_writer.go`).
//!
//! Hintergrund: Auf manchen Rechnern schreibt ein Netzwerkfilter (vermutlich eine Schutzsoftware) HTTP-Antworten auf
//! MCP-Verkehr um. Antworten mit `Content-Length` (das 202 auf `notifications/initialized`, Fehlerantworten) macht er
//! zu „chunked“ ohne Chunk-Rahmen und ohne Abschluss; Claude Code liest den Body zu Ende und läuft in sein
//! 30-s-Timeout. Von Haus aus chunked gesendete Antworten (SSE) lässt der Filter unverändert durch.

use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use axum::middleware::Next;
use axum::response::Response;

/// Axum-Middleware: ohne bekannte Länge sendet hyper chunked.
pub(super) async fn force_chunked(request: Request, next: Next) -> Response {
    let (mut parts, body) = next.run(request).await.into_parts();
    let event_stream = parts
        .headers
        .get(CONTENT_TYPE)
        .is_some_and(|value| value == "text/event-stream");
    let code = parts.status;
    if event_stream || code.is_informational() || code == StatusCode::NO_CONTENT || code == StatusCode::NOT_MODIFIED {
        return Response::from_parts(parts, body);
    }
    parts.headers.remove(CONTENT_LENGTH);
    Response::from_parts(parts, Body::from_stream(body.into_data_stream()))
}

#[cfg(test)]
#[path = "chunked_tests.rs"]
mod tests;
