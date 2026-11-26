//! Recherche- und Katalog-Treffer der Kandidatenliste samt Lizenz-Marke (Teil von Go `candidates.go`).

use crate::candidates::{
    Builder, Candidate, Found, ID_CATALOG, ID_HIT, MARK_BLOCKED, MARK_CHECK, MARK_FREE, MARK_SA, OFFLINE_REASON,
    ORIGIN_RESEARCH, STAGE_NET,
};
use crate::catalog::{OWN_BUILDING, SELF_MADE, license_flag, search};
use crate::credits::LICENSE_CONFIRMED;
use crate::fsutil;
use crate::import_download::download_allowed;
use crate::net::{is_test_origin, parse_with_host, plain_authority, url_path};
use crate::research::{Hit, KNOWN_LICENSES, LICENSE_CHECK, LICENSE_EXCLUDED, search_term};

/// Gilt für die Katalog-Treffer, wenn `limit` ≤ 0 ist.
const DEFAULT_CANDIDATE_LIMIT: usize = 50;

impl Builder<'_> {
    /// Die Recherche-Treffer ohne Import und ohne Ausschluss (Q70). Ein Treffer mit der Datei-URL eines Vorschlags
    /// ergänzt dessen Herkunft, statt eine zweite Zeile zu bilden.
    pub(crate) fn hits(&self, list: &[Hit], found: &mut Found) {
        for h in list {
            if !h.file.is_empty() || h.license_status == LICENSE_EXCLUDED {
                continue;
            }
            if !h.download.is_empty()
                && let Some(&seen) = found.by_url.get(&h.download)
            {
                if let Some(i) = seen {
                    found.list[i].origins.push(ORIGIN_RESEARCH);
                }
                continue;
            }
            found.net.push(found.list.len());
            found.list.push(self.hit_candidate(h));
        }
    }

    fn hit_candidate(&self, h: &Hit) -> Candidate {
        let mut confirm_license = false;
        let reason = if license_blocked(&h.license, &h.flag) {
            blocked_reason(&h.license, &h.flag)
        } else if h.license_status == LICENSE_CHECK {
            confirm_license = true;
            "Lizenz prüfen".to_string()
        } else if !hit_downloadable(h) {
            "kein direkter Download, Seite öffnen".to_string()
        } else if self.opt.offline {
            OFFLINE_REASON.to_string()
        } else {
            String::new()
        };
        Candidate {
            id: format!("{ID_HIT}{}", h.id),
            stage: STAGE_NET,
            title: h.title.clone(),
            file: hit_file_name(h),
            source: h.source.clone(),
            artist: h.artist.clone(),
            license: h.license.clone(),
            mark: hit_mark(h),
            link: h.link.clone(),
            origins: vec![ORIGIN_RESEARCH],
            assignable: reason.is_empty(),
            reason,
            confirm_license,
            ..Candidate::default()
        }
    }

    /// Katalog-Treffer zur Suche für die Art des Slots, ohne Zuordnungen des Slots und ohne Dateien, die schon als
    /// Vorrat-Kandidat erscheinen.
    pub(crate) fn catalog_hits(&self) -> Vec<Candidate> {
        let mut query = self.opt.query.trim();
        if query.is_empty() {
            query = search_term(&self.slot);
        }
        if query.is_empty() {
            query = &self.slot.id;
        }
        let limit = usize::try_from(self.opt.limit)
            .ok()
            .filter(|&n| n > 0)
            .unwrap_or(DEFAULT_CANDIDATE_LIMIT);
        search(&self.catalog, query, &self.slot.kind, 0)
            .iter()
            .filter(|a| !self.chosen.contains(&a.file) && !self.taken.contains(&a.file))
            .take(limit)
            .map(|a| {
                let mut c = self.game_candidate(ID_CATALOG, &a.file, a);
                if license_blocked(&a.license, &a.flag) {
                    (c.mark, c.assignable) = (MARK_BLOCKED, false);
                    c.reason = blocked_reason(&a.license, &a.flag);
                }
                c
            })
            .collect()
    }
}

/// ND, NC und Sampling+ sind nie übernehmbar (Q70), auch nicht nach „Lizenz bestätigen“. `flag` ist das Kennzeichen des
/// Treffers oder Katalog-Eintrags; zusätzlich zählt der Lizenztext.
pub(crate) fn license_blocked(license: &str, flag: &str) -> bool {
    [flag, license_flag(license)].iter().any(|f| *f == "ND" || *f == "NC")
        || license.to_uppercase().contains("SAMPLING")
}

pub(crate) fn blocked_reason(license: &str, flag: &str) -> String {
    let name = if license.is_empty() { flag } else { license };
    format!("Lizenz {name} nicht erlaubt (Q70)")
}

/// Unbestätigte unklare Lizenz = prüfen; eine von 🧑 bestätigte gilt als frei, sofern kein SA-Kennzeichen bleibt;
/// gesperrte Lizenzen bleiben gesperrt.
fn hit_mark(h: &Hit) -> &'static str {
    let mark = license_mark(&h.license);
    if license_blocked(&h.license, &h.flag) {
        MARK_BLOCKED
    } else if h.license_status == LICENSE_CHECK {
        MARK_CHECK
    } else if h.license_status == LICENSE_CONFIRMED && mark == MARK_CHECK {
        MARK_FREE
    } else {
        mark
    }
}

/// Direkter Download über https von einem Host aus `DOWNLOAD_HOSTS` (wie `download_hit`).
pub(crate) fn hit_downloadable(h: &Hit) -> bool {
    plain_authority(&h.download)
        && parse_with_host(&h.download)
            .is_some_and(|u| (u.scheme() == "https" || is_test_origin(&u)) && download_allowed(&u))
}

fn hit_file_name(h: &Hit) -> String {
    if let Some(path) = url_path(&h.download) {
        let base = fsutil::path_base(&path);
        if base != "." && base != "/" {
            return base.to_string();
        }
    }
    h.title.clone()
}

/// Ordnet einen Lizenztext ein: CC0, CC-BY, OGA-BY u. a. = frei, SA = sa, NC, ND, Sampling+ = gesperrt, leer oder
/// unklar = pruefen.
pub(crate) fn license_mark(license: &str) -> &'static str {
    if license_blocked(license, "") {
        return MARK_BLOCKED;
    }
    if license_flag(license) == "SA" {
        return MARK_SA;
    }
    if license == SELF_MADE || license == OWN_BUILDING {
        return MARK_FREE;
    }
    let upper = license.to_uppercase();
    if KNOWN_LICENSES.iter().any(|k| upper.contains(k)) {
        MARK_FREE
    } else {
        MARK_CHECK
    }
}
