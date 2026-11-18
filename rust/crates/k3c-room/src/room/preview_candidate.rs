//! Vorschau aus einer Kandidaten-hub.json (WZ9.1, B-462): der Gebäude-Planer zeigt seinen ungespeicherten Stand, ohne
//! Neubau und ohne die Daten zu wechseln. Port von `preview_candidate.go`. Geplante Chunks des Levels (WZ8.4) gehen mit; der
//! Generator prüft sie gegen die Linien des Kandidaten.
use k3c_data::biome::Biome;
use k3c_level::types::PlannedChunk;
use k3c_sim::hub::validate::{HubFinding, RULE_JSON};
use k3c_sim::hub::{parse_hub, validate_hub_data};
use k3c_sim::island::create::CreateOptions;
use k3c_sim::state::World;
use k3c_sim::world::create_world_with;

use crate::fehler::Fehler;

/// Go: `CandidateError.Error`: die Befunde der Prüfung als Fehler.
fn candidate_error(findings: &[HubFinding]) -> Fehler {
    let parts: Vec<String> = findings.iter().map(HubFinding::to_string).collect();
    Fehler::Candidate(format!(
        "vorschau: Kandidaten-hub.json mit {} Befunden: {}",
        findings.len(),
        parts.join("; ")
    ))
}

/// Liest und prüft den Kandidaten einmal und baut die Stufe über `create_world_with` (ohne Insel, wie die Vorschau ohne
/// Kandidat). Liefert dazu die Zahl der Hub-Stufen des Kandidaten.
pub(super) fn candidate_world(
    biome: &Biome,
    seed: &str,
    raw: &str,
    planned: &[PlannedChunk],
    chunk_count: i64,
) -> Result<(World, i64), Fehler> {
    let h = match parse_hub(raw) {
        Ok(h) => h,
        Err(e) => {
            let finding = HubFinding::new(RULE_JSON, "hub.json", e.to_string());
            return Err(candidate_error(&[finding]));
        }
    };
    let report = validate_hub_data(&h);
    if !report.findings.is_empty() {
        return Err(candidate_error(&report.findings));
    }
    let levels = h.levels.len() as i64;
    let opts = CreateOptions {
        candidate: Some(h),
        ..CreateOptions::default()
    };
    let w = create_world_with(biome, seed, &opts, planned, chunk_count)
        .map_err(|e| Fehler::Preview(format!("vorschau: Level aus dem Kandidaten nicht erzeugbar: {e}")))?;
    Ok((w, levels))
}
