//! Tests der verteilten Mauer- und Turm-Plätze (Go: `world_lines_test.go`, `lines_shim_test.go`; B-652, BS12.2). Positionen
//! kommen aus `Layout.sites` der Stufe (Zone n ab Hub-Stufe n), bezahlbar erst im Freiraum von Bäumen und Felsen; ein
//! Tor-Platz existiert nicht mehr. Die Go-Tests bauen Stufen mit `create_world` (Plätze, Ruinen RU18) und prüfen
//! `site_payable` und Zahlungen (RU18); sie tragen `ignore`. Eigene Tests an einer Hand-Welt: `lines_own_tests.rs`.

use super::lines::level_site;
use super::testutil::*;
use super::{add_player, step};
use crate::island::testutil::{must_island, test_data};
use crate::state::common::destroy_site;
use crate::state::{PlayerCommand, ResourceNode, Site, World};

/// Go: `sitePayable` (economy.go) ohne Insel.
fn site_payable(w: &World, s: &Site) -> bool {
    crate::economy::site_payable(w, &crate::world::vals(w), None, s)
}

/// Go: `wantPayable`: prüft `site_payable` für `kind` der Zone `k` auf Seite `side`.
fn want_payable(w: &mut World, kind: &str, side: i64, k: i64, want: bool) {
    let si = line_site(w, kind, side, k);
    let got = site_payable(w, &w.sites[si]);
    assert_eq!(
        got, want,
        "Hub-Stufe {}: {kind} Linie {k} Seite {side:+} bezahlbar = {got}, erwartet {want}",
        w.hub_level
    );
}

// Zone n ab Hub-Stufe n, je Seite; innere Mauern müssen nicht gebaut sein.
#[test]
fn zone_freischaltung_je_seite() {
    let mut w = quiet_world();
    clear_nodes(&mut w);
    assert_eq!(w.hub_level, 1, "Start-Hub-Stufe");
    for kind in ["wall", "tower"] {
        want_payable(&mut w, kind, -1, 1, true);
        want_payable(&mut w, kind, 1, 1, true);
        want_payable(&mut w, kind, -1, 2, false);
        want_payable(&mut w, kind, 1, 3, false);
    }
    w.hub_level = 3;
    for kind in ["wall", "tower"] {
        for side in [-1, 1] {
            want_payable(&mut w, kind, side, 3, true); // ohne gebaute Mauern der Zonen 1 und 2
            want_payable(&mut w, kind, side, 4, false);
        }
    }
}

// Freiraum: Ein Baum oder Fels im Radius sperrt den Platz, nach dem Abbau (gone/entfernt) ist er bezahlbar.
#[test]
fn freiraum_sperrt_platz_bis_zum_abbau() {
    let mut w = quiet_world();
    clear_nodes(&mut w);
    let si = line_site(&mut w, "wall", -1, 1);
    let site_x = w.sites[si].x;
    let radius = clear_radius();
    let id = w.new_id();
    w.nodes.push(ResourceNode {
        id,
        kind: "tree".into(),
        x: site_x + radius - 1.0,
        ..ResourceNode::default()
    });
    let tree = w.nodes.len() - 1;
    want_payable(&mut w, "wall", -1, 1, false);
    w.nodes[tree].x = site_x + radius + 1.0;
    want_payable(&mut w, "wall", -1, 1, true);
    let id = w.new_id();
    w.nodes.push(ResourceNode {
        id,
        kind: "rock".into(),
        x: site_x,
        ..ResourceNode::default()
    });
    let rock = w.nodes.len() - 1;
    want_payable(&mut w, "wall", -1, 1, false);
    w.nodes[rock].gone = true;
    want_payable(&mut w, "wall", -1, 1, true);
    let id = w.new_id();
    w.nodes.push(ResourceNode {
        id,
        kind: "bush".into(),
        x: site_x,
        ..ResourceNode::default()
    }); // Büsche sperren nicht
    want_payable(&mut w, "wall", -1, 1, true);
}

// Das Tor ist die äußerste gebaute Mauer je Seite: Es wandert nach innen, wenn sie zerstört wird, und nach außen, wenn
// weiter außen gebaut wird.
#[test]
fn tor_ist_aeusserst_gebaute_mauer() {
    let mut w = quiet_world();
    let inner = line_site(&mut w, "wall", 1, 1);
    let outer = line_site(&mut w, "wall", 1, 2);
    assert!(outermost_gate(&w, 1).is_none(), "ohne gebaute Mauer gibt es kein Tor");
    built(&mut w.sites[inner]);
    assert_eq!(
        outermost_gate(&w, 1),
        Some(w.sites[inner].x),
        "Tor: erwartet die innere Mauer"
    );
    built(&mut w.sites[outer]);
    assert_eq!(
        outermost_gate(&w, 1),
        Some(w.sites[outer].x),
        "Tor: erwartet die äußere Mauer"
    );
    let outer_id = w.sites[outer].id;
    destroy_site(&mut w, outer_id);
    assert_eq!(
        outermost_gate(&w, 1),
        Some(w.sites[inner].x),
        "Tor nach Zerstörung: erwartet wieder die innere Mauer"
    );
    assert!(
        w.sites.iter().all(|s| s.kind != "gate"),
        "ein eigenes Tor gibt es nicht mehr"
    );
}

