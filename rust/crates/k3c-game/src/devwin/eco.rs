//! Befehle des Bereichs Wirtschaft (RU56.7, B-1134): Gold, Ressourcen, Lager, Baukosten, Ausbau und Burgfall. Werte
//! werden geprüft, dann laufen sie über dieselbe Dev-Aktion wie im F9-Menü (Gruppen Vorrat und Ausbau).

use k3c_devproto::DevCommand;

use super::Host;
use super::commands::{Outcome, run_dev};

const RESOURCES: [&str; 5] = ["wood", "stone", "copper", "iron", "crystal"];
/// Größter Betrag eines freien Gold- oder Ressourcen-Befehls (gegen Tippfehler).
const MAX_AMOUNT: i64 = 1_000_000;

/// dev führt die Aktionen der Wirtschaft aus; `None`: nicht Sache dieses Bereichs.
pub fn dev(host: &mut dyn Host, player: i64, c: &DevCommand) -> Outcome {
    let bad = match c.action.as_str() {
        "gold" | "material" if c.amount == 0 || c.amount.abs() > MAX_AMOUNT => {
            Some(format!("Betrag {} nicht erlaubt (höchstens {MAX_AMOUNT})", c.amount))
        }
        "material" if !RESOURCES.contains(&c.resource.as_str()) => {
            Some(format!("Unbekannte Ressource „{}“", c.resource))
        }
        "buildAll" if !(1..=5).contains(&c.level) => Some(format!("Ausbaustufe {} gibt es nicht (1 bis 5)", c.level)),
        "gold" | "material" | "buildAll" | "maxStock" | "fillStock" | "cheapBuild" | "castleFall" => None,
        _ => return None,
    };
    Some(match bad {
        Some(e) => Err(e),
        None => run_dev(host, player, c),
    })
}

/// ui: die Wirtschaft hat keine Tasten.
pub fn ui(_host: &mut dyn Host, _key: &str, _value: Option<i64>) -> Outcome {
    None
}

#[cfg(test)]
#[path = "eco_tests.rs"]
mod eco_tests;
