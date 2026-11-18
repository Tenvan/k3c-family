//! Bürger holen das nächste Werkzeug oder die nächste Waffe, nur ohne Gegner in der Nähe (Go: `unit_fetch_test.go`,
//! B-929, docs/rules/kampf.md § 4). `weapon_to_fetch` rechnet mit `fighters`, `troop_limit` und `weapons_in_flight`
//! (Kasernen, RU18); bis dahin laufen beide Tests nicht.

use super::warrior::weapon_to_fetch;
use crate::enemies::testutil::*;
use crate::state::{ItemDrop, World};

/// Go: `fetchFixture`: Welt ohne Gegner mit Bogner (1 Bogen im Regal) und einem Bauern am Bogner (Go: `portalIsland`,
/// eine Insel mit 2 Spielern; hier die Stufe ohne Portale).
fn fetch_fixture() -> (World, usize, usize) {
    let mut w = bare_world();
    let hub = w.hub_x;
    let shop = built_site(&mut w, "bowyer", hub + 60.0);
    w.sites[shop].bows = 1;
    w.sites[shop].swords = 0;
    let x = w.sites[shop].x;
    let p = add_peasant(&mut w, x, "");
    (w, p, shop)
}

#[test]
#[ignore = "wartet auf RU18"]
fn buerger_holen_das_naechste_boden_vor_regal_bei_gleichstand() {
    let (mut w, p, shop) = fetch_fixture();
    let v = w.vals.clone().expect("Wertesatz");
    let sx = w.sites[shop].x;
    w.troops[p].x = sx - 10.0;
    let j = weapon_to_fetch(&w, &v, p).expect("nur das Regal");
    assert!(
        j.r#type == "fetchBow" && j.site_id == w.sites[shop].id,
        "nur das Regal: {j:?}"
    );
    let id = w.new_id();
    w.drops.push(ItemDrop {
        id,
        kind: "archer".into(),
        x: sx + 30.0,
        work_site: 0,
    });
    assert_eq!(
        weapon_to_fetch(&w, &v, p).expect("Regal").r#type,
        "fetchBow",
        "das Regal ist näher als der Boden"
    );
    w.drops[0].x = w.troops[p].x + 5.0;
    assert_eq!(
        weapon_to_fetch(&w, &v, p).expect("Boden").r#type,
        "pickup",
        "der Boden ist näher"
    );
    w.drops[0].x = sx; // gleiche Distanz
    assert_eq!(
        weapon_to_fetch(&w, &v, p).expect("Boden").r#type,
        "pickup",
        "Boden vor Regal bei Gleichstand"
    );
}

#[test]
#[ignore = "wartet auf RU18"]
fn buerger_warten_mit_gegner_in_der_naehe() {
    let (mut w, p, shop) = fetch_fixture();
    let v = w.vals.clone().expect("Wertesatz");
    let near = v.troops["peasant"].fetch_enemy_distance;
    assert_eq!(near, 8.0, "Distanz aus den Daten");
    let px = w.troops[p].x;
    let e = spawn(&mut w, "goblin", px + near - 0.5);
    assert!(
        weapon_to_fetch(&w, &v, p).is_none(),
        "Gegner innerhalb {near} Units, Bürger holt trotzdem"
    );
    w.enemies[e].x = px + near + 3.0; // außerhalb am Bürger, aber am Regal: das Ziel zählt auch
    w.sites[shop].x = w.enemies[e].x + 1.0;
    assert!(weapon_to_fetch(&w, &v, p).is_none(), "Gegner am Ziel");
    w.enemies[e].x = px + near + 40.0;
    assert!(
        weapon_to_fetch(&w, &v, p).is_some(),
        "kein Gegner in der Nähe: der Bürger soll holen"
    );
    // Auch nachts ohne Gegner in der Nähe wird geholt (ersetzt die globale Gefahr).
    w.cycle.phase = "night".into();
    assert!(
        weapon_to_fetch(&w, &v, p).is_some(),
        "nachts ohne Gegner in der Nähe wartet der Bürger"
    );
}
