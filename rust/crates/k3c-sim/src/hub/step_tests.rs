//! Tests zu `hub_step_test.go` (GEB17.4, B-592/AC-02): Ausbauschritte je Hub-Stufe, Material-Tor im letzten Schritt mit dem
//! Wartegrund „Material der nächsten Insel“, Schritt im Spielstand. Werte aus data/hub.json. Die Tests, die `step` rechnen,
//! warten auf RU18.4 und RU18.5 (`merchant_dawn`, `step_passives`, `speed_of`).

use k3c_data::buildings::Cost;
use k3c_data::hub::Stock;

use super::level::next_level;
use super::testutil::hub;
use crate::economy::view::{WAITING_NEXT_ISLAND, economy_of, material_of};
use crate::enemies::testutil::run;
use crate::island::testutil::{load_island, must_island, save_json, test_data};
use crate::state::World;
use crate::state::save::HubSave;
use crate::state::save_apply::apply_hub_level;
use crate::world::testutil::{day_world, pay_at, quiet_world, run_until};
use crate::world::{add_player, hub_of, vals};

/// Die Daten: Hub-Stufe 1 ohne Schritte, jede weitere drei; nur der letzte Schritt hat ein Tor, ab Stufe 5 keins mehr.
#[test]
fn hub_schritte_in_den_daten() {
    let h = hub();
    assert!(
        h.levels[0].steps.is_empty(),
        "Hub-Stufe 1 hat Schritte: {:?}",
        h.levels[0].steps
    );
    for n in 2..=h.levels.len() {
        let steps = &h.levels[n - 1].steps;
        assert_eq!(steps.len(), 3, "Stufe {n}: {} Schritte, erwartet 3", steps.len());
        for (i, st) in steps.iter().enumerate() {
            assert!(
                st.cost.gold > 0 && st.build_seconds > 0.0 && material_of(&st.cost) != Stock::default(),
                "Stufe {n} Schritt {} ohne Gold, Material oder Bauzeit: {st:?}",
                i + 1
            );
            let has_gate = st.gate != Cost::default();
            assert_eq!(
                has_gate,
                i == 2 && n < h.levels.len(),
                "Stufe {n} Schritt {}: Tor {:?}",
                i + 1,
                st.gate
            );
            let (g, c) = (&st.gate, &st.cost); // Tor nur aus Material, das der Schritt selbst kostet (B-631/AC-03)
            for (gate, cost) in [
                (g.wood, c.wood),
                (g.stone, c.stone),
                (g.copper, c.copper),
                (g.iron, c.iron),
                (g.crystal, c.crystal),
            ] {
                assert!(
                    !(gate > 0 && cost == 0),
                    "Stufe {n}: Tor {g:?} nicht aus dem Material der Stufe {c:?}"
                );
            }
        }
    }
}

/// Go: `payHubStep`: zahlt den laufenden Ausbauschritt mit zwei Spielern (je die Hälfte) an der Burg.
fn pay_hub_step(w: &mut World) {
    let hub_site = w.hub_site.clone().expect("Hub-Platz");
    let gold = next_level(w, None, &hub_site).expect("nächster Schritt").cost.gold;
    w.players[0].gold = gold / 2;
    w.players[1].gold = gold - gold / 2;
    let x = w.castle.x;
    let started = |w: &World| w.hub_site.as_ref().is_some_and(|h| !h.upgrade.is_empty());
    assert!(
        pay_at(w, &[0, 1], x, 30.0, started),
        "Schritt {} nicht bezahlt: {:?}",
        w.hub_step,
        w.hub_site
    );
}

fn hub_upgrade_of(w: &World) -> String {
    w.hub_site.as_ref().map(|h| h.upgrade.clone()).unwrap_or_default()
}

