//! Aufrufe der Seite Dienste.

use std::sync::Arc;

use k3c_kit::uikit;
use serde_json::Value;

use super::{RpcError, encode, name, no_args};
use crate::app::App;

pub(super) fn view(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.services())
}

pub(super) fn reload(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.services_reload()?)
}

pub(super) fn start(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    encode(app.service_start(&name(args)?)?)
}

pub(super) fn stop(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (name, force) = uikit::args::<(String, bool)>(args.to_vec())?;
    encode(app.service_stop(&name, force)?)
}

pub(super) fn restart(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    encode(app.service_restart(&name(args)?)?)
}

pub(super) fn start_all(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.services_start_all()?)
}

pub(super) fn stop_all(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.services_stop_all()?)
}

pub(super) fn log_levels(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    encode(app.service_log_levels(&name(args)?)?)
}
