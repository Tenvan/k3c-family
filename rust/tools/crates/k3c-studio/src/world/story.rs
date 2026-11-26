//! Geschichte einer Welt (Port von `story.go`, docs/rules/welten.md § 2.2). RU70.3 zieht nur die Typen vor; Schreiben
//! und Lücken folgen in RU70.5.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

fn empty_list<T>(v: &Option<Vec<T>>) -> bool {
    v.as_ref().is_none_or(Vec::is_empty)
}

fn empty_map(v: &Option<BTreeMap<String, Vec<String>>>) -> bool {
    v.as_ref().is_none_or(BTreeMap::is_empty)
}

/// `story` der Welt wie in der Datei. In einem [`StoryPatch`] heißt `None` „bleibt“, eine leere Liste „entfernen“.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct Story {
    #[serde(skip_serializing_if = "empty_list")]
    pub lexicon: Option<Vec<String>>,
    #[serde(skip_serializing_if = "empty_list")]
    pub start: Option<Vec<String>>,
    #[serde(skip_serializing_if = "empty_map")]
    pub etappen: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "empty_list")]
    pub end: Option<Vec<String>>,
    #[serde(skip_serializing_if = "empty_list")]
    pub bottles: Option<Vec<Vec<String>>>,
}

/// `story` einer Insel: Ankunft, Endboss ausgelöst, Abschlusszeile.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct IslandStory {
    #[serde(skip_serializing_if = "empty_list")]
    pub arrival: Option<Vec<String>>,
    #[serde(skip_serializing_if = "empty_list")]
    pub boss: Option<Vec<String>>,
    #[serde(skip_serializing_if = "empty_list")]
    pub done: Option<Vec<String>>,
}

/// Änderung der Geschichte: `etappen` und `islands` gelten je Schlüssel, `bottles` ersetzt die ganze Liste.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StoryPatch {
    pub story: Story,
    pub islands: BTreeMap<String, IslandStory>,
}

impl StoryPatch {
    /// Der Patch nennt keinen Eintrag.
    pub fn is_empty(&self) -> bool {
        let s = &self.story;
        s.lexicon.is_none()
            && s.start.is_none()
            && empty_map(&s.etappen)
            && s.end.is_none()
            && s.bottles.is_none()
            && self.islands.is_empty()
    }
}

/// Was `SetStory` gesetzt und entfernt hat, die fehlenden Einträge und die Warnungen der Prüfung.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StoryResult {
    pub set: Vec<String>,
    pub removed: Vec<String>,
    pub missing: Vec<String>,
    pub warnings: Vec<String>,
}

/// Legt `patch` über die Geschichte in `raw`.
#[allow(clippy::unimplemented)]
pub(super) fn apply_story(_raw: &[u8], _patch: &StoryPatch) -> Result<(Vec<u8>, StoryResult), String> {
    unimplemented!("RU70.5")
}
