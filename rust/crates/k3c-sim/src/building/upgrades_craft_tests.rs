//! Eigene Tests zu `upgrades.rs` (ohne Go-Gegenstück, laufen ohne `create_world`): Elite-Upgrade der Schmiede und
//! Elite-Angebote.

use k3c_data::hub::Stock;

use super::testutil::*;
use super::upgrades::{elite_offer_at, offer_at, step_crafts};
use super::upgrades_tests::elite_spot;
use crate::world::vals;

/// Eigener Test: Die Schmiede stellt den Elite-Bogenschützen nach `craft_seconds` her; ohne Ausgangsfigur wartet das
/// fertige Upgrade, danach wertet der nächste Bogenschütze zur Schmiede auf.
#[test]
fn schmiede_elite_wartet_auf_ausgangsfigur() {
    let mut w = bare_world();
    let v = vals(&w);
    let hub = w.hub_x;
    let si = built_site(&mut w, "smithy", hub + 60.0);
    let (cost, from) = (
        v.troops["eliteArcher"].cost.clone(),
        v.troops["eliteArcher"].upgrade_from.clone(),
    );
    let secs = v.buildings["smithy"]
        .craft_seconds
        .as_ref()
        .map_or(0.0, |m| m["eliteArcher"]);
    w.sites[si].bow_paid_gold = cost.gold;
    w.stock = Stock {
        wood: 1000,
        stone: 1000,
        copper: 1000,
        iron: 1000,
        crystal: 1000,
    };
    step_crafts(&mut w, &v, None);
    assert!(w.sites[si].craft_done > 0.0 && (w.sites[si].craft_done - secs).abs() < 1e-9);
    assert_eq!(1000 - w.stock.stone, cost.stone, "Material abgebucht");
    w.time = secs;
    step_crafts(&mut w, &v, None);
    assert!(
        w.sites[si].craft_done > 0.0,
        "ohne Ausgangsfigur bleibt das fertige Upgrade liegen"
    );
    let far = fighter_at(&mut w, &from, hub - 60.0);
    let near = fighter_at(&mut w, &from, hub + 90.0);
    step_crafts(&mut w, &v, None);
    assert_eq!(w.troops[near].kind, "eliteArcher", "der nächste wird Elite");
    assert_eq!(w.troops[far].kind, from, "der fernere nicht");
    assert_eq!(w.troops[near].hp, v.troops["eliteArcher"].hp);
    assert_eq!(w.troops[near].max_hp, v.troops["eliteArcher"].hp);
    assert_eq!(w.sites[si].craft_done, 0.0);
}

/// Eigener Test: Elite-Angebote (`eliteWarrior` der Schmiede) sind nur am gebauten Platz in Zahl-Reichweite und nur
/// bezahlbar, solange der Preis nicht voll ist; `offer_at` bevorzugt das Berufs-Angebot.
#[test]
fn elite_angebot_nur_gebaut_in_reichweite_und_unbezahlt() {
    let mut w = bare_world();
    let v = vals(&w);
    let hub = w.hub_x;
    let si = built_site(&mut w, "smithy", hub + 60.0);
    let offers = v.buildings["smithy"].offers.clone();
    w.sites[si].offer_paid = vec![0; offers.len()];
    let (spot, idx) = elite_spot(&w, si);
    let o = elite_offer_at(&w, &v, None, spot, true).expect("Elite-Angebot am Zahlziel");
    assert_eq!((o.site, o.kind()), (si, "eliteWarrior"));
    assert_eq!(o.idx, idx);
    assert!(offer_at(&w, &v, None, spot).is_some());
    assert!(
        elite_offer_at(&w, &v, None, spot + 1000.0, true).is_none(),
        "außer Reichweite"
    );
    w.sites[si].offer_paid[idx] = v.troops["eliteWarrior"].cost.gold;
    assert!(elite_offer_at(&w, &v, None, spot, true).is_none(), "voll bezahlt");
    assert!(
        elite_offer_at(&w, &v, None, spot, false).is_some(),
        "ohne Bezahlbarkeit zählt es weiter"
    );
    w.sites[si].state = "unpaid".into();
    assert!(elite_offer_at(&w, &v, None, spot, false).is_none(), "ungebaut");
}
