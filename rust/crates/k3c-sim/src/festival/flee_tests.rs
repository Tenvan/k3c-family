//! Lebende Gegner beim Sieg (RU62.12, AC-06): Sie fliehen, schaden nicht, rauben keine Krone, lassen Beute fallen und
//! verschwinden am Portal oder am Inselrand. Gegenprobe: ohne Frieden bleiben sie Gegner.
use super::flee::exit_x;
use super::testutil::{count, enemies_total, festival_data, festival_island, put_enemy, run, tick};
use crate::island::Island;
use crate::island::testutil::test_data;
use crate::state::shared::{CROWN_ENEMY, CROWN_GROUND, Crown};
use crate::state::{Event, World};

/// Die Art des Gegners der Tests (kommt in data/enemies.json vor).
const KIND: &str = "goblin";

/// Schaden an Burg und Monarchen sowie Treffer auf Ziele außer Gegnern in den Ereignissen.
fn harm(events: &[Event]) -> usize {
    events
        .iter()
        .filter(|e| e["type"] == "hit" && e["target"] != "enemy")
        .count()
}

/// Gold aller Spieler und HP der Burgen der Insel.
fn assets(isl: &Island) -> (i64, Vec<f64>) {
    let gold = isl.stages.iter().flat_map(|w| &w.players).map(|p| p.gold).sum();
    (gold, isl.stages.iter().map(|w| w.castle.hp).collect())
}

/// AC-06: Gegner in beiden Stufen, einer dicht an der Burg: Nach dem Sieg entfernt sich jeder von ihr und vom Spieler, nichts
/// wird getroffen, Burg und Gold bleiben, und nach kurzer Zeit sind alle am Ausgang verschwunden.
#[test]
fn lebende_gegner_fliehen_und_schaden_nicht() {
    let data = festival_data();
    let (_, mut isl) = festival_island(&data, 1);
    run(&mut isl, &data, 1.0);
    let hub0 = isl.stages[0].hub_x;
    let hub1 = isl.stages[1].hub_x;
    put_enemy(&mut isl, 0, KIND, hub0 + 1.5);
    put_enemy(&mut isl, 1, KIND, hub1 - 2.0);
    isl.endboss_defeated = true;
    let before = assets(&isl);
    let mut far = [0.0f64; 2];
    let mut ev_all = Vec::new();
    for _ in 0..(120.0 * 30.0) as usize {
        ev_all.extend(tick(&mut isl, &data));
        for (si, hub) in [hub0, hub1].into_iter().enumerate() {
            for e in &isl.stages[si].enemies {
                assert!(e.fleeing, "Gegner flieht");
                let d = (e.x - hub).abs();
                assert!(d + 1e-9 >= far[si] - 3.0, "Gegner kehrt um: Abstand {d}");
                far[si] = far[si].max(d);
            }
        }
        if enemies_total(&isl) == 0 {
            break;
        }
    }
    assert_eq!(enemies_total(&isl), 0, "alle sind verschwunden");
    assert_eq!(harm(&ev_all), 0, "kein Schaden an Burg, Monarch, Truppen oder Gebäuden");
    let after = assets(&isl);
    assert_eq!(after.1, before.1, "Burgen unberührt");
    assert!(
        after.0 >= before.0,
        "kein Gold verloren (Beute der Truppen ist erlaubt)"
    );
    assert!(far[0] > 1.5 && far[1] > 2.0, "sie sind weggelaufen: {far:?}");
    assert!(!isl.over);
}

/// AC-06: Gegenprobe: In einer Welt ohne Fest bleibt der Gegner nach dem Sieg ein Gegner: er flieht nicht, die Stufe ist nicht
/// friedlich.
#[test]
fn ohne_frieden_bleiben_gegner_gegner() {
    let data = test_data();
    let (_, mut isl) = festival_island(&data, 1);
    run(&mut isl, &data, 1.0);
    let hub0 = isl.stages[0].hub_x;
    put_enemy(&mut isl, 0, KIND, hub0 + 30.0);
    isl.endboss_defeated = true;
    run(&mut isl, &data, 0.2);
    assert!(isl.stages.iter().all(|w| !w.peace && w.festival.is_none()));
    assert_eq!(enemies_total(&isl), 1);
    assert!(
        isl.stages[0].enemies.iter().all(|e| !e.fleeing),
        "der Gegner greift weiter an"
    );
}

