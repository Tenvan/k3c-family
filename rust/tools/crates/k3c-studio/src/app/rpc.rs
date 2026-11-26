//! Die eine Aufruf-Tabelle der Oberfläche (V4, Port von `uiapi.go`): je Methode von `GoApp` ein Eintrag. Sie bedient den
//! Tauri-Command `rpc` und die UI-API (`k3c_kit::uikit`, `POST /api/dev/rpc/<Methode>`, Strom `/api/dev/events`).

use std::collections::HashMap;
use std::sync::Arc;

use k3c_kit::uikit::{self, Method};
use serde::Serialize;
use serde_json::Value;

use super::App;

#[cfg(test)]
#[path = "rpc_tests.rs"]
mod tests;

/// Eine Methode: Argumente als JSON-Array, Ergebnis oder Fehlertext. Sie darf blockieren.
pub type Handler = fn(&App, &[Value]) -> Result<Value, String>;

/// Alle Methoden nach Namen wie in Go. Spätere Sessions ergänzen hier und kürzen `PENDING_METHODS`.
pub const METHODS: &[(&str, Handler)] = &[("Info", info)];

fn encode(value: impl Serialize) -> Result<Value, String> {
    serde_json::to_value(value).map_err(|e| e.to_string())
}

fn info(app: &App, args: &[Value]) -> Result<Value, String> {
    uikit::args::<()>(args.to_vec())?;
    encode(app.info())
}

fn find(name: &str) -> Option<Handler> {
    METHODS
        .iter()
        .find(|(method, _)| *method == name)
        .map(|(_, handler)| *handler)
}

/// Ruft die Methode `name` auf (Tauri-Command).
pub fn call(app: &App, name: &str, args: &[Value]) -> Result<Value, String> {
    match find(name) {
        Some(handler) => handler(app, args),
        None => Err(format!("Unbekannte Methode {name}")),
    }
}

/// Die UI-API über der Tabelle. Braucht eine Laufzeit mit mehreren Threads.
pub fn api(app: &Arc<App>) -> uikit::Api {
    let methods: HashMap<String, Method> = METHODS
        .iter()
        .map(|&(name, handler)| {
            let app = Arc::clone(app);
            let method: Method = Arc::new(move |args: Vec<Value>| {
                let app = Arc::clone(&app);
                // `block_in_place` statt eigener Task: so merkt sich uikit abgelehnte Argumente (400 statt 422).
                Box::pin(async move { tokio::task::block_in_place(|| handler(&app, &args)) })
            });
            (name.to_owned(), method)
        })
        .collect();
    uikit::Api {
        app: super::LOG_SOURCE.to_owned(),
        methods,
        bus: app.bus(),
        track: None,
    }
}
