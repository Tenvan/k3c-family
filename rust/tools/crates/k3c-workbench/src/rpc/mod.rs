//! Die eine Aufruf-Tabelle der Oberfläche (Port von `uiMethods` und den Bindings): je Methode von `GoApp` ein Eintrag.
//! Sie bedient den Tauri-Command `rpc` und die Oberflächen-API ([`uiapi`]). Die Argumente stehen positionsgleich wie in
//! Wails im JSON-Array.

mod logs;
mod services;
mod tasks;
pub mod uiapi;

use std::sync::Arc;

use k3c_kit::uikit;
use serde::Serialize;
use serde_json::Value;
use tauri::State;

use crate::app::App;
use crate::app::window::AppState;

/// Fehlertext eines Aufrufs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RpcError(pub String);

impl From<String> for RpcError {
    fn from(message: String) -> Self {
        RpcError(message)
    }
}

/// Eine Methode: Argumente rein, Ergebnis oder Fehler raus. Sie darf blockieren.
pub type Handler = fn(&Arc<App>, &[Value]) -> Result<Value, RpcError>;

/// Alle Methoden nach Namen wie in Go.
pub const METHODS: &[(&str, Handler)] = &[
    ("Info", info),
    ("McpOverview", mcp_overview),
    ("McpRestart", mcp_restart),
    ("McpInstructions", mcp_instructions),
    ("McpCalls", mcp_calls),
    ("McpUsage", mcp_usage),
    ("Services", services::view),
    ("ServicesReload", services::reload),
    ("ServiceStart", services::start),
    ("ServiceStop", services::stop),
    ("ServiceRestart", services::restart),
    ("ServicesStartAll", services::start_all),
    ("ServicesStopAll", services::stop_all),
    ("ServiceLogLevels", services::log_levels),
    ("Sources", tasks::sources),
    ("ConsoleTail", tasks::console_tail),
    ("Tasks", tasks::catalog),
    ("TasksReload", tasks::reload),
    ("TaskStart", tasks::start),
    ("TaskStop", tasks::stop),
    ("TaskRuns", tasks::runs),
    ("LogsQuery", logs::query),
    ("LogsErrors", logs::errors),
];

/// Das eine Argument einer Methode mit einem Namen.
fn name(args: &[Value]) -> Result<String, RpcError> {
    let (name,) = uikit::args::<(String,)>(args.to_vec())?;
    Ok(name)
}

fn encode(value: impl Serialize) -> Result<Value, RpcError> {
    serde_json::to_value(value).map_err(|e| RpcError(e.to_string()))
}

/// Methoden ohne Argument lehnen jedes ab.
fn no_args(args: &[Value]) -> Result<(), RpcError> {
    Ok(uikit::args::<()>(args.to_vec())?)
}

fn info(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.info())
}

fn mcp_overview(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.mcp_overview()?)
}

fn mcp_restart(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.mcp_restart()?)
}

fn mcp_instructions(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.mcp_instructions())
}

fn mcp_calls(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.mcp_calls()?)
}

fn mcp_usage(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.mcp_usage()?)
}

/// Ruft die Methode `name` auf und misst sie.
pub fn call(app: &Arc<App>, name: &str, args: &[Value]) -> Result<Value, RpcError> {
    let Some((_, handler)) = METHODS.iter().find(|(method, _)| *method == name) else {
        return Err(RpcError(format!("Unbekannte Methode {name}")));
    };
    let done = app.track(&format!("ui:{name}"));
    let result = handler(app, args);
    done();
    result
}

/// Command `rpc {method, args}` des Fensters.
#[tauri::command]
pub(crate) async fn rpc(state: State<'_, AppState>, method: String, args: Vec<Value>) -> Result<Value, String> {
    let app = Arc::clone(&state.0);
    tauri::async_runtime::spawn_blocking(move || call(&app, &method, &args).map_err(|e| e.0))
        .await
        .map_err(|e| e.to_string())?
}
