use super::*;
use crate::source::Source;

fn all_sites(h: &HubData) -> impl Iterator<Item = &HubSite> {
    h.sites.iter().chain(&h.island_sites)
}

#[test]
fn kein_paar_benachbarter_plaetze_ueberlappt() {
    let d = crate::load_game(&Source::Embedded).expect("Daten");
    let w = crate::sizes::load(&Source::Embedded.read(crate::sizes::FILE).unwrap())
        .unwrap()
        .sites;
    for side in [-1.0_f64, 1.0] {
        let mut v: Vec<&HubSite> = all_sites(&d.hub).filter(|s| s.offset_units * side > 0.0).collect();
        v.sort_by(|a, b| a.offset_units.abs().total_cmp(&b.offset_units.abs()));
        let mut edge = w.castle / 2.0;
        for s in v.into_iter().filter(|s| width_of(&w, &s.kind) > 0.0) {
            let half = width_of(&w, &s.kind) / 2.0;
            let left = s.offset_units.abs() - half;
            assert!(
                left + 1e-9 >= edge + w.gap,
                "{} bei {} überlappt (links {left}, Rand {edge})",
                s.kind,
                s.offset_units
            );
            edge = s.offset_units.abs() + half;
        }
    }
}

#[test]
fn turm_und_mauer_stehen_hinter_den_gebaeuden() {
    let d = crate::load_game(&Source::Embedded).expect("Daten");
    let w = crate::sizes::load(&Source::Embedded.read(crate::sizes::FILE).unwrap())
        .unwrap()
        .sites;
    let outer = all_sites(&d.hub)
        .filter(|s| width_of(&w, &s.kind) > 0.0)
        .map(|s| s.offset_units.abs() + width_of(&w, &s.kind) / 2.0)
        .fold(0.0, f64::max);
    let p = &d.hub.site_placement;
    assert!(
        p.zone1_tower_units - 1.9 >= outer + w.gap - 1e-9,
        "Turm {} liegt in den Gebäuden (bis {outer})",
        p.zone1_tower_units
    );
    assert!(p.zone1_wall_units - p.zone1_tower_units >= w.pay_gap - 1e-9);
}

#[test]
fn rohe_hub_json_bleibt_wie_fuer_go() {
    let raw = crate::load_all(&Source::Embedded).expect("Daten").hub;
    let game = crate::load_game(&Source::Embedded).expect("Daten").hub;
    assert_ne!(raw.sites, game.sites, "die Plätze müssen sich verschieben");
    assert_eq!(raw.sites.len(), game.sites.len());
    for (a, b) in raw.sites.iter().zip(&game.sites) {
        assert_eq!(a.kind, b.kind);
        assert!(
            b.offset_units.abs() >= a.offset_units.abs() - 1e-9,
            "{} rückt nur nach außen",
            a.kind
        );
        assert_eq!(a.offset_units.signum(), b.offset_units.signum());
    }
}

#[test]
fn zu_breite_gebaeude_sind_ein_fehler() {
    let mut h = crate::load_all(&Source::Embedded).expect("Daten").hub;
    let mut w = SiteWidths {
        castle: 15.0,
        gap: 1.5,
        ..SiteWidths::default()
    };
    w.widths.insert("armory".into(), 120.0);
    assert!(spread(&mut h, &w, 50.0).is_err());
}

#[test]
fn spread_ist_stabil_bei_zweitem_lauf() {
    let mut h = crate::load_game(&Source::Embedded).expect("Daten").hub;
    let w = crate::sizes::load(&Source::Embedded.read(crate::sizes::FILE).unwrap())
        .unwrap()
        .sites;
    let before = h.clone();
    spread(&mut h, &w, 50.0).unwrap();
    assert_eq!(before, h);
}
