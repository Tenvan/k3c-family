//! Kandidaten eines Asset-Slots (B-677, WZ33, Go `candidates.go`): Zuordnungen, Vorrat, Netz (Vorschläge, Recherche)
//! und Katalog in einer Liste, ohne HTTP-Server und Oberfläche. Felder und Werte wie `api/candidateTypes.ts` des
//! Studio-Frontends. Recherche- und Katalog-Treffer stehen in `candidates_hits.rs`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::candidates_hits::license_mark;
use crate::catalog::{Asset, catalog};
use crate::error::{Error, Result};
use crate::fsutil;
use crate::register::{Slot, load, public_file};
use crate::research::hits_of;
use crate::suggestions::{Suggestion, load_suggestions};
use crate::vorrat::{VORRAT_LOADED, VORRAT_OPEN, VorratEntry, VorratState, vorrat_states};
use crate::vorrat_adopt::ERR_VORRAT_PACKAGE;
use crate::vorrat_download::check_vorrat_download;

/// Stufe eines Kandidaten auf dem Weg ins Spiel.
pub const STAGE_NET: &str = "netz";
pub const STAGE_STOCK: &str = "vorrat";
pub const STAGE_GAME: &str = "spiel";

/// Lizenz-Marke eines Kandidaten.
pub const MARK_FREE: &str = "frei";
pub const MARK_SA: &str = "sa";
pub const MARK_CHECK: &str = "pruefen";
pub const MARK_BLOCKED: &str = "gesperrt";

/// Herkunft eines Kandidaten.
pub const ORIGIN_CATALOG: &str = "katalog";
pub const ORIGIN_STOCK: &str = "vorrat";
pub const ORIGIN_SUGGESTION: &str = "vorschlag";
pub const ORIGIN_RESEARCH: &str = "recherche";

/// Präfixe der Kandidaten-IDs.
pub(crate) const ID_CHOICE: &str = "wahl:";
pub(crate) const ID_CATALOG: &str = "cat:";
pub(crate) const ID_SUGGESTION: &str = "sug:";
pub(crate) const ID_HIT: &str = "hit:";

pub(crate) const OFFLINE_REASON: &str = "offline";

/// Zeile der Kandidatenliste.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Candidate {
    pub id: String,
    #[serde(rename = "stufe")]
    pub stage: &'static str,
    /// 0 = nicht zugeordnet, n = Präferenz n des Slots.
    #[serde(rename = "praef")]
    pub pref: i64,
    #[serde(rename = "titel")]
    pub title: String,
    #[serde(rename = "datei")]
    pub file: String,
    #[serde(rename = "quelle")]
    pub source: String,
    #[serde(rename = "urheber", skip_serializing_if = "String::is_empty")]
    pub artist: String,
    #[serde(rename = "lizenz")]
    pub license: String,
    #[serde(rename = "marke")]
    pub mark: &'static str,
    /// Warum „Zuordnen“ gesperrt ist.
    #[serde(rename = "grund", skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(rename = "groesse")]
    pub size: i64,
    #[serde(rename = "vorschau")]
    pub preview: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub link: String,
    #[serde(rename = "herkunft")]
    pub origins: Vec<&'static str>,
    #[serde(rename = "zuordenbar")]
    pub assignable: bool,
    /// Treffer mit Lizenz „prüfen“: 🧑 kann bestätigen.
    #[serde(rename = "lizenzBestaetigen", skip_serializing_if = "std::ops::Not::not")]
    pub confirm_license: bool,
}

/// Steuert die Liste: Katalog-Suche (leer = Beschreibung des Slots), Netz-Zustand und die Adress-Präfixe für
/// Vorschauen (Dateien unter `public/` und im Vorrat).
#[derive(Debug, Clone, Default)]
pub struct CandidateOptions {
    pub query: String,
    pub limit: i64,
    pub offline: bool,
    pub public_url: String,
    pub vorrat_url: String,
}

pub(crate) struct Builder<'a> {
    pub(crate) root: &'a Path,
    pub(crate) slot: Slot,
    pub(crate) opt: &'a CandidateOptions,
    pub(crate) catalog: Vec<Asset>,
    pub(crate) chosen: HashSet<String>,
    /// Katalog-Dateien, die schon als Vorrat-Kandidat erscheinen.
    pub(crate) taken: HashSet<String>,
}

