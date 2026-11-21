//! HB1.4 (B-1187): Lager zwischen 2 bis 3 Bäumen, Zerstörung bei Rodung, Taverne am Platz, höchstens 5 Landstreicher.

use super::camp_guard::{GUARD_MAX, plant_camp_trees, step_native_rules};
use crate::state::{Camp, ResourceNode, World};
use crate::world::testutil::quiet_world;
use crate::world::vals;

const CAMP_X: f64 = 40.0;

fn tree(w: &mut World, x: f64) -> i64 {
    let id = w.new_id();
    w.nodes.push(ResourceNode {
        id,
        kind: "tree".into(),
        x,
        ..ResourceNode::default()
    });
    id
}

fn forest_world() -> World {
    let mut w = quiet_world();
    w.biome.primary_resource = "wood".into();
    w.nodes.clear();
    w.camps.push(Camp {
        x: CAMP_X,
        respawn_in: 25.0,
        ..Camp::default()
    });
    w
}

fn trees_near(w: &World) -> usize {
    w.nodes.iter().filter(|n| n.kind == "tree" && (n.x - CAMP_X).abs() <= 4.0).count()
}

#[test]
fn lager_ohne_baeume_bekommt_zwei_bis_drei() {
    let mut w = forest_world();
    let v = vals(&w);
    plant_camp_trees(&mut w, &v);
    let n = trees_near(&w);
    assert!((2..=GUARD_MAX).contains(&n), "{n} Bäume am Lager");
}

#[test]
fn lager_faellt_mit_den_letzten_baeumen_und_taverne_ist_baubar() {
    let mut w = forest_world();
    let ids: Vec<i64> = (0..5).map(|i| tree(&mut w, CAMP_X + f64::from(i) - 2.0)).collect();
    let v = vals(&w);
    step_native_rules(&mut w, &v);
    assert_eq!(w.camps[0].guards.len(), GUARD_MAX, "nur die nächsten drei bewachen");
    let mine = w.camps[0].guards.clone();
    w.nodes.retain(|n| !mine.contains(&n.id) || n.id == mine[0]);
    let v = vals(&w);
    step_native_rules(&mut w, &v);
    assert_eq!(w.camps.len(), 1, "ein Wächter steht noch");
    w.nodes.retain(|n| n.id != mine[0]);
    assert!(w.nodes.iter().any(|n| ids.contains(&n.id)), "fremde Bäume stehen weiter");
    let v = vals(&w);
    step_native_rules(&mut w, &v);
    assert!(w.camps.is_empty(), "Lager zerstört");
    let site = w.sites.iter().find(|s| s.kind == "tavern" && (s.x - CAMP_X).abs() < 0.5);
    assert_eq!(site.map(|s| s.state.as_str()), Some("unpaid"), "Taverne baubar");
}

#[test]
fn unter_tage_bleibt_das_lager() {
    let mut w = forest_world();
    w.biome.primary_resource = "stone".into();
    let v = vals(&w);
    step_native_rules(&mut w, &v);
    assert_eq!(w.camps.len(), 1);
}

#[test]
fn ohne_schalter_bleibt_alles_wie_bei_go() {
    let mut w = forest_world();
    let mut v = (*vals(&w)).clone();
    v.native_rules = false;
    plant_camp_trees(&mut w, &v);
    step_native_rules(&mut w, &v);
    assert!(w.nodes.is_empty() && w.camps.len() == 1);
}
