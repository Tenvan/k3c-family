//! Endboss je Insel und Skins der Welt (Port von `endboss_skins.go`). RU70.3 zieht nur die Liste der Endbosse vor, die
//! `load` braucht; das Setzen folgt in RU70.5.

use serde::{Deserialize, Serialize};

use super::{Editor, null_default};

/// Ein wählbarer Endboss (ID und Anzeigename).
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct EndBossOption {
    #[serde(deserialize_with = "null_default")]
    pub id: String,
    #[serde(deserialize_with = "null_default")]
    pub name: String,
}

impl Editor {
    /// `data/bosses.json` › endBosses; fehlt die Datei, ist die Liste leer.
    pub(super) fn end_boss_options(&self) -> Vec<EndBossOption> {
        #[derive(Deserialize)]
        struct Bosses {
            #[serde(rename = "endBosses")]
            end_bosses: Option<Vec<EndBossOption>>,
        }
        std::fs::read(self.root.join("data").join("bosses.json"))
            .ok()
            .and_then(|raw| serde_json::from_slice::<Bosses>(&raw).ok())
            .and_then(|doc| doc.end_bosses)
            .unwrap_or_default()
    }
}