/// Schritt 3 von Hub-Stufe 1 wartet mit bezahltem Gold auf die 20 Stein des Tors (Material der eigenen neuen Stufe,
/// B-631/AC-03, kein Kupfer), meldet dann „Material der nächsten Insel“ und hebt mit dem Tor-Stein die Hub-Stufe.
#[test]
fn hub_material_tor() {
    let mut w = day_world();
    add_player(&mut w);
    add_player(&mut w);
    w.hub_step = 3;
    let st = hub().levels[1].steps[2].clone();
    assert_eq!(
        st.gate,
        Cost {
            stone: 20,
            ..Cost::default()
        },
        "Tor von Hub-Stufe 2"
    );
    w.stock = material_of(&st.cost); // Stein des Schritts da, Tor-Stein fehlt
    pay_hub_step(&mut w);
    run(&mut w, 3.0);
    let up = economy_of(&w, &vals(&w), None).hub_upgrade;
    let ok = up.as_ref().is_some_and(|u| {
        u.state == WAITING_NEXT_ISLAND && u.material.stone == st.cost.stone + st.gate.stone && u.material.copper == 0
    });
    assert!(
        hub_upgrade_of(&w) == "waitingMaterial" && ok,
        "ohne Tor: Platz {:?}, Zustand {up:?}",
        hub_upgrade_of(&w)
    );
    assert!(
        w.hub_level == 1 && economy_of(&w, &vals(&w), None).hub_step == 3,
        "ohne Tor ausgebaut"
    );
    w.stock.stone += st.gate.stone;
    let took = run_until(&mut w, 2.0 * st.build_seconds + 1.0, |w| w.hub_level == 2);
    assert!(took >= 0.0, "mit Tor nicht ausgebaut: {:?}", w.hub_site);
    assert!(
        w.hub_step == 1 && w.stock == Stock::default(),
        "nach dem Tor: Schritt {}, Vorrat {:?}",
        w.hub_step,
        w.stock
    );
}

/// Fehlt im Tor-Schritt mehr als der Tor-Anteil (der Stein des Schritts selbst), bleibt der Wartegrund waitingMaterial.
#[test]
fn hub_tor_schritt_ohne_eigenes_material() {
    let mut w = day_world();
    add_player(&mut w);
    add_player(&mut w);
    w.hub_step = 3;
    w.stock = Stock {
        stone: hub().levels[1].steps[2].gate.stone,
        ..Stock::default()
    };
    pay_hub_step(&mut w);
    run(&mut w, 1.0);
    let up = economy_of(&w, &vals(&w), None).hub_upgrade;
    assert!(
        up.as_ref().is_some_and(|u| u.state == "waitingMaterial"),
        "Tor da, Stein fehlt: {up:?}"
    );
}

/// Schritt 2 nach dem Bau von Schritt 1: Zustand und Spielstand nennen ihn, das Laden stellt ihn wieder her.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story: load_island lädt die Standard-Welt)"]
fn hub_schritt_im_spielstand() {
    let data = test_data();
    let mut isl = must_island(&data, "geb17-schritt", &[0, 1]);
    isl.stages[0].hub_step = 2;
    (isl.stages[1].hub_level, isl.stages[1].hub_step) = (2, 3);
    let loaded = load_island(&data, &save_json(&isl, "2026-10-25T00:00:00Z"));
    let got = &loaded.stages[0];
    let step0 = economy_of(got, &vals(got), None).hub_step;
    assert!(
        got.hub_level == 1 && got.hub_step == 2 && step0 == 2,
        "Stufe 0: Hub-Stufe {} Schritt {}",
        got.hub_level,
        got.hub_step
    );
    let got = &loaded.stages[1];
    assert!(
        got.hub_level == 2 && got.hub_step == 3,
        "Stufe 1: Hub-Stufe {} Schritt {}",
        got.hub_level,
        got.hub_step
    );
    let saved = isl.to_save("x").expect("Spielstand");
    assert_eq!(saved.stages[0].hub_step, 2, "Spielstand ohne hubStep");
    isl.stages[0].hub_step = 1;
    let saved = isl.to_save("x").expect("Spielstand");
    assert_eq!(saved.stages[0].hub_step, 0, "Schritt 1 steht im Spielstand");
}

/// Ein Schritt außerhalb der Stufe (z. B. aus einem anderen Datenstand) wird beim Laden auf die Schrittzahl begrenzt.
#[test]
fn hub_schritt_begrenzt_beim_laden() {
    let mut w = quiet_world();
    let save = |hub_level, hub_step| HubSave {
        hub_level,
        hub_step,
        ..HubSave::default()
    };
    apply_hub_level(&mut w, &save(1, 9));
    assert_eq!(w.hub_step, 3, "Schritt {}, erwartet 3", w.hub_step);
    let n = hub_of(&w).levels.len() as i64;
    apply_hub_level(&mut w, &save(n, 2));
    assert_eq!(w.hub_step, 1, "letzte Stufe: Schritt {}, erwartet 1", w.hub_step);
}