/// AC-06: Ein Gegner mit Krone lässt sie vor der Flucht fallen: kein `crownStolen`, kein Game Over, die Krone liegt am Boden
/// bei ihrem Besitzer; geraubtes Gold fällt als Münzen. Gegenprobe: ohne Frieden ist der Kronenraub das Ende.
#[test]
fn getragene_krone_faellt_vor_der_flucht() {
    for (data, peace) in [(festival_data(), true), (test_data(), false)] {
        let (_, mut isl) = festival_island(&data, 1);
        run(&mut isl, &data, 1.0);
        let hub = isl.stages[0].hub_x;
        let w = &mut isl.stages[0];
        let ei = put_enemy_in(w, hub + 6.0);
        let owner = w.players[0].index;
        let crown = Crown {
            owner,
            state: CROWN_ENEMY.to_string(),
            x: hub + 6.0,
            carrier: w.enemies[ei].id,
            ..Crown::default()
        };
        w.players[0].crown = Some(crown.clone());
        w.enemies[ei].crown = Some(crown);
        w.enemies[ei].carried_gold = 3;
        w.enemies[ei].fleeing = true; // wie nach dem Raub
        let coins = w.coins.len();
        isl.endboss_defeated = true;
        let mut ev = tick(&mut isl, &data);
        if peace {
            let w = &isl.stages[0];
            assert_eq!(w.enemies[0].crown, None, "Gegner trägt keine Krone mehr");
            assert_eq!(w.enemies[0].carried_gold, 0);
            assert_eq!(w.coins.len(), coins + 3, "geraubtes Gold fällt als Münzen");
            let c = w.players[0].crown.as_ref().expect("Krone");
            assert!(c.state == CROWN_GROUND && c.carrier == 0, "Krone liegt am Boden: {c:?}");
        }
        ev.extend(run(&mut isl, &data, 60.0));
        if peace {
            assert_eq!(count(&ev, "crownStolen"), 0);
            assert!(!isl.over, "kein Game Over nach dem Sieg");
            assert_eq!(enemies_total(&isl), 0);
        } else {
            assert_eq!(count(&ev, "crownStolen"), 1, "Kronenraub ohne Frieden");
            assert!(isl.over);
        }
    }
}

/// Ein Gegner der Tests bei `x` in der Stufe `w`; liefert den Index.
fn put_enemy_in(w: &mut World, x: f64) -> usize {
    crate::enemies::spawn_enemy(w, KIND, x, None)
}

/// AC-06: Ohne Portal fliehen Gegner zum nächsten Inselrand und verschwinden dort.
#[test]
fn ohne_portal_zum_inselrand() {
    let data = festival_data();
    let (_, mut isl) = festival_island(&data, 1);
    run(&mut isl, &data, 1.0);
    let w = &mut isl.stages[0];
    w.portals.clear();
    w.portal_hp.clear();
    let width = w.width_units;
    assert_eq!(exit_x(w, 3.0), 0.0);
    assert_eq!(exit_x(w, width - 3.0), width);
    let hub = w.hub_x;
    put_enemy(&mut isl, 0, KIND, hub - 5.0);
    isl.endboss_defeated = true;
    let mut min_x = f64::MAX;
    for _ in 0..(300.0 * 30.0) as usize {
        tick(&mut isl, &data);
        if let Some(e) = isl.stages[0].enemies.first() {
            min_x = min_x.min(e.x);
        }
        if enemies_total(&isl) == 0 {
            break;
        }
    }
    assert_eq!(enemies_total(&isl), 0, "am Rand verschwunden");
    assert!(min_x < hub - 5.0, "er lief nach links zum Rand: {min_x}");
}

/// Mit Portalen ist das nächste stehende Portal das Ziel, der Rand zählt dann nicht.
#[test]
fn ausgang_ist_das_naechste_portal() {
    let data = festival_data();
    let (_, mut isl) = festival_island(&data, 1);
    let w = &mut isl.stages[0];
    w.portals = vec![30.0, 90.0];
    assert_eq!(exit_x(w, 40.0), 30.0);
    assert_eq!(exit_x(w, 80.0), 90.0);
    assert_eq!(exit_x(w, 5.0), 30.0);
}
