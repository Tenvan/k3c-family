//! Tests zu `outpost.rs` (Go: `building_outpost_test.go`, WZ14.4, B-468/AC-03, docs/rules/geplante-chunks.md § 8): Plätze
//! jenseits des äußersten Tors laut Layout, mit 2 Spielern über `fixture_island` (alle drei Ruinen der Fixture liegen dort).
//! Die Tests über `fixture_island` warten auf RU18 (Zahlplatz `new_hub_site`).

use k3c_level::types::WallSite;

use super::outpost::{outermost_wall as outermost_gate, outpost}; // Go: `outermostGate` ist `outermostWall` (Mauer-Plätze des Levels)
use super::testutil::*;
use crate::state::common::{destroy_site, is_dangerous};
use crate::state::{Job, World};
use crate::unit::jobs::build_job;
use crate::unit::warrior::weapon_to_fetch;
use crate::unit::{leave_on_danger, step_peasant};
use crate::world::vals;

/// Go: `edgeWalls` setzt die äußersten Mauer-Plätze der Fixture auf Hub ± 124 (alte Linie 5): Die Ruinen der Fixture
/// liegen dahinter, die Plätze der Werkstatt und des Lagers innerhalb (die verteilten Zonen reichen sonst bis ±290, B-652).
fn edge_walls(w: &mut World) {
    let hub = w.hub_x;
    w.level.sites = vec![
        WallSite {
            side: -1,
            kind: "wall".into(),
            x: hub - 124.0,
            zone: 5,
        },
        WallSite {
            side: 1,
            kind: "wall".into(),
            x: hub + 124.0,
            zone: 5,
        },
    ];
}

/// Go: `outpostPeasant` ist der Bauer der Stufe, tagsüber ohne Gegner.
fn outpost_peasant(w: &World) -> usize {
    match w.troops.iter().position(|t| t.serving() && !is_dangerous(w)) {
        Some(ti) => ti,
        None => panic!("Ausgangslage: Bauer fehlt oder Gefahr"),
    }
}

/// Go: `endanger` setzt einen betäubten Gegner an den Rand der Stufe: Gefahr, ohne dass jemand angegriffen wird.
fn endanger(w: &mut World) {
    let x = w.width_units - 1.0;
    let e = spawn(w, "goblin", x);
    w.enemies[e].stun = f64::INFINITY;
}

#[test]
fn vorposten_lage_laut_layout() {
    let (mut isl, _) = fixture_island();
    let w = &mut isl.stages[0];
    edge_walls(w);
    let left = outermost_gate(w, -1).unwrap_or(0.0);
    let right = outermost_gate(w, 1).unwrap_or(0.0);
    for s in &w.sites {
        let want = s.x < left || s.x > right;
        assert_eq!(
            outpost(w, s.x),
            want,
            "{}@{}: äußerste Mauern {left}/{right}",
            s.kind,
            s.x
        );
        assert!(
            !s.ruin || outpost(w, s.x),
            "Ruine {}@{} liegt nicht jenseits der äußersten Mauer",
            s.kind,
            s.x
        );
    }
    w.level.sites.clear();
    assert!(
        !outpost(w, 0.0) && !outpost(w, w.width_units),
        "ohne Mauer-Plätze gibt es keine Vorposten"
    );
}

