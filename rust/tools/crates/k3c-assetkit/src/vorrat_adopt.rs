//! Übernahme aus dem Vorrat nach `public/`, Zuordnung an den Slot und Aufräumen (Go `vorrat_adopt.go`).

use std::fs;
use std::io;
use std::path::Path;
use std::sync::{Mutex, PoisonError};

use chrono::{DateTime, FixedOffset};

use crate::credits::add_credit;
use crate::error::{Error, Result};
use crate::fsutil;
use crate::import::{check_signature, import_ext, store_import, unknown_slot};
use crate::register::{self, Choice, load, public_file, set_choices_locked};
use crate::research::{Hit, LICENSE_KNOWN};
use crate::stamp::stamp;
use crate::suggestions::{Suggestion, load_suggestions, suggestion_license_allowed};
use crate::vorrat::{
    VORRAT_LOADED, VorratEntry, VorratIndex, change_vorrat, file_exists, load_vorrat, lock_vorrat, used_files,
    vorrat_path, vorrat_states,
};

/// Reiht Übernahmen hintereinander (B-678): der zweite Aufruf sieht die Datei des ersten und legt keine Kopie an.
static ADOPT_MU: Mutex<()> = Mutex::new(());

/// Gruppen der Vorschlagsliste ohne eigenen Slot (Skill-Effekt-Grafiken je Linie).
const GROUP_PREFIX: &str = "skill-effekt:";

/// Pakete (zip, 7z) übernimmt das Studio nicht selbst, die passende Datei wählt 🧑 von Hand.
pub const ERR_VORRAT_PACKAGE: &str = "Paket: Datei von Hand auswählen";

/// Übernimmt einen geladenen Eintrag: Kopie unter `public/`, Credit-Zeile und Zuordnung als letzte Präferenz; Gruppen
/// wie `skill-effekt:*` bekommen keine Zuordnung. Ein schon übernommener Eintrag liefert seine Datei, ohne zu schreiben.
pub fn vorrat_adopt(root: &Path, id: &str, now: DateTime<FixedOffset>) -> Result<String> {
    adopt_on(root, id, None, now)
}

/// [`vorrat_adopt`] nur auf dem Stand `stand` des Registers, sonst [`Error::Stale`] vor jedem Schreibzugriff; ein schon
/// übernommener Eintrag wird dem Slot zugeordnet, falls er dort fehlt. Liefert die Datei und den neuen Stand.
pub fn vorrat_adopt_if(root: &Path, id: &str, stand: &str, now: DateTime<FixedOffset>) -> Result<(String, String)> {
    if stand.is_empty() || stamp(root)? != stand {
        return Err(Error::Stale);
    }
    let rel = adopt_on(root, id, Some(stand), now)?;
    let next = stamp(root)?;
    Ok((rel, next))
}

/// Mit `stand`: geprüft unter der Register-Sperre vor dem ersten Schreiben; sie bleibt bis nach dem Anhängen gehalten,
/// damit bei [`Error::Stale`] weder Datei noch Credit-Zeile übrig bleiben.
pub(crate) fn adopt_on(root: &Path, id: &str, stand: Option<&str>, now: DateTime<FixedOffset>) -> Result<String> {
    let _adopting = ADOPT_MU.lock().unwrap_or_else(PoisonError::into_inner);
    let (s, entry) = adopt_target(root, id)?;
    let _register = match stand {
        Some(stand) => {
            let guard = register::lock();
            check_stand(root, stand)?;
            Some(guard)
        }
        None => None,
    };
    // Mit Stand hält diese Funktion die Register-Sperre schon.
    let attach = |rel: &str| match stand {
        Some(_) => append_choice_locked(root, &s.slot, rel, stand),
        None => append_choice(root, &s.slot, rel, None),
    };
    if !entry.public.is_empty() && public_file(root, &entry.public).is_ok_and(|p| file_exists(&p)) {
        if stand.is_some() {
            attach(&entry.public)?;
        }
        return Ok(entry.public.clone());
    }
    let rel = copy_to_public(root, &s, &entry, now)?;
    attach(&rel)?;
    mark_adopted(root, id, &rel);
    Ok(rel)
}

/// Kopie nach `public/` samt Credit-Zeile.
fn copy_to_public(root: &Path, s: &Suggestion, entry: &VorratEntry, now: DateTime<FixedOffset>) -> Result<String> {
    let (data, ext) = read_adopt_file(root, s, entry)?;
    let rel = store_import(root, &s.kind, &s.title, &ext, &data)?;
    let hit = Hit {
        title: s.title.clone(),
        artist: s.artist.clone(),
        license: s.license.clone(),
        link: s.page.clone(),
        source: s.source.clone(),
        license_status: LICENSE_KNOWN.to_string(),
        ..Hit::default()
    };
    add_credit(root, &s.kind, &rel, &hit, now)?;
    Ok(rel)
}

/// Liest die Vorrat-Datei des Eintrags und prüft vor dem Schreiben Endung, Slot und Signatur.
fn read_adopt_file(root: &Path, s: &Suggestion, entry: &VorratEntry) -> Result<(Vec<u8>, String)> {
    let ext = fsutil::path_ext(&entry.file).to_lowercase();
    if ext == ".zip" || ext == ".7z" {
        return Err(Error::Msg(ERR_VORRAT_PACKAGE.to_string()));
    }
    if !import_ext(&s.kind).contains(&ext.as_str()) {
        return Err(Error::Msg(format!("{}: {ext} passt nicht zur Art {}", s.id, s.kind)));
    }
    check_adopt_slot(root, s)?;
    let data = fs::read(vorrat_path(root, &entry.file))?;
    check_signature(&data, &ext)?;
    Ok((data, ext))
}

