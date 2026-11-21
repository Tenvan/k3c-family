//! ECO1.4 (B-1194): Der Werkzeuggürtel (`Troop::hammer`) macht den Bauern zum Handwerker: Bau, Reparatur, Rodung und Fels.
//! Ohne Gürtel arbeitet er nicht an Gebäuden und Gelände (Ausnahme Bergmann an Fels und Adern); ohne native Regeln
//! (Go-Parität) sammelt jeder Bauer. Fällt ein Handwerker, bleibt der Gürtel liegen und ein Bauer holt ihn: `hammer_tests.rs`.

use super::jobs::find_job;
use crate::enemies::testutil::*;
use crate::state::{ResourceNode, World, WorldValues};
use std::sync::Arc;

/// Ein markierter Baum und ein markierter Fels neben dem Hub, sonst nichts.
fn marked_nodes(w: &mut World) {
    w.nodes.clear();
    for kind in ["tree", "rock"] {
        let id = w.new_id();
        let x = w.hub_x + 4.0;
        w.nodes.push(ResourceNode {
            id,
            kind: kind.into(),
            x,
            marked: true,
            ..ResourceNode::default()
        });
    }
}

fn values(w: &World) -> Arc<WorldValues> {
    Arc::clone(w.vals.as_ref().expect("Werte"))
}

fn gather_kind(w: &mut World, ti: usize) -> Option<String> {
    let v = values(w);
    let job = find_job(w, &v, ti, None)?;
    (job.r#type == "gather").then(|| w.nodes.iter().find(|n| n.id == job.node_id).map(|n| n.kind.clone()))?
}

/// Rodung und Fels: ohne Gürtel kein Sammelauftrag, mit Gürtel Baum und Fels, der Bergmann bleibt ohne Gürtel bei Fels.
#[test]
fn nur_der_gurtel_rodet_und_haut() {
    let mut w = quiet_world();
    let _players = [add_player(&mut w), add_player(&mut w)];
    marked_nodes(&mut w);
    let hub = w.hub_x;
    let plain = add_peasant(&mut w, hub, "");
    assert_eq!(
        gather_kind(&mut w, plain),
        None,
        "Bauer ohne Gürtel arbeitet nicht am Gelände"
    );
    w.troops[plain].hammer = true;
    let first = gather_kind(&mut w, plain).expect("Handwerker bekommt einen Auftrag");
    assert!(first == "tree" || first == "rock");
    let mut seen = vec![first];
    for n in &mut w.nodes {
        n.worker_id = None;
    }
    w.troops[plain].job = None;
    let other = add_peasant(&mut w, hub, "");
    w.troops[other].hammer = true;
    seen.extend(gather_kind(&mut w, other));
    assert!(seen.iter().all(|k| k == "tree" || k == "rock"));
}

/// Der Bergmann (Beruf) darf ohne Gürtel an Fels und Adern arbeiten.
#[test]
fn bergmann_braucht_keinen_gurtel() {
    let mut w = quiet_world();
    let _players = [add_player(&mut w), add_player(&mut w)];
    marked_nodes(&mut w);
    let hub = w.hub_x;
    let miner = add_peasant(&mut w, hub, "miner");
    assert!(gather_kind(&mut w, miner).is_some());
}

/// Ohne native Regeln (Go-Parität) sammelt jeder Bauer wie bisher.
#[test]
fn ohne_native_regeln_sammelt_jeder_bauer() {
    let mut w = quiet_world();
    marked_nodes(&mut w);
    let mut v = (*values(&w)).clone();
    v.native_rules = false;
    w.vals = Some(Arc::new(v));
    let hub = w.hub_x;
    let plain = add_peasant(&mut w, hub, "");
    assert!(gather_kind(&mut w, plain).is_some());
}

/// Bau und Reparatur brauchen den Gürtel: ohne ihn kein Auftrag, mit ihm baut er und repariert.
#[test]
fn nur_der_gurtel_baut_und_repariert() {
    let mut w = quiet_world();
    let _players = [add_player(&mut w), add_player(&mut w)];
    let site = build_site(&mut w, "wall");
    w.sites[site].hp = w.sites[site].max_hp / 2.0;
    let sx = w.sites[site].x;
    let plain = add_peasant(&mut w, sx, "");
    assert!(
        crate::building::repair::repair_job(&mut w, plain).is_none(),
        "ohne Gürtel keine Reparatur"
    );
    w.troops[plain].hammer = true;
    fund_repairs(&mut w);
    assert!(
        crate::building::repair::repair_job(&mut w, plain).is_some(),
        "mit Gürtel Reparatur"
    );
}

/// ECO1.5: Mit nativen Regeln kostet ein Reparaturauftrag 1 Grundmaterial und 2 Gold; fehlt eines, wartet das Gebäude
/// und nichts wird gebucht. Das Gold zahlt, wer genug hat (2 Spieler).
#[test]
fn reparatur_kostet_material_und_zwei_gold() {
    let mut w = quiet_world();
    let ps = [add_player(&mut w), add_player(&mut w)];
    let site = build_site(&mut w, "wall");
    w.sites[site].hp = w.sites[site].max_hp / 2.0;
    let sx = w.sites[site].x;
    let ti = add_peasant(&mut w, sx, "");
    w.troops[ti].hammer = true;
    w.players[ps[0]].gold = 1;
    w.players[ps[1]].gold = 5;
    w.stock.wood = 0;
    w.stock.stone = 0;
    let rep = crate::building::repair::repair_job;
    assert!(rep(&mut w, ti).is_none(), "ohne Material wartet das Gebäude");
    assert_eq!(w.players[ps[1]].gold, 5, "nichts gebucht");
    w.stock.wood = 3;
    w.stock.stone = 3;
    let before = (w.stock.wood, w.stock.stone);
    assert!(rep(&mut w, ti).is_some(), "mit Material und Gold");
    assert_eq!(
        (w.players[ps[0]].gold, w.players[ps[1]].gold),
        (1, 3),
        "2 Gold vom reicheren Spieler"
    );
    assert_eq!(
        before.0 + before.1 - w.stock.wood - w.stock.stone,
        1,
        "genau 1 Material"
    );
    w.players[ps[1]].gold = 1;
    w.sites[site].worker_id = None;
    assert!(rep(&mut w, ti).is_none(), "ohne 2 Gold keine Reparatur");
}
