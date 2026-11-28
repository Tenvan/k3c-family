//! Aufrufe der Seite Planung und des Ticket-Buttons.

use std::sync::Arc;

use k3c_assetkit::ticket::Input;
use k3c_kit::uikit;
use serde_json::Value;

use super::{RpcError, encode, name, no_args};
use crate::app::App;

pub(super) fn data(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.planning_data()?)
}

pub(super) fn list(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.planning_list()?)
}

pub(super) fn items(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (ids,) = uikit::args::<(Option<Vec<String>>,)>(args.to_vec())?;
    encode(app.planning_items(&ids.unwrap_or_default())?)
}

pub(super) fn docs(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.planning_docs())
}

pub(super) fn doc(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    encode(app.planning_doc(&name(args)?)?)
}

pub(super) fn set(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (id, field, value) = uikit::args::<(String, String, String)>(args.to_vec())?;
    encode(app.planning_set(&id, &field, &value)?)
}

pub(super) fn get(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    encode(app.planning_get(&name(args)?)?)
}

pub(super) fn worktree_check(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.worktree_check()?)
}

pub(super) fn ticket_create(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (input,) = uikit::args::<(Input,)>(args.to_vec())?;
    encode(app.ticket_create(input)?)
}

pub(super) fn ticket_recent(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (n,) = uikit::args::<(i64,)>(args.to_vec())?;
    encode(app.ticket_recent(n)?)
}
