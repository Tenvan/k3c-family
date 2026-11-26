//! Welten anlegen und bearbeiten (Port von `edit.go`). RU70.3 zieht nur `Create` und `Set` samt Schreibweg vor, weil
//! die Katalog-Tests sie brauchen; `Check`, `Raw`, die Seiten-Antworten und die Insel-Bearbeitung folgen in RU70.4.
//! Geschrieben wird nur die Welt-Datei, atomar und format-stabil; vorher prüft die Spiel-EXE den Entwurf wie das Laden
//! im Spiel.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use k3c_kit::logger::Logger;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use super::catalog::check_patch_keys;
use super::edit_island::new_island;
use super::inline::inline;
use super::overrides::{given, temp_file, write_atomic, write_lock};
use super::patch::{Span, remove_key, same_json, set_key, value_span};
use super::story::{StoryPatch, apply_story};
use super::{Editor, WORLDS_DIR, check_id};
use crate::kitx::go_quote;

/// Kopf-Felder, die `world_set` ändert (Reihenfolge wie in der Datei); `islands` kommt dazu.
pub(super) const HEADER_KEYS: [&str; 7] = ["name", "gewaesser", "notes", "extends", "kind", "combat", "skins"];

/// `story` ändert nur `world_story_set`.
pub(super) const ERR_STORY_TOOL: &str = "story ändert world_story_set, nicht world_set";

/// Die Datei hat sich zwischen Lesen und Schreiben geändert (Lost Update vermieden).
const ERR_CHANGED: &str = "die Welt wurde inzwischen geändert, neu lesen";

/// Antwort von „Welt anlegen“ und den anderen Änderungen der Seite: ID, Warnungen der Prüfung (`None` = `null`) oder
/// der Fehler.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateView {
    pub id: String,
    pub warnings: Option<Vec<String>>,
    pub error: String,
}

/// Warnungen der Prüfung; `None`, wenn sie keine nennt (Gos nil-Liste).
pub(super) type Warnings = Option<Vec<String>>;

#[derive(Default, Deserialize)]
struct Check {
    warnings: Warnings,
}

#[derive(Serialize)]
struct CheckRequest<'a> {
    op: &'a str,
    world: &'a str,
    draft: &'a RawValue,
}

/// `s` mit den Zeilenenden der Datei (CRLF bleibt CRLF).
fn eol(raw: &[u8], s: &str) -> Vec<u8> {
    if raw.windows(2).any(|w| w == b"\r\n") {
        return s.replace('\n', "\r\n").into_bytes();
    }
    s.as_bytes().to_vec()
}

/// Schreibt `out` atomar, aber nur, wenn die Datei noch `expect` enthält; ohne `expect` wird sie exklusiv angelegt (eine
/// zwischenzeitlich entstandene Datei bleibt, Link statt Rename).
pub(super) fn write_checked(path: &Path, expect: Option<&[u8]>, out: &[u8], log: &Logger) -> Result<(), String> {
    if let Some(expect) = expect {
        if std::fs::read(path).ok().as_deref() != Some(expect) {
            return Err(ERR_CHANGED.to_owned());
        }
        return write_atomic(path, out, log);
    }
    let (tmp, mut file) = temp_file(path).map_err(|e| e.to_string())?;
    let written = file.write_all(out);
    drop(file);
    let result = written
        .map_err(|e| format!("schreiben: {e}"))
        .and_then(|()| match std::fs::hard_link(&tmp, path) {
            Ok(()) => Ok(()),
            Err(_) if path.exists() => {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                Err(format!("{name} gibt es schon (ändern mit world_set)"))
            }
            Err(e) => Err(e.to_string()),
        });
    let _ = std::fs::remove_file(&tmp);
    result
}

/// Setzt `key` im Objekt `obj` (`null` oder leer entfernt es). Ein neues Feld der obersten Ebene (`top`) steht vor
/// `islands`, in einer Insel am Ende der Zeile; ein inhaltlich gleicher Wert lässt die Bytes, wie sie sind.
pub(super) fn set_field(raw: &[u8], obj: Span, key: &str, value: &str, top: bool) -> Result<Vec<u8>, String> {
    let at = value_span(raw, obj, key)?;
    if !given(value) {
        return Ok(at.map_or_else(|| raw.to_vec(), |at| remove_key(raw, at, key)));
    }
    let text = inline(value).map_err(|e| format!("{key}: {e}"))?;
    if at.is_some_and(|at| same_json(&raw[at.from..at.to], value.as_bytes())) {
        return Ok(raw.to_vec());
    }
    if at.is_some() || !top {
        return set_key(raw, obj, key, text.as_bytes());
    }
    let Ok(Some(islands)) = value_span(raw, obj, "islands") else {
        return set_key(raw, obj, key, text.as_bytes());
    };
    let name = b"\"islands\"";
    let Some(k) = raw[..islands.from].windows(name.len()).rposition(|w| w == name) else {
        return set_key(raw, obj, key, text.as_bytes());
    };
    let added = eol(raw, &format!("{}: {text},\n  ", go_quote(key)));
    Ok([&raw[..k], added.as_slice(), &raw[k..]].concat())
}

