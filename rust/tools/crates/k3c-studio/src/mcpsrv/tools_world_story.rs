//! Geschichte einer Welt als MCP-Tool (Port von `tools_world_story.go`, B-975, docs/rules/welten.md § 2.2):
//! `world_story_set` schreibt `story` der Welt und je Insel; `world_get` zeigt sie, `world_check` nennt fehlende
//! Einträge als Hinweis, `world_create` nimmt sie mit auf. Die Regeln prüft die Engine, nicht das Studio.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

use super::tools_world::{done, number_keys};
use crate::world::{Detail, Editor, IslandStory, Story, StoryPatch, StoryResult};

#[cfg(test)]
#[path = "tools_world_story_tests.rs"]
mod tests;

/// Schluss der Antwort von `world_create` ohne Geschichte (B-975/R4).
pub(super) const STORY_MISSING: &str = "Geschichte fehlt: world_story_set";

/// Geschichte einer Insel als Argument.
#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
#[schemars(inline)]
pub(super) struct IslandStoryIn {
    #[schemars(description = "Zeilen bei der Ankunft auf der Insel (1 bis 2); [] entfernt, fehlt = unverändert")]
    arrival: Option<Vec<String>>,
    #[schemars(description = "Zeilen, wenn der Endboss ausgelöst wird (1 bis 2); [] entfernt, fehlt = unverändert")]
    boss: Option<Vec<String>>,
    #[schemars(
        description = "Abschlusszeile nach dem Sieg über den Endboss (1 bis 2 Zeilen); [] entfernt, fehlt = unverändert"
    )]
    done: Option<Vec<String>>,
}

/// Die Geschichte als Argument: jedes Feld fehlt = unverändert, `[]` = entfernen. `null` in `etappen` und `bottles` ist
/// wie bei Gos Slices die leere Liste.
#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
#[schemars(inline)]
pub(super) struct StoryArgs {
    #[schemars(
        description = "Wortschatz der Kunstsprache: Kunstwörter klein, Apostroph und Bindestrich im Wort erlaubt; ersetzt die ganze Liste, [] entfernt"
    )]
    lexicon: Option<Vec<String>>,
    #[schemars(
        description = "Zeilen beim Start einer neuen Kampagne (1 bis 2; je Zeile 2 bis 3 echte Wörter in GROSSBUCHSTABEN, jedes andere Wort aus lexicon); [] entfernt"
    )]
    start: Option<Vec<String>>,
    #[schemars(
        description = "Etappen-Nummer (z. B. 1) → Zeilen beim Abschluss der Etappe; nur genannte Etappen ändern sich, [] entfernt die Etappe"
    )]
    etappen: BTreeMap<String, Option<Vec<String>>>,
    #[schemars(description = "Zeilen beim Kampagnensieg (1 bis 2); [] entfernt")]
    end: Option<Vec<String>>,
    #[schemars(
        description = "Einträge der Flaschenpost, je Eintrag 1 bis 2 Zeilen; ersetzt die ganze Liste, [] entfernt"
    )]
    bottles: Option<Vec<Option<Vec<String>>>>,
    #[schemars(description = "Insel-ID → {arrival, boss, done}; nur genannte Inseln und Einträge ändern sich")]
    islands: BTreeMap<String, IslandStoryIn>,
}

impl StoryArgs {
    /// Die Änderung des Editors (`None` bleibt: unverändert).
    pub(super) fn patch(self) -> StoryPatch {
        let etappen: BTreeMap<String, Vec<String>> = self
            .etappen
            .into_iter()
            .map(|(number, lines)| (number, lines.unwrap_or_default()))
            .collect();
        let islands = self.islands.into_iter().map(|(id, i)| {
            let story = IslandStory {
                arrival: i.arrival,
                boss: i.boss,
                done: i.done,
            };
            (id, story)
        });
        let bottles = self.bottles;
        StoryPatch {
            story: Story {
                lexicon: self.lexicon,
                start: self.start,
                etappen: (!etappen.is_empty()).then_some(etappen),
                end: self.end,
                bottles: bottles.map(|list| list.into_iter().map(Option::unwrap_or_default).collect()),
            },
            islands: islands.collect(),
        }
    }
}

