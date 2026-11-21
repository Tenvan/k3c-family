//! RU36.3 (B-1158): Dev-Aktion „Schild +1“ legt Schilde aufs Gestell, erst ab der Hub-Stufe der Schilde, höchstens bis
//! zum vollen Stapel.

use super::testutil::island_with_player;
use super::{dev_add_shield, dev_build_all};
use crate::state::links::values;

#[test]
fn schild_plus_nur_freigeschaltet_und_bis_zum_stapel() {
    let (_, mut isl, index) = island_with_player("dev-schild");
    let (min, max) = {
        let v = values(&isl.stages[0]);
        (v.economy.combat.shield_min_hub_level, v.economy.combat.shield_stack_max)
    };
    if min > 1 {
        assert!(!dev_add_shield(&mut isl, index), "Schild vor der Freischaltung");
        assert_eq!(isl.stages[0].combat.shields, 0);
    }
    assert!(dev_build_all(&mut isl, min.max(1)));
    for n in 1..=max {
        assert!(dev_add_shield(&mut isl, index), "Schild {n}");
    }
    assert_eq!(isl.stages[0].combat.shields, max);
    assert!(!dev_add_shield(&mut isl, index), "Gestell voll");
    assert!(!dev_add_shield(&mut isl, index + 99), "unbekannter Spieler");
}
