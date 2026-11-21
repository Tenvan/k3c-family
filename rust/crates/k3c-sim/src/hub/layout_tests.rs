//! LA3.1 (B-1182): Die nach Gebäudebreite gespreizten Hub-Plätze (`k3c_data::load_game`) bestehen die Platz-Regeln der
//! Sim (`validate_hub_data`: Zahlziel-Abstand, Treppen, Hub-Breite, Zone 1).

use k3c_data::{Source, load_game};

use super::validate::validate_hub_data;

#[test]
fn gespreizte_plaetze_bestehen_die_platz_regeln() {
    let d = load_game(&Source::Embedded).expect("Daten");
    let r = validate_hub_data(&d.hub);
    let lines: Vec<String> = r
        .findings
        .iter()
        .map(|f| format!("{} {}: {}", f.rule, f.site, f.detail))
        .collect();
    assert!(lines.is_empty(), "Befunde:\n{}", lines.join("\n"));
}
