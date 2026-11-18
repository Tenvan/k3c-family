//! Tests zu `players.rs`: Einsammeln von Münzen und Truhen, `to_hub`. `step_players` und `move_player` brauchen `speed_of`,
//! `mount_of` (RU18.5), `step_attack` (RU18.4) und `step_skills` (RU18.5); Münz-Aufnahme meldet über `emit` (RU19): diese
//! Tests tragen `ignore`.

use super::players::{collect_coins, collect_pickups, to_hub};
use crate::enemies::testutil::bare_world;
use crate::state::shared::Crown;
use crate::state::{Coin, Pickup, Player, World};
use crate::world::vals;

fn world_with_players(gold: &[i64]) -> World {
    let mut w = bare_world();
    w.width_units = 400.0;
    for (index, g) in gold.iter().enumerate() {
        let id = w.new_id();
        w.players.push(Player {
            id,
            index: index as i64,
            x: 100.0,
            gold: *g,
            ..Player::default()
        });
    }
    w
}

fn coin(w: &mut World, x: f64) -> usize {
    let id = w.new_id();
    w.coins.push(Coin {
        id,
        x,
        ..Coin::default()
    });
    w.coins.len() - 1
}

#[test]
fn to_hub_stellt_gerade_links_ungerade_rechts_neben_die_burg() {
    let mut w = world_with_players(&[0, 0]);
    w.players[0].vx = 5.0;
    w.players[1].vx = -5.0;
    to_hub(&mut w, 0);
    to_hub(&mut w, 1);
    assert_eq!((w.players[0].x, w.players[0].vx), (w.hub_x - 3.0, 0.0));
    assert_eq!((w.players[1].x, w.players[1].vx), (w.hub_x + 3.0, 0.0));
}

#[test]
fn collect_coins_laesst_unerreichbare_muenzen_liegen() {
    let mut w = world_with_players(&[0, 0]);
    let v = vals(&w);
    let reach = v.economy.pickup_range_units;
    let far = coin(&mut w, 100.0 + reach + 1.0);
    let air = coin(&mut w, 100.0);
    w.coins[air].lands_at = w.time + 1.0; // noch in der Luft (Fontäne)
    let blocked = coin(&mut w, 100.0);
    w.coins[blocked].blocked_player_id = Some(w.players[0].id);
    w.coins[blocked].blocked_until = w.time + 5.0;
    w.players[1].crown = Some(Crown::default()); // ohne Krone: nimmt nichts
    w.players[0].gold = v.economy.purse.max_gold; // Beutel voll
    let _ = far;
    collect_coins(&mut w, &v, None);
    assert_eq!(w.coins.len(), 3, "alle Münzen bleiben");
    assert_eq!(w.players[0].gold, v.economy.purse.max_gold);
    assert!(w.events.is_empty());
}

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x für coinPickup)"]
fn collect_coins_gibt_dem_ersten_spieler_in_reichweite_und_zaehlt_die_insel() {
    let mut w = world_with_players(&[0, 0]);
    let v = vals(&w);
    coin(&mut w, 100.0);
    coin(&mut w, 100.0);
    w.coins[1].blocked_player_id = Some(w.players[0].id); // gesperrt für Spieler 0: Spieler 1 bekommt sie
    w.coins[1].blocked_until = w.time + 5.0;
    let mut isl = crate::state::island_ref::IslandRef::default();
    collect_coins(&mut w, &v, Some(&mut isl));
    assert!(w.coins.is_empty());
    assert_eq!((w.players[0].gold, w.players[1].gold), (1, 1));
    assert_eq!(isl.gold_collected, 2);
    assert_eq!(w.events.len(), 2);
}

#[test]
fn collect_pickups_laesst_ferne_truhen_liegen() {
    let mut w = world_with_players(&[0]);
    let v = vals(&w);
    let id = w.new_id();
    let far = 100.0 + v.economy.pickup_range_units * 1.5 + 1.0;
    w.pickups.push(Pickup {
        id,
        kind: "chest".into(),
        x: far,
    });
    w.players[0].crown = Some(Crown::default());
    let id2 = w.new_id();
    w.pickups.push(Pickup {
        id: id2,
        kind: "chest".into(),
        x: 100.0,
    });
    collect_pickups(&mut w, &v, None); // die eine in Reichweite gehört niemandem ohne Krone
    assert_eq!(w.pickups.len(), 2);
    assert!(w.coins.is_empty() && w.events.is_empty());
}

#[test]
fn collect_pickups_truhe_wirft_gold_als_fontaene() {
    let mut w = world_with_players(&[0]);
    let v = vals(&w);
    let id = w.new_id();
    w.pickups.push(Pickup {
        id,
        kind: "chest".into(),
        x: 100.0,
    });
    collect_pickups(&mut w, &v, None);
    assert!(w.pickups.is_empty());
    let [lo, hi] = v.economy.chest_gold;
    let n = w.coins.len() as i64;
    assert!((lo..=hi).contains(&n), "{n} Münzen, erwartet {lo} bis {hi}");
    assert!(w.coins.iter().all(|c| c.lands_at > c.launch_at && c.from_x == 100.0));
    assert_eq!(w.events[0]["type"], "chest");
}

#[test]
fn collect_pickups_skill_punkt_geht_in_den_pool() {
    let mut w = world_with_players(&[0]);
    let v = vals(&w);
    let id = w.new_id();
    w.pickups.push(Pickup {
        id,
        kind: "skill".into(),
        x: 100.0,
    });
    collect_pickups(&mut w, &v, None);
    assert!(w.pickups.is_empty());
    assert_eq!(w.skill_points, v.monarch.skill_point_sources.hidden);
    assert_eq!(w.events[0]["type"], "skillPoint");
}
