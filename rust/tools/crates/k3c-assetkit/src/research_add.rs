//! Recherche-Treffer als Vorschläge übernehmen (B-673, Go `research_add.go`): Treffer mit direkter Datei-URL, erlaubter
//! Quelle und Lizenz erkannt oder bestätigt werden zu Kandidaten; alle anderen stehen mit Grund in der Liste der
//! übersprungenen. Die Treffer bleiben in der Recherche stehen (der Vorschlag verweist per Datei-URL auf dieselbe Datei).

use std::collections::BTreeMap;
use std::path::Path;

use crate::credits::LICENSE_CONFIRMED;
use crate::error::Result;
use crate::fsutil;
use crate::net::url_path;
use crate::register::{Register, load};
use crate::research::{Hit, LICENSE_KNOWN, load_research};
use crate::suggestions::{SUGGESTION_SOURCES, Suggestion, suggestion_license_allowed, suggestion_targets};
use crate::suggestions_add::{SizeFunc, SkippedSuggestion, SuggestionAdd, add_suggestions, plan_suggestions};

/// Wandelt die Treffer in Kandidaten. `source` ≠ "" nimmt nur Treffer dieser Quelle. Slots in alphabetischer
/// Reihenfolge, damit Plan und Übernahme dieselbe Reihenfolge haben.
pub fn research_candidates(root: &Path, source: &str) -> Result<(Vec<Suggestion>, Vec<SkippedSuggestion>)> {
    let research = load_research(root)?;
    let register = load(root)?;
    let targets = suggestion_targets(&register);
    let source = source.to_lowercase();
    let (mut candidates, mut skipped) = (Vec::new(), Vec::new());
    for (slot, hits) in &research.hits {
        for h in hits {
            if !source.is_empty() && h.source.to_lowercase() != source {
                continue;
            }
            match research_candidate(slot, h, &register, &targets) {
                Ok(c) => candidates.push(c),
                Err(reason) => skipped.push(SkippedSuggestion {
                    title: h.title.clone(),
                    file_url: h.download.clone(),
                    reason,
                }),
            }
        }
    }
    Ok((candidates, skipped))
}

/// Der Treffer als Vorschlag, sonst der Grund.
fn research_candidate(
    slot: &str,
    h: &Hit,
    register: &Register,
    targets: &BTreeMap<String, String>,
) -> std::result::Result<Suggestion, String> {
    let source = h.source.to_lowercase();
    if !SUGGESTION_SOURCES.iter().any(|n| n.to_lowercase() == source) {
        return Err("Quelle nicht erlaubt".to_string());
    }
    if h.download.trim().is_empty() {
        return Err("keine direkte Datei-URL".to_string());
    }
    if h.license_status != LICENSE_KNOWN && h.license_status != LICENSE_CONFIRMED {
        return Err(format!("Lizenz nicht geklärt ({})", h.license_status));
    }
    let license = normalize_license(&h.license);
    if !suggestion_license_allowed(&license) {
        return Err(format!("Lizenz „{}“ nicht erlaubt (Q70)", h.license));
    }
    let kind = targets
        .get(slot)
        .or_else(|| register.find(slot).map(|s| &s.kind))
        .ok_or_else(|| "Slot nicht im Register".to_string())?;
    let file_name = url_path(&h.download).map_or_else(String::new, |p| fsutil::path_base(&p).to_string());
    Ok(Suggestion {
        slot: slot.to_string(),
        kind: kind.clone(),
        title: h.title.clone(),
        source: h.source.clone(),
        page: h.link.clone(),
        file_url: h.download.clone(),
        artist: h.artist.clone(),
        license,
        file_name,
        priority: 2,
        ..Suggestion::default()
    })
}

/// Schreibt „CC BY 4.0“ als „CC-BY 4.0“, wie die Liste und `suggestion_license_allowed` es erwarten.
fn normalize_license(license: &str) -> String {
    let license = license.trim();
    match (license.get(..5), license.get(5..)) {
        (Some(head), Some(tail)) if head.eq_ignore_ascii_case("CC BY") => format!("CC-BY{tail}"),
        _ => license.to_string(),
    }
}

/// Nennt, was die Übernahme anhängen würde (Größe per `size`), und was sie überspringt; schreibt nichts.
pub async fn plan_research_import(root: &Path, source: &str, size: Option<&SizeFunc>) -> Result<SuggestionAdd> {
    let (candidates, skipped) = research_candidates(root, source)?;
    let result = plan_suggestions(root, &candidates, size).await?;
    Ok(prepend(skipped, result))
}

/// Hängt die Treffer an die Vorschlagsliste. `only` nimmt nur Treffer mit diesen Datei-URLs (die im Dialog
/// bestätigten); die übrigen werden nicht aufgeführt.
pub async fn import_research(
    root: &Path,
    source: &str,
    only: Option<&[String]>,
    size: Option<&SizeFunc>,
) -> Result<SuggestionAdd> {
    let (mut candidates, skipped) = research_candidates(root, source)?;
    if let Some(only) = only {
        candidates.retain(|c| only.contains(&c.file_url));
    }
    let result = add_suggestions(root, &candidates, size).await?;
    Ok(prepend(skipped, result))
}

fn prepend(mut skipped: Vec<SkippedSuggestion>, mut result: SuggestionAdd) -> SuggestionAdd {
    skipped.append(&mut result.skipped);
    result.skipped = skipped;
    result
}
