//! Kandidat zuordnen (Go `candidates_assign.go`): bei Bedarf laden (Netz → Vorrat → Spiel in einem Aufruf).

use std::path::Path;
use std::sync::Arc;

use chrono::{DateTime, FixedOffset};

use crate::candidates::{ID_CATALOG, ID_CHOICE, ID_HIT, ID_SUGGESTION, is_package};
use crate::candidates_hits::{blocked_reason, hit_downloadable, license_blocked};
use crate::catalog::catalog;
use crate::error::{Error, Result};
use crate::import::import_target;
use crate::import_download::download_hit_on;
use crate::net::DownloadClient;
use crate::register::{Choice, Slot, load};
use crate::research::LICENSE_CHECK;
use crate::stamp::{set_choices_if, stamp};
use crate::suggestions::{Suggestion, load_suggestions};
use crate::vorrat::{VORRAT_LOADED, VORRAT_OPEN, vorrat_states};
use crate::vorrat_adopt::{ERR_VORRAT_PACKAGE, vorrat_adopt_if};
use crate::vorrat_download::{check_vorrat_download, vorrat_download};

/// Naht von [`assign_candidate`] zum Netz: der Client der Downloads (Host- und Redirect-Prüfung liegt in den
/// Funktionen darunter), die Offline-Probe (nur gefragt, wenn ein Download nötig ist), der Download-Platz (`reserve`
/// belegt ihn nur für die Dauer eines Downloads, `false` = es läuft schon einer; `release` gibt ihn frei) und die Uhr.
#[derive(Clone)]
pub struct AssignOptions {
    pub client: DownloadClient,
    pub offline: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
    pub reserve: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
    pub release: Option<Arc<dyn Fn() + Send + Sync>>,
    pub now: DateTime<FixedOffset>,
}

/// Belegter Download-Platz; wird beim Fallenlassen frei (auch wenn der Aufruf abgebrochen wird).
struct Reserved(Option<Arc<dyn Fn() + Send + Sync>>);

impl Drop for Reserved {
    fn drop(&mut self) {
        if let Some(release) = &self.0 {
            release();
        }
    }
}

impl AssignOptions {
    /// Belegt den Download-Platz; ohne `reserve` ist er immer frei.
    fn reserve(&self) -> Result<Reserved> {
        let Some(reserve) = &self.reserve else {
            return Ok(Reserved(None));
        };
        if !reserve() {
            return Err(Error::Msg("es läuft schon ein Download".to_string()));
        }
        Ok(Reserved(self.release.clone()))
    }

    fn check_online(&self) -> Result<()> {
        if self.offline.as_ref().is_some_and(|offline| offline()) {
            return Err(Error::Msg("offline: Downloads brauchen Internet".to_string()));
        }
        Ok(())
    }
}

/// Ordnet den Kandidaten `id` dem Slot als letzte Präferenz zu und lädt ihn dafür bei Bedarf. Der Kandidat wird aus
/// Katalog, Vorschlagsliste und Treffern des Slots neu aufgelöst; Lizenz und Größe kommen nie aus der Anfrage. Der Stand
/// des Registers wird vor jedem Schreiben und jedem Download geprüft ([`Error::Stale`]); `confirmed_bytes` muss bei
/// Vorschlägen deren Größe sein. Scheitert ein Schritt, ist nichts zugeordnet (ein schon geladener Vorrat-Eintrag bleibt
/// im Vorrat). Liefert den Slot und den neuen Stand.
pub async fn assign_candidate(
    root: &Path,
    slot_id: &str,
    id: &str,
    stand: &str,
    confirmed_bytes: i64,
    options: &AssignOptions,
) -> Result<(Slot, String)> {
    if stand.is_empty() || stamp(root)? != stand {
        return Err(Error::Stale);
    }
    let register = load(root)?;
    let Some(slot) = register.find(slot_id) else {
        return Err(unknown_slot(slot_id));
    };
    if let Some(file) = id.strip_prefix(ID_CATALOG) {
        return assign_catalog(root, slot, file, stand);
    }
    if id.starts_with(ID_CHOICE) {
        return Err(Error::Msg("schon zugeordnet".to_string()));
    }
    if let Some(sid) = id.strip_prefix(ID_SUGGESTION) {
        assign_suggestion(root, slot, sid, stand, confirmed_bytes, options).await?;
    } else if let Some(hid) = id.strip_prefix(ID_HIT) {
        assign_hit(root, slot, hid, stand, options).await?;
    } else {
        return Err(Error::Msg(format!("kandidat {id:?} unbekannt")));
    }
    reload_slot(root, slot_id)
}

fn unknown_slot(slot_id: &str) -> Error {
    Error::Msg(format!("unbekannter Asset-Slot {slot_id:?}"))
}

