//! Aufrufe der Seite Logs.

use std::sync::Arc;

use k3c_kit::uikit;
use serde_json::Value;

use super::{RpcError, encode};
use crate::app::App;
use crate::app::logview::LogQuery;

pub(super) fn query(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (source, filter) = uikit::args::<(String, LogQuery)>(args.to_vec())?;
    encode(app.logs_query(&source, &filter)?)
}

pub(super) fn errors(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (source, level) = uikit::args::<(String, String)>(args.to_vec())?;
    encode(app.logs_errors(&source, &level)?)
}
