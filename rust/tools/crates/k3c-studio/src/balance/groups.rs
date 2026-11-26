//! Werte-Gruppen (B-640, BS7.1): Jedes Ziel und jeder Hebel gehört zu genau einer Gruppe. Nach einem Meilenstein sind
//! die abgenommenen Gruppen eingefroren; eine Delta-Runde justiert nur die freien.

use super::hints::Hints;
use super::targets::Targets;
use crate::kitx::go_quoted;

/// Mauern, Türme, Truppen, Hub, Kampf, Nächte.
pub const GROUP_BASIS: &str = "basis";
/// Bäume, Holz fürs Markieren, Baumschule.
pub const GROUP_HOLZ: &str = "holz";
/// Schiffskosten, Endspiel-Korridor.
pub const GROUP_SCHIFF: &str = "schiff";

/// Die bekannten Werte-Gruppen.
pub const GROUPS: [&str; 3] = [GROUP_BASIS, GROUP_HOLZ, GROUP_SCHIFF];

pub fn is_group(name: &str) -> bool {
    GROUPS.contains(&name)
}

/// Nur bekannte Gruppen, keine doppelten.
pub(crate) fn check_frozen(frozen: &[String]) -> Result<(), String> {
    for (i, g) in frozen.iter().enumerate() {
        if !is_group(g) {
            return Err(format!(
                "frozen: gruppe {} unbekannt (bekannt: {})",
                go_quoted(g),
                GROUPS.join(", ")
            ));
        }
        if frozen.iter().take(i).any(|seen| seen == g) {
            return Err(format!("frozen: gruppe {} doppelt", go_quoted(g)));
        }
    }
    Ok(())
}

impl Targets {
    /// Ist die Gruppe eingefroren?
    pub fn is_frozen(&self, group: &str) -> bool {
        self.frozen.iter().flatten().any(|g| g == group)
    }
}

impl Hints {
    /// Verlangt in der Datei der Richtwerte eine bekannte Gruppe an jedem Hebel.
    pub(crate) fn check_groups(&self) -> Result<(), String> {
        for (key, l) in &self.levers {
            for lv in l.low.iter().chain(&l.high) {
                if !is_group(&lv.group) {
                    return Err(format!(
                        "hebel {key}, pfad {}: group {}, erlaubt: {}",
                        lv.path,
                        go_quoted(&lv.group),
                        GROUPS.join(", ")
                    ));
                }
            }
        }
        Ok(())
    }
}
