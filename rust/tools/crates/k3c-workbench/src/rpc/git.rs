//! Aufrufe der Seite Git.

use std::sync::Arc;

use k3c_kit::uikit;
use serde_json::Value;

use super::{RpcError, encode, name, no_args};
use crate::app::App;

pub(super) fn overview(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.git_overview()?)
}

pub(super) fn action(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    encode(app.git_action(&name(args)?)?)
}

pub(super) fn delete_branches(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (names, remote) = uikit::args::<(Option<Vec<String>>, bool)>(args.to_vec())?;
    encode(app.git_delete_branches(&names.unwrap_or_default(), remote)?)
}

pub(super) fn check_merged(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (names,) = uikit::args::<(Option<Vec<String>>,)>(args.to_vec())?;
    encode(app.git_check_merged(&names.unwrap_or_default())?)
}

pub(super) fn stamp(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.git_stamp()?)
}
