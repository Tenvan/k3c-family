//! Inseln einer Welt bearbeiten (Port von `edit_island.go`). RU70.3 zieht nur die Zeile einer neuen Insel vor; der Rest
//! folgt in RU70.4.

use super::Editor;
use crate::kitx::go_quote;

/// Zeile einer neuen Insel; `extra` sind fertige `"schlüssel": wert`-Paare.
pub(super) fn new_island(id: &str, name: &str, depths: &str, extra: &[String]) -> String {
    let mut parts = vec![
        format!("\"id\": {}", go_quote(id)),
        format!("\"name\": {}", go_quote(name)),
        format!("\"depths\": {depths}"),
        "\"depthScaling\": { \"hp\": 1.5, \"damage\": 1.3, \"speed\": 1.1 }".to_owned(),
    ];
    parts.extend_from_slice(extra);
    format!("{{ {} }}", parts.join(", "))
}

impl Editor {
    /// Wendet die Insel-Einträge der Liste `list` an (`world_set` › islands).
    #[allow(clippy::unimplemented)]
    pub(super) fn patch_islands(&self, _raw: &[u8], _id: &str, _list: &str) -> Result<Vec<u8>, String> {
        unimplemented!("RU70.4")
    }
}