/// Vorrat- und Netz-Kandidaten; ein Recherche-Treffer kann die Herkunft eines früheren ergänzen.
#[derive(Default)]
pub(crate) struct Found {
    pub(crate) list: Vec<Candidate>,
    /// Stellen in `list`.
    pub(crate) stock: Vec<usize>,
    pub(crate) net: Vec<usize>,
    /// Datei-URL → Stelle in `list`; `None` = die URL gehört einem verworfenen oder verwendeten Eintrag.
    pub(crate) by_url: HashMap<String, Option<usize>>,
}

impl Found {
    fn push(&mut self, candidate: Candidate) -> usize {
        self.list.push(candidate);
        self.list.len() - 1
    }

    /// Erst der Vorrat, dann das Netz.
    fn ordered(&self) -> impl Iterator<Item = Candidate> + '_ {
        self.stock.iter().chain(&self.net).map(|&i| self.list[i].clone())
    }
}

/// Kandidaten des Slots: Zuordnungen (nach Präferenz), Vorrat, Netz (Vorschläge nach Priorität, dann
/// Recherche-Treffer) und Katalog-Treffer zur Suche. Gleiche Datei-URL erscheint einmal mit allen Herkünften.
pub fn candidates(root: &Path, slot_id: &str, options: &CandidateOptions) -> Result<Vec<Candidate>> {
    let register = load(root)?;
    let Some(slot) = register.find(slot_id) else {
        return Err(Error::Msg(format!("unbekannter Asset-Slot {slot_id:?}")));
    };
    let catalog = catalog(root)?;
    let suggestions = load_suggestions(root)?;
    let states = vorrat_states(root, &suggestions.list)?;
    let hits = hits_of(root, slot_id)?;
    let mut builder = Builder {
        root,
        slot: slot.clone(),
        opt: options,
        catalog,
        chosen: HashSet::new(),
        taken: HashSet::new(),
    };
    let mut out = builder.assigned();
    let mut found = builder.suggestions(&suggestions.list, &states);
    builder.hits(&hits, &mut found);
    out.extend(found.ordered());
    out.extend(builder.catalog_hits());
    Ok(out)
}

/// Ist `file` eine vorhandene Datei unter `public/`?
fn public_exists(root: &Path, file: &str) -> bool {
    public_file(root, file).is_ok()
}

