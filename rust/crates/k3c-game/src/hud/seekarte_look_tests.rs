//! Die ersten vier Tests aus `native/k3c-game/hud/seekarte_paper_test.go` (`look_of` und `refresh`, nicht das Zeichnen).
//! Die übrigen gehören zu RU23.4 (Rückfall ohne Assets, Diorama mit Assets, Lage) und RU23.6.

use super::seekarte::{SeaKind, SeaNode, SeaView};
use super::seekarte_look::*;
use super::testutil::open_switch;
use crate::art::SeaMap;
use crate::view::frame::RouteIsland;
use crate::view::profile::{PROFILE_GAMEPAD, PROFILE_WASD};

/// Welt wie `sea_route`, mit Aussehen: a klein mit 1 Stufe, b mittel mit 3, c sehr groß mit 9, d ohne Angaben.
fn paper_route() -> Vec<RouteIsland> {
    let isl = |id: &str, etappe, world: &str, stages, chunks| RouteIsland {
        id: id.to_string(),
        name: id.to_uppercase(),
        etappe,
        world: world.to_string(),
        stages,
        chunks,
    };
    vec![
        isl("a", 1, "sengoku", 1, 16),
        isl("b", 2, "sengoku", 3, 40),
        isl("c", 2, "sengoku", 9, 100),
        isl("d", 3, "gibt-es-nicht", 0, 0),
    ]
}

fn node_of(v: &SeaView, id: &str) -> SeaNode {
    v.rows
        .iter()
        .flat_map(|r| &r.nodes)
        .find(|n| n.id == id)
        .unwrap_or_else(|| panic!("Insel {id} fehlt"))
        .clone()
}

/// AC-02 (B-964/R2): Größe aus der Inselgröße, Schichten aus der Stufenzahl, Aufsatz aus dem Hash der ID; bei beiden
/// Spielern und nach jedem Neubau gleich.
#[test]
fn sea_look_per_island() {
    let route = paper_route();
    let (mut p0, mut p1) = (SeaView::default(), SeaView::default());
    p0.refresh("de", PROFILE_WASD, Some(&open_switch("")), &route);
    p1.refresh("en", PROFILE_GAMEPAD, Some(&open_switch("c")), &route);
    for isl in &route {
        let (a, b) = (node_of(&p0, &isl.id), node_of(&p1, &isl.id));
        assert!(
            a.look == b.look && a.look == look_of(isl) && a.look.world == isl.world,
            "{}: Spieler 0 {:?}, Spieler 1 {:?}",
            isl.id,
            a.look,
            b.look
        );
    }
}

/// AC-02 (B-964/R2): Werte des Aussehens: Maßstab zwischen kleinster und größter Inselgröße, Schichten je Stufe
/// (begrenzt), fester Hash, alle Aufsätze kommen vor.
#[test]
fn sea_look_values() {
    let r = paper_route();
    let (a, b, c, d) = (look_of(&r[0]), look_of(&r[1]), look_of(&r[2]), look_of(&r[3]));
    assert!(
        a.scale == SEA_SCALE_MIN && c.scale == SEA_SCALE_MAX && a.scale < b.scale && b.scale < c.scale,
        "Maßstab a {}, b {}, c {}",
        a.scale,
        b.scale,
        c.scale
    );
    assert!(
        d.scale == f64::midpoint(SEA_SCALE_MIN, SEA_SCALE_MAX)
            && sea_scale(8) == SEA_SCALE_MIN
            && sea_scale(500) == SEA_SCALE_MAX,
        "Maßstab ohne Angabe {}, zu klein {}, zu groß {}",
        d.scale,
        sea_scale(8),
        sea_scale(500)
    );
    assert!(
        a.layers == 1 && b.layers == 3 && c.layers == SEA_LAYERS_MAX && d.layers == 1,
        "Schichten a {}, b {}, c {}, d {}",
        a.layers,
        b.layers,
        c.layers,
        d.layers
    );
}

/// AC-02 (B-964/R2): Der Aufsatz hängt an einem festen Hash der Insel-ID (FNV-1a), kein Zufall; alle Aufsätze kommen vor.
#[test]
fn sea_topper_from_hash() {
    assert!(
        sea_seed("a") == 0xe40c_292c && sea_seed("") == 2_166_136_261 && sea_seed("a") != sea_seed("b"),
        "Hash a {:#x}, leer {:#x}",
        sea_seed("a"),
        sea_seed("")
    );
    let m = SeaMap {
        toppers: ["turm", "baum", "fels"].map(str::to_string).to_vec(),
        ..SeaMap::default()
    };
    let seen: std::collections::BTreeSet<&str> = ["a", "b", "c", "d", "kyoto", "theben", "avalon", "hafen"]
        .iter()
        .map(|id| m.topper(sea_seed(id)))
        .collect();
    assert_eq!(seen.len(), 3, "Aufsätze über acht Inseln: {seen:?}");
}

/// AC-02 (B-964/R3): Zustände aus dem Wechselzustand: Schiff an der aktuellen Insel (die Simulation nennt sie als letzte
/// der Abgeschlossenen), abgeschlossen, wählbar, gesperrt; Cursor nur auf Zielen.
#[test]
fn sea_states_diorama() {
    let route = paper_route();
    let mut v = SeaView::default();
    let mut sw = open_switch("c");
    sw.completed = ["x", "a"].map(str::to_string).to_vec();
    v.refresh("de", PROFILE_WASD, Some(&sw), &route);
    let want = [
        ("a", SeaKind::Done, true, false, false),
        ("b", SeaKind::Target, false, false, false),
        ("c", SeaKind::Target, false, true, true),
        ("d", SeaKind::Locked, false, false, false),
    ];
    for (id, kind, current, chosen, cursor) in want {
        let n = node_of(&v, id);
        assert!(
            n.kind == kind && n.current == current && n.chosen == chosen && n.cursor == cursor,
            "{id}: {n:?}"
        );
    }
    sw.completed.clear(); // ohne Abgeschlossene liegt das Schiff nirgends
    v.refresh("de", PROFILE_WASD, Some(&sw), &route);
    assert!(
        !v.rows.iter().any(|r| r.nodes.iter().any(|n| n.current)),
        "Schiff ohne aktuelle Insel"
    );
}
