//! Ablehnung eines Overrides erklärt, was geht (Port von `override_reject.go`, B-986). Erlaubte Dateien und Felder
//! kommen aus der Basis der Engine (`values`), gesperrte Felder aus deren Prüfung (welten.md § 1.3); keine zweite Liste.

use super::Editor;
use super::overrides::overrides_of;
use super::patch::patch_overrides;
use super::values::ValueEntry;
use serde_json::value::RawValue;

/// Zahl der Felder einer Ebene in einer Meldung; mehr werden mit `…` abgekürzt.
const MAX_FIELDS_SHOWN: usize = 12;

/// Die Namen unter dem Pfad `prefix` der Datei `file` in der Reihenfolge der Datei, ohne Wiederholung.
fn level_fields<'a>(entries: &'a [ValueEntry], file: &str, prefix: &[String]) -> Vec<&'a str> {
    let mut out: Vec<&str> = Vec::new();
    for v in entries {
        if v.file != file || v.path.len() <= prefix.len() || !v.path.starts_with(prefix) {
            continue;
        }
        let name = v.path[prefix.len()].as_str();
        if !out.contains(&name) {
            out.push(name);
        }
    }
    out
}

/// Höchstens [`MAX_FIELDS_SHOWN`] Namen, danach `…`.
fn shown(names: &[&str]) -> String {
    if names.len() > MAX_FIELDS_SHOWN {
        return format!("{}, …", names[..MAX_FIELDS_SHOWN].join(", "));
    }
    names.join(", ")
}

impl Editor {
    /// Erklärt, warum das Feld `base` › `path` kein Blatt der Ansicht „Werte“ ist: unbekannte Datei (mit den erlaubten
    /// Dateien), Objekt oder Liste statt Blatt (gesperrt, wenn die Engine es so meldet, sonst mit den Feldern der Ebene)
    /// oder unbekanntes Feld (mit den Feldern der Ebene).
    pub(super) fn reject_override(&self, id: &str, entries: &[ValueEntry], base: &str, path: &[String]) -> String {
        let mut files: Vec<&str> = Vec::new();
        for v in entries {
            if !files.contains(&v.file.as_str()) {
                files.push(&v.file);
            }
        }
        if !files.contains(&base) {
            // Die Engine kennt auch Dateien ohne Blatt in der Ansicht (welt-events, RU40.4).
            let engine = self.probe_files(id, base);
            let list = engine.unwrap_or_else(|| files.join(", "));
            return format!("unbekannte Datei {base:?} (erlaubt: {list})");
        }
        if path.is_empty() {
            let fields = shown(&level_fields(entries, base, &[]));
            return format!("der Pfad fehlt (Felder von {base}: {fields})");
        }
        // Gesperrt kann ein Feld auch unterhalb der Stelle sein, die die Ansicht kennt: erst die Engine fragen.
        if let Err(locked) = self.probe_locked(id, base, path) {
            return locked;
        }
        let at = format!("{base} › {}", path.join(" › "));
        let below = level_fields(entries, base, path);
        if !below.is_empty() {
            return format!(
                "{at} ist ein Objekt oder eine Liste, übersteuerbar sind nur die Felder darunter ({})",
                shown(&below)
            );
        }
        for k in (0..path.len()).rev() {
            let fields = level_fields(entries, base, &path[..k]);
            if fields.is_empty() {
                continue;
            }
            let mut level = base.to_owned();
            if k > 0 {
                level = format!("{base} › {}", path[..k].join(" › "));
            }
            return format!("unbekanntes Feld {:?} in {level} (Felder: {})", path[k], shown(&fields));
        }
        format!("unbekanntes Feld {:?} in {base}", path.join(" › "))
    }

    /// Lässt die Engine einen Entwurf mit leerem Objekt am Pfad prüfen; nur die Ablehnung „nicht übersteuerbar“
    /// (welten.md § 1.3) kommt zurück, andere Fehler des Entwurfs sind hier ohne Belang.
    fn probe_locked(&self, id: &str, base: &str, path: &[String]) -> Result<(), String> {
        if path.len() > 1 {
            self.probe_locked(id, base, &path[..1])?;
        }
        let Ok((raw, out)) = self.draft_override(id, base, path, "{}", "") else {
            return Ok(());
        };
        if raw == out {
            return Ok(());
        }
        match self.check_draft(id, &out) {
            Err(e) if e.contains("nicht übersteuerbar") => Err(e),
            _ => Ok(()),
        }
    }

    /// Lässt die Engine einen Entwurf mit leerem Override der Datei `base` prüfen und liefert die Liste aus ihrer
    /// Ablehnung „(erlaubt: …)“; `None`, wenn die Engine keine solche Liste nennt.
    fn probe_files(&self, id: &str, base: &str) -> Option<String> {
        let raw = self.read(id).ok()?;
        let mut ov = overrides_of(&raw).ok()?;
        ov.insert(base.to_owned(), RawValue::from_string("{}".to_owned()).ok()?);
        let out = patch_overrides(&raw, &ov).ok()?;
        let error = self.check_draft(id, &out).err()?;
        let (_, list) = error.split_once("(erlaubt: ")?;
        let list = list.split(')').next().unwrap_or(list).trim();
        (!list.is_empty()).then(|| list.to_owned())
    }
}
