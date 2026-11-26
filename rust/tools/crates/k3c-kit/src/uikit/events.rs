//! Ereignisse der App als Server-Sent Events (`uikit/events.go`): je Ereignis `event: <name>` und `data: <JSON>`.

use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::Extension;
use axum::extract::State;
use axum::http::header;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use serde_json::Value;
use tokio::sync::{broadcast, watch};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::{BroadcastStream, WatchStream};

use super::Api;

/// Ereignisse, die ein Strom fasst, bevor der Browser sie abholt. Bei Überlauf fallen Ereignisse weg (die Oberfläche
/// lädt Listen ohnehin beim Seitenwechsel neu).
const EVENT_BUFFER: usize = 1024;

/// Hält Proxys (Vite) und Browser vom Schließen eines ruhigen Stroms ab.
const KEEP_ALIVE: Duration = Duration::from_secs(20);

/// Ein Ereignis mit fertig kodierter Nutzlast.
#[derive(Clone)]
struct BusEvent {
    name: Arc<str>,
    data: Arc<str>,
}

/// Verteilt die Ereignisse der App an die offenen SSE-Ströme.
pub struct Bus {
    sender: broadcast::Sender<BusEvent>,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    /// Ein leerer Bus.
    pub fn new() -> Bus {
        Bus {
            sender: broadcast::Sender::new(EVENT_BUFFER),
        }
    }

    /// Kodiert die Nutzlast wie Wails: das erste Argument, ohne Argument `null`. Ein Name mit Zeilenumbruch könnte
    /// fremde Zeilen in den Strom schreiben und wird verworfen.
    pub fn publish(&self, name: &str, data: Vec<Value>) {
        if name.contains(['\r', '\n']) {
            return;
        }
        let payload = data.into_iter().next().unwrap_or(Value::Null);
        // Ohne offenen Strom gibt es niemanden, dem etwas fehlt.
        let _ = self.sender.send(BusEvent {
            name: name.into(),
            data: payload.to_string().into(),
        });
    }
}

/// Signal von [`super::serve`], das beim Stoppen alle Ströme beendet (Go: abgebrochener Anfrage-Kontext).
#[derive(Clone)]
pub(super) struct StreamStop(pub(super) watch::Receiver<bool>);

/// `GET PREFIX + "events"`: der Strom beginnt mit `: offen` und hält sich mit `: halten` offen.
pub(super) async fn stream(State(api): State<Arc<Api>>, stop: Option<Extension<StreamStop>>) -> Response {
    let events = api.bus.sender.subscribe();
    // Ohne `serve` gibt es kein Signal: der Sender fällt sofort weg, der Wert bleibt `false`.
    let stop = stop.map_or_else(|| watch::channel(false).1, |Extension(s)| s.0);
    let stopped = WatchStream::new(stop).filter(|stopped| *stopped).map(|_| None);
    let live = BroadcastStream::new(events)
        .filter_map(|event| event.ok())
        .map(|event| Some(Event::default().event(&*event.name).data(&*event.data)));
    let rest = live
        .merge(stopped)
        .take_while(Option::is_some)
        .filter_map(|event| event);
    let all = tokio_stream::once(Event::default().comment("offen"))
        .chain(rest)
        .map(Ok::<_, Infallible>);
    let sse = Sse::new(all).keep_alive(KeepAlive::new().interval(KEEP_ALIVE).text("halten"));
    ([(header::CACHE_CONTROL, "no-store")], sse).into_response()
}
