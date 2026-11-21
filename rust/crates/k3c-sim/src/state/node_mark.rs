//! Markieren von Ressourcen-Knoten (Port von `engine/sim/node_mark.go`).

use std::collections::BTreeMap;

use super::links::{gather_of, values};
use super::save::key;
use super::{Player, ResourceNode, World};

/// Go: `nodeMarkable`: Nimmt der Knoten gerade eine Markier-Münze? Sammelbar (Baum, Fels, Erz, Ader) und noch nicht markiert.
pub fn node_markable(w: &World, n: &ResourceNode) -> bool {
    let v = values(w);
    let ok = gather_of(&v, &n.kind).is_some();
    !n.marked && ok && crate::world::stands::tree_markable(w, &v, n) // WD1.2: Rodungsregel der Waldstücke
}

/// Go: `markNode`: markiert einen Knoten und merkt den Spieler der letzten Markier-Münze.
pub fn mark_node(n: &mut ResourceNode, p: &Player) {
    n.marked = true;
    n.marked_by = Some(p.index);
}

/// Go: `markedByOf`: den Markierer je markiertem Knoten (`kind@x`) für den Spielstand; leer, wenn keiner bekannt ist.
pub fn marked_by_of(w: &World) -> BTreeMap<String, i64> {
    let mut out = BTreeMap::new();
    for n in &w.nodes {
        if let (true, Some(by)) = (n.marked, n.marked_by) {
            out.insert(key(&n.kind, n.x), by);
        }
    }
    out
}

/// Go: `applyMarkedBy`: setzt den gespeicherten Markierer eines markierten Knotens; fehlt er, bleibt er unbekannt.
pub fn apply_marked_by(n: &mut ResourceNode, saved: &BTreeMap<String, i64>) {
    if let Some(by) = saved.get(&key(&n.kind, n.x)).filter(|_| n.marked) {
        n.marked_by = Some(*by);
    }
}
