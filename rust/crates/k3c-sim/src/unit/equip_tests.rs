//! EQ1.3 (B-1206): Ausrüstungswerte (Rüstungspreis, Regalgrößen, Herstellzeiten) aus `data/buildings.json`.

use super::hammer::*;
use crate::enemies::testutil::*;

/// EQ1.3 (B-1206) AC-01 und AC-02: Rüstungspreis, Regal und Schwert-Regal kommen aus den Daten; ohne Feld gelten die
/// bisherigen Zahlen.
#[test]
fn ausruestungswerte_kommen_aus_den_daten() {
    let w = quiet_world();
    let v = w.vals.as_ref().expect("Werte");
    let armory = &v.buildings["armory"];
    assert_eq!(
        (armory.armor_cost.iron, armory.armor_cost.gold, armory.armor_rack),
        (30, 8, 3)
    );
    assert_eq!(
        v.buildings["smithy"]
            .craft_seconds
            .as_ref()
            .and_then(|m| m.get("sword")),
        Some(&10.0)
    );

    let mut changed = (**v).clone();
    let a = changed.buildings.get_mut("armory").expect("armory");
    a.armor_cost.iron = 40;
    a.armor_rack = 5;
    assert_eq!(armor_cost(&changed).iron, 40, "Wert aus den Daten wirkt");
    assert_eq!(armor_rack(&changed), 5);

    let a = changed.buildings.get_mut("armory").expect("armory");
    a.armor_cost = Default::default();
    a.armor_rack = 0;
    assert_eq!(
        (armor_cost(&changed).iron, armor_cost(&changed).gold),
        (30, 8),
        "ohne Feld: alter Preis"
    );
    assert_eq!(armor_rack(&changed), ARMOR_RACK);
    changed.buildings.get_mut("workshop").expect("workshop").sword_rack = 0;
    assert_eq!(sword_rack(&changed, "workshop"), SWORD_RACK);
}
