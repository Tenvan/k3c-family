//! Oberflächen-API über der Aufruf-Tabelle (Port von `uiapi.go`): `POST /api/dev/rpc/<Methode>`, Strom
//! `/api/dev/events`, Schutz und Fehlercodes aus `k3c_kit::uikit`.

use std::collections::HashMap;
use std::sync::Arc;

use k3c_kit::uikit::{self, Method};
use serde_json::Value;

use super::METHODS;
use crate::app::{APP_NAME, App, playtest_media};

#[cfg(test)]
#[path = "uiapi_tests.rs"]
mod tests;

fn api(app: &Arc<App>) -> uikit::Api {
    let methods: HashMap<String, Method> = METHODS
        .iter()
        .map(|&(name, handler)| {
            let app = Arc::clone(app);
            let method: Method = Arc::new(move |args: Vec<Value>| {
                let app = Arc::clone(&app);
                // `block_in_place` statt `spawn_blocking`: nur so merkt sich uikit abgelehnte Argumente (400 statt 422).
                Box::pin(async move { tokio::task::block_in_place(|| handler(&app, &args)).map_err(|e| e.0) })
            });
            (name.to_owned(), method)
        })
        .collect();
    let tracked = Arc::clone(app);
    uikit::Api {
        app: APP_NAME.to_owned(),
        methods,
        bus: app.bus(),
        track: Some(Arc::new(move |name: &str| tracked.track(name))),
    }
}

/// Alle Adressen unter `/api/dev/` (Aufrufe, Strom, Player und Marker-Bilder). Braucht eine Laufzeit mit mehreren
/// Threads und `uikit::serve`.
pub fn router(app: &Arc<App>) -> axum::Router {
    api(app).router().merge(playtest_media::router(app))
}
