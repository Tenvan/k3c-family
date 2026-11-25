//! Werte-Gruppen (Port von `engine/balance/groups.go`, B-640, BS7.1): Jedes Ziel und jeder Hebel gehört zu genau einer
//! Gruppe. Nach einem Meilenstein sind die abgenommenen Gruppen eingefroren (`Targets.frozen`); eine Delta-Runde justiert
//! nur die freien. Die Prüfung von `frozen` und der Gruppe je Ziel liegt beim Laden in `k3c_data::balance_targets`.

use k3c_data::balance_targets::Targets;

pub use k3c_data::balance_hints::GROUPS;

use crate::hints::Hints;

/// Mauern, Türme, Truppen, Hub, Kampf, Nächte.
pub const GROUP_BASIS: &str = "basis";
/// Bäume, Holz fürs Markieren, Baumschule.
pub const GROUP_HOLZ: &str = "holz";
/// Schiffskosten, Endspiel-Korridor.
pub const GROUP_SCHIFF: &str = "schiff";

/// `name` ist eine bekannte Gruppe (Go: `IsGroup`).
pub fn is_group(name: &str) -> bool {
    GROUPS.contains(&name)
}

/// Die Gruppe ist eingefroren (Go: `Targets.IsFrozen`).
pub fn is_frozen(t: &Targets, group: &str) -> bool {
    t.frozen.iter().any(|g| g == group)
}

impl Hints {
    /// Verlangt in der Datei der Richtwerte eine bekannte Gruppe an jedem Hebel (Go: `checkGroups`).
    pub fn check_groups(&self) -> Result<(), String> {
        for (key, l) in &self.levers {
            for lv in l.low.iter().chain(&l.high) {
                if !is_group(&lv.group) {
                    return Err(format!(
                        "hebel {key}, pfad {}: group {:?}, erlaubt: {}",
                        lv.path,
                        lv.group,
                        GROUPS.join(", ")
                    ));
                }
            }
        }
        Ok(())
    }
}