/// Reparatur am Vorposten tagsüber; bei Gefahr bleibt sie liegen und der Bauer kehrt zur Burg zurück.
#[test]
fn vorposten_reparatur_nur_ohne_gefahr() {
    let (mut isl, data) = fixture_island();
    let w = &mut isl.stages[0];
    edge_walls(w);
    let p = outpost_peasant(w);
    crate::enemies::testutil::fund_repairs(w);
    w.troops[p].job = super::repair::repair_job(w, p);
    let job = w.troops[p].job.clone().expect("kein Reparatur-Auftrag");
    let si = w.sites.iter().position(|s| s.id == job.site_id).expect("Platz");
    assert!(
        w.sites[si].ruin && outpost(w, w.sites[si].x),
        "Reparatur-Auftrag {job:?}, erwartet eine Ruine am Vorposten"
    );
    let v = vals(w);
    let _ = &data;
    for _ in 0..30 {
        step_peasant(w, &v, p, None, DT);
    }
    assert!(
        w.troops[p].job.as_ref().is_some_and(|j| j.r#type == "repair"),
        "tagsüber Reparatur liegen gelassen"
    );
    endanger(w);
    let start = (w.troops[p].x - w.hub_x).abs();
    for _ in 0..30 {
        step_peasant(w, &v, p, None, DT);
    }
    assert!(
        w.troops[p].job.is_none() && w.sites[si].worker_id.is_none(),
        "bei Gefahr Auftrag nicht abgegeben"
    );
    assert!((w.troops[p].x - w.hub_x).abs() < start, "Bauer geht nicht zur Burg");
}

/// Bau am Vorposten nur ohne Gefahr; ein Bauplatz innerhalb der Mauern wird auch bei Gefahr gebaut.
#[test]
fn vorposten_bau_nur_ohne_gefahr() {
    let (mut isl, _) = fixture_island();
    let w = &mut isl.stages[0];
    edge_walls(w);
    let p = outpost_peasant(w);
    let ri = ruin_of(w, "storage");
    let rid = w.sites[ri].id;
    destroy_site(w, rid);
    w.sites[ri].state = "waitingWorker".into();
    assert!(
        build_job(w, p).is_some_and(|j| j.site_id == rid),
        "tagsüber kein Bau-Auftrag für die Ruine"
    );
    w.sites[ri].worker_id = None;
    endanger(w);
    assert!(build_job(w, p).is_none(), "bei Gefahr Bau am Vorposten angenommen");
    let ii = site_at(w, "storage", 526.0).expect("Lager innerhalb der Mauern");
    let iid = w.sites[ii].id;
    w.sites[ii].state = "waitingWorker".into();
    assert!(
        build_job(w, p).is_some_and(|j| j.site_id == iid),
        "bei Gefahr Bau innerhalb der Mauern"
    );
    let inner_job = Job {
        r#type: "build".into(),
        site_id: iid,
        ..Job::default()
    };
    assert!(
        !leave_on_danger(w, Some(&inner_job)),
        "Neubau innerhalb der Mauern bleibt bei Gefahr liegen"
    );
    let pid = w.troops[p].id;
    w.troops[p].job = Some(Job {
        r#type: "build".into(),
        site_id: rid,
        ..Job::default()
    });
    w.sites[ri].worker_id = Some(pid);
    w.sites[ii].worker_id = None;
    let v = vals(w);
    step_peasant(w, &v, p, None, DT);
    assert!(
        !w.troops[p].job.as_ref().is_some_and(|j| j.site_id == rid),
        "laufender Bau am Vorposten bei Gefahr nicht abgebrochen"
    );
}

/// Waffen holen: an einer Werkstatt jenseits des Tors nur ohne Gefahr, innerhalb der Mauern auch bei Gefahr.
#[test]
fn vorposten_waffen_nur_ohne_gefahr() {
    let (mut isl, _) = fixture_island();
    let w = &mut isl.stages[0];
    edge_walls(w);
    let p = outpost_peasant(w);
    w.troops[p].hammer = false; // Waffen holt nur ein Bauer ohne Hammer (B-950)
    let right = outermost_gate(w, 1).unwrap_or(0.0);
    let mut far = crate::world::empty_site(w, "bowyer", right + 20.0);
    far.state = "built".into();
    far.bows = 1;
    let fid = far.id;
    w.sites.push(far);
    let v = vals(w);
    let j = weapon_to_fetch(w, &v, p).expect("tagsüber Bogen holen");
    assert!(
        j.r#type == "fetchBow" && j.site_id == fid,
        "erwartet Werkstatt {fid}: {j:?}"
    );
    endanger(w);
    assert!(
        weapon_to_fetch(w, &v, p).is_none(),
        "bei Gefahr Waffe am Vorposten geholt"
    );
    let sword = Job {
        r#type: "fetchSword".into(),
        site_id: fid,
        ..Job::default()
    };
    assert!(
        leave_on_danger(w, Some(&sword)),
        "Schwert holen am Vorposten bleibt bei Gefahr nicht liegen"
    );
    let ii = site_at(w, "workshop", 534.0).expect("Werkstatt innerhalb der Mauern");
    let iid = w.sites[ii].id;
    w.sites[ii].state = "built".into();
    w.sites[ii].swords = 1;
    let j = weapon_to_fetch(w, &v, p).expect("bei Gefahr Schwert innerhalb der Mauern");
    assert!(j.r#type == "fetchSword" && j.site_id == iid, "erwartet {iid}: {j:?}");
}

/// Dev-Funktion Burgfall: Stufenverlust auch ohne gefallene Burg, die zerstörte Ruine steht wieder.
#[test]
fn dev_castle_fall_stellt_ruine_wieder_her() {
    let (mut isl, data) = fixture_island();
    let w = &mut isl.stages[0];
    let si = ruin_of(w, "smithy");
    let id = w.sites[si].id;
    destroy_site(w, id);
    w.players[0].gold = 10;
    w.players[1].gold = 7;
    assert!(
        crate::dev::dev_castle_fall(&mut isl, &data.difficulty, 1),
        "Spieler 1 nicht gefunden"
    );
    let w = &isl.stages[0];
    let s = &w.sites[si];
    let hp = ruin_start_hp(w, "smithy");
    assert!(
        s.id == id && s.ruin && s.state == "built" && s.hp == hp,
        "Ruine nach Dev-Burgfall: {s:?}"
    );
    assert_eq!((w.players[0].gold, w.players[1].gold), (5, 3), "Gold nach Dev-Burgfall");
    assert!(w.troops.iter().all(|t| t.is_wild()), "Truppe überlebt den Burgfall");
    assert!(
        !crate::dev::dev_castle_fall(&mut isl, &data.difficulty, 9),
        "unbekannter Spieler akzeptiert"
    );
}

/// Eigener Test (ohne Go-Gegenstück, läuft ohne `create_world`): Vorposten laut Layout, Mauer-Plätze nach `side`.
#[test]
fn outpost_laut_layout_ohne_welt() {
    let mut w = bare_world();
    w.width_units = 400.0;
    assert!(
        !outpost(&w, 0.0) && !outpost(&w, 400.0),
        "ohne Mauer-Plätze gibt es keine Vorposten"
    );
    edge_walls(&mut w);
    let hub = w.hub_x;
    assert!(!outpost(&w, hub) && !outpost(&w, hub + 124.0) && !outpost(&w, hub - 124.0));
    assert!(outpost(&w, hub + 124.5) && outpost(&w, hub - 124.5));
    w.level.sites.push(WallSite {
        side: 1,
        kind: "tower".into(),
        x: hub + 300.0,
        zone: 6,
    });
    assert!(!outpost(&w, hub + 124.0), "Türme zählen nicht als Mauer-Platz");
    w.level.sites.push(WallSite {
        side: 1,
        kind: "wall".into(),
        x: hub + 150.0,
        zone: 6,
    });
    assert!(
        !outpost(&w, hub + 140.0) && outpost(&w, hub + 151.0),
        "äußerster Mauer-Platz gilt"
    );
}

/// Eigener Test: `outpost_job` gilt für Bau, Reparatur und Waffen-Holen am Vorposten, nicht für `gather`.
#[test]
fn outpost_job_je_auftrag() {
    let mut w = bare_world();
    w.width_units = 400.0;
    edge_walls(&mut w);
    let hub = w.hub_x;
    let far = built_site(&mut w, "bowyer", hub + 200.0);
    let id = w.sites[far].id;
    for (typ, want) in [
        ("build", true),
        ("repair", true),
        ("fetchBow", true),
        ("fetchHammer", true),
        ("gather", false),
    ] {
        let j = Job {
            r#type: typ.into(),
            site_id: id,
            ..Job::default()
        };
        assert_eq!(super::outpost::outpost_job(&w, &j), want, "{typ}");
    }
}
