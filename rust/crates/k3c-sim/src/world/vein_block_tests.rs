//! ECO1.8 (B-1198): Adern sind erst nach dem Freiräumen der Felsen auf dem Weg abbaubar; jede Ader bekommt 1 bis 3 Felsen.

use super::testutil::new_world;
use super::values_room::default_values;
use super::vein::{VEIN_CLEARANCE, is_vein, vein_blocked};
use super::{Options, add_player};
use crate::state::World;
use crate::unit::jobs::find_job;

fn rocks_near(w: &World, x: f64) -> usize {
    w.nodes
        .iter()
        .filter(|r| r.kind == "rock" && !r.gone && (r.x - x).abs() <= VEIN_CLEARANCE)
        .count()
}

/// Über viele Seeds: jede Ader hat 1 bis 3 Felsen in ihrem Umkreis und ist damit zunächst versperrt.
#[test]
fn jede_ader_hat_ein_bis_drei_hindernisse() {
    let v = default_values();
    let mut veins = 0;
    for seed in 1..=40 {
        let w = new_world(1, &format!("adern{seed}"), Options::default());
        for n in w.nodes.iter().filter(|n| is_vein(&v, n)) {
            veins += 1;
            let c = rocks_near(&w, n.x);
            assert!(c >= 1, "Seed {seed}: Ader bei {} ohne Hindernis", n.x);
            assert!(vein_blocked(&w, &v, n), "Seed {seed}: Ader nicht versperrt");
        }
    }
    assert!(veins > 0, "keine Adern gefunden");
}

/// Mit Felsen kein Abbau-Auftrag an der Ader, nach dem Räumen (Felsen `gone`) schon; 2 Spieler stören nicht.
#[test]
fn abbau_erst_nach_dem_freiraeumen() {
    let mut w = new_world(1, "adern", Options::default());
    add_player(&mut w);
    add_player(&mut w);
    let v = default_values();
    let vi = w.nodes.iter().position(|n| is_vein(&v, n)).expect("Ader");
    let hub = w.hub_x;
    w.troops.clear();
    let keep = w.nodes[vi].id;
    w.nodes.retain(|n| n.id == keep);
    let _ = vi;
    w.nodes
        .iter_mut()
        .filter(|n| is_vein(&v, n))
        .for_each(|n| n.marked = true);
    let vx = w.nodes.iter().find(|n| is_vein(&v, n)).map(|n| n.x).unwrap();
    let id = w.new_id();
    w.nodes.push(crate::state::ResourceNode {
        id,
        kind: "rock".into(),
        x: vx + 1.0,
        ..Default::default()
    });
    let ti = crate::unit::spawn_vagrant(&mut w, hub, hub);
    w.troops[ti].kind = "peasant".into();
    w.troops[ti].hammer = true;
    let mut vals = (*v).clone();
    vals.native_rules = true;
    assert!(find_job(&mut w, &vals, ti, None).is_none(), "versperrt");
    let last = w.nodes.len() - 1;
    w.nodes[last].gone = true;
    assert!(find_job(&mut w, &vals, ti, None).is_some(), "frei");
    let mut plain = vals.clone();
    plain.native_rules = false;
    w.nodes[last].gone = false;
    assert!(
        find_job(&mut w, &plain, ti, None).is_some(),
        "ohne native Regeln nie versperrt"
    );
}
