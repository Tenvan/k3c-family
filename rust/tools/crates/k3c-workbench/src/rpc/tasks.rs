//! Aufrufe der Seite Tasks und der Konsole.

use std::sync::Arc;

use k3c_kit::uikit;
use serde_json::Value;

use super::{RpcError, encode, name, no_args};
use crate::app::App;

pub(super) fn catalog(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.tasks())
}

pub(super) fn reload(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.tasks_reload())
}

pub(super) fn start(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (task, extra) = uikit::args::<(String, Option<Vec<String>>)>(args.to_vec())?;
    encode(app.task_start(&task, extra.unwrap_or_default())?)
}

pub(super) fn stop(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    encode(app.task_stop(&name(args)?)?)
}

pub(super) fn runs(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.task_runs())
}

pub(super) fn sources(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.sources())
}

pub(super) fn console_tail(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    encode(app.console_tail(&name(args)?)?)
}
