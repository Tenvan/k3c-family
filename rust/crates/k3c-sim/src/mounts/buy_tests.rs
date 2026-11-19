//! Kauf am Gatter (RU62.8, AC-04): Abzug, Katalogwerte, abgelehnt bei zu wenig Münzen oder schon gekauft, zwei Monarchen
//! unabhängig, Stärke nur beim Käufer, später Beitritt.

use super::mount_factors;
use super::testutil::{count, gate, mounts_data, mounts_island, stand_at_gate, tick};
use crate::island::add_island_player;
use crate::state::PlayerCommand;
use crate::world::vals;

#[test]
fn kauf_zieht_den_preis_ab_und_gibt_katalogwerte() {
    let data = mounts_data();
    let (_, mut isl, _) = mounts_island(&data, 1);
    stand_at_gate(&mut isl, 0);
    let v = vals(&isl.stages[0]);
    assert_eq!(
        mount_factors(&isl.stages[0], &v, 0),
        (1.0, 1.0),
        "vor dem Kauf: Standard-Pferd"
    );
    let ev = tick(&mut isl, &data, &[0]);
    let p = &isl.stages[0].players[0];
    assert_eq!((p.gold, p.mount.as_str()), (70, "camel"));
    assert_eq!(count(&ev, "mountBought"), 1);
    let (speed, sprint) = mount_factors(&isl.stages[0], &v, 0);
    let camel = data.mounts.entry("camel").expect("Kamel");
    assert_eq!((speed, sprint), (camel.speed_factor, camel.sprint_factor));
    assert!(sprint > 1.0, "die Stärke: schneller im Sprint");
    assert!(
        isl.stages[0].coins.is_empty(),
        "am Gatter fällt keine Münze auf den Boden"
    );
}

#[test]
fn zu_wenig_muenzen_kein_abzug_einmal_abgelehnt() {
    let data = mounts_data();
    let (_, mut isl, _) = mounts_island(&data, 1);
    stand_at_gate(&mut isl, 0);
    isl.stages[0].players[0].gold = 29;
    let mut rejected = 0;
    for _ in 0..5 {
        rejected += count(&tick(&mut isl, &data, &[0]), "mountRejected");
    }
    let p = &isl.stages[0].players[0];
    assert_eq!((p.gold, p.mount.as_str()), (29, ""), "nichts abgezogen, nichts gekauft");
    assert_eq!(rejected, 1, "gehaltene Taste: ein Ereignis je Druck");
    // loslassen und wieder drücken: wieder eine Ablehnung
    tick(&mut isl, &data, &[]);
    assert_eq!(count(&tick(&mut isl, &data, &[0]), "mountRejected"), 1);
}

#[test]
fn schon_gekauft_wird_abgelehnt() {
    let data = mounts_data();
    let (_, mut isl, _) = mounts_island(&data, 1);
    stand_at_gate(&mut isl, 0);
    tick(&mut isl, &data, &[0]);
    tick(&mut isl, &data, &[]);
    let ev = tick(&mut isl, &data, &[0]);
    let p = &isl.stages[0].players[0];
    assert_eq!((p.gold, p.mount.as_str()), (70, "camel"), "kein zweiter Abzug");
    assert_eq!(count(&ev, "mountRejected"), 1);
    assert_eq!(count(&ev, "mountBought"), 0);
}

#[test]
fn ausserhalb_der_reichweite_kein_kauf() {
    let data = mounts_data();
    let (_, mut isl, _) = mounts_island(&data, 1);
    let g = gate(&isl);
    isl.stages[0].players[0].x = g.x + g.range + 1.0;
    let ev = tick(&mut isl, &data, &[0]);
    assert_eq!(count(&ev, "mountBought") + count(&ev, "mountRejected"), 0);
    assert_eq!(isl.stages[0].players[0].mount, "");
}

