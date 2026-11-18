//! Gegner und Pools der tiefen Stufen Eisenstollen (Tiefe 3) und Kristallhöhle (Tiefe 4) (Go: `enemies_pools_test.go`,
//! K1.3, gegner.md §§ 1 bis 3).

use std::collections::BTreeMap;

use k3c_data::biome::{Biome, check_pool};
use k3c_data::{Source, load_all};
use k3c_rng::Rng;

use super::testutil::*;
use super::waves::plan_wave;

const DEEP_POOLS: [(i64, [&str; 3]); 2] = [
    (3, ["lavaSlime", "ironBeetle", "fireSpirit"]),
    (4, ["crystalSpider", "shardling", "crystalGuardian"]),
];

/// Go: `biomeForDepth`.
pub fn biome_for_depth(depth: i64) -> Biome {
    let d = load_all(&Source::Embedded).expect("eingebettete Daten");
    d.biomes
        .into_iter()
        .find(|b| b.depth == depth)
        .expect("Biom dieser Tiefe")
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_string()).collect()
}

/// (a) Beide Biome laden gültige Pools (2 Standard, 1 Elite, keine Nacht-Gegner); ohne Elite oder mit unbekanntem
/// Gegner scheitert das Laden.
#[test]
fn pool_tiefe_stufen_laden() {
    let v = test_values();
    for (depth, want) in DEEP_POOLS {
        let b = biome_for_depth(depth);
        let mut tiers: BTreeMap<&str, i64> = BTreeMap::new();
        for k in &b.enemies.portal {
            *tiers.entry(v.enemies[k].tier.as_str()).or_default() += 1;
        }
        assert!(check_pool(&b, &v.enemies).is_ok(), "Tiefe {depth}");
        assert_eq!(b.enemies.portal, strings(&want));
        assert!(b.enemies.night.is_empty());
        assert_eq!((tiers["standard"], tiers["elite"]), (2, 1), "Tiefe {depth}");
        let (mut no_elite, mut unknown) = (b.clone(), b.clone());
        no_elite.enemies.portal = strings(&want[..2]);
        unknown.enemies.night = strings(&["dragon"]);
        assert!(
            check_pool(&no_elite, &v.enemies).is_err(),
            "Tiefe {depth}: Pool ohne Elite geladen"
        );
        assert!(
            check_pool(&unknown, &v.enemies).is_err(),
            "Tiefe {depth}: unbekannter Gegner geladen"
        );
    }
}

/// (b) Welle 6 zieht nur Gegner aus dem Pool, darunter mindestens einen Elite.
#[test]
fn pool_welle_plant_aus_dem_pool() {
    let v = test_values();
    for (depth, pool) in DEEP_POOLS {
        let mut r = Rng::new("k1-pool");
        let plan = plan_wave(&v, &biome_for_depth(depth), 6, &mut r, &[10.0, 50.0, 90.0], false, 1.0);
        let mut elites = 0;
        for s in &plan {
            assert!(
                pool.contains(&s.kind.as_str()),
                "Tiefe {depth}: {} nicht im Pool",
                s.kind
            );
            if v.enemies[&s.kind].tier == "elite" {
                elites += 1;
            }
        }
        assert!(
            elites >= 1 && plan.len() >= 11,
            "Tiefe {depth}: {} Gegner, {elites} Elite",
            plan.len()
        );
    }
}

/// (c) Skalierung nach der Tabelle für Insel 1 (`depthScaling`, Grad-Faktor 1): HP, Schaden und Tempo je Tiefe.
#[test]
fn pool_skalierung_tiefe() {
    let v = test_values();
    let s = &v.waves.depth_scaling;
    for (depth, pool) in DEEP_POOLS {
        let mut w = bare_world();
        w.biome = biome_for_depth(depth);
        let f = depth as f64;
        for kind in pool {
            let d = &v.enemies[kind];
            let at = w.hub_x - 40.0;
            let ei = spawn(&mut w, kind, at);
            let e = &w.enemies[ei];
            let (hp, dmg) = ((d.hp * s.hp.powf(f)).round(), (d.damage * s.damage.powf(f)).round());
            let speed = d.speed * s.speed.powf(f);
            assert!(
                e.hp == hp && e.max_hp == hp && e.damage == dmg,
                "{kind} in Tiefe {depth}"
            );
            assert!(
                (e.speed - speed).abs() <= 1e-9 * speed.abs().max(1.0),
                "{kind}: Tempo {}",
                e.speed
            );
        }
    }
    // Stichprobe mit festen Zahlen: Lavaschleim in Tiefe 3 = 120 * 1,5^3 HP, 25 * 1,3^3 Schaden.
    let mut w = bare_world();
    w.biome = biome_for_depth(3);
    let ei = spawn(&mut w, "lavaSlime", 0.0);
    let e = &w.enemies[ei];
    assert!(
        e.hp == 405.0 && e.damage == 55.0,
        "Lavaschleim Tiefe 3: HP {}, Schaden {}",
        e.hp,
        e.damage
    );
}

