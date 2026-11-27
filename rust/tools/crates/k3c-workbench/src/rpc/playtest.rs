//! Aufrufe der Seite Spieltest (Aufnahme und Auswertung).

use std::sync::Arc;

use k3c_kit::uikit;
use serde_json::Value;

use super::{RpcError, encode, no_args};
use crate::app::App;
use crate::playtest::{FindingPatch, Options, TranscribeOptions};

pub(super) fn list(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.playtest_list()?)
}

pub(super) fn plan(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (action, id) = uikit::args::<(String, String)>(args.to_vec())?;
    encode(app.playtest_plan(&action, &id)?)
}

pub(super) fn delete(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (action, id, confirm) = uikit::args::<(String, String, bool)>(args.to_vec())?;
    encode(app.playtest_delete(&action, &id, confirm)?)
}

pub(super) fn devices(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.playtest_devices()?)
}

pub(super) fn start(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (options,) = uikit::args::<(Options,)>(args.to_vec())?;
    encode(app.playtest_start(&options)?)
}

pub(super) fn stop(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    app.playtest_stop()?;
    Ok(Value::Null)
}

pub(super) fn status(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.playtest_status())
}

pub(super) fn check(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.playtest_check()?)
}

pub(super) fn transcribe(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (id, options) = uikit::args::<(String, TranscribeOptions)>(args.to_vec())?;
    encode(app.playtest_transcribe(&id, &options)?)
}

pub(super) fn models(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.playtest_models())
}

pub(super) fn transcribe_status(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    encode(app.playtest_transcribe_status())
}

pub(super) fn transcribe_cancel(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    no_args(args)?;
    app.playtest_transcribe_cancel()?;
    Ok(Value::Null)
}

pub(super) fn timeline(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (id,) = uikit::args::<(String,)>(args.to_vec())?;
    encode(app.playtest_timeline(&id)?)
}

pub(super) fn findings(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (id,) = uikit::args::<(String,)>(args.to_vec())?;
    encode(app.playtest_findings(&id)?)
}

pub(super) fn finding_update(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (id, finding, patch) = uikit::args::<(String, String, FindingPatch)>(args.to_vec())?;
    encode(app.playtest_finding_update(&id, &finding, &patch)?)
}

pub(super) fn frame_skip(app: &Arc<App>, args: &[Value]) -> Result<Value, RpcError> {
    let (id, nr, skip) = uikit::args::<(String, i64, bool)>(args.to_vec())?;
    app.playtest_frame_skip(&id, nr, skip)?;
    Ok(Value::Null)
}