impl Builder<'_> {
    /// Die Zuordnungen des Slots in der Reihenfolge der Präferenz; eine Datei mit mehreren Ausschnitten erscheint
    /// einmal (die ID ist die Datei).
    fn assigned(&mut self) -> Vec<Candidate> {
        let by_file: HashMap<&str, &Asset> = self.catalog.iter().map(|a| (a.file.as_str(), a)).collect();
        let none = Asset::default();
        let mut out = Vec::new();
        for (i, choice) in (1i64..).zip(self.slot.choices()) {
            if !self.chosen.insert(choice.file.clone()) {
                continue;
            }
            let asset = by_file.get(choice.file.as_str()).copied().unwrap_or(&none);
            let mut c = self.game_candidate(ID_CHOICE, &choice.file, asset);
            (c.pref, c.assignable, c.reason) = (i, false, "schon zugeordnet".to_string());
            if !public_exists(self.root, &choice.file) {
                c.reason = "Datei fehlt unter public/".to_string();
            }
            out.push(c);
        }
        out
    }

    /// Zeile einer Datei unter `public/` (Zuordnung oder Katalog); `asset` ist ihr Katalog-Eintrag, sonst leer.
    pub(crate) fn game_candidate(&self, prefix: &str, file: &str, asset: &Asset) -> Candidate {
        let title = if asset.title.is_empty() {
            fsutil::path_base(file).to_string()
        } else {
            asset.title.clone()
        };
        let link = if asset.source.starts_with("https://") {
            asset.source.clone()
        } else {
            String::new()
        };
        let size = public_file(self.root, file)
            .ok()
            .and_then(|path| fs::metadata(path).ok())
            .map_or(0, |meta| i64::try_from(meta.len()).unwrap_or(i64::MAX));
        Candidate {
            id: format!("{prefix}{file}"),
            stage: STAGE_GAME,
            title,
            file: file.to_string(),
            source: asset.source.clone(),
            artist: asset.artist.clone(),
            license: asset.license.clone(),
            mark: license_mark(&asset.license),
            preview: format!("{}{file}", self.opt.public_url),
            link,
            size,
            origins: vec![ORIGIN_CATALOG],
            assignable: true,
            ..Candidate::default()
        }
    }

    /// Teilt die Vorschläge des Slots in Vorrat (geladen, nicht verwendet) und Netz (offen); verworfene, gesperrte und
    /// verwendete Einträge fehlen, ihre Datei-URL bleibt für die Dublettenprüfung der Treffer vermerkt.
    fn suggestions(&mut self, list: &[Suggestion], states: &BTreeMap<String, VorratState>) -> Found {
        let mut mine: Vec<&Suggestion> = list
            .iter()
            .filter(|s| s.slot == self.slot.id && s.kind == self.slot.kind)
            .collect();
        mine.sort_by_key(|s| s.priority);
        let mut found = Found::default();
        for s in mine {
            let state = states.get(&s.id);
            let status = state.map_or("", |st| st.status);
            let used = state.is_some_and(|st| st.used);
            let mut index = None;
            if status == VORRAT_LOADED
                && !used
                && let Some(entry) = state.and_then(|st| st.entry.as_ref())
            {
                let c = self.stock_candidate(s, entry);
                let i = found.push(c);
                found.stock.push(i);
                index = Some(i);
            } else if status == VORRAT_OPEN {
                let i = found.push(self.net_suggestion(s));
                found.net.push(i);
                index = Some(i);
            }
            // Ein verworfener Eintrag mit gleicher URL verdrängt keinen Kandidaten.
            if index.is_some() || found.by_url.get(&s.file_url).copied().flatten().is_none() {
                found.by_url.insert(s.file_url.clone(), index);
            }
        }
        found
    }

    fn stock_candidate(&mut self, s: &Suggestion, entry: &VorratEntry) -> Candidate {
        let file = if entry.public.is_empty() {
            &s.file_name
        } else {
            &entry.public
        };
        let mut c = Candidate {
            id: format!("{ID_SUGGESTION}{}", s.id),
            stage: STAGE_STOCK,
            title: s.title.clone(),
            file: file.clone(),
            source: s.source.clone(),
            artist: s.artist.clone(),
            license: s.license.clone(),
            mark: license_mark(&s.license),
            size: entry.size,
            preview: format!("{}{}", self.opt.vorrat_url, entry.file),
            link: s.page.clone(),
            origins: vec![ORIGIN_STOCK],
            assignable: true,
            ..Candidate::default()
        };
        if is_package(&s.file_name) {
            (c.assignable, c.reason) = (false, ERR_VORRAT_PACKAGE.to_string());
        }
        if !entry.public.is_empty() && public_exists(self.root, &entry.public) {
            // Dieselbe Datei steht im Katalog: eine Zeile mit beiden Herkünften.
            self.taken.insert(entry.public.clone());
            c.origins.push(ORIGIN_CATALOG);
        }
        c
    }

    fn net_suggestion(&self, s: &Suggestion) -> Candidate {
        let reason = match check_vorrat_download(s) {
            Err(e) => e.to_string(),
            Ok(_) if is_package(&s.file_name) => ERR_VORRAT_PACKAGE.to_string(),
            Ok(_) if self.opt.offline => OFFLINE_REASON.to_string(),
            Ok(_) => String::new(),
        };
        Candidate {
            id: format!("{ID_SUGGESTION}{}", s.id),
            stage: STAGE_NET,
            title: s.title.clone(),
            file: s.file_name.clone(),
            source: s.source.clone(),
            artist: s.artist.clone(),
            license: s.license.clone(),
            mark: license_mark(&s.license),
            size: s.size,
            link: s.page.clone(),
            origins: vec![ORIGIN_SUGGESTION],
            assignable: reason.is_empty(),
            reason,
            ..Candidate::default()
        }
    }
}

pub(crate) fn is_package(name: &str) -> bool {
    let ext = fsutil::path_ext(name).to_lowercase();
    ext == ".zip" || ext == ".7z"
}

#[cfg(test)]
#[path = "candidates_tests.rs"]
mod tests;