/// (d) Ab Tiefe 3 drei Portale (Biom-Daten; die Portale der Welt baut `create_world`, RU14).
#[test]
fn pool_drei_portale_im_biom() {
    for (depth, _) in DEEP_POOLS {
        assert_eq!(biome_for_depth(depth).portals.count, 3, "Tiefe {depth}");
    }
}

#[test]
fn pool_drei_portale_in_der_welt() {
    for (depth, _) in DEEP_POOLS {
        let w = crate::world::testutil::new_world(depth, "k1-portale", crate::world::Options::default());
        assert_eq!(
            biome_for_depth(depth).portals.count,
            3,
            "Tiefe {depth}: Portale im Biom"
        );
        assert_eq!(w.portals.len(), 3, "Tiefe {depth}: Portale in der Welt");
    }
}

/// (e) Splitterwicht: ein Schwarm-Platz erzeugt 4 Gegner am selben Portal.
#[test]
fn gegner_splitterwicht_schwarm() {
    let v = test_values();
    let mut b = biome_for_depth(4);
    b.enemies.portal = strings(&["shardling"]);
    let mut r = Rng::new("splitter");
    let portals = [10.0, 50.0, 90.0];
    let plan = plan_wave(&v, &b, 1, &mut r, &portals, false, 1.0);
    assert!(
        v.enemies["shardling"].swarm_size == 4 && plan.len() >= 20 && plan.len().is_multiple_of(4),
        "{} Splitterwichte",
        plan.len()
    );
    for (i, s) in plan.iter().enumerate() {
        let want = portals[(i / 4) % 3];
        assert!(
            s.kind == "shardling" && s.x == want,
            "Gegner {i}: {} am Portal {}, erwartet {want}",
            s.kind,
            s.x
        );
    }
}

/// (e) Kristallwächter: Flächenschlag trifft beide Spieler im Radius 3, nicht den außen stehenden Krieger.
#[test]
#[ignore = "wartet auf RU19 (unit_x)"]
fn gegner_kristallwaechter_flaechenschlag() {
    let mut w = quiet_world();
    let (p1, p2) = (add_player(&mut w), add_player(&mut w));
    let x = w.hub_x - 30.0;
    spawn(&mut w, "crystalGuardian", x);
    w.players[p1].x = x + 2.0;
    w.players[p2].x = x + 4.5;
    probe(&mut w, &[p1, p2]);
    let out = warrior_at(&mut w, x + 6.0);
    super::step_enemies(&mut w, None, DT);
    let a = &test_values().enemies["crystalGuardian"].aoe;
    assert!(a.radius == 3.0 && a.interval_seconds == 4.0);
    assert!(was_hit(&w, p1) && was_hit(&w, p2) && w.troops[out].hp == w.troops[out].max_hp);
}

/// (e) Feuergeist: hält 8 Units Abstand zum Krieger und schießt.
#[test]
#[ignore = "wartet auf RU19 (moon_damage)"]
fn gegner_feuergeist_kiting() {
    let mut w = quiet_world();
    let x = w.hub_x - 30.0;
    let e = spawn(&mut w, "fireSpirit", x - 10.0);
    w.enemies[e].x = x;
    let warrior = warrior_at(&mut w, x + 5.0);
    step_for(&mut w, 3.0);
    let dist = w.troops[warrior].x - w.enemies[e].x;
    assert!(
        (8.0..=8.2).contains(&dist) && w.projectiles.len() >= 2,
        "Abstand {dist}, {} Geschosse",
        w.projectiles.len()
    );
}