impl Editor {
    /// Lässt den Entwurf `out` der Welt `id` prüfen und schreibt ihn atomar (siehe [`write_checked`]).
    fn commit(&self, id: &str, expect: Option<&[u8]>, out: &[u8]) -> Result<Warnings, String> {
        let draft = serde_json::from_slice::<&RawValue>(out).map_err(|e| e.to_string())?;
        let check = self.call::<Check>(&CheckRequest {
            op: "check",
            world: id,
            draft,
        })?;
        let path = self.file(id);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        write_checked(&path, expect, out, &self.log)?;
        Ok(check.warnings)
    }

    /// Legt die Welt `id` an: eine Insel „island1“ (Tiefen 0-4), Kampf groups/5, optional von `extends` erbend. Eine
    /// vorhandene Welt wird nie überschrieben.
    pub fn create(&self, id: &str, name: &str, extends: &str) -> Result<Warnings, String> {
        self.create_with_story(id, name, extends, None)
    }

    /// Wie [`Editor::create`] mit der Geschichte gleich dabei; besteht sie die Prüfung nicht, entsteht keine Datei.
    pub fn create_with_story(
        &self,
        id: &str,
        name: &str,
        extends: &str,
        story: Option<&StoryPatch>,
    ) -> Result<Warnings, String> {
        check_id(id)?;
        let name = name.trim();
        if name.is_empty() {
            return Err("der Name fehlt".to_owned());
        }
        let _guard = write_lock();
        if self.file(id).exists() {
            return Err(format!("{WORLDS_DIR}/{id}.json gibt es schon (ändern mit world_set)"));
        }
        let mut head = format!("{{\n  \"id\": {},\n  \"name\": {},\n", go_quote(id), go_quote(name));
        if !extends.is_empty() {
            check_id(extends).map_err(|e| format!("extends: {e}"))?;
            head += &format!("  \"extends\": {},\n", go_quote(extends));
        }
        let island = new_island("island1", "Insel 1", "[0, 1, 2, 3, 4]", &[]);
        head += "  \"combat\": { \"mode\": \"groups\", \"groupSize\": 5 },\n";
        head += &format!("  \"islands\": [\n    {island}\n  ]\n}}\n");
        let mut out = head.into_bytes();
        if let Some(story) = story.filter(|s| !s.is_empty()) {
            (out, _) = apply_story(&out, story)?;
        }
        self.commit(id, None, &out)
    }

    /// Ändert Kopf und Inseln der Welt `id` nach `patch` (JSON-Objekt): Kopf-Felder (`null` entfernt das Feld),
    /// `islands` und die Katalog-Arten.
    pub fn set(&self, id: &str, patch: &str) -> Result<Warnings, String> {
        let patch = serde_json::from_str::<Option<BTreeMap<String, Box<RawValue>>>>(patch)
            .ok()
            .flatten()
            .ok_or_else(|| "patch muss ein JSON-Objekt sein".to_owned())?;
        check_patch_keys(&patch)?;
        self.apply_picks(id, &patch)?;
        let _guard = write_lock();
        let raw = self.read(id)?;
        let mut out = raw.clone();
        for key in HEADER_KEYS {
            if let Some(value) = patch.get(key) {
                out = set_field(&out, Span::whole(&out), key, value.get(), true)?;
            }
        }
        if let Some(list) = patch.get("islands") {
            out = self.patch_islands(&out, id, list.get())?;
        }
        self.commit_change(id, &raw, &out).map_err(|e| {
            if !e.contains("kein Hafen") {
                return e;
            }
            format!(
                "{e}; Hinweis: den Hafen im selben world_set setzen, z. B. islands: [{{id: <Insel davor>, plannedChunks: \
                 {{\"0\": [{{\"slot\": -7, \"kind\": \"harbor\"}}]}}}}, {{id: <neue Insel>}}] (Feld plannedChunks, Tiefe 0)"
            )
        })
    }

    /// Schreibt nur eine inhaltliche Änderung (sonst bleibt die Datei, wie sie ist).
    pub(super) fn commit_change(&self, id: &str, raw: &[u8], out: &[u8]) -> Result<Warnings, String> {
        if raw == out {
            return Ok(None);
        }
        self.commit(id, Some(raw), out)
    }
}
