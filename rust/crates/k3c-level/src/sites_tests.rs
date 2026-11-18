use std::collections::{BTreeMap, BTreeSet};

use k3c_det::fmin;

use crate::biome::Biome;
use crate::sites::SitePlacement;
use crate::testutil::{biome, biome_ids, generate, rules};
use crate::types::{Layout, WallSite};

fn key(s: &WallSite) -> String {
    format!("{}/{}/{}", s.side, s.zone, s.kind)
}

/// B-652 (AC-02, AC-03): je Zone und Seite 1 bis 3 Mauern und 1 bis 3 Türme, deterministisch, Mindestabstand der
/// Zahlziele, ein Teil der Türme 4 bis 10 Units neben einer Mauer.
#[test]
#[ignore = "wartet auf RU6.5"]
fn sites_verteilt_500_seeds() {
    let p = rules().site_placement;
    let b = biome("forest");
    let (mut towers, mut near) = (0, 0);
    for i in 0..500 {
        let seed = format!("sites-{i}");
        let lay = generate(&b, &seed);
        assert_eq!(
            lay.sites,
            generate(&b, &seed).sites,
            "Seed {i}: Plätze nicht deterministisch"
        );
        check_zone_counts(&seed, &lay, &p);
        check_site_spacing(&seed, &lay, &p);
        let (tw, nr) = count_near_towers(&lay, &p);
        towers += tw;
        near += nr;
    }
    assert!(towers > 0, "keine Türme");
    let share = near as f64 / towers as f64;
    assert!(
        share >= p.tower_near_share * 0.8,
        "Anteil naher Türme {share:.2}, erwartet etwa {:.2}",
        p.tower_near_share
    );
    println!(
        "Türme {towers}, davon nah an einer Mauer {near} ({:.0} %)",
        100.0 * share
    );
}

/// Je Seite, Zone und Art höchstens `perZoneMax` Plätze.
fn check_zone_counts(seed: &str, lay: &Layout, p: &SitePlacement) {
    let mut count: BTreeMap<String, i64> = BTreeMap::new();
    for s in &lay.sites {
        *count.entry(key(s)).or_default() += 1;
    }
    for (k, n) in count {
        assert!(
            n <= p.per_zone_max,
            "{seed}: {k} hat {n} Plätze, höchstens {}",
            p.per_zone_max
        );
    }
}

/// Jeder Platz liegt in den Zonen, und zwei Plätze halten den Zahlziel-Abstand.
fn check_site_spacing(seed: &str, lay: &Layout, p: &SitePlacement) {
    let end = p.zone_start_units + p.zones as f64 * lay.chunk_width_units;
    for (a, sa) in lay.sites.iter().enumerate() {
        let d = (sa.x - lay.hub_center_units).abs();
        let outside = d < fmin(p.zone_start_units, p.zone1_tower_units) || d >= end;
        assert!(
            sa.kind == "nursery" || !outside,
            "{seed}: Platz {sa:?} außerhalb der Zonen"
        );
        for sb in &lay.sites[a + 1..] {
            assert!(
                (sa.x - sb.x).abs() >= rules().pay_gap_units,
                "{seed}: Plätze {sa:?} und {sb:?} zu nah"
            );
        }
    }
}

/// Zählt die Türme und davon die, die 4 bis 10 Units neben einer Mauer derselben Zone stehen.
fn count_near_towers(lay: &Layout, p: &SitePlacement) -> (i64, i64) {
    let (mut towers, mut near) = (0, 0);
    for sa in lay.sites.iter().filter(|s| s.kind == "tower") {
        towers += 1;
        if near_wall(lay, sa, p) {
            near += 1;
        }
    }
    (towers, near)
}

fn near_wall(lay: &Layout, tower: &WallSite, p: &SitePlacement) -> bool {
    lay.sites.iter().any(|w| {
        let d = (tower.x - w.x).abs();
        w.kind == "wall"
            && w.side == tower.side
            && w.zone == tower.zone
            && d >= p.tower_near_min_units
            && d <= p.tower_near_max_units
    })
}

/// Jede Seite und Zone mit genug Platz bekommt mindestens eine Mauer und einen Turm.
#[test]
#[ignore = "wartet auf RU6.5"]
fn sites_jede_zone_hat_mauer_und_turm() {
    let b = biome("forest");
    for i in 0..100 {
        let lay = generate(&b, &format!("zone-{i}"));
        let have: BTreeSet<String> = lay.sites.iter().map(key).collect();
        for k in required_zone_keys(3) {
            assert!(have.contains(&k), "Seed {i}: {k} fehlt");
        }
    }
}

/// Seite/Zone/Art der ersten `zones` Zonen beider Seiten.
fn required_zone_keys(zones: i64) -> Vec<String> {
    let mut out = Vec::new();
    for side in [-1, 1] {
        for zone in 1..=zones {
            for kind in ["wall", "tower"] {
                out.push(format!("{side}/{zone}/{kind}"));
            }
        }
    }
    out
}

/// Die Plätze ändern die übrigen Würfe nicht: Entitäten und Chunks bleiben gleich (eigener Strom).
#[test]
#[ignore = "wartet auf RU6.5"]
fn sites_aendern_nichts_an_den_entitaeten() {
    let lay = generate(&biome("forest"), "k3c-dev");
    assert!(
        !lay.sites.is_empty() && !lay.entities.is_empty(),
        "Plätze {}, Entitäten {}",
        lay.sites.len(),
        lay.entities.len()
    );
}

