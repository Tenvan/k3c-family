//! `plan_check` (B-784, Go `check.go`): je Ticket oder Sprint die Mängel der Planungsangaben für Arbeitswellen
//! (docs/arbeitsweise.md › Planungsangaben): Bereich leer, Umgebung ?, Größe L, unbekannte ID in „Abhängig von“,
//! Bereich überlappt mit einem anderen offenen Ticket, dazu fehlende Zeilen. Archivierte Tickets werden nicht geprüft.

use std::path::Path;

use serde::Deserialize;

use super::edit::head_fields;
use super::store::{read_text, resolve};
use super::waves_nodes::{WaveData, load_wave_data};
use super::waves_spec::{PLAN_LINES, area_overlap, parse_plan_spec};

/// Eingabe von [`check`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CheckIn {
    /// Ticket B-123 oder Sprint M8
    pub id: String,
}

/// Die Mängel eines Tickets oder Sprints als Text.
pub fn check(root: &Path, input: &CheckIn) -> Result<String, String> {
    let id = input.id.trim();
    let found = resolve(root, id)?;
    if found.kind != "ticket" && found.kind != "sprint" {
        return Err(format!(
            "plan_check prüft Tickets und Sprints, nicht {} {id}",
            found.kind
        ));
    }
    if found.kind == "ticket" && found.state == "archiv" {
        return Ok(format!("{id}: archiviert, nicht geprüft"));
    }
    let (text, _) = read_text(root, &found.rel)?;
    let waves = load_wave_data(root)?;
    let defects = defects(&waves, found.kind, id, &text);
    if defects.is_empty() {
        return Ok(format!("{id}: keine Mängel in den Planungsangaben"));
    }
    Ok(format!("{id}: {} Mängel\n- {}", defects.len(), defects.join("\n- ")))
}

/// Mängel eines Tickets oder einer Sprint-README.
fn defects(waves: &WaveData, kind: &str, id: &str, text: &str) -> Vec<String> {
    let spec = parse_plan_spec(text);
    let mut out = Vec::new();
    if !spec.has_lines {
        out.push("Planungsangaben fehlen im Abschnitt „Regeln und Einschränkungen“".to_owned());
    }
    for name in PLAN_LINES {
        if spec.has_lines && !spec.seen.contains(name) && (name != "Größe" || kind == "ticket") {
            out.push(format!("Zeile „{name}“ fehlt"));
        }
    }
    if spec.seen.contains("Bereich") && spec.area.is_empty() {
        out.push("Bereich leer".to_owned());
    }
    out.extend(env_defects(waves, kind, id, text));
    if spec.size.to_lowercase() == "l" {
        out.push("Größe: L (vor der Freigabe in Tickets je Teil zerlegen)".to_owned());
    }
    for dep in &spec.deps {
        if waves.dep_target(dep).unknown {
            out.push(format!("unbekannte ID {dep} in „Abhängig von“"));
        }
    }
    for bad in &spec.bad_deps {
        out.push(format!("keine ID in „Abhängig von“: {bad}"));
    }
    out.extend(overlaps(waves, kind, id, &spec.area));
    out
}

/// `Umgebung: ?` beim Ticket, bei einem Sprint in jeder offenen Session.
fn env_defects(waves: &WaveData, kind: &str, id: &str, text: &str) -> Vec<String> {
    if kind == "ticket" {
        let fields = head_fields(&text.replace("\r\n", "\n"));
        let unknown = fields.get("Umgebung").is_some_and(|v| v.starts_with('?'));
        return unknown.then(|| "Umgebung: ?".to_owned()).into_iter().collect();
    }
    waves.sprint(id).map_or_else(Vec::new, |sprint| {
        sprint
            .sessions
            .iter()
            .filter(|x| x.env.starts_with('?') && x.status != "fertig" && x.status != "verworfen")
            .map(|x| format!("Umgebung: ? in {}", x.nr))
            .collect()
    })
}

/// Bereich überlappt mit anderen offenen Tickets (beim Sprint ohne seine eigenen Tickets).
fn overlaps(waves: &WaveData, kind: &str, id: &str, area: &[String]) -> Vec<String> {
    let mut own = vec![id];
    if kind == "sprint"
        && let Some(sprint) = waves.sprint(id)
    {
        own.extend(sprint.tickets.iter().map(String::as_str));
    }
    waves
        .data
        .tickets
        .iter()
        .filter(|t| t.status == "offen" && !own.contains(&t.nr.as_str()))
        .filter_map(|t| {
            let other = waves.specs.get(&t.nr)?;
            let pair = area_overlap(area, &other.area)?;
            Some(format!("Bereich überlappt mit {} ({pair})", t.nr))
        })
        .collect()
}