// (d) Q54: Ein innerer Turm trägt weiter Schützen, Gegner bleiben an der äußersten gebauten Mauer stehen.
#[test]
#[ignore = "wartet auf RU18.5 (step_passives)"]
fn zone_innerer_turm_und_aeusserste_mauer() {
    use crate::unit::archer::is_on_tower;
    use crate::unit::jobs::make_archer;
    let mut w = quiet_world();
    w.hub_level = 2;
    let inner = line_site(&mut w, "tower", -1, 1);
    built(&mut w.sites[inner]);
    let wall = line_site(&mut w, "wall", -1, 1);
    built(&mut w.sites[wall]);
    let tower2 = line_site(&mut w, "tower", -1, 2);
    built(&mut w.sites[tower2]);
    let outer = line_site(&mut w, "wall", -1, 2);
    built(&mut w.sites[outer]);
    let inner_x = w.sites[inner].x;
    let archer = crate::unit::spawn_vagrant(&mut w, inner_x, inner_x);
    make_archer(&mut w, archer);
    w.troops[archer].anchor_x = inner_x;
    // Go prüft hier `freeTower(w, archer) == inner`; die Funktion ist in Rust nicht öffentlich: das Ergebnis zeigt der Lauf.
    let outer_x = w.sites[outer].x;
    let e = spawn(&mut w, "goblin", outer_x - 10.0);
    run_until(&mut w, 6.0, |_| false);
    let (inner_id, t) = (w.sites[inner].id, &w.troops[archer]);
    assert!(
        t.tower_id == Some(inner_id) && is_on_tower(&w, t),
        "Schütze nicht auf dem inneren Turm: {t:?}"
    );
    let (en, o) = (&w.enemies[e], &w.sites[outer]);
    assert!(
        !(en.hp > 0.0 && (en.x >= o.x || o.hp >= o.max_hp)),
        "Gegner bei {}, äußere Mauer bei {} mit {}/{}",
        en.x,
        o.x,
        o.hp,
        o.max_hp
    );
}

/// Go: `payResult`: Zustand und bezahltes Gold zweier Plätze und das ausgegebene Gold der Spieler, die dort stehen.
#[derive(Debug, PartialEq)]
struct PayResult {
    state_a: String,
    state_b: String,
    paid_a: i64,
    paid_b: i64,
    spent_a: i64,
    spent_b: i64,
}

/// Go: `payTogether`: lässt zwei Spieler gleichzeitig an `a` und `b` zahlen; `swap` tauscht, welcher Spieler wo steht.
fn pay_together(pick: fn(&mut World) -> (usize, usize), swap: bool) -> PayResult {
    let mut w = quiet_world();
    clear_nodes(&mut w);
    let (p0, p1) = (add_player(&mut w), add_player(&mut w));
    let (a, b) = pick(&mut w);
    let (pa, pb) = if swap { (p1, p0) } else { (p0, p1) };
    let (ax, bx) = (w.sites[a].x, w.sites[b].x);
    w.players[pa].x = ax;
    w.players[pb].x = bx;
    const GOLD: i64 = 50;
    w.players[p0].gold = GOLD;
    w.players[p1].gold = GOLD;
    let cmds = [
        PlayerCommand {
            pay: true,
            ..PlayerCommand::default()
        },
        PlayerCommand {
            pay: true,
            ..PlayerCommand::default()
        },
    ];
    for _ in 0..300 {
        step(&mut w, &cmds, None, DT);
        w.players[pa].x = ax;
        w.players[pb].x = bx;
    }
    PayResult {
        state_a: w.sites[a].state.clone(),
        state_b: w.sites[b].state.clone(),
        paid_a: w.sites[a].paid_gold,
        paid_b: w.sites[b].paid_gold,
        spent_a: GOLD - w.players[pa].gold,
        spent_b: GOLD - w.players[pb].gold,
    }
}

// (e) AC-06: Zwei Spieler zahlen gleichzeitig an Mauer und Turm derselben Zone bzw. an beiden Seiten.
#[test]
#[ignore = "wartet auf RU18.5 (speed_of)"]
fn zone_zwei_spieler_zahlen_gleichzeitig() {
    type Pick = fn(&mut World) -> (usize, usize);
    let cases: [(&str, Pick); 2] = [
        ("Mauer und Turm links", |w| {
            (line_site(w, "wall", -1, 1), line_site(w, "tower", -1, 1))
        }),
        ("Mauer links und rechts", |w| {
            (line_site(w, "wall", -1, 1), line_site(w, "wall", 1, 1))
        }),
    ];
    for (name, pick) in cases {
        let (got, swapped) = (pay_together(pick, false), pay_together(pick, true));
        assert_eq!(got, swapped, "{name}: Ergebnis hängt von der Spieler-Reihenfolge ab");
        assert!(
            got.state_a != "unpaid" && got.state_b != "unpaid" && got.spent_a != 0 && got.spent_b != 0,
            "{name}: nicht beide Zahlungen gezählt: {got:?}"
        );
    }
}

// Jede Stufe einer Insel hat Mauern und Türme ihres eigenen Layouts, fest nach X angelegt.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn zonen_insel_jede_stufe_eigene_sites() {
    use super::ship::SHIP_KIND;
    let isl = must_island(&test_data(), "linien", &[0, 1]);
    for (i, w) in isl.stages.iter().enumerate() {
        let mut sites = 0;
        for (j, s) in w.sites.iter().enumerate() {
            if level_site(w, s).is_some() {
                sites += 1;
            }
            if j > 0 && s.x < w.sites[j - 1].x && s.kind != "storage" && s.kind != SHIP_KIND {
                let p = &w.sites[j - 1];
                panic!(
                    "Stufe {i}: Platz {}@{} vor {}@{} nicht nach X",
                    s.kind, s.x, p.kind, p.x
                );
            }
        }
        assert!(
            sites == w.level.sites.len() && sites != 0,
            "Stufe {i}: {sites} verteilte Plätze, erwartet {}",
            w.level.sites.len()
        );
    }
    let (a, b) = (&isl.stages[0], &isl.stages[1]);
    assert!(a.level.sites != b.level.sites, "beide Stufen haben dieselben Plätze");
}
