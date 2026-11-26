//! Geschichte einer Welt schreiben und Lücken nennen (Port von `story.go`, docs/rules/welten.md § 2.2). `story` steht in
//! der Welt und je Insel. Format und Regeln prüft die Spiel-EXE vor dem Schreiben; hier steht keine eigene Prüfregel.
//! Geschrieben wird format-stabil: Nur `story` bekommt neue Bytes, alle übrigen Felder bleiben byte-gleich.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use super::edit::{eol, set_field};
use super::inline::inline;
use super::nature::island_span;
use super::overrides::write_lock;
use super::patch::{Span, remove_key, same_json, value_span};
use super::{Detail, Editor, Island, check_id, null_default};
use crate::kitx::go_quote;

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

impl Story {
    /// Kein Eintrag hat Inhalt.
    fn is_blank(&self) -> bool {
        empty_list(&self.lexicon)
            && empty_list(&self.start)
            && empty_map(&self.etappen)
            && empty_list(&self.end)
            && empty_list(&self.bottles)
    }
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

impl IslandStory {
    fn is_blank(&self) -> bool {
        empty_list(&self.arrival) && empty_list(&self.boss) && empty_list(&self.done)
    }
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

/// Was `set_story` gesetzt und entfernt hat, die fehlenden Einträge und die Warnungen der Prüfung.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StoryResult {
    pub set: Vec<String>,
    pub removed: Vec<String>,
    pub missing: Vec<String>,
    pub warnings: Vec<String>,
}

impl StoryResult {
    /// Trägt den Eintrag `label` als gesetzt (`n > 0`) oder entfernt ein.
    fn note(&mut self, label: String, n: usize) {
        if n > 0 {
            self.set.push(label);
        } else {
            self.removed.push(label);
        }
    }
}

/// Schlüssel einer Map nach Zahlenwert, sonst nach Text (Gos `numericKeys`). Anders als Go stehen Zahlen immer vor
/// Texten, damit die Ordnung eindeutig bleibt.
pub(super) fn numeric_keys<V>(map: &BTreeMap<String, V>) -> Vec<&str> {
    let mut keys: Vec<&str> = map.keys().map(String::as_str).collect();
    keys.sort_by_key(|k| {
        let n = k.parse::<i64>();
        (n.is_err(), n.unwrap_or_default(), *k)
    });
    keys
}

/// Der Teil der Welt-Datei, den die Geschichte braucht.
#[derive(Default, Deserialize)]
#[serde(default)]
struct StoryFile {
    story: Option<Story>,
    #[serde(deserialize_with = "null_default")]
    islands: Vec<Option<StoryIsland>>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct StoryIsland {
    #[serde(deserialize_with = "null_default")]
    id: String,
    story: Option<IslandStory>,
}

fn story_file(raw: &[u8]) -> serde_json::Result<StoryFile> {
    Ok(serde_json::from_slice::<Option<StoryFile>>(raw)?.unwrap_or_default())
}

/// Setzt den Eintrag `dst` auf `lines` (`None` lässt ihn, leer entfernt ihn).
fn merge_entry(dst: &mut Option<Vec<String>>, lines: Option<&Vec<String>>, label: &str, res: &mut StoryResult) {
    let Some(lines) = lines else { return };
    *dst = Some(lines.clone());
    res.note(label.to_owned(), lines.len());
}

/// Legt die genannten Felder von `p` über `s` und trägt sie in `res` ein.
fn merge_world_story(s: &mut Story, p: &Story, res: &mut StoryResult) {
    if let Some(words) = &p.lexicon {
        s.lexicon = Some(words.clone());
        res.note(format!("lexicon ({} Wörter)", words.len()), words.len());
    }
    merge_entry(&mut s.start, p.start.as_ref(), "start", res);
    if let Some(etappen) = &p.etappen {
        for key in numeric_keys(etappen) {
            let lines = &etappen[key];
            let target = s.etappen.get_or_insert_with(BTreeMap::new);
            if lines.is_empty() {
                target.remove(key);
            } else {
                target.insert(key.to_owned(), lines.clone());
            }
            res.note(format!("etappen.{key}"), lines.len());
        }
    }
    merge_entry(&mut s.end, p.end.as_ref(), "end", res);
    if let Some(bottles) = &p.bottles {
        s.bottles = Some(bottles.clone());
        res.note(format!("bottles ({} Einträge)", bottles.len()), bottles.len());
    }
}

fn merge_island_story(s: &mut IslandStory, p: &IslandStory, id: &str, res: &mut StoryResult) {
    merge_entry(&mut s.arrival, p.arrival.as_ref(), &format!("{id}.arrival"), res);
    merge_entry(&mut s.boss, p.boss.as_ref(), &format!("{id}.boss"), res);
    merge_entry(&mut s.done, p.done.as_ref(), &format!("{id}.done"), res);
}

/// JSON-Text der Liste, wenn sie Einträge hat.
fn list_json<T: Serialize>(list: &Option<Vec<T>>) -> Option<String> {
    list.as_ref().filter(|l| !l.is_empty()).map(|l| json!(l).to_string())
}

/// `etappen` als Objekt, die Schlüssel nach Nummer sortiert.
fn etappen_json(etappen: &Option<BTreeMap<String, Vec<String>>>) -> Option<String> {
    let map = etappen.as_ref().filter(|m| !m.is_empty())?;
    let parts: Vec<String> = numeric_keys(map)
        .into_iter()
        .map(|k| format!("{}: {}", go_quote(k), json!(map[k])))
        .collect();
    Some(format!("{{{}}}", parts.join(",")))
}

/// `story` der Welt im Stil der Datei: je Feld eine Zeile.
fn world_story_text(s: &Story) -> Result<String, String> {
    let fields = [
        ("lexicon", list_json(&s.lexicon)),
        ("start", list_json(&s.start)),
        ("etappen", etappen_json(&s.etappen)),
        ("end", list_json(&s.end)),
        ("bottles", list_json(&s.bottles)),
    ];
    let mut lines = Vec::new();
    for (key, value) in fields {
        if let Some(value) = value {
            lines.push(format!("    {}: {}", go_quote(key), inline(&value)?));
        }
    }
    Ok(format!("{{\n{}\n  }}", lines.join(",\n")))
}

/// Setzt `story` der Welt auf `s` (leer entfernt das Feld): ein Feld je Zeile, ein neues `story` steht vor `islands`.
/// Inhaltlich gleich bleibt die Datei byte-gleich.
fn set_world_story(raw: &[u8], s: &Story) -> Result<Vec<u8>, String> {
    let whole = Span::whole(raw);
    let at = value_span(raw, whole, "story")?;
    if s.is_blank() {
        return Ok(at.map_or_else(|| raw.to_vec(), |at| remove_key(raw, at, "story")));
    }
    let text = world_story_text(s)?;
    if let Some(at) = at {
        if same_json(&raw[at.from..at.to], text.as_bytes()) {
            return Ok(raw.to_vec());
        }
        return Ok([&raw[..at.from], &eol(raw, &text), &raw[at.to..]].concat());
    }
    let missing = || "islands fehlt in der Welt-Datei".to_owned();
    let islands = value_span(raw, whole, "islands").ok().flatten().ok_or_else(missing)?;
    let name = b"\"islands\"";
    let k = raw[..islands.from]
        .windows(name.len())
        .rposition(|w| w == name)
        .ok_or_else(missing)?;
    let added = eol(raw, &format!("\"story\": {text},\n  "));
    Ok([&raw[..k], &added, &raw[k..]].concat())
}

/// Setzt `story` der Insel `id` auf `s` (leer entfernt das Feld), in der Zeile der Insel am Ende.
fn set_island_story(raw: &[u8], id: &str, s: &IslandStory) -> Result<Vec<u8>, String> {
    let (obj, _) = island_span(raw, id)?;
    let value = if s.is_blank() {
        String::new()
    } else {
        serde_json::to_string(s).map_err(|e| e.to_string())?
    };
    set_field(raw, obj, "story", &value, false)
}

/// Legt `patch` über die Geschichte in `raw`: erst `story` der Welt, dann je genannter Insel (nach ID sortiert).
pub(super) fn apply_story(raw: &[u8], patch: &StoryPatch) -> Result<(Vec<u8>, StoryResult), String> {
    let current = story_file(raw).map_err(|e| format!("Welt-Datei: {e}"))?;
    let mut res = StoryResult::default();
    let mut story = current.story.unwrap_or_default();
    merge_world_story(&mut story, &patch.story, &mut res);
    let mut out = raw.to_vec();
    // Nur Inseln genannt: `story` der Welt bleibt byte-gleich.
    if !res.set.is_empty() || !res.removed.is_empty() {
        out = set_world_story(raw, &story)?;
    }
    let mut known = BTreeMap::new();
    let mut ids = Vec::new();
    for island in current.islands {
        let island = island.unwrap_or_default();
        ids.push(island.id.clone());
        known.insert(island.id, island.story.unwrap_or_default());
    }
    for (id, change) in &patch.islands {
        let Some(mut island) = known.get(id).cloned() else {
            return Err(format!("die Insel {id:?} gibt es nicht (bekannt: {})", ids.join(", ")));
        };
        merge_island_story(&mut island, change, id, &mut res);
        out = set_island_story(&out, id, &island)?;
    }
    Ok((out, res))
}

/// Fehlende Einträge der Geschichte in `raw` (siehe [`Detail::story_gaps`]).
fn story_gaps(raw: &[u8]) -> Vec<String> {
    let Ok(file) = story_file(raw) else {
        return Vec::new();
    };
    let islands = file.islands.into_iter().map(|island| {
        let island = island.unwrap_or_default();
        Island {
            id: island.id,
            story: island.story,
            ..Island::default()
        }
    });
    let detail = Detail {
        story: file.story,
        islands: islands.collect(),
        ..Detail::default()
    };
    detail.story_gaps()
}

impl Detail {
    /// Fehlende Einträge der Geschichte (B-975/R3): Welt ohne `story`, ohne `start`, `end` oder `bottles`, Insel ohne
    /// `story`, ohne `arrival`, `boss` oder `done`. Es sind Hinweise, keine Fehler.
    pub fn story_gaps(&self) -> Vec<String> {
        let mut gaps = Vec::new();
        let mut add = |prefix: &str, missing: bool, field: &str| {
            if missing {
                gaps.push(format!("{prefix}{field}"));
            }
        };
        match self.story.as_ref().filter(|s| !s.is_blank()) {
            None => add("", true, "story"),
            Some(s) => {
                add("story.", empty_list(&s.start), "start");
                add("story.", empty_list(&s.end), "end");
                add("story.", empty_list(&s.bottles), "bottles");
            }
        }
        for island in &self.islands {
            let prefix = format!("Insel {}: story", island.id);
            let Some(s) = island.story.as_ref().filter(|s| !s.is_blank()) else {
                add(&prefix, true, "");
                continue;
            };
            let prefix = format!("{prefix}.");
            add(&prefix, empty_list(&s.arrival), "arrival");
            add(&prefix, empty_list(&s.boss), "boss");
            add(&prefix, empty_list(&s.done), "done");
        }
        gaps
    }
}

impl Editor {
    /// Ändert die Geschichte der Welt `id` nach `patch` und schreibt nur, wenn die Prüfung der Engine besteht; sonst
    /// bleibt die Datei, und der Fehler nennt Feld und Zeile. Eine unbekannte Welt oder Insel wird mit den bekannten
    /// IDs abgelehnt.
    pub fn set_story(&self, id: &str, patch: &StoryPatch) -> Result<StoryResult, String> {
        if patch.is_empty() {
            return Err("kein Eintrag genannt (lexicon, start, etappen, end, bottles, islands)".to_owned());
        }
        let _guard = write_lock();
        let raw = self.read(id).map_err(|e| {
            if check_id(id).is_ok() && matches!(self.file(id).try_exists(), Ok(false)) {
                return format!("die Welt {id:?} gibt es nicht (bekannt: {})", self.ids().join(", "));
            }
            e
        })?;
        let (out, mut res) = apply_story(&raw, patch)?;
        res.warnings = self.commit_change(id, &raw, &out)?.unwrap_or_default();
        res.missing = story_gaps(&out);
        Ok(res)
    }

    /// Die IDs aller Welten des Checkouts.
    fn ids(&self) -> Vec<String> {
        self.list().into_iter().filter(|w| !w.missing).map(|w| w.id).collect()
    }
}

#[cfg(test)]
#[path = "story_tests.rs"]
mod tests;
