//! `level_generate` rechnet mit der Spiel-EXE des Checkouts (`k3c-game sim`, Port von `tools_engine.go`); ist sie nicht
//! gebaut oder veraltet, nennt der Fehler den Bau-Befehl. `sim_run` (RU70.20) nutzt denselben Aufruf.

use std::time::Duration;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::kit::Workspace;
use crate::gameexe::{self, Cancel, Event, Options};

#[cfg(test)]
#[path = "tools_engine_tests.rs"]
mod tests;

/// Begrenzt einen Aufruf (auch `sim_run`, 100000 Ticks dauern Sekunden).
pub(super) const ENGINE_TIMEOUT: Duration = Duration::from_secs(120);

/// Argumente von `level_generate`; so gehen sie auch als Eingabe-JSON an die EXE.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(super) struct LevelIn {
    #[schemars(description = "Seed als Text, z. B. 0 oder familie-2026")]
    seed: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[schemars(description = "forest (Standard), cave oder mine")]
    biome: String,
}

/// Aufruf der EXE, der nach [`ENGINE_TIMEOUT`] oder mit `canceled` abbricht.
pub(super) async fn game_call(
    options: Options,
    canceled: impl Future<Output = ()> + Send,
) -> Result<Event, gameexe::Error> {
    game_call_within(options, ENGINE_TIMEOUT, canceled).await
}

/// Wie [`game_call`] mit eigener Frist (`sim_run` mit Fenster, `replay_run`).
pub(super) async fn game_call_within(
    options: Options,
    limit: Duration,
    canceled: impl Future<Output = ()> + Send,
) -> Result<Event, gameexe::Error> {
    let stop = async {
        tokio::select! {
            () = canceled => Cancel::Canceled,
            () = tokio::time::sleep(limit) => Cancel::Deadline,
        }
    };
    gameexe::call(options, stop).await
}

/// Startet `k3c-game sim <sub>` aus dem Checkout des Aufrufs (RU34.2) und liefert den Text der Zeile `result`. `call`
/// ist [`game_call`]; Tests ersetzen ihn.
async fn engine_call<F>(
    ws: &Workspace,
    sub: &str,
    input: &impl Serialize,
    call: impl FnOnce(Options) -> F,
) -> Result<String, String>
where
    F: Future<Output = Result<Event, gameexe::Error>>,
{
    let options = Options {
        root: ws.root.clone(),
        sub: sub.to_owned(),
        input: serde_json::to_vec(input).map_err(|e| e.to_string())?,
        ..Options::default()
    };
    call(options).await.map(|ev| ev.text).map_err(|e| e.to_string())
}

/// Das Tool `level_generate` (`k3c-game sim level`, kein Server nötig).
pub(super) async fn level_generate<F>(
    ws: &Workspace,
    input: &LevelIn,
    call: impl FnOnce(Options) -> F,
) -> Result<String, String>
where
    F: Future<Output = Result<Event, gameexe::Error>>,
{
    engine_call(ws, "level", input, call).await
}
