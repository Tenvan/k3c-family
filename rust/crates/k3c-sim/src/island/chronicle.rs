//! Chronik der Insel (GS3.7, B-966, `ISLAND_SAVE_VERSION` 13): Beim Ablegen von einer Insel mit besiegtem Endboss hält die
//! Insel einen Eintrag fest: Insel-ID, Tag der Insel beim Ablegen und Zahl der Monarchen, die den Wechsel mitmachen. Die
//! Einträge ziehen wie `Island::completed` zur nächsten Insel mit (Reihenfolge des Abschlusses) und stehen im Spielstand
//! (`chronicle`, optional). Sie entstehen nur aus dem Zustand der Insel: kein Zufall, keine Uhrzeit.

use serde::{Deserialize, Serialize};

use super::Island;
use super::save::IslandSave;
use crate::state::links::crowned;

/// Ein Eintrag der Chronik: eine abgeschlossene Insel (`welten.md` § 5/§ 6.1).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ChronicleEntry {
    /// Insel-ID in der Welt (steht auch in `Island::completed`).
    #[serde(rename = "islandId")]
    pub island: String,
    /// Tag der Insel beim Ablegen (Tageszähler der ersten Stufe).
    #[serde(rename = "days")]
    pub days: i64,
    /// Zahl der Monarchen, die den Wechsel mitgemacht haben (lebend, gekrönt, nicht frei).
    #[serde(rename = "monarchs")]
    pub monarchs: i64,
}

impl Island {
    /// Der Chronik-Eintrag dieser Insel, wie er beim Ablegen entsteht: Tag der ersten Stufe, Monarchen wie bei der
    /// Bedingung des Wechsels (`all_at_gate`): lebend, gekrönt und nicht frei.
    pub fn chronicle_entry(&self) -> ChronicleEntry {
        ChronicleEntry {
            island: self.island_id.clone(),
            days: self.stages.first().map_or(0, |w| w.cycle.day),
            monarchs: i64::try_from(self.players().iter().filter(|p| crowned(p) && !p.free).count())
                .unwrap_or(i64::MAX),
        }
    }
}

/// Die Chronik gehört zu den abgeschlossenen Inseln: jeder Eintrag nennt eine abgeschlossene Insel, jede höchstens einmal
/// (GS3.7). Die aktuelle Insel steht nie darin (das prüft schon die Liste `completed`).
pub(super) fn check_chronicle(s: &IslandSave) -> Result<(), String> {
    for (i, e) in s.chronicle.iter().enumerate() {
        if !s.completed.contains(&e.island) {
            return Err(format!(
                "spielstand: Chronik-Eintrag für Insel {:?}, die nicht abgeschlossen ist",
                e.island
            ));
        }
        if s.chronicle[..i].iter().any(|o| o.island == e.island) {
            return Err(format!("spielstand: Chronik-Eintrag für Insel {:?} doppelt", e.island));
        }
        if e.days < 0 || e.monarchs < 0 {
            return Err(format!(
                "spielstand: Chronik-Eintrag für Insel {:?} mit negativen Zahlen",
                e.island
            ));
        }
    }
    Ok(())
}
