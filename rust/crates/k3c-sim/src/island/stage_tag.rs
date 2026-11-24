//! Stufen-Kennung der Ereignisse einer Insel.

use crate::state::Event;

/// Setzt `stage` im Ereignis.
pub(super) fn tag_stage(ev: &mut Event, stage: usize) {
    ev.insert("stage".to_string(), serde_json::Value::from(stage as i64));
}
