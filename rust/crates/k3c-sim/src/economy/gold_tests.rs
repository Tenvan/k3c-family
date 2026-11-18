//! Tests zu `sites.rs`, `fountain.rs` und `dawn.rs`: Gold geben, Münzen streuen, Fontäne, Gegner-Gold, Morgengold.
//! `step_sites` braucht `site_cost` und `step_upgrade` (RU18.3): dieser Test trägt `ignore`.

use k3c_rng::Rng;

use super::{add_stock, dawn_gold, fountain_coins, give_gold, pay_dawn_income, scaled_gold, scatter_coins, step_sites};
use crate::enemies::testutil::bare_world;
use crate::island::storage::capacity;
use crate::state::island_ref::IslandRef;
use crate::state::shared::Crown;
use crate::state::{Player, World};
use crate::world::vals;

fn world_with_players(gold: &[i64]) -> World {
    let mut w = bare_world();
    w.width_units = 400.0;
    w.rng = Rng::new("gold-test");
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

#[test]
fn scatter_coins_wuerfelt_je_muenze_einmal_und_klemmt_an_die_welt() {
    let mut w = world_with_players(&[]);
    let mut probe = Rng::new("gold-test");
    let expected: Vec<f64> = (0..4)
        .map(|_| (1.0 + (probe.next() - 0.5) * 3.0).max(0.0).min(w.width_units))
        .collect();
    let first = w.next_id;
    scatter_coins(&mut w, 1.0, 4);
    assert_eq!(w.coins.iter().map(|c| c.x).collect::<Vec<_>>(), expected);
    assert_eq!(
        w.coins.iter().map(|c| c.id).collect::<Vec<_>>(),
        (first..first + 4).collect::<Vec<_>>()
    );
    assert!(w.coins.iter().all(|c| (0.0..=400.0).contains(&c.x)));
    scatter_coins(&mut w, 1.0, 0);
    scatter_coins(&mut w, 1.0, -3);
    assert_eq!(w.coins.len(), 4, "keine Münze bei 0 oder weniger");
}

#[test]
fn give_gold_fuellt_den_beutel_und_streut_den_rest() {
    let mut w = world_with_players(&[0, 0]);
    let max = vals(&w).economy.purse.max_gold;
    w.players[0].gold = max - 2;
    give_gold(&mut w, None, 0, 5, 100.0);
    assert_eq!(w.players[0].gold, max);
    assert_eq!(w.coins.len(), 3, "3 Münzen passen nicht mehr");
    w.players[1].gold = max + 4; // nach Respec über der Grenze: nichts abziehen
    give_gold(&mut w, None, 1, 2, 100.0);
    assert_eq!(w.players[1].gold, max + 4);
    assert_eq!(w.coins.len(), 5);
}

#[test]
fn fountain_coins_staffelt_start_und_flug_aus_dem_rng() {
    let mut w = world_with_players(&[]);
    w.time = 10.0;
    let v = vals(&w);
    let f = v.economy.chest_fountain.clone();
    let mut probe = Rng::new("gold-test");
    let mut expected = Vec::new();
    for i in 0..3 {
        let to = (50.0 + (probe.next() * 2.0 - 1.0) * f.spread_units).clamp(0.0, 400.0);
        let launch = 10.0 + i as f64 * f.stagger_seconds;
        let flight = f.flight_seconds * (0.8 + 0.4 * probe.next());
        expected.push((to, launch, launch + flight));
    }
    fountain_coins(&mut w, &v, 50.0, 3);
    let got: Vec<_> = w.coins.iter().map(|c| (c.x, c.launch_at, c.lands_at)).collect();
    assert_eq!(got, expected);
    assert!(w.coins.iter().all(|c| c.from_x == 50.0));
}

#[test]
fn scaled_gold_rundet_und_laesst_faktor_null_unveraendert() {
    let mut w = world_with_players(&[]);
    let f = vals(&w).economy.enemy_gold_factor;
    let expected = |g: i64| if f <= 0.0 { g } else { (g as f64 * f).round() as i64 };
    for g in [0, 1, 3, 7, 10, 25] {
        assert_eq!(scaled_gold(&w, g), expected(g), "Gold {g}, Faktor {f}");
    }
    let mut v = (*vals(&w)).clone();
    v.economy.enemy_gold_factor = 0.0;
    w.vals = Some(std::sync::Arc::new(v));
    assert_eq!(scaled_gold(&w, 7), 7, "Faktor 0 = unverändert");
    let mut v = (*vals(&w)).clone();
    v.economy.enemy_gold_factor = 1.5;
    w.vals = Some(std::sync::Arc::new(v));
    assert_eq!(scaled_gold(&w, 3), 5, "4,5 rundet auf 5 (math.Round)");
}

#[test]
fn dawn_gold_waechst_mit_der_tiefe_und_geht_nur_an_lebende_monarchen() {
    let mut w = world_with_players(&[0, 0]);
    let v = vals(&w);
    w.biome.depth = 2;
    let e = &v.economy;
    assert_eq!(dawn_gold(&w, &v), e.dawn_gold_per_player + 2 * e.dawn_gold_per_depth);
    w.players[1].crown = Some(Crown::default());
    pay_dawn_income(&mut w, &v, None);
    assert_eq!(w.players[0].gold, dawn_gold(&w, &v).min(e.purse.max_gold));
    assert_eq!(w.players[1].gold, 0, "ohne Krone kein Morgengold");
}

#[test]
fn pay_dawn_income_streut_den_ueberschuss_bei_jedem_spieler() {
    let mut w = world_with_players(&[0, 0]);
    let v = vals(&w);
    let max = v.economy.purse.max_gold;
    w.players[0].gold = max;
    w.players[1].gold = max;
    pay_dawn_income(&mut w, &v, None);
    assert_eq!(
        w.coins.len() as i64,
        2 * dawn_gold(&w, &v),
        "voller Beutel: alles fällt als Münzen"
    );
}

#[test]
fn add_stock_nur_fuer_baumaterial_und_bis_zur_kapazitaet() {
    let mut w = world_with_players(&[]);
    let v = vals(&w);
    assert!(add_stock(&mut w, &v, None, "wood", 5));
    assert!(add_stock(&mut w, &v, None, "iron", 2));
    assert_eq!((w.stock.wood, w.stock.iron), (5, 2));
    assert!(!add_stock(&mut w, &v, None, "gold", 5), "kein Baumaterial");
    let isl = IslandRef {
        stages: 1,
        ..IslandRef::default()
    };
    let cap = capacity(&w, &v, Some(&isl)).expect("Kapazität in der Insel");
    assert!(add_stock(&mut w, &v, Some(&isl), "stone", cap + 10));
    assert_eq!(w.stock.stone, cap, "höchstens bis zur Kapazität");
}

#[test]
fn step_sites_bezahlter_platz_zieht_material_und_wartet_auf_den_bauern() {
    let mut w = world_with_players(&[]);
    let si = crate::enemies::testutil::built_site(&mut w, "workshop", 100.0);
    w.sites[si].state = "waitingMaterial".into();
    w.stock.wood = 1000;
    w.stock.stone = 1000;
    step_sites(&mut w, None);
    assert_eq!(w.sites[si].state, "waitingWorker");
}