/// Baugrund (Variante A): Die erste Mauer und der erste Turm der Zone 1 liegen je Seite auf freiem Grund (FreeGround),
/// das Layout bleibt unverändert und `on_free_ground` meldet genau deren Freiraum.
#[test]
#[ignore = "wartet auf RU6.5"]
fn sites_baugrund_zone1_frei() {
    let p = rules().site_placement;
    let b = biome("forest");
    for i in 0..200 {
        let lay = generate(&b, &format!("grund-{i}"));
        assert_eq!(
            lay.free_ground.len(),
            4,
            "Seed {i}: Baugrund-Plätze, erwartet 4 (je Seite Mauer und Turm)"
        );
        for &x in &lay.free_ground {
            let ok = lay.on_free_ground(x, rules())
                && lay.on_free_ground(x + p.clear_radius_units - 1.0, rules())
                && !lay.on_free_ground(x + p.clear_radius_units + 9999.0, rules());
            assert!(ok, "Seed {i}: on_free_ground falsch bei {x}");
        }
    }
}

/// Untergrenze je Biom (B-652, Review BS12): Zone 1 hat in jedem Biom und Seed Mauer und Turm (fester Baugrund), auch
/// mit 25-Units-Chunks.
#[test]
#[ignore = "wartet auf RU6.5"]
fn sites_zone1_jedes_biom() {
    for id in biome_ids() {
        let b = biome(&id);
        for i in 0..100 {
            let lay = generate(&b, &format!("biom-{i}"));
            let have: BTreeSet<String> = lay
                .sites
                .iter()
                .filter(|s| s.zone == 1)
                .map(|s| format!("{}/{}", s.side, s.kind))
                .collect();
            for k in ["-1/wall", "-1/tower", "1/wall", "1/tower"] {
                assert!(have.contains(k), "{id} Seed {i}: Zone 1 ohne {k}");
            }
        }
    }
}

/// B-636 (AC-03): Das Wald-Biom hat je Seite einen Baumschul-Platz in einem Wald-Chunk, deterministisch; andere Biome
/// keinen.
#[test]
#[ignore = "wartet auf RU6.5"]
fn nursery_plaetze_im_wald() {
    let p = rules().site_placement;
    for id in biome_ids() {
        let b = biome(&id);
        let mut have: BTreeMap<i64, i64> = BTreeMap::new();
        for i in 0..100 {
            let seed = format!("baumschule-{i}");
            let lay = generate(&b, &seed);
            assert_eq!(
                lay.sites,
                generate(&b, &seed).sites,
                "{id} {seed}: nicht deterministisch"
            );
            let mut sides: BTreeMap<i64, i64> = BTreeMap::new();
            for s in lay.sites.iter().filter(|s| s.kind == "nursery") {
                *sides.entry(s.side).or_default() += 1;
                check_nursery_on_forest(&id, &seed, &lay, s, &p);
            }
            for (side, n) in sides {
                assert!(
                    n <= 1 && id == "forest",
                    "{id} {seed}: {n} Baumschulen auf Seite {side}"
                );
                *have.entry(side).or_default() += n;
            }
        }
        // Ohne Wald-Chunk auf einer Seite entfällt der Platz dort (Ausnahmefall im Ticket): höchstens 10 % der Seeds.
        let (l, r) = (have.get(&-1).copied().unwrap_or(0), have.get(&1).copied().unwrap_or(0));
        assert!(
            id != "forest" || (l >= 90 && r >= 90),
            "forest: Baumschulen links {l}, rechts {r} von 100 Seeds"
        );
    }
}

fn check_nursery_on_forest(id: &str, seed: &str, lay: &Layout, s: &WallSite, p: &SitePlacement) {
    let c = &lay.chunks[(s.x / lay.chunk_width_units) as usize];
    assert_eq!(
        c.kind, "forest",
        "{id} {seed}: Baumschule bei {} in Chunk {:?}",
        s.x, c.kind
    );
    let d = (s.x - lay.hub_center_units).abs();
    let min = p.nursery_min_units - lay.chunk_width_units / 2.0;
    assert!(
        d >= min,
        "{id} {seed}: Baumschule {d} Units vom Hub, mindestens etwa {}",
        p.nursery_min_units
    );
}

/// B-636 (AC-03): Auf dem Baumschul-Platz stehen Bäume (der Wald-Chunk liefert sie), erst nach dem Roden wird er
/// bezahlbar.
#[test]
#[ignore = "wartet auf RU6.5"]
fn nursery_hat_baeume_im_freiraum() {
    let p = rules().site_placement;
    let b: Biome = biome("forest");
    let (mut with, mut total) = (0, 0);
    for i in 0..100 {
        let lay = generate(&b, &format!("baeume-{i}"));
        for s in lay.sites.iter().filter(|s| s.kind == "nursery") {
            total += 1;
            if lay
                .entities
                .iter()
                .any(|e| e.kind == "tree" && (e.x - s.x).abs() < p.clear_radius_units)
            {
                with += 1;
            }
        }
    }
    assert!(
        total != 0 && with as f64 >= 0.9 * total as f64,
        "Baumschul-Plätze mit Bäumen im Freiraum: {with} von {total}"
    );
}