/// Vermerkt die übernommene Datei am Index-Eintrag. Die Zuordnung steht dann schon: scheitert nur der Index, gilt die
/// Übernahme trotzdem.
fn mark_adopted(root: &Path, id: &str, rel: &str) {
    let marked = change_vorrat(root, |index| {
        if let Some(entry) = index.find_mut(id) {
            entry.public = rel.to_string();
        }
        Ok(())
    });
    if let Err(e) = marked {
        log::warn!("❌ Vorrat-Index nicht aktualisiert id={id} datei={rel} fehler={e}");
    }
}

/// Sucht den Eintrag in Liste und Index; er muss geladen sein.
fn adopt_target(root: &Path, id: &str) -> Result<(Suggestion, VorratEntry)> {
    let list = load_suggestions(root)?.list;
    let mut states = vorrat_states(root, &list)?;
    let Some(s) = list.into_iter().find(|s| s.id == id) else {
        return Err(Error::Msg(format!("eintrag {id:?} nicht in der Vorschlagsliste")));
    };
    if !suggestion_license_allowed(&s.license) {
        return Err(Error::Msg(format!("{id}: Lizenz {} nicht erlaubt (Q70)", s.license)));
    }
    let state = states.remove(id).unwrap_or_default();
    match state.entry {
        Some(entry) if state.status == VORRAT_LOADED => Ok((s, entry)),
        _ => Err(Error::Msg(format!("{id} ist {}, nicht geladen", state.status))),
    }
}

/// Prüft vor dem Kopieren, dass der Slot existiert und zur Art passt (Gruppen ausgenommen).
fn check_adopt_slot(root: &Path, s: &Suggestion) -> Result<()> {
    if s.slot.starts_with(GROUP_PREFIX) {
        return Ok(());
    }
    let register = load(root)?;
    let slot = register.find(&s.slot).ok_or_else(|| unknown_slot(&s.slot))?;
    if slot.kind != s.kind {
        return Err(Error::Msg(format!(
            "{} ist {}, der Eintrag {}",
            s.slot, slot.kind, s.kind
        )));
    }
    Ok(())
}

/// Hängt `rel` als letzte Präferenz an den Slot, wenn er sie noch nicht hat; Gruppen bekommen keine Zuordnung. Mit
/// `stand` nur auf diesem Stand des Registers (sonst [`Error::Stale`], nichts geschrieben).
pub(crate) fn append_choice(root: &Path, slot_id: &str, rel: &str, stand: Option<&str>) -> Result<()> {
    let _guard = register::lock();
    append_choice_locked(root, slot_id, rel, stand)
}

/// Liefert [`Error::Stale`], wenn das Register nicht mehr den Stand `stand` hat; der Aufrufer hält die Sperre.
fn check_stand(root: &Path, stand: &str) -> Result<()> {
    if stamp(root)? != stand {
        return Err(Error::Stale);
    }
    Ok(())
}

/// [`append_choice`] ohne Sperre; der Aufrufer hält [`register::lock`].
fn append_choice_locked(root: &Path, slot_id: &str, rel: &str, stand: Option<&str>) -> Result<()> {
    if slot_id.starts_with(GROUP_PREFIX) {
        return Ok(());
    }
    if let Some(stand) = stand {
        check_stand(root, stand)?;
    }
    let register = load(root)?;
    let slot = register.find(slot_id).ok_or_else(|| unknown_slot(slot_id))?;
    if slot.choices().iter().any(|c| c.file == rel) {
        return Ok(());
    }
    let mut choices = slot.choices().to_vec();
    choices.push(Choice {
        file: rel.to_string(),
        ..Choice::default()
    });
    set_choices_locked(root, slot_id, &choices)?;
    Ok(())
}

/// Die geladenen Dateien, die keinem Slot zugeordnet sind (das Aufräumen entfernt sie).
pub fn vorrat_cleanup_preview(root: &Path) -> Result<Vec<VorratEntry>> {
    let _guard = lock_vorrat();
    let index = load_vorrat(root)?;
    unused_vorrat(root, &index)
}

/// Entfernt die geladenen, unverwendeten Dateien samt Index-Eintrag; die Einträge sind danach wieder offen.
pub fn vorrat_cleanup(root: &Path) -> Result<Vec<VorratEntry>> {
    let mut removed: Vec<VorratEntry> = Vec::new();
    change_vorrat(root, |index| {
        for entry in unused_vorrat(root, index)? {
            match fsutil::remove(&vorrat_path(root, &entry.file)) {
                Err(e) if e.kind() != io::ErrorKind::NotFound => return Err(Error::Io(e)),
                _ => removed.push(entry),
            }
        }
        if let Some(entries) = &mut index.entries {
            entries.retain(|e| !removed.iter().any(|gone| gone.id == e.id));
        }
        Ok(())
    })?;
    Ok(removed)
}

/// Einträge mit Datei im Vorrat, deren übernommene Datei (falls vorhanden) keinem Slot zugeordnet ist.
fn unused_vorrat(root: &Path, index: &VorratIndex) -> Result<Vec<VorratEntry>> {
    let used = used_files(root)?;
    Ok(index
        .entries()
        .iter()
        .filter(|e| file_exists(&vorrat_path(root, &e.file)))
        .filter(|e| e.public.is_empty() || !used.contains(&e.public))
        .cloned()
        .collect())
}

#[cfg(test)]
#[path = "vorrat_adopt_parallel_tests.rs"]
mod parallel_tests;

#[cfg(test)]
#[path = "vorrat_adopt_stale_tests.rs"]
mod stale_tests;
