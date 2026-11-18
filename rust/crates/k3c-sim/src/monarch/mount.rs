//! Reittier des Monarchen (Port von `engine/sim/monarch_mount.go`; docs/rules/monarch.md § 7, B-152).

use k3c_data::monarch::MountData;

use crate::state::WorldValues;

/// Go: `MountOf`: das Reittier des Monarchen `pi` in der Stufe. Heute reitet jeder das Standard-Reittier aus den Daten seiner
/// Welt (kein Feld in `Player`, Snapshot oder Spielstand); ein abweichendes Reittier käme hier hinzu.
pub fn mount_of(v: &WorldValues, _pi: usize) -> &MountData {
    &v.monarch.mount
}
