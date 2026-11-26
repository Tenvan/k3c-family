//! Endboss je Insel und Skins der Welt (Port von `endboss_skins.go`, WE6.4): Beides geht über den Patch von
//! [`Editor::set`], damit gelten die gleichen Prüfungen wie bei `world_set`. Die Endboss-Liste steht in
//! `data/bosses.json`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{CreateView, Editor, null_default};

/// Ein wählbarer Endboss (ID und Anzeigename).
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct EndBossOption {
    #[serde(deserialize_with = "null_default")]
    pub id: String,
    #[serde(deserialize_with = "null_default")]
    pub name: String,
}

/// Antwort mit Fehler, ohne Warnungen (`null`).
fn rejected(id: &str, error: String) -> CreateView {
    CreateView {
        id: id.to_owned(),
        warnings: None,
        error,
    }
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

    /// Setzt den Endboss der Insel `island`; leer entfernt das Feld (Endboss der tiefsten Stufe).
    pub fn set_island_end_boss(&self, id: &str, island: &str, boss: &str) -> CreateView {
        // Eine unbekannte Insel würde als neue Insel angelegt: vorher prüfen, dann ablehnen.
        let detail = self.load(id);
        if !detail.error.is_empty() {
            return rejected(id, detail.error);
        }
        if !detail.islands.iter().any(|i| i.id == island) {
            return rejected(id, format!("die Insel {island:?} gibt es in {id} nicht"));
        }
        if !boss.is_empty() && !self.end_boss_options().iter().any(|b| b.id == boss) {
            return rejected(id, format!("unbekannter Endboss {boss:?}"));
        }
        let value = if boss.is_empty() { Value::Null } else { json!(boss) };
        self.set_patch(id, &json!({"islands": [{"id": island, "endBoss": value}]}))
    }

    /// Ersetzt die Skins der Welt; eine leere Auswahl entfernt das Feld.
    pub fn set_skins(&self, id: &str, skins: &BTreeMap<String, String>) -> CreateView {
        let value = if skins.is_empty() { Value::Null } else { json!(skins) };
        self.set_patch(id, &json!({"skins": value}))
    }

    /// Schickt einen Patch an [`Editor::set`] und fasst den Fehler in der Antwort zusammen.
    fn set_patch(&self, id: &str, patch: &Value) -> CreateView {
        match self.set(id, &patch.to_string()) {
            Ok(warnings) => CreateView {
                id: id.to_owned(),
                warnings,
                error: String::new(),
            },
            Err(error) => rejected(id, error),
        }
    }
}

#[cfg(test)]
#[path = "endboss_skins_tests.rs"]
mod tests;
