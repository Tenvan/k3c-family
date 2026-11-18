//! Tests zu `plantation.rs` (Go: `building_plantation_test.go`, W2.1, B-114/AC-03, AC-04, Sprint W2 AC-02): Plantage an
//! beiden Baumschule-Plätzen, Werte aus data/buildings.json. Die Tests über `step` und `create_world` warten auf RU18.

use super::plantation::{plot_x, step_plantations};
use super::testutil::*;
use crate::state::{ResourceNode, World};
use crate::world::lines::site_needs_clearing;
use crate::world::vals;

/// Go: `farms`: die Indizes der Baumschule-Plätze.
fn farms(w: &World) -> Vec<usize> {
    (0..w.sites.len()).filter(|&i| w.sites[i].kind == "nursery").collect()
}

/// Go: `plantedAt` zählt die Plantage-Bäume im Streifen der Baumschule.
fn planted_at(w: &World, nursery: usize) -> i64 {
    let strip = vals(w).buildings["nursery"].plantation.strip_units;
    let nx = w.sites[nursery].x;
    w.nodes
        .iter()
        .filter(|n| n.kind == "tree" && n.marked && (n.x - nx).abs() <= strip)
        .count() as i64
}

#[test]
fn plantage_waechst_je_platz() {
    let mut w = quiet_world();
    w.cycle_speed = 1e-6;
    let p = vals(&w).buildings["nursery"].plantation.clone();
    let fs = farms(&w);
    assert_eq!(fs.len(), 2, "{} Baumschule-Plätze, erwartet 2 (je Seite)", fs.len());
    let level = w.nodes.len();
    run(&mut w, p.grow_seconds + 1.0);
    assert_eq!(w.nodes.len(), level, "Bäume ohne gebaute Baumschule");
    for &f in &fs {
        built(&mut w.sites[f]);
    }
    run(&mut w, p.grow_seconds - 1.0);
    assert_eq!(planted_at(&w, fs[0]), 0, "Baum vor der Wachstumszeit");
    run(&mut w, 1.0 + DT);
    for &f in &fs {
        assert_eq!(
            planted_at(&w, f),
            p.slots,
            "Baumschule {}: nach {} s",
            w.sites[f].x,
            p.grow_seconds
        );
    }
    run(&mut w, 10.0 * p.grow_seconds);
    assert_eq!(
        (w.nodes.len() - level) as i64,
        2 * p.slots,
        "Plantage-Bäume nach langer Zeit"
    );
}

/// Plantage-Bäume sind kein Zahlziel, ein Bauer holt sie ohne Münze; der Platz wächst danach neu.
#[test]
fn plantage_ohne_markierung() {
    let mut w = day_world();
    let p = vals(&w).buildings["nursery"].plantation.clone();
    let nursery = farms(&w)[0];
    built(&mut w.sites[nursery]);
    w.nodes.clear(); // ohne Level-Bäume im Streifen
    run(&mut w, p.grow_seconds + DT);
    let pi = add_player(&mut w);
    w.players[pi].x = plot_x(&w.sites[nursery], &p, 0);
    let target = crate::economy::find_pay_target(&w, &vals(&w), None, &w.players[pi]);
    assert!(target.is_none(), "Plantage-Baum ist Zahlziel");
    let gold = w.players[pi].gold;
    run(&mut w, 300.0); // der Platz liegt weit draußen im Wald, der Bauer muss hinlaufen
    assert!(
        w.stock.wood != 0 && w.players[pi].gold == gold,
        "Holz {}, Gold {gold} → {}",
        w.stock.wood,
        w.players[pi].gold
    );
    assert!(planted_at(&w, nursery) != 0, "abgebaute Plätze wachsen nicht nach");
}

/// B-636 (AC-04): Eine Baumschule ist erst nach dem Roden bezahlbar; vorher meldet der Server „erst roden“.
#[test]
fn baumschule_erst_nach_dem_roden() {
    let mut w = day_world();
    w.hub_level = 1;
    let nursery = farms(&w)[0];
    let id = w.new_id();
    let nx = w.sites[nursery].x + 1.0;
    w.nodes = vec![ResourceNode {
        id,
        kind: "tree".into(),
        x: nx,
        ..ResourceNode::default()
    }];
    let v = vals(&w);
    let payable = |w: &World| crate::economy::site_payable(w, &v, None, &w.sites[nursery]);
    assert!(
        !payable(&w) && site_needs_clearing(&w, &w.sites[nursery]),
        "mit Baum im Freiraum"
    );
    w.nodes[0].gone = true;
    assert!(
        payable(&w) && !site_needs_clearing(&w, &w.sites[nursery]),
        "nach dem Roden"
    );
}

/// Eigener Test (ohne Go-Gegenstück): `plot_x` verteilt die Plätze gleichmäßig im Streifen, ein Platz liegt auf der Mitte.
#[test]
fn plot_x_verteilt_im_streifen() {
    let mut w = bare_world();
    let ni = built_site(&mut w, "nursery", 100.0);
    let s = &w.sites[ni];
    let mut p = vals(&w).buildings["nursery"].plantation.clone();
    p.slots = 5;
    p.strip_units = 8.0;
    let xs: Vec<f64> = (0..5).map(|i| plot_x(s, &p, i)).collect();
    assert_eq!(xs, vec![92.0, 96.0, 100.0, 104.0, 108.0]);
    p.slots = 1;
    assert_eq!(plot_x(s, &p, 0), 100.0);
}

/// Eigener Test (läuft ohne `create_world`): Bäume wachsen nur an gebauten Baumschulen, je Platz einer, abgebaute wachsen
/// nach `grow_seconds` neu.
#[test]
fn plantage_waechst_und_waechst_nach() {
    let mut w = bare_world();
    let p = vals(&w).buildings["nursery"].plantation.clone();
    let ni = built_site(&mut w, "nursery", 100.0);
    w.sites[ni].state = "unpaid".into();
    step_plantations(&mut w, p.grow_seconds + 1.0);
    assert!(w.nodes.is_empty(), "Bäume ohne gebaute Baumschule");
    w.sites[ni].state = "built".into();
    step_plantations(&mut w, 1.0); // legt die Plätze an
    assert!(w.nodes.is_empty(), "Baum vor der Wachstumszeit");
    step_plantations(&mut w, p.grow_seconds);
    assert_eq!(w.nodes.len() as i64, p.slots, "ein Baum je Platz");
    assert!(w.nodes.iter().all(|n| n.kind == "tree" && n.marked && n.plant));
    step_plantations(&mut w, 10.0 * p.grow_seconds);
    assert_eq!(w.nodes.len() as i64, p.slots, "höchstens ein Baum je Platz");
    // Ein abgebauter Baum (aus `nodes` entfernt) wächst nach der Wachstumszeit neu.
    w.nodes.remove(0);
    step_plantations(&mut w, 1.0);
    assert_eq!(w.nodes.len() as i64, p.slots - 1);
    step_plantations(&mut w, p.grow_seconds);
    assert_eq!(w.nodes.len() as i64, p.slots, "abgebauter Platz wächst nicht nach");
}
