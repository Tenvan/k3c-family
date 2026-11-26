//! Der eine Ablauf für alle `tools/call` (Port von `mcpkit/observe.go`): Checkout zuordnen, Zähler und Aufruf-Log,
//! unbekannte Felder ablehnen, Panik als Fehler, Parameter-Hinweis, Checkout-Zeile.

use std::any::Any;
use std::sync::Arc;

use rmcp::handler::server::tool::ToolCallContext;
use rmcp::model::{CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, JsonObject};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer};
use serde_json::json;

use super::param_hint::add_param_hint;
use super::progress::Progress;
use super::service::Core;
use super::{HEADER_ROOT, Outcome, PANIC_TEXT, Workspace, clip, clip_args, outcome_of, split_checkout, text_result};
use crate::usage::Event;

type Response = Result<CallToolResponse, ErrorData>;

/// Zuordnung eines Aufrufs zu seinem Checkout.
struct Target {
    workspace: Result<Workspace, String>,
    /// Der Kopf [`HEADER_ROOT`] kam an.
    header: bool,
    /// Das Argument `checkout` hat den Checkout bestimmt.
    checkout_arg: bool,
}

/// Jeder Aufruf arbeitet im Checkout seines Clients (Repo-Wurzel oder Worktree), nie in einem fremden; das Argument
/// `checkout` gilt vor dem Header (B-388). Bei Tools mit diesem Argument wird es aus `arguments` entfernt.
async fn locate<A: Clone + Send + Sync + 'static>(
    core: &Core<A>,
    tool: &str,
    context: &RequestContext<RoleServer>,
    arguments: &mut Option<JsonObject>,
) -> Target {
    let header = context
        .extensions
        .get::<axum::http::request::Parts>()
        .and_then(|parts| parts.headers.get(HEADER_ROOT))
        .map(|value| String::from_utf8_lossy(value.as_bytes()).into_owned())
        .unwrap_or_default();
    let mut target = Target {
        workspace: core.resolver.resolve_header(Some(&header)),
        header: !header.is_empty(),
        checkout_arg: false,
    };
    if !core.opts.takes_checkout.as_ref().is_some_and(|takes| takes(tool)) {
        return target;
    }
    // Klon, weil `split_checkout` die Argumente bei einem Fehler verbraucht; Log und Statistik brauchen sie.
    match split_checkout(arguments.clone()) {
        Ok((value, rest)) => {
            *arguments = rest;
            if !value.is_empty() {
                target.checkout_arg = true;
                target.workspace = core.resolver.resolve_checkout(&value).await;
            }
        }
        Err(e) => target.workspace = Err(e),
    }
    target
}

pub(super) async fn call_tool<A: Clone + Send + Sync + 'static>(
    core: &Arc<Core<A>>,
    mut request: CallToolRequestParams,
    mut context: RequestContext<RoleServer>,
) -> Response {
    let tool = request.name.to_string();
    let target = locate(core, &tool, &context, &mut request.arguments).await;
    // Wie Go: ohne Zuordnung ein leerer Checkout (nur für das Log, der Aufruf läuft dann nicht).
    let workspace = target.workspace.clone().unwrap_or_else(|_| Workspace {
        root: Default::default(),
        name: String::new(),
    });
    let progress = Progress::for_request(&context);
    context.extensions.insert(workspace.clone());
    context.extensions.insert(progress);
    let done = core.opts.track.as_ref().map(|track| track(&format!("tool:{tool}")));
    let args = request
        .arguments
        .as_ref()
        .and_then(|a| serde_json::to_string(a).ok())
        .unwrap_or_default();
    let id = core.stats.begin(&tool, &args);
    let attrs = [
        ("ns", json!("mcp")),
        ("tool", json!(tool)),
        ("args", json!(clip_args(args.as_bytes()))),
    ];
    core.opts.log.debug(&format!("📨 {tool}: start"), &attrs);
    let response = match &target.workspace {
        Err(e) => Ok(text_result(e.clone(), true).into()),
        Ok(workspace) => checked_call(core, &tool, workspace, request, context).await,
    };
    finish(core, id, &tool, &response, &target, &workspace, args);
    if let Some(done) = done {
        done();
    }
    response
}