#[test]
fn zwei_monarchen_unabhaengig_staerke_nur_beim_kaeufer() {
    let data = mounts_data();
    let (_, mut isl, _) = mounts_island(&data, 2);
    stand_at_gate(&mut isl, 0);
    stand_at_gate(&mut isl, 1);
    isl.stages[0].players[1].gold = 10; // der zweite kann sich nichts leisten
    let ev = tick(&mut isl, &data, &[0, 1]);
    assert_eq!((count(&ev, "mountBought"), count(&ev, "mountRejected")), (1, 1));
    let w = &isl.stages[0];
    assert_eq!(
        (w.players[0].mount.as_str(), w.players[1].mount.as_str()),
        ("camel", "")
    );
    let v = vals(w);
    assert!(
        mount_factors(w, &v, 0).1 > mount_factors(w, &v, 1).1,
        "Stärke nur beim Käufer"
    );
    // das Gatter bleibt offen: der zweite kauft später
    isl.stages[0].players[1].gold = 50;
    tick(&mut isl, &data, &[]);
    tick(&mut isl, &data, &[1]);
    let w = &isl.stages[0];
    assert_eq!((w.players[1].mount.as_str(), w.players[1].gold), ("camel", 20));
    assert_eq!(w.players[0].gold, 70, "der erste zahlt nicht noch einmal");
}

#[test]
fn gleicher_takt_beide_kaufen_nach_spielerindex() {
    let data = mounts_data();
    let (_, mut isl, _) = mounts_island(&data, 2);
    stand_at_gate(&mut isl, 0);
    stand_at_gate(&mut isl, 1);
    let ev = tick(&mut isl, &data, &[0, 1]);
    let order: Vec<i64> = ev
        .iter()
        .filter(|e| e.get("type").and_then(|t| t.as_str()) == Some("mountBought"))
        .filter_map(|e| e.get("player").and_then(serde_json::Value::as_i64))
        .collect();
    assert_eq!(order, vec![0, 1]);
    let w = &isl.stages[0];
    assert!(w.players.iter().all(|p| p.mount == "camel" && p.gold == 70));
}

#[test]
fn spaeter_beitritt_startet_mit_dem_standard_pferd() {
    let data = mounts_data();
    let (_, mut isl, _) = mounts_island(&data, 1);
    stand_at_gate(&mut isl, 0);
    tick(&mut isl, &data, &[0]);
    let late = add_island_player(&mut isl, 0);
    let w = &isl.stages[0];
    assert_eq!(w.players[late].mount, "");
    let v = vals(w);
    assert_eq!(mount_factors(w, &v, late), (1.0, 1.0));
    // das Gatter der aktuellen Insel steht auch ihm offen
    stand_at_gate(&mut isl, late);
    isl.stages[0].players[late].gold = 40;
    let cmds = [
        PlayerCommand::default(),
        PlayerCommand {
            pay: true,
            ..PlayerCommand::default()
        },
    ];
    crate::island::step_island(&mut isl, &cmds, &data.difficulty, crate::island::testutil::DT);
    assert_eq!(isl.stages[0].players[late].mount, "camel");
}

#[test]
fn sprint_und_tempo_wirken_beim_kaeufer() {
    let data = mounts_data();
    let (_, mut isl, _) = mounts_island(&data, 2);
    isl.stages[0].players[0].mount = "camel".to_string();
    let w = &isl.stages[0];
    let v = vals(w);
    let walk = crate::monarch::speed_of(w, &v, 0);
    let walk_std = crate::monarch::speed_of(w, &v, 1);
    assert_eq!(walk, walk_std, "Tempo: Kamel speedFactor 1.0 wie das Pferd");
    let camel = data.mounts.entry("camel").expect("Kamel");
    assert_eq!(mount_factors(w, &v, 0).1, camel.sprint_factor);
}

/// Review RU62.8: Wer die Taste gehalten in die Reichweite läuft, kauft nicht; erst ein neuer Druck kauft.
#[test]
fn gehaltene_taste_beim_hineinlaufen_kauft_nicht() {
    let data = mounts_data();
    let (_, mut isl, _) = mounts_island(&data, 1);
    let g = gate(&isl);
    isl.stages[0].players[0].x = g.x + g.range + 5.0;
    tick(&mut isl, &data, &[0]); // Taste gehalten, noch draußen
    stand_at_gate(&mut isl, 0);
    let ev = tick(&mut isl, &data, &[0]);
    assert_eq!(count(&ev, "mountBought"), 0);
    assert_eq!(isl.stages[0].players[0].mount, "");
    tick(&mut isl, &data, &[]);
    tick(&mut isl, &data, &[0]);
    assert_eq!(isl.stages[0].players[0].mount, "camel");
}
