//! WD1.2 (B-1204): Waldstück-Erkennung und Rodungsregel (äußere 5 je Seite, kleine Gruppen höchstens 5 zugleich).

use super::{OUTER, SMALL_LIMIT, stands};
use crate::enemies::testutil::quiet_world;
use crate::state::{ResourceNode, World};
use crate::world::vals;

/// Eine Welt mit genau diesen Bäumen (x) auf der Oberfläche; Lichtungen und Hub-Streifen bleiben wie im Level.
fn with_trees(xs: &[f64]) -> World {
    let mut w = quiet_world();
    w.nodes.clear();
    w.level.entities.retain(|e| e.kind == "castle");
    for &x in xs {
        let id = w.new_id();
        w.nodes.push(ResourceNode {
            id,
            kind: "tree".into(),
            x,
            ..ResourceNode::default()
        });
    }
    w
}

fn row(from: f64, n: usize, step: f64) -> Vec<f64> {
    (0..n).map(|i| from + i as f64 * step).collect()
}

fn markable(w: &World, i: usize) -> bool {
    let v = vals(w);
    crate::state::node_mark::node_markable(w, &w.nodes[i]) && super::tree_markable(w, &v, &w.nodes[i])
}

#[test]
fn luecke_ueber_drei_units_trennt_und_hub_streifen_trennt() {
    let mut xs = row(300.0, 10, 1.0);
    xs.extend(row(320.0, 10, 1.0)); // Lücke 11 > 3
    let w = with_trees(&xs);
    let v = vals(&w);
    let s = stands(&w, &v);
    assert_eq!(s.len(), 2, "{s:?}");
    assert_eq!((s[0].trees.len(), s[1].trees.len()), (10, 10));
}

#[test]
fn grosses_waldstueck_nur_aeussere_fuenf_je_seite_und_nachrueckend() {
    let w0 = quiet_world();
    let len = w0.level.width_units * 0.3;
    let n = (len / 1.0) as usize;
    let mut w = with_trees(&row(w0.hub_x + 40.0, n, 1.0));
    let v = vals(&w);
    assert!(stands(&w, &v)[0].big);
    let ok: Vec<usize> = (0..w.nodes.len()).filter(|i| markable(&w, *i)).collect();
    assert_eq!(ok.len(), 2 * OUTER, "äußere fünf je Seite");
    assert!(ok.contains(&0) && ok.contains(&(OUTER - 1)) && !ok.contains(&OUTER));
    // der erste fällt: Baum 6 rückt nach
    w.nodes[0].gone = true;
    assert!(markable(&w, OUTER), "Nachrücken");
    assert!(!markable(&w, OUTER + 1));
}

#[test]
fn kleine_gruppen_hoechstens_fuenf_zugleich() {
    let w0 = quiet_world();
    let mut w = with_trees(&row(w0.hub_x + 40.0, 8, 10.0)); // 8 Einzelbäume
    for i in 0..SMALL_LIMIT {
        assert!(markable(&w, i), "Baum {i}");
        w.nodes[i].marked = true;
    }
    assert!(!markable(&w, SMALL_LIMIT), "sechster zugleich");
    w.nodes[0].gone = true; // einer gefällt
    assert!(markable(&w, SMALL_LIMIT), "danach wieder frei");
}

/// Zwei Spieler teilen das Limit: Hat Spieler 1 fünf Einzelbäume markiert, bietet der sechste auch Spieler 2 kein
/// Markier-Zahlziel mehr; nach einer Rodung wieder.
#[test]
fn zwei_spieler_teilen_das_limit_der_kleinen_gruppen() {
    use crate::economy::{PayTarget, find_pay_target};
    let mut w = quiet_world();
    let (p1, p2) = (crate::world::add_player(&mut w), crate::world::add_player(&mut w));
    w.nodes.clear();
    w.level.entities.retain(|e| e.kind == "castle");
    let base = w.hub_x + 40.0;
    for i in 0..7 {
        let id = w.new_id();
        w.nodes.push(ResourceNode {
            id,
            kind: "tree".into(),
            x: base + f64::from(i) * 10.0,
            ..ResourceNode::default()
        });
    }
    for i in 0..SMALL_LIMIT {
        w.nodes[i].marked = true;
    }
    let v = vals(&w);
    w.players[p1].x = base + 50.0;
    w.players[p2].x = base + 50.0;
    for p in [p1, p2] {
        let t = find_pay_target(&w, &v, None, &w.players[p]);
        assert!(
            !matches!(t, Some(PayTarget::Node(_))),
            "Spieler {p}: Limit erreicht, kein Markier-Ziel"
        );
    }
    w.nodes[0].gone = true;
    let t = find_pay_target(&w, &v, None, &w.players[p2]);
    assert!(
        matches!(t, Some(PayTarget::Node(_))),
        "nach der Rodung wieder markierbar"
    );
}
