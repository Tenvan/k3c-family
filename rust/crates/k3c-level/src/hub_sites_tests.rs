use crate::lines::HubSite;
use crate::planned::{geometry_of, validate_plan_with};
use crate::planned_gen::Options;
use crate::testutil::{biome, generate_with, p, rules, with_ruin};
use crate::types::Layout;

// B-983: Der Generator rechnet mit den Plätzen der Welt (Options.HubSites); nil ist die Basis.

fn wall_near(l: &Layout, x: f64) -> bool {
    l.sites.iter().any(|s| (s.x - x).abs() < rules().pay_gap_units)
}

fn hub_site(kind: &str, offset_units: f64) -> HubSite {
    HubSite {
        kind: kind.into(),
        offset_units,
    }
}

/// Ein Platz der Welt hält Mauern und Türme auf Abstand: mit dem Platz weit draußen kommt keine Verteilung in seine
/// Nähe, in der Basis (kein Platz dort) geschieht das bei vielen Seeds.
#[test]
fn hub_sites_halten_zahlziele_auf_abstand() {
    let b = biome("forest");
    let world = vec![
        hub_site("workshop", -16.0),
        hub_site("smithy", 60.0),
        hub_site("stairsUp", 16.0),
    ];
    let own_options = Options {
        hub_sites: Some(world),
        ..Options::default()
    };
    let mut near = 0;
    for i in 0..200_u8 {
        let seed = format!("plaetze-{}{}", char::from(b'a' + i % 26), char::from(b'a' + i / 26));
        let base = generate_with(&b, &seed, &Options::default()).expect("Basis");
        let own = generate_with(&b, &seed, &own_options).expect("Welt");
        assert!(
            !wall_near(&own, own.hub_center_units + 60.0),
            "Seed {seed}: Mauer oder Turm näher als {} Units am Platz der Welt",
            rules().pay_gap_units
        );
        if wall_near(&base, base.hub_center_units + 60.0) {
            near += 1;
        }
    }
    assert!(near != 0, "kein Seed zeigt den Unterschied: der Test beweist nichts");
}

/// Ohne eigene Plätze (nil) und mit den Plätzen der Basis ist das Level gleich.
#[test]
fn hub_sites_basis_gleich_nil() {
    let b = biome("forest");
    let own: Vec<HubSite> = rules().hub_sites.iter().map(|s| hub_site(&s.kind, s.offset)).collect();
    let with_own = Options {
        hub_sites: Some(own),
        ..Options::default()
    };
    for seed in ["eins", "zwei", "drei"] {
        let a = generate_with(&b, seed, &Options::default()).expect("a");
        let c = generate_with(&b, seed, &with_own).expect("c");
        assert!(
            a.sites.len() == c.sites.len() && a.entities.len() == c.entities.len(),
            "Seed {seed}: Level weicht von der Basis ab"
        );
        for (i, (x, y)) in a.sites.iter().zip(&c.sites).enumerate() {
            assert_eq!(x, y, "Seed {seed}: Platz {i} weicht ab");
        }
    }
}

/// Die Prüfung der Planung nimmt die Plätze der Welt: Eine Ruine auf der Lage eines Platzes gleicher Art wird
/// abgelehnt.
#[test]
fn hub_sites_plan_pruefung_folgt_welt() {
    let b = biome("forest");
    let lane = geometry_of(&b, 0, rules().hub_sites.clone()).hub_half + 10.0;
    let at = [hub_site("smithy", lane)];
    let plan = [with_ruin(p(1, "forest", &[]), "smithy", 10.0)]; // Lage hub_half + 10 Units vom Hub
    if let Err(e) = validate_plan_with(&b, &plan, 0, None, rules()) {
        panic!("Basis: {e}");
    }
    let err = validate_plan_with(&b, &plan, 0, Some(&at), rules());
    let ok = err
        .as_ref()
        .is_err_and(|e| e.to_string().contains("Hub-Platzes gleicher Art"));
    assert!(
        ok,
        "Platz der Welt auf der Ruinen-Lage: Ablehnung erwartet, war {err:?}"
    );
    let o = Options {
        planned: plan.to_vec(),
        hub_sites: Some(at.to_vec()),
        ..Options::default()
    };
    assert!(
        generate_with(&b, "x", &o).is_err(),
        "generate_with nimmt die Plätze der Welt nicht für die Prüfung"
    );
}