/// Argumente von `world_story_set`: `id` neben den Feldern der Geschichte. `flatten` verträgt sich in serde nicht mit
/// `deny_unknown_fields`, darum liest [`Deserialize`] von Hand.
#[derive(Debug, JsonSchema)]
pub(super) struct StoryIn {
    #[schemars(description = "Welt-ID")]
    id: String,
    #[schemars(flatten)]
    story: StoryArgs,
}

impl<'de> Deserialize<'de> for StoryIn {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut map = Map::<String, Value>::deserialize(deserializer)?;
        let id = map.remove("id").ok_or_else(|| D::Error::missing_field("id"))?;
        Ok(StoryIn {
            id: String::deserialize(id).map_err(D::Error::custom)?,
            story: StoryArgs::deserialize(Value::Object(map)).map_err(D::Error::custom)?,
        })
    }
}

/// `world_story_set`.
pub(super) fn story_set(editor: &Editor, input: StoryIn) -> Result<String, String> {
    let result = editor.set_story(&input.id, &input.story.patch())?;
    let ok = story_done(&input.id, &result);
    Ok(done(ok, Some(result.warnings)))
}

/// Erfolgssatz von `world_story_set`: gesetzte, entfernte und noch fehlende Einträge.
fn story_done(id: &str, result: &StoryResult) -> String {
    let mut out = format!("Geschichte der Welt {id} gespeichert");
    let parts = [
        ("Gesetzt", &result.set),
        ("Entfernt", &result.removed),
        ("Fehlt noch", &result.missing),
    ];
    for (label, entries) in parts {
        if !entries.is_empty() {
            out += &format!("\n{label}: {}", entries.join(", "));
        }
    }
    out
}

/// Hinweise von `world_check`: je fehlendem Eintrag eine Zeile (keine Fehler, B-975/R3).
pub(super) fn story_hints(detail: &Detail) -> String {
    detail
        .story_gaps()
        .into_iter()
        .map(|gap| format!("\nHinweis: Geschichte fehlt ({gap})"))
        .collect()
}

/// Eine Zeile des Abschnitts „Geschichte“; ohne Text steht dort „fehlt“.
fn entry(label: &str, lines: Option<&Vec<String>>) -> String {
    let text = lines.map_or_else(String::new, |l| l.join(" | "));
    format!("  {label}: {}\n", or_missing(&text))
}

fn or_missing(text: &str) -> &str {
    if text.is_empty() { "fehlt" } else { text }
}

/// Wendepunkte der Welt: Wortschatz, Start, Etappen, Ende, Flaschenpost.
fn world_lines(story: &Story) -> String {
    let lexicon = story.lexicon.as_deref().unwrap_or_default();
    let mut out = format!("  lexicon ({}): {}\n", lexicon.len(), or_missing(&lexicon.join(", ")));
    out += &entry("start", story.start.as_ref());
    match story.etappen.as_ref().filter(|e| !e.is_empty()) {
        Some(etappen) => {
            for number in number_keys(etappen) {
                out += &entry(&format!("etappen.{number}"), etappen.get(&number.to_string()));
            }
        }
        None => out += &entry("etappen", None),
    }
    out += &entry("end", story.end.as_ref());
    let bottles = story.bottles.as_deref().unwrap_or_default();
    for (i, lines) in bottles.iter().enumerate() {
        out += &entry(&format!("bottles[{}]", i + 1), Some(lines));
    }
    if bottles.is_empty() {
        out += &entry("bottles", None);
    }
    out
}

/// Abschnitt „Geschichte“ von `world_get`: Wortschatz und je Wendepunkt die Zeilen, fehlende als „fehlt“.
pub(super) fn story_text(detail: &Detail) -> String {
    let mut out = "Geschichte:\n".to_owned();
    out += &world_lines(&detail.story.clone().unwrap_or_default());
    for island in &detail.islands {
        let story = island.story.clone().unwrap_or_default();
        let id = &island.id;
        out += &entry(&format!("{id}.arrival"), story.arrival.as_ref());
        out += &entry(&format!("{id}.boss"), story.boss.as_ref());
        out += &entry(&format!("{id}.done"), story.done.as_ref());
    }
    out
}