/// Hängt eine Datei des Katalogs an; ND und NC bleiben gesperrt (Q70).
fn assign_catalog(root: &Path, slot: &Slot, file: &str, stand: &str) -> Result<(Slot, String)> {
    let list = catalog(root)?;
    let fail = |text: String| Err(Error::Msg(text));
    let Some(asset) = list.iter().find(|a| a.file == file) else {
        return fail(format!("datei {file:?} nicht im Katalog"));
    };
    if asset.kind != slot.kind {
        return fail(format!("{file} ist {}, der Slot {}", asset.kind, slot.kind));
    }
    if license_blocked(&asset.license, &asset.flag) {
        return fail(format!("{file}: Lizenz {} nicht erlaubt (Q70)", asset.license));
    }
    if slot.choices().iter().any(|c| c.file == file) {
        return fail("schon zugeordnet".to_string());
    }
    let mut choices = slot.choices().to_vec();
    choices.push(Choice {
        file: file.to_string(),
        ..Choice::default()
    });
    set_choices_if(root, &slot.id, &choices, stand)
}

/// Übernimmt einen Vorschlag aus dem Vorrat; ist er noch offen, lädt es ihn vorher (nach Offline-, Lizenz- und
/// Größenprüfung) in den Vorrat.
async fn assign_suggestion(
    root: &Path,
    slot: &Slot,
    sid: &str,
    stand: &str,
    confirmed: i64,
    options: &AssignOptions,
) -> Result<()> {
    let list = load_suggestions(root)?;
    let fail = |text: String| Err(Error::Msg(text));
    let Some(s) = list.list.iter().find(|s| s.id == sid) else {
        return fail(format!("vorschlag {sid:?} unbekannt"));
    };
    if s.slot != slot.id {
        return fail(format!("vorschlag {sid:?} gehört zu {}, nicht zu {}", s.slot, slot.id));
    }
    let states = vorrat_states(root, std::slice::from_ref(s))?;
    let state = states.get(&s.id).cloned().unwrap_or_default();
    match state.status {
        VORRAT_LOADED => {}
        VORRAT_OPEN => load_suggestion(root, s, confirmed, options).await?,
        status => return fail(format!("{sid} ist {status}: {}", state.reason)),
    }
    vorrat_adopt_if(root, sid, stand, options.now)?;
    Ok(())
}

/// Prüft und lädt einen offenen Vorschlag in den Vorrat.
async fn load_suggestion(root: &Path, s: &Suggestion, confirmed: i64, options: &AssignOptions) -> Result<()> {
    options.check_online()?;
    check_vorrat_download(s)?;
    if is_package(&s.file_name) {
        return Err(Error::Msg(ERR_VORRAT_PACKAGE.to_string()));
    }
    if confirmed != s.size {
        return Err(Error::Msg(format!(
            "bestätigt {confirmed} Bytes, der Vorschlag hat {}: bitte neu bestätigen",
            s.size
        )));
    }
    let _reserved = options.reserve()?;
    vorrat_download(root, s, &options.client, options.now, None)
        .await
        .map_err(|e| Error::Msg(format!("download {}: {e}", s.id)))?;
    Ok(())
}

/// Lädt einen Recherche-Treffer mit direktem Download und erkannter Lizenz; `download_hit_on` legt Datei, Credit und
/// Zuordnung (letzte Präferenz, nur auf dem Stand `stand`) an. Die Größe ist bei Treffern unbekannt, `confirmed_bytes`
/// zählt hier nicht. Wird der Stand erst nach dem Download veraltet, ist die Datei importiert (Katalog), aber nicht
/// zugeordnet.
async fn assign_hit(root: &Path, slot: &Slot, hid: &str, stand: &str, options: &AssignOptions) -> Result<()> {
    let (_, hit) = import_target(root, &slot.id, hid)?;
    let fail = |text: String| Err(Error::Msg(text));
    if !hit.file.is_empty() {
        return fail(format!("treffer {hid} ist schon importiert"));
    }
    if license_blocked(&hit.license, &hit.flag) {
        let reason = blocked_reason(&hit.license, &hit.flag);
        return fail(format!("treffer {hid}: {reason}"));
    }
    if hit.license_status == LICENSE_CHECK {
        return fail(format!("treffer {hid}: Lizenz prüfen und erst bestätigen"));
    }
    if !hit_downloadable(&hit) {
        return fail(format!(
            "treffer {hid}: kein direkter Download, bitte manuell laden und importieren"
        ));
    }
    options.check_online()?;
    let _reserved = options.reserve()?;
    download_hit_on(root, &slot.id, hid, Some(stand), &options.client, options.now).await?;
    Ok(())
}

fn reload_slot(root: &Path, id: &str) -> Result<(Slot, String)> {
    let register = load(root)?;
    let slot = register.find(id).ok_or_else(|| unknown_slot(id))?.clone();
    Ok((slot, stamp(root)?))
}
