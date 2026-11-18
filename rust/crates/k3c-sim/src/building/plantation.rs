//! Plantage (Port von `engine/sim/building_plantation.go`, B-114, Q25, Q51): Eine gebaute Baumschule lässt auf jedem ihrer
//! Plätze (data/buildings.json › nursery.plantation) `grow_seconds` nach dem Leerwerden einen Baum wachsen, gleichmäßig im
//! Streifen ±`strip_units` um die Baumschule. Plantage-Bäume brauchen keine Münze: Sie entstehen markiert und sind damit
//! kein Zahlziel. Sie haben keine Level-Position; der Spielstand speichert sie nicht, nach dem Laden wachsen sie neu.
//! ponytail: Eine zerstörte und neu gebaute Baumschule vergisst ihre alten Bäume (bis zu 6 weitere), Grenze bei Bedarf über
//! die Bäume im Streifen zählen.

use k3c_data::buildings::Plantation;

use crate::state::{Plot, ResourceNode, Site, World};
use crate::world::values_room::values;

/// Go: `plotX` ist die Position von Platz `i` im Streifen um die Baumschule.
pub fn plot_x(s: &Site, p: &Plantation, i: usize) -> f64 {
    if p.slots < 2 {
        return s.x;
    }
    s.x - p.strip_units + (i as f64 * 2.0 * p.strip_units) / (p.slots - 1) as f64
}

/// Go: `stepPlantations` lässt auf den Plätzen gebauter Baumschulen Bäume nachwachsen.
pub fn step_plantations(w: &mut World, dt: f64) {
    let v = values(w);
    for si in 0..w.sites.len() {
        if w.sites[si].state != "built" {
            continue;
        }
        let Some(p) = v.buildings.get(&w.sites[si].kind).map(|b| &b.plantation) else {
            continue;
        };
        if p.slots == 0 {
            continue;
        }
        if w.sites[si].plots.is_empty() {
            w.sites[si].plots = vec![
                Plot {
                    tree: None,
                    grow_in: p.grow_seconds
                };
                p.slots as usize
            ];
        }
        for i in 0..w.sites[si].plots.len() {
            grow_plot(w, si, p, i, dt);
        }
    }
}

/// Go: `growPlot`. Der Baum eines Platzes ist seine ID; fehlt der Knoten oder ist er `gone`, ist der Baum abgebaut.
fn grow_plot(w: &mut World, si: usize, p: &Plantation, i: usize, dt: f64) {
    if let Some(id) = w.sites[si].plots[i].tree
        && !w.node(id).is_some_and(|n| !n.gone)
    {
        w.sites[si].plots[i] = Plot {
            tree: None,
            grow_in: p.grow_seconds,
        };
    }
    if w.sites[si].plots[i].tree.is_some() {
        return;
    }
    w.sites[si].plots[i].grow_in -= dt;
    if w.sites[si].plots[i].grow_in <= 0.0 {
        let id = w.new_id();
        let x = plot_x(&w.sites[si], p, i);
        w.nodes.push(ResourceNode {
            id,
            kind: "tree".into(),
            x,
            marked: true,
            plant: true,
            ..ResourceNode::default()
        });
        w.sites[si].plots[i].tree = Some(id);
    }
}