/// Aufruf mit Prüfung vorher und Hinweisen nachher.
async fn checked_call<A: Clone + Send + Sync + 'static>(
    core: &Arc<Core<A>>,
    tool: &str,
    workspace: &Workspace,
    request: CallToolRequestParams,
    context: RequestContext<RoleServer>,
) -> Response {
    let Some(params) = core.params.get(tool) else {
        return Err(ErrorData::invalid_params(format!("unknown tool {tool:?}"), None));
    };
    let response = match params.reject_unknown(request.arguments.as_ref()) {
        Some(rejected) => rejected.into(),
        None => isolated_call(core, request, context).await?,
    };
    let CallToolResponse::Complete(mut result) = response else {
        return Ok(response);
    };
    // Go lässt isError bei Erfolg weg (`omitempty`).
    if result.is_error == Some(false) {
        result.is_error = None;
    }
    add_param_hint(&params.names, &mut result);
    add_checkout(core, tool, &mut result, workspace);
    Ok(result.into())
}

/// Führt das Tool in einer eigenen Task aus: eine Panik wird zum Fehler-Ergebnis, der Server läuft weiter.
async fn isolated_call<A: Clone + Send + Sync + 'static>(
    core: &Arc<Core<A>>,
    request: CallToolRequestParams,
    context: RequestContext<RoleServer>,
) -> Response {
    let app = core.app.clone();
    let tools = Arc::clone(&core.tools);
    let task = tokio::spawn(async move { tools.call(ToolCallContext::new(&app, request, context)).await });
    match task.await {
        Ok(response) => response,
        Err(e) if e.is_panic() => {
            let message = panic_message(e.into_panic().as_ref());
            Ok(text_result(format!("{PANIC_TEXT}: {message}"), true).into())
        }
        Err(e) => Ok(text_result(e.to_string(), true).into()),
    }
}

fn panic_message(payload: &(dyn Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        return (*s).to_owned();
    }
    payload.downcast_ref::<String>().cloned().unwrap_or_default()
}

/// Hängt an die Antwort eines schreibenden Tools die Zeile `Checkout: …`. Sie nennt den Checkout immer, auch wenn der
/// Header fehlt und die Repo-Wurzel gilt (B-275): nicht ablehnen, aber nie still in die Wurzel schreiben.
fn add_checkout<A: Clone + Send + Sync + 'static>(
    core: &Core<A>,
    tool: &str,
    result: &mut CallToolResult,
    workspace: &Workspace,
) {
    if core.opts.write_tool.as_ref().is_some_and(|writes| writes(tool)) {
        let line = format!("Checkout: {}", workspace.label());
        result.content.push(ContentBlock::text(line));
    }
}

fn outcome(response: &Response) -> Outcome {
    match response {
        Ok(CallToolResponse::Complete(result)) => outcome_of(&Ok(result.clone())),
        // Zwischenergebnisse (Rückfrage, Task) gelten als gelungen.
        Ok(_) => Outcome {
            ok: true,
            ..Outcome::default()
        },
        Err(e) => outcome_of(&Err(e.clone())),
    }
}

/// Schließt einen Aufruf ab: Zähler und Aufruf-Log, eigenes Log, Nutzungsstatistik mit den ungekürzten Argumenten
/// (das Aufruf-Log kürzt sie, gekürztes JSON ließe sich nicht mehr normieren). `header`, `checkout_arg` und `checkout`
/// zeigen je Aufruf, ob der Kopf ankam und welcher Checkout galt (B-275, B-388).
fn finish<A: Clone + Send + Sync + 'static>(
    core: &Core<A>,
    id: i64,
    tool: &str,
    response: &Response,
    target: &Target,
    workspace: &Workspace,
    args: String,
) {
    let outcome = outcome(response);
    let call = core.stats.end(id, outcome.clone());
    let duration_ms = call.as_ref().map_or(0.0, |c| c.duration_ms);
    let attrs = [
        ("ns", json!("mcp")),
        ("tool", json!(tool)),
        ("ms", json!(duration_ms)),
        ("ok", json!(outcome.ok)),
        ("header", json!(target.header)),
        ("checkout_arg", json!(target.checkout_arg)),
        ("checkout", json!(workspace.label())),
    ];
    if outcome.ok {
        core.opts.log.info(&format!("✅ {tool}: ok"), &attrs);
    } else {
        // Tool und Fehler in der Meldung, damit logs_errors gleichartige Fehler je Tool gruppiert.
        let message = format!("❌ {tool}: {}", clip(&outcome.err));
        core.opts.log.warn(&message, &attrs);
    }
    // Nur Tools aus dem Katalog: erfundene Namen eines Clients ließen Statistik und Datei sonst ohne Grenze wachsen.
    if let (Some(call), Some(usage)) = (call, &core.opts.usage)
        && core.params.contains_key(tool)
    {
        usage.record(Event {
            at: chrono::Local::now(),
            tool: tool.to_owned(),
            args,
            duration_ms: call.duration_ms,
            ok: call.ok,
            error: outcome.err,
        });
    }
}
